/// 🔬️ Items of support module 1 that came from included fixture files.
//#region 🧰️TestAppCommandOwner
/// 🚫️ No app-owned tool factory and no bounded proof row at all — the shape the three fail-closed
/// laws (`unproved_command_fails_before_an_overrun_reducer_can_start`,
/// `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations`,
/// `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command`) need: they assert that an
/// UNPROVED verb is refused and that a non-migrated verb never reaches the live tool bus at all.
#[doc(hidden)]
pub const TEST_APP_TOOLS_NONE: u8 = 0;
/// 🧰️ Owned factories plus the proof rows for the generated tool ids — the migrated fixture every
/// behavioural law uses, paired with `migrated_contract_registry`.
#[doc(hidden)]
pub const TEST_APP_TOOLS_FULL: u8 = 2;

#[doc(hidden)]
pub const TEST_APP_COMMAND_SCHEMA: &str = "semio.test.app-command.v1";
#[doc(hidden)]
pub const TEST_APP_COMMAND_RAW_BYTES: usize = 65_536;

/// 🪪️ Every tool id `TestApp::command_id` can answer — the app's own dispatch surface. A verb
/// missing here has no exact owner-qualified reducer and is refused with
/// `interactive-job.missing-factory` before it reaches `handle`.
#[doc(hidden)]
pub const TEST_APP_COMMAND_TOOL_IDS: &[&str] = &[
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
#[doc(hidden)]
pub const TEST_APP_COMMAND_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
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

#[doc(hidden)]
pub fn test_app_command_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(TEST_APP_COMMAND_RAW_BYTES, 4, 1, TEST_APP_COMMAND_RAW_BYTES, 7_500, 1, 1)
}

/// 🧮️ Runs the app's OWN reducer against the roots this job retained — `TestApp::handle` and this
/// route share `test_app_reduce` verbatim, so a migrated dispatch and a direct one cannot diverge.
#[doc(hidden)]
pub fn test_app_command_emit<const RETAINED: bool, const TOOLS: u8>(job: &TestAppCommandJob<RETAINED, TOOLS>) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
    let command = job.command.as_deref().ok_or_else(|| Fault::from("test-app-command-consumed"))?;
    let snapshot = job.snapshot.as_deref().ok_or_else(|| Fault::from("test-app-command-snapshot-retired"))?;
    let config = job.config.as_deref().ok_or_else(|| Fault::from("test-app-command-config-retired"))?;
    let history = job.history.as_deref().ok_or_else(|| Fault::from("test-app-command-history-retired"))?;
    let children = job.context.as_ref().map_or(ChildContentView::EMPTY, |context| ChildContentView::clone(&context.children));
    let doc = ArtifactView::with_children(snapshot, history, children);
    let cfg = ConfigView { snapshot: config, window: job.context.as_ref().and_then(|context| context.window_config.as_ref()) };
    test_app_reduce(command, &doc, &cfg)
}

pub struct TestAppCommandInputs {
    controller_id:String,
    tool_id:String,
    schema_id:String,
    pub window_config: Option<semio_framework_plugin_window_config::WindowConfigSnapshot>,
    pub interaction_state: semio_framework_value::retirement::arc_custody::SharedControlledRetirement<InteractionState>,
    pub interaction_hover: semio_framework_value::retirement::arc_custody::SharedControlledRetirement<InteractionHoverState>,
    pub instance_operation_owner: semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle,
    pub output_chunks: ArtifactOutputChunks,
}

#[doc(hidden)]
pub struct TestAppCommandJob<const RETAINED: bool, const TOOLS: u8> {
    pub residual: Option<TestAppCommandInputs>,
    pub rejected: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection<TestApp<RETAINED, TOOLS>>>,
    pub admitted: bool,
    pub metadata: Option<AppOperationContext>,
    pub command: Option<Box<TestCommand>>,
    pub snapshot: Option<std::sync::Arc<TestSnapshot>>,
    pub snapshot_read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
    pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub config: Option<std::sync::Arc<TestConfig>>,
    pub history: Option<std::sync::Arc<HistoryView>>,
    pub context: Option<ArtifactOwnedContextHandle<TestApp<RETAINED, TOOLS>>>,
    pub completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
    pub raw: Option<action_bus::RetainedToolWireInput>,
    pub page: usize,
    pub closing: bool,
}

