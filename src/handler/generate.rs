use leptos::prelude::{Get, RwSignal};
use thaw::*;
use wasm_bindgen::JsValue;

use crate::component::toast::dispatch_toast;
use crate::handler::{invoke, FileArgs};

pub async fn generate(
    file_path: RwSignal<String>,
    markdown_path: RwSignal<String>,
    selected_worksheet: RwSignal<Option<String>>,
    toaster: ToasterInjection,
) {
    let selected_file = file_path.get();
    let selected_path = markdown_path.get();

    if let Some(selected_sheet) = selected_worksheet.get() {
        let args = serde_wasm_bindgen::to_value(&FileArgs {
            input: selected_file.as_str(),
            output: selected_path.as_str(),
            sheet: selected_sheet.as_str(),
        })
        .unwrap();

        let js_value: JsValue = invoke("generate_markdown", args).await;

        if js_value.as_bool().unwrap_or(false) {
            dispatch_toast(toaster, ToastIntent::Success);
        } else {
            dispatch_toast(toaster, ToastIntent::Error);
        }
    };
}
