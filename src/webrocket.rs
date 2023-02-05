#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

use crate::console_log;

#[wasm_bindgen]
pub struct WebRocket {
    ws: WebSocket,
    message_queue: Vec<String>,
    event_fkts: HashMap<String, Vec<js_sys::Function>>,
}

// TODO: see https://rustwasm.github.io/wasm-bindgen/examples/websockets.html
#[wasm_bindgen]
impl WebRocket {

    #[wasm_bindgen(constructor)]
    pub fn new(url: Option<String>) -> Result<WebRocket, JsValue> {
        crate::utils::set_panic_hook();

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

        Ok(Self {
            ws: WebSocket::new(&url)?,
            message_queue: vec![],
            event_fkts: HashMap::new(),
        })
    }

    pub fn on(&mut self, event: String, callback: &js_sys::Function) {
        let entry = self.event_fkts.entry(event).or_default();
        entry.push(callback.to_owned());
        self.set_onmessage();
    }

    fn set_onmessage(&mut self) {
        let mut cloned_events = self.event_fkts.clone();

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let txt = txt.split(":");
                let payload = txt.slice(1, txt.length()).join("");
                for event in cloned_events
                    .entry(txt.get(0).as_string().unwrap())
                    .or_default()
                {
                    event.call1(&JsValue::null(), &payload).unwrap();
                }
            }
        });

        self.ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    fn send_queue(ws: WebSocket, queue: Vec<String>) {
        for msg in queue.iter() {
            match ws.send_with_str(msg) {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }
    }

    pub fn emit(&mut self, event: String, payload: String) {
        let packet: String = event + ":" + &payload;

        self.message_queue.push(packet);

        let ws = self.ws.clone();
        let queue = self.message_queue.drain(0..).collect();

        match ws.ready_state() {
            WebSocket::OPEN => WebRocket::send_queue(ws, queue),
            WebSocket::CONNECTING => {
                let ws2 = ws.clone();
                let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                    WebRocket::send_queue(ws2.clone(), queue.clone());
                });
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();
            }
            _ => ()
        }
    }
}
