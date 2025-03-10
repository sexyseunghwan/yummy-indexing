use crate::common::*;



#[doc = ""]
#[derive(Debug, FromQueryResult)]
pub struct StorTypesResult {
    pub seq: i32,
    pub major_type: i32,
    pub sub_type: i32
}


// #[doc = ""]
// #[derive(Debug, Serialize, new)]
// pub struct StorTypes {
//     pub seq: i32,
//     pub major_types: i32,
//     pub sub_types: i32
// }