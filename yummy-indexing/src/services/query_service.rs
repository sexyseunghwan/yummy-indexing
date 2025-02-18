use crate::common::*;

use crate::models::store_to_elastic::*;

use crate::repository::mysql_repository::*;

use crate::utils_module::time_utils::*;

use crate::entity::{recommend_tbl, store, store_recommend_tbl, zero_possible_market};

pub trait QueryService {
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<Vec<StoreResult>, anyhow::Error>;

    fn get_distinct_store_table(
        &self,
        stores: &Vec<StoreResult>,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    #[doc = "색인할 Store 정보를 조회해주는 함수 -> batch"]
    /// # Arguments
    /// * `batch_size` - 한번에 DB 에서 가져올 데이터 개수
    ///
    /// # Returns
    /// * Result<Vec<StoreResult>, anyhow::Error>
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<Vec<StoreResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let mut total_store_list: Vec<StoreResult> = Vec::new();
        let mut last_seq: Option<i32> = None;

        let cur_kor_date: NaiveDateTime = get_current_kor_naive_datetime();

        loop {
            let mut query: Select<store::Entity> = store::Entity::find()
                .left_join(zero_possible_market::Entity)
                .left_join(store_recommend_tbl::Entity)
                .join(
                    JoinType::LeftJoin,
                    store_recommend_tbl::Relation::RecommendTbl
                        .def()
                        .on_condition(move |_r, _| {
                            Condition::all()
                                .add(Expr::col(recommend_tbl::Column::RecommendYn).eq("Y"))
                                .add(
                                    Expr::col(store_recommend_tbl::Column::RecommendEndDt)
                                        .gt(cur_kor_date),
                                )
                        }),
                )
                .order_by_asc(store::Column::Seq)
                .limit(batch_size as u64)
                .select_only()
                .columns([
                    store::Column::Seq,
                    store::Column::Name,
                    store::Column::Type,
                    store::Column::Address,
                    store::Column::Lat,
                    store::Column::Lng,
                ])
                .expr_as(
                    Expr::col((
                        zero_possible_market::Entity,
                        zero_possible_market::Column::Name,
                    ))
                    .is_not_null(),
                    "zero_possible",
                )
                .column_as(recommend_tbl::Column::RecommendName, "recommend_name");

            if let Some(seq) = last_seq {
                query = query.filter(store::Column::Seq.gt(seq)); /* `seq`가 마지막 값보다 큰 데이터 가져오기 */
            }

            let mut store_results: Vec<StoreResult> = query.into_model().all(db).await?;

            if store_results.is_empty() {
                break;
            }

            total_store_list.append(&mut store_results);
            last_seq = total_store_list.last().map(|s| s.seq);
        }

        Ok(total_store_list)
    }

    #[doc = "색인할 Store 정보를 조회해주는 함수 -> 중복 제거"]
    /// # Arguments
    /// * `stores` - store 데이터 객체 리스트
    ///
    /// # Returns
    /// * Result<Vec<DistinctStoreResult>, anyhow::Error>
    fn get_distinct_store_table(
        &self,
        stores: &Vec<StoreResult>,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error> {
        let mut store_map: HashMap<i32, DistinctStoreResult> = HashMap::new();
        let cur_time_utc: String = get_str_curdatetime_utc();

        for store in stores {
            store_map
                .entry(store.seq)
                .and_modify(|existing| {
                    if let Some(recommend) = &store.recommend_name {
                        existing.recommend_names.push(recommend.to_string());
                    }
                })
                .or_insert_with(|| {
                    DistinctStoreResult::new(
                        cur_time_utc.to_string(),
                        store.seq,
                        store.name.clone(),
                        store.r#type.clone(),
                        store.address.clone(),
                        store.lat,
                        store.lng,
                        store.zero_possible,
                        store.recommend_name.clone().map_or(vec![], |r| vec![r]),
                    )
                });
        }

        let result: Vec<DistinctStoreResult> = store_map.into_values().collect();
        Ok(result)
    }
}
