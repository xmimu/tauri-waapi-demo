// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use waapi_rs::{ak, SubscriptionHandle, WaapiClient};

struct SubscriptionState {
    client: Option<WaapiClient>,
    handle: Option<SubscriptionHandle>,
}

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

#[tauri::command]
async fn subscribe_selection_start(
    app: AppHandle,
    state: State<'_, Mutex<SubscriptionState>>,
    return_str: Option<String>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.client.is_some() {
        return Err("订阅已开启".into());
    }
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
    let options = json!({ "return": return_array });
    let client = WaapiClient::connect().await.map_err(|e| e.to_string())?;
    let app_clone = app.clone();
    let handle = client
        .subscribe(
            ak::wwise::ui::SELECTION_CHANGED,
            Some(options),
            move |_args, kwargs| {
                let payload: Value = json!({
                    "args": serde_json::to_value(_args).unwrap_or(Value::Null),
                    "kwargs": serde_json::to_value(kwargs).unwrap_or(Value::Null),
                });
                let _ = app_clone.emit("waapi-selection-changed", payload);
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    guard.client = Some(client);
    guard.handle = Some(handle);
    Ok(())
}

#[tauri::command]
async fn subscribe_selection_stop(
    state: State<'_, Mutex<SubscriptionState>>,
) -> Result<(), String> {
    let (client, handle) = {
        let mut guard = state.lock().await;
        (
            guard.client.take(),
            guard.handle.take(),
        )
    };
    if let Some(h) = handle {
        h.unsubscribe().await.map_err(|e| e.to_string())?;
    }
    if let Some(c) = client {
        c.disconnect().await;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SubscriptionState {
            client: None,
            handle: None,
        }))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_wwise_info,
            get_project_info,
            object_get,
            subscribe_selection_start,
            subscribe_selection_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
