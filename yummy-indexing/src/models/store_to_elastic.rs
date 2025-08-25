use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult)]
pub struct StoreResult {
    pub seq: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub address: Option<String>,
    pub road_address: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lng: Decimal,
    pub zero_possible: bool,
    pub recommend_name: Option<String>,
    pub tel: Option<String>,
    pub url: Option<String>,
    pub category_group_name: String,
    pub category_group_code: String,
    pub category_name: String,
    pub category_icon: Option<String>,
    pub avg_rating: Option<f64>,
    pub review_count: Option<i64>,
}

#[doc = "Elasticsearch 와 mapping 할 구조체"]
#[derive(Debug, Serialize, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct DistinctStoreResult {
    pub timestamp: String,
    pub seq: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub address: Option<String>,
    pub road_address: Option<String>,
    pub lat: Decimal,
    pub lng: Decimal,
    pub zero_possible: bool,
    pub recommend_names: Vec<String>,
    pub tel: Option<String>,
    pub url: Option<String>,
    pub category_group_name: String,
    pub category_group_code: String,
    pub category_name: String,
    pub location: GeoPoint,
    pub category_icon: Option<String>,
    pub avg_rating: Option<f64>,
    pub review_count: Option<i64>,
}

#[derive(Debug, Serialize, new)]
pub struct GeoPoint {
    lat: f64,
    lon: f64,
}
