use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{view, IntoView};
use leptos_i18n::t;
use thaw::*;

use crate::handler::settings::{reset_app_settings, set_app_settings};
use crate::i18n::use_i18n;

#[component]
pub fn AppSetting(
    open_settings: RwSignal<bool>,
    language: RwSignal<String>,
    direct_generation: RwSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

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
                    {t!(i18n, app_setting_title)}
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <Flex vertical=true>
                    <p><b>{t!(i18n, app_setting_language)}</b></p>
                    <Select value=language default_value=language.get_untracked()>
                        <option value="en">"English"</option>
                        <option value="jp">"日本語"</option>
                        <option value="sc">"简体中文"</option>
                    </Select>
                </Flex>
                <Flex vertical=true>
                    <p><b>{t!(i18n, app_setting_direct_generation)}</b></p>
                    <Switch checked=direct_generation/>
                    <p>
                        {t!(i18n, app_setting_note, <b> = <b/>)}
                    </p>
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Button on:click=reset_settings>{t!(i18n, app_setting_reset_button)}</Button>
                    <Button on:click=save_settings appearance=ButtonAppearance::Primary>{t!(i18n, app_setting_save_button)}</Button>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}
