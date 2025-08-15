use crate::common::*;

use crate::services::analyzer_service::*;
use crate::services::es_query_service::*;
use crate::services::query_service::*;

use crate::configuration::{index_schedules_config::*};

use crate::models::{auto_complete::*, auto_search_keyword::*, 
    store_to_elastic::*, subway_info::*
};

use crate::utils_module::time_utils::*;

#[derive(Debug, new)]
pub struct ProcessHandler<
    Q: QueryService + Sync + Send + 'static,
    E: EsQueryService + Sync + Send + 'static,
    A: AnalyzerService + Sync + Send + 'static,
> {
    query_service: Q,
    es_query_service: E,
    analyzer_service: A,
}

impl<
        Q: QueryService + Sync + Send + 'static,
        E: EsQueryService + Sync + Send + 'static,
        A: AnalyzerService + Sync + Send + 'static,
    > ProcessHandler<Q, E, A>
{
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

        /* 리뷰개수, 평점 색인은 5분..에 한번씩 도는걸로 진행해도 괜찮을 듯 보임 */
        match function_name {
            "store_static_index" => self.store_static_index(index_schedule).await?,
            "store_dynamic_index" => self.store_dynamic_index(index_schedule).await?,
            "auto_complete_static_index" => self.auto_complete_static_index(index_schedule).await?,
            "subway_static_index" => self.subway_static_index(index_schedule).await?,
            _ => {
                return Err(anyhow!(
                    "[Error][main_task()] The mapped function does not exist.: {}",
                    function_name
                ))
            }
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
        let stores_distinct: Vec<DistinctStoreResult> = self
            .query_service
            .get_all_store_table(&index_schedule, cur_utc_date)
            .await?;
        
        /* 위치정보를 geopoint 형식으로 재정의 하기 위함 */
        
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
        let changed_list: Vec<DistinctStoreResult> = self
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


    #[doc = "입력된 항목 리스트에서 초성(Chosung)을 추출하여 `AutoComplete` 리스트로 변환한다."]
    /// # Arguments
    /// * `items` - 문자열로 변환 가능한 항목들의 반복 가능한 컬렉션
    /// * `to_string` - 각 항목을 `String`으로 변환하는 함수
    ///
    /// # Returns
    /// * Vec<AutoComplete> 
    fn to_auto_complete_list<T>(
        &self,
        items: impl IntoIterator<Item = T>,
        to_string: impl Fn(T) -> String,
        to_integer: impl Fn(T) -> i32,
    ) -> Vec<AutoComplete>
    where T: Clone
    {
        items
            .into_iter()
            .map(|item| {
                let name: String = to_string(item.clone());
                let chosung: String = self.analyzer_service.extract_chosung(&name);
                let keyword_weight: i32 = to_integer(item.clone());
                AutoComplete::new(name, chosung, keyword_weight)
            })
            .collect()
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
        let mut auto_completes: Vec<AutoComplete> = Vec::new();
        
        /* 1. 자동 완성 키워드 */
        let auto_keyword_list: Vec<AutoSearchKeyword> = 
            self.query_service.get_auto_search_keyword_by_batch(&index_schedule).await?;
        
        let mut auto_complete_keyword: Vec<AutoComplete> =
            self.to_auto_complete_list(
                auto_keyword_list,  
                |a: AutoSearchKeyword| a.keyword, 
                |a: AutoSearchKeyword| a.keyword_weight);
        
        auto_completes.append(&mut auto_complete_keyword);
        /* 1. 상점 이름 리스트 */
        // let store_auto_complete: Vec<StoreAutoComplete> = self
        //     .query_service
        //     .get_store_name_by_batch(&index_schedule)
        //     .await?;

        // let mut auto_complete_store: Vec<AutoComplete> =
        //     self.to_auto_complete_list(store_auto_complete, |s: StoreAutoComplete| s.name);

        // /* 2. 지역이름 키워드 데이터 리스트 */
        // let location_auto_complete: Vec<String> = self.query_service.get_locations_name().await?;

        // let mut auto_complete_location: Vec<AutoComplete> = 
        //     self.to_auto_complete_list(location_auto_complete, |s: String| s);

        // auto_completes.append(&mut auto_complete_store);
        // auto_completes.append(&mut auto_complete_location);

        /* 자동완성 키워드 데이터 리스트 */
        //let mut auto_completes: Vec<AutoComplete> = Vec::new();
        
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


    #[doc = "지하철 관련 정보를 정적색인해주는 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn subway_static_index(&self, index_schedule: IndexSchedules) -> Result<(), anyhow::Error> {

        /* 현재기준 UTC 시간 */
        let cur_utc_date: NaiveDateTime = get_current_utc_naive_datetime();
        
        let subway_infos: Vec<SubwayInfo> = self.query_service.get_subway_info_by_batch(&index_schedule).await?;
        let subway_infos_es: Vec<SubwayInfoEs> = subway_infos.iter().map(|s| s.to_es()).collect();

        /* Elasticsearch 에 데이터 색인. */
        self.es_query_service
            .post_indexing_data_by_bulk_static::<SubwayInfoEs>(
                &index_schedule,
                &subway_infos_es,
            )
            .await?;
        
        /* 색인시간 최신화 */
        self.query_service
            .update_recent_date_to_elastic_index_info(&index_schedule, cur_utc_date)
            .await?;

        info!("Subway - Static Create Indexing: {}", subway_infos.len());

        Ok(())
    }
}