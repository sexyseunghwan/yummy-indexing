use crate::common::*;

// use crate::models::consume_prodt_detail::*;
// use crate::models::consume_prodt_keyword::*;

use crate::repository::mysql_repository::*;

use crate::schema::store::dsl::*;
use crate::schema::zero_possible_market::*;
use crate::schema::store_recommend_tbl::*;
use crate::schema::recommend_tbl::*;

pub trait QueryService {

}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    
}
