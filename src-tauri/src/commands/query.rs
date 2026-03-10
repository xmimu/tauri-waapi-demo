use serde_json::{json, Value};
use tauri::State;
use tokio::sync::Mutex;
use waapi_rs::ak;

use crate::{
    state::AppState,
    waapi_client::{call_json, parse_return_fields},
};

#[tauri::command]
pub async fn object_get(
    state: State<'_, Mutex<AppState>>,
    waql: String,
    return_str: Option<String>,
) -> Result<Value, String> {
    let args = json!({ "waql": waql });
    let options = json!({ "return": parse_return_fields(return_str.as_deref()) });

    let mut guard = state.lock().await;
    call_json(
        &mut guard,
        ak::wwise::core::OBJECT_GET,
        Some(args),
        Some(options),
    )
    .await
}
