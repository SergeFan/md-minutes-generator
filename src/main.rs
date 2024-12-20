mod app;

use leptos::prelude::mount_to_body;
use leptos::*;
use thaw::{ConfigProvider, ToasterProvider};

use app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ConfigProvider>
                <ToasterProvider>
                    <App/>
                </ToasterProvider>
            </ConfigProvider>
        }
    })
}
