use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체 - store 테이블 관련 - 음식점 이름만"]
#[derive(Debug, FromQueryResult, Serialize, Setters)]
pub struct StoreAutoComplete {
    pub seq: i32,
    pub name: String,
}
