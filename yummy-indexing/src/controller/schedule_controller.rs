use rand::seq::index;

use crate::common::*;

use crate::configuration::index_schedules_config::*;

use crate::services::es_query_service::*;
use crate::services::query_service::*;

use crate::handler::process_handler::*;

#[doc = "인덱스 색인 메인 스케쥴러 함수"]
/// # Arguments
/// * `index_schedules` - 인덱스 스케쥴 객체
///
/// # Returns
/// * Result<(), anyhow::Error>
pub async fn centralized_schedule_loop(
    index_schedules: IndexSchedulesConfig,
) -> Result<(), anyhow::Error> {
    /* Service 및 Handler 초기화 */
    let query_service: QueryServicePub = QueryServicePub::new();
    let es_query_service: EsQueryServicePub = EsQueryServicePub::new();
    let process_handler: Arc<ProcessHandler<QueryServicePub, EsQueryServicePub>> =
        Arc::new(ProcessHandler::new(query_service, es_query_service));

    let mut interval: Interval = tokio::time::interval(Duration::from_millis(1000)); /* 1초마다 체크 */

    /* 한국 표준시 GMT + 9 */
    let kst_offset: FixedOffset = match FixedOffset::east_opt(9 * 3600) {
        Some(kst_offset) => kst_offset,
        None => {
            error!("[Error][main_schedule_task()] There was a problem initializing 'kst_offset'.");
            panic!("[Error][main_schedule_task()] There was a problem initializing 'kst_offset'.");
        }
    };

    /* cron 스케줄 index_schedules */
    let schedule_map: HashMap<String, Schedule> = index_schedules
        .index
        .iter()
        .map(|index| {
            let schedule: Schedule = Schedule::from_str(&index.time)
                .expect("[Error][schedule_controller->centralized_schedule_loop] Invalid CRON expression in config");
            (index.index_name.clone() + &index.function_name, schedule) /* 복합 키 사용 */ 
        })
        .collect();

    // for (key, value) in &schedule_map {
    //     println!("{}: {}", key, value);
    // }

    loop {
        interval.tick().await;

        let now: DateTime<Utc> = Utc::now();
        let now_kst: DateTime<FixedOffset> = now.with_timezone(&kst_offset);

        for index in index_schedules.index() {
            let key: String = format!("{}{}", index.index_name(), index.function_name());

            if let Some(schedule) = schedule_map.get(&key) {
                if let Some(next) = schedule.upcoming(kst_offset).take(1).next() {
                    if (next - now_kst).num_seconds() < 1 {
                        let arc_handler: Arc<ProcessHandler<QueryServicePub, EsQueryServicePub>> =
                            Arc::clone(&process_handler);
                        let index_clone: IndexSchedules = index.clone();

                        tokio::spawn(async move {
                            if let Err(e) = arc_handler.main_task_schedule(index_clone).await {
                                error!("[Error][centralized_schedule_loop] Error: {:?}", e);
                            }
                        });
                    }
                }
            }
        }
    }
}
