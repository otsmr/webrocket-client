#![allow(non_snake_case, non_upper_case_globals)]

mod utils;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebRocket {
    url: String,
    event_fkts: HashMap<String, Vec<js_sys::Function>>,
}
#[wasm_bindgen]
impl WebRocket {
    #[wasm_bindgen(constructor)]
    pub fn new(url: Option<String>) -> Self {
        let url = if let Some(h) = url {
            h
        } else {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let location = document
                .location()
                .expect("should habe a location on document");
            "ws://".to_owned() + &location.host().expect("should have a host on location")
        };
        Self {
            url,
            event_fkts: HashMap::new(),
        }
    }
    pub fn on(&mut self, event: String, callback: &js_sys::Function) {
        console_log!("on_connect");
        let entry = self.event_fkts.entry(event).or_default();
        entry.push(callback.to_owned());
    }

    pub fn connect(&self) {
        console_log!("Connectin to {}.", self.url);

        // TODO: create WebSocket connection, see https://rustwasm.github.io/wasm-bindgen/examples/websockets.html

        if let Some(callbacks) = self.event_fkts.get("connect") {
            for callback in callbacks.iter() {
                callback.call0(&JsValue::NULL).expect("Callback was wrong");
            }
        }
    }
}
