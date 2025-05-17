use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult, Serialize, Setters)]
pub struct AutoComplete {
    pub seq: i32,
    pub name: String
}