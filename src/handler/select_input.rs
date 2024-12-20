use crate::handler::invoke_without_args;
use leptos::prelude::{RwSignal, Set};

pub async fn select_input(file_path: RwSignal<String>) {
    let selected_file = invoke_without_args("select_file")
        .await
        .as_string()
        .unwrap();

    file_path.set(selected_file.to_owned());
}
