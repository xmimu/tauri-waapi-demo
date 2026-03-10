use serde_json::{json, Value};
use tauri::{App, AppHandle, Emitter, Manager, State, WindowEvent};
use tokio::sync::Mutex;
use waapi_rs::ak;

use crate::{
    state::AppState,
    waapi_client::{ensure_client, parse_return_fields},
};

const SUBSCRIPTION_ALREADY_STARTED: &str = "Subscription already started";

fn ensure_subscription_not_active(is_active: bool) -> Result<(), String> {
    if is_active {
        Err(SUBSCRIPTION_ALREADY_STARTED.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn subscribe_selection_start(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    return_str: Option<String>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    ensure_subscription_not_active(guard.sub_handle.is_some())?;

    let options = json!({
        "return": parse_return_fields(return_str.as_deref()),
    });

    ensure_client(&mut guard).await?;

    let app_clone = app.clone();
    let handle = guard
        .client
        .as_ref()
        .expect("client should exist after ensure_client")
        .subscribe(
            ak::wwise::ui::SELECTION_CHANGED,
            Some(options),
            move |args, kwargs| {
                let payload: Value = json!({
                    "args": serde_json::to_value(args).unwrap_or(Value::Null),
                    "kwargs": serde_json::to_value(kwargs).unwrap_or(Value::Null),
                });
                let _ = app_clone.emit("waapi-selection-changed", payload);
            },
        )
        .await;

    match handle {
        Ok(subscription_handle) => {
            guard.sub_handle = Some(subscription_handle);
            Ok(())
        }
        Err(err) => {
            guard.client = None;
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn subscribe_selection_stop(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let handle = {
        let mut guard = state.lock().await;
        guard.sub_handle.take()
    };

    if let Some(subscription_handle) = handle {
        subscription_handle
            .unsubscribe()
            .await
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

pub fn attach_window_destroy_cleanup(app: &App) {
    let app_handle = app.handle().clone();
    if let Some(window) = app.get_webview_window("main") {
        window.on_window_event(move |event| {
            if let WindowEvent::Destroyed = event {
                let state = app_handle.state::<Mutex<AppState>>();
                tauri::async_runtime::block_on(async {
                    let mut guard = state.lock().await;
                    if let Some(subscription_handle) = guard.sub_handle.take() {
                        let _ = subscription_handle.unsubscribe().await;
                    }
                    guard.client.take();
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{ensure_subscription_not_active, SUBSCRIPTION_ALREADY_STARTED};

    #[test]
    fn reject_duplicate_subscription_start() {
        let result = ensure_subscription_not_active(true);
        assert_eq!(
            result.expect_err("duplicate start should be rejected"),
            SUBSCRIPTION_ALREADY_STARTED
        );
    }
}
