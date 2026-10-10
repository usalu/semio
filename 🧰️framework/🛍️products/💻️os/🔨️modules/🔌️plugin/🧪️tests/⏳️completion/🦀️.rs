//#region 🧰️TestAppCommandOwner
/// 🚫️ No app-owned tool factory and no bounded proof row at all — the shape the three fail-closed
/// laws (`unproved_command_fails_before_an_overrun_reducer_can_start`,
/// `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations`,
/// `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command`) need: they assert that an
/// UNPROVED verb is refused and that a non-migrated verb never reaches the live tool bus at all.
pub(crate) const TEST_APP_TOOLS_NONE: u8 = 0;
/// 🧾️ Owned factories for every verb, but NO `bounded_first_step_tool_proofs` row. A proof row is
/// only admissible while the LIVE registry declares its `OpBinary::TOOL_JOB_IDS` id `Migrated`
/// (`validate_tool_job_rows`'s `seen != expected` gate), so a registry-LESS wrapper can only ever
/// carry zero rows — which is exactly what `registry_less_construction_rejects_before_the_reducer`
/// constructs, and why it needs its own mode rather than the full one.
pub(crate) const TEST_APP_TOOLS_FACTORIES: u8 = 1;
/// 🧰️ Owned factories plus the proof rows for the generated tool ids — the migrated fixture every
/// behavioural law uses, paired with `migrated_contract_registry`.
pub(crate) const TEST_APP_TOOLS_FULL: u8 = 2;

const TEST_APP_COMMAND_SCHEMA: &str = "semio.test.app-command.v1";
const TEST_APP_COMMAND_RAW_BYTES: usize = 65_536;
const TEST_APP_COMMAND_WORK_ITEMS: usize = 1;

/// 🪪️ Every tool id `TestApp::command_id` can answer — the app's own dispatch surface. A verb
/// missing here has no exact owner-qualified reducer and is refused with
/// `interactive-job.missing-factory` before it reaches `handle`.
const TEST_APP_COMMAND_TOOL_IDS: &[&str] = &[
    "increment",
    "setLabel",
    "streamLabel",
    "badView",
    "select",
    "navigate",
    "noopMutation",
    "viewNoScope",
    "viewPartialScope",
    "incrementViaCommand",
    "watchdogOverrun",
    "setLabelViaCommand",
    "setActiveUtility",
    "targetWindow",
    "mode.increment",
    "compositeEdit",
    "probeChild",
    "spawnCountTask",
    "applyCountFromTask",
];

/// 🚚️ The store lanes each `TestApp` verb's reducer actually writes. `publish_typed_operation`
/// refuses any emit that touches a lane its factory did not declare, and app construction refuses a
/// lane whose store preparation authority the app does not install — so this table, the reducer and
/// `build_*_store_one_item_preparation_factory` are one declaration in three places.
const TEST_APP_COMMAND_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "increment", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "setLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "streamLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "badView", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "select", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "navigate", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "noopMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "viewNoScope", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "viewPartialScope", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "incrementViaCommand", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "watchdogOverrun", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setLabelViaCommand", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "targetWindow", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "mode.increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "probeChild", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "spawnCountTask", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "applyCountFromTask", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn test_app_command_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(TEST_APP_COMMAND_RAW_BYTES, 4, 1, TEST_APP_COMMAND_RAW_BYTES, 7_500, 1, 1)
}

/// 🧮️ Runs the app's OWN reducer against the roots this job retained — `TestApp::handle` and this
/// route share `test_app_reduce` verbatim, so a migrated dispatch and a direct one cannot diverge.
fn test_app_command_emit<const RETAINED: bool, const TOOLS: u8>(job: &TestAppCommandJob<RETAINED, TOOLS>) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
    let command = job.command.as_deref().ok_or_else(|| Fault::from("test-app-command-consumed"))?;
    let snapshot = job.snapshot.as_deref().ok_or_else(|| Fault::from("test-app-command-snapshot-retired"))?;
    let config = job.config.as_deref().ok_or_else(|| Fault::from("test-app-command-config-retired"))?;
    let history = job.history.as_deref().ok_or_else(|| Fault::from("test-app-command-history-retired"))?;
    let children = job.context.as_ref().map_or(ChildContentView::EMPTY, |context| ChildContentView::clone(&context.children));
    let doc = ArtifactView::with_children(snapshot, history, children);
    let cfg = ConfigView { snapshot: config, window: job.context.as_ref().and_then(|context| context.window_config.as_ref()) };
    test_app_reduce(command, &doc, &cfg)
}

