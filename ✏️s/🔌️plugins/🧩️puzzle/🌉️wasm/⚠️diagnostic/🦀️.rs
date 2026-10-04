//! 🌉️ Private puzzle artifact browser fault projection.
use semio_framework_diagnostic::Fault;
use semio_framework_value::ToValue;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
/// 🌐️ Surfaces a structured {@link Fault} to JavaScript callers.
pub fn fault_to_js(fault: Fault) -> wasm_bindgen::JsValue {
    let rendered = serde_json::to_string(&fault.to_value()).unwrap_or_else(|_| fault.message.clone());
    wasm_bindgen::JsValue::from_str(&rendered)
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
/// 🌐️ Maps `Result<T, Fault>` into `Result<T, JsValue>` for wasm exports.
pub fn result_fault_to_js<T>(result: Result<T, Fault>) -> Result<T, wasm_bindgen::JsValue> {
    result.map_err(fault_to_js)
}
