use sea_orm::entity::prelude::*;

use crate::entity::location_city_tbl;
use crate::entity::location_county_tbl;
use crate::entity::location_district_tbl;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "location_district_tbl")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub location_district_code: i32,
    #[sea_orm(primary_key)]
    pub location_city_code: i32,
    #[sea_orm(primary_key)]
    pub location_county_code: i32,
    pub location_district: String,
    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

// #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
// pub enum PrimaryKey {
//     LocationCityCode,
//     LocationCountyCode,
//     LocationDistrictCode,
// }

// impl PrimaryKeyTrait for PrimaryKey {
//     type ValueType = (i32, i32, i32);

//     fn auto_increment() -> bool {
//         false
//     }
// }

// impl ColumnTrait for PrimaryKey {
//     fn def(&self) -> ColumnDef {
//         match self {
//             Self::LocationCityCode => ColumnType::Integer.def(),
//             Self::LocationCountyCode => ColumnType::Integer.def(),
//             Self::LocationDistrictCode => ColumnType::Integer.def(),
//         }
//     }
// }

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::location_county_tbl::Entity",
        from = "Column::LocationCountyCode",
        to = "super::location_county_tbl::Column::LocationCountyCode"
    )]
    LocationCountyTbl,
    #[sea_orm(
        belongs_to = "super::location_city_tbl::Entity",
        from = "Column::LocationCityCode",
        to = "super::location_city_tbl::Column::LocationCityCode"
    )]
    LocationCityTbl,
}

impl Related<location_county_tbl::Entity> for location_district_tbl::Entity {
    fn to() -> RelationDef {
        Relation::LocationCountyTbl.def()
    }
}

impl Related<location_city_tbl::Entity> for location_district_tbl::Entity {
    fn to() -> RelationDef {
        Relation::LocationCityTbl.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
