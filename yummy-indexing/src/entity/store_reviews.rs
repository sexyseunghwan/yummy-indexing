use sea_orm::entity::prelude::*;

use crate::entity::reviews;
use crate::entity::store;
use crate::entity::store_reviews;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "store_reviews")]
pub struct Model {
    // 복합 PK: seq + review_id
    #[sea_orm(primary_key)]
    pub seq: i32,

    #[sea_orm(primary_key, column_type = "BigUnsigned")]
    pub review_id: u64,

    pub user_no: i32,

    // TINYINT UNSIGNED DEFAULT 0 → ActiveEnum으로 매핑
    #[sea_orm(column_type = "TinyUnsigned", default_value = 0)]
    pub visit_type: VisitType,

    #[sea_orm(column_type = "Unsigned")]
    pub visit_cnt: u32,

    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum VisitType {
    #[sea_orm(num_value = 0)]
    DineIn,

    #[sea_orm(num_value = 1)]
    Takeout,

    #[sea_orm(num_value = 2)]
    Delivery,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::Seq",
        to = "super::store::Column::Seq"
    )]
    Store,
    #[sea_orm(
        belongs_to = "super::reviews::Entity",
        from = "Column::ReviewId",
        to = "super::reviews::Column::ReviewId"
    )]
    Reviews,
}

impl Related<store::Entity> for store_reviews::Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<reviews::Entity> for store_reviews::Entity {
    fn to() -> RelationDef {
        Relation::Reviews.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
