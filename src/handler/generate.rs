use leptos::prelude::{GetUntracked, RwSignal};
use leptos_i18n::I18nContext;
use thaw::*;
use wasm_bindgen::JsValue;

use crate::component::toast::dispatch_toast;
use crate::handler::{invoke, GenerationOptions};
use crate::i18n::Locale;

pub async fn generate_markdown(
    file_path: RwSignal<String>,
    markdown_path: RwSignal<String>,
    selected_worksheet: RwSignal<String>,
    i18n: I18nContext<Locale>,
    toaster: ToasterInjection,
) {
    let args = serde_wasm_bindgen::to_value(&GenerationOptions {
        input: file_path.get_untracked(),
        output: Some(markdown_path.get_untracked()),
        sheet: Some(selected_worksheet.get_untracked()),
    })
    .unwrap();

    let js_value: JsValue = invoke("generate_markdown", args).await;

    if js_value.as_bool().unwrap_or(false) {
        dispatch_toast(i18n, toaster, ToastIntent::Success);
    } else {
        dispatch_toast(i18n, toaster, ToastIntent::Error);
    }
}