/// 🧵️ `TestApp`'s app-owned reducer job. Deliberately the SAME minimal shape as `TestRestartJob` and
/// `DummyFixtureJob` rather than the generic `retained_command::ArtifactRetainedCommandJob`: that
/// type inlines an `Emit`, an `EphemeralEmit`, a 512-byte checkpoint buffer and two wire-page
/// owners, and the framework-reserved dispatch generator this fixture also drives is already
/// measured at ~1.75 MiB of the 2 MiB bounded thread stack
/// (`one_framework_reserved_route_fits_a_bounded_thread_stack`).
struct TestAppCommandInputs {
    raw_wire: crate::app::ArtifactToolRawInput,
    window_config: Option<crate::app::WindowConfigSnapshot>,
    interaction_state: std::sync::Arc<InteractionState>,
    interaction_hover: std::sync::Arc<InteractionHoverState>,
    instance_operation_owner: crate::app::ArtifactInstanceOperationOwnerHandle,
    output_chunks: ArtifactOutputChunks,
}

struct TestAppCommandJob<const RETAINED: bool, const TOOLS: u8> {
    residual: Option<TestAppCommandInputs>,
    rejected: Option<crate::app::ArtifactToolCompletionRejection<TestApp<RETAINED, TOOLS>>>,
    admitted: bool,
    metadata: Option<AppOperationContext>,
    command: Option<Box<TestCommand>>,
    snapshot: Option<std::sync::Arc<TestSnapshot>>,
    snapshot_read: Option<store::SnapshotRead<TestSnapshot>>,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    config: Option<std::sync::Arc<TestConfig>>,
    history: Option<std::sync::Arc<HistoryView>>,
    context: Option<ArtifactOwnedContextHandle<TestApp<RETAINED, TOOLS>>>,
    completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
    raw: Option<action_bus::RetainedToolWireInput>,
    page: usize,
    closing: bool,
}

