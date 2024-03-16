use ::entity::{letter_variant, letter_variant::Entity as LetterVariant};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_letter_variant_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<Option<letter_variant::Model>, DbErr> {
        LetterVariant::find_by_id(id).one(db).await
    }
}
