//! 🌐️ Puzzle 5d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle5dDocumentVcs` handle over the typed `Puzzle5dStore`. Lives at the app level — the
//! `.puzzle5d` DSL-text parser stays in the headless `⚙️engine` artifact node next to the session it
//! shares a target gate with, and this is where every other wasm-bindgen-exported puzzle-5d document
//! surface already lived.

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

use crate::artifacts::puzzle5d::engine::empty_puzzle5d_projection;
use crate::artifacts::puzzle5d::spr::{Puzzle5dEnvelope, Puzzle5dStore};
use crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA;
use std::cell::RefCell;
use store::create_document_envelope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Puzzle5dDocumentVcs {
    store: RefCell<Puzzle5dStore>,
}

#[wasm_bindgen]
impl Puzzle5dDocumentVcs {
    #[wasm_bindgen(constructor)]
    pub fn new(envelope_json: Option<String>) -> Result<Puzzle5dDocumentVcs, JsValue> {
        let store = match envelope_json {
            Some(json) => {
                let envelope: Puzzle5dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Puzzle5dStore::new(envelope)
            }
            None => Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_projection(), None)),
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
