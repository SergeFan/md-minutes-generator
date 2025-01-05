use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos_i18n::{t, t_string};
use thaw::*;

use crate::handler::generate::generate_markdown;
use crate::handler::path::{select_input, select_output};
use crate::i18n::use_i18n;

#[component]
pub fn InputSection(
    file_path: RwSignal<String>,
    markdown_path: RwSignal<String>,
    selected_worksheet: RwSignal<String>,
    worksheet_options: RwSignal<Vec<String>>,
) -> impl IntoView {
    let i18n = use_i18n();
    let toaster = ToasterInjection::expect_context();

    // Select input path
    let select_file = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(select_input(file_path));
    };

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
        <Flex vertical=true>
            <Flex justify=FlexJustify::Center>
                <Field attr:style="width: 300px">
                    <Input value=file_path placeholder=t_string!(i18n, input_path_placeholder) />
                </Field>
                <Button
                    attr:style="width: 150px"
                    on:click=select_file
                    appearance=ButtonAppearance::Secondary
                >
                    {t!(i18n, input_path_button)}
                </Button>
            </Flex>
            <Flex justify=FlexJustify::Center>
                <Field attr:style="width: 300px">
                    <Input
                        value=markdown_path
                        placeholder=t_string!(i18n, output_path_placeholder)
                    />
                </Field>
                <Button
                    attr:style="width: 150px"
                    on:click=select_path
                    appearance=ButtonAppearance::Secondary
                >
                    {t!(i18n, output_path_button)}
                </Button>
            </Flex>
            <Flex justify=FlexJustify::Center>
                <Combobox
                    attr:style="width: 300px"
                    value=selected_worksheet
                    placeholder=t_string!(i18n, target_worksheet_placeholder)
                >
                    {worksheet_options
                        .get()
                        .iter()
                        .map(|option| view! { <ComboboxOption text=option /> })
                        .collect_view()}
                </Combobox>
                // <Select
                //     attr:style="width: 300px"
                //     value=selected_worksheet
                //     default_value=selected_worksheet.get_untracked()
                // >
                //     <For
                //         each=move || worksheet_options.get()
                //         key=move |worksheet_option| worksheet_option.clone()
                //         children=move |worksheet_option| {
                //             view! { <option>{worksheet_option}</option> }
                //         }
                //     />
                // </Select>
                <Button
                    attr:style="width: 150px"
                    on:click=generate_markdown
                    appearance=ButtonAppearance::Primary
                >
                    {t!(i18n, generate_button)}
                </Button>
            </Flex>
        </Flex>
    }
}
