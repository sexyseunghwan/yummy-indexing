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
              2025-05-15 Seunghwan Shin       # [v.2.4.0]
                                                1) Elasticsearch connection pool 세마포어로 관리
                                                2) Centralized Polling 방식으로 전환
              2025-06-20 Seunghwan Shin       # [v.2.5.0] 테이블 구조변화에 따른 색인 구조 변경
              2025-06-25 Seunghwan Shin       # [v.2.6.0] 위치정보를 double -> geo_point 로 전환
              2025-07-07 Seunghwan Shin       # [v.2.7.0] 음식점 아이콘 색인 추가              
*/
mod common;
use common::*;

mod utils_module;
use configuration::system_config::*;

use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod repository;

mod services;

mod controller;
use controller::cli_controller::*;
use controller::schedule_controller::*;

mod handler;

mod configuration;
use configuration::elastic_server_config::*;
use configuration::index_schedules_config::*;

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

    match compile_type {
        "schedule" => match centralized_schedule_loop(index_schdules).await {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
            }
        },
        "cli" => match centralized_cli_loop(index_schdules).await {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
            }
        },
        other => {
            error!(
                "[Error][main()] Invalid COMPILE_TYPE: '{}'. Must be 'schedule' or 'cli'.",
                other
            );
            panic!("[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'.");
        }
    }
}
