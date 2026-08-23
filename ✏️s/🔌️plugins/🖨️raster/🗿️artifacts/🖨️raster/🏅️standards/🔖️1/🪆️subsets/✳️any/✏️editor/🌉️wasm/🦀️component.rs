//! 🌉️ Raster play app — the `wasm-bindgen` VCS bridge: a JS-facing surface distinct from the WASM
//! Component Model plugin ABI the rest of this crate speaks. Only compiled for `target_arch = "wasm32"`
//! (was: the plugin-root `📦️glue.rs`'s `RasterArtifactVcs` in the old bundle crate).

#[cfg(target_arch = "wasm32")]
mod document_vcs {
    //#region 🔖️ArtifactVcs
    use std::cell::RefCell;

    use wasm_bindgen::prelude::*;

    use crate::artifacts::raster::op::{RasterEnvelope, RasterStore};

    #[wasm_bindgen]
    pub struct RasterArtifactVcs {
        store: RefCell<RasterStore>,
    }

    #[wasm_bindgen]
    impl RasterArtifactVcs {
        #[wasm_bindgen(constructor)]
        pub async fn new(envelope_json: &str) -> Result<RasterArtifactVcs, JsValue> {
            let envelope: RasterEnvelope = store::reject_whole_buffer_artifact_envelope_ingress(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(RasterStore::new(envelope).map_err(|e| JsValue::from_str(&e.to_string()))?) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub async fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub async fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub async fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub async fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub async fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
    //#endregion 🔖️ArtifactVcs
}

#[cfg(target_arch = "wasm32")]
pub use document_vcs::*;
