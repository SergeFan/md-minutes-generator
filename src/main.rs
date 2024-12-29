// leptos_i18n::load_locales!();

use leptos::prelude::mount_to_body;
use leptos::*;
use thaw::{ConfigProvider, ToasterProvider};

// use i18n::*;
use md_minutes_generator_ui::App;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            // <I18nContextProvider>
                <ConfigProvider>
                    <ToasterProvider>
                        <App/>
                    </ToasterProvider>
                </ConfigProvider>
            // </I18nContextProvider>
        }
    })
}
