//! 🌐️ Puzzle 3d play app — the browser wasm-bindgen bridge (`wasm32`, non-WASI-P2 only): a
//! `Puzzle3dArtifactVcs` handle over the typed `Puzzle3dStore`, plus a `.puzzle3d` DSL-text parser
//! that hands non-Rust consumers (e.g. Storybook stories) the same camelCase JSON shape the example
//! fixtures used to ship as. Lives at the app level — the artifact (schema + io) must not depend on
//! `wasm-bindgen`, and this is where every other wasm-bindgen-exported puzzle-3d surface already lives.

#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

#[path = "../../../../../../../../🌉️wasm/⚠️diagnostic/🦀️.rs"]
pub mod fault_bridge;
use crate::Puzzle3dPlayApp;
use semio_framework_plugin::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, EditorApp, PluginApp, VcsArtifactApp};
use semio_s_plugin_puzzle_jobs::BrowserCaller;
use std::cell::{Cell, RefCell};
use wasm_bindgen::prelude::*;
pub use js_sys::{Promise, Uint8Array};

pub type Puzzle3dApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

const PUZZLE3D_ENVELOPE_MAXIMUM_PAGES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const PUZZLE3D_ENVELOPE_MAXIMUM_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

/// 🪪️ Fixed identity-authoring ceiling the browser handle admits once before it authors its first command.
const PUZZLE3D_IDENTITY_CEILING_BYTES: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;

