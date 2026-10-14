//! 🔌️ The runtime app contract every hosted app instance satisfies, closed per plugin by `dyn_enum_close!`.
use semio_framework::kernel::PresenceUpdate;
use semio_framework_ui_contract::UiIntent;
use semio_framework_dispatch_macros::dyn_enum;
use crate::MountedOwnerPhaseV1;
use crate::MountedOwnerTurnV1;
use crate::window_config::WindowConfigPack;
use semio_framework_value::DslValue;
use semio_framework::kernel::FixedCommandPage;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::manifest::ActionInvocation as ManifestActionInvocation;
use semio_framework::manifest::CommandInvocation as ManifestCommandInvocation;
use semio_framework::kernel::AppEvent;
use semio_framework::kernel::Effect;
use semio_framework::kernel::HistoryPatch;
use semio_framework::kernel::InvocationResult;
use semio_framework::Fault;
use semio_framework::ToolRunTraceCursor;
use semio_framework::ViewModel;
use semio_framework_dispatch_macros::dyn_enum_close;
use semio_framework_ui_runtime::ComponentTree;
use std::collections::HashMap;
use ui_wgpu::wgpu::ContextMenuItemSpec;
use ui_wgpu::wgpu::ContextMenuRequest;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowMeasure;
use semio_framework::MediaWireFormat;
use semio_framework::Media;
use semio_framework::MediaError;
use semio_framework::MediaFingerprint;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use crate::app_emission::ActionMeta;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::document_ingress::ArtifactStoreConstructorKind;
use crate::document_ingress::ArtifactStoreConstructorPhase;
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportPoll;
use crate::media_contract::ArtifactMediaExportResult;
use crate::peer_roster::PresenceRosterAdmission;
use crate::peer_roster::PresenceRosterOutcome;
use crate::reserved_jobs::FrameworkReservedCommitProgress;
use crate::tool_contract::ToolOwnerWitness;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_publication::ComposedGestureResult;
use crate::tool_contract::ArtifactToolPublicContract;
use crate::tool_jobs::EmitWire;
use crate::tool_jobs::TransactionProposalDraft;
use crate::tool_jobs::TypedOperationCompletion;
use crate::tool_jobs::TypedOperationResultPage;
use crate::tool_jobs::TypedOperationResultToken;
use crate::tool_jobs::TypedOperationUiProgress;
use crate::app_contract::DOCUMENT_LOAD_UNAVAILABLE_CODE;
use crate::app_contract::EphemeralSnapshot;
use crate::app_contract::MediaArtifact;
use crate::app_contract::MediaArtifactError;
use crate::app_contract::MediaConsumption;
use crate::app_contract::TransactionPrepareOutcome;
use crate::app_contract::document_load_fault;
use crate::app_contract::whole_document_archive;
use crate::app_contract::whole_document_media;

