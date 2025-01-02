use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_i18n::t;
use thaw::*;

use crate::i18n::use_i18n;

#[component]
pub fn ApplySettingDialog(open: RwSignal<bool>, action: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();

    let apply_settings = move |ev: MouseEvent| {
        ev.prevent_default();
        open.set(false);
        action.set(true);
    };

    view! {
        <Dialog open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t!(i18n, dialog_title)}</DialogTitle>
                    <DialogContent>
                        {t!(i18n, dialog_body)}
                    </DialogContent>
                    <DialogActions>
                        <Flex justify=FlexJustify::SpaceBetween>
                            <Button on:click=move |_| open.set(false)>{t!(i18n, dialog_button_cancel)}</Button>
                            <Button on:click=apply_settings appearance=ButtonAppearance::Primary>{t!(i18n, dialog_button_confirm)}</Button>
                        </Flex>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
