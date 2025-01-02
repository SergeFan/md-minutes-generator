use std::time::Duration;

use leptos::view;
use leptos_i18n::{t, I18nContext};
use thaw::*;

use crate::i18n::Locale;

pub fn dispatch_toast(
    i18n: I18nContext<Locale>,
    toaster: ToasterInjection,
    toast_intent: ToastIntent,
) {
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
                            <ToastTitle>{t!(i18n, toast_title_success)}</ToastTitle>
                            <ToastBody>{t!(i18n, toast_body_success)}</ToastBody>
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
                            <ToastTitle>{t!(i18n, toast_title_error)}</ToastTitle>
                            <ToastBody>{t!(i18n, toast_body_error)}</ToastBody>
                        </Toast>
                    }
                },
                toast_option,
            );
        }
        _ => {}
    }
}
