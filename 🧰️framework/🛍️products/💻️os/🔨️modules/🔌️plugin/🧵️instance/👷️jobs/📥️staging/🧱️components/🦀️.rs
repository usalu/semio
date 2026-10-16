//! 🪁️ The instance components of `instance-jobs`: PendingReserved, ToolOperations (WP6, plan section 7.3).
use super::*;

/// 🪂️ Pending reserved work: the reserved commits, render trees, configuration commands and their failure custody.
pub struct PendingReserved<A: ArtifactApp> {
    /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
    pub last_emit_wire: Option<EmitWire>,
    /// 📨️ The authoritative outcome of the current app command, reset before dispatch and then
    /// filled from the store receipt or policy rejection before the runtime frames it.
    pub dispatch_report: protocol::DispatchReport,
    pub pending_render_tree: std::mem::ManuallyDrop<Option<ComponentTree>>,
    pub pending_render_tree_identity: Option<OriginalRenderTreeIdentity>,
    pub pending_render_tree_ready: bool,
    pub pending_render_tree_close:std::mem::ManuallyDrop<Option<semio_framework_ui_contract::OriginalBuiltTreeRetirement>>,
    pub pending_configuration_completion:Option<crate::reserved_jobs::ConfigurationDecodeRetirement<A::ConfigMutation>>,
    pub pending_configuration_command:std::mem::ManuallyDrop<Option<ArtifactCommand<A::ConfigMutation>>>,
    pub pending_configuration_command_close:Option<ControlledRetirement<ArtifactCommand<A::ConfigMutation>>>,
    pub pending_configuration_permit:Option<FrameworkReservedCommitPermit>,
    pub pending_configuration_failure:std::mem::ManuallyDrop<Option<crate::reserved_jobs::OriginalConfigurationFailure>>,
    pub pending_configuration_identity:Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub pending_configuration_retirement:Option<ControlledRetirement<crate::reserved_jobs::OriginalConfigurationFailure>>,
    pub pending_render_failure: std::mem::ManuallyDrop<Option<crate::OriginalRenderFailure>>,
    pub pending_render_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub pending_render_close_failure:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    pub pending_render_compound:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalClosureFailure>>,
    pub pending_render_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<crate::OriginalRenderFailure>>>,
    pub pending_reserved: ArtifactFixedRegistry<PendingFrameworkReserved>,
    pub reserved_commits: std::collections::VecDeque<FrameworkReservedCommit>,
    pub reserved_commit_outcome: Option<Result<InvocationResult, Fault>>,
    pub pending_document_window_admission:std::mem::ManuallyDrop<Option<DocumentWindowResetAdmission>>,
}

impl<A: ArtifactApp> PendingReserved<A> {
    /// 🛟️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new(dispatch_report: protocol::DispatchReport) -> Self {
        Self {
            last_emit_wire: None,
            dispatch_report,
            pending_render_tree: std::mem::ManuallyDrop::new(None),
            pending_render_tree_identity: None,
            pending_render_tree_ready: false,
            pending_render_tree_close: std::mem::ManuallyDrop::new(None),
            pending_configuration_completion: None,
            pending_configuration_command: std::mem::ManuallyDrop::new(None),
            pending_configuration_command_close: None,
            pending_configuration_permit: None,
            pending_configuration_failure: std::mem::ManuallyDrop::new(None),
            pending_configuration_identity: None,
            pending_configuration_retirement: None,
            pending_render_failure: std::mem::ManuallyDrop::new(None),
            pending_render_identity: None,
            pending_render_close_failure: std::mem::ManuallyDrop::new(None),
            pending_render_compound: std::mem::ManuallyDrop::new(None),
            pending_render_retirement: std::mem::ManuallyDrop::new(None),
            pending_reserved: ArtifactFixedRegistry::new(),
            reserved_commits: std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS),
            reserved_commit_outcome: None,
            pending_document_window_admission: std::mem::ManuallyDrop::new(None),
        }
    }
}

