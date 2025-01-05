use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};
use leptos_i18n::t;

use crate::i18n::use_i18n;

#[component]
pub fn UsageGuide() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <p>
            {t!(i18n, usage_guide_1, <b> = <b />)}<br />
            {t!(i18n, or)}<br />
            {t!(i18n, usage_guide_2, <b> = <b />)}
        </p>
    }
}
