use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};
use thaw::{Flex, FlexJustify};

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <Flex justify=FlexJustify::Center>
            <a href="https://tauri.app" target="_blank">
                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://leptos.dev" target="_blank">
                <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo" />
            </a>
        </Flex>
    }
}
