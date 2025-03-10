use crate::common::*;



#[doc = ""]
#[derive(Debug, FromQueryResult)]
pub struct StoreTypesResult {
    pub seq: i32,
    pub major_type: i32,
    pub sub_type: i32
}


#[doc = ""]
#[derive(Debug, Serialize, new)]
pub struct StoreTypesMap {
    pub store_type_major_map: HashMap<i32, Vec<i32>>,
    pub store_type_sub_map: HashMap<i32, Vec<i32>>
}