use leptos::logging;
use leptos::prelude::{RwSignal, Set};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::handler::listen;

#[derive(Debug, Serialize, Deserialize)]
struct EventData {
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

pub async fn drag_drop(file_path: RwSignal<String>) {
    let closure = Closure::<dyn FnMut(_)>::new(move |js_value: JsValue| {
        match from_value::<EventData>(js_value) {
            Ok(data) => {
                if let Some(path) = data.payload.paths.first() {
                    file_path.set(path.to_owned());
                }
            }
            Err(err) => logging::error!("error: {:?}", err),
        }
    });

    listen("tauri://drag-drop", closure.as_ref().unchecked_ref()).await;

    closure.forget();
}
