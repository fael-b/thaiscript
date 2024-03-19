use crate::AppState;
use entity::letter_variant::Model as LetterVariant;
use std::collections::HashSet;
// use entity::review_outcome::Model as ReviewOutcome;
use rip_shuffle::RipShuffleSequential;
use serde::{Deserialize, Serialize};
use thaiscript_db_service::Query;

#[tauri::command]
pub async fn get_next_reviews(state: tauri::State<'_, AppState>) -> Result<Vec<Review>, String> {
    let db = state.db.clone();
    let mut letter_variants = Query::find_all_letter_variants(&db)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    letter_variants.seq_shuffle(&mut rand::thread_rng());

    let reviews = letter_variants
        .iter()
        .take(5)
        .map(|letter_variant| {
            let review_type = "initial";
            let options = get_random_distinct_options(letter_variants.clone(), letter_variant, 4);

            Review {
                review_type: review_type.to_string(),
                letter_variant: letter_variant.clone(),
                options,
            }
        })
        .collect();

    Ok(reviews)
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
