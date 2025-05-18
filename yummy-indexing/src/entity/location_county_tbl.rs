use sea_orm::entity::prelude::*;

use crate::entity::location_city_tbl;
use crate::entity::location_county_tbl;
use crate::entity::location_district_tbl;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "location_county_tbl")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub location_county_code: i32,
    pub location_county: String,
    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::location_city_tbl::Entity")]
    LocationCityTbl,
    #[sea_orm(has_one = "super::location_district_tbl::Entity")]
    LocationDistrictTbl,
}

impl Related<location_city_tbl::Entity> for location_county_tbl::Entity {
    fn to() -> RelationDef {
        Relation::LocationCityTbl.def()
    }
}

impl Related<location_district_tbl::Entity> for location_county_tbl::Entity {
    fn to() -> RelationDef {
        Relation::LocationDistrictTbl.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
