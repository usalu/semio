extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::artifact_owned_tool_job_context_identity_digest;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::BOUNDED_FIRST_STEP_FACTORY;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedFirstStepProof;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
#[cfg(test)]
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page;
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedToolFault;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_value::DslValue;
use protocol::{OpBinary, OpText};
use semio_framework::kernel::UiDirtyScope;
use semio_framework::kernel::{Effect, HistoryPatch, InvocationResult};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use pack::snapshot::ArtifactPack;
use store::{Mutation, MutationDiff};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_decl_app::ArtifactCompletionFault;
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)]
use semio_framework_plugin_app_builder::App;
#[cfg(test)]
use semio_framework_plugin_ownership::bounded_ownership::plugin_demand_grant;
#[cfg(test)]
use semio_framework_plugin_operation::ToolOperationKey;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
use semio_framework_plugin_decl_views::child_view::MemberKey;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::InteractionHoverState;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_interaction::interaction_topology::is_inline_interaction_verb;
use semio_framework_plugin_decl_views::PeerPresenceRoot;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
#[cfg(test)]
use semio_framework_plugin_window_kits::AppActionRegistry;
#[cfg(test)]
use crate::reserved_jobs::QualifiedBoundedFirstStepProof;
#[cfg(test)]
use crate::lifted_full_operation_job::TypedCommandFullOperationJobFactory;



pub use crate::lifted_job::is_tool_run_action_id;






/// 🧬️ Generic wrapper turning any typed {@link ArtifactApp} into the object-safe runtime
/// {@link PluginApp}. Owns a persistent `ArtifactStore<Snapshot, Mutation>` — the single source of
/// truth for the app's document across every call — intercepts the six injected history actions into
/// `ArtifactCommand`s, dispatches `Apply` or a tool transaction's commands for typed operations, and builds an
/// `InvocationResult` whose inverses come from the just-recorded `Edit.inverse`. A snapshot+history
/// cache keyed on `(store generation, log generation, history filter)` keeps renders O(1). Holds an
/// {@link AppActionRegistry} to enforce the actions contract before/after delegating to the app.
/// Host `ArtifactSession` already mirrors opaque document/config/draft packs; typed stores here
/// remain guest-owned until `AppCommand::PureCommand` returns `AppFrame::Emit` for host apply.
/// 🔀️ Guest-side transaction membership state (contract-freeze.md §5.2/§5.6/§5.8-§5.10):
/// captured by `VcsArtifactApp::transaction_prepare` when this instance accepts a
/// `TransactionPrepare` and consumed by `transaction_commit`, which applies `ops` as exactly
/// ONE `Edit` (`group_id = txn_id`, every `MutationMeta.origin = origin`) — the
/// one-edit-per-member invariant `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`'s tail-based group
/// undo depends on. `base_generation` is the store generation `transaction_prepare` observed;
/// `transaction_commit` rejects (`transaction.generation-mismatch`) if the live generation has
/// since moved.
#[doc(hidden)]
pub struct PendingTransaction<Op> {
    pub txn_id: String,
    pub ops: Vec<Op>,
    pub children: Vec<ChildEmit>,
    pub origin: protocol::MutationOrigin,
    pub base_generation: u64,
}

/// 🧾️ The op packs one dispatch produced, per lane, as the channel's `AppFrame::Emit` carries them:
/// `document`/`config`/`draft` are `protocol::encode_ops_vec` packs, `children` the owned-child share
/// ([`ChildEmit::encode_groups`]), filled only by the agent preview (an applying dispatch has already
/// committed its children in-guest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EmitWire {
    pub document: Vec<u8>,
    pub config: Vec<u8>,
    pub draft: Vec<u8>,
    pub children: Vec<u8>,
}

/// 🔀️ A `dispatch_emit` gesture whose `artifact_mutations` carried foreign steps (contract
/// §5.1) — stashed instead of applied, drained by `plugin_exchange` via
/// `PluginApp::take_pending_transaction_proposal` and framed as `AppFrame::TransactionProposal`.
pub struct TransactionProposalDraft {
    pub local_ops: Vec<Vec<u8>>,
    pub foreign: Vec<protocol::ForeignStep>,
}

#[doc(hidden)]
#[cfg(test)]
pub fn test_unregistered_tool_job_admission_rejected<A: ArtifactApp>(catalog: &AppActionRegistry, generated: &[&str]) -> bool {
    crate::reserved_jobs::tool_job_registration::<A>(catalog, A::APP_ID, A::DOCUMENT_SCHEMA, generated, &semio_framework::ActionBus::new(), &BTreeMap::new()).is_err()
}

