use leptos::prelude::mount_to_body;
use leptos::*;
use thaw::{ConfigProvider, ToasterProvider};

use md_minutes_generator_ui::component::app::App;
use md_minutes_generator_ui::i18n::I18nContextProvider;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <I18nContextProvider>
                <ConfigProvider>
                    <ToasterProvider>
                        <App/>
                    </ToasterProvider>
                </ConfigProvider>
            </I18nContextProvider>
        }
    })
}
