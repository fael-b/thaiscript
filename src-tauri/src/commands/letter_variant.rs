use crate::AppState;
use entity::letter_variant::Model as LetterVariant;
use thaiscript_db_service::Query;

#[tauri::command]
pub async fn get_letter_variants_by_category(
    category: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<LetterVariant>, String> {
    let db = state.db.clone();
    Query::find_letter_variants_by_category(&db, category)
        .await
        .map_err(|e| e.to_string())
}
