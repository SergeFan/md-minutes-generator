use chrono::prelude::*;
use js_sys::try_iter;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::convert::Into;
use std::time::Duration;
use thaw::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // Tauri event listener
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FileArgs<'a> {
    input: &'a str,
    output: &'a str,
    sheet: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct Object {
    id: u32,
    event: String,
    payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    paths: Vec<String>,
    position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: u32,
    y: u32,
}

#[component]
pub fn App() -> impl IntoView {
    let file_path = RwSignal::new(String::new());
    let markdown_path = RwSignal::new(String::new());
    let worksheet_options = RwSignal::new(Vec::new());
    let selected_worksheet = RwSignal::new(None::<String>);

    // Drag & drop handler
    spawn_local(async move {
        let closure = Closure::<dyn FnMut(_)>::new(move |js_value: JsValue| {
            match from_value::<Object>(js_value) {
                Ok(data) => {
                    logging::log!("recv event: {:?}", data);
                    if let Some(path) = data.payload.paths.first() {
                        file_path.set(path.to_owned());
                    }
                }
                Err(err) => logging::error!("error: {:?}", err),
            }
        });

        listen("tauri://drag-drop", closure.as_ref().unchecked_ref()).await;

        closure.forget();
    });

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

    // File select handler
    let select_file = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let selected_file = invoke_without_args("select_file")
                .await
                .as_string()
                .unwrap();

            file_path.set(selected_file.to_owned());
        });
    };

    let select_path = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let selected_path = invoke_without_args("select_path")
                .await
                .as_string()
                .unwrap();

            if !selected_path.is_empty() {
                markdown_path.set(selected_path)
            }
        })
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
        spawn_local(async move {
            let selected_file = file_path.get();
            let selected_path = markdown_path.get();
            if let Some(selected_sheet) = selected_worksheet.get() {
                let args = serde_wasm_bindgen::to_value(&FileArgs {
                    input: selected_file.as_str(),
                    output: selected_path.as_str(),
                    sheet: selected_sheet.as_str(),
                })
                .unwrap();

                let js_value: JsValue = invoke("generate_markdown", args).await;

                if js_value.as_bool().unwrap_or(false) {
                    toaster.dispatch_toast(
                        move || {
                            view! {
                                <Toast>
                                    <ToastTitle>"Generation completed"</ToastTitle>
                                    <ToastBody>"Markdown generation succeeded."</ToastBody>
                                </Toast>
                            }
                        },
                        ToastOptions::default()
                            .with_position(ToastPosition::Top)
                            .with_intent(ToastIntent::Success)
                            .with_timeout(Duration::from_secs(5)),
                    );
                } else {
                    toaster.dispatch_toast(
                        move || {
                            view! {
                                <Toast>
                                    <ToastTitle>"Generation failed"</ToastTitle>
                                    <ToastBody>"Markdown generation has been cancelled."</ToastBody>
                                </Toast>
                            }
                        },
                        ToastOptions::default()
                            .with_position(ToastPosition::Top)
                            .with_intent(ToastIntent::Error)
                            .with_timeout(Duration::from_secs(5)),
                    );
                }
            };
        })
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

            <Flex align=FlexAlign::End justify=FlexJustify::Center>
                {is_worksheet_selected}
            </Flex>
        </main>
    }
}

async fn setup_output_options(
    selected_file: &str,
    markdown_path: RwSignal<String>,
    worksheet_options: RwSignal<Vec<String>>,
    selected_worksheet: RwSignal<Option<String>>,
) {
    if markdown_path.get_untracked().is_empty() {
        markdown_path.set(
            invoke_without_args("get_desktop_dir")
                .await
                .as_string()
                .unwrap(),
        );
    }

    let args = serde_wasm_bindgen::to_value(&FileArgs {
        input: selected_file,
        output: "",
        sheet: "",
    })
    .unwrap();
    let js_value: JsValue = invoke("read_excel", args).await;

    if let Ok(Some(js_iterator)) = try_iter(&js_value) {
        let options: Vec<String> = js_iterator
            .filter_map(|item| item.ok().unwrap().as_string())
            .collect();

        if let Some(first_option) = options.first() {
            selected_worksheet.set(Some(first_option.to_owned()));
        }

        worksheet_options.set(options);
    }
}
