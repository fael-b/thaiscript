// Returns a TTS audio file from Google Translate
// The return type is Blob in JavaScript, which corresponds to Vec<u8> in Rust
#[tauri::command]
pub async fn speak(text: String) -> Result<Vec<u8>, String> {
    let response = reqwest::get(format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=th-TH&total=1&idx=0&textlen={}&client=tw-ob", text, text.len()))
        .await
        .map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    Ok(bytes.to_vec())
}
