use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{view, IntoView};
use leptos_i18n::t;
use thaw::*;

use crate::component::dialog::ApplySettingDialog;
use crate::handler::settings::{reset_app_settings, set_app_settings};
use crate::i18n::use_i18n;

#[derive(Clone)]
enum SavePattern {
    Save,
    Reset,
    Cancel,
}

#[component]
pub fn AppSetting(
    open_settings: RwSignal<bool>,
    language: RwSignal<String>,
    direct_generation: RwSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

    let open_dialog = RwSignal::new(false);
    let save_pattern = RwSignal::new(SavePattern::Cancel);
    let save_action = RwSignal::new(false);

    let reset_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        open_dialog.set(true);
        save_pattern.set(SavePattern::Reset);
    };

    let save_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        open_dialog.set(true);
        save_pattern.set(SavePattern::Save);
    };

    Effect::new(move || {
        if save_action.get() {
            match save_pattern.get() {
                SavePattern::Save => spawn_local(set_app_settings(language, direct_generation)),
                SavePattern::Reset => spawn_local(reset_app_settings()),
                SavePattern::Cancel => {}
            }
        }
    });

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
                    <p>
                        <b>{t!(i18n, app_setting_language)}</b>
                    </p>
                    <Select value=language default_value=language.get_untracked()>
                        <option value="en">"English"</option>
                        <option value="jp">"日本語"</option>
                        <option value="sc">"简体中文"</option>
                    </Select>
                </Flex>
                <Flex vertical=true>
                    <p>
                        <b>{t!(i18n, app_setting_direct_generation)}</b>
                    </p>
                    <Switch checked=direct_generation />
                    <p>{t!(i18n, app_setting_note, <b> = <b />)}</p>
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Button on:click=reset_settings>{t!(i18n, app_setting_reset_button)}</Button>
                    <Button on:click=save_settings appearance=ButtonAppearance::Primary>
                        {t!(i18n, app_setting_save_button)}
                    </Button>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>

        <ApplySettingDialog open=open_dialog action=save_action />
    }
}
