use serde_json::Value;
use tauri::State;
use tokio::sync::Mutex;
use waapi_rs::ak;

use crate::{state::AppState, waapi_client::call_json};

#[tauri::command]
pub async fn get_project_info(state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut guard = state.lock().await;
    call_json(&mut guard, ak::wwise::core::GET_PROJECT_INFO, None, None).await
}