/// 🧪 Executes hostile proof substitutions inside the private authority module in test builds only.
#[doc(hidden)]
#[cfg(test)]
pub fn test_retained_factory_proof_join<A, F, G, W>(mut catalog: AppActionRegistry, controller: &'static str, tool: &'static str, factory: F)
where
    A: ArtifactApp,
    F: ArtifactOwnedToolJobFactory<Owner = A>,
    G: ArtifactOwnedToolJobFactory<Owner = A>,
    W: ArtifactApp,
{
    let fixture: Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔬️tool-factory-proof.json")).expect("language-neutral factory join fixture");
    let bus = semio_framework::ActionBus::new();
    let empty_bus = semio_framework::ActionBus::new();
    let contract = semio_framework::ToolJobFactory::execution_contract(&factory);
    let mut factories = ArtifactToolFactoryRegistry::<A>::new(&bus, controller);
    factories.register(factory).expect("exact app-owned factory");
    let registrations = factories.finish();
    let mut declaration = catalog.actions.get("noopMutation").expect("fixture action").clone();
    declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    catalog.actions.insert(tool.into(), declaration);
    let exact = ArtifactBoundedFirstStepProof::new::<A>(file!(), controller, "TestRetainedCommandFactory", tool, A::DOCUMENT_SCHEMA, contract).with_factory_type::<A, F>();
    for case in fixture["cases"].as_array().expect("join cases") {
        let mut row = exact.clone();
        let mut registered = registrations.clone();
        let mut live_bus = &bus;
        let change = case["change"].as_str().expect("case change");
        match change {
            "none" | "duplicate" => {}
            "missingType" => row.factory_type_id = None,
            "wrongType" => row.factory_type_id = Some(std::any::TypeId::of::<A>()),
            "sameOwnerDifferentFactory" => row = row.with_factory_type::<A, G>(),
            "wrongTypeName" => row.factory_type_name = Some(std::any::type_name::<A>()),
            "wrongFactory" => row.factory = "NotTheRegisteredFactory",
            "wrongOwner" => row.owner = ToolOwnerWitness::of::<W>(),
            "wrongController" => row.controller_id = "s.test.other@1/*#editor",
            "wrongDocumentSchema" => row.artifact_schema = "semio.test.other.v1",
            "wrongTool" => row.tool_id = "otherTool",
            "wrongContract" => row.contract = semio_framework::ToolExecutionContract::resumable(4_096, 5, 1, 4_096, 7_500, 1, 1),
            "wrongPayloadSchema" => registered.get_mut(tool).expect("registered tool").schema_id = "wrong.payload.schema".into(),
            "missingRegistration" => registered.clear(),
            "differentBus" => live_bus = &empty_bus,
            "sentinel" => {
                row.factory = BOUNDED_FIRST_STEP_FACTORY;
                row.factory_type_id = None;
                row.factory_type_name = None;
            }
            other => panic!("unknown fixture change {other}"),
        }
        let rows = if change == "duplicate" { vec![row.clone(), row] } else { vec![row] };
        let actual = crate::reserved_jobs::validate_tool_job_rows::<A>(&catalog, controller, A::DOCUMENT_SCHEMA, &[tool], live_bus, &registered, rows);
        assert_eq!(actual.is_ok(), case["accepted"].as_bool().expect("expected admission"), "{}", case["id"]);
        if let Ok((resolved_controller, generic)) = actual {
            assert_eq!(resolved_controller, controller);
            assert!(generic.is_empty(), "a registered custom factory must never construct the generic fallback");
        }
    }
    assert!(TypedCommandFullOperationJobFactory::<A>::from_proof(QualifiedBoundedFirstStepProof { owner: exact.owner, row: exact }).is_none());
    let generic = ArtifactBoundedFirstStepProof::new::<A>(file!(), controller, BOUNDED_FIRST_STEP_FACTORY, tool, A::DOCUMENT_SCHEMA, contract);
    let (_, mut generic_proofs) = crate::reserved_jobs::validate_tool_job_rows::<A>(&catalog, controller, A::DOCUMENT_SCHEMA, &[tool], &empty_bus, &BTreeMap::new(), vec![generic]).expect("unregistered exact generic reducer remains supported");
    assert_eq!(generic_proofs.len(), 1);
    assert!(TypedCommandFullOperationJobFactory::<A>::from_proof(generic_proofs.pop().expect("generic proof")).is_some());
}

/// 📏️ Largest `setContributions` raw JSON wire ONE app must admit — the host hands a receiver the
/// whole capability closure that receiver's registry `consumes` row names, in one crossing, and
/// the guest re-encodes it as `["setContributions",{"json":…}]` before
/// `admit_command_json_with_proof` prices it against the addressed tool's `max_raw_wire_bytes`.
///
/// 📐️ Measured 2026-09-16 over the built dev manifests (ticket
/// 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS): the demonstrator consumes `cad.computer` (4
/// extensions), `process.machines` (4) and `sourcing.module` (3) — 29 909 bytes of pack JSON,
/// 38 081 bytes once escaped into the command wire. This is that, rounded to 48 KiB so it stays
/// under the 64 KiB contiguous guest-allocation ceiling with room for one more extension. An app
/// that declares the app-wide default instead refuses the real pack with
/// `typed command raw JSON exceeds its registered retained-page admission`.
pub const CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES: usize = 49_152;

pub const TYPED_OPERATION_RESULT_PAGE_BYTES: usize = 4_096;

pub const TYPED_OPERATION_MAXIMUM_RETRIES: u8 = 2;

/// 📊️ What the typed-operation publication ladder spends its units on, on the calling thread. One unit is one
/// step of the publication state machine; a HOST TURN is one or more units
/// ([`TYPED_OPERATION_PUBLICATION_PUMPS`]), so "how many turns does one mutation cost" is only
/// answerable by splitting the units by the ladder that ran them.
///
/// 🧾️ Before wave B54 every unit was its own host round trip, and one `deleteSelection` on a
/// one-object document spent 104–235 of them (wave B44 §2.3) with no way to say which ladder owned
/// them. This census is that instrument: `store` is the artifact/config/draft apply-batch phase
/// machine, `page` a unit that produced a result page the host must acknowledge (the only unit that
/// is INHERENTLY one round trip), `retirement` the slot-release ladder, `worker` a mounted job step,
/// `latest_wins` the coalesced-command lane, and `idle` a unit that found nothing advanceable.
/// Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TypedOperationUnitCensus {
    pub units: u64,
    pub store: u64,
    pub page: u64,
    pub retirement: u64,
    pub worker: u64,
    pub latest_wins: u64,
    pub idle: u64,
}

impl std::fmt::Display for TypedOperationUnitCensus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "units={} store={} page={} retirement={} worker={} latestWins={} idle={}", self.units, self.store, self.page, self.retirement, self.worker, self.latest_wins, self.idle)
    }
}

impl std::ops::Sub for TypedOperationUnitCensus {
    type Output = Self;

    fn sub(self, base: Self) -> Self {
        Self {
            units: self.units.saturating_sub(base.units),
            store: self.store.saturating_sub(base.store),
            page: self.page.saturating_sub(base.page),
            retirement: self.retirement.saturating_sub(base.retirement),
            worker: self.worker.saturating_sub(base.worker),
            latest_wins: self.latest_wins.saturating_sub(base.latest_wins),
            idle: self.idle.saturating_sub(base.idle),
        }
    }
}

/// 📊️ One counted ladder of the publication census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypedOperationUnitKind {
    Store,
    Page,
    Retirement,
    Worker,
    LatestWins,
    Idle,
}

thread_local! {
    /// 🧵️ The census counters of the thread that drives publication units. Per thread, because the ladder runs on
    /// the host turn's own thread and a reading must attribute exactly that host's dispatches: a process-wide
    /// counter folds every concurrently driven instance (another document, another law of the same test
    /// binary) into the reading and turns a document-independent ladder into a load-dependent number.
    static PUBLICATION_UNITS: [std::cell::Cell<u64>; 7] = const { [const { std::cell::Cell::new(0) }; 7] };
}

