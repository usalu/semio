//! 🌐️ Puzzle 5d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle5dArtifactVcs` handle over the typed `Puzzle5dStore`, and every other wasm-bindgen-exported
//! puzzle-5d document surface (incl. the `.puzzle5d` DSL-text parser, relocated here from the deleted
//! artifact-side `⚙️engine` per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — a
//! `wasm_bindgen`/`JsValue`-returning fn is app-boundary behaviour, not artifact schema).

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

#[path = "../../../../../../../../🌉️wasm/⚠️diagnostic/🦀️.rs"]
mod fault_bridge;

use crate::editor::puzzle5d::Puzzle5dPlayApp;
use semio_framework_plugin::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, EditorApp, PluginApp, VcsArtifactApp};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;

const PUZZLE5D_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const PUZZLE5D_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

/// 🪪️ Fixed identity-authoring ceiling the browser handle admits once before it authors its first command.
const PUZZLE5D_IDENTITY_CEILING_BYTES: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

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
    /// 🧬️ Registry-backed, because there is no other kind: `EditorApp<Puzzle5dPlayApp>` publishes a
    /// `bounded_first_step_tool_proofs!` roster, and `with_registry_on_bus` joins it against the
    /// registry's `Migrated` tool ids — the registry-LESS `VcsArtifactApp::new` panics at construction
    /// with `interactive-job.catalog-authority … generated_migrated=false, migrated={}`, and could
    /// never have admitted a typed command anyway (`admit_command_wire_with_proof`).
    pub fn create() -> js_sys::Promise {
        semio_framework_async::future_to_promise(async {
            let registry = semio_framework_plugin::AppActionRegistry::from_definition(&crate::editor::puzzle5d::create_puzzle5d_app());
            let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
            let mounted_policy = semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
            let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(PUZZLE5D_IDENTITY_CEILING_BYTES, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).map_err(js_fault)?;
            let app = VcsArtifactApp::with_registry(EditorApp::<Puzzle5dPlayApp>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
            Ok(Self { app: RefCell::new(app) }.into())
        })
    }

    #[wasm_bindgen(js_name = beginEnvelopeLoad)]
    pub fn begin_envelope_load(&self, maximum_pages: usize, maximum_bytes: usize) -> Result<Puzzle5dEnvelopeLoadHandle, JsValue> {
        if maximum_pages == 0 || maximum_pages > PUZZLE5D_ENVELOPE_MAXIMUM_PAGES || maximum_bytes == 0 || maximum_bytes > PUZZLE5D_ENVELOPE_MAXIMUM_BYTES {
            return Err(js_fault("puzzle5d-envelope.invalid-credits"));
        }
        let handle = self.app.borrow_mut().begin_artifact_envelope_ingress(maximum_pages, maximum_bytes).map_err(fault_bridge::fault_to_js)?;
        Ok(Puzzle5dEnvelopeLoadHandle { operation: handle.operation.0, generation: handle.generation.0 })
    }

    #[wasm_bindgen(js_name = admitEnvelopePage)]
    pub fn admit_envelope_page(&self, handle: &Puzzle5dEnvelopeLoadHandle, source: &js_sys::Uint8Array) -> Result<(), JsValue> {
        let len = usize::try_from(source.length()).map_err(js_fault)?;
        if len > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Err(js_fault("puzzle5d-envelope.page-too-large"));
        }
        let mut app = self.app.borrow_mut();
        app.preflight_artifact_envelope_ingress_page(handle.runtime_handle(), len).map_err(fault_bridge::fault_to_js)?;
        app.construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len, || {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            source.copy_to(&mut bytes[..len]);
            store::ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)
        })
        .map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = sealEnvelopeLoad)]
    pub fn seal_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<bool, JsValue> {
        self.app.borrow_mut().seal_artifact_envelope_ingress(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = pollEnvelopeLoad)]
    pub fn poll_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<u8, JsValue> {
        let mut app = self.app.borrow_mut();
        let demand = app.maintenance_retirement_demands(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| fault_bridge::fault_to_js(semio_framework_plugin::Fault::from(error.into_message())))?;
        app.maintenance_step(funded_turn(demand)).map_err(fault_bridge::fault_to_js)?;
        match app.advance_artifact_envelope_load(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)? {
            ArtifactEnvelopeDecodeOperationPoll::Pending => Ok(0),
            ArtifactEnvelopeDecodeOperationPoll::Progress => Ok(1),
            ArtifactEnvelopeDecodeOperationPoll::Ready => {
                if !app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)? {
                    return Ok(1);
                }
                Ok(2)
            }
            ArtifactEnvelopeDecodeOperationPoll::Cancelled => {
                let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)?;
                Ok(3)
            }
            ArtifactEnvelopeDecodeOperationPoll::Fault => {
                let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)?;
                Ok(4)
            }
        }
    }

    #[wasm_bindgen(js_name = cancelEnvelopeLoad)]
    pub fn cancel_envelope_load(&self, handle: &Puzzle5dEnvelopeLoadHandle) -> Result<(), JsValue> {
        self.app.borrow_mut().cancel_artifact_envelope_load(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = closeStep)]
    pub fn close_step(&self) -> Result<bool, JsValue> {
        let mut app = self.app.borrow_mut();
        let demand = app.close_retirement_demands(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| fault_bridge::fault_to_js(semio_framework_plugin::Fault::from(error.into_message())))?;
        match app.close_step(funded_turn(demand)).map_err(fault_bridge::fault_to_js)? {
            semio_framework_plugin::PluginLifecycleStep::Complete(_) => Ok(true),
            semio_framework_plugin::PluginLifecycleStep::Progress(_) | semio_framework_plugin::PluginLifecycleStep::AwaitingInput { .. } | semio_framework_plugin::PluginLifecycleStep::Blocked { .. } => Ok(false),
        }
    }
}

//#region 🔖️WasmBridge
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets
/// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the
/// DSL grammar.
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use store::ArtifactDsl;
    let projection = crate::Puzzle5dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    Ok(semio_framework_pack_json::to_json_string(&projection))
}
//#endregion 🔖️WasmBridge

/// 🎟️ One close or maintenance turn funded exactly by what the owner quoted for it.
fn funded_turn(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}
