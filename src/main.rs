mod app;

use app::*;
use leptos::prelude::mount_to_body;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
                <App/>
        }
    })
}
