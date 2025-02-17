use crate::common::*;

// use crate::models::consume_prodt_detail::*;
// use crate::models::consume_prodt_keyword::*;

use crate::repository::mysql_repository::*;

use crate::entity::store;

// use crate::schema::zero_possible_market::*;
// use crate::schema::store_recommend_tbl::*;
// use crate::schema::recommend_tbl::*;

pub trait QueryService {
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<Vec<store::Model>, anyhow::Error>;
    //async fn test(&self) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    #[doc = ""]
    /// # Arguments
    /// * `batch_size` - 한번에 DB 에서 가져올 데이터 개수
    ///
    /// # Returns
    /// * Result<Vec<store::Model>, anyhow::Error>
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<Vec<store::Model>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let mut store_list: Vec<store::Model> = Vec::new();
        let mut last_seq: Option<i32> = None;

        loop {
            let mut query: Select<store::Entity> = store::Entity::find()
                .order_by_asc(store::Column::Seq) /* seq 기준 정렬 */
                .limit(batch_size as u64);

            if let Some(seq) = last_seq {
                query = query.filter(store::Column::Seq.gt(seq)); /* `seq`가 마지막 값보다 큰 데이터 가져오기 */
            }

            let mut stores: Vec<store::Model> = query.all(db).await?;

            if stores.is_empty() {
                break;
            }

            store_list.append(&mut stores);

            last_seq = stores.last().map(|s| s.seq);
        }

        Ok(store_list)
    }
}
