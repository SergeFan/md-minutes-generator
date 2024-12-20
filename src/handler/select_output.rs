use leptos::prelude::{RwSignal, Set};

use crate::handler::invoke_without_args;

pub async fn select_output(markdown_path: RwSignal<String>) {
    let selected_path = invoke_without_args("select_path")
        .await
        .as_string()
        .unwrap();

    if !selected_path.is_empty() {
        markdown_path.set(selected_path)
    }
}
