use {
    log::{error, info, trace},
    nem_gui_dom::{
        universal_input::{results::Result, UniversalInput},
        Request, Response,
    },
    std::ops::Deref,
    sycamore::prelude::*,
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{ErrorEvent, MessageEvent, WebSocket},
};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).unwrap();
    let universal_input_state = Signal::new(String::new());
    let state = Signal::new(vec![
        Result {
            key: 0,
            message: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".into(),
        },
        Result {
            key: 1,
            message: "Ut quis facilisis urna, sed finibus urna. Morbi nec rutrum nisl, ac consequat nunc.".into(),
        },
    ]);
    sycamore::render({
        let universal_input_state = universal_input_state.clone();
        move || {
            view! {
                UniversalInput((universal_input_state, state.handle()))
            }
        }
    });

    let ws = WebSocket::new("ws://127.0.0.1:8000/ws").unwrap();
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            info!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            info!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                Ok(_) => info!("binary message successfully sent"),
                Err(err) => info!("error sending message: {:?}", err),
            }
        } else {
            info!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        error!("websocket error callback: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        info!("socket opened");
        // send off binary message
        match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            Ok(_) => info!("binary message successfully sent"),
            Err(err) => error!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    create_effect({
        let universal_input_state = universal_input_state.clone();
        let ws = ws.clone();
        move || {
            info!("ws::readyState: {:?}", ws.ready_state());
            trace!("ws::readyState: {:?}", ws.ready_state());
            // 1 == open
            // if ws.ready_state() == 1 {
            let req = Request::UniversalInput {
                input: universal_input_state.get().deref().clone(),
            };
            trace!("sending message: {:?}", req);
            //ws.send_with_u8_array(serde_json::to_vec(&req).unwrap().as_slice())
            //    .unwrap();
            //}
        }
    });
}
