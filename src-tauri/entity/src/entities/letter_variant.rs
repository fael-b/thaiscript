//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "letter_variant")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub letter: String,
    pub learning_order: i32,
    pub category: String,
    pub group: String,
    pub position: String,
    pub romanization: String,
    pub example_word: String,
    pub example_word_explanation: String,
    pub example_word_transliteration: String,
    pub example_word_emoji: String,
    pub similar_words: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::review_outcome::Entity")]
    ReviewOutcome,
}

impl Related<super::review_outcome::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReviewOutcome.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
