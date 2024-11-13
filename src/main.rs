mod app;

use app::*;
use leptos::*;
use thaw::MessageProvider;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <MessageProvider>
                <App/>
            </MessageProvider>
        }
    })
}
