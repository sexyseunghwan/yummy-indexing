use crate::common::*;

use crate::repository::es_repository::*;

use crate::utils_module::io_utils::*;
use crate::utils_module::time_utils::*;

#[async_trait]
pub trait EsQueryService {
    async fn post_indexing_data_by_bulk<T: Serialize + Send + Sync + Debug>(
        &self,
        index_alias_name: &str,
        index_settings_path: &str,
        data: &Vec<T>,
    ) -> Result<(), anyhow::Error>;

    async fn get_test(&self) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct EsQueryServicePub;

#[async_trait]
impl EsQueryService for EsQueryServicePub {
    #[doc = "static index function"]
    /// # Arguments
    /// * `index_alias_name` - alias for index
    /// * `index_settings_path` - File path for setting index schema
    /// * `data` - Vector information to be indexed
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn post_indexing_data_by_bulk<T: Serialize + Send + Sync + Debug>(
        &self,
        index_alias_name: &str,
        index_settings_path: &str,
        data: &Vec<T>,
    ) -> Result<(), anyhow::Error> {
        let es_conn: ElasticConnGuard = get_elastic_guard_conn().await?;

        /* Put today's date time on the index you want to create. */
        let curr_time: String = get_current_kor_naive_datetime()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let new_index_name: String = format!("{}-{}", index_alias_name, curr_time);

        let json_body: Value = read_json_from_file(index_settings_path)?;
        es_conn.create_index(&new_index_name, &json_body).await?;

        /* Bulk post the data to the index above at once. */
        es_conn.bulk_indexing_query(&new_index_name, data).await?;
        
        /* 해당 인덱스가 있는지 없는지 확인해준다. */
        let index_exists_yn: bool = match es_conn.check_index_exist(index_alias_name).await {
            Ok(_index_exists_yn) => true,
            Err(e) => {
                error!("[Error][post_indexing_data_by_bulk()] An index starting with that name does not exist.: {}, {:?}", index_alias_name, e);
                false
            }
        };

        if index_exists_yn {
            /* 기존 인덱스가 존재하는 경우 */
            let alias_resp: Value = es_conn
                .get_indexes_mapping_by_alias(index_alias_name)
                .await?;

            let old_index_name: String;
            
            if let Some(first_key) = alias_resp.as_object().and_then(|map| map.keys().next()) {
                old_index_name = first_key.to_string();
            } else {
                return Err(anyhow!("[Error][post_indexing_data_by_bulk()] Failed to extract index name within 'index-alias'"));
            }
    
            es_conn
                .update_index_alias(index_alias_name, &new_index_name, &old_index_name)
                .await?;
            
            es_conn.delete_query(&old_index_name).await?;
        } else {
            /* 기존 인덱스가 존재하지 않는 경우 -> 새로운 인덱스를 생성해준다. */
            es_conn
                .create_index_alias(index_alias_name, &new_index_name)
                .await?;
        }
        
        /* Functions to enable search immediately after index */
        es_conn.refresh_index(index_alias_name).await?;

        Ok(())
    }


    async fn get_test(&self) -> Result<(), anyhow::Error> {

        let es_conn: ElasticConnGuard = get_elastic_guard_conn().await?;

        let test = es_conn.check_index_exist("kakrftel").await?;

        println!("{:?}", test);

        Ok(())
    }
}
