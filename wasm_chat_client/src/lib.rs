use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent, console};
use std::rc::Rc;
use std::cell::RefCell;
use js_sys::Function;


// Macro to log messages to the browser's console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// WebSocket callback functions
fn on_message(event: MessageEvent) {
    if let Some(text) = event.data().as_string() {
        console_log!("Message received: {}", text);
    }
}

fn on_error(e: ErrorEvent) {
    console_log!("Error: {:?}", e);
}

fn on_close(e: CloseEvent) {
    console_log!("Connection closed: {:?}", e);
}

fn create_websocket() -> Result<Rc<RefCell<WebSocket>>, JsValue> {
    let ws = WebSocket::new("ws://localhost:8080/ws/")?;
    let ws_rc = Rc::new(RefCell::new(ws));

    // Closure for the 'onopen' event
    {
        let ws_clone = ws_rc.clone();
        let on_open = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let mut ws = ws_clone.borrow_mut();
            if let Err(e) = ws.send_with_str("Hello from WASM!") {
                console::log_1(&format!("Error sending message: {:?}", e).into());
            }
        }) as Box<dyn FnMut(_)>);

        ws_rc.borrow().set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
    }
    {
        let ws_clone = ws_rc.clone();
        let on_message_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Some(text) = e.data().as_string() {
                console::log_1(&text.into());
            }
        }) as Box<dyn FnMut(_)>);

        let ws_clone = ws_rc.clone();
        let on_error_closure = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console::log_1(&e.message().into());
        }) as Box<dyn FnMut(_)>);

        let ws_clone = ws_rc.clone();
        let on_close_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
            console::log_1(&e.reason().into());
        }) as Box<dyn FnMut(_)>);

        ws_clone.borrow().set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        ws_clone.borrow().set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
        ws_clone.borrow().set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));

        on_message_closure.forget();
        on_error_closure.forget();
        on_close_closure.forget();
    }

    Ok(ws_rc)
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let ws_rc = create_websocket()?;

    // The WebSocket event handlers are already set in create_websocket

    Ok(())
}