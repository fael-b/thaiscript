use ::entity::review_outcome;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_review_outcome(
        db: &DbConn,
        form_data: review_outcome::Model,
    ) -> Result<review_outcome::ActiveModel, DbErr> {
        review_outcome::ActiveModel {
            id: NotSet,
            letter_variant_id: Set(form_data.letter_variant_id.to_owned()),
            correct: Set(form_data.correct),
            review_type: Set(form_data.review_type.to_owned()),
            date: Set(form_data.date.to_owned()),
            ms_time_taken: Set(form_data.ms_time_taken.to_owned()),
        }
        .save(db)
        .await
    }
}
