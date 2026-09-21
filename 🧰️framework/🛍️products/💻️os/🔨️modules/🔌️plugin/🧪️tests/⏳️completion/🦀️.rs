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
    "amendLabel",
    "commitLabel",
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
    ArtifactToolPublicationContract { tool_id: "increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "amendLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "commitLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
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
struct TestAppCommandJob<const RETAINED: bool, const TOOLS: u8> {
    command: Option<Box<TestCommand>>,
    snapshot: Option<std::sync::Arc<TestSnapshot>>,
    config: Option<std::sync::Arc<TestConfig>>,
    history: Option<std::sync::Arc<HistoryView>>,
    context: Option<std::sync::Arc<ArtifactOwnedToolJobContext<TestApp<RETAINED, TOOLS>>>>,
    completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
    raw: Option<action_bus::RetainedToolWireInput>,
    page: usize,
    closing: bool,
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestAppCommandJob<RETAINED, TOOLS> {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
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
        let emit = test_app_command_emit(self);
        let completion = self.completion.as_ref().expect("exact test app command completion");
        completion.complete(emit, EphemeralEmit::default()).expect("one test app command completion");
        self.command = None;
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(raw) = self.raw.as_mut() {
            if raw.terminal_is_empty() {
                self.raw = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            return raw.close_step(1, maximum_bytes);
        }
        if self.command.take().is_some() || self.snapshot.take().is_some() || self.config.take().is_some() || self.history.take().is_some() || self.context.take().is_some() || self.completion.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw.is_none() && self.command.is_none() && self.snapshot.is_none() && self.config.is_none() && self.history.is_none() && self.context.is_none() && self.completion.is_none()
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
        let job = TestRestartJob { command: Some(request.command), completion: Some(request.completion), raw: None, page: 0, closing: false };
        return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)));
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
        command: Some(request.command),
        snapshot: Some(request.snapshot),
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
    let bytes = law["closeBytes"].as_u64().unwrap() as usize;
    let mut app = VcsArtifactApp::<TestApp<true>>::with_registry(TestApp::<true>::default(), test_restart_registry().await).await;
    let outcome: Result<(u64, u64, u64, u64, u64, i32), Fault> = async {
        app.bind_instance_id(meta.instance_id).await;
        let contracts = app.tool_public_contracts().await;
        let exact = contracts.iter().filter(|contract| contract.tool_id == TEST_RESTART_TOOL && contract.owner == ToolOwnerWitness::of::<TestApp<true>>() && contract.controller_id == TestApp::<true>::APP_ID && contract.schema_id == TEST_RESTART_SCHEMA).count();
        if exact as u64 != law["retainedProofs"].as_u64().unwrap() { return Err(Fault::from("restart app lost its exact registered tool contract")); }
        let admitted = app.dispatch_typed(command, meta).await?;
        if !admitted.mutations.is_empty() { return Err(Fault::from("restart command bypassed retained publication")); }
        let (mut artifact, mut ui, mut scopes, mut terminal, mut completions) = (0, 0, 0, 0, 0);
        for _ in 0..100_000 {
            if let PluginCloseStep::Pending { released_items, released_bytes } = app.maintenance_step(items, bytes)? {
                if released_items > items || released_bytes > bytes { return Err(Fault::from("restart maintenance exceeded the exact grant")); }
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
                eprintln!("[DEBUG] restart typed-operation completion operation={} revision={} history={}", completion.operation, completion.revision, completion.history_patch.is_some());
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
        match app.close_step(items, bytes) {
            Ok(PluginCloseStep::Pending { released_items, released_bytes }) if released_items > items || released_bytes > bytes => {
                close_fault = Some(Fault::from("restart close exceeded the exact grant"));
                break;
            }
            Err(fault) => { close_fault = Some(fault); break; }
            _ => {}
        }
        std::thread::yield_now();
    }
    let closed = app.close_terminal_is_empty();
    eprintln!("[DEBUG] restart retained publication outcome={outcome:?}, closed={closed}, close_fault={close_fault:?}");
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

struct TestRestartJob<const RETAINED: bool, const TOOLS: u8> {
    command: Option<Box<TestCommand>>,
    completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
    raw: Option<action_bus::RetainedToolWireInput>,
    page: usize,
    closing: bool,
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestRestartJob<RETAINED, TOOLS> {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() { return semio_framework_job::StepOutcome::Cancelled; }
        if cx.should_yield() { return semio_framework_job::StepOutcome::Yield; }
        if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return semio_framework_job::StepOutcome::Yield;
        }
        let TestCommand::ApplyCountFromTask { value } = self.command.as_deref().expect("exact restart command") else { panic!("restart owner received another command"); };
        self.completion.as_ref().expect("exact restart completion").complete(Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: *value })])), EphemeralEmit::default()).expect("one restart completion");
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        if let Some(raw) = self.raw.as_mut() {
            if raw.terminal_is_empty() { self.raw = None; return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }; }
            return raw.close_step(1, maximum_bytes);
        }
        if self.command.is_some() {
            let bytes = size_of::<TestCommand>();
            if maximum_bytes < bytes { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
            assert!(matches!(self.command.as_deref(), Some(TestCommand::ApplyCountFromTask { .. })));
            self.command = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if self.completion.take().is_some() { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }; }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.command.is_none() && self.completion.is_none() && self.raw.is_none() }
}

