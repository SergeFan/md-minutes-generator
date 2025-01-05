use chrono::Local;
use leptos::leptos_dom::logging::console_warn;
use leptos::prelude::{Effect, Get, RwSignal, Set};
use leptos::{component, view, IntoView};
use leptos_i18n::t_string;
use thaw::*;

use crate::handler::{match_worksheet_name, MatchResult};
use crate::i18n::use_i18n;

#[component]
pub fn InputStatus(selected_worksheet: RwSignal<String>) -> impl IntoView {
    let i18n = use_i18n();

    let message_intent = RwSignal::new(MessageBarIntent::Info);
    let message_title = RwSignal::new(t_string!(i18n, message_bar_title_info));
    let message_body = RwSignal::new(t_string!(i18n, message_bar_body_info));

    Effect::new(
        move || match match_worksheet_name(selected_worksheet.get(), Local::now()) {
            MatchResult::Match => {
                message_intent.set(MessageBarIntent::Success);
                message_title.set(t_string!(i18n, message_bar_title_success));
                message_body.set(t_string!(i18n, message_bar_body_success));
            }
            MatchResult::Mismatch => {
                message_intent.set(MessageBarIntent::Warning);
                message_title.set(t_string!(i18n, message_bar_title_warning));
                message_body.set(t_string!(i18n, message_bar_body_warning));
            }
            MatchResult::WorksheetNotFound => console_warn("No worksheet has been found."),
        },
    );

    view! {
        <Flex justify=FlexJustify::Center>
            <MessageBar intent=message_intent>
                <MessageBarBody>
                    <MessageBarTitle>{message_title}</MessageBarTitle>
                    {message_body}
                </MessageBarBody>
            </MessageBar>
        </Flex>
    }
}
