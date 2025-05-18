use sea_orm::entity::prelude::*;

use crate::entity::location_city_tbl;
use crate::entity::location_county_tbl;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "location_city_tbl")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub location_city_code: i32,
    #[sea_orm(primary_key)]
    pub location_county_code: i32,
    pub location_city: String,
    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

// #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
// pub enum PrimaryKey {
//     LocationCityCode,
//     LocationCountyCode
// }

// impl PrimaryKeyTrait for PrimaryKey {
//     type ValueType = (i32, i32);

//     fn auto_increment() -> bool {
//         false
//     }
// }

// impl ColumnTrait for PrimaryKey {
//     fn def(&self) -> ColumnDef {
//         match self {
//             Self::LocationCityCode => ColumnType::Integer.def(),
//             Self::LocationCountyCode => ColumnType::Integer.def()
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
}

impl Related<location_county_tbl::Entity> for location_city_tbl::Entity {
    fn to() -> RelationDef {
        Relation::LocationCountyTbl.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
