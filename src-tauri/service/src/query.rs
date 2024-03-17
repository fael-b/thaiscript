use ::entity::letter_variant::{self, Entity as LetterVariant};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_all_letter_variants(
        db: &DbConn,
    ) -> Result<Vec<letter_variant::Model>, DbErr> {
        LetterVariant::find().all(db).await
    }

    pub async fn find_letter_variant_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<Option<letter_variant::Model>, DbErr> {
        LetterVariant::find_by_id(id).one(db).await
    }

    pub async fn find_letter_variants_by_category(
        db: &DbConn,
        category: String,
    ) -> Result<Vec<letter_variant::Model>, DbErr> {
        LetterVariant::find()
            .filter(letter_variant::Column::Category.eq(category))
            .all(db)
            .await
    }
}
