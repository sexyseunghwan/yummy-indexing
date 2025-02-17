use sea_orm::sea_query::Alias;

use crate::common::*;

use crate::models::store_ro_elastic::*;

use crate::repository::mysql_repository::*;

use crate::utils_module::time_utils::*;

use crate::entity::
{
    store,
    zero_possible_market,
    store_recommend_tbl,
    recommend_tbl
};

pub trait QueryService {
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    #[doc = "색인할 Store 정보를 조회해주는 함수 -> batch"]
    /// # Arguments
    /// * `batch_size` - 한번에 DB 에서 가져올 데이터 개수
    ///
    /// # Returns
    /// * Result<Vec<store::Model>, anyhow::Error>
    async fn get_all_store_table(
        &self,
        batch_size: usize,
    ) -> Result<(), anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let mut store_list: Vec<store::Model> = Vec::new();
        let mut last_seq: Option<i32> = None;

        let cur_kor_date: NaiveDateTime = get_current_kor_naive_datetime();

        // let store_alias = Alias::new("s");
        // let zero_possible_market_alias = Alias::new("zpm");
        // let store_recommend_tbl_alias = Alias::new("sr");
        // let recommend_tbl_alias = Alias::new("r");
        

        let mut query = store::Entity::find()
            .left_join(zero_possible_market::Entity)
            .left_join(store_recommend_tbl::Entity)
            .select_only()
            .columns([
                store::Column::Name
            ]);
    
        let stores: Vec<StoreResult> = query
            .into_model()
            .all(db)
            .await?;
            
        //println!("{:?}", stores);
            
        for elem in stores {
            println!("{:?}", elem);
        }
        
        // loop {



        //     // let mut query = store::Entity::find()
        //     //     .left_join(zero_possible_market::Entity)
        //     //     .left_join(store_recommend_tbl::Entity)
        //     //     .left_join(recommend_tbl::Entity.on_condition(
        //     //         |r, _| r.column(recommend_tbl::Column::RecommendYn).eq("Y")
        //     //             .and(recommend_tbl::Column::RecommendEndDt.gt(cur_kor_date))
        //     //     ))
        //     //     .select_only()
        //     //     .columns([
        //     //         store::Column::Name,
        //     //         store::Column::Type,
        //     //         store::Column::Address,
        //     //         store::Column::Lat,
        //     //         store::Column::Lng,
        //     //     ])
        //     //     .column_as(recommend_tbl::Column::RecommendName, "recommend_name")
        //     //     .expr_as(
        //     //         Expr::col((zero_possible_market::Entity, zero_possible_market::Column::Name))
        //     //             .is_not_null(),
        //     //         "zero_possible"
        //     //     )
        //     //     .order_by_asc(store::Column::Seq)
        //     //     .limit(batch_size as u64);

        //     // let mut query = store::Entity::find()
        //     //     .join(
        //     //         JoinType::LeftJoin, 
        //     //         zero_possible_market::Entity::belongs_to(store::Entity)
        //     //             .from(zero_possible_market::Column::Seq)
        //     //             .to(store::Column::Seq)
        //     //             .into(),
        //     //     )
        //     //     .join(
        //     //         JoinType::LeftJoin, 
        //     //         store_recommend_tbl::Entity::belongs_to(store::Entity)
        //     //             .from(store_recommend_tbl::Column::Seq)
        //     //             .to(store::Column::Seq)
        //     //             .into(),
        //     //     )
        //     //     .join(
        //     //         JoinType::LeftJoin, 
        //     //         recommend_tbl::Entity::belongs_to(store_recommend_tbl::Entity)
        //     //             .from(recommend_tbl::Column::RecommendSeq)
        //     //             .to(store_recommend_tbl::Column::RecommendSeq)
        //     //             .into()
        //     //     )
        //     //     .filter(recommend_tbl::Column::RecommendYn.eq("Y"))
        //     //     .filter(store_recommend_tbl::Column::RecommendEndDt.gt(cur_kor_date))
        //     //     .order_by_asc(store::Column::Seq) /* seq 기준 정렬 */
        //     //     .limit(batch_size as u64)
        //     //     .select_only()
        //     //     .columns([
        //     //         store::Column::Name,
        //     //         store::Column::Type,
        //     //         store::Column::Address,
        //     //         store::Column::Lat,
        //     //         store::Column::Lng,
        //     //     ])
        //     //     .column_as(
        //     //         Expr::col((recommend_tbl::Entity, recommend_tbl::Column::RecommendName)), // ✅ 조인된 테이블의 컬럼을 `column_as()`로 지정
        //     //         "recommend_name"
        //     //     )
        //     //     .expr_as(
        //     //         Expr::case(
        //     //             Expr::col((zero_possible_market::Entity, zero_possible_market::Column::Name))
        //     //                 .is_null(),
        //     //             false
        //     //         )
        //     //         .finally(true),
        //     //         "zero_possible"
        //     //     );
            
        //     // if let Some(seq) = last_seq {
        //     //     query = query.filter(store::Column::Seq.gt(seq)); /* `seq`가 마지막 값보다 큰 데이터 가져오기 */
        //     // }
            
        //     // let mut stores = query.all(db).await?;

        //     // for elem in &stores {
        //     //     println!("{:?}", elem);
        //     // }

            
        //     // if stores.is_empty() {
        //     //     break;
        //     // }
            
        //     // //store_list.append(&mut stores);

        //     // last_seq = stores.last().map(|s| s.seq);
        // }

        Ok(())
    }
}
