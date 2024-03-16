#![allow(unused_imports)]
use ::entity::{letter_variant, letter_variant::Entity as LetterVariant};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_letter_variant(
        db: &DbConn,
        form_data: letter_variant::Model,
    ) -> Result<letter_variant::ActiveModel, DbErr> {
        letter_variant::ActiveModel {
            letter: Set(form_data.letter.to_owned()),
            // etc...
            ..Default::default()
        }
        .save(db)
        .await
    }
}
