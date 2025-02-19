use crate::common::*;

use crate::services::es_query_service::*;
use crate::services::query_service::*;

use crate::configuration::{index_schedules_config::*, system_config::*};

use crate::models::store_to_elastic::*;

use crate::entity::{
    recommend_tbl, store, store_location_info_tbl, store_recommend_tbl, zero_possible_market,
};

use crate::utils_module::time_utils::*;

// use crate::configuration::elasitc_index_name::*;

#[derive(Debug, new)]
pub struct MainController<Q: QueryService, E: EsQueryService> {
    query_service: Q,
    es_query_service: E,
}

impl<Q: QueryService, E: EsQueryService> MainController<Q, E> {
    #[doc = "메인 스케쥴러 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_schedule_task(
        &self,
        index_schedule: IndexSchedules,
    ) -> Result<(), anyhow::Error> {
        let schedule: Schedule =
            Schedule::from_str(&index_schedule.time).expect("Failed to parse CRON expression");

        let system_config: Arc<SystemConfig> = get_system_config();

        let mut interval: Interval = tokio::time::interval(tokio::time::Duration::from_millis(
            system_config.schedule_term,
        ));

        /* 한국 표준시 GMT + 9 */
        let kst_offset: FixedOffset = match FixedOffset::east_opt(9 * 3600) {
            Some(kst_offset) => kst_offset,
            None => {
                error!(
                    "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
                );
                panic!(
                    "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
                );
            }
        };

        loop {
            interval.tick().await;

            let now: DateTime<Utc> = Utc::now();
            let kst_now: DateTime<FixedOffset> = now.with_timezone(&kst_offset); /* Converting UTC Current Time to KST */

            if let Some(next) = schedule.upcoming(kst_offset).take(1).next() {
                if (next - kst_now).num_seconds() < 1 {
                    match self.main_task(index_schedule.clone()).await {
                        Ok(_) => (),
                        Err(e) => {
                            error!("[Error][main_schedule_task() -> main_task()] {:?}", e);
                        }
                    }
                }
            }
        }
    }

    #[doc = "메인 작업 함수 -> 색인 진행 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_task(&self, index_schedule: IndexSchedules) -> Result<(), anyhow::Error> {
        let function_name: &str = index_schedule.function_name().as_str();

        match function_name {
            "store_static_index" => self.store_static_index(index_schedule).await?,
            "store_dynamic_index" => self.store_dynamic_index(index_schedule).await?,
            _ => {
                return Err(anyhow!(
                    "[Error][main_task()] The mapped function does not exist.: {}",
                    function_name
                ))
            }
        }

        //let index_alias_name: &String = index_schedule.index_name();

        /* 해당 색인이 static/dynamic 인지 확인 */
        // if index_schedule.indexing_type() == "static" {
        //     //self.es_query_service.post_indexing_data_by_bulk(index_alias_name, index_settings_path, data).await?;
        // } else if index_schedule.indexing_type() == "dynamic" {

        // } else {
        //     return Err(anyhow!("[Error][main_task()] Static index or dynamic index only."))
        // }

        /* 중복이 존재하는 store 리스트 */
        // let stores: Vec<StoreResult> = self.query_service.get_all_store_table(10).await?;

        // /* 중복을 제외한 store 리스트 */
        // let stores_distinct: Vec<DistinctStoreResult> =
        //     self.query_service.get_distinct_store_table(&stores)?;

        // // for elem in stores_distinct {
        // //     println!("{:?}", elem);
        // // }

        // /* Elasticsearch 색인 */
        // //let index_alias: &str = "yummy-index";
        // let index_alias: &str = "test-index";

        // /* index_alias 에 대한 인덱스가 없을경우 새롭게 생성 */
        // self.es_query_service
        //     .post_indexing_data_by_bulk::<DistinctStoreResult>(
        //         index_alias,
        //         "./indexing_settings/store_infos.json",
        //         &stores_distinct,
        //     )
        //     .await?;

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
        let index_alias_name: &String = index_schedule.index_name();
        let index_setting_path: &str = match index_schedule.setting_path() {
            Some(index_setting_path) => index_setting_path.as_str(),
            None => {
                return Err(anyhow!(
                    "[Error][store_static_index()] Please specify 'setting_path' for index"
                ))
            }
        };

        let sql_batch_size: usize = match index_schedule.sql_batch_size() {
            Some(sql_batch_size) => *sql_batch_size,
            None => {
                return Err(anyhow!(
                    "[Error][store_static_index()] Please specify 'sql_batch_size' for index"
                ))
            }
        };

        /* 중복이 존재하는 store 리스트 */
        let stores: Vec<StoreResult> = self
            .query_service
            .get_all_store_table(sql_batch_size)
            .await?;

        /* 중복을 제외한 store 리스트 */
        let stores_distinct: Vec<DistinctStoreResult> =
            self.query_service.get_distinct_store_table(&stores)?;

        self.es_query_service
            .post_indexing_data_by_bulk::<DistinctStoreResult>(
                index_alias_name,
                index_setting_path,
                &stores_distinct,
            )
            .await?;

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
        let index_name: &String = index_schedule.index_name();

        /* 일단, 검색엔진에 색인된 정보중에 가장 최근의 timestamp 정보를 가져와 준다. */
        let recent_index_datetime: NaiveDateTime = self
            .es_query_service
            .get_recent_index_datetime(index_name, "timestamp")
            .await?;

        /* 증분색인은 Create -> Update -> Delete 세단계로 나눠준다. */
        let cur_utc_date: NaiveDateTime = get_current_utc_naive_datetime(); /* 현재기준 UTC 시간 */

        /* 1. Create */

        /* 2. Update */

        /* 3. Delete */

        // /* 일단, 검색엔진에 색인된 정보중에 가장 최근의 timestamp 정보를 가져와 준다. */
        // let recent_index_datetime: NaiveDateTime = self
        //     .es_query_service
        //     .get_recent_index_datetime(index_name, "timestamp")
        //     .await?;

        // /* 해당 timestamp 정보를 기준으로 더 최근 데이터를 DB 에서 가져와준다. */
        // let recent_store_data: Vec<StoreResult> = self
        //     .query_service
        //     .get_updated_store_table(recent_index_datetime)
        //     .await?;

        // for elem in recent_store_data {
        //     println!("{:?}", elem);
        // }

        Ok(())
    }

    // #[doc = "정적색인 함수"]
    // /// # Arguments
    // /// * `index_schedule` - 인덱스 스케쥴 객체
    // ///
    // /// # Returns
    // /// * Result<(), anyhow::Error>
    // async fn static_index(&self, index_schedule: IndexSchedules) -> Result<(), anyhow::Error> {

    //     let index_alias_name: &String = index_schedule.index_name();

    //     Ok(())
    // }

    // #[doc = "동적색인 함수"]
    // /// # Arguments
    // /// * `index_schedule` - 인덱스 스케쥴 객체
    // ///
    // /// # Returns
    // /// * Result<(), anyhow::Error>
    // async fn dynamic_index(&self, index_schedule: IndexSchedules) -> Result<(), anyhow::Error> {

    //     Ok(())
    // }
}
