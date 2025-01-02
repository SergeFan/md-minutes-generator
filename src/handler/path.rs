use leptos::prelude::{RwSignal, Set};

use crate::handler::invoke_without_args;

pub async fn select_input(file_path: RwSignal<String>) {
    let selected_file = invoke_without_args("select_input")
        .await
        .as_string()
        .unwrap();

    file_path.set(selected_file.to_owned());
}

pub async fn select_output(markdown_path: RwSignal<String>) {
    let selected_path = invoke_without_args("select_output")
        .await
        .as_string()
        .unwrap();

    if !selected_path.is_empty() {
        markdown_path.set(selected_path)
    }
}
