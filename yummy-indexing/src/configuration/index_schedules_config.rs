use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct IndexSchedules {
    pub index_name: String,
    pub time: String,
    pub indexing_type: String,
    pub setting_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct IndexSchedulesConfig {
    pub index: Vec<IndexSchedules>,
}
