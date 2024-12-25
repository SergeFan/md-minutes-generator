use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use md_minutes_generator_ui::component::drawer::drawer;
use md_minutes_generator_ui::component::message_bar::FileStatus;
use md_minutes_generator_ui::handler::drag_drop::drag_drop;
use md_minutes_generator_ui::handler::generate::generate;
use md_minutes_generator_ui::handler::select_input::select_input;
use md_minutes_generator_ui::handler::select_output::select_output;
use md_minutes_generator_ui::handler::setup_output_options;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let open_settings = RwSignal::new(false);

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

    // Select output path
    let select_path = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(select_output(markdown_path));
    };

    // Generate markdown
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
            <Grid cols=7>
                <GridItem offset=1 column=5>
                    <h1>"Powered by Tauri + Leptos"</h1>
                </GridItem>
                <GridItem attr:style="display:flex align-items:center">
                    <Button appearance=ButtonAppearance::Transparent on:click=move |_| open_settings.set(true)>"Settings"</Button>
                </GridItem>
            </Grid>

            {drawer(open_settings)}

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

            <Flex vertical=true>
                <Flex justify=FlexJustify::Center>
                    <Field attr:style="width: 300px">
                        <Input value=file_path placeholder="Select input excel path..."/>
                    </Field>
                    <Button attr:style="width: 150px" on:click=select_file appearance=ButtonAppearance::Secondary>"Input Path"</Button>
                </Flex>
                <Flex justify=FlexJustify::Center>
                    <Field attr:style="width: 300px">
                        <Input value=markdown_path placeholder="Select output markdown path..."/>
                    </Field>
                    <Button attr:style="width: 150px" on:click=select_path appearance=ButtonAppearance::Secondary>"Output Path"</Button>
                </Flex>
                <Flex justify=FlexJustify::Center>
                    <Select attr:style="width: 300px">
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
                    <Button attr:style="width: 150px" on:click=generate_markdown appearance=ButtonAppearance::Primary>"Generate!"</Button>
                </Flex>
            </Flex>

            <br/>

            <Flex justify=FlexJustify::Center>
                <FileStatus selected_worksheet/>
            </Flex>
        </main>
    }
}
