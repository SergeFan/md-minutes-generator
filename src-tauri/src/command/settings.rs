use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    language: String,
    direct_generation: bool,
}

#[tauri::command]
pub fn get_app_settings(app_handle: AppHandle) -> AppSettings {
    let store = app_handle.store("store.json").unwrap();

    let language = store
        .get("language")
        .expect("No language setting is found.");
    let direct_generation = store
        .get("direct_generation")
        .expect("No direct generation setting is found.");

    AppSettings {
        language: language.get("value").unwrap().as_str().unwrap().to_string(),
        direct_generation: direct_generation.get("value").unwrap().as_bool().unwrap(),
    }
}

#[tauri::command]
pub fn reset_app_settings(app_handle: AppHandle) {
    app_handle.store("store.json").unwrap().reset();

    app_handle.request_restart();
}

#[tauri::command]
pub fn set_app_settings(app_handle: AppHandle, language: &str, direct_generation: bool) {
    let store = app_handle.store("store.json").unwrap();

    store.set("language", json!({"value": language}));
    store.set("direct_generation", json!({"value": direct_generation}));

    app_handle.request_restart();
}
