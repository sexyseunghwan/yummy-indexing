use sea_orm::entity::prelude::*;

use crate::entity::reviews;
use crate::entity::store_reviews;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true, column_type = "BigUnsigned")]
    pub review_id: u64,

    #[sea_orm(column_type = "TinyUnsigned")]
    pub rating: u8,

    pub title: String,

    pub content: String,

    /* MySQL DATETIME ↔ SeaORM DateTime (time::PrimitiveDateTime) */
    pub visit_date: DateTime,

    #[sea_orm(column_type = "Unsigned", default_value = 0)]
    pub photos_count: u32,

    #[sea_orm(column_type = "Unsigned", default_value = 0)]
    pub helpful_count: u32,

    #[sea_orm(column_type = "Unsigned", default_value = 0)]
    pub reported_count: u32,

    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::store_reviews::Entity")]
    StoreReview,
}

impl Related<store_reviews::Entity> for reviews::Entity {
    fn to() -> RelationDef {
        Relation::StoreReview.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
