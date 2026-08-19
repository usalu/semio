//! 🌐️ Trinity Jack app — standalone WASM document-VCS bridge (no external Rust consumer today,
//! preserved verbatim from the old `trinity_ram` bundle; not the `semio_plugin!` guest ABI, which
//! goes through the framework's own `component-guest` machinery instead).

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::artifacts::jack::op::{create_trinity_graph_envelope, TrinityGraphEnvelope, TrinityGraphStore};
    use crate::artifacts::jack::empty_trinity_graph_fixture;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct JackSnapshotVcs {
        store: RefCell<TrinityGraphStore>,
    }

    #[wasm_bindgen]
    impl JackSnapshotVcs {
        #[wasm_bindgen(constructor)]
        pub async fn new(envelope_json: Option<String>) -> Result<JackSnapshotVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: TrinityGraphEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    TrinityGraphStore::new(envelope).map_err(|e| JsValue::from_str(&e.to_string()))?
                }
                None => TrinityGraphStore::new(create_trinity_graph_envelope("trinity", empty_trinity_graph_fixture())).map_err(|e| JsValue::from_str(&e.to_string()))?,
            };
            Ok(Self { store: RefCell::new(store) })
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
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge::JackSnapshotVcs;
