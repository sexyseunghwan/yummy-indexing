use crate::common::*;

use crate::services::es_query_service::*;
use crate::services::query_service::*;

use crate::configuration::{index_schedules_config::*, system_config::*};

use crate::models::store_to_elastic::*;
use crate::models::store_types::*;
use crate::models::auto_complete::*;

use crate::utils_module::time_utils::*;

#[derive(Debug, new)]
pub struct ProcessHandler<
    Q: QueryService + Sync + Send + 'static,
    E: EsQueryService + Sync + Send + 'static,
> {
    query_service: Q,
    es_query_service: E,
}

impl<Q: QueryService + Sync + Send + 'static, E: EsQueryService + Sync + Send + 'static>
    ProcessHandler<Q, E> {
    
    #[doc = "메인 작업 함수 -> 색인 진행 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_indexing_task(
        &self,
        index_schedule: IndexSchedules,
    ) -> Result<(), anyhow::Error> {
        let function_name: &str = index_schedule.function_name().as_str();

        match function_name {
            "store_static_index" => self.store_static_index(index_schedule).await?,
            "store_dynamic_index" => self.store_dynamic_index(index_schedule).await?,
            "auto_complete_static_index" => self.auto_complete_static_index(index_schedule).await?,
            _ => {
                return Err(anyhow!(
                    "[Error][main_task()] The mapped function does not exist.: {}",
                    function_name
                ))
            }
        }

        Ok(())
    }

    #[doc = "상점분류 정해주는 함수"]
    /// # Arguments
    /// * `store_seq` - 인덱스 스케쥴 객체
    /// * `stores_distinct` - 중복을 제외한 store list
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn handling_store_type(
        &self,
        store_seq: Option<Vec<i32>>,
        stores_distinct: &mut Vec<DistinctStoreResult>,
    ) -> Result<(), anyhow::Error> {
        /* store 리스트와 대응되는 상점분류 데이터 가져오기 */
        let store_types_all: StoreTypesMap = if let Some(seq) = store_seq {
            self.query_service.get_store_types(Some(seq)).await?
        } else {
            self.query_service.get_store_types(None).await?
        };

        let store_type_major_map: HashMap<i32, Vec<i32>> = store_types_all.store_type_major_map;
        let store_type_sub_map: HashMap<i32, Vec<i32>> = store_types_all.store_type_sub_map;

        for store_elem in stores_distinct {
            let seq: i32 = store_elem.seq;

            let major_vec: &Vec<i32> = store_type_major_map
                .get(&seq)
                .ok_or_else(|| anyhow!("[Error][handling_store_type()] No 'seq' corresponding to 'store_type_major_map'. seq: {}", seq))?;

            let sub_vec: &Vec<i32> = store_type_sub_map
                .get(&seq)
                .ok_or_else(|| anyhow!("[Error][handling_store_type()] No 'seq' corresponding to 'store_type_sub_map'. seq: {}", seq))?;

            store_elem.set_major_type(major_vec.clone());
            store_elem.set_sub_type(sub_vec.clone());
        }

        Ok(())
    }

    #[doc = "Store 객체를 정적색인 해주는 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn store_static_index(
        &self,
        index_schedule: IndexSchedules,
    ) -> Result<(), anyhow::Error> {
        /* 현재기준 UTC 시간 */
        let cur_utc_date: NaiveDateTime = get_current_utc_naive_datetime();

        /* 중복을 제외한 store 리스트 */
        let mut stores_distinct: Vec<DistinctStoreResult> = self
            .query_service
            .get_all_store_table(&index_schedule, cur_utc_date)
            .await?;

        self.handling_store_type(None, &mut stores_distinct).await?;

        /* Elasticsearch 에 데이터 색인. */
        self.es_query_service
            .post_indexing_data_by_bulk_static::<DistinctStoreResult>(
                &index_schedule,
                &stores_distinct,
            )
            .await?;

        /* 색인시간 최신화 */
        self.query_service
            .update_recent_date_to_elastic_index_info(&index_schedule, cur_utc_date)
            .await?;

        info!("Store - Static Create Indexing: {}", stores_distinct.len());

        Ok(())
    }

    #[doc = "Store 객체를 증분색인 해주는 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn store_dynamic_index(
        &self,
        index_schedule: IndexSchedules,
    ) -> Result<(), anyhow::Error> {
        let cur_utc_date: NaiveDateTime = get_current_utc_naive_datetime(); /* 현재기준 UTC 시간 */

        /* RDB 에서 검색엔진에 가장 마지막으로 색인한 날짜를 가져와준다. */
        let recent_index_datetime: NaiveDateTime = self
            .query_service
            .get_recent_date_from_elastic_index_info(&index_schedule)
            .await?;

        /*
            증분색인은 Delete -> Create 로 나눔
            일단 수정되거나 새로 등록된 데이터를 기준으로 하는 상점 데이터를 모두 지워준다.
            그 다음 Create 를 사용해서 update,create 된 모든 데이터를 실제로 색인해준다.
        */

        /* 0. 변경된 데이터 추출 */
        let mut changed_list: Vec<DistinctStoreResult> = self
            .query_service
            .get_specific_store_table(&index_schedule, cur_utc_date, recent_index_datetime)
            .await?;

        /* 1. Delete */
        if !changed_list.is_empty() {
            self.es_query_service
                .delete_index(&index_schedule, &changed_list, "seq")
                .await?;
            info!("DELETE Data: {:?}", changed_list);
        }

        /* 2. Create */
        let seq_list: Vec<i32> = changed_list.iter().map(|item| item.seq).collect();

        self.handling_store_type(Some(seq_list), &mut changed_list)
            .await?;

        if !changed_list.is_empty() {
            self.es_query_service
                .post_indexing_data_by_bulk_dynamic::<DistinctStoreResult>(
                    &index_schedule,
                    &changed_list,
                )
                .await?;
            info!("CREATE Data: {:?}", changed_list);
        }

        if !changed_list.is_empty() {
            /* 색인시간 최신화 */
            self.query_service
                .update_recent_date_to_elastic_index_info(&index_schedule, cur_utc_date)
                .await?;
        }

        Ok(())
    }

    #[doc = "자동완성 키워드 정적색인 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn auto_complete_static_index(
        &self,
        index_schedule: IndexSchedules,
    ) -> Result<(), anyhow::Error> {
        /* 현재기준 UTC 시간 */
        let cur_utc_date: NaiveDateTime = get_current_utc_naive_datetime();
        
        /* 자동완성 키워드 데이터 리스트 */
        let auto_completes: Vec<AutoComplete> = self.query_service.get_store_name_distinct_by_batch(&index_schedule).await?;

        /* Elasticsearch 에 데이터 색인. */
        self.es_query_service
            .post_indexing_data_by_bulk_static::<AutoComplete>(
                &index_schedule,
                &auto_completes,
            )
            .await?;
        
        /* 색인시간 최신화 */
        self.query_service
            .update_recent_date_to_elastic_index_info(&index_schedule, cur_utc_date)
            .await?;
        
        Ok(())
    }
}
