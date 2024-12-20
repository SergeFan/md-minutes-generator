use std::time::Duration;

use leptos::prelude::{Get, RwSignal};
use leptos::view;
use thaw::*;
use wasm_bindgen::JsValue;

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
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"Generation completed"</ToastTitle>
                            <ToastBody>"Markdown generation succeeded."</ToastBody>
                        </Toast>
                    }
                },
                ToastOptions::default()
                    .with_position(ToastPosition::Top)
                    .with_intent(ToastIntent::Success)
                    .with_timeout(Duration::from_secs(5)),
            );
        } else {
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"Generation failed"</ToastTitle>
                            <ToastBody>"Markdown generation has been cancelled."</ToastBody>
                        </Toast>
                    }
                },
                ToastOptions::default()
                    .with_position(ToastPosition::Top)
                    .with_intent(ToastIntent::Error)
                    .with_timeout(Duration::from_secs(5)),
            );
        }
    };
}
