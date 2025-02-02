use leptos::prelude::*;
use leptos::{component, view, IntoView};
use thaw::*;

#[component]
pub fn Header(open_settings: RwSignal<bool>) -> impl IntoView {
    view! {
        <Flex vertical=true>
            <Flex justify=FlexJustify::End>
                <Button
                    attr:style="position: relative; top: 2em; right: 2em;"
                    icon=icondata::OcGearSm
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open_settings.set(true)
                />
            </Flex>
            <Flex justify=FlexJustify::Center>
                <h1>"Powered by Tauri + Leptos"</h1>
            </Flex>
        </Flex>
    }
}
