use tokio::sync::Mutex;

mod commands;
mod state;
mod waapi_client;

use commands::{
    info::get_wwise_info,
    project::get_project_info,
    query::object_get,
    subscription::{
        attach_window_destroy_cleanup, subscribe_selection_start, subscribe_selection_stop,
    },
};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState::new()))
        .setup(|app| {
            attach_window_destroy_cleanup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_wwise_info,
            get_project_info,
            object_get,
            subscribe_selection_start,
            subscribe_selection_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
