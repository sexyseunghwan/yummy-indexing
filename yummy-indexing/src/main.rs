/*
Author      : Seunghwan Shin
Create date : 2025-02-20
Description :

History     : 2025-02-20 Seunghwan Shin       # [v.1.0.0] first create
              2025-03-03 Seunghwan Shin       # [v.2.0.0] CLI 를 통해서 사용자가 직접 색인하는 기능 추가
              2025-03-12 Seunghwan Shin       # [v.2.1.0] 1) 증분색인 알고리즘 변경
                                                          2) 음식점 분류타입 색인에 추가
              2025-03-17 Seunghwan Shin       # [v.2.2.0] dotenv -> dotenvy 로 변경
              2025-05-07 Seunghwan Shin       # [v.2.3.0] store 테이블 tel, url 정보 색인에 추가
              2025-05-00 Seunghwan Shin       # [v.2.4.0]
                                                1) Elasticsearch connection pool 세마포어로 관리
                                                2) Centralized Polling 방식으로 전환
*/

mod common;
use common::*;

mod utils_module;
use configuration::system_config::*;
use controller::schedule_controller::centralized_schedule_loop;
use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod repository;

mod services;
use services::es_query_service::*;
use services::query_service::*;

mod controller;

mod handler;
use handler::process_handler::*;

mod configuration;
use configuration::elastic_server_config::*;
use configuration::index_schedules_config::*;
use configuration::system_config::*;

mod models;

mod env_configuration;
use env_configuration::env_config::*;

mod entity;

#[tokio::main]
async fn main() {
    set_global_logger();
    load_env();

    info!("Yummy Indexing Batch Program Start");

    /* Elasticsearch connection 정보 전역화 */
    init_elastic_config();

    let system_infos: Arc<SystemConfig> = get_system_config();
    let compile_type: &str = system_infos.complie_type().as_str();

    /* 모니터링 대상이 되는 색인될 인덱스 정보들 */
    let index_schdules: IndexSchedulesConfig =
        match read_toml_from_file::<IndexSchedulesConfig>(&INDEX_LIST_PATH) {
            Ok(index_schdules) => index_schdules,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e);
            }
        };

    centralized_schedule_loop(index_schdules).await.unwrap();

    // match compile_type {
    //     "schedule" => {

    //     }
    //     "cli" => {

    //     }
    //     other => {
    //         error!(
    //             "[Error][main()] Invalid COMPILE_TYPE: '{}'. Must be 'schedule' or 'cli'.",
    //             other
    //         );
    //         panic!(
    //             "[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'."
    //         );
    //     }
    // }

    // let query_service: Arc<QueryServicePub> = Arc::new(QueryServicePub::new());
    // let es_query_service: Arc<EsQueryServicePub> = Arc::new(EsQueryServicePub::new());
    // let controller: MainController<QueryServicePub, EsQueryServicePub>
    //     = MainController::new(query_service, es_query_service);

    /* 모니터링 대상이 되는 색인될 인덱스 정보들 */
    // let index_schdules: IndexSchedulesConfig =
    //     match read_toml_from_file::<IndexSchedulesConfig>(&INDEX_LIST_PATH) {
    //         Ok(index_schdules) => index_schdules,
    //         Err(e) => {
    //             error!("{:?}", e);
    //             panic!("{:?}", e);
    //         }
    //     };

    // match controller.main_task(compile_type).await {
    //     Ok(_) => (),
    //     Err(e) => {
    //         error!("[Error][main] {:?}", e);
    //     }
    // }

    /* 모니터링 대상이 되는 색인될 인덱스 정보들 */
    // let index_schdules: IndexSchedulesConfig =
    //     match read_toml_from_file::<IndexSchedulesConfig>(&INDEX_LIST_PATH) {
    //         Ok(index_schdules) => index_schdules,
    //         Err(e) => {
    //             error!("{:?}", e);
    //             panic!("{:?}", e);
    //         }
    //     };

    // if compile_type == "schedule" {
    //     /*
    //         [스케쥴 타입의 색인 프로그램]
    //         각 인덱스 별로 모니터링을 비동기적으로 실시해준다.
    //         스케쥴링 대기 작업 진행
    //     */
    //     for index in index_schdules.index {
    //         let index_clone: IndexSchedules = index.clone();

    //         let controller_arc_clone: Arc<MainController<QueryServicePub, EsQueryServicePub>> =
    //             Arc::clone(&controller_arc);

    //         tokio::spawn(async move {
    //             if let Err(e) = controller_arc_clone.main_schedule_task(index_clone).await {
    //                 error!("[Error][main_schedule_task] {:?}", e);
    //             }
    //         });
    //     }

    //     /* 모두 서브테스크로 실행되므로 아래와 같이 메인 태스크를 계속 유지시켜줘야 한다. */
    //     tokio::select! {
    //         _ = signal::ctrl_c() => {
    //             info!("Received Ctrl+C, shutting down...");
    //         }
    //     }
    // } else if compile_type == "cli" {
    //     /* [사용자 입력을 받아서 색인을 처리하는 프로그램] */
    //     match controller_arc.cli_indexing_task(index_schdules).await {
    //         Ok(_) => (),
    //         Err(e) => {
    //             error!("[Error][main()] {:?}", e);
    //             panic!("[Error][main()] {:?}", e);
    //         }
    //     }
    // } else {
    //     error!("[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'.");
    //     panic!("[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'.");
    // }

    /* test 코드 */
    // let index_schdule = index_schdules.index().get(0).unwrap();
    // println!("{:?}", index_schdule);
    // controller_arc
    //     .main_task(index_schdule.clone())
    //     .await
    //     .unwrap();
}