/// 📊️ Counts one publication unit against its ladder on the calling thread.
pub fn record_typed_operation_unit(kind: TypedOperationUnitKind) {
    let index = match kind {
        TypedOperationUnitKind::Store => 1,
        TypedOperationUnitKind::Page => 2,
        TypedOperationUnitKind::Retirement => 3,
        TypedOperationUnitKind::Worker => 4,
        TypedOperationUnitKind::LatestWins => 5,
        TypedOperationUnitKind::Idle => 6,
    };
    PUBLICATION_UNITS.with(|units| {
        units[0].set(units[0].get().saturating_add(1));
        units[index].set(units[index].get().saturating_add(1));
    });
}

/// 📊️ The calling thread's census as of now. Subtract two readings to attribute one dispatch.
pub fn typed_operation_unit_census() -> TypedOperationUnitCensus {
    PUBLICATION_UNITS.with(|units| {
        let read = |index: usize| units[index].get();
        TypedOperationUnitCensus { units: read(0), store: read(1), page: read(2), retirement: read(3), worker: read(4), latest_wins: read(5), idle: read(6) }
    })
}

/// 🏃️ Upper bound on worker pump iterations one reactor turn spends on a single mounted worker
/// session — a Worker-stage typed operation or a live artifact-envelope decode.
pub const INTERACTIVE_TURN_WORKER_PUMPS: usize = 256;

/// ⏱️ Wall-clock slice one reactor turn spends driving one mounted worker session (half the interactive lane ceiling).
pub const INTERACTIVE_TURN_WORKER_WALL_US: u64 = 4_000;

/// 🏃️ Publication units one continuation unit may run before it hands its turn back, the exact twin of
/// [`INTERACTIVE_TURN_WORKER_PUMPS`] for the publication ladder.
pub const TYPED_OPERATION_PUBLICATION_PUMPS: usize = 256;

/// 🩺️ Consecutive unchanged publication units before one stalled typed operation is named on the
/// diagnostic channel — far above any legitimate multi-unit ladder step, so a healthy operation
/// never prints.
#[doc(hidden)]
pub const TYPED_OPERATION_STALL_WITNESS_FLOOR: u64 = 64;

/// 🛑️ Unchanged `Publishing`-stage units before the operation is terminated with
/// `interactive-job.publication-stalled` rather than left spinning. A power of two so the stall
/// trace's own streak rule names it on the way past.
#[doc(hidden)]
pub const TYPED_OPERATION_STALL_FAULT_CEILING: u64 = 4_096;

/// 🧮️ Folds the publication ladder's monotone progress term into the one-`u64` witness key the
/// streak counter remembers between turns, so two witnesses that differ ONLY in how far the
/// publication has advanced are never mistaken for the same one.
#[doc(hidden)]
pub const TYPED_OPERATION_STALL_PROGRESS_MIX: u64 = 0x9E37_79B9_7F4A_7C15;

/// 🩺️ The stage code [`ArtifactApp::typed_operation_stall_witness`] gives a `Publishing` operation.
#[doc(hidden)]
pub const TYPED_OPERATION_STALL_PUBLISHING_STAGE: u8 = 1;

#[doc(hidden)]
pub const TYPED_OPERATION_HOST_OUTBOX_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TypedOperationResultToken {
    pub receiver: u32,
    pub operation: u64,
    pub generation: u64,
    pub sequence: u32,
    pub attempt: u8,
}

/// 🏁️ One mounted typed operation's terminal witness, captured the instant its Terminal result
/// page is minted and drained exactly once by `advance_typed_operation_output` into an
/// unsolicited `AppFrame::OperationCompleted`. The command that started the operation resolved
/// long before this — its "started" `InvocationResult` carries no outcome — so this is the ONLY
/// carrier of the operation's final `UiDirtyScope`, command-log delta and content revision.
pub struct TypedOperationCompletion {
    pub operation: u64,
    pub revision: u64,
    pub ui_scope: UiDirtyScope,
    pub history_patch: Option<HistoryPatch>,
}

/// 🎞️ One mounted typed operation's `Ui` progress unit as the host receives it: the emit's
/// `UiDirtyScope` (widened by every interaction verb folded inline, ticket
/// 26/09/16/INPUT-CAUSALITY-LEDGER §2 C) and, when such a verb was folded, the last verb's
/// leftover `InteractionView` — published as the `output` of the unsolicited `Invocation`
/// frame that carries the scope (`advance_typed_operation_output`), the typed ladder's twin of
/// the `Invocation.output.interactionView` lane a host-dispatched verb answers on.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedOperationUiProgress {
    pub ui_scope: UiDirtyScope,
    pub leftover: Option<TypedOperationLeftover>,
    /// ⏪️ The history delta a driver turn owes the hosts (history-edit stage and replay progress), riding the same
    /// unsolicited frame as the scope so the band moves without a dispatch.
    pub history_patch: Option<HistoryPatch>,
}

/// 🕹️ The leftover `InteractionView` envelope (`{ "interactionView": … }`, exactly what
/// `leftover_interaction_view_from` builds for a host-dispatched verb) a mounted typed operation's
/// inline-folded interaction verb owes the host, tagged with the operation that folded it so
/// `plugin_exchange` can merge it into THAT command's own `Invocation` reply (`output` =
/// `{ operationId, generation, interactionView }`) when the fold landed inside the admitting
/// exchange — the one lane `applyLeftoverInteractionView` (`🏛️ShellHost`) already reads for a
/// typed command. It also rides the `Ui` progress frame's `output` for a fold that lands on a
/// later turn.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedOperationLeftover {
    pub operation: u64,
    pub view: DslValue,
}

/// 🏁️ The terminal witness before its command-log delta and revision are resolved — the queue
/// entry itself, minted inside the publication step that has no `&mut self` history authority.
#[doc(hidden)]
pub struct TypedOperationCompletionWitness {
    pub operation: u64,
    pub ui_scope: UiDirtyScope,
}

#[derive(Clone)]
pub struct TypedOperationResultPage {
    pub token: TypedOperationResultToken,
    pub lane: TypedOperationResultLane,
    pub bytes: [u8; TYPED_OPERATION_RESULT_PAGE_BYTES],
    pub len: usize,
}

/// 📦️ The Download lane's wire shape is a flat 4-element JSON array — matching what
/// `serde_json` produced for the ad hoc `(&str, &str, Option<&str>, usize)` tuple this
/// replaces. `ToValue`'s tuple impls (`🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`) stop at
/// 3 elements, and a `#[derive(ToValue)]` struct encodes as an object, not an array, so this
/// hand-written impl is the local (no shared-file edit) way to keep the array shape exact.
#[doc(hidden)]
pub struct DownloadResultPayload<'a> {
    pub filename: &'a str,
    pub mime_type: &'a str,
    pub encoding: Option<&'a str>,
    pub bytes: usize,
}

