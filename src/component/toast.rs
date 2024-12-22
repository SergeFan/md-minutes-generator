use std::time::Duration;

use leptos::view;
use thaw::*;

pub fn dispatch_toast(toaster: ToasterInjection, toast_intent: ToastIntent) {
    let toast_option = ToastOptions::default()
        .with_intent(toast_intent)
        .with_position(ToastPosition::Top)
        .with_timeout(Duration::from_secs(5));

    match toast_intent {
        ToastIntent::Success => {
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"Completed!"</ToastTitle>
                            <ToastBody>"Markdown generation succeeded."</ToastBody>
                        </Toast>
                    }
                },
                toast_option,
            );
        }
        ToastIntent::Error => {
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"Failed!"</ToastTitle>
                            <ToastBody>"Markdown generation has been cancelled."</ToastBody>
                        </Toast>
                    }
                },
                toast_option,
            );
        }
        _ => {}
    }
}
