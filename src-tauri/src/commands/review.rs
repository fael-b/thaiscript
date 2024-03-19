use crate::AppState;
use entity::letter_variant::Model as LetterVariant;
use entity::review_outcome::Model as ReviewOutcome;
use rip_shuffle::RipShuffleSequential;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thaiscript_db_service::Query;
use tokio::join;

const LEARNED_PERCENTAGE_THRESHOLD: f32 = 0.8;

#[tauri::command]
pub async fn get_next_reviews(state: tauri::State<'_, AppState>) -> Result<Vec<Review>, String> {
    let db = state.db.clone();

    let (every_letter_variant, learning_letter_variants, latest_review_outcome) = join!(
        Query::find_all_letter_variants(&db),
        Query::find_currently_learning_letter_variants(&db),
        Query::find_latest_review_outcome(&db)
    );
    let every_letter_variant = every_letter_variant.map_err(|e| e.to_string())?;
    let mut learning_letter_variants = learning_letter_variants.map_err(|e| e.to_string())?;
    let latest_review_outcome = latest_review_outcome.map_err(|e| e.to_string())?;

    // println!(
    //     "learning_letter_variants len: {}",
    //     learning_letter_variants.len()
    // );

    if learning_letter_variants.len() == 0 {
        // First time user
        // Get the first 5 letter variants
        // Return them as initial reviews
        let initial_reviews = every_letter_variant
            .iter()
            .take(5)
            .map(|letter_variant| Review {
                review_type: "initial".to_owned(),
                letter_variant: letter_variant.clone(),
                options: Vec::with_capacity(0),
            })
            .collect();
        return Ok(initial_reviews);
    }

    // Check if the user has reached the learned percentage threshold for every currently learning letter variant
    // If so, introduce a new letter variant to learn
    // If not, continue reviewing the current letter variants
    let has_reached_threshold =
        learning_letter_variants
            .iter()
            .all(|(_letter_variant, review_outcomes)| {
                let relevant_review_outcomes = review_outcomes
                    .iter()
                    .filter(|review_outcome| review_outcome.review_type != "initial")
                    .collect::<Vec<&ReviewOutcome>>();

                // If the user has reviewed less than 5 times, don't consider it as learned
                if relevant_review_outcomes.len() > 5 {
                    return false;
                }

                let learned_count = review_outcomes
                    .iter()
                    .filter(|review_outcome| {
                        review_outcome.correct && review_outcome.review_type != "initial"
                    })
                    .count() as f32;
                let learned_percentage = learned_count / relevant_review_outcomes.len() as f32;
                let has_reached_threshold = learned_percentage >= LEARNED_PERCENTAGE_THRESHOLD;

                // println!(
                //     "learned_count: {}, learned_percentage: {}, has_reached_threshold: {}",
                //     learned_count, learned_percentage, has_reached_threshold
                // );

                return has_reached_threshold;
            });

    // Shuffle the learning letter variants
    learning_letter_variants.seq_shuffle(&mut rand::thread_rng());

    if !has_reached_threshold {
        let mut reviews: Vec<Review> = learning_letter_variants
            .iter()
            .take(5)
            .map(|(letter_variant, _review_outcomes)| {
                let review_type = "letter-to-romanization"; // TODO: Randomize review type
                let options =
                    get_random_distinct_options(every_letter_variant.clone(), letter_variant, 4);

                Review {
                    review_type: review_type.to_string(),
                    letter_variant: letter_variant.clone(),
                    options,
                }
            })
            .collect();

        // Shuffle the reviews
        reviews.seq_shuffle(&mut rand::thread_rng());

        // Move the first review if it's the same as the latest review
        move_first_review_if_same_as_latest_review(latest_review_outcome, &mut reviews);

        return Ok(reviews);
    }

    // Get next letter variant to learn
    let next_letter_variant_index = learning_letter_variants.len() + 1;
    let next_letter_variant = every_letter_variant[next_letter_variant_index].clone();
    let next_letter_variant_review = Review {
        review_type: "initial".to_owned(),
        letter_variant: next_letter_variant,
        options: Vec::with_capacity(0),
    };

    // Get 4 random letter variants to review
    let mut random_reviews: Vec<Review> = learning_letter_variants
        .iter()
        .take(4)
        .map(|(letter_variant, _review_outcomes)| {
            let options =
                get_random_distinct_options(every_letter_variant.clone(), letter_variant, 4);

            Review {
                review_type: "letter-to-romanization".to_owned(),
                letter_variant: letter_variant.clone(),
                options,
            }
        })
        .collect();

    random_reviews.insert(0, next_letter_variant_review);

    Ok(random_reviews)

    // OLD
    // every_letter_variant.seq_shuffle(&mut rand::thread_rng());

    // let reviews = every_letter_variant
    //     .iter()
    //     .take(5)
    //     .map(|letter_variant| {
    //         let review_type = "initial";
    //         let options =
    //             get_random_distinct_options(every_letter_variant.clone(), letter_variant, 4);

    //         Review {
    //             review_type: review_type.to_string(),
    //             letter_variant: letter_variant.clone(),
    //             options,
    //         }
    //     })
    //     .collect();

    // Ok(reviews)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    #[serde(rename = "type")]
    review_type: String,
    letter_variant: LetterVariant,
    options: Vec<LetterVariant>,
}

fn get_random_distinct_options(
    mut letter_variants: Vec<LetterVariant>,
    correct_option: &LetterVariant,
    num_options: usize,
) -> Vec<LetterVariant> {
    letter_variants.seq_shuffle(&mut rand::thread_rng());

    let mut options_romanizations = HashSet::new();
    options_romanizations.insert(correct_option.romanization.clone());

    let mut options = Vec::with_capacity(num_options);
    options.push(correct_option.clone());

    letter_variants
        .into_iter()
        .filter(|lv| lv.id != correct_option.id)
        .filter(|lv| {
            if options_romanizations.contains(&lv.romanization) {
                false
            } else {
                options_romanizations.insert(lv.romanization.clone());
                true
            }
        })
        .take(num_options - 1)
        .for_each(|lv| options.push(lv));

    options.seq_shuffle(&mut rand::thread_rng());

    options
}

fn move_first_review_if_same_as_latest_review(
    latest_review_outcome: Option<ReviewOutcome>,
    reviews: &mut Vec<Review>,
) {
    if let Some(latest_review_outcome) = latest_review_outcome {
        if let Some(first_review) = reviews.first() {
            if first_review.letter_variant.id == latest_review_outcome.letter_variant_id {
                let first_review = reviews.remove(0);
                reviews.push(first_review);
            }
        }
    }
}
