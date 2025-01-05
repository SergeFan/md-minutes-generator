pub mod drag_drop;
pub mod generate;
pub mod path;
pub mod settings;

use chrono::{DateTime, Local};
use js_sys::try_iter;
use leptos::prelude::{GetUntracked, RwSignal, Set};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // Tauri event listener
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

#[derive(PartialEq)]
pub enum MatchResult {
    Match,
    Mismatch,
    WorksheetNotFound,
}

pub fn match_worksheet_name(selected_worksheet: String, date: DateTime<Local>) -> MatchResult {
    if selected_worksheet.is_empty() {
        return MatchResult::WorksheetNotFound;
    }

    if selected_worksheet == date.format("%Y%m%d").to_string() {
        return MatchResult::Match;
    }

    MatchResult::Mismatch
}

#[derive(Serialize, Deserialize)]
struct GenerationOptions {
    input: String,
    output: Option<String>,
    sheet: Option<String>,
}

pub async fn load_output_options(
    selected_file: RwSignal<String>,
    markdown_path: RwSignal<String>,
    worksheet_options: RwSignal<Vec<String>>,
    selected_worksheet: RwSignal<String>,
) {
    if markdown_path.get_untracked().is_empty() {
        markdown_path.set(
            invoke_without_args("get_desktop_dir")
                .await
                .as_string()
                .unwrap(),
        );
    }

    let args = serde_wasm_bindgen::to_value(&GenerationOptions {
        input: selected_file.get_untracked(),
        output: None,
        sheet: None,
    })
    .unwrap();
    let js_value: JsValue = invoke("read_excel", args).await;

    if let Ok(Some(js_iterator)) = try_iter(&js_value) {
        let options: Vec<String> = js_iterator
            .filter_map(|item| item.ok().unwrap().as_string())
            .collect();

        if let Some(first_option) = options.first() {
            selected_worksheet.set(first_option.to_owned());
        }

        worksheet_options.set(options);
    }
}
