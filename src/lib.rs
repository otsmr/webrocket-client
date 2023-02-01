#![allow(non_snake_case, non_upper_case_globals)]

mod utils;

// use std::net::SocketAddr;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WebRocket {
    secure: bool,
    host: String,
    event_callbacks: HashMap<String, Vec<js_sys::Function>>,
}
#[wasm_bindgen]
impl WebRocket {
    #[wasm_bindgen(constructor)]
    pub fn new(host: Option<String>) -> Self {
        console_log!("HALLO: {:?}", host);
        let host = if let Some(h) = host {
            h
        } else {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let location = document
                .location()
                .expect("should habe a location on document");
            location.host().expect("should have a host on location")
        };
        console_log!("Url = {:?}", host);
        Self {
            secure: false,
            host,
            event_callbacks: HashMap::new(),
        }
    }
    pub fn on(&mut self, event: String, callback: &js_sys::Function) {
        console_log!("on_connect");
        let entry = self.event_callbacks.entry(event).or_default();
        entry.push(callback.to_owned());
    }

    pub fn connect(&self) {
        console_log!("Connectin to {} ({}).", self.host, self.secure);

        // TODO: create WebSocket connection, see https://rustwasm.github.io/wasm-bindgen/examples/websockets.html

        if let Some(callbacks) = self.event_callbacks.get("connect") {
            for callback in callbacks.iter() {
                callback.call0(&JsValue::NULL).expect("Callback was wrong");
            }
        }
    }
}
