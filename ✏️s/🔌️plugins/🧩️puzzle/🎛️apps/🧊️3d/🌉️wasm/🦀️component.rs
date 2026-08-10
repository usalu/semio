//! 🌐️ Puzzle 3d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle3dArtifactVcs` handle over the typed `Puzzle3dStore`, plus a `.puzzle3d` DSL-text parser
//! that hands non-Rust consumers (e.g. Storybook stories) the same camelCase JSON shape the example
//! fixtures used to ship as. Lives at the app level — the headless `⚙️engine` artifact node must not
//! depend on `wasm-bindgen`, and this is where every other wasm-bindgen-exported puzzle-3d surface
//! already lives.

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

use crate::artifacts::puzzle3d::engine::empty_puzzle3d_snapshot;
use crate::artifacts::puzzle3d::spr::{Puzzle3dEnvelope, Puzzle3dStore};
use crate::artifacts::puzzle3d::{Puzzle3dSnapshot, PUZZLE_3D_SCHEMA};
use std::cell::RefCell;
use store::create_document_envelope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Puzzle3dArtifactVcs {
    store: RefCell<Puzzle3dStore>,
}

#[wasm_bindgen]
impl Puzzle3dArtifactVcs {
    #[wasm_bindgen(constructor)]
    pub fn new(envelope_json: Option<String>) -> Result<Puzzle3dArtifactVcs, JsValue> {
        let store = match envelope_json {
            Some(json) => {
                let envelope: Puzzle3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Puzzle3dStore::new(envelope)
            }
            None => Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_snapshot(), None)),
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

/// 🔤️ Parses `.puzzle3d` DSL text (`Puzzle3dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.3d.json` fixture — lets
/// non-Rust consumers load the real example fixtures without duplicating the DSL grammar.
#[wasm_bindgen(js_name = puzzle3dParseDslJson)]
pub fn puzzle3d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use store::ArtifactDsl;
    let projection = Puzzle3dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| JsValue::from_str(&error.to_string()))
}