impl<const RETAINED: bool, const TOOLS: u8> TestAppCommandJob<RETAINED, TOOLS> {
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(raw) = self.raw.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? });
        }
        if let Some(owner) = self.retirement.as_ref() { return store::artifact_retirement_box_demands(owner, body); }
        if self.command.is_some() { return store::artifact_retirement_owned_birth_demands(&self.command); }
        if self.snapshot.is_some() { return store::artifact_retirement_owned_birth_demands(&self.snapshot); }
        if self.config.is_some() { return store::artifact_retirement_owned_birth_demands(&self.config); }
        if self.history.is_some() { return store::artifact_retirement_owned_birth_demands(&self.history); }
        if self.completion.is_some() { return store::artifact_retirement_owned_birth_demands(&self.completion); }
        if self.metadata.is_some() { return store::artifact_retirement_owned_birth_demands(&self.metadata); }
        if self.context.is_some() || self.residual.is_some() || self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture retains original context until its issuing authority is installed")); }
        if self.snapshot_read.is_some() { return store::artifact_retirement_owned_birth_demands(&self.snapshot_read); }
        Ok(Default::default())
    }
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestAppCommandJob<RETAINED, TOOLS> {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if self.admitted {
            return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput) });
        }
        if self.rejected.is_some() { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: crate::app::retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, b"test app retains its rejected original completion") }); }
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.set_stage("test-command-ephemeral");
        cx.consume_fuel(1);
        let command = self.command.as_deref().expect("exact worker command");
        let context = self.context.as_ref().expect("exact worker context");
        let doc = ArtifactView::with_children(self.snapshot.as_deref().unwrap(), self.history.as_deref().unwrap(), ChildContentView::clone(&context.children));
        let cfg = ConfigView { snapshot: self.config.as_deref().unwrap(), window: context.window_config.as_ref() };
        let presence = context.presence_view().expect("captured worker presence authority");
        let transient = TransientView { snapshot: context.transient.as_ref(), window: context.window_transient.as_ref() };
        let ephemeral = ::semio_framework_async::poll::resolve_ready(TestApp::<RETAINED, TOOLS>::ephemeral(command, &doc, &cfg, &presence, &transient));
        let emit = test_app_command_emit(self);
        let completion = self.completion.as_ref().expect("exact test app command completion");
        if let Err(original) = completion.complete(emit, ephemeral) {
            self.rejected = Some(original);
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: crate::app::retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, b"test app original completion was rejected") });
        }
        self.admitted = true;
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if !self.closing { return InteractiveJobCloseStep::Blocked; }
        if grant.maximum_items == 0 { return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }; }
        let demand = match self.close_demands(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return InteractiveJobCloseStep::Refused(error.kind) };
        if grant.maximum_depth < demand.depth { return InteractiveJobCloseStep::Refused(semio_framework_value::ValueRefusalKind::DepthLimit); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }; }
        if let Some(raw) = self.raw.as_mut() {
            let step = raw.close_step(grant);
            if raw.terminal_is_empty() { self.raw.take(); }
            return match step { InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress }, step => step };
        }
        let step = if self.retirement.is_some() { store::artifact_retirement_box_close_step(&mut self.retirement, grant) }
            else if self.command.is_some() { store::artifact_retirement_admit_owned(&mut self.command, &mut self.retirement, grant) }
            else if self.snapshot.is_some() { store::artifact_retirement_admit_owned(&mut self.snapshot, &mut self.retirement, grant) }
            else if self.config.is_some() { store::artifact_retirement_admit_owned(&mut self.config, &mut self.retirement, grant) }
            else if self.history.is_some() { store::artifact_retirement_admit_owned(&mut self.history, &mut self.retirement, grant) }
            else if self.completion.is_some() { store::artifact_retirement_admit_owned(&mut self.completion, &mut self.retirement, grant) }
            else if self.metadata.is_some() { store::artifact_retirement_admit_owned(&mut self.metadata, &mut self.retirement, grant) }
            else if self.snapshot_read.is_some() { store::artifact_retirement_admit_owned(&mut self.snapshot_read, &mut self.retirement, grant) }
            else { Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())) };
        match step {
            Ok(step) if self.terminal_is_empty() => InteractiveJobCloseStep::Complete { progress: step.progress() },
            Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
            Err(error) => InteractiveJobCloseStep::Refused(error.kind),
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.rejected.is_none() && self.residual.is_none() && self.metadata.is_none() && self.retirement.is_none() && self.raw.is_none() && self.command.is_none() && self.snapshot_read.is_none() && self.snapshot.is_none() && self.config.is_none() && self.history.is_none() && self.context.is_none() && self.completion.is_none()
    }
}

struct TestAppCommandFactory<const RETAINED: bool, const TOOLS: u8> {
    keys: Vec<ToolFactoryKey>,
}

impl<const RETAINED: bool, const TOOLS: u8> TestAppCommandFactory<RETAINED, TOOLS> {
    fn new(controller_id: &str) -> Self {
        Self { keys: TEST_APP_COMMAND_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl<const RETAINED: bool, const TOOLS: u8> ToolJobFactory for TestAppCommandFactory<RETAINED, TOOLS> {
    type Payload = TestAppCommandJob<RETAINED, TOOLS>;
    type Job = TestAppCommandJob<RETAINED, TOOLS>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        TEST_APP_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        test_app_command_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        mut payload: Self::Payload,
        input: action_bus::RetainedToolWireInput,
        checkpoint: Option<action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("test app command resume starts a fresh command owner"), input, checkpoint));
        }
        payload.raw = Some(input);
        Ok(payload)
    }
}

impl<const RETAINED: bool, const TOOLS: u8> ArtifactOwnedToolJobFactory for TestAppCommandFactory<RETAINED, TOOLS> {
    type Owner = TestApp<RETAINED, TOOLS>;
    const TOOL_IDS: &'static [&'static str] = TEST_APP_COMMAND_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TEST_APP_COMMAND_PUBLICATION_CONTRACTS;
}

//#endregion 🧰️TestAppCommandOwner

//#region 📬️TestCompletionRetirement
type TestCompletionValue<const RETAINED: bool, const TOOLS: u8> = ArtifactToolCompletionValue<TestApp<RETAINED, TOOLS>>;

struct TestCompletionPayload<const RETAINED: bool, const TOOLS: u8>(TestCompletionValue<RETAINED, TOOLS>);