impl semio_framework_value::ToValue for DownloadResultPayload<'_> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![semio_framework_value::ToValue::to_value(&self.filename), semio_framework_value::ToValue::to_value(&self.mime_type), semio_framework_value::ToValue::to_value(&self.encoding), semio_framework_value::ToValue::to_value(&self.bytes)])
    }
}

impl TypedOperationResultPage {
    pub const RENDERER_PAGE_MAGIC: &'static [u8] = b"semio.typed-operation-page.v1\0";
    pub const RENDERER_ACK_MAGIC: &'static [u8] = b"semio.typed-operation-ack.v1\0";

    pub fn try_new(token: TypedOperationResultToken, lane: TypedOperationResultLane, bytes: &[u8]) -> Result<Self, Fault> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed page authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn try_serialize<T: semio_framework_value::ToValue>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
        let text = semio_framework_pack_json::to_json_string(value);
        let encoded = text.as_bytes();
        if encoded.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed incremental encoder authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: encoded.len() };
        page.bytes[..encoded.len()].copy_from_slice(encoded);
        Ok(page)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn renderer_exchange_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::RENDERER_PAGE_MAGIC.len() + 30 + self.len);
        bytes.extend_from_slice(Self::RENDERER_PAGE_MAGIC);
        bytes.extend_from_slice(&self.token.receiver.to_le_bytes());
        bytes.extend_from_slice(&self.token.operation.to_le_bytes());
        bytes.extend_from_slice(&self.token.generation.to_le_bytes());
        bytes.extend_from_slice(&self.token.sequence.to_le_bytes());
        bytes.push(self.token.attempt);
        bytes.push(match self.lane {
            TypedOperationResultLane::Artifact => 0,
            TypedOperationResultLane::Config => 1,
            TypedOperationResultLane::Draft => 2,
            TypedOperationResultLane::Presence => 3,
            TypedOperationResultLane::Transient => 4,
            TypedOperationResultLane::Child => 5,
            TypedOperationResultLane::Effect => 6,
            TypedOperationResultLane::Event => 7,
            TypedOperationResultLane::Ui => 8,
            TypedOperationResultLane::Download => 9,
            TypedOperationResultLane::Terminal => 10,
            TypedOperationResultLane::Fault => 11,
            TypedOperationResultLane::Interaction => 12,
            TypedOperationResultLane::WindowTransient => 13,
            TypedOperationResultLane::WindowConfig => 14,
        });
        bytes.extend_from_slice(&(self.len as u32).to_le_bytes());
        bytes.extend_from_slice(self.bytes());
        bytes
    }

    pub fn renderer_ack_token(bytes: &[u8]) -> Option<TypedOperationResultToken> {
        let body = bytes.strip_prefix(Self::RENDERER_ACK_MAGIC)?;
        if body.len() != 25 {
            return None;
        }
        Some(TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        })
    }
}

/// 📸️ Every root one typed command is reduced against — see `VcsArtifactApp::capture_typed_command_roots`.
#[doc(hidden)]
pub struct TypedCommandRoots<A: ArtifactApp> {
    pub snapshot: std::sync::Arc<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub history: std::sync::Arc<HistoryView>,
    pub children: std::sync::Arc<ChildContentView>,
    pub draft_snapshot: std::sync::Arc<A::Draft>,
    pub interaction_state: std::sync::Arc<protocol::InteractionState>,
    pub interaction_hover: InteractionHoverState,
    pub presence_peers: std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>,
    pub transient: std::sync::Arc<A::Transient>,
    pub peer_presence: std::sync::Arc<PeerPresenceRoot>,
    pub canonical_base_revision: [u8; 32],
    pub base_revision: semio_framework_job::RevisionId,
    pub generation: semio_framework_job::Generation,
    pub config_generation: u64,
    pub draft_generation: u64,
    pub presence_generation: u64,
    pub transient_generation: u64,
    pub window_config_authority: Option<semio_framework_plugin_window_config::WindowConfigAuthority>,
    pub window_transient_authority: Option<semio_framework_plugin_window_transient::window_transient::WindowTransientAuthority>,
}

#[doc(hidden)]
#[cfg(test)]
pub fn test_artifact_owned_tool_job_context_identity_digest(app_instance_id: u32, canonical_base_revision: [u8; 32], draft_generation: u64, transient_generation: u64, children_digest: u64) -> u64 {
    artifact_owned_tool_job_context_identity_digest(app_instance_id, None, canonical_base_revision, draft_generation, transient_generation, children_digest, (None, None))
}

#[doc(hidden)]
#[cfg(test)]
pub fn test_window_transient_context_identity(snapshot: Option<&WindowTransientSnapshot>) -> u64 {
    artifact_owned_tool_job_context_identity_digest(7, None, [17; 32], 0, 0, 0, (None, snapshot))
}

#[doc(hidden)]
#[cfg(test)]
pub fn test_window_config_context_identity(snapshot: Option<&WindowConfigSnapshot>) -> u64 {
    artifact_owned_tool_job_context_identity_digest(7, None, [17; 32], 0, 0, 0, (snapshot, None))
}

/// 🪙️ Admits one child frame under fixed host limits independent of a producer demand.
#[doc(hidden)]
pub fn artifact_owned_tool_birth_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, maximum_depth: 1, ..Default::default() }
}

/// 📨️ Funds one zero-heap terminal ownership transfer independently of frame birth: one item, one level and the copy of one sealed shared alias handle.
#[doc(hidden)]
pub fn artifact_owned_tool_handoff_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: ARTIFACT_OWNED_TOOL_HANDOFF_COPY_BYTES, maximum_depth: 1, ..Default::default() }
}

/// 🔗️ Copy bytes one sealed shared alias handle costs when ownership of a captured root moves; a handle is one pointer wide.
const ARTIFACT_OWNED_TOOL_HANDOFF_COPY_BYTES: usize = 64;

/// ⏱️ The whole-run wall budget of one agent-lane preview. A preview answers an agent's `action_prepare` inside one
/// guest call, so a job still running past it is refused by name (`interactive-job.preview-budget`) instead of
/// holding the gateway; its human runs it from the shell, where the job keeps its progress and cancellation.
#[doc(hidden)]
pub const AGENT_LANE_PREVIEW_WALL_US: u64 = 2_000_000;

