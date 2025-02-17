use crate::common::*;

#[derive(Debug, FromQueryResult)]
pub struct StoreResult {
    pub seq: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub address: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lng: Decimal,
    pub zero_possible: bool,
    pub recommend_name: Option<String>,
}