struct TestCompletionCursor<const RETAINED: bool, const TOOLS: u8> {
    original: std::mem::ManuallyDrop<Option<TestCompletionValue<RETAINED, TOOLS>>>,
}

/// 📬️ The fixture app retains only vacant emitted completions: every lane is empty, so the original payload owns no further allocation.
fn test_completion_supported<const RETAINED: bool, const TOOLS: u8>(value: &TestCompletionValue<RETAINED, TOOLS>) -> bool {
    match value {
        ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
            emit.artifact_mutations.is_empty()
                && emit.config_mutations.is_empty()
                && emit.window_config_mutations.is_empty()
                && emit.draft_mutations.is_empty()
                && emit.effects.is_empty()
                && emit.extension_invocations.is_empty()
                && emit.events.is_empty()
                && emit.child_emits.is_empty()
                && emit.owned_child_emits.is_empty()
                && emit.child_preparations.is_empty()
                && emit.interaction_writes.is_empty()
                && emit.tasks.is_empty()
                && ephemeral.presence.is_empty()
                && ephemeral.transient.is_empty()
                && ephemeral.window_transient.is_empty()
        }
        _ => false,
    }
}

fn test_completion_birth<const RETAINED: bool, const TOOLS: u8>(value: &TestCompletionValue<RETAINED, TOOLS>) -> Option<usize> {
    test_completion_supported(value).then_some(std::mem::size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<TestCompletionPayload<RETAINED, TOOLS>>>())
}

fn test_completion_admit<const RETAINED: bool, const TOOLS: u8>(
    value: &mut Option<TestCompletionValue<RETAINED, TOOLS>>,
    grant: semio_framework_value::retained_clone::RetainedCloneGrant,
) -> Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
    let Some(original) = value.as_ref() else { return Ok(None) };
    let bytes = test_completion_birth(original).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture completion retains a non-vacant payload without a retirement facet"))?;
    if grant.maximum_items == 0 || grant.maximum_capacity_bytes < bytes || grant.maximum_depth == 0 {
        return Ok(None);
    }
    match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(TestCompletionPayload(value.take().unwrap()), grant) {
        Ok((owner, progress)) => Ok(Some((owner as Box<dyn semio_framework_value::ErasedSnapshotRetirement>, progress))),
        Err((error, TestCompletionPayload(original))) => {
            *value = Some(original);
            Err(error)
        }
    }
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_value::retirement::RetireOwned for TestCompletionPayload<RETAINED, TOOLS> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(TestCompletionCursor { original: std::mem::ManuallyDrop::new(Some(self.0)) })
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        test_completion_supported(&self.0).then_some(std::mem::size_of::<TestCompletionCursor<RETAINED, TOOLS>>())
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_value::retirement::RetirementCursor for TestCompletionCursor<RETAINED, TOOLS> {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if self.original.is_none() {
            return RetirementStep::Complete;
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        drop(self.original.take());
        RetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.original.is_none()
    }

    fn next_birth_bytes(&self, _copy: usize) -> Option<usize> {
        Some(0)
    }

    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(0)
    }

    fn terminal_release_bytes(&self) -> Option<usize> {
        self.original.is_none().then_some(std::mem::size_of::<Self>())
    }
}

impl<const RETAINED: bool, const TOOLS: u8> Drop for TestCompletionCursor<RETAINED, TOOLS> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.original.is_none(), "fixture completion payload reached Drop before it returned");
        if self.original.is_none() {
            unsafe { std::mem::ManuallyDrop::drop(&mut self.original) };
        }
    }
}
//#endregion 📬️TestCompletionRetirement

//#region 🧵️RestartCommandOwner
const TEST_RESTART_TOOL: &str = "applyCountFromTask";
const TEST_RESTART_SCHEMA: &str = "semio.test.restart-command.v1";