/// 🔢️ The turn budget of one agent-lane preview — the same refusal on a host whose clock cannot be read, and the
/// bound of every close loop the preview runs.
#[doc(hidden)]
pub const AGENT_LANE_PREVIEW_TURNS: u64 = 65_536;

/// 🧭️ The refusal for a verb whose previewed result publishes `lanes` an agent transaction cannot carry.
#[doc(hidden)]
pub fn agent_lane_uncarried_fault(verb: &str, lanes: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.agent-lane-uncarried"), format!("action '{verb}' publishes {lanes} that an agent transaction cannot carry; it runs only from the shell"))
}

/// 🫥️ The refusal for a verb whose previewed result carries no operation and only presents something to the human who
/// ran it (a notice, a selection): in this state the command changes nothing. Each notice the app would have shown
/// rides the fault as its own structured cause (`app.notice`), never concatenated into the message, so a host renders
/// the app's own words beside its localized text for `app.command.no-effect`.
#[doc(hidden)]
pub fn agent_lane_no_effect_fault(verb: &str, effects: &[Effect]) -> Fault {
    let mut fault = Fault::new(FaultOrigin::App, FaultCode::new("app.command.no-effect"), format!("action '{verb}' changes nothing here"));
    fault.causes = effects.iter().filter_map(|effect| if let Effect::Notify { message } = effect { Some(semio_framework::FaultCause { retained_progress:None, message: message.clone(), code: Some(FaultCode::new("app.notice")) }) } else { None }).collect();
    fault
}

/// ⚖️ What the agent lane answers for one previewed publication ([`agent_lane_carriage`]).
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentLaneCarriage {
    /// ✅️ Everything it publishes rides the transaction; presentation beside carried operations is omitted.
    Carried,
    /// 🫥️ It carries no operation and only presents something to the human who ran it — refused `app.command.no-effect`.
    NoEffect,
    /// 🚧️ It publishes these lanes the transaction cannot carry — refused `interactive-job.agent-lane-uncarried`.
    Uncarried(Vec<&'static str>),
}

/// ⚖️ The agent lane's verdict on one previewed publication: carried when [`agent_lane_uncarried_lanes`] names nothing,
/// no effect when it names only [`AGENT_LANE_PRESENTATION_LANES`] and nothing is carried, uncarried otherwise.
#[doc(hidden)]
pub fn agent_lane_carriage(publication: &AgentLanePublication<'_>) -> AgentLaneCarriage {
    let lanes = agent_lane_uncarried_lanes(publication);
    if lanes.is_empty() {
        AgentLaneCarriage::Carried
    } else if publication.carried_ops == 0 && lanes.iter().all(|lane| AGENT_LANE_PRESENTATION_LANES.contains(lane)) {
        AgentLaneCarriage::NoEffect
    } else {
        AgentLaneCarriage::Uncarried(lanes)
    }
}

/// 📦️ What one previewed command publishes: how many operations an agent transaction carries (its document, config
/// and draft ops and owned-child groups), its host effects and the size of every other lane — the shape
/// [`agent_lane_uncarried_lanes`] judges.
#[doc(hidden)]
pub struct AgentLanePublication<'a> {
    pub carried_ops: usize,
    pub effects: &'a [Effect],
    pub window_config_ops: usize,
    pub extension_calls: usize,
    pub events: usize,
    pub selection_writes: usize,
    pub tasks: usize,
    pub presence: usize,
    pub transient: usize,
    pub window_transient: usize,
}

/// 🪞️ The lanes that only present a command's result to the human who ran it — a notice, a selection of what it made.
/// Beside operations the transaction carries they are omitted (an agent reads its result from the prepared report);
/// on their own they are the command's whole effect, and the agent lane refuses them like any other uncarried lane.
#[doc(hidden)]
pub const AGENT_LANE_PRESENTATION_LANES: [&str; 2] = ["a notice", "selection changes"];

/// 🏷️ Every lane an agent transaction cannot carry, in the order a refusal names them.
#[doc(hidden)]
pub const AGENT_LANE_UNCARRIED_LANES: [&str; 17] = [
    "a whole-document replacement",
    "a file download",
    "a file request",
    "a follow-up dispatch",
    "a dialog",
    "a notice",
    "a clipboard write",
    "a shell change",
    "a host request",
    "extension calls",
    "app events",
    "selection changes",
    "window configuration changes",
    "follow-up tasks",
    "presence changes",
    "transient state changes",
    "window transient state changes",
];

