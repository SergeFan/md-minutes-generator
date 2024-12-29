use leptos::ev::MouseEvent;
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
    let reset_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(reset_app_settings(language, direct_generation));
    };

    let save_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(set_app_settings(language, direct_generation));
    };

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
                    <Select value=language default_value=language.get_untracked()>
                        <option value="en">"English"</option>
                        <option value="jp">"日本語"</option>
                        <option value="sc">"简体中文"</option>
                    </Select>
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
