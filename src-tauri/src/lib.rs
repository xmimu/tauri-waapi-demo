// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use waapi_rs::{ak, SubscriptionHandle, WaapiClient};

/// Single persistent WAAPI connection shared by all commands.
/// Multiple concurrent WaapiClient connections cause the new connect() calls
/// to block indefinitely when a subscription is already active, making other
/// pages' buttons unresponsive. Using one shared client serializes operations
/// through the Mutex and avoids this issue.
struct AppState {
    client: Option<WaapiClient>,
    sub_handle: Option<SubscriptionHandle>,
}

impl Drop for AppState {
    fn drop(&mut self) {
        // 确保顺序：先释放订阅句柄，再释放连接（兜底，应对崩溃或强制退出）
        let _ = self.sub_handle.take();
        let _ = self.client.take();
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_wwise_info(state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut guard = state.lock().await;
    if guard.client.is_none() {
        guard.client = Some(WaapiClient::connect().await.map_err(|e| e.to_string())?);
    }
    let result = guard
        .client
        .as_ref()
        .unwrap()
        .call(ak::wwise::core::GET_INFO, None, None)
        .await;
    match result {
        Ok(v) => Ok(v.unwrap_or(Value::Object(serde_json::Map::new()))),
        Err(e) => {
            guard.client = None;
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_project_info(state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut guard = state.lock().await;
    if guard.client.is_none() {
        guard.client = Some(WaapiClient::connect().await.map_err(|e| e.to_string())?);
    }
    let result = guard
        .client
        .as_ref()
        .unwrap()
        .call(ak::wwise::core::GET_PROJECT_INFO, None, None)
        .await;
    match result {
        Ok(v) => Ok(v.unwrap_or(Value::Object(serde_json::Map::new()))),
        Err(e) => {
            guard.client = None;
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn object_get(
    state: State<'_, Mutex<AppState>>,
    waql: String,
    return_str: Option<String>,
) -> Result<Value, String> {
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

    let mut guard = state.lock().await;
    if guard.client.is_none() {
        guard.client = Some(WaapiClient::connect().await.map_err(|e| e.to_string())?);
    }
    let result = guard
        .client
        .as_ref()
        .unwrap()
        .call(ak::wwise::core::OBJECT_GET, Some(args), Some(options))
        .await;
    match result {
        Ok(v) => Ok(v.unwrap_or(Value::Object(serde_json::Map::new()))),
        Err(e) => {
            guard.client = None;
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn subscribe_selection_start(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    return_str: Option<String>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.sub_handle.is_some() {
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
    if guard.client.is_none() {
        guard.client = Some(WaapiClient::connect().await.map_err(|e| e.to_string())?);
    }
    let app_clone = app.clone();
    let handle = guard
        .client
        .as_ref()
        .unwrap()
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
        .await;
    match handle {
        Ok(h) => {
            guard.sub_handle = Some(h);
            Ok(())
        }
        Err(e) => {
            guard.client = None;
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn subscribe_selection_stop(
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let handle = {
        let mut guard = state.lock().await;
        guard.sub_handle.take()
        // client stays connected for reuse by other commands
    };
    if let Some(h) = handle {
        h.unsubscribe().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState {
            client: None,
            sub_handle: None,
        }))
        .setup(|app| {
            let app_handle = app.handle().clone();
            app.get_webview_window("main")
                .unwrap()
                .on_window_event(move |event| {
                    if let tauri::WindowEvent::Destroyed = event {
                        let state = app_handle.state::<Mutex<AppState>>();
                        tauri::async_runtime::block_on(async {
                            let mut guard = state.lock().await;
                            if let Some(h) = guard.sub_handle.take() {
                                let _ = h.unsubscribe().await;
                            }
                            guard.client.take();
                        });
                    }
                });
            Ok(())
        })
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
