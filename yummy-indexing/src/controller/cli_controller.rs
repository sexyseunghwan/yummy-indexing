use crate::common::*;

use crate::handler::process_handler::*;

use crate::configuration::index_schedules_config::*;

use crate::services::es_query_service::*;
use crate::services::query_service::*;

#[doc = ""]
/// # Arguments
/// * `index_schedules` - 인덱스 스케쥴 객체
///
/// # Returns
/// * Result<(), anyhow::Error>
pub async fn centralized_cli_loop(
    index_schedules: IndexSchedulesConfig,
) -> Result<(), anyhow::Error> {
    let query_service: QueryServicePub = QueryServicePub::new();
    let es_query_service: EsQueryServicePub = EsQueryServicePub::new();
    let process_handler: ProcessHandler<QueryServicePub, EsQueryServicePub> =
        ProcessHandler::new(query_service, es_query_service);

    let mut stdout: io::Stdout = io::stdout();

    let mut idx: i32 = 0;

    writeln!(
        stdout,
        "[================ Yummy Indexing CLI ================]"
    )
    .expect("[Error][centralized_cli_loop] Standard Output Error. 1");

    writeln!(stdout, "Select the index you want to perform.")
        .expect("[Error][centralized_cli_loop] Standard Output Error. 2");

    for index in index_schedules.index() {
        idx += 1;
        writeln!(
            stdout,
            "[{}] {:?} - {:?}",
            idx,
            index.index_name(),
            index.indexing_type
        )
        .expect("[Error][centralized_cli_loop] Standard Output Error. 3");
    }

    loop {
        writeln!(stdout, "\n").expect("[Error][centralized_cli_loop] Standard Output Error. 4");
        write!(stdout, "Please enter your number: ")
            .expect("[Error][centralized_cli_loop] Standard Output Error. 5");
        stdout
            .flush()
            .expect("[Error][centralized_cli_loop] Standard Output Error. 6");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("[Error][centralized_cli_loop] Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(number) => {
                if number > 0 && number <= idx {
                    let index: &IndexSchedules =
                        index_schedules.index().get((number - 1) as usize).unwrap();

                    /* 여기서 색인 작업을 진행해준다. */
                    // match self.main_task(index.clone()).await {
                    //     Ok(_) => (),
                    //     Err(e) => {
                    //         error!("[Error][cli_indexing_task() -> main_task()] {:?}", e);
                    //         writeln!(stdout, "Index failed.").unwrap();
                    //         break;
                    //     }
                    // }

                    writeln!(stdout, "Indexing operation completed.").unwrap();
                    break;
                } else {
                    writeln!(
                        stdout,
                        "Invalid input, please enter a number between 1 and {}.",
                        idx
                    )
                    .unwrap();
                }
            }
            Err(_) => {
                writeln!(stdout, "Invalid input, please enter a number.").unwrap();
            }
        }
    }

    Ok(())
}
