use crate::AppState;
use entity::letter_variant::Model as LetterVariant;
use entity::review_outcome::Model as ReviewOutcome;
use rand::Rng;
use rip_shuffle::RipShuffleSequential;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thaiscript_db_service::Query;
use tokio::join;

const LEARNED_PERCENTAGE_THRESHOLD: f32 = 0.6;

#[tauri::command]
pub async fn get_next_reviews(state: tauri::State<'_, AppState>) -> Result<Vec<Review>, String> {
    let db = state.db.clone();

    let (every_letter_variant, learning_letter_variants, latest_review_outcome) = join!(
        Query::find_all_letter_variants(&db),
        Query::find_currently_learning_letter_variants(&db),
        Query::find_latest_review_outcome(&db)
    );
    let every_letter_variant = every_letter_variant.map_err(|e| e.to_string())?;
    let learning_letter_variants = learning_letter_variants.map_err(|e| e.to_string())?;
    let latest_review_outcome = latest_review_outcome.map_err(|e| e.to_string())?;

    // println!(
    //     "first learning_letter_variant: {:?}",
    //     learning_letter_variants.first().clone()
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

                // If the user has reviewed less than 3 times, don't consider it as learned
                if relevant_review_outcomes.len() < 3 {
                    // println!(
                    //     "relevant_review_outcomes.len() < 3: {}",
                    //     relevant_review_outcomes.len()
                    // );
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
    // learning_letter_variants.seq_shuffle(&mut rand::thread_rng());

    if !has_reached_threshold {
        let mut reviews = get_weighted_reviews(learning_letter_variants, every_letter_variant, 5);
        // let mut reviews: Vec<Review> = learning_letter_variants
        //     .iter()
        //     .take(5)
        //     .map(|(letter_variant, _review_outcomes)| {
        //         let review_type = "letter-to-romanization"; // TODO: Randomize review type
        //         let options =
        //             get_random_distinct_options(every_letter_variant.clone(), letter_variant, 4);

        //         Review {
        //             review_type: review_type.to_string(),
        //             letter_variant: letter_variant.clone(),
        //             options,
        //         }
        //     })
        //     .collect();

        // Shuffle the reviews
        reviews.seq_shuffle(&mut rand::thread_rng());

        // Move the first review if it's the same as the latest review
        move_first_review_if_same_as_latest_review(latest_review_outcome, &mut reviews);

        return Ok(reviews);
    }

    // Get next letter variant to learn
    // Determining the index can be complex since the learning order can change between versions of the app
    let mut learning_letter_orders = learning_letter_variants
        .iter()
        .map(|(letter_variant, _review_outcomes)| letter_variant.learning_order as usize)
        .collect::<Vec<usize>>();
    learning_letter_orders.sort();
    let mut next_letter_variant_index: usize = 0;
    for learning_order in learning_letter_orders.into_iter() {
        if learning_order == next_letter_variant_index + 1 {
            next_letter_variant_index += 1;
        } else {
            break;
        }
    }

    let next_letter_variant = every_letter_variant[next_letter_variant_index].clone();
    let next_letter_variant_review = Review {
        review_type: "initial".to_owned(),
        letter_variant: next_letter_variant,
        options: Vec::with_capacity(0),
    };

    // Get 4 random letter variants to review
    let mut random_reviews =
        get_weighted_reviews(learning_letter_variants, every_letter_variant, 4);
    // let mut random_reviews: Vec<Review> = learning_letter_variants
    //     .iter()
    //     .take(4)
    //     .map(|(letter_variant, _review_outcomes)| {
    //         let options =
    //             get_random_distinct_options(every_letter_variant.clone(), letter_variant, 4);

    //         Review {
    //             review_type: "letter-to-romanization".to_owned(),
    //             letter_variant: letter_variant.clone(),
    //             options,
    //         }
    //     })
    //     .collect();

    random_reviews.insert(0, next_letter_variant_review);

    Ok(random_reviews)
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

fn get_weighted_reviews(
    mut letter_variants: Vec<(LetterVariant, Vec<ReviewOutcome>)>,
    every_letter_variant: Vec<LetterVariant>,
    count: usize,
) -> Vec<Review> {
    let mut reviews = Vec::with_capacity(count);

    // Letter variants are weighted based on:
    // 1. The learning percentage of the letter variant (lower percentage = higher weight)
    // 2. The last time the letter variant was reviewed (older = higher weight)
    // 3. The number of times the letter variant has been reviewed (less = higher weight)

    // (the letter_variants are already sorted by the time of last review (newest first))

    let max_review_count = letter_variants
        .iter()
        .map(|(_, review_outcomes)| review_outcomes.len())
        .max()
        .unwrap();

    letter_variants.sort_by(
        |(_letter_variant_a, review_outcomes_a), (_letter_variant_b, review_outcomes_b)| {
            let learning_percentage_a = review_outcomes_a
                .iter()
                .filter(|review_outcome| review_outcome.review_type != "initial")
                .filter(|review_outcome| review_outcome.correct)
                .count() as f32
                / review_outcomes_a.len() as f32;

            let learning_percentage_b = review_outcomes_b
                .iter()
                .filter(|review_outcome| review_outcome.review_type != "initial")
                .filter(|review_outcome| review_outcome.correct)
                .count() as f32
                / review_outcomes_b.len() as f32;

            let review_count_a = review_outcomes_a.len();
            let normalized_review_count_a = review_count_a as f32 / max_review_count as f32;
            let review_count_b = review_outcomes_b.len();
            let normalized_review_count_b = review_count_b as f32 / max_review_count as f32;

            // Weights are averaged to get a single f32 value
            let weight_a = (learning_percentage_a + normalized_review_count_a) / 2.5;
            let weight_b = (learning_percentage_b + normalized_review_count_b + 0.5) / 2.5;

            weight_a.partial_cmp(&weight_b).unwrap()
        },
    );

    let random_chance = 0.65;
    for (letter_variant, _review_outcomes) in letter_variants.iter() {
        // Randomly choose whether to include the letter variant
        let letter_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);
        if letter_chance > random_chance {
            continue;
        }

        let review_type = "letter-to-romanization"; // TODO: Randomize review type
        let options = get_random_distinct_options(every_letter_variant.clone(), &letter_variant, 4);

        reviews.push(Review {
            review_type: review_type.to_string(),
            letter_variant: letter_variant.clone(),
            options,
        });
    }

    if reviews.len() < count {
        let filtered_letter_variants = letter_variants
            .into_iter()
            .filter(|(letter_variant, _)| {
                reviews
                    .iter()
                    .all(|review| review.letter_variant.id != letter_variant.id)
            })
            .collect();
        let mut remaining_reviews = get_weighted_reviews(
            filtered_letter_variants,
            every_letter_variant,
            count - reviews.len(),
        );
        reviews.append(&mut remaining_reviews);
    }

    reviews
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
