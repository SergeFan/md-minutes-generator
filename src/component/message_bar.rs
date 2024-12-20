use chrono::Local;
use leptos::prelude::{Effect, Get, RwSignal, Set};
use leptos::{component, view, IntoView};
use thaw::{MessageBar, MessageBarBody, MessageBarIntent, MessageBarTitle};

#[component]
pub fn FileStatus(selected_worksheet: RwSignal<Option<String>>) -> impl IntoView {
    let message_intent = RwSignal::new(MessageBarIntent::Info);
    let message_title = RwSignal::new(String::from("Excel not found"));
    let message_body = RwSignal::new(String::from(
        "No excel has been found, please select one to proceed.",
    ));

    Effect::new(move || {
        if let Some(selected_worksheet) = selected_worksheet.get() {
            if selected_worksheet == Local::now().format("%Y%m%d").to_string() {
                message_intent.set(MessageBarIntent::Success);
                message_title.set(String::from("Date matched"));
                message_body.set(String::from(
                    "Today's worksheet has been found, you can generate now.",
                ));
            } else {
                message_intent.set(MessageBarIntent::Warning);
                message_title.set(String::from("Date mismatched"));
                message_body.set(String::from(
                    "No worksheet matches today, please choose one to generate.",
                ));
            }
        }
    });

    view! {
        <MessageBar intent={message_intent}>
            <MessageBarBody>
                <MessageBarTitle>{message_title}</MessageBarTitle>
                {message_body}
            </MessageBarBody>
        </MessageBar>
    }
}
