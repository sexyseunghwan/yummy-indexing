use diesel::sql_types::Decimal;

use crate::common::*;

use crate::repository::mysql_repository::*;

use crate::schema::store;

#[derive(Queryable, Debug, Insertable, AsChangeset, Getters)]
#[table_name = "store"]
#[getset(get = "pub")]
pub struct Store {
    pub seq: i32,
    pub name: String,
    pub address: String,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
    pub reg_dt: NaiveDateTime,
    pub chg_dt: NaiveDateTime,
    pub reg_id: String,
    pub chg_id: String
}