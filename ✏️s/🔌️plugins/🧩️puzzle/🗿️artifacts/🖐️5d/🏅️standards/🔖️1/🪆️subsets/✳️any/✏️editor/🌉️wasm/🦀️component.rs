//! 🌐️ Puzzle 5d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle5dArtifactVcs` handle over the typed `Puzzle5dStore`, and every other wasm-bindgen-exported
//! puzzle-5d document surface (incl. the `.puzzle5d` DSL-text parser, relocated here from the deleted
//! artifact-side `⚙️engine` per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — a
//! `wasm_bindgen`/`JsValue`-returning fn is app-boundary behaviour, not artifact schema).

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

use crate::editor::puzzle5d::Puzzle5dPlayApp;
use semio_framework_plugin::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, EditorApp, PluginApp, VcsArtifactApp};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;

const PUZZLE5D_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const PUZZLE5D_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

fn js_fault(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[wasm_bindgen]
pub struct Puzzle5dEnvelopeLoadHandle {
    operation: u64,
    generation: u64,
}

impl Puzzle5dEnvelopeLoadHandle {
    fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle {
        ArtifactEnvelopeDecodeOperationHandle { operation: semio_framework_job::OperationId(self.operation), generation: semio_framework_job::Generation(self.generation) }
    }
}

#[wasm_bindgen]
impl Puzzle5dEnvelopeLoadHandle {
    #[wasm_bindgen(getter)]
    pub fn operation(&self) -> u64 {
        self.operation
    }

    #[wasm_bindgen(getter)]
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

#[wasm_bindgen]
pub struct Puzzle5dArtifactVcs {
    app: RefCell<Puzzle5dApp>,
}

#[wasm_bindgen]
impl Puzzle5dArtifactVcs {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Puzzle5dArtifactVcs, JsValue> {
        let app = VcsArtifactApp::new(EditorApp::<Puzzle5dPlayApp>::default()).await;
        Ok(Self { app: RefCell::new(app) })
    }

    #[wasm_bindgen(js_name = beginEnvelopeLoad)]
    pub fn begin_envelope_load(&self, maximum_pages: usize, maximum_bytes: usize) -> Result<Puzzle5dEnvelopeLoadHandle, JsValue> {
        if maximum_pages == 0 || maximum_pages > PUZZLE5D_ENVELOPE_MAXIMUM_PAGES || maximum_bytes == 0 || maximum_bytes > PUZZLE5D_ENVELOPE_MAXIMUM_BYTES {
            return Err(js_fault("puzzle5d-envelope.invalid-credits"));
        }
        let handle = self.app.borrow_mut().begin_artifact_envelope_ingress(maximum_pages, maximum_bytes).map_err(js_fault)?;
        Ok(Puzzle5dEnvelopeLoadHandle { operation: handle.operation.0, generation: handle.generation.0 })
    }

    #[wasm_bindgen(js_name = admitEnvelopePage)]
    pub fn admit_envelope_page(&self, handle: &Puzzle5dEnvelopeLoadHandle, source: &js_sys::Uint8Array) -> Result<(), JsValue> {
        let len = usize::try_from(source.length()).map_err(js_fault)?;
        if len > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Err(js_fault("puzzle5d-envelope.page-too-large"));
        }
        let mut app = self.app.borrow_mut();
        app.preflight_artifact_envelope_ingress_page(handle.runtime_handle(), len).map_err(js_fault)?;
        app.construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len, || {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            source.copy_to(&mut bytes[..len]);
            store::ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)
        })
        .map_err(js_fault)
    }

    #[wasm_bindgen(js_name = sealEnvelopeLoad)]
    pub fn seal_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<bool, JsValue> {
        self.app.borrow_mut().seal_artifact_envelope_ingress(handle.runtime_handle()).map_err(js_fault)
    }

    #[wasm_bindgen(js_name = pollEnvelopeLoad)]
    pub fn poll_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<u8, JsValue> {
        let mut app = self.app.borrow_mut();
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(js_fault)?;
        match app.advance_artifact_envelope_load(handle.runtime_handle()).map_err(js_fault)? {
            ArtifactEnvelopeDecodeOperationPoll::Pending => Ok(0),
            ArtifactEnvelopeDecodeOperationPoll::Progress => Ok(1),
            ArtifactEnvelopeDecodeOperationPoll::Ready => {
                if !app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)? {
                    return Ok(1);
                }
                Ok(2)
            }
            ArtifactEnvelopeDecodeOperationPoll::Cancelled => {
                let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)?;
                Ok(3)
            }
            ArtifactEnvelopeDecodeOperationPoll::Fault => {
                let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)?;
                Ok(4)
            }
        }
    }

    #[wasm_bindgen(js_name = cancelEnvelopeLoad)]
    pub fn cancel_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<(), JsValue> {
        self.app.borrow_mut().cancel_artifact_envelope_load(handle.runtime_handle()).map_err(js_fault)
    }

    #[wasm_bindgen(js_name = closeStep)]
    pub fn close_step(&self) -> Result<bool, JsValue> {
        match self.app.borrow_mut().close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(js_fault)? {
            semio_framework_plugin::PluginCloseStep::Complete => Ok(true),
            semio_framework_plugin::PluginCloseStep::Pending { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. } => Ok(false),
        }
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
