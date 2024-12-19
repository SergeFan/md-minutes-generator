use chrono::prelude::*;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use md_minutes_generator_ui::handler::drag_drop::drag_drop;
use md_minutes_generator_ui::handler::generate::generate;
use md_minutes_generator_ui::handler::select_input::select_input;
use md_minutes_generator_ui::handler::select_output::select_output;
use md_minutes_generator_ui::handler::setup_output_options;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let file_path = RwSignal::new(String::new());
    let markdown_path = RwSignal::new(String::new());
    let worksheet_options = RwSignal::new(Vec::new());
    let selected_worksheet = RwSignal::new(None::<String>);

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
                setup_output_options(
                    &file_path.get_untracked(),
                    markdown_path,
                    worksheet_options,
                    selected_worksheet,
                )
                .await;
            });
        }
    });

    let select_path = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(select_output(markdown_path));
    };

    let is_worksheet_selected = move || {
        if let Some(selected_worksheet) = selected_worksheet.get() {
            let today = Local::now().format("%Y%m%d").to_string();
            if selected_worksheet == today {
                return view! {
                    <MessageBar intent=MessageBarIntent::Success>
                        <MessageBarBody>
                            <MessageBarTitle>"Date matched"</MessageBarTitle>
                            "Today's worksheet has been found, you can generate now."
                        </MessageBarBody>
                    </MessageBar>
                };
            }

            return view! {
                <MessageBar intent=MessageBarIntent::Warning>
                    <MessageBarBody>
                        <MessageBarTitle>"Date mismatched"</MessageBarTitle>
                        "No worksheet matches today, please select one to generate."
                    </MessageBarBody>
                </MessageBar>
            };
        };

        view! {
            <MessageBar intent=MessageBarIntent::Info>
                <MessageBarBody>
                    <MessageBarTitle>"Excel not found"</MessageBarTitle>
                    "No excel has been found, please select one to proceed."
                </MessageBarBody>
            </MessageBar>
        }
    };

    // Generate markdown
    let toaster = ToasterInjection::expect_context();

    let generate_markdown = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(generate(
            file_path,
            markdown_path,
            selected_worksheet,
            toaster,
        ))
    };

    view! {
        <main class="container">
            <h1>"Powered by Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://leptos.dev" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>
                <b>"Click the button"</b>" to select your input path"<br/>
                "OR"<br/>
                <b>"Drag & drop"</b>" your file to the App window"
            </p>

            <Grid cols=5 x_gap=8 y_gap=8>
                <GridItem offset=1 column=2>
                    <Field>
                        <Input value=file_path placeholder="Select input excel path..."/>
                    </Field>
                </GridItem>
                <GridItem>
                    <Button block=true on:click=select_file appearance=ButtonAppearance::Secondary>"Input Path"</Button>
                </GridItem>
                <GridItem offset=1 column=2>
                    <Field>
                        <Input value=markdown_path placeholder="Select output markdown path..."/>
                    </Field>
                </GridItem>
                <GridItem>
                    <Button block=true on:click=select_path appearance=ButtonAppearance::Secondary>"Output Path"</Button>
                </GridItem>
                <GridItem offset=1 column=2>
                    <Select>
                        <For
                            each=move || worksheet_options.get()
                            key=move |worksheet_option| worksheet_option.clone()
                            children=move |worksheet_option| {
                                view!{
                                    <option>{worksheet_option}</option>
                                }
                            }
                        />
                    </Select>
                </GridItem>
                <GridItem>
                    <Button block=true on:click=generate_markdown appearance=ButtonAppearance::Primary>"Generate!"</Button>
                </GridItem>
            </Grid>

            <br/>

            <Flex justify=FlexJustify::Center>
                {is_worksheet_selected}
            </Flex>
        </main>
    }
}
