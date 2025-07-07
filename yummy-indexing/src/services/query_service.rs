use crate::common::*;

use crate::configuration::index_schedules_config::*;

use crate::entity::category_tbl;
use crate::entity::store_category_tbl;
use crate::models::store_auto_complete::*;
use crate::models::store_to_elastic::*;
use crate::models::auto_search_keyword::*;


use crate::repository::mysql_repository::*;

use crate::utils_module::time_utils::*;

use crate::entity::{
    elastic_index_info_tbl, location_city_tbl, location_county_tbl, location_district_tbl,
    recommend_tbl, store, store_location_info_tbl, store_recommend_tbl,
    zero_possible_market, auto_search_keyword_tbl,store_location_road_info_tbl
};

pub trait QueryService {
    async fn get_store_by_batch(
        &self,
        batch_size: usize,
        query_filter: Condition,
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<StoreResult>, anyhow::Error>;
    async fn get_all_store_table(
        &self,
        index_schedule: &IndexSchedules,
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error>;
    async fn get_specific_store_table(
        &self,
        index_schedule: &IndexSchedules,
        cur_utc_date: NaiveDateTime,
        recent_datetime: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error>;
    fn get_distinct_store_table(
        &self,
        stores: &[StoreResult],
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error>;
    async fn get_recent_date_from_elastic_index_info(
        &self,
        index_schedule: &IndexSchedules,
    ) -> Result<NaiveDateTime, anyhow::Error>;
    async fn update_recent_date_to_elastic_index_info(
        &self,
        index_schedule: &IndexSchedules,
        new_datetime: NaiveDateTime,
    ) -> Result<(), anyhow::Error>;
    async fn get_store_name_by_batch(
        &self,
        index_schedule: &IndexSchedules,
    ) -> Result<Vec<StoreAutoComplete>, anyhow::Error>;
    async fn get_locations_name(&self) -> Result<Vec<String>, anyhow::Error>;
    async fn get_auto_search_keyword_by_batch(&self, index_schedule: &IndexSchedules) -> Result<Vec<AutoSearchKeyword>, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    #[doc = "store 색인 관련 배치 함수"]
    /// # Arguments
    /// * `batch_size` - 쿼리 배치 사이즈
    /// * `query_filter` - 쿼리 필터
    /// * `cur_utc_date` - 현재 시각
    ///
    /// # Returns
    /// * Result<Vec<StoreResult>, anyhow::Error>
    async fn get_store_by_batch(
        &self,
        batch_size: usize,
        query_filter: Condition,
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<StoreResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let mut total_store_list: Vec<StoreResult> = Vec::new();
        let mut last_seq: Option<i32> = None;

        loop {
            
            let mut query: Select<store::Entity> = store::Entity::find()
                .join(JoinType::InnerJoin,store::Relation::StoreLocationInfoTbl.def())
                .join(JoinType::InnerJoin, store::Relation::StoreLocationRoadInfoTbl.def())
                .join(JoinType::InnerJoin, store::Relation::StoreCategoryTbl.def())
                .join(JoinType::InnerJoin, store_category_tbl::Relation::CategoryTbl.def())
                .join(JoinType::LeftJoin, store::Relation::ZeroPossibleMarket.def())
                .join(JoinType::LeftJoin, store::Relation::StoreRecommendTbl.def())
                .join(
                    JoinType::LeftJoin,
                    store_recommend_tbl::Relation::RecommendTbl
                        .def()
                        .on_condition(move |_r, _| {
                            Condition::all()
                                .add(Expr::col(recommend_tbl::Column::RecommendYn).eq("Y"))
                                .add(
                                    Expr::col(store_recommend_tbl::Column::RecommendEndDt)
                                        .gt(cur_utc_date),
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
                    store::Column::Tel,
                    store::Column::Url,
                ])
                .expr_as(
                    Expr::case(
                        Expr::col((
                            zero_possible_market::Entity,
                            zero_possible_market::Column::UseYn,
                        ))
                        .eq("N"),
                        false,
                    )
                    .case(
                        Expr::col((
                            zero_possible_market::Entity,
                            zero_possible_market::Column::Name,
                        ))
                        .is_not_null(),
                        true,
                    )
                    .finally(false),
                    "zero_possible",
                )
                .column_as(store_location_info_tbl::Column::Address, "address")
                .column_as(store_location_info_tbl::Column::Lat, "lat")
                .column_as(store_location_info_tbl::Column::Lng, "lng")
                .column_as(store_location_road_info_tbl::Column::Address, "road_address")
                .column_as(recommend_tbl::Column::RecommendName, "recommend_name")
                .column_as(category_tbl::Column::CategoryGroupName, "category_group_name")
                .column_as(category_tbl::Column::CategoryGroupCode, "category_group_code")
                .column_as(category_tbl::Column::CategoryName, "category_name")
                .column_as(category_tbl::Column::CategoryIcon, "category_icon")
                .filter(query_filter.clone());


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

    #[doc = "색인할 Store 정보를 조회해주는 함수 -> 모든 정보를 가져와준다: 정적색인 용도"]
    /// # Arguments
    /// * `index_schedule` - index_schedule 정보
    /// * `cur_utc_date` - 현재 시각
    ///
    /// # Returns
    /// * Result<Vec<DistinctStoreResult>, anyhow::Error>
    async fn get_all_store_table(
        &self,
        index_schedule: &IndexSchedules,
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error> {
        let batch_size: usize = *index_schedule.es_batch_size();
        let query_filter: Condition =
            Condition::all().add(Expr::col((store::Entity, store::Column::UseYn)).eq("Y"));

        /* 중복이 존재하는 store 리스트 */
        let stores: Vec<StoreResult> = self
            .get_store_by_batch(batch_size, query_filter, cur_utc_date)
            .await?;

        /*
            중복을 제외한 store 리스트
            - 어떠한 중복을 말하는건지?
            -> recommend_names: 즉 추천내용 조인 중복 때문에 중복제거로직이 들어간다.
        */
        let stores_distinct: Vec<DistinctStoreResult> =
            self.get_distinct_store_table(&stores, cur_utc_date)?;

        Ok(stores_distinct)
    }

    #[doc = "색인할 Store 정보를 조회해주는 함수 -> 특정 정보를 가져와준다: 증분색인 용도"]
    /// # Arguments
    /// * `index_schedule` - index_schedule 정보
    /// * `cur_utc_date` - 현재 시각정보
    /// * `recent_datetime` - 가장 최근 색인 시각정보
    ///
    /// # Returns
    /// * Result<Vec<DistinctStoreResult>, anyhow::Error>
    async fn get_specific_store_table(
        &self,
        index_schedule: &IndexSchedules,
        cur_utc_date: NaiveDateTime,
        recent_datetime: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error> {
        let batch_size: usize = *index_schedule.es_batch_size();
        
        let query_filter: Condition = Condition::all()
            .add(Expr::col((store::Entity, store::Column::UseYn)).eq("Y"))
            .add(
                Condition::any()
                    .add(Expr::col((store::Entity, store::Column::ChgDt)).gt(recent_datetime))
                    .add(Expr::col((store::Entity, store::Column::RegDt)).gt(recent_datetime))
                    .add(
                        Expr::col((
                            zero_possible_market::Entity,
                            zero_possible_market::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            zero_possible_market::Entity,
                            zero_possible_market::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_recommend_tbl::Entity,
                            store_recommend_tbl::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_recommend_tbl::Entity,
                            store_recommend_tbl::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_location_info_tbl::Entity,
                            store_location_info_tbl::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_location_info_tbl::Entity,
                            store_location_info_tbl::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_location_road_info_tbl::Entity,
                            store_location_road_info_tbl::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_location_road_info_tbl::Entity,
                            store_location_road_info_tbl::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_category_tbl::Entity,
                            store_category_tbl::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            store_category_tbl::Entity,
                            store_category_tbl::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            category_tbl::Entity,
                            category_tbl::Column::ChgDt,
                        ))
                        .gt(recent_datetime),
                    )
                    .add(
                        Expr::col((
                            category_tbl::Entity,
                            category_tbl::Column::RegDt,
                        ))
                        .gt(recent_datetime),
                    )
                    
            );

        /* 중복이 존재하는 store 리스트 */
        let stores: Vec<StoreResult> = self
            .get_store_by_batch(batch_size, query_filter, cur_utc_date)
            .await?;

        /* 중복을 제외한 store 리스트 */
        let stores_distinct: Vec<DistinctStoreResult> =
            self.get_distinct_store_table(&stores, cur_utc_date)?;

        Ok(stores_distinct)
    }
    
    #[doc = "색인할 Store 정보를 조회해주는 함수 -> 중복 제거"]
    /// # Arguments
    /// * `stores` - store 데이터 객체 리스트
    /// * `cur_utc_date` - 현재 UTC 기준 시간 데이터
    ///
    /// # Returns
    /// * Result<Vec<DistinctStoreResult>, anyhow::Error>
    fn get_distinct_store_table(
        &self,
        stores: &[StoreResult],
        cur_utc_date: NaiveDateTime,
    ) -> Result<Vec<DistinctStoreResult>, anyhow::Error> {
        let mut store_map: HashMap<i32, DistinctStoreResult> = HashMap::new();
        let cur_time_utc: String = get_str_from_naive_datetime(cur_utc_date);

        for store in stores {
            store_map
                .entry(store.seq)
                .and_modify(|existing| {
                    if let Some(recommend) = &store.recommend_name {
                        existing.recommend_names.push(recommend.to_string());
                    }
                })
                .or_insert_with(|| {

                    let lat_f64: f64 = store.lat.to_f64().unwrap_or(0.0);
                    let lng_f64: f64 = store.lng.to_f64().unwrap_or(0.0);

                    DistinctStoreResult::new(
                        cur_time_utc.clone(),
                        store.seq,
                        store.name.clone(),
                        store.r#type.clone(),
                        store.address.clone(),
                        store.road_address.clone(),
                        store.lat,
                        store.lng,
                        store.zero_possible,
                        store.recommend_name.clone().map_or(vec![], |r| vec![r]),
                        store.tel.clone(),
                        store.url.clone(),
                        store.category_group_name.clone(),
                        store.category_group_code.clone(),
                        store.category_name.clone(),
                        GeoPoint::new( lat_f64, lng_f64),
                        Some(store.category_icon.clone().map_or("".to_string(), |s| s))
                    )
                });
        }

        let result: Vec<DistinctStoreResult> = store_map.into_values().collect();
        Ok(result)
    }

    #[doc = "특정 인덱스에서 가장 최근에 색인된 날짜/시간 정보를 가져와주는 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 정보
    ///
    /// # Returns
    /// * Result<NaiveDateTime, anyhow::Error>
    async fn get_recent_date_from_elastic_index_info(
        &self,
        index_schedule: &IndexSchedules,
    ) -> Result<NaiveDateTime, anyhow::Error> {
        let index_name: &String = index_schedule.index_name();

        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<elastic_index_info_tbl::Entity> = elastic_index_info_tbl::Entity::find()
            .filter(elastic_index_info_tbl::Column::IndexName.eq(index_name));

        let query_results: Vec<elastic_index_info_tbl::Model> = query.all(db).await?;

        if query_results.is_empty() {
            return Err(anyhow!(
                "[Error][get_recent_date_from_elastic_index_info()] query_results is EMPTY"
            ));
        }

        let recent_datetime: NaiveDateTime = query_results
            .get(0)
            .ok_or_else(|| anyhow!("[Error][get_recent_date_from_elastic_index_info()] The first element of 'query_results' does not exist."))?
            .chg_dt;

        Ok(recent_datetime)
    }

    #[doc = "elastic_index_info 테이블의 chg_dt 데이터를 update 해주는 함수 - 색인시간 최신화"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 정보
    /// * `new_datetime` - 새로운 날짜/시간 데이터
    ///
    /// # Returns
    /// * Result<NaiveDateTime, anyhow::Error>
    async fn update_recent_date_to_elastic_index_info(
        &self,
        index_schedule: &IndexSchedules,
        new_datetime: NaiveDateTime,
    ) -> Result<(), anyhow::Error> {
        let index_name: &String = index_schedule.index_name();

        let db: &DatabaseConnection = establish_connection().await;

        elastic_index_info_tbl::Entity::update_many()
            .col_expr(
                elastic_index_info_tbl::Column::ChgDt,
                Expr::value(new_datetime),
            )
            .filter(elastic_index_info_tbl::Column::IndexName.eq(index_name))
            .exec(db)
            .await?;

        Ok(())
    }

    #[doc = "상점이름만 리턴해주는 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 정보
    ///
    /// # Returns
    /// * Result<Vec<StoreAutoComplete>, anyhow::Error>
    async fn get_store_name_by_batch(
        &self,
        index_schedule: &IndexSchedules,
    ) -> Result<Vec<StoreAutoComplete>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let batch_size: usize = index_schedule.sql_batch_size;

        let mut total_auto_complete_list: Vec<StoreAutoComplete> = Vec::new();
        let mut last_seq: Option<i32> = None;

        loop {
            let mut query: Select<store::Entity> = store::Entity::find()
                .order_by_asc(store::Column::Seq)
                .limit(batch_size as u64)
                .select_only()
                .columns([store::Column::Seq, store::Column::Name])
                .filter(store::Column::UseYn.eq("Y"));

            if let Some(seq) = last_seq {
                query = query.filter(store::Column::Seq.gt(seq)); /* `seq`가 마지막 값보다 큰 데이터 가져오기 */
            }

            let mut auto_completes: Vec<StoreAutoComplete> = query.into_model().all(db).await?;

            if auto_completes.is_empty() {
                break;
            }

            total_auto_complete_list.append(&mut auto_completes);

            last_seq = total_auto_complete_list.last().map(|s| s.seq);
        }

        Ok(total_auto_complete_list)
    }

    #[doc = "지역이름만 리턴해주는 함수"]
    async fn get_locations_name(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut locations_list: Vec<String> = Vec::new();

        let db: &DatabaseConnection = establish_connection().await;

        /* 시/도 이름 */
        let county_query: Select<location_county_tbl::Entity> = location_county_tbl::Entity::find()
            .select_only()
            .column(location_county_tbl::Column::LocationCounty);

        let mut county_list: Vec<String> = county_query.into_tuple().all(db).await?;

        /* 구/군 이름 */
        let city_query: Select<location_city_tbl::Entity> = location_city_tbl::Entity::find()
            .select_only()
            .column(location_city_tbl::Column::LocationCity);

        let mut city_list: Vec<String> = city_query.into_tuple().all(db).await?;

        /* 음면동 이름 */
        let district_query: Select<location_district_tbl::Entity> =
            location_district_tbl::Entity::find()
                .select_only()
                .column(location_district_tbl::Column::LocationDistrict);

        let mut district_list: Vec<String> = district_query.into_tuple().all(db).await?;

        locations_list.append(&mut county_list);
        locations_list.append(&mut city_list);
        locations_list.append(&mut district_list);

        Ok(locations_list)
    }
    
    #[doc = "자동완성/연관검색어 데이터를 가져오기 위한 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 정보
    ///
    /// # Returns
    /// * Result<Vec<AutoSearchKeyword>, anyhow::Error>
    async fn get_auto_search_keyword_by_batch(&self, index_schedule: &IndexSchedules) -> Result<Vec<AutoSearchKeyword>, anyhow::Error> {
        
        let db: &DatabaseConnection = establish_connection().await;

        let batch_size: usize = index_schedule.sql_batch_size;
        let mut total_auto_search_keywords: Vec<AutoSearchKeyword> = Vec::new();
        let mut last_key: Option<String> = None;
        
        loop {

            let mut query: Select<auto_search_keyword_tbl::Entity> = auto_search_keyword_tbl::Entity::find()
                .order_by_asc(auto_search_keyword_tbl::Column::Keyword)
                .limit(batch_size as u64)
                .columns([
                    auto_search_keyword_tbl::Column::Keyword,
                    auto_search_keyword_tbl::Column::KeywordWeight
                ]);
            
            if let Some(ref last_key) = last_key {
                query = query.filter(auto_search_keyword_tbl::Column::Keyword.gt(last_key));
            }
            
            let mut select_list: Vec<AutoSearchKeyword> = query.into_model().all(db).await?;

            if select_list.is_empty() {
                break;
            }

            total_auto_search_keywords.append(&mut select_list);
            last_key = total_auto_search_keywords.last().map(|a| a.keyword.clone());
        }
        
        Ok(total_auto_search_keywords)
    }
}
