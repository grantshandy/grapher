use crate::FunctionEntry;

#[cfg(target_arch = "wasm32")]
use crate::Grapher;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_web(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();

    eframe::start_web(canvas_id, Box::new(Grapher::new()))
}

#[cfg(target_arch = "wasm32")]
pub fn update_url(data: &Vec<FunctionEntry>) {
    let history = web_sys::window()
        .expect("Couldn't get window")
        .history()
        .expect("Couldn't get window.history");

    let info_str = url_string_from_data(data);

    history
        .push_state_with_url(&JsValue::NULL, "", Some(&info_str))
        .unwrap();
}

pub fn url_string_from_data(data: &Vec<FunctionEntry>) -> String {
    let mut info_str = String::from("#");

    for entry in data {
        info_str.push_str(format!("{},", entry.text).as_str());
    }

    info_str.pop();

    info_str
}

#[cfg(target_arch = "wasm32")]
pub fn get_data_from_url(data: &mut Vec<FunctionEntry>) -> Option<String> {
    let href = web_sys::window()
        .expect("Couldn't get window")
        .document()
        .expect("Couldn't get document")
        .location()
        .expect("Couldn't get location")
        .href()
        .expect("Couldn't get href");

    if !href.contains('#') {
        return None;
    }

    let func_string = match href.split('#').last() {
        Some(x) => x,
        None => return None,
    };

    if func_string == "" {
        return None;
    }

    let mut error: Option<String> = None;

    for entry in func_string.split(',') {
        let func = match exmex::parse::<f64>(&entry.replace("e", crate::EULER)) {
            Ok(func) => Some(func),
            Err(e) => {
                error = Some(e.to_string());
                None
            }
        };

        data.push(FunctionEntry {
            text: entry.to_string(),
            func,
        });
    }

    error
}