/// 🗄️ Object-safe runtime contract every hosted app satisfies. Owns persistent document state
/// (via {@link VcsArtifactApp}'s store) across calls — no per-call document JSON is threaded in.
/// History actions (undo/redo/checkpoint/alternative) are intercepted by the wrapper; typed
/// operations are dispatched with real inverses; operations flow to/from the backbone as the wire format.
/// 🗃️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-dedyn, O1): `#[dyn_enum]` captures every
/// method signature into a hidden `__semio_dispatch_PluginApp!` macro. Each fleet plugin closes
/// its OWN per-plugin app enum with `dyn_enum_close! { pub enum <Name>Apps: PluginApp { .. } }`
/// (cross-crate — that call site needs `use semio_framework_plugin::__semio_dispatch_PluginApp;`
/// immediately above it, per 📓️terra-dyn-enum-macro-report.md finding 1/recipe §5). `PluginApp:
/// Send` is a TYPE bound (R3) — it constrains the state an app enum's variants hold, not the
/// futures its async methods return, which stay `?Send` per the guest ruling.
#[dyn_enum]
pub trait PluginApp: Send + 'static {
    /// 🏇️ Advances the real live mounted owner under its original scheduling and physical turn.
    fn mounted_owner_maintenance_phase(&self) -> Result<MountedOwnerPhaseV1, Fault>;
    fn mounted_owner_maintenance_step(&mut self, turn: &mut MountedOwnerTurnV1<'_, '_>) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure>;
    /// 🏁️ Closes the real mounted owner under the caller's independently declared close turn.
    fn mounted_owner_close_step(&mut self, turn: &mut MountedOwnerTurnV1<'_, '_>) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure>;
    /// 📏️ Exact physical allocation needed by the currently retained close authority.
    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    /// 🌱️ Exact physical allocation needed by the selected live maintenance authority.
    fn maintenance_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    /// 🪪️ Binds the host's live instance identity before any operation-scoped work starts.
    async fn bind_instance_id(&mut self, _instance_id: u32) {}
    /// 🧍️ Binds the actor admitted for this instance at its open: who it acts as from then on — across a reload too —
    /// on every route that carries no actor of its own.
    async fn bind_actor(&mut self, _actor: &str) {}
    /// 🧹 Releases one synchronous, bounded close unit. Implementations must never suspend and
    /// may return `Complete` only after their final destructor is constant-time.
    fn close_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure>;
    /// 🧹 Proves that every deep owner has reached its terminal empty shell. The runtime checks
    /// this after `close_step` returns `Complete` and before releasing the erased app allocation.
    fn close_terminal_is_empty(&self) -> bool;
    /// 🧹 Releases one bounded nonterminal quarantine unit while the app remains live. This is
    /// scheduled on the shared maintenance worker and must never perform application-visible work.
    fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure>;
    /// 🪹️ Every live retirement frontier and its retained directory backing is empty.
    fn maintenance_terminal_is_empty(&self) -> bool;
    /// 🌡️ True while a displaced-owner queue of the document or a config-lane store sits at or
    /// above [`store::os_store::snapshot_read::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`], or a child-root or child-member
    /// retirement registry at or above [`MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`]. The runtime
    /// then spends a burst of maintenance steps on this instance in the same turn instead of one — the
    /// single fair step per turn drains one owner per 26-stage rotation while a live commit cadence
    /// displaces three or more owners (and, for a content-addressed child, one child root and member) per commit.
    fn maintenance_under_pressure(&self) -> bool;
    async fn app_id(&self) -> &str;
    async fn artifact_schema(&self) -> &str;
    /// 🧬️ Compiler-derived concrete `ArtifactApp` identity used by fail-closed tool proof admission.
    async fn tool_owner_witness(&self) -> ToolOwnerWitness;
    async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract>;
    /// 📬️ Advances at most one typed-operation store or host publication unit.
    async fn advance_typed_operation_publication(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure>;
    /// 🧾️ Returns one exact preparation receipt without deriving work from its checkpoint.
    fn take_typed_operation_preparation_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>;
    /// ♻️ Transfers one original mounted Close receipt before another ownership producer.
    fn take_typed_operation_retirement_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>;
    /// 🧾️ Transfers the exact original history command grant and receipt without assigning a job identity.
    fn take_history_command_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)>;
    /// 🛠️ Transfers the original tool state-machine turn receipt without assigning a history identity.
    fn take_tool_run_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)>;
    /// 📃️ Transfers the exact original query turn receipt before any subsequent semantic work.
    fn take_local_interaction_query_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>;
    /// 🏗️ Keeps semantic commands behind the original staged catalog constructors.
    fn store_constructor_is_pending(&self) -> bool;
    /// 🧱️ Transfers one original constructor kind and full receipt without inventing a job.
    fn take_constructor_receipt(&mut self) -> Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>;
    /// 🎟️ Retains the actual app creation parent until its canonical cancellation issuer is paid and ready.
    fn admit_cancellation_issuer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>;
    /// 👥️ This instance's tool run summary for the local `PresencePeer.tool_run` (contract §3.4).
    fn tool_run_presence(&self) -> Option<protocol::PresenceToolRun> {
        None
    }
    /// 📼️ Base64url `ToolRunTraceDelta` after the renderer-echoed cursor, for the scene lane
    /// `toolRunTrace` (contract §3.2, §4.1 layer 2); `None` when idle.
    fn tool_run_trace_delta(&self, _cursor: Option<ToolRunTraceCursor>) -> Option<String> {
        None
    }
    /// 🔁️ Keeps admitted work live through worker preparation, publication, ACK, and retirement.
    fn has_pending_typed_operations(&self) -> bool;
    /// 🚨️ The first structural refusal a live typed operation retains before its retirement can surface it: an owner that can never be retired (`UnsupportedOwner`) keeps its operation pending for ever, so a fixture reads the retained reason here instead of waiting for a deadline.
    fn typed_operation_refusal(&self) -> Option<String> {
        None
    }
    /// 🩺️ One line per live typed operation naming its verb, stage and retained refusal, for deadline reports.
    fn typed_operation_diagnostic(&self) -> String {
        String::new()
    }
    /// 🩺️ The close-ladder frontier a stalled close is parked on, for deadline reports.
    fn close_diagnostic(&self) -> String {
        String::new()
    }
    /// ▶️ Whether this app still owes the host a typed-operation round trip — the sole source the
    /// reactor turn folds into `TurnStatus::MoreWork` for typed operations, and so the only thing a
    /// host call that reads nothing but the status (the eventless drain poll, a `refresh-ui` settle)
    /// can decide on. A mounted operation counts at EVERY stage, an unacknowledged presented page
    /// included: the ACK that frees it is an event the host submits with its next continuation, which
    /// it only makes while this answers true (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16).
    fn has_runnable_typed_operations(&self) -> bool;
    /// 🎰️ Live occupancy of the fixed typed-operation and segmented-output slot array, as the
    /// admitting authority itself counts it. Diagnostics only — the host reads it to prove every
    /// admitted slot reached its release path, never to decide anything.
    fn live_typed_operation_slots(&self) -> usize {
        0
    }
    /// 📖️ Captures exact current local roots into the app's single ACK-owned query slot.
    fn begin_local_interaction_query(&mut self, request_id: u64, _query_generation: u64) -> Option<protocol::LocalInteractionQueryReply> {
        Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed })
    }
    /// 📨️ ACK and cancellation validate the full runtime-generation and frozen-root token.
    fn acknowledge_local_interaction_query(&mut self, _token: &protocol::LocalInteractionQueryToken) -> bool {
        false
    }
    fn cancel_local_interaction_query(&mut self, _token: &protocol::LocalInteractionQueryToken) -> bool {
        false
    }
    /// 📤️ Drains at most one bounded page, start token, or terminal outcome.
    fn take_local_interaction_query_reply(&mut self) -> Option<protocol::LocalInteractionQueryReply> {
        None
    }
    /// 📬️ Transfers a reply only after fixed frame bytes and an output slot are admitted.
    fn publish_local_interaction_query_reply(&mut self, _frames: &mut Vec<Vec<u8>>, _maximum_frames: usize) -> bool {
        false
    }
    /// 📤️ Borrows one fixed result page while its operation retains ACK authority.
    fn take_typed_operation_result_page(&mut self, receiver: u32) -> Option<TypedOperationResultPage>;
    /// 📨️ ACKs one exact receiver/operation/generation/sequence/attempt token.
    fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault>;
    /// 📤️ Drains one effect already accepted by the typed-operation host receiver.
    fn take_typed_operation_effect(&mut self) -> Option<Effect>;
    /// 📤️ Drains one event already accepted by the typed-operation host receiver.
    fn take_typed_operation_event(&mut self) -> Option<AppEvent>;
    /// 📤️ Drains one UI scope already accepted by the typed-operation host receiver.
    fn take_typed_operation_ui_scope(&mut self) -> Option<UiDirtyScope>;
    /// 📤️ Drains one UI progress unit — the scope plus the leftover `InteractionView` a folded
    /// interaction verb owes the host (`TypedOperationUiProgress`). The default answers the scope
    /// alone, for a `PluginApp` that folds nothing.
    fn take_typed_operation_ui_progress(&mut self) -> Option<TypedOperationUiProgress> {
        self.take_typed_operation_ui_scope().map(|ui_scope| TypedOperationUiProgress { ui_scope, leftover: None, history_patch: None })
    }
    /// 🏁️ Drains one terminal typed-operation witness, resolving its command-log delta and
    /// content revision against CURRENT state — the payload of one `AppFrame::OperationCompleted`.
    async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault>;
    /// 🧩️ Takes one composed gesture's published result — see [`ComposedGestureResult`].
    fn take_typed_operation_composed_result(&mut self) -> Option<ComposedGestureResult>;
    /// 📨️ Starts one command-scoped report at the app's live merge policy.
    async fn begin_dispatch_report(&mut self);
    /// 📨️ Returns the current command's authoritative merge-policy report.
    async fn dispatch_report(&self) -> &protocol::DispatchReport;
    /// ⚖️ Sets this instance's local merge policy without creating shared document history.
    async fn set_merge_policy(&mut self, policy: protocol::MergePolicy);
    /// ⚔️ Resolves one current open conflict through the authoritative document store.
    async fn resolve_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault>;
    /// ⚔️ Returns the authoritative projection of currently open document conflicts.
    async fn open_conflicts(&self) -> Vec<protocol::Conflict>;
    /// 🕰️ FRAMEWORK-reserved action dispatch only (undo/redo/checkpoint/alternative/clipboard/
    /// revert-to-command/history-filter/noteShellCommand) — B1 deleted the generic app-declared-action
    /// fallback this used to carry (`ArtifactApp::handle_action` no longer exists; an app's own
    /// behavior is reached exclusively through `handle_command_frame`'s typed `Self::Command` decode).
    /// An unrecognized `action` id is a hard error pointing at the typed channel.
    async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;
    /// 🕰️ Hands one host-driven framework-reserved spawn-job to the app's reserved-commit queue,
    /// whose units the typed-operation continuation then drives across turns. Answers `false` for a
    /// job the app never admitted. Default is a no-op so only `VcsArtifactApp` owns the queue.
    fn admit_reserved_spawned_job(&mut self, _job: u64, _output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        Ok(false)
    }
    /// 🧾️ Takes the result of the last finished reserved commit — the payload of one unsolicited
    /// `AppFrame::Invocation` (or `Error`), framed before that operation's `OperationCompleted`.
    fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        None
    }
    /// 📊️ Progress of the reserved commit this app is driving, if any.
    fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        None
    }
    /// 📍️ Validates and dispatches a window-instance-owned action invocation.
    async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;
    /// 🎯️ Dispatches the manifest protocol's structurally addressed app- or mode-owned
    /// command. This is deliberately `ManifestCommandInvocation`, not the kernel execution envelope
    /// re-exported under the same unqualified name; the active mode is mandatory for mode ownership.
    /// 📨️ Borrows the original continuation arguments and whole fault through deferred app-command admission.
    async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure>;
    async fn handle_command(&mut self, address: &semio_framework::manifest::CommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;
    /// 🎯️ The single dispatch point `plugin_runtime::plugin_exchange` calls for every
    /// `AppCommand::Command` frame: recognizes a framework-reserved `{kind,name,args}` wire-value
    /// action envelope (routed through `handle_action` above) and otherwise decodes
    /// `command_bytes` directly as the app's typed `ArtifactApp::Command` via `OpBinary::decode_op`
    /// and calls the pure `ArtifactApp::handle`.
    fn original_command_frame_pending(&self)->bool{false}
    async fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;
    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): the single dispatch point
    /// `plugin_runtime::plugin_dispatch_intents` calls for every `UiIntent` that survived the
    /// reactor's revision guard — resolves this app's typed `Command` via
    /// `ArtifactApp::command_from_intent` and dispatches it through the SAME internal path
    /// `handle_command_frame` uses (view/shell kind discipline, the command log, undo grouping,
    /// `AsyncTask` spawning all apply automatically; no parallel dispatch path to keep in sync).
    async fn handle_intent_frame(&mut self, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumes an `AsyncTask`
    /// whose future resolved with `TaskResolution::Emit` — `artifact_ops`/`config_ops`/
    /// `draft_ops` are the SAME `protocol::encode_ops_vec`/`OpBinary::encode_op`-wire triple
    /// `take_last_emit_wire` drains, decoded back into this app's typed `Mutation`/
    /// `ConfigMutation`/`DraftMutation` and applied via `dispatch_emit` DIRECTLY — no re-entry
    /// into `ArtifactApp::handle` (see `TaskResolution::Emit`'s own doc for why this is
    /// deliberately narrower than a `TaskResolution::Command` resume, which goes through
    /// `handle_command_frame` above instead).
    async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault>;
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumes an `AsyncTask`
    /// whose future resolved with `TaskResolution::Command` — `command_bytes` are that
    /// resolution's own `OpBinary::encode_op` of this app's typed `Command`, re-entered through
    /// the exact APPLYING lane a fresh command uses (`dispatch_typed`), stamped with the task's
    /// originating `ActionMeta`.
    ///
    /// 🚪️ Deliberately NOT `handle_command_frame`: that is the agent PREPARE lane
    /// (`preview_addressed_action` — it runs the reducer and hands the ops back for a separate
    /// `TransactionPrepare`/`TransactionCommit`, and it decodes an owner-qualified
    /// `ManifestActionInvocation`, which a task resolution is not). A task follow-up is
    /// Msg-from-Cmd: it APPLIES, exactly once, against CURRENT state.
    async fn resume_task_command(&mut self, command_bytes: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault>;
    /// 🧾 Drains the last Emit op packs captured during `handle_command_frame` (PureCommand path).
    async fn take_last_emit_wire(&mut self) -> Option<EmitWire>;
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): drains the render-plane presence outbox
    /// `stamp_and_cache_interaction_ui` fills every render (`VcsArtifactApp::pending_presence`) —
    /// `plugin_runtime::plugin_render_surface`'s object-safe entry point (mirrors
    /// `take_last_emit_wire`'s own shape: an owned drain, never a borrow, since `PluginApp` is the
    /// dyn-enum-closed boundary and `pending_presence` is a private field on the concrete app).
    async fn take_pending_presence(&mut self) -> Vec<PresenceUpdate>;
    /// 🔀️ Drains the pending `TransactionProposalDraft` `dispatch_emit` stashed instead of
    /// applying, if any (contract §5.1) — consumed once by `plugin_exchange` to frame
    /// `AppFrame::TransactionProposal` instead of the ordinary `Invocation`/`Done` frame.
    async fn take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft>;
    /// 🔀️ Handles a `TransactionPrepare` wire command (contract §5.3/§5.9), EITHER form (flat
    /// fields — `prepared_ops` empty selects the owner-mutation form, decoding `payload` as one
    /// `OpBinary`-encoded op of this app's own `Mutation` type; non-empty selects the
    /// pre-planned form, decoding each `prepared_ops` entry the same way plus the wire-encoded
    /// `origin`). Never fails structurally — a rejection is reported through the returned
    /// `TransactionPrepareOutcome::rejection`, not `Err`, so a caller can always frame a
    /// `TransactionPrepared` reply.
    /// `prepared_child_ops` is the pre-planned form's owned-child share ([`ChildEmit::encode_groups`]).
    async fn transaction_prepare(&mut self, txn_id: &str, mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome;
    /// 🔀️ Handles a `TransactionCommit` wire command (contract §5.6/§5.8): applies this
    /// member's prepared ops as exactly ONE `Edit` (`group_id = txn_id`, every
    /// `MutationMeta.origin` stamped to the prepared origin), clearing the pending transaction,
    /// and returns the committed edit id. `Err` carries a frozen rejection code
    /// (`transaction.generation-mismatch` if the live generation moved since
    /// `transaction_prepare`) or a plumbing fault for an unknown/mismatched `txn_id` — the
    /// caller never applies anything on `Err`. `meta` supplies the local actor id the fresh
    /// `Edit` is attributed to, matching every other `dispatch_emit`-driven apply.
    async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault>;
    /// 🔀️ Handles a `TransactionRollback` wire command (contract §5.5/§5.6): discards the
    /// pending transaction without applying anything — `transaction_prepare` never applies, so
    /// this is pure bookkeeping.
    async fn transaction_rollback(&mut self, txn_id: &str) -> Result<(), Fault>;
    /// 🔀️ Handles a `TransactionUndo{group_id}` fan-out (contract §5.7): undoes this member's
    /// tail edit if (and only if) it carries `MutationMeta.group_id == group_id`, mirroring
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`'s own tail-based membership test for a
    /// single member reached over the wire instead of in-process.
    async fn transaction_undo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault>;
    /// 🔀️ `transaction_undo`'s redo-direction twin, testing the top of the redo stack instead
    /// of the applied tail.
    async fn transaction_redo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault>;
    /// 🫥️ Hydrates a pure command's head-only document (design §20.8): `head` is the HEAD snapshot pack without history;
    /// empty evaluates on the live document.
    async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>;
    /// 👥️ Packs the typed local presence snapshot, exposes both ephemeral generations, and
    /// assembles the zero-app-code declared-broadcast interaction slice — see
    /// {@link EphemeralSnapshot}'s own doc comment.
    async fn ephemeral_snapshot(&self) -> EphemeralSnapshot;
    /// 👥️ Reserves the exact persistent roster and outcome slots before any Presence payload decode.
    fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault>;
    /// 👥️ Transfers the exact encoded command owner into its retained decode/publication job.
    fn admit_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64);
    /// 👥️ Transfers one subsequent exact peer page into the reserved publication.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    fn push_presence_ingress(&mut self, generation: u64, page_index: u32, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)>;
    /// 👥️ Transfers malformed encoded pages into the same bounded FIFO cleanup/outcome lane.
    fn reject_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: FixedCommandPage, fault: Fault);
    /// 👥️ Drains one ordered asynchronous roster outcome.
    fn take_presence_outcome(&mut self) -> Option<PresenceRosterOutcome>;
    /// 🧾️ Returns a complete ordered history projection for initial connection or cursor resync.
    async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
        Ok(HistoryPatch::default())
    }
    /// 🧮️ Object-safe counterpart to `ArtifactApp::Config`'s binary-pack encoding — the config-store
    /// twin of `document_pack` below.
    async fn config_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault>;
    /// 🧮️ Object-safe load of the config store from its binary pack.
    async fn load_config_pack(&mut self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<(), Fault>;
    /// 🪟️ Returns every exact persisted-local window config envelope.
    async fn window_config_packs(&self) -> Result<Vec<WindowConfigPack>, Fault>;
    /// 🪟️ Reopens one exact persisted-local window config envelope under its registered owner.
    async fn load_window_config_pack(&mut self, pack: WindowConfigPack) -> Result<(), Fault>;
    /// 🧸️ Adopts one owned child's persisted envelope into a live child store — the
    /// `AppCommand::LoadChildren` handler. A composing document restores its children through
    /// this, one call per child, after its own document archive load.
    async fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault>;
    /// 🧸️ Every live owned child's current envelope, for persistence — the `ReadChildren`
    /// handler and the child-side twin of `document_pack`.
    async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault>;
    /// 🪆️ Every live owned child's CURRENT content as its head snapshot pack, nested members included, sorted by slot and child
    /// id — what a reader that composes on read takes beside the parent (design §20.15), e.g. the `child:<slot>/<childId>`
    /// dependencies of an inference request on this document.
    async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, Fault>;
    /// 🎟️ Transfers one recursive archive owner into a fixed operation slot before decode.
    fn begin_document_archive_load(&mut self, _operation: u64, _archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive loading is unavailable for this app"))
    }
    /// 🔀️ Admits a read-back of the document this program already shows as a MERGE (design §22.22): its events join the
    /// log, nothing is replaced. Polled, cancelled and acknowledged like a load under the same operation.
    fn begin_document_archive_merge(&mut self, _operation: u64, _archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive merging is unavailable for this app"))
    }
    /// 📬️ Advances and observes one exact retained recursive archive operation.
    async fn poll_document_archive_load(&mut self, operation: u64, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, format!("recursive document archive operation {operation} is unavailable for this app")))
    }
    /// 🛑️ Requests cancellation without releasing the retained archive owner.
    fn cancel_document_archive_load(&mut self, _operation: u64) -> Result<(), Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive loading is unavailable for this app"))
    }
    /// 📨️ Releases one terminal archive operation after the caller consumed its status.
    fn acknowledge_document_archive_load(&mut self, _operation: u64) -> Result<(), Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive loading is unavailable for this app"))
    }
    /// 🛬️ The archive (`members: []`) a whole-document load of `files` into THIS instance admits: the pair stamped with the
    /// live store's identity, exactly like an `Effect::LoadDocument` leaving the instance (`📓️api-stepped-document-load.md` §4).
    fn document_load_archive(&self, _files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive loading is unavailable for this app"))
    }
    /// 🗃️ Reads a generation-fenced root and complete recursive member closure.
    /// 🪪️ Reads the live store root without serializing its document or media bytes.
    fn document_identity(&self) -> Option<String> {
        None
    }
    async fn document_archive(&self) -> Result<protocol::DocumentArchivePack, Fault> {
        Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive reading is unavailable for this app"))
    }
    /// 🧮️ Dispatches one binary-encoded `store::os_store::store_schema::ArtifactCommand<Self::ConfigMutation>` against the
    /// config store — the `AppCommand::ConfigCommand` wire frame's real handler (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    async fn dispatch_config_command(&mut self, command_bytes: &mut Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure>;
    /// 📥️ Ingests binary-encoded remote `MutationEnvelope`s (`protocol::decode_envelopes`)
    /// into the causal DAG (idempotent — duplicate mutation ids are dropped), one
    /// `crate::protocol::MergeReport` per envelope — `plugin_exchange`'s `ApplyEnvelopes` handler
    /// frames each as an unsolicited `AppFrame::MergeReport` (§C9: "pushed unsolicited after every
    /// ingest, next to `DocumentChanged`").
    async fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault>;
    /// 📜️ Text-DSL counterpart to {@link Self::ingest_operations}: applies one already-authored
    /// `Self::Mutation` per non-blank line (via `store::OpText::parse_op`) as a fresh local edit — unlike
    /// the JSON path (which ingests already-caused remote `MutationEnvelope`s into the causal DAG
    /// via `store.ingest_remote`, preserving their original ids/deps), each parsed line here goes
    /// through the normal `ArtifactCommand::Apply` path (a fresh id/timestamp, a real computed
    /// inverse) — the natural mapping for hand-authored or externally-generated op-text, which carries
    /// no envelope metadata of its own.
    async fn ingest_operations_text(&mut self, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault>;
    /// 📜️ Text-DSL counterpart to {@link Self::document_pack}: the whole document as
    /// {@link store::os_store::text_format::ArtifactTextFiles} (the `dsl` initial-snapshot text plus the full `ops` op-log
    /// text) via `store::os_store::text_format::print_document_text` — returned as the established two-file struct rather than
    /// a single concatenated string, since that struct (not an ad hoc delimiter format) is already the
    /// canonical text representation everywhere else in this codebase (`FolderTextStorage`,
    /// `parse_document_text`).
    async fn document_text(&self) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault>;
    /// 📦️ Binary-pack counterpart to {@link Self::document_text}: the whole document as
    /// {@link store::os_store::serialization::ArtifactPackFiles} (pack-encoded initial snapshot plus the same `ops` op-log
    /// text — the op grammar is format-invariant) via `store::os_store::text_format::print_document_pack`.
    async fn document_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault>;
    async fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault>;
    async fn attach_hot_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault>;
    async fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault>;
    async fn detach_backbone(&mut self) -> Result<(), Fault>;
    /// 🕰️ `view_state` supplies wrapper chrome and the OS-owned render context.
    /// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): return type flipped
    /// from `UiNode` to `ui_runtime::ComponentTree`, matching `ArtifactApp::render` above.
    async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<()>,semio_framework_job::OriginalTurnFailure>;
    fn original_render_tree_source<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<&'a mut Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>;
    async fn window_engagements(&mut self, _view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    async fn window_measures(&mut self, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🛠️ Object-safe counterpart to `ArtifactApp::tool_measures` — keyed by tool id.
    async fn tool_measures(&mut self, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🛍️ Object-safe counterpart to `ArtifactApp::app_catalogue_json` — the app-static catalogue the
    /// reserved `framework.section.catalogue` surface carries.
    async fn app_catalogue_json(&mut self) -> String {
        String::from("{}")
    }
    /// ⏱️ Object-safe counterpart to `ArtifactApp::pending_effects` — called once per `refreshUi` pass.
    async fn pending_effects(&mut self, _view: Option<&ViewModel>) -> Vec<Effect> {
        Vec::new()
    }
    /// 🖱️ Object-safe counterpart to `ArtifactApp::context_menu` — the WIT `context-menu` export's
    /// dispatch target.
    async fn context_menu(&mut self, _request: &ContextMenuRequest, _view_state: &ViewModel) -> Vec<ContextMenuItemSpec> {
        Vec::new()
    }
    /// 🎞️ Explicit batch-only counterpart to `ArtifactApp::export_media`. Interactive UI and
    /// workflow callers use `submit_media_export` plus poll/cancel and never enter this terminal loop.
    /// 🌊️ O1/§1.5 (sdk-dedyn): plain AFIT, not the old boxed `PluginAppMediaFuture` — with enum
    /// dispatch there is no object-safety pressure, so the future is `?Send` exactly when the
    /// state it holds is (R3), and no consumer moves it across threads (guest is single-threaded).
    async fn export_media(&mut self, _port: &str,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<Media>,semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"batch media export has no original owning source receiver").into())
    }
    /// 🎬️ Submits one interactive media export without awaiting terminal completion.
    async fn submit_media_export(&mut self, _port: &str,_receiver:&mut Option<ArtifactMediaExportHandle>,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"media submission has no owning source receiver").into())
    }
    async fn submit_media_export_checked(&mut self, _port: &str, _expected_parent_document_id: &str, _expected_base_revision: u64,_receiver:&mut Option<ArtifactMediaExportHandle>,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"checked media submission has no owning source receiver").into())
    }
    /// 📡️ Advances exactly one bounded worker slice and returns current progress/checkpoint/result.
    async fn poll_media_export(&mut self, _handle: crate::media_contract::ArtifactMediaExportHandleLoan<'_>,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<ArtifactMediaExportPoll>,semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"media polling has no owning source receiver").into())
    }
    /// 🛑️ Cancels an externally held live export handle.
    async fn cancel_media_export(&mut self, _handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        Err(MediaError::NotImplemented)
    }
    async fn retain_media_export_result(&mut self, _handle: &ArtifactMediaExportHandle, _result: ArtifactMediaExportResult) -> Result<(), MediaError> {
        Err(MediaError::NotImplemented)
    }
    async fn take_media_export_chunk(&mut self, _handle: &ArtifactMediaExportHandle) -> Result<Option<Vec<u8>>, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🧵️ Takes one bounded action-download chunk by its operation handle.
    async fn take_segmented_download_chunk(&mut self, _operation_id: u64) -> Result<Option<Vec<u8>>, Fault> {
        Err(plugin_sdk_fault("segmented download not implemented"))
    }
    /// 🎞️ Object-safe counterpart to `ArtifactApp::import_media` — dispatches through the same
    /// `ArtifactStore` as `handle_action`, so a headless import is an ordinary, undoable edit.
    /// 🎟️ Phases original cancellation admission while the receiving command still owns its media source.
    async fn prepare_media_import(&mut self,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{Err(plugin_sdk_fault(MediaError::NotImplemented.to_string()).into())}
    async fn import_media(&mut self, _port: &str, _media: Media, _meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        Err(plugin_sdk_fault(MediaError::NotImplemented.to_string()))
    }
    /// 🌀️ O1/§1.5 (sdk-dedyn): plain AFIT — see `export_media` above.
    async fn media_fingerprint(&mut self, _port: &str) -> Result<MediaFingerprint, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🎞️ ABI-level media artifact request for one port (`framework/wit/📜️world.wit`'s `produce-media`).
    /// Default: the whole document ([`whole_document_media`] of `document_archive()`, members included) — the fallback every
    /// `PluginApp` gets for free without declaring any `media_ports()`. Apps whose media output isn't simply their raw document
    /// (computed/derived outputs) override this directly; `port` is accepted for parity with `export_media` and ignored by the
    /// default (there is exactly one document to hand back).
    async fn produce_media(&mut self, port: &str, _owner: &mut crate::io::control::NativeSnapshotEncodeOwner<'_,'_>,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,crate::app_contract::MediaArtifactOperationFailure> {
        let archive = self.document_archive().await?;
        whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await.map(Some).map_err(Into::into)
    }
    /// 🎞️ ABI-level media artifact delivery for one port (`framework/wit/📜️world.wit`'s `consume-media`).
    /// Default: a `Document{schema}` wire matching this app's own `artifact_schema()` is a whole document — the carrier
    /// `produce_media` hands out ([`whole_document_archive`]) — answered as [`MediaConsumption::DocumentLoad`] with its archive
    /// verbatim (the source document and its members), which the runtime admits as a stepped archive load. Anything else (a
    /// foreign document schema, or a `Binary{format}` wire) has no SDK-level importer registry yet, so the default rejects it;
    /// apps that need one override this method directly.
    async fn consume_media(&mut self, _port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> {
        let schema_now = self.artifact_schema().await.to_string();
        match artifact.descriptor.wire {
            MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),
            MediaWireFormat::Document { schema } => Err(MediaArtifactError::SchemaMismatch { expected: schema_now, found: schema }),
            MediaWireFormat::Binary { format_kind } => Err(MediaArtifactError::NoImporter(format_kind)),
            MediaWireFormat::Intrinsic { schema } => Err(MediaArtifactError::NoImporter(schema)),
        }
    }
}

dyn_enum_close! {
    /// 🗃️ sdk-dedyn (O1/§1.5): the default-composes-nothing case (design-dedyn.md §1.6's `NoMembers`
    /// pattern, applied here to `PluginApp`) — a zero-variant enum, `dyn_enum_close!`-generated
    /// (every method's body degenerates to `match *self {}` since there is no value to construct).
    /// The default `PA` for every generic in this file's declaration tree, so a library-only plugin
    /// (or a test that never actually instantiates an app) never has to name a real app enum.
    pub enum NoPluginApp: PluginApp {}
}
