use chrono::Local;
use leptos::leptos_dom::logging::console_warn;
use leptos::prelude::{Effect, RwSignal, Set};
use leptos::{component, view, IntoView};
use thaw::{MessageBar, MessageBarBody, MessageBarIntent, MessageBarTitle};

use crate::handler::{match_worksheet, MatchResult};

#[component]
pub fn FileStatus(selected_worksheet: RwSignal<Option<String>>) -> impl IntoView {
    let message_intent = RwSignal::new(MessageBarIntent::Info);
    let message_title = RwSignal::new(String::from("Excel not found"));
    let message_body = RwSignal::new(String::from(
        "No excel has been found, please select one to proceed.",
    ));

    Effect::new(
        move || match match_worksheet(selected_worksheet, Local::now()) {
            MatchResult::Match => {
                message_intent.set(MessageBarIntent::Success);
                message_title.set(String::from("Date matched"));
                message_body.set(String::from(
                    "Today's worksheet has been found, you can generate now.",
                ));
            }
            MatchResult::Mismatch => {
                message_intent.set(MessageBarIntent::Warning);
                message_title.set(String::from("Date mismatched"));
                message_body.set(String::from(
                    "No worksheet matches today, please choose one to generate.",
                ));
            }
            MatchResult::WorksheetNotFound => console_warn("No worksheet has been found."),
        },
    );

    view! {
        <MessageBar intent=message_intent>
            <MessageBarBody>
                <MessageBarTitle>{message_title}</MessageBarTitle>
                {message_body}
            </MessageBarBody>
        </MessageBar>
    }
}
