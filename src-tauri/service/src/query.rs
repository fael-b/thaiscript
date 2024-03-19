use ::entity::letter_variant::{self, Entity as LetterVariant};
use ::entity::review_outcome::{self, Entity as ReviewOutcome};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_all_letter_variants(
        db: &DbConn,
    ) -> Result<Vec<letter_variant::Model>, DbErr> {
        LetterVariant::find()
            .order_by_asc(letter_variant::Column::LearningOrder)
            .all(db)
            .await
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

    pub async fn find_currently_learning_letter_variants(
        db: &DbConn,
    ) -> Result<Vec<(letter_variant::Model, Vec<review_outcome::Model>)>, DbErr> {
        LetterVariant::find()
            .find_with_related(ReviewOutcome)
            .all(db)
            .await
            .map(|letter_variants| {
                letter_variants
                    .into_iter()
                    .filter(|(_letter_variant, review_outcomes)| review_outcomes.len() > 0)
                    .collect()
            })
    }

    pub async fn find_latest_review_outcome(
        db: &DbConn,
    ) -> Result<Option<review_outcome::Model>, DbErr> {
        ReviewOutcome::find()
            .order_by_desc(review_outcome::Column::Id)
            .one(db)
            .await
    }
}
