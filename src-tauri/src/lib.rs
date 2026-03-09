// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::{json, Value};
use waapi_rs::{ak, WaapiClient};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_wwise_info() -> Result<Value, String> {
    let client = WaapiClient::connect()
        .await
        .map_err(|e| e.to_string())?;
    let result = client
        .call(ak::wwise::core::GET_INFO, None, None)
        .await
        .map_err(|e| e.to_string())?;
    let info = result.unwrap_or(Value::Object(serde_json::Map::new()));
    client.disconnect().await;
    Ok(info)
}

#[tauri::command]
async fn get_project_info() -> Result<Value, String> {
    let client = WaapiClient::connect()
        .await
        .map_err(|e| e.to_string())?;
    let result = client
        .call(ak::wwise::core::GET_PROJECT_INFO, None, None)
        .await
        .map_err(|e| e.to_string())?;
    let info = result.unwrap_or(Value::Object(serde_json::Map::new()));
    client.disconnect().await;
    Ok(info)
}

#[tauri::command]
async fn object_get(waql: String, return_str: Option<String>) -> Result<Value, String> {
    let return_array: Vec<String> = match return_str.as_deref() {
        None => vec!["id".into(), "name".into(), "type".into(), "notes".into()],
        Some(s) if s.trim().is_empty() => {
            vec!["id".into(), "name".into(), "type".into(), "notes".into()]
        }
        Some(s) => s
            .split_whitespace()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect(),
    };
    let args = json!({ "waql": waql });
    let options = json!({ "return": return_array });

    let client = WaapiClient::connect()
        .await
        .map_err(|e| e.to_string())?;
    let result = client
        .call(ak::wwise::core::OBJECT_GET, Some(args), Some(options))
        .await
        .map_err(|e| e.to_string())?;
    let out = result.unwrap_or(Value::Object(serde_json::Map::new()));
    client.disconnect().await;
    Ok(out)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_wwise_info, get_project_info, object_get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