impl<const RETAINED: bool, const TOOLS: u8> TestAppCommandJob<RETAINED, TOOLS> {
    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(raw) = self.raw.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? });
        }
        if let Some(owner) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body); }
        if self.command.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.command); }
        if self.snapshot.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.snapshot); }
        if self.config.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.config); }
        if self.history.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.history); }
        if self.completion.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion); }
        if self.metadata.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.metadata); }
        if self.context.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.context); }
        if self.residual.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.residual); }
        if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture retains original context until its issuing authority is installed")); }
        if self.snapshot_read.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.snapshot_read); }
        Ok(Default::default())
    }
}

impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestAppCommandJob<RETAINED, TOOLS> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.admitted {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
        }
        if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "test app retains its rejected original completion").into()); }
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() {
            return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
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
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "test app original completion was rejected").into());
        }
        self.admitted = true;
        JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step};
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep};
        if !self.closing { return Ok(InteractiveJobCloseStep::Blocked); }
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        if let Some(raw)=self.raw.as_mut() {
            if !raw.terminal_is_empty(){return close_original_retained_step(cx,demand,|grant|raw.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress())));}
            return close_original_retained_step(cx,demand,|_|{self.raw=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))});
        }
        let step=close_original_retained_step(cx,demand,|grant|{
            if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
            else if self.command.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.command,&mut self.retirement,grant)}
            else if self.snapshot.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.snapshot,&mut self.retirement,grant)}
            else if self.config.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.config,&mut self.retirement,grant)}
            else if self.history.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.history,&mut self.retirement,grant)}
            else if self.completion.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.retirement,grant)}
            else if self.metadata.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.metadata,&mut self.retirement,grant)}
            else if self.context.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.context,&mut self.retirement,grant)}
            else if self.residual.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.residual,&mut self.retirement,grant)}
            else if self.snapshot_read.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.snapshot_read,&mut self.retirement,grant)}
            else{Ok(RetainedCloneStep::Complete(Default::default()))}
        })?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.rejected.is_none() && self.residual.is_none() && self.metadata.is_none() && self.retirement.is_none() && self.raw.is_none() && self.command.is_none() && self.snapshot_read.is_none() && self.snapshot.is_none() && self.config.is_none() && self.history.is_none() && self.context.is_none() && self.completion.is_none()
    }
}

#[doc(hidden)]
pub struct TestAppCommandFactory<const RETAINED: bool, const TOOLS: u8> {
    pub keys: Vec<ToolFactoryKey>,
}

impl<const RETAINED: bool, const TOOLS: u8> TestAppCommandFactory<RETAINED, TOOLS> {
    pub fn new(controller_id: &str) -> Self {
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
    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}

}

impl<const RETAINED: bool, const TOOLS: u8> ArtifactOwnedToolJobFactory for TestAppCommandFactory<RETAINED, TOOLS> {
    type Owner = TestApp<RETAINED, TOOLS>;
    const TOOL_IDS: &'static [&'static str] = TEST_APP_COMMAND_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TEST_APP_COMMAND_PUBLICATION_CONTRACTS;
}

//#endregion 🧰️TestAppCommandOwner

#[doc(hidden)]
pub const TEST_RESTART_TOOL: &str = "applyCountFromTask";
#[doc(hidden)]
pub const TEST_RESTART_SCHEMA: &str = "semio.test.restart-command.v1";

