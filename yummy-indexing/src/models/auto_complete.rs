use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult, Serialize, Setters, new)]
pub struct AutoComplete {
    pub name: String,
    pub name_chosung: String,
    pub keyword_weight: i32,
}
