use chrono::Local;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::component::drawer::AppSetting;
use crate::component::message_bar::FileStatus;
use crate::handler::drag_drop::drag_drop;
use crate::handler::generate::generate_markdown;
use crate::handler::path::{select_input, select_output};
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

    let settings = LocalResource::new(move || get_app_settings(language, direct_generation));

    // Drag & drop handler
    spawn_local(drag_drop(file_path));

    // File select handler
    let select_file = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(select_input(file_path));
    };

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
                    && match_worksheet_name(selected_worksheet, Local::now()) == MatchResult::Match
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

    // Select output path
    let select_path = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(select_output(markdown_path));
    };

    // Generate markdown
    let generate_markdown = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(generate_markdown(
            file_path,
            markdown_path,
            selected_worksheet,
            i18n,
            toaster,
        ));
    };

    view! {
        <main class="container">
            <Transition>
                {move || {settings.read().as_deref().map(|a| {
                    match a.language.as_str() {
                        "en" => i18n.set_locale(Locale::en),
                        "jp" => i18n.set_locale(Locale::jp),
                        "sc" => i18n.set_locale(Locale::sc),
                        _ => i18n.set_locale(Locale::en),
                    };

                    view! {
                        <AppSetting open_settings language direct_generation/>

                        <Flex vertical=true>
                            <Flex justify=FlexJustify::End>
                                <Button
                                    icon=icondata::OcGearSm
                                    appearance=ButtonAppearance::Subtle
                                    on_click=move |_| open_settings.set(true)
                                />
                            </Flex>
                            <Flex justify=FlexJustify::Center>
                                <h1>"Powered by Tauri + Leptos"</h1>
                            </Flex>
                        </Flex>

                        <div class="row">
                            <a href="https://tauri.app" target="_blank">
                                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                            </a>
                            <a href="https://leptos.dev" target="_blank">
                                <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                            </a>
                        </div>

                        <p>
                            {t!(i18n, usage_guide_1, <b> = <b/>)}<br/>
                            {t!(i18n, or)}<br/>
                            {t!(i18n, usage_guide_2, <b> = <b/>)}
                        </p>

                        <Flex vertical=true>
                            <Flex justify=FlexJustify::Center>
                                <Field attr:style="width: 300px">
                                    <Input value=file_path placeholder=t_string!(i18n, input_path_placeholder)/>
                                </Field>
                                <Button attr:style="width: 150px" on:click=select_file appearance=ButtonAppearance::Secondary>{t!(i18n, input_path_button)}</Button>
                            </Flex>
                            <Flex justify=FlexJustify::Center>
                                <Field attr:style="width: 300px">
                                    <Input value=markdown_path placeholder=t_string!(i18n, output_path_placeholder)/>
                                </Field>
                                <Button attr:style="width: 150px" on:click=select_path appearance=ButtonAppearance::Secondary>{t!(i18n, output_path_button)}</Button>
                            </Flex>
                            <Flex justify=FlexJustify::Center>
                                <Combobox attr:style="width: 300px" value=selected_worksheet placeholder=t_string!(i18n, target_worksheet_placeholder)>
                                    <For
                                        each=move || worksheet_options.get()
                                        key=move |worksheet_option| worksheet_option.clone()
                                        children=move |worksheet_option| {
                                            view!{
                                                <ComboboxOption text={worksheet_option}/>
                                            }
                                        }
                                    />
                                </Combobox>
                                // <Select attr:style="width: 300px" value=selected_worksheet default_value=selected_worksheet.get_untracked()>
                                //     <For
                                //         each=move || worksheet_options.get()
                                //         key=move |worksheet_option| worksheet_option.clone()
                                //         children=move |worksheet_option| {
                                //             view!{
                                //                 <option>{worksheet_option}</option>
                                //             }
                                //         }
                                //     />
                                // </Select>
                                <Button attr:style="width: 150px" on:click=generate_markdown appearance=ButtonAppearance::Primary>{t!(i18n, generate_button)}</Button>
                            </Flex>
                        </Flex>

                        <br/>

                        <Flex justify=FlexJustify::Center>
                            <FileStatus selected_worksheet/>
                        </Flex>
                    }
                })}}
            </Transition>
        </main>
    }
}
