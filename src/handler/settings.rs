use leptos::leptos_dom::logging::console_log;
use leptos::prelude::{GetUntracked, RwSignal, Set};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

use crate::handler::{invoke, invoke_without_args};

#[derive(Serialize, Deserialize)]
struct AppSettings {
    language: String,
    #[serde(rename(serialize = "directGeneration"))]
    direct_generation: bool,
}

pub async fn get_app_settings(language: RwSignal<String>, direct_generation: RwSignal<bool>) {
    let js_value = invoke_without_args("get_app_settings").await;
    let settings: AppSettings = from_value(js_value).unwrap();

    language.set(settings.language);
    direct_generation.set(settings.direct_generation);

    // TODO: remove console log after feature completed
    console_log(format!("language is set to: {}", language.get_untracked()).as_str());
}

pub async fn reset_app_settings(language: RwSignal<String>, direct_generation: RwSignal<bool>) {
    let js_value = invoke_without_args("reset_app_settings").await;
    let settings: AppSettings = from_value(js_value).unwrap();

    // TODO: remove console log after feature completed
    console_log(format!("direct_generation is {}", settings.direct_generation).as_str());

    language.set(settings.language);
    direct_generation.set(settings.direct_generation);
}

pub async fn set_app_settings(language: RwSignal<String>, direct_generation: RwSignal<bool>) {
    let args = to_value(&AppSettings {
        language: language.get_untracked(),
        direct_generation: direct_generation.get_untracked(),
    })
    .unwrap();

    invoke("set_app_settings", args).await;
}
