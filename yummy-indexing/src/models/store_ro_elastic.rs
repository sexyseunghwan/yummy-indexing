use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult)]
pub struct StoreResult {
    name: String,
}