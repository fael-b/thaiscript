use crate::AppState;
use entity::review_outcome::Model as ReviewOutcome;
use thaiscript_db_service::Mutation;

#[tauri::command]
pub async fn save_review_outcome(
    letter_variant_id: String,
    review_type: String,
    date: String,
    correct: bool,
    ms_time_taken: i32,
    state: tauri::State<'_, AppState>,
) -> Result<ReviewOutcome, String> {
    let db = state.db.clone();
    Mutation::create_review_outcome(
        &db,
        ReviewOutcome {
            id: 0,
            letter_variant_id,
            review_type,
            date,
            correct,
            ms_time_taken,
        },
    )
    .await
    .map_err(|e| e.to_string())
    .map(|active_model| active_model.try_into().expect("Failed to convert to model"))
}
