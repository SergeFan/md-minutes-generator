use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{view, IntoView};
use thaw::*;

use crate::handler::settings::{reset_app_settings, set_app_settings};

#[component]
pub fn AppSetting(
    open_settings: RwSignal<bool>,
    language: RwSignal<String>,
    direct_generation: RwSignal<bool>,
) -> impl IntoView {
    console_log(format!("language is set to: {}", language.get_untracked()).as_str());

    let reset_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(reset_app_settings(language, direct_generation));
    };

    let save_settings = move |ev: MouseEvent| {
        ev.prevent_default();

        // TODO: delete console log after feature finished
        console_log(language.get_untracked().as_str());

        spawn_local(set_app_settings(language, direct_generation));
    };

    // let test_language = RwSignal::new(String::from("sc"));

    view! {
        <OverlayDrawer open=open_settings position=DrawerPosition::Left>
            <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            icon=icondata::CgClose
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open_settings.set(false)
                        />
                    </DrawerHeaderTitleAction>
                "App Settings"
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <Flex vertical=true>
                    <p><b>"Language"</b></p>
                    <Combobox value=language>
                        <ComboboxOption value="en" text="English"/>
                        <ComboboxOption value="jp" text="日本語"/>
                        <ComboboxOption value="sc" text="简体中文"/>
                    </Combobox>
                    // <Select value=test_language>
                    //     <option value="en">"English"</option>
                    //     <option value="jp">"日本語"</option>
                    //     <option value="sc">"简体中文"</option>
                    // </Select>
                    // <Button on:click=move |_| test_language.set("jp".to_string())>"test"</Button>
                </Flex>
                <Flex vertical=true>
                    <p><b>"Direct Generation"</b></p>
                    <Switch checked=direct_generation/>
                    <p>
                        <b>"Note: "</b>
                        "Turn on Direct Generation will generate markdown at selected output path "
                        <b>"instantly "</b>
                        "when worksheet with name matching today's date has been found."
                    </p>
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Button on:click=reset_settings>"Reset"</Button>
                    <Button on:click=save_settings appearance=ButtonAppearance::Primary>"Save"</Button>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}