struct TestRestartFactory<const RETAINED: bool, const TOOLS: u8> { keys: Vec<ToolFactoryKey> }

impl<const RETAINED: bool, const TOOLS: u8> ToolJobFactory for TestRestartFactory<RETAINED, TOOLS> {
    type Payload = TestRestartJob<RETAINED, TOOLS>;
    type Job = TestRestartJob<RETAINED, TOOLS>;
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
struct TestRestartTransientDisposer {
    retired: std::mem::ManuallyDrop<Option<store::TransientStore<PublicationTransient, PublicationTransientMutation>>>,
    terminal: Option<(std::sync::Weak<PublicationTransient>, u64)>,
    complete: bool,
}

impl TestRestartTransientDisposer {
    fn new() -> Self { Self { retired: std::mem::ManuallyDrop::new(None), terminal: None, complete: false } }
    fn exact_terminal(&self, owner: &store::TransientStore<PublicationTransient, PublicationTransientMutation>) -> bool {
        self.terminal.as_ref().is_some_and(|(root, generation)| *generation == owner.generation_now() && root.upgrade().is_some_and(|root| std::sync::Arc::ptr_eq(&root, &owner.current_root())))
    }
}

impl ArtifactOwnedDisposer<store::TransientStore<PublicationTransient, PublicationTransientMutation>> for TestRestartTransientDisposer {
    fn close_step(&mut self, owner: &mut store::TransientStore<PublicationTransient, PublicationTransientMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.complete { return self.exact_terminal(owner).then_some(PluginCloseStep::Complete).ok_or_else(|| Fault::from("restart transient terminal owner changed")); }
        if maximum_items == 0 || maximum_bytes < 4_096 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.terminal.is_none() {
            *self.retired = Some(std::mem::replace(owner, store::TransientStore::new(PublicationTransient::default())));
            self.terminal = Some((std::sync::Arc::downgrade(&owner.current_root()), owner.generation_now()));
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
        }
        if !self.exact_terminal(owner) { return Err(Fault::from("restart transient owner changed during retirement")); }
        if self.retired.take().is_some() { return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 }); }
        self.complete = true;
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self, owner: &store::TransientStore<PublicationTransient, PublicationTransientMutation>) -> bool { self.complete && self.retired.is_none() && self.exact_terminal(owner) }
}

impl Drop for TestRestartTransientDisposer {
    fn drop(&mut self) { assert!(self.retired.is_none(), "restart transient owner requires incremental retirement"); }
}

#[test]
fn checkpoint_restart_transient_close_retains_the_exact_store_until_granted() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏳️completion/🔣️.json")).unwrap();
    let law = &fixture["transientClose"];
    let mut owner = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient { revision: 9 });
    let original = std::sync::Arc::downgrade(&owner.current_root());
    let mut close = TestApp::<true>::build_transient_store_disposer().expect("exact restart transient disposer");
    assert_eq!(close.terminal_is_empty(&owner), law["initiallyTerminal"].as_bool().unwrap());
    assert!(size_of::<PublicationTransient>() + size_of_val(&owner) + 2 * size_of::<usize>() <= 4_096);
    assert_eq!(close.close_step(&mut owner, 0, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(close.close_step(&mut owner, 1, law["shortBytes"].as_u64().unwrap() as usize).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(std::sync::Arc::ptr_eq(&original.upgrade().unwrap(), &owner.current_root()));
    let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
        panic!("injected after the original transient store entered its structural retirement owner");
    }));
    assert!(interrupted.is_err());
    assert_eq!(original.upgrade().is_some(), law["originalAliveAfterHandoff"].as_bool().unwrap());
    assert!(!std::sync::Arc::ptr_eq(&original.upgrade().unwrap(), &owner.current_root()));
    assert!(!close.terminal_is_empty(&owner));
    assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
    assert_eq!(original.upgrade().is_some(), law["originalAliveAfterRelease"].as_bool().unwrap());
    assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Complete);
    assert!(close.terminal_is_empty(&owner));
    let mut foreign = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient::default());
    assert_eq!(close.terminal_is_empty(&foreign), law["foreignTerminal"].as_bool().unwrap());
    assert!(close.close_step(&mut foreign, 1, 4_096).is_err());
}
//#endregion 🫧️RestartTransientOwner
