mod command;
mod tool;

use serde_json::json;
use tauri_plugin_store::StoreBuilder;

use crate::command::file::*;
use crate::command::path::*;
use crate::command::settings::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            StoreBuilder::new(app, "store.json")
                .default("language", json!({"value": "en"}))
                .default("direct_generation", json!({"value": false}))
                .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_markdown,
            get_app_settings,
            get_desktop_dir,
            read_excel,
            reset_app_settings,
            select_input,
            select_output,
            set_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
