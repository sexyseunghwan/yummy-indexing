use crate::common::*;

use crate::repository::es_repository::*;

// use crate::models::consume_prodt_detail::*;
// use crate::models::consume_prodt_detail_es::*;
// use crate::models::consume_prodt_keyword::*;
// use crate::models::document_with_id::*;
// use crate::models::score_manager::*;

// use crate::utils_module::io_utils::*;
// use crate::utils_module::time_utils::*;

// use crate::configuration::elasitc_index_name::*;

#[async_trait]
pub trait EsQueryService {}

#[derive(Debug, new)]
pub struct EsQueryServicePub;

#[async_trait]
impl EsQueryService for EsQueryServicePub {}