/// 🪐️ Tool operations: typed operation outboxes, tool jobs and registrations, cancellations and latest-wins commands.
pub struct ToolOperations<A: ArtifactApp> {
    pub pending_tool_lease_admission:ManuallyDrop<Option<OriginalToolLeaseAdmission>>,
    pub typed_effect_outbox: ArtifactFixedQueue<Effect>,
    pub typed_event_outbox: ArtifactFixedQueue<AppEvent>,
    pub typed_ui_outbox: ArtifactFixedQueue<UiDirtyScope>,
    pub operation_progress_retired: bool,
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — per mounted typed operation (keyed by
    /// its operation id), the guest-emitted interaction verbs (`Effect::ReplayShellCommand`/
    /// `DispatchAction` with an `INTERACTION_ACTION_IDS` action, see [`is_inline_interaction_verb`])
    /// peeled out of its emit the INSTANT the ladder takes the completion
    /// (`publish_mounted_typed_operation_unit`'s install step) — before any effect can be popped
    /// into `typed_effect_outbox` — and parked until `mounted_typed_inline_interaction_verbs_are_next`
    /// folds them in-reactor. The host never sees one: a verb that reached the effect outbox on
    /// the install pump was re-dispatched by the shell as a second `onAction` (18
    /// `replayShellCommand dispatch interactionSelect` lines per battery on the fem2d lane),
    /// behind any pointer input issued meanwhile. Bounded by the live operation slots; an entry
    /// is consumed by the fold or dropped with its operation's exact removal.
    pub typed_inline_interaction_verbs: Vec<(u64, Vec<(String, Option<DslValue>)>)>,
    /// 🔀️ The LAST folded verb's leftover `InteractionView` (`AppliedInteractionVerb::leftover`),
    /// one slot, latest wins (a later verb's snapshot already includes the earlier ones — the
    /// same rule `InlineInteractionFold::leftover` follows). It rides the next `Ui` progress
    /// frame's `output` (`take_typed_operation_ui_progress`) — the typed ladder's twin of the
    /// `Invocation.output.interactionView` lane a host-dispatched verb answers on.
    pub typed_inline_interaction_leftover: Option<TypedOperationLeftover>,
    pub typed_completion_outbox: ArtifactFixedQueue<TypedOperationCompletionWitness>,
    /// 🧩️ See [`ComposedGestureResult`] — one entry per composed gesture this actor published.
    pub typed_composed_outbox: ArtifactFixedQueue<ComposedGestureResult>,
    pub registry: AppActionRegistry,
    pub tool_job_controller_id: String,
    pub tool_jobs: semio_framework::ActionBus,
    pub agent_lane_preview:Option<crate::tool_jobs::AgentLanePreview<A>>,
    pub bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>,
    pub bounded_tool_contracts: Vec<ArtifactToolPublicContract>,
    pub framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    pub app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    pub instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub tool_cancellations: crate::tool_cancellation::ToolCancellationIssuer,
    pub live_runtime_instance_id: Option<u32>,
    pub tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>,
    #[cfg(test)]
    pub test_tool_clock: Option<fn() -> Option<u64>>,
    pub typed_operation_reservations: [Option<u64>; ARTIFACT_LIVE_OUTPUT_SLOTS],
    pub typed_publication_cursor: usize,
    pub typed_result_cursor: usize,
    pub latest_wins_commands: ArtifactFixedRegistry<PendingLatestWinsCommand<A>>,
    pub latest_wins_order: ArtifactFixedQueue<u64>,
    pub latest_wins_keys: ToolLatestWinsRegistry,
    pub latest_wins_turn: bool,
}

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🔭️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new(registry: AppActionRegistry, tool_job_controller_id: String, tool_jobs: semio_framework::ActionBus, bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>, bounded_tool_contracts: Vec<ArtifactToolPublicContract>, framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, instance_operation_owner: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self {
            pending_tool_lease_admission: ManuallyDrop::new(None),
            typed_effect_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_event_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_ui_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            operation_progress_retired: false,
            typed_inline_interaction_verbs: Vec::new(),
            typed_inline_interaction_leftover: None,
            typed_completion_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_composed_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            registry,
            tool_job_controller_id,
            tool_jobs,
            agent_lane_preview: None,
            bounded_tool_proofs,
            bounded_tool_contracts,
            framework_tool_registrations,
            app_tool_registrations,
            instance_operation_owner,
            tool_cancellations: crate::tool_cancellation::ToolCancellationIssuer::empty(),
            live_runtime_instance_id: None,
            tool_operations: ArtifactFixedRegistry::new(),
            #[cfg(test)]
            test_tool_clock: None,
            typed_operation_reservations: [None; ARTIFACT_LIVE_OUTPUT_SLOTS],
            typed_publication_cursor: 0,
            typed_result_cursor: 0,
            latest_wins_commands: ArtifactFixedRegistry::new(),
            latest_wins_order: ArtifactFixedQueue::new(ARTIFACT_LIVE_OUTPUT_SLOTS),
            latest_wins_keys: ToolLatestWinsRegistry::new(),
            latest_wins_turn: true,
        }
    }
}
