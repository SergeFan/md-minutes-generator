use js_sys::try_iter;
use leptos::ev::MouseEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::convert::Into;
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

#[derive(Debug, Deserialize)]
struct Object {
    id: u32,
    event: String,
    payload: Payload,
}

#[derive(Debug, Deserialize)]
struct Payload {
    paths: Vec<String>,
    position: Position,
}

#[derive(Debug, Deserialize)]
struct Position {
    x: u32,
    y: u32,
}

#[component]
pub fn App() -> impl IntoView {
    let file_path = RwSignal::new(String::new());
    let markdown_path = RwSignal::new(String::new());
    let options = RwSignal::new(Vec::new());
    let value = RwSignal::new(None::<String>);

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

    create_effect(move |_| {
        if !file_path.get().is_empty() {
            spawn_local(async move {
                setup_output_options(&file_path.get_untracked(), markdown_path, options, value)
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

    let message = use_message();

    let generate_markdown = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let selected_file = file_path.get();
            let selected_path = markdown_path.get();
            if let Some(selected_sheet) = value.get() {
                let args = serde_wasm_bindgen::to_value(&FileArgs {
                    input: selected_file.as_str(),
                    output: selected_path.as_str(),
                    sheet: selected_sheet.as_str(),
                })
                .unwrap();

                let js_value: JsValue = invoke("generate_markdown", args).await;

                if js_value.as_bool().unwrap_or(false) {
                    message.create(
                        format!("Markdown file has been generated at '{}'.", selected_path),
                        MessageVariant::Success,
                        Default::default(),
                    );
                } else {
                    message.create(
                        "Markdown generation failed.".into(),
                        MessageVariant::Error,
                        Default::default(),
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
            <p>"Click button to select input or simply drag-drop your file."</p>

            <Grid cols=5 x_gap=8 y_gap=8>
            <GridItem offset=1 column=2>
                <Input value=file_path placeholder="Select input excel path..."/>
            </GridItem>
            <GridItem>
                <Button on:click=select_file variant=ButtonVariant::Primary>"Input Path"</Button>
            </GridItem>
            <GridItem offset=1 column=2>
                <Input value=markdown_path placeholder="Select output markdown path..."/>
            </GridItem>
            <GridItem>
                <Button on:click=select_path variant=ButtonVariant::Primary>"Output Path"</Button>
            </GridItem>
            <GridItem offset=1 column=2>
                <Select value options />
            </GridItem>
            <GridItem>
                <Button on:click=generate_markdown color=ButtonColor::Success>"Generate!"</Button>
            </GridItem>
            </Grid>
        </main>
    }
}

async fn setup_output_options(
    selected_file: &str,
    markdown_path: RwSignal<String>,
    options: RwSignal<Vec<SelectOption<String>>>,
    value: RwSignal<Option<String>>,
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
        let option_strings = js_iterator
            .filter_map(|item| item.ok().unwrap().as_string())
            .map(|item| SelectOption::new(item.to_owned(), item))
            .collect();

        options.set(option_strings);

        if let Some(first_option) = options.get().first() {
            value.set(Some(first_option.label.to_owned()));
        }
    }
}