fn js_fault(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// 🧯️ Surfaces a value refusal to JavaScript as the structured fault its message names.
fn value_to_js(error: semio_framework_value::ValueError) -> JsValue {
    fault_bridge::fault_to_js(semio_framework_plugin::Fault::from(error.into_message()))
}

/// 🧯️ Surfaces a lifecycle failure to JavaScript: the original fault when the cause is one, else the whole cause.
fn lifecycle_to_js(failure: semio_framework_plugin::PluginLifecycleFailure) -> JsValue {
    match failure.primary {
        semio_framework_plugin::PluginLifecyclePrimary::Fault(fault) => fault_bridge::fault_to_js(fault),
        other => js_fault(format!("{other:?}")),
    }
}

#[wasm_bindgen]
pub struct Puzzle3dEnvelopeLoadHandle {
    operation: u64,
    generation: u64,
}

impl Puzzle3dEnvelopeLoadHandle {
    fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle {
        ArtifactEnvelopeDecodeOperationHandle { operation: semio_framework_job::OperationId(self.operation), generation: semio_framework_job::Generation(self.generation) }
    }
}

#[wasm_bindgen]
impl Puzzle3dEnvelopeLoadHandle {
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
pub struct Puzzle3dArtifactVcs {
    app: RefCell<Puzzle3dApp>,
    caller: RefCell<BrowserCaller>,
    closed: Cell<bool>,
}

#[wasm_bindgen]
impl Puzzle3dArtifactVcs {
    /// 🧬️ Registry-backed, because there is no other kind: `EditorApp<Puzzle3dPlayApp>` publishes a
    /// `bounded_first_step_tool_proofs!` roster, and `with_registry_on_bus` joins it against the
    /// registry's `Migrated` tool ids — the registry-LESS `VcsArtifactApp::new` panics at construction
    /// with `interactive-job.catalog-authority … generated_migrated=false, migrated={}`, and could
    /// never have admitted a typed command anyway (`admit_command_wire_with_proof`).
    pub fn create() -> Promise {
        semio_framework_async::future_to_promise(async {
            let registry = semio_framework_plugin::AppActionRegistry::from_definition(&crate::create_puzzle3d_app());
            let mounted_policy = semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
            let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(PUZZLE3D_IDENTITY_CEILING_BYTES, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).map_err(js_fault)?;
            let app = VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.map_err(fault_bridge::fault_to_js)?;
            let caller = BrowserCaller::admit(mounted_policy.maintenance).map_err(value_to_js)?;
            Ok(Self { app: RefCell::new(app), caller: RefCell::new(caller), closed: Cell::new(false) }.into())
        })
    }

    #[wasm_bindgen(js_name = beginEnvelopeLoad)]
    pub fn begin_envelope_load(&self, maximum_pages: usize, maximum_bytes: usize) -> Result<Puzzle3dEnvelopeLoadHandle, JsValue> {
        if maximum_pages == 0 || maximum_pages > PUZZLE3D_ENVELOPE_MAXIMUM_PAGES || maximum_bytes == 0 || maximum_bytes > PUZZLE3D_ENVELOPE_MAXIMUM_BYTES {
            return Err(js_fault("puzzle3d-envelope.invalid-credits"));
        }
        let handle = self.app.borrow_mut().begin_artifact_envelope_ingress(maximum_pages, maximum_bytes).map_err(fault_bridge::fault_to_js)?;
        Ok(Puzzle3dEnvelopeLoadHandle { operation: handle.operation.0, generation: handle.generation.0 })
    }

    #[wasm_bindgen(js_name = admitEnvelopePage)]
    pub fn admit_envelope_page(&self, handle: &Puzzle3dEnvelopeLoadHandle, source: &Uint8Array) -> Result<(), JsValue> {
        let len = usize::try_from(source.length()).map_err(js_fault)?;
        if len > store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Err(js_fault("puzzle3d-envelope.page-too-large"));
        }
        let mut app = self.app.borrow_mut();
        app.preflight_artifact_envelope_ingress_page(handle.runtime_handle(), len).map_err(fault_bridge::fault_to_js)?;
        app.construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len, || {
            let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            source.copy_to(&mut bytes[..len]);
            store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)
        })
        .map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = sealEnvelopeLoad)]
    pub fn seal_envelope_load(&self, handle: &Puzzle3dEnvelopeLoadHandle) -> Result<bool, JsValue> {
        let mut app = self.app.borrow_mut();
        self.caller.borrow_mut().turn(|cx| app.seal_artifact_envelope_ingress(handle.runtime_handle(), cx)).map_err(value_to_js)?.map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = pollEnvelopeLoad)]
    pub fn poll_envelope_load(&self, handle: &Puzzle3dEnvelopeLoadHandle) -> Result<u8, JsValue> {
        let mut app = self.app.borrow_mut();
        let demand = app.maintenance_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| fault_bridge::fault_to_js(semio_framework_plugin::Fault::from(error.into_message())))?;
        let poll = self
            .caller
            .borrow_mut()
            .turn(|cx| {
                app.maintenance_step(cx, funded_turn(demand))?;
                app.advance_artifact_envelope_load(handle.runtime_handle(), cx)
            })
            .map_err(value_to_js)?
            .map_err(lifecycle_to_js)?;
        match poll {
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
    pub fn cancel_envelope_load(&self, handle: &Puzzle3dEnvelopeLoadHandle) -> Result<(), JsValue> {
        self.app.borrow_mut().cancel_artifact_envelope_load(handle.runtime_handle()).map_err(fault_bridge::fault_to_js)
    }

    #[wasm_bindgen(js_name = closeStep)]
    pub fn close_step(&self) -> Result<bool, JsValue> {
        let mut caller = self.caller.borrow_mut();
        if !self.closed.get() {
            let mut app = self.app.borrow_mut();
            let demand = app.close_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(value_to_js)?;
            let step = caller.turn(|cx| app.close_step(cx, funded_turn(demand))).map_err(value_to_js)?.map_err(lifecycle_to_js)?;
            if !matches!(step, semio_framework_plugin::PluginLifecycleStep::Complete(_)) {
                return Ok(false);
            }
            self.closed.set(true);
        }
        caller.retire_step().map_err(value_to_js)
    }
}

/// 🔤️ Parses `.puzzle3d` DSL text (`Puzzle3dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.3d.json` scene_snapshot — lets
/// non-Rust consumers load the real example fixtures without duplicating the DSL grammar.
#[wasm_bindgen(js_name = puzzle3dParseDslJson)]
pub fn puzzle3d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use pack::snapshot::ArtifactDsl;
    let projection = Puzzle3dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    Ok(semio_framework_pack_json::to_json_string(&projection))
}

/// 🎟️ One close or maintenance turn funded exactly by what the owner quoted for it.
pub fn funded_turn(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}