/// 🚧️ The agent lane fails closed: the lanes of one previewed command its transaction cannot carry, each once, in
/// [`AGENT_LANE_UNCARRIED_LANES`] order. The carried set is enumerated, not inferred — only document, config, draft and
/// owned-child ops ride an agent transaction — so every host effect is named by an exhaustive match (a new [`Effect`]
/// variant does not compile until it is classified; a guest-emitted interaction verb, [`is_inline_interaction_verb`], is
/// a selection change whichever effect carries it) and every other non-empty lane is named too, except the
/// [`AGENT_LANE_PRESENTATION_LANES`] beside carried operations. The preview settles only on an empty answer; anything
/// else is refused `interactive-job.agent-lane-uncarried`, so an agent is never told "done" for a verb whose shell run
/// acts where its transaction would do nothing. Language-agnostic fixture: `🧫️fixtures/🤖️agent-lane-carriage.json`.
#[doc(hidden)]
pub fn agent_lane_uncarried_lanes(publication: &AgentLanePublication<'_>) -> Vec<&'static str> {
    let effect_lanes = publication.effects.iter().map(|effect| match effect {
        _ if is_inline_interaction_verb(effect) => "selection changes",
        Effect::LoadDocument { .. } => "a whole-document replacement",
        Effect::DownloadMediaExport { .. } | Effect::IconRenderExport { .. } | Effect::VideoRenderExport { .. } => "a file download",
        Effect::RequestFileOpen { .. } | Effect::RequestMediaFrames { .. } => "a file request",
        Effect::DispatchAction { .. } => "a follow-up dispatch",
        Effect::OpenDialog { .. } => "a dialog",
        Effect::Notify { .. } => "a notice",
        Effect::ClipboardWrite { .. } => "a clipboard write",
        Effect::OpenWindow { .. }
        | Effect::CloseWindow { .. }
        | Effect::SetPanel { .. }
        | Effect::Navigate { .. }
        | Effect::OpenExternalUrl { .. }
        | Effect::SpawnPluginInstance { .. }
        | Effect::OpenPluginInstance { .. }
        | Effect::SetActiveUtility { .. }
        | Effect::SetActiveTool { .. }
        | Effect::ReplayShellCommand { .. } => "a shell change",
        Effect::RequestSync
        | Effect::SendMessage { .. }
        | Effect::PublishEvent { .. }
        | Effect::BlobWrite { .. }
        | Effect::BlobLoad { .. }
        | Effect::HttpRequest { .. }
        | Effect::DocumentRead { .. }
        | Effect::DocumentWrite { .. }
        | Effect::LinkResolve { .. }
        | Effect::RegistryQuery { .. }
        | Effect::IoCompose { .. }
        | Effect::CacheDerive { .. }
        | Effect::CacheRead { .. }
        | Effect::SetTimer { .. }
        | Effect::SpawnJob { .. }
        | Effect::CancelJob { .. }
        | Effect::Respond { .. }
        | Effect::StorageRead { .. }
        | Effect::StorageWrite { .. }
        | Effect::StorageDelete { .. }
        | Effect::RequestCapability { .. }
        | Effect::ReleaseCapability { .. }
        | Effect::Subscribe { .. }
        | Effect::Unsubscribe { .. }
        | Effect::RequestServiceOperation { .. } => "a host request",
        Effect::InvokeExtension { .. } => "extension calls",
    });
    let counted = [
        ("extension calls", publication.extension_calls),
        ("app events", publication.events),
        ("selection changes", publication.selection_writes),
        ("window configuration changes", publication.window_config_ops),
        ("follow-up tasks", publication.tasks),
        ("presence changes", publication.presence),
        ("transient state changes", publication.transient),
        ("window transient state changes", publication.window_transient),
    ]
    .into_iter()
    .filter(|(_, count)| *count != 0)
    .map(|(lane, _)| lane);
    let found = effect_lanes.chain(counted).collect::<BTreeSet<_>>();
    AGENT_LANE_UNCARRIED_LANES.into_iter().filter(|lane| found.contains(lane) && (publication.carried_ops == 0 || !AGENT_LANE_PRESENTATION_LANES.contains(lane))).collect()
}

/// 🏃️ Drives one previewed job on the calling turn through the SAME [`semio_framework_job_worker::BatchJobSession`] step,
/// checkpoint and close protocol a worker runs: every step is bounded by the job's own contract (`params.config`),
/// the run by [`AGENT_LANE_PREVIEW_WALL_US`], [`AGENT_LANE_PREVIEW_TURNS`] and `params.cancel`, and every progress,
/// checkpoint or yield payload is released before the next step. Answers how many outcomes the job reported up to
/// and including its completion. A job fault answers what the shell's fault page carries
/// ([`decode_typed_operation_fault_page`] over the job's bounded detail), except that a retained reducer refusal
/// answers the reducer's own code ([`crate::retained_command::reducer_fault_of_detail`]); a cancellation, an
/// exhausted budget or a lost step is refused by name. The session is closed on every path: to terminal-empty, or
/// handed to the worker-session retirement pump when its close stalls.
#[doc(hidden)]
pub struct ChildEmissionPreviewJob<A:ArtifactApp>{
    pub emit:Option<Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>>,
    pub ephemeral:Option<EphemeralEmit<A>>,
    pub completion:Option<ArtifactToolCompletion<A>>,
    pub retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub retirement_payload:Option<ArtifactToolCompletionValue<A>>,
    pub retirement_fault:Option<Fault>,
    pub rejection:Option<ArtifactToolCompletionRejection<A>>,
    pub completion_retirement:Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub maximum_bytes:usize,
    pub wire_mode:bool,
    pub fault_publication:semio_framework_job_fault_publication::RetainedFaultPublication,
    pub closing:bool,
}

impl<A:ArtifactApp> semio_framework_job::InteractiveJob for ChildEmissionPreviewJob<A>{
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::JobOutcomeBorrow;
        if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}
        if cx.should_yield(){return Ok(None)}
        if let Some(fault)=self.retirement_fault.as_ref().or_else(||self.rejection.as_ref().map(|original|&original.fault)){return self.fault_publication.advance_from_fault(fault,cx).map_err(semio_framework_job::OriginalTurnFailure::from)}
        let Some(emit)=self.emit.as_mut()else{return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(semio_framework_job::OriginalTurnFailure::from)};
        cx.set_stage("agent-child-emission-preparation");
        let grant=cx.retained_grant();
        let demand=emit.child_preparation_demands(grant.maximum_copy_bytes)?;
        if demand.capacity_bytes>self.maximum_bytes{
            if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            self.retirement_fault=Some(plugin_sdk_fault("child emission exceeds its captured preview output allocation authority"));
            return Ok(None);
        }
        let step=if self.wire_mode{emit.prepare_child_preview_one(grant)}else{emit.prepare_child_one(grant)};
        let progress=match &step{Ok(step)=>step.progress(),Err(fault)=>fault.retained_progress()};
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        match step{
            Ok(ChildEmitPreparationStep::Pending(_))=>Ok(None),
            Ok(ChildEmitPreparationStep::Ready(progress))=>{
                if progress!=RetainedCloneProgress::default(){return Ok(None)}
                if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}
                let completion=self.completion.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original child preview completion owner is absent"))?;
                if self.ephemeral.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original child preview ephemeral owner is absent").into())}
                cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
                let ephemeral=self.ephemeral.take().expect("original ephemeral owner was borrowed before transfer");
                let emit=self.emit.take().expect("ready preview retains its original complete emission");
                if let Err(original)=completion.complete(Ok(emit),ephemeral){self.rejection=Some(original)}
                Ok(None)
            }
            Ok(ChildEmitPreparationStep::Refused(fault,_))|Err(fault)=>{self.retirement_fault=Some(fault);Ok(None)}
        }
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{
        match descriptor.kind(){
            semio_framework_job::JobOutcomeKind::Yield=>descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.emit.is_none()&&self.rejection.is_none()&&self.retirement_fault.is_none()=>descriptor.complete(None,None),
            semio_framework_job::JobOutcomeKind::Fault=>self.fault_publication.borrow_outcome(descriptor),
            _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"child preview descriptor requires its original semantic result")),
        }
    }
    fn begin_close(&mut self){self.closing=true;}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if !self.closing||cx.should_yield(){return Ok(Step::Blocked);}
        let demand=self.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_owned_turn(grant).map(RetainedCloneStep::Progress))?;
        Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_owned_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.fault_publication.terminal_is_empty()&&self.emit.is_none()&&self.ephemeral.is_none()&&self.completion.is_none()&&self.retirement.is_none()&&self.retirement_payload.is_none()&&self.retirement_fault.is_none()&&self.rejection.is_none()&&self.completion_retirement.is_none()}
}

