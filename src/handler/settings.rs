use leptos::prelude::{GetUntracked, RwSignal};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::handler::{invoke, invoke_without_args};

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    #[serde(rename(serialize = "directGeneration"))]
    pub direct_generation: bool,
}

pub async fn get_app_settings() -> AppSettings {
    let js_value = invoke_without_args("get_app_settings").await;
    let settings: AppSettings = from_value(js_value).unwrap();

    settings
}

pub async fn reset_app_settings() {
    invoke_without_args("reset_app_settings").await;
}

pub async fn set_app_settings(language: RwSignal<String>, direct_generation: RwSignal<bool>) {
    let args = to_value(&AppSettings {
        language: language.get_untracked(),
        direct_generation: direct_generation.get_untracked(),
    })
    .unwrap();

    invoke("set_app_settings", args).await;
}
