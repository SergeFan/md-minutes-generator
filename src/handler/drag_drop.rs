use crate::handler::{listen, Object};
use leptos::logging;
use leptos::prelude::{RwSignal, Set};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub async fn drag_drop(file_path: RwSignal<String>) {
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
}
