//#region 🧵️RestartCommandOwner
const TEST_RESTART_TOOL: &str = "applyCountFromTask";
const TEST_RESTART_SCHEMA: &str = "semio.test.restart-command.v1";

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
    let mut owner = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient { revision: 9 });
    let original = std::sync::Arc::downgrade(&owner.current_root());
    let mut close = TestRestartTransientDisposer::new();
    assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    assert_eq!(close.close_step(&mut owner, 0, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(close.close_step(&mut owner, 1, 4_095).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(std::sync::Arc::ptr_eq(&original.upgrade().unwrap(), &owner.current_root()));
    assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
    assert!(original.upgrade().is_some());
    assert!(!std::sync::Arc::ptr_eq(&original.upgrade().unwrap(), &owner.current_root()));
    assert!(!close.terminal_is_empty(&owner));
    assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
    assert!(original.upgrade().is_none());
    assert_eq!(close.close_step(&mut owner, 1, 4_096).unwrap(), PluginCloseStep::Complete);
    assert!(close.terminal_is_empty(&owner));
    let mut foreign = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient::default());
    assert!(!close.terminal_is_empty(&foreign));
    assert!(close.close_step(&mut foreign, 1, 4_096).is_err());
}
//#endregion 🫧️RestartTransientOwner