impl<A:ArtifactApp> ChildEmissionPreviewJob<A>{
    pub fn close_owned_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if let Some(ticket)=self.retirement.as_ref(){return semio_framework_value::factory_ticket_demands(ticket,copy);}
        if let Some(original)=self.retirement_payload.as_ref(){return Ok(RetirementDemand{capacity_bytes:A::completion_retirement_birth_bytes(original).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"preview completion has no original app retirement issuer"))?,depth:1,..Default::default()});}
        if self.retirement_fault.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<Fault>(),depth:1,..Default::default()});}
        if self.rejection.is_some(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
        if self.emit.is_some()||self.ephemeral.is_some(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
        if let Some(owner)=self.completion_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        Ok(RetirementDemand{depth:usize::from(self.completion.is_some()),..Default::default()})
    }
    pub fn close_owned_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let demand=self.close_owned_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.close_step(grant).map(|step|step.progress());}
        if self.retirement.is_some(){return Ok(semio_framework_value::close_factory_ticket(&mut self.retirement,grant)?.progress());}
        if self.retirement_payload.is_some(){return match A::admit_completion_retirement(&mut self.retirement_payload,grant)?{Some((ticket,progress))=>{self.retirement=Some(ticket);Ok(progress)},None=>Ok(Default::default())};}
        if let Some(original)=self.retirement_fault.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((ticket,progress))=>{self.retirement=Some(ticket);Ok(progress)},Err((error,original))=>{self.retirement_fault=Some(original);Err(error)}};}
        if let Some(original)=self.rejection.take(){self.retirement_payload=Some(ArtifactToolCompletionValue::Emit(original.emit.map_err(ArtifactCompletionFault::new),original.ephemeral));self.retirement_fault=Some(original.fault);return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});}
        if self.emit.is_some()||self.ephemeral.is_some(){self.retirement_payload=Some(ArtifactToolCompletionValue::Emit(Ok(self.emit.take().unwrap_or_default()),self.ephemeral.take().unwrap_or_default()));return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});}
        if let Some(owner)=self.completion_retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.completion_retirement.take();}return Ok(step.progress());}
        if let Some(original)=self.completion.take(){return match ControlledRetirement::new(original){Ok(owner)=>{self.completion_retirement=Some(owner);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})},Err((error,original))=>{self.completion=Some(original);Err(error)}};}
        Ok(Default::default())
    }
}

