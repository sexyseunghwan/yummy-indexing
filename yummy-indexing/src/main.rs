/*
Author      : Seunghwan Shin
Create date : 2025-02-22
Description :

History     : 2025-02-22 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use common::*;

mod utils_module;
use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod repository;

mod services;
use services::es_query_service::*;
use services::query_service::*;

mod controller;
use controller::main_controller::*;

mod configuration;
use configuration::index_schedules_config::*;

mod models;

mod schema;

mod env_configuration;
use env_configuration::env_config::*;

#[tokio::main]
async fn main() {
    set_global_logger();
    dotenv().ok();

    info!("Yummy Indexing Batch Program Start");

    let query_service: QueryServicePub = QueryServicePub::new();
    let es_query_service: EsQueryServicePub = EsQueryServicePub::new();
    let controller_arc: Arc<MainController<QueryServicePub, EsQueryServicePub>> =
        Arc::new(MainController::new(query_service, es_query_service));

    /* 모니터링 대상이 되는 색인될 인덱스 정보들 */
    let index_schdules: IndexSchedulesConfig =
        match read_toml_from_file::<IndexSchedulesConfig>(&INDEX_LIST_PATH) {
            Ok(index_schdules) => index_schdules,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e);
            }
        };

    /*
        각 인덱스 별로 모니터링을 비동기적으로 실시해준다.
        스케쥴링 대기 작업 진행
    */
    for index in index_schdules.index {
        let index_clone: IndexSchedules = index.clone();

        let controller_arc_clone: Arc<MainController<QueryServicePub, EsQueryServicePub>> =
            Arc::clone(&controller_arc);

        tokio::spawn(async move {
            if let Err(e) = controller_arc_clone.main_schedule_task(index_clone).await {
                error!("[Error][main_schedule_task] {:?}", e);
            }
        });
    }

    println!("Hello, world!");
}