fn test_restart_proofs<const RETAINED: bool, const TOOLS: u8>() -> Vec<ArtifactBoundedFirstStepProof> {
    if RETAINED {
        return vec![
            ArtifactBoundedFirstStepProof::new::<TestApp<RETAINED, TOOLS>>(file!(), TestApp::<RETAINED, TOOLS>::APP_ID, "TestRestartFactory<true, 2>", TEST_RESTART_TOOL, TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA, ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1))
                .with_factory_type::<TestApp<RETAINED, TOOLS>, TestRestartFactory<RETAINED, TOOLS>>(),
        ];
    }
    if TOOLS != TEST_APP_TOOLS_FULL {
        return Vec::new();
    }
    <TestCommand as ::protocol::OpBinary>::TOOL_JOB_IDS
        .iter()
        .map(|tool_id| {
            ArtifactBoundedFirstStepProof::new::<TestApp<RETAINED, TOOLS>>(file!(), TestApp::<RETAINED, TOOLS>::APP_ID, "TestAppCommandFactory<false, 2>", tool_id, TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA, test_app_command_contract())
                .with_factory_type::<TestApp<RETAINED, TOOLS>, TestAppCommandFactory<RETAINED, TOOLS>>()
        })
        .collect()
}

fn test_restart_register<const RETAINED: bool, const TOOLS: u8>(registry: &mut ArtifactToolFactoryRegistry<'_, TestApp<RETAINED, TOOLS>>) -> Result<(), Fault> {
    if RETAINED {
        return registry.register(TestRestartFactory::<RETAINED, TOOLS> { keys: vec![ToolFactoryKey::new(registry.controller_id(), TEST_RESTART_TOOL)] });
    }
    if TOOLS == TEST_APP_TOOLS_NONE {
        return Ok(());
    }
    registry.register(TestAppCommandFactory::<RETAINED, TOOLS>::new(registry.controller_id()))
}

async fn test_restart_build<const RETAINED: bool, const TOOLS: u8>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if RETAINED {
        assert!(matches!(request.command.as_ref(), TestCommand::ApplyCountFromTask { .. }));
        assert!(request.snapshot.label.is_empty(), "restart fixture begins from its actual fresh snapshot");
        return test_app_command_spec::<RETAINED, TOOLS>(request);
    }
    if TOOLS == TEST_APP_TOOLS_NONE || !TEST_APP_COMMAND_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    test_app_command_spec::<RETAINED, TOOLS>(request)
}

/// 🧵️ Builds the job on its OWN synchronous frame — every real editor's
/// `ArtifactEditor::build_tool_job` is sync for the same bounded-stack reason.
#[inline(never)]
fn test_app_command_spec<const RETAINED: bool, const TOOLS: u8>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Option<ToolOperationSpec>, Fault> {
    let job = TestAppCommandJob {
        rejected: None,
        admitted: false,
        residual: Some(TestAppCommandInputs { raw_wire: request.raw_wire, window_config: request.window_config, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, instance_operation_owner: request.instance_operation_owner, output_chunks: request.output_chunks }),
        metadata: Some(AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed }),
        command: Some(request.command),
        snapshot: Some(request.snapshot),
        snapshot_read: Some(request.snapshot_read),
        retirement: None,
        config: Some(request.config),
        history: Some(request.history),
        context: Some(request.context),
        completion: Some(request.completion),
        raw: None,
        page: 0,
        closing: false,
    };
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
}

async fn test_restart_registry() -> AppActionRegistry {
    let manifest = App::from_builder(
        App::builder(TestApp::<true>::APP_ID, LocalizedLabel::data("Restart Fixture"))
            .await.document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil").await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow).await
            .app_command(TEST_RESTART_TOOL, LocalizedLabel::data("Apply Count From Task"), "task", ActionKind::Mutation).await
            .interactive_jobs(InteractiveJobClassification::Migrated).await,
    ).await;
    AppActionRegistry::from_definition(&manifest.definition)
}

