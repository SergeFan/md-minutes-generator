mod command;
mod file_process;

use directories::UserDirs;
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreBuilder;

use crate::command::settings::*;

#[tauri::command]
fn get_desktop_dir() -> String {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(desktop_dir) = user_dirs.desktop_dir() {
            return desktop_dir
                .to_path_buf()
                .into_os_string()
                .into_string()
                .unwrap();
        }
    }

    String::new()
}

#[tauri::command]
async fn select_file(app_handle: AppHandle) -> String {
    if let Some(user_dirs) = UserDirs::new() {
        let file_path = app_handle
            .dialog()
            .file()
            .set_directory(user_dirs.home_dir())
            .add_filter("Excel", &["xlsx", "xls"])
            .blocking_pick_file();

        return match file_path {
            None => String::new(),
            Some(file_path) => file_path.to_string(),
        };
    };

    String::new()
}

#[tauri::command]
async fn select_path(app_handle: AppHandle) -> String {
    if let Some(user_dirs) = UserDirs::new() {
        let file_path = app_handle
            .dialog()
            .file()
            .set_directory(user_dirs.home_dir())
            .blocking_pick_folder();

        return match file_path {
            None => String::new(),
            Some(file_path) => file_path.to_string(),
        };
    };

    String::new()
}

#[tauri::command]
fn read_excel(input: &str, _: Option<&str>, _: Option<&str>) -> Vec<String> {
    if let Ok(sheet_names) = file_process::read_excel(input) {
        return sheet_names;
    }

    vec![]
}

#[tauri::command]
fn generate_markdown(app_handle: AppHandle, input: &str, output: &str, sheet: &str) -> bool {
    if output.is_empty() || sheet.is_empty() {
        return false;
    }

    file_process::generate_markdown(app_handle, input, output, sheet).is_ok()
}

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
            select_file,
            select_path,
            set_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