#[doc(hidden)]
pub fn test_restart_proofs<const RETAINED: bool, const TOOLS: u8>() -> Vec<ArtifactBoundedFirstStepProof> {
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

#[doc(hidden)]
pub fn test_restart_register<const RETAINED: bool, const TOOLS: u8>(registry: &mut ArtifactToolFactoryRegistry<'_, TestApp<RETAINED, TOOLS>>) -> Result<(), Fault> {
    if RETAINED {
        return registry.register(TestRestartFactory::<RETAINED, TOOLS> { keys: vec![ToolFactoryKey::new(registry.controller_id(), TEST_RESTART_TOOL)] });
    }
    if TOOLS == TEST_APP_TOOLS_NONE {
        return Ok(());
    }
    registry.register(TestAppCommandFactory::<RETAINED, TOOLS>::new(registry.controller_id()))
}

#[doc(hidden)]
pub fn test_restart_build<const RETAINED:bool,const TOOLS:u8>(request:&mut Option<ArtifactOwnedToolJobRequest<TestApp<RETAINED,TOOLS>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original fixture request absent"))?;if RETAINED{assert!(matches!(original.command.as_ref(),TestCommand::ApplyCountFromTask{..}));assert!(original.snapshot.label.is_empty());}else if TOOLS==TEST_APP_TOOLS_NONE||!TEST_APP_COMMAND_TOOL_IDS.contains(&original.tool_id.as_str()){return Ok(false)}test_app_command_job(request,output,cx)}

/// 🧵️ Builds the job on its OWN synchronous frame — every real editor's
/// `ArtifactEditor::build_tool_job` is sync for the same bounded-stack reason.
#[doc(hidden)]
#[inline(never)]
pub fn test_app_command_job<const RETAINED:bool,const TOOLS:u8>(request:&mut Option<ArtifactOwnedToolJobRequest<TestApp<RETAINED,TOOLS>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::app_testing::mutation_fixture::job_close::admit_fixture_tool_job(request,output,cx,|mut request|{
    let job = TestAppCommandJob {
        rejected: None,
        admitted: false,
        residual: Some(TestAppCommandInputs { controller_id:request.controller_id,tool_id:request.tool_id,schema_id:request.payload_schema_id, window_config: request.window_config, interaction_state: semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(request.interaction_state), interaction_hover: semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(request.interaction_hover), instance_operation_owner: request.instance_operation_owner, output_chunks: request.output_chunks }),
        metadata: Some(AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, retained: request.retained, authoring_seed: request.authoring_seed }),
        command: Some(request.command),
        snapshot: Some(request.snapshot),
        snapshot_read: Some(request.snapshot_read),
        retirement: None,
        config: Some(request.config),
        history: Some(request.history),
        context: Some(request.context),
        completion: Some(request.completion),
        raw: request.input.take(),
        page: 0,
        closing: false,
    };

job})}

#[doc(hidden)]
pub struct TestRestartFactory<const RETAINED: bool, const TOOLS: u8> { pub keys: Vec<ToolFactoryKey> }

impl<const RETAINED: bool, const TOOLS: u8> ToolJobFactory for TestRestartFactory<RETAINED, TOOLS> {
    type Payload = TestAppCommandJob<RETAINED, TOOLS>;
    type Job = TestAppCommandJob<RETAINED, TOOLS>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { TEST_RESTART_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}

}

impl<const RETAINED: bool, const TOOLS: u8> ArtifactOwnedToolJobFactory for TestRestartFactory<RETAINED, TOOLS> {
    type Owner = TestApp<RETAINED, TOOLS>;
    const TOOL_IDS: &'static [&'static str] = &[TEST_RESTART_TOOL];
    const DOCUMENT_SCHEMA: &'static str = TestApp::<RETAINED, TOOLS>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TEST_RESTART_TOOL, lanes: &[ArtifactToolPublicationLane::Artifact] }];
}#[doc(hidden)]
pub fn reactor_native_lifecycle_budget() -> Budget {
    Budget { retained: semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn(), fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
}

#[doc(hidden)]
pub fn reactor_native_lifecycle_open(instance: u32, request_sequence: u64, actor: String) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 41, instance_id: instance, request_sequence },
        app_id: AppInstanceId(TestApp::<false>::APP_ID.into()),
        actor, config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: Default::default(),
    }
}

