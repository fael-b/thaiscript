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
        // let testing = LetterVariant::find()
        //     // .order_by(letter_variant::Column::Id, Order::Desc)
        //     .find_also_related(ReviewOutcome)
        //     .order_by(review_outcome::Column::Id, Order::Desc)
        //     .build(DbBackend::Sqlite)
        //     .to_string();

        // println!("test: {:?}", testing);

        let pairs: Vec<(letter_variant::Model, review_outcome::Model)> = LetterVariant::find()
            .find_also_related(ReviewOutcome)
            .order_by_desc(review_outcome::Column::Id)
            .all(db)
            .await
            .map(|letter_variants| {
                letter_variants
                    .into_iter()
                    .filter(|(_letter_variant, review_outcome)| review_outcome.is_some())
                    .map(|(letter_variant, review_outcome)| {
                        (letter_variant, review_outcome.unwrap())
                    })
                    .collect()
            })?;

        let pairings_map: std::collections::HashMap<
            letter_variant::Model,
            Vec<review_outcome::Model>,
        > = pairs.into_iter().fold(
            std::collections::HashMap::new(),
            |mut acc, (letter_variant, review_outcome)| {
                let review_outcomes = acc.entry(letter_variant).or_insert_with(Vec::new);
                if review_outcomes.len() < 50 {
                    review_outcomes.push(review_outcome);
                }
                acc
            },
        );

        let mut sorted_pairs: Vec<(letter_variant::Model, Vec<review_outcome::Model>)> =
            pairings_map.into_iter().collect();
        sorted_pairs.sort_by(|(_, a), (_, b)| {
            let a = a.first().unwrap();
            let b = b.first().unwrap();
            b.id.cmp(&a.id)
        });

        Ok(sorted_pairs)
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
