use chrono::Local;
use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::component::drawer::AppSetting;
use crate::component::header::Header;
use crate::component::input_section::InputSection;
use crate::component::logo::Logo;
use crate::component::message_bar::InputStatus;
use crate::component::usage_guide::UsageGuide;
use crate::handler::drag_drop::drag_drop;
use crate::handler::generate::generate_markdown;
use crate::handler::settings::get_app_settings;
use crate::handler::{load_output_options, match_worksheet_name, MatchResult};
use crate::i18n::*;

#[component]
pub fn App() -> impl IntoView {
    let i18n = use_i18n();
    let toaster = ToasterInjection::expect_context();

    let open_settings = RwSignal::new(false);
    let language = RwSignal::new(String::new());
    let direct_generation = RwSignal::new(false);

    let file_path = RwSignal::new(String::new());
    let markdown_path = RwSignal::new(String::new());
    let worksheet_options = RwSignal::new(Vec::new());
    let selected_worksheet = RwSignal::new(String::new());

    let settings = LocalResource::new(get_app_settings);

    // Drag & drop handler
    spawn_local(drag_drop(file_path));

    Effect::new(move |_| {
        if !file_path.get().is_empty() {
            spawn_local(async move {
                load_output_options(
                    file_path,
                    markdown_path,
                    worksheet_options,
                    selected_worksheet,
                )
                .await;

                if direct_generation.get_untracked()
                    && match_worksheet_name(selected_worksheet.get_untracked(), Local::now())
                        == MatchResult::Match
                {
                    spawn_local(generate_markdown(
                        file_path,
                        markdown_path,
                        selected_worksheet,
                        i18n,
                        toaster,
                    ));
                }
            });
        }
    });

    view! {
        <main class="container">
            <Header open_settings />

            <Logo />

            <Suspense>
                {move || {settings.read().as_deref().map(|settings| {
                    match settings.language.as_str() {
                        "en" => i18n.set_locale(Locale::en),
                        "jp" => i18n.set_locale(Locale::jp),
                        "sc" => i18n.set_locale(Locale::sc),
                        _ => i18n.set_locale(Locale::en),
                    };

                    language.set(settings.language.to_owned());
                    direct_generation.set(settings.direct_generation);

                    view! {
                        <AppSetting open_settings language direct_generation/>

                        <UsageGuide />

                        <InputSection file_path markdown_path selected_worksheet worksheet_options/>

                        <br />

                        <InputStatus selected_worksheet />
                    }
                })}}
            </Suspense>
        </main>
    }
}