async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &Value) {
    let law = &fixture["restartAuthority"];
    let items = law["closeItems"].as_u64().unwrap() as usize;
    let body = law["closeBodyBytes"].as_u64().unwrap() as usize;
    let mut app = VcsArtifactApp::<TestApp<true>>::with_registry(TestApp::<true>::default(), test_restart_registry().await, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
    let outcome: Result<(u64, u64, u64, u64, u64, i32), Fault> = async {
        app.bind_instance_id(meta.instance_id).await;
        let contracts = app.tool_public_contracts().await;
        let exact = contracts.iter().filter(|contract| contract.tool_id == TEST_RESTART_TOOL && contract.owner == ToolOwnerWitness::of::<TestApp<true>>() && contract.controller_id == TestApp::<true>::APP_ID && contract.schema_id == TEST_RESTART_SCHEMA).count();
        if exact as u64 != law["retainedProofs"].as_u64().unwrap() { return Err(Fault::from("restart app lost its exact registered tool contract")); }
        let admitted = app.dispatch_typed(command, meta).await?;
        if !admitted.mutations.is_empty() { return Err(Fault::from("restart command bypassed retained publication")); }
        let (mut artifact, mut ui, mut scopes, mut terminal, mut completions) = (0, 0, 0, 0, 0);
        for _ in 0..100_000 {
            let demand = app.maintenance_retirement_demands(body).map_err(|error| Fault::from(error.into_message()))?;
            let grant = RetainedCloneGrant { maximum_items: items, ..plugin_demand_grant(demand) };
            if let Some(progress) = app.maintenance_step(grant)?.progress() {
                if !progress.fits(grant) { return Err(Fault::from("restart maintenance exceeded the exact grant")); }
            }
            app.advance_typed_operation_publication().await?;
            if let Some(page) = app.take_typed_operation_result_page(meta.instance_id) {
                let fault = match page.lane {
                    TypedOperationResultLane::Artifact => { artifact += 1; None }
                    TypedOperationResultLane::Ui => { ui += 1; None }
                    TypedOperationResultLane::Terminal => { terminal += 1; None }
                    TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),
                    _ => Some(Fault::from("restart publication produced an undeclared lane")),
                };
                if !app.acknowledge_typed_operation_result(page.token)? { return Err(Fault::from("restart publication rejected its exact result ACK")); }
                if let Some(fault) = fault { return Err(fault); }
            }
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                if !matches!(scope, semio_framework::kernel::UiDirtyScope::Full) { return Err(Fault::from("restart publication changed its declared full UI scope")); }
                scopes += 1;
            }
            if let Some(completion) = app.take_typed_operation_completion().await? {
                if !matches!(completion.ui_scope, semio_framework::kernel::UiDirtyScope::Full) { return Err(Fault::from("restart completion changed its declared full UI scope")); }
                if completion.operation == 0 { return Err(Fault::from("restart completion lost its exact operation id")); }
                completions += 1;
            }
            if !app.has_pending_typed_operations() { return Ok((artifact, ui, scopes, terminal, completions, app.snapshot()?.count)); }
            std::thread::yield_now();
        }
        Err(Fault::from("restart publication did not retire within the existing fixture turn bound"))
    }.await;
    let mut close_fault = None;
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() { break; }
        let grant = match app.close_retirement_demands(body) {
            Ok(demand) => RetainedCloneGrant { maximum_items: items, ..plugin_demand_grant(demand) },
            Err(error) => { close_fault = Some(Fault::from(error.into_message())); break; }
        };
        match app.close_step(grant) {
            Ok(step) if step.progress().is_some_and(|progress| !progress.fits(grant)) => {
                close_fault = Some(Fault::from("restart close exceeded the exact grant"));
                break;
            }
            Err(fault) => { close_fault = Some(fault); break; }
            _ => {}
        }
        std::thread::yield_now();
    }
    let closed = app.close_terminal_is_empty();
    eprintln!("restart retained publication outcome={outcome:?}, closed={closed}, close_fault={close_fault:?}");
    assert!(closed, "original restart app must retire before the collected publication result is asserted");
    drop(app);
    assert!(close_fault.is_none(), "{close_fault:?}");
    let (artifact, ui, scopes, terminal, completions, count) = outcome.expect("actual registered restart publication");
    assert_eq!(artifact, law["artifactPublications"].as_u64().unwrap());
    assert_eq!(ui, law["uiPublications"].as_u64().unwrap());
    assert_eq!(scopes, law["uiScopes"].as_u64().unwrap());
    assert_eq!(terminal, law["terminalReceipts"].as_u64().unwrap());
    assert_eq!(completions, law["operationCompletions"].as_u64().unwrap(), "one terminal typed operation publishes exactly one AppFrame::OperationCompleted witness");
    assert_eq!(i64::from(count), fixture["checkpoint"]["restartValue"].as_i64().unwrap());
}

struct TestRestartFactory<const RETAINED: bool, const TOOLS: u8> { keys: Vec<ToolFactoryKey> }