#[doc(hidden)]
pub async fn reactor_native_lifecycle_poll(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, events: Vec<Event>) -> TurnResult {
    for attempt in 0..64 {
        match semio_framework_plugin_reactor_turn::poll_kernel(runtime, events.clone(), None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await {
            Ok(result) => return result,
            Err(fault) if fault.code.0 == "plugin.reactor-turn-deadline" && fault.retryable => {
                eprintln!("retained lifecycle exact-event deadline retry={} events={:?}", attempt + 1, events);
                std::thread::yield_now();
            }
            Err(fault) => panic!("actual native lifecycle turn: {fault:?}"),
        }
    }
    panic!("retained lifecycle could not finish within 64 exact-event retries")
}

#[doc(hidden)]
pub async fn reactor_native_lifecycle_ack(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, receipt: ActorInstanceLifecycleReceipt) {
    reactor_native_lifecycle_poll(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

#[doc(hidden)]
pub async fn reactor_native_lifecycle_finish(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, lifetime: ActorInstanceLifetime, request_sequence: u64) {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt as Receipt, Event};
    let request = ActorInstanceCloseRequest { lifetime, request_sequence };
    let first = reactor_native_lifecycle_poll(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt.expect("accepted native close");
    let Receipt::Accepted { close_generation, .. } = first else { panic!("close must publish Accepted before Retired") };
    assert!(close_generation > 0);
    assert!(semio_framework_plugin_reactor_lifecycle::life_instance_registry::instance_lifetime::plugin_capture_instance_close(runtime, lifetime.instance_id).is_err());
    let duplicate = reactor_native_lifecycle_poll(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt;
    assert_eq!(duplicate, Some(first), "duplicate exact close retains the same native generation");
    reactor_native_lifecycle_ack(runtime, first).await;
    let mut retired = None;
    for _ in 0..16384 {
        let turn = reactor_native_lifecycle_poll(runtime, Vec::new()).await;
        if let Some(receipt @ Receipt::Retired { .. }) = turn.lifecycle_receipt {
            let lifetimes = runtime.guest_lifetimes.borrow();
            let owner = lifetimes.get(lifetime.instance_id).unwrap().cell.owner().unwrap();
            assert!(semio_framework_plugin_reactor_lifecycle::instance_lifetime::GuestLifetimeOwner::terminal_is_empty(owner, &semio_framework_plugin_reactor_core::ReactorClosePeers).unwrap(), "Retired must prove actual native emptiness");
            retired = Some(receipt);
            break;
        }
        std::thread::yield_now();
    }
    let retired = retired.expect("bounded real reactor and native close retirement");
    let Receipt::Retired { close_generation: retired_generation, .. } = retired else { unreachable!() };
    assert_eq!(retired_generation, close_generation);
    assert!(runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_some(), "terminal receipt keeps its native owner");
    for _ in 0..8 {
        reactor_native_lifecycle_ack(runtime, retired).await;
        if runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none() { break; }
    }
    assert!(runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none(), "exact final ACK releases the structural owner");
}

macro_rules! recursive_fixture_snapshot {
    ($snapshot:ty, $extension:literal) => {
        impl pack::snapshot::ArtifactDsl for $snapshot {
            const EXTENSION: &'static str = $extension;
            fn parse_dsl(text: &str) -> Result<Self, TextError> {
                let value = serde_json::from_str::<Value>(text).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;
                <Self as semio_framework_value::FromValue>::from_value(value.into()).map_err(|error| TextError::from_value_error(error, TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String { semio_framework_pack_json::to_json_string(self) }
        }

        impl ArtifactPack for $snapshot {
            fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
            }
            fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let value = serde_json::from_slice::<Value>(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })?;
                <Self as semio_framework_value::FromValue>::from_value(value.into()).map_err(|error| store::PackError::from(error))
            }
        }
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for $snapshot {

}

        impl store::os_store::document_store_owners::MemberStoreProducer<$snapshot,RecursiveFixtureMutation> for RecursiveMemberStoreProducer<$snapshot> {
            type SnapshotOpen = RecursiveBranchSnapshotOpen;
            fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
                recursive_fixture_owners_birth_demand::<$snapshot>()
            }
            fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<$snapshot, RecursiveFixtureMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<$snapshot, RecursiveFixtureMutation>> {
                recursive_fixture_owners::<$snapshot>("recursive-member", grant)
            }
        }
    };
}// 🧾️ `Effect::LoadDocument` for a CHILDLESS single-document app — architect's, animate's and
// writer's exact shape. The composition suite next door only ever drives archives that carry owned
// members; the whole-document swap a `setActiveExample` emits carries `members: []` and a bare
// `store::os_store::text_format::empty_document_spr`, and that lane had no native coverage at all. Every batch-A plugin
// died on it with one opaque sentence (`📓️b1a-dormant-plugin-boots.md`).

/// 🏛️ The app shape every `setActiveExample` plugin has: `bounded_document_store_owners`, a
/// `bounded_document_store_initialization_job` over its own schema, a disposer, a one-item artifact
/// preparation factory, and a snapshot with no `#[child]` field. Delegates every other hook to
/// `TestApp::<true>` so this fixture states only what it is testing.
#[doc(hidden)]
#[derive(Default)]
pub struct SingleDocumentApp;

impl ArtifactApp for SingleDocumentApp {
    const APP_ID: &'static str = "s.test.single-document@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.test.single-document/v1";
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.single-document", standard: StandardId("1"), subset: SubsetId::ANY };

    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = TestCommand;

    async fn initial_snapshot() -> Self::Snapshot {
        TestSnapshot { count: 0, label: "initial".into(), slot: Vec::new() }
    }

    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        Ok(Emit::default())
    }

    /// 📚️ The refresh poll's whole-document replacement, minted the way every app mints one:
    /// `encode_pack` plus a bare `store::os_store::text_format::empty_document_spr` naming the APP, not the live store.
    /// This lane never passed through the runtime's identity stamp.
    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        let pack = <TestSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&TestSnapshot { count: 42, label: "example".into(), slot: Vec::new() });
        let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(Self::APP_ID, Self::DOCUMENT_SCHEMA));
        vec![Effect::LoadDocument { pack, spr }]
    }

    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data("Single-document fixture"))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(test_document_preparation_factory())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> semio_framework_plugin_ownership::store_lifecycle::ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        Ok(bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        TestApp::<false>::build_config_store_disposer()
    }

    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        TestApp::<false>::build_draft_store_disposer()
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        TestApp::<false>::build_presence_local_root_retirement_factory()
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        TestApp::<false>::build_presence_peer_retirement_factory()
    }

    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        TestApp::<false>::build_presence_store_disposer()
    }

    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        TestApp::<false>::build_transient_store_disposer()
    }
}

#[doc(hidden)]
pub type SingleDocumentAppUnderTest = VcsArtifactApp<SingleDocumentApp, TestMembers>;

#[doc(hidden)]
pub async fn drive_single_document_archive(app: &mut SingleDocumentAppUnderTest, operation: u64, cx:&mut semio_framework_job::StepContext<'_>) -> protocol::DocumentArchiveLoadStatus {
    semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(app, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx)).await.expect("single-document archive operation status");
        if matches!(status.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            return status;
        }
        let demand = PluginApp::maintenance_retirement_demands(app, store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("single-document archive maintenance quote");
        let _ = PluginApp::maintenance_step(app,cx, RetainedCloneGrant { maximum_depth: semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance.maximum_depth, ..plugin_demand_grant(demand) }).expect("single-document archive maintenance step");
        semio_framework_async::yield_once().await;
    }
    panic!("single-document archive exceeded its public maintenance progress authority")
}

#[doc(hidden)]
pub fn archive_fault_text(status: &protocol::DocumentArchiveLoadStatus) -> String {
    if status.fault.is_empty() {
        return "<no fault bytes>".to_string();
    }
    semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe()
}