//#region 🧵️RestartCommandOwner
const TEST_RESTART_TOOL: &str = "applyCountFromTask";
const TEST_RESTART_SCHEMA: &str = "semio.test.restart-command.v1";

fn test_restart_proofs<const RETAINED: bool>() -> Vec<ArtifactBoundedFirstStepProof> {
    if !RETAINED { return Vec::new(); }
    vec![ArtifactBoundedFirstStepProof::new::<TestApp<RETAINED>>(file!(), TestApp::<RETAINED>::APP_ID, "TestRestartFactory<true>", TEST_RESTART_TOOL, TestApp::<RETAINED>::DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1)).with_factory_type::<TestApp<RETAINED>, TestRestartFactory<RETAINED>>()]
}

fn test_restart_register<const RETAINED: bool>(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, TestApp<RETAINED>>) -> Result<(), Fault> {
    if !RETAINED { return Ok(()); }
    registry.register(TestRestartFactory::<RETAINED> { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), TEST_RESTART_TOOL)] })
}

async fn test_restart_build<const RETAINED: bool>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    if !RETAINED { return Ok(None); }
    assert!(matches!(request.command.as_ref(), TestCommand::ApplyCountFromTask { .. }));
    assert!(request.snapshot.label.is_empty(), "restart fixture begins from its actual fresh snapshot");
    let job = TestRestartJob { command: Some(request.command), completion: Some(request.completion), raw: None, page: 0, closing: false };
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
}

async fn test_restart_registry() -> AppActionRegistry {
    let manifest = App::from_builder(
        App::builder(TestApp::<true>::APP_ID, LocalizedLabel::data("Restart Fixture"))
            .await.document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil").await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow).await
            .app_command(TEST_RESTART_TOOL, LocalizedLabel::data("Apply Count From Task"), "task", ActionKind::Mutation).await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await,
    ).await;
    AppActionRegistry::from_definition(&manifest.definition)
}

async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &serde_json::Value) {
    let law = &fixture["restartAuthority"];
    let items = law["closeItems"].as_u64().unwrap() as usize;
    let bytes = law["closeBytes"].as_u64().unwrap() as usize;
    let mut app = VcsArtifactApp::<TestApp<true>>::with_registry(TestApp::<true>::default(), test_restart_registry().await).await;
    let outcome: Result<(u64, u64, u64, u64, i32), Fault> = async {
        app.bind_instance_id(meta.instance_id).await;
        let contracts = app.tool_public_contracts().await;
        let exact = contracts.iter().filter(|contract| contract.tool_id == TEST_RESTART_TOOL && contract.owner == ToolOwnerWitness::of::<TestApp<true>>() && contract.controller_id == TestApp::<true>::APP_ID && contract.schema_id == TEST_RESTART_SCHEMA).count();
        if exact as u64 != law["retainedProofs"].as_u64().unwrap() { return Err(Fault::from("restart app lost its exact registered tool contract")); }
        let admitted = app.dispatch_typed(command, meta).await?;
        if !admitted.mutations.is_empty() { return Err(Fault::from("restart command bypassed retained publication")); }
        let (mut artifact, mut ui, mut scopes, mut terminal) = (0, 0, 0, 0);
        for _ in 0..100_000 {
            if let PluginCloseStep::Pending { released_items, released_bytes } = app.maintenance_step(items, bytes)? {
                if released_items > items || released_bytes > bytes { return Err(Fault::from("restart maintenance exceeded the exact grant")); }
            }
            app.advance_typed_operation_publication().await?;
            if let Some(page) = app.take_typed_operation_result_page(meta.instance_id) {
                let fault = match page.lane {
                    crate::app::TypedOperationResultLane::Artifact => { artifact += 1; None }
                    crate::app::TypedOperationResultLane::Ui => { ui += 1; None }
                    crate::app::TypedOperationResultLane::Terminal => { terminal += 1; None }
                    crate::app::TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),
                    _ => Some(Fault::from("restart publication produced an undeclared lane")),
                };
                if !app.acknowledge_typed_operation_result(page.token)? { return Err(Fault::from("restart publication rejected its exact result ACK")); }
                if let Some(fault) = fault { return Err(fault); }
            }
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                if !matches!(scope, semio_framework::kernel::UiDirtyScope::Full) { return Err(Fault::from("restart publication changed its declared full UI scope")); }
                scopes += 1;
            }
            if !app.has_pending_typed_operations() { return Ok((artifact, ui, scopes, terminal, app.snapshot()?.count)); }
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
    let (artifact, ui, scopes, terminal, count) = outcome.expect("actual registered restart publication");
    assert_eq!(artifact, law["artifactPublications"].as_u64().unwrap());
    assert_eq!(ui, law["uiPublications"].as_u64().unwrap());
    assert_eq!(scopes, law["uiScopes"].as_u64().unwrap());
    assert_eq!(terminal, law["terminalReceipts"].as_u64().unwrap());
    assert_eq!(i64::from(count), fixture["checkpoint"]["restartValue"].as_i64().unwrap());
}

struct TestRestartJob<const RETAINED: bool> {
    command: Option<Box<TestCommand>>,
    completion: Option<ArtifactToolCompletion<TestApp<RETAINED>>>,
    raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
    page: usize,
    closing: bool,
}

impl<const RETAINED: bool> semio_framework_job::InteractiveJob for TestRestartJob<RETAINED> {
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
            let bytes = std::mem::size_of::<TestCommand>();
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

struct TestRestartFactory<const RETAINED: bool> { keys: Vec<semio_framework::ToolFactoryKey> }

impl<const RETAINED: bool> semio_framework::ToolJobFactory for TestRestartFactory<RETAINED> {
    type Payload = TestRestartJob<RETAINED>;
    type Job = TestRestartJob<RETAINED>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { TEST_RESTART_SCHEMA }
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() { return Err((semio_framework::ToolJobFactoryError::new("restart resume starts a fresh command owner"), input, checkpoint)); }
        payload.raw = Some(input);
        Ok(payload)
    }
}

impl<const RETAINED: bool> ArtifactOwnedToolJobFactory for TestRestartFactory<RETAINED> {
    type Owner = TestApp<RETAINED>;
    const TOOL_IDS: &'static [&'static str] = &[TEST_RESTART_TOOL];
    const DOCUMENT_SCHEMA: &'static str = TestApp::<RETAINED>::DOCUMENT_SCHEMA;
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    let law = &fixture["transientClose"];
    let mut owner = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient { revision: 9 });
    let original = std::sync::Arc::downgrade(&owner.current_root());
    let mut close = TestApp::<true>::build_transient_store_disposer().expect("exact restart transient disposer");
    assert_eq!(close.terminal_is_empty(&owner), law["initiallyTerminal"].as_bool().unwrap());
    assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
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
