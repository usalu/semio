//! 🌐️ Puzzle 5d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle5dArtifactVcs` handle over the typed `Puzzle5dStore`, and every other wasm-bindgen-exported
//! puzzle-5d document surface (incl. the `.puzzle5d` DSL-text parser, relocated here from the deleted
//! artifact-side `⚙️engine` per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — a
//! `wasm_bindgen`/`JsValue`-returning fn is app-boundary behaviour, not artifact schema).

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::empty_puzzle5d_snapshot;
use crate::artifacts::puzzle5d::spr::{Puzzle5dEnvelope, Puzzle5dStore};
use crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA;
use std::cell::RefCell;
use store::create_document_envelope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Puzzle5dArtifactVcs {
    store: RefCell<Puzzle5dStore>,
}

#[wasm_bindgen]
impl Puzzle5dArtifactVcs {
    #[wasm_bindgen(constructor)]
    pub fn new(envelope_json: Option<String>) -> Result<Puzzle5dArtifactVcs, JsValue> {
        let store = match envelope_json {
            Some(json) => {
                let envelope: Puzzle5dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Puzzle5dStore::new(envelope)
            }
            None => Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_snapshot(), None)),
        };
        Ok(Self { store: RefCell::new(store) })
    }

    #[wasm_bindgen(js_name = dispatchText)]
    pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = dispatchBinary)]
    pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = projectionJson)]
    pub fn projection_json(&self) -> Result<String, JsValue> {
        self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
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

//#region 🔖️WasmBridge
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets
/// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the
/// DSL grammar.
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, wasm_bindgen::JsValue> {
    use store::ArtifactDsl;
    let projection = crate::artifacts::puzzle5d::Puzzle5dSnapshot::parse_dsl(dsl_text).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
//#endregion 🔖️WasmBridge