impl<const RETAINED: bool, const TOOLS: u8> ToolJobFactory for TestRestartFactory<RETAINED, TOOLS> {
    type Payload = TestAppCommandJob<RETAINED, TOOLS>;
    type Job = TestAppCommandJob<RETAINED, TOOLS>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { TEST_RESTART_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() { return Err((ToolJobFactoryError::new("restart resume starts a fresh command owner"), input, checkpoint)); }
        payload.raw = Some(input);
        Ok(payload)
    }
}

impl<const RETAINED: bool, const TOOLS: u8> ArtifactOwnedToolJobFactory for TestRestartFactory<RETAINED, TOOLS> {
    type Owner = TestApp<RETAINED, TOOLS>;
    const TOOL_IDS: &'static [&'static str] = &[TEST_RESTART_TOOL];
    const DOCUMENT_SCHEMA: &'static str = TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TEST_RESTART_TOOL, lanes: &[ArtifactToolPublicationLane::Artifact] }];
}
//#endregion 🧵️RestartCommandOwner

//#region 🫧️RestartTransientOwner
#[test]
fn checkpoint_restart_transient_close_retains_the_exact_store_until_granted() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏳️completion/🔣️.json")).unwrap();
    let law = &fixture["transientClose"];
    let ((mut owner, mut close), original_heap) = observe_heap_allocations_on_this_thread(|| (
        store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient { revision: law["sourceRevision"].as_u64().unwrap() }),
        TestApp::<true>::build_transient_store_disposer().expect("actual owned transient issuer"),
    ));
    let source = owner.current_root();
    let pointer = std::sync::Arc::as_ptr(&source);
    assert_eq!(serde_json::to_value(source.as_ref()).unwrap(), json!({ "revision": law["sourceRevision"] }));
    drop(source);
    assert_eq!(close.terminal_is_empty(&owner), law["initiallyTerminal"].as_bool().unwrap());
    let mut births = 0usize;
    let mut releases = 0usize;
    let mut complete = false;
    for _ in 0..65_536 {
        let grant = crate::app::retirement_self_grant(|body| close.retirement_demands(&owner, body), 7).unwrap();
        for axis in law["deniedAxes"].as_array().unwrap() {
            let mut denied = grant;
            let positive = match axis.as_str().unwrap() {
                "items" => { denied.maximum_items = 0; true },
                "copy" => { denied.maximum_copy_bytes = 0; grant.maximum_copy_bytes > 0 },
                "capacity" => { denied.maximum_capacity_bytes = 0; grant.maximum_capacity_bytes > 0 },
                "release" => { denied.maximum_release_bytes = 0; grant.maximum_release_bytes > 0 },
                "depth" => { denied.maximum_depth = 0; grant.maximum_depth > 0 },
                _ => panic!("declared independent grant axis"),
            };
            if !positive { continue; }
            let (step, heap) = observe_heap_allocations_on_this_thread(|| close.close_step(&mut owner, denied));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if let Ok(step) = step { assert_eq!(step.progress().unwrap_or_default(), Default::default()); }
            if !owner.detached_terminal_is_empty() {
                let source = owner.current_root();
                assert_eq!(std::sync::Arc::as_ptr(&source), pointer);
            }
        }
        let (step, heap) = observe_heap_allocations_on_this_thread(|| close.close_step(&mut owner, grant).unwrap());
        let progress = step.progress().expect("physical transient lifecycle receipt");
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        births += heap.requested_bytes;
        releases += heap.released_bytes;
        if close.terminal_is_empty(&owner) { complete = true; break; }
    }
    assert!(complete);
    assert!(owner.detached_terminal_is_empty());
    let mut foreign = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient::default());
    assert_eq!(close.terminal_is_empty(&foreign), law["foreignTerminal"].as_bool().unwrap());
    let frame = close.terminal_frame_release_bytes().expect("actual terminal disposer Box");
    let ((), heap) = observe_heap_allocations_on_this_thread(|| drop(close));
    assert_eq!(heap.released_bytes, frame);
    releases += heap.released_bytes;
    assert_eq!(releases, original_heap.requested_bytes + births);
    let result = json!({ "terminal": complete, "detachesWithoutReplacement": owner.detached_terminal_is_empty() });
    assert_eq!(result, law["expected"]);
    drop(foreign);
    eprintln!("[DEBUG] Actual restart transient retained each denied axis; body and original disposer frame released from their independent quotes");
}
//#endregion 🫧️RestartTransientOwner
                 