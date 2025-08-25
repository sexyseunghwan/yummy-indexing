use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult, Serialize, Setters, Clone, new)]
pub struct AutoSearchKeyword {
    pub keyword: String,
    pub keyword_weight: i32,
}
