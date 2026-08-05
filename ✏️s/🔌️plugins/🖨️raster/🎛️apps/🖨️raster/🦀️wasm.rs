//! 🌉️ Raster play app — the `wasm-bindgen` VCS bridge: a JS-facing surface distinct from the WASM
//! Component Model plugin ABI the rest of this crate speaks. Only compiled for `target_arch = "wasm32"`
//! (was: the plugin-root `📦️lib.rs`'s `RasterDocumentVcs` in the old bundle crate).

#[cfg(target_arch = "wasm32")]
mod document_vcs {
    //#region 🔖️DocumentVcs
    use std::cell::RefCell;

    use wasm_bindgen::prelude::*;

    use crate::artifacts::raster::op::{RasterEnvelope, RasterStore};

    #[wasm_bindgen]
    pub struct RasterDocumentVcs {
        store: RefCell<RasterStore>,
    }

    #[wasm_bindgen]
    impl RasterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<RasterDocumentVcs, JsValue> {
            let envelope: RasterEnvelope = serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(RasterStore::new(envelope)) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
    //#endregion 🔖️DocumentVcs
}

#[cfg(target_arch = "wasm32")]
pub use document_vcs::*;