#[doc(hidden)]
pub struct AgentLanePreview<A:ArtifactApp>{
    pub key:Option<semio_framework_plugin_operation::ToolOperationKey>,
    pub caller:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub starting:bool,
    pub producer:Option<OriginalPreviewDrive<semio_framework::ErasedToolJob>>,
    pub children:Option<OriginalPreviewDrive<ChildEmissionPreviewJob<A>>>,
    pub completion:Option<ArtifactToolCompletion<A>>,
    pub completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub lease:Option<semio_framework_plugin_operation::ToolCancellationLease>,
    pub lease_close:Option<Box<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>>,
    pub config:semio_framework_job_worker_authority::BatchDriveConfig,
    pub maximum_bytes:usize,
    pub steps:u64,
    pub publication:Option<ArtifactToolCompletionValue<A>>,
    pub complete:bool,
    pub closing:bool,
    pub publication_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletionValue<A>>>,
}
impl<A:ArtifactApp> AgentLanePreview<A>{
    pub fn retirement_demands(&self,body:usize,canonical:Option<&semio_framework_plugin_operation::ToolCancellationHandle>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.producer.as_ref(){return owner.retirement_demands(body).map(|demand|if demand==Default::default(){semio_framework_value::RetirementDemand{depth:1,..Default::default()}}else{demand})}
        if let Some(owner)=self.children.as_ref(){return owner.retirement_demands(body).map(|demand|if demand==Default::default(){semio_framework_value::RetirementDemand{depth:1,..Default::default()}}else{demand})}
        if self.completion.is_some()||self.completion_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.completion,&self.completion_close,body)}
        if self.publication.is_some()||self.publication_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.publication,&self.publication_close,body)}
        if let Some(owner)=self.lease_close.as_ref(){return if owner.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{release_bytes:std::mem::size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{owner.retirement_demands(canonical.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"admitted preview lease lost its original cancellation handle"))?,body)}}
        if self.lease.is_some(){return Ok(semio_framework_value::RetirementDemand{capacity_bytes:std::mem::size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),copy_bytes:std::mem::size_of::<semio_framework_plugin_operation::ToolCancellationLease>(),depth:1,..Default::default()})}
        Ok(self.caller.as_ref().map_or(semio_framework_value::RetirementDemand{depth:1,..Default::default()},|caller|caller.retirement_demands()))
    }
    pub fn terminal_is_empty(&self)->bool{self.key.is_none()&&self.caller.is_none()&&self.producer.is_none()&&self.children.is_none()&&self.completion.is_none()&&self.completion_close.is_none()&&self.publication.is_none()&&self.publication_close.is_none()&&self.lease.is_none()&&self.lease_close.is_none()}
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,canonical:Option<&semio_framework_plugin_operation::ToolCancellationHandle>)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Close;
        self.closing=true;if cx.should_yield(){return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default()))}
        if let Some(owner)=self.producer.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.producer.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress),Close::Blocked=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"original preview producer awaits paid closure"}})}
        if let Some(owner)=self.children.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.children.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress),Close::Blocked=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"original preview child producer awaits paid closure"}})}
        let grant=cx.retained_grant();
        let step=if self.completion.is_some()||self.completion_close.is_some(){semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.completion,&mut self.completion_close,grant)?}
        else if self.publication.is_some()||self.publication_close.is_some(){semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.publication,&mut self.publication_close,grant)?}
        else if let Some(owner)=self.lease_close.as_mut(){if owner.terminal_is_empty(){semio_framework_plugin_operation::original_lease_cursor_release(&mut self.lease_close,grant).map(RetainedCloneStep::Progress)?}else{owner.close_step(canonical.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"admitted preview lease lost its original cancellation handle"))?,grant)?}}
        else if self.lease.is_some(){let(owner,progress)=semio_framework_plugin_operation::original_lease_cursor_admit(&mut self.lease,grant)?;self.lease_close=owner;RetainedCloneStep::Progress(progress)}else if self.key.is_some(){if grant.maximum_items==0||grant.maximum_depth==0{return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default()))}self.key.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else if let Some(caller)=self.caller.as_mut(){let step=caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){self.caller.take();}return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()))}else{return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()))};
        cx.consume_retained(step.progress())?;Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()))
    }
}
#[doc(hidden)]
pub struct OriginalPreviewDrive<J:semio_framework_job::InteractiveJob+'static>{
    pub pending:Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<J>>,
    pub session:Option<semio_framework_job_worker::BatchJobSession<J>>,
    pub config:semio_framework_job_worker_authority::BatchDriveConfig,
    pub source:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub steps:u64,
    acknowledgement_pending:bool,
    pub resume_pending:bool,
    pub closing:bool,
    pub verdict:Option<Result<u64,semio_framework_job::OriginalTurnFailure>>,
    verdict_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Result<u64,semio_framework_job::OriginalTurnFailure>>>,
}
impl<J:semio_framework_job::InteractiveJob+'static> OriginalPreviewDrive<J>{
    pub fn new(job:J,config:semio_framework_job_worker_authority::BatchDriveConfig,cx:&semio_framework_job::StepContext<'_>)->Self{Self{pending:Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job,(cx.operation(),cx.generation()))),session:None,config,source:(cx.operation(),cx.generation()),steps:0,acknowledgement_pending:false,resume_pending:false,closing:false,verdict:None,verdict_close:None}}
    pub fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.pending.as_ref(){return owner.retirement_demands(body)}
        if let Some(owner)=self.session.as_ref(){return owner.retirement_demands(body).map_err(|source|match source{semio_framework_job_worker::WorkerJobDemandError::Refused(source)=>source,semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original preview producer remains checked out")})}
        semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.verdict,&self.verdict_close,body)
    }
    pub fn begin_close(&mut self){self.closing=true;if let Some(session)=self.session.as_mut(){session.begin_close()}}
    pub fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.session.is_none()}
    pub(crate) fn cancellation_terminal_is_empty(&self)->bool{self.terminal_is_empty()&&self.verdict.is_none()&&self.verdict_close.is_none()}
    pub(crate) fn cancel_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if !self.terminal_is_empty(){return self.close_step(cx)}
        let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.verdict,&mut self.verdict_close,cx.retained_grant())?;cx.consume_retained(step.progress())?;
        Ok(if self.cancellation_terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()}})
    }
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Close};
use semio_framework_job_worker_authority::{WorkerJobCloseStep as Worker};
        if self.source!=(cx.operation(),cx.generation()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview close requires its original initiating source").into())}
        if cx.should_yield(){return Ok(Close::Pending{progress:Default::default()})}
        self.begin_close();
        if let Some(owner)=self.pending.as_mut(){let step=owner.close_step(cx)?;if owner.terminal_is_empty(){self.pending.take();}return Ok(step)}
        if let Some(owner)=self.session.as_mut(){let step=owner.close_step(cx)?;if owner.terminal_is_empty(){self.session.take();}return Ok(match step{Worker::Pending{progress}=>Close::Pending{progress},Worker::Complete{progress}=>Close::Complete{progress},Worker::Blocked=>Close::Blocked})}
        Ok(Close::Complete{progress:Default::default()})
    }
}
#[doc(hidden)]
pub fn drive_agent_lane_preview<J:semio_framework_job::InteractiveJob+'static>(owner:&mut OriginalPreviewDrive<J>,verb:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<u64>,semio_framework_job::OriginalTurnFailure>{
    use semio_framework_job_worker::{WorkerJobPoll as Poll};
    if owner.source!=(cx.operation(),cx.generation()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview continuation requires its initiating source").into())}
    if cx.should_yield(){return Ok(None)}
    if owner.closing{
        owner.close_step(cx)?;
        if !owner.terminal_is_empty()||cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return owner.verdict.take().unwrap_or(Ok(owner.steps)).map(Some)
    }
    if let Some(pending)=owner.pending.as_mut(){if let Some(session)=pending.admit_pending_batch(owner.config,cx)?{owner.session=Some(session);owner.pending.take();}return Ok(None)}
    let session=owner.session.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview continuation lost its admitted producer"))?;
    if owner.acknowledgement_pending{let step=session.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){owner.acknowledgement_pending=false;owner.resume_pending=true;}return Ok(None)}
    if owner.resume_pending{session.resume().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview continuation could not return its original outcome"))?;owner.resume_pending=false;return Ok(None)}
    match session.step_on_original_caller(cx){Ok(Poll::Outcome|Poll::Terminal)=>{},Ok(_)=>return Ok(None),Err(failure)=>{owner.begin_close();return Err(failure)}}
    if !session.checkout_outcome(){return Ok(None)}
    let Some((issued,receipt))=session.take_checked_out_retained_step_receipt()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original preview callback receipt is absent").into())};
    if !receipt.fits(issued){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original preview callback exceeded its issued receipt").into())}
    owner.steps=owner.steps.saturating_add(1);
    let verdict=match session.checked_out_outcome()?{
        Some(semio_framework_job::JobOutcomeView::Complete{..})=>Some(Ok(owner.steps)),
        Some(semio_framework_job::JobOutcomeView::Cancelled{..})=>Some(Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),format!("the preview of action '{verb}' was cancelled")).into())),
        Some(semio_framework_job::JobOutcomeView::Fault{detail,..})=>{let original=ArtifactBoundedToolFault::from_payload(detail);Some(Err(semio_framework_plugin_decl_app::tool_contract::reducer_fault_of_detail(original.as_bytes()).unwrap_or_else(||decode_typed_operation_fault_page(original.as_bytes())).into()))},
        Some(_)=>None,
        None=>return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original preview outcome is absent").into()),
    };
    if let Some(verdict)=verdict{owner.verdict=Some(verdict);owner.begin_close()}else{owner.acknowledgement_pending=true}
    Ok(None)
}

#[path = "🧾️bytes/🦀️.rs"]
pub mod reserved_job_bytes;
#[path = "📤️outcomes/🦀️.rs"]
pub mod reserved_job_outcomes;
#[path = "🏛️reserved/🦀️.rs"]
pub mod reserved_jobs;
#[path = "🛬️lifted/⏯️tool-run/🦀️.rs"]
pub mod lifted_job;
#[path = "🛬️lifted/🔄️full-operation/🦀️.rs"]
pub mod lifted_full_operation_job;
