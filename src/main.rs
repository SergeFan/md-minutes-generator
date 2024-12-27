// leptos_i18n::load_locales!();

mod app;

use leptos::prelude::mount_to_body;
use leptos::*;
use thaw::{ConfigProvider, ToasterProvider};

use app::*;
// use i18n::*;

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
