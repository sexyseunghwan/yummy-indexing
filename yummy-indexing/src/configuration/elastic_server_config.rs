use crate::common::*;

static ELASTIC_CONFIG: once_cells<ElasticServerConfig> = once_cells::new();

#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct ElasticServerConfig {
    pub elastic_host: Vec<String>,
    pub elastic_id: Option<String>,
    pub elastic_pw: Option<String>,
    pub elastic_pool_cnt: i32,
}

#[doc = "ElasticServerConfig 정보를 전역적으로 초기화해주는 함수"]
pub fn init_elastic_config() {
    let config: ElasticServerConfig = ElasticServerConfig::new();

    ELASTIC_CONFIG
        .set(config)
        .expect("[Error][init_config] CONFIG is already initialized");
}

#[doc = "전역화된 ElasticServerConfig 정보를 안전하게 사용가능하게 하는 함수"]
pub fn get_elastic_config() -> &'static ElasticServerConfig {
    ELASTIC_CONFIG
        .get()
        .expect("[Error][get_config] CONFIG not initialized")
}

impl ElasticServerConfig {
    pub fn new() -> Self {
        let elastic_host: Vec<String> = env::var("ES_DB_URL")
            .expect("[ENV file read Error][initialize_db_clients()] 'ES_DB_URL' must be set")
            .split(",")
            .map(|s| s.to_string())
            .collect();

        let elastic_id: String = env::var("ES_ID")
            .expect("[ENV file read Error][initialize_db_clients()] 'ES_ID' must be set");
        let elastic_pw: String = env::var("ES_PW")
            .expect("[ENV file read Error][initialize_db_clients()] 'ES_PW' must be set");
        let elastic_pool_cnt: i32 = env::var("ES_POOL_CNT")
            .expect("[ENV file read Error][initialize_db_clients()] 'ES_POOL_CNT' must be set")
            .parse::<i32>()
            .expect("[Error][ElasticServerConfig->new] ");

        if elastic_pool_cnt > 10 {
            panic!("[Error][ElasticServerConfig->new] The number of elastic search connection pools cannot exceed 10.");
        }

        Self {
            elastic_host,
            elastic_id: Some(elastic_id),
            elastic_pw: Some(elastic_pw),
            elastic_pool_cnt,
        }
    }
}
