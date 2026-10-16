//! 🌐️ Boot subphase declaration: the one console line a boot subphase declares, lifted out of the browser worker for the `host-base` crate of the renderer partition (plan section 5.2).
use super::*;

pub fn declare_boot_subphase(phase: &str, state: &str, elapsed_ms: f64) {
    let global = js_sys::global();
    if let Ok(func) = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("semioDeclareBootSubphase")) {
        if let Ok(func) = func.dyn_into::<js_sys::Function>() {
            let _ = func.call3(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(phase), &wasm_bindgen::JsValue::from_str(state), &wasm_bindgen::JsValue::from_f64(elapsed_ms));
        }
    }
}
