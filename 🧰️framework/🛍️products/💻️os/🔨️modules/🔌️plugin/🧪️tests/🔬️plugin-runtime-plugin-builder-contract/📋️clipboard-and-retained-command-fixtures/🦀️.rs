// 📋️ Clipboard and retained command fixtures: the clipboard job, the retained command work, factory and payload, the command ids and the reducer of the test app.

    //#region 📋️ContractClipboardFixture
    /// 📋️ The exact reserved clipboard producer of the contract app: `copy` and `cut` author the clipboard write, `paste` the placed label, each through its one completion cell, and every original request owner retires under paid turns.
    #[doc(hidden)]
pub struct TestClipboardJob<const RETAINED: bool, const TOOLS: u8> {
        pub tool: String,
        pub label: String,
        pub args: Option<DslValue>,
        pub read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
        pub completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
        pub rejected: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection<TestApp<RETAINED, TOOLS>>>,
        pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub outcomes: semio_framework_plugin_decl_app::tool_contract::ReservedJobOutcomes,
        pub completed: bool,
        pub closing: bool,
    }

    impl<const RETAINED: bool, const TOOLS: u8> TestClipboardJob<RETAINED, TOOLS> {
        pub fn new(request: semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Self, Fault> {
            let semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolInput::Action { args, .. } = request.input else { return Err(Fault::from("the contract clipboard requires an action input")) };
            Ok(Self { tool: request.tool_id, label: request.snapshot.label.clone(), args, read: Some(request.snapshot_read), completion: Some(request.completion), rejected: None, retirement: None, outcomes: semio_framework_plugin_decl_app::tool_contract::ReservedJobOutcomes::new(), completed: false, closing: false })
        }

        pub fn emit(&self) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
            let media_type = MediaType { class: MediaClass::Data, form: MediaForm::Value };
            if self.tool != "paste" {
                if self.label.is_empty() {
                    return Ok(Emit::default());
                }
                let fragment = ClipboardFragment { schema: TestApp::<false>::DOCUMENT_SCHEMA.to_string(), media_type, dsl_text: self.label.clone(), pack_bytes: None, source_app: TestApp::<false>::APP_ID.to_string(), label: self.label.clone() };
                let mut emit = Emit::effect(Effect::ClipboardWrite { fragment });
                if self.tool == "cut" {
                    emit.artifact_mutations = vec![TestMutation::SetLabel(SetLabel { value: String::new() })];
                }
                return Ok(emit);
            }
            let Some(fragment) = self.args.as_ref().and_then(|args| args.get("fragment")) else { return Ok(Emit::default()) };
            let text = fragment.get("dslText").and_then(DslValue::as_str).ok_or_else(|| Fault::from("the contract paste fragment carries no text"))?;
            let anchor = match self.args.as_ref().and_then(|args| args.get("anchor")).and_then(DslValue::as_str) {
                None | Some("original") => PasteAnchor::Original,
                Some("middle") => PasteAnchor::Middle,
                Some("centroid") => PasteAnchor::Centroid,
                Some("bottomLeft") => PasteAnchor::BottomLeft,
                Some("bottomRight") => PasteAnchor::BottomRight,
                Some("topLeft") => PasteAnchor::TopLeft,
                Some("topRight") => PasteAnchor::TopRight,
                Some(other) => return Err(Fault::from(format!("the contract paste anchor '{other}' is undeclared"))),
            };
            let value = if anchor == PasteAnchor::Original { text.to_string() } else { format!("{text}-{anchor:?}") };
            Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value })]))
        }

        pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if !self.outcomes.terminal_is_empty() { return self.outcomes.close_demands(); }
            if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.rejected.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.rejected); }
            if self.read.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.read); }
            if self.completion.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion); }
            Ok(Default::default())
        }
    }

    impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestClipboardJob<RETAINED, TOOLS> {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            if cx.is_cancelled() { return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.outcomes.is_delivered() {
                self.outcomes.retire(cx)?;
                return Ok(None);
            }
            if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard completion remains rejected").into()); }
            if cx.should_yield() { return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if !self.completed {
                let emit = self.emit().map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, fault.message))?;
                let completion = self.completion.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard lost its completion authority"))?;
                if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                    self.rejected = Some(rejected);
                    return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard completion was rejected").into());
                }
                self.completed = true;
            }
            semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
            self.outcomes.borrow_outcome(descriptor)
        }

        fn begin_close(&mut self) { self.closing = true; }

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
            let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
            let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}
                if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                else if self.rejected.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected,&mut self.retirement,grant)}
                else if self.read.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.read,&mut self.retirement,grant)}
                else if self.completion.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.retirement,grant)}
                else{Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))}
            })?;
            step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.read.is_none() && self.completion.is_none() && self.rejected.is_none() && self.retirement.is_none() && self.outcomes.terminal_is_empty()
        }
    }

    impl<const RETAINED: bool, const TOOLS: u8> semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob for TestClipboardJob<RETAINED, TOOLS> {}
    //#endregion 📋️ContractClipboardFixture

    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_CONTROLLER: &str = "s.test.synthetic@1/*#editor";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_TOOL: &str = "setLabel";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_SCHEMA: &str = "semio.test.retained-command.v1";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_RAW_BYTES: usize = 4_096;
    #[doc(hidden)]
pub static TEST_RETAINED_COMMAND_STEP_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    #[doc(hidden)]
pub struct TestRetainedCommandWork {
        pub cursor: u8,
    }

    impl semio_framework_plugin_retained_command::ArtifactCommandWork<TestApp> for TestRetainedCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(3)
        }

        fn step(&mut self, input: &semio_framework_plugin_retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_retained_command::ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
            let semio_framework_plugin_retained_command::ArtifactCommandInputs { snapshot_owner: _, snapshot_read: _, command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            TEST_RETAINED_COMMAND_STEP_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.cursor < 2 {
                self.cursor += 1;
                return Ok(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Progress { stage: "test-retained-command-work", preview: br#"{"en":"Applying","de":"Anwenden"}"# });
            }
            Ok(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Complete(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })])))
        }

        fn checkpoint_byte(&self, index: usize) -> Option<u8> {
            (index == 0).then_some(self.cursor)
        }

        fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
            let [cursor] = checkpoint else { return Err(Fault::from("test-retained-command-checkpoint-invalid")) };
            if *cursor > 2 {
                return Err(Fault::from("test-retained-command-checkpoint-cursor"));
            }
            self.cursor = *cursor;
            Ok(())
        }
    }

    #[doc(hidden)]
pub struct TestRetainedCommandFactory {
        pub keys: Vec<ToolFactoryKey>,
    }

    impl TestRetainedCommandFactory {
        pub fn new() -> Self {
            Self { keys: vec![ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
        }
    }

    impl ArtifactOwnedToolJobFactory for TestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TEST_RETAINED_COMMAND_TOOL, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }

    impl ToolJobFactory for TestRetainedCommandFactory {
        type Payload = semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            TEST_RETAINED_COMMAND_SCHEMA
        }

        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
        }

        semio_framework_plugin_retained_command::retained_command_factory_admission!(maximum_raw_bytes:TEST_RETAINED_COMMAND_RAW_BYTES,checkpoint_resume:true);
    }

    #[doc(hidden)]
pub fn test_retained_command_id(command: &TestCommand) -> &'static str {
        match command {
            TestCommand::SetLabel { .. } => TEST_RETAINED_COMMAND_TOOL,
            _ => "unsupported",
        }
    }

    #[doc(hidden)]
pub const TEST_COMMAND_RETAINED_GRANT: semio_framework_value::RetainedCloneGrant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };

    #[doc(hidden)]
pub  async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> (semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>,crate::job_close_fixture::FixtureSnapshotOwner) {
        let snapshot=std::sync::Arc::new(TestSnapshot::default());
        let(snapshot_read,issuer)=crate::job_close_fixture::FixtureSnapshotOwner::new(snapshot.clone(),3,[5;32]);
        let payload=semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin_retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot,
                snapshot_read,
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32], retained: TEST_COMMAND_RETAINED_GRANT, authoring_seed: "authoring-seed-test".into() },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            3,
            Box::new(TestRetainedCommandWork { cursor: 0 }),
        );(payload,issuer)
    }


    /// 🌊️ The one tool transaction every `streamLabel` tick streams into and its `commit: true` tick closes (design §15).
    #[doc(hidden)]
pub const TEST_STREAM_LABEL_TRANSACTION: &str = "tx-00000000000005ab";

    /// 🪪️ `TestApp`'s verb for one command variant, as a plain `fn` so the retained
    /// command payload (`ArtifactRetainedCommandPayload::command_id`) and the async
    /// `ArtifactApp::command_id` answer the same id from ONE table.
    ///
    /// 🔀️ Rides the keyed fixture's ONE generated tool id (`TOOL_JOB_IDS` is fixture-checked):
    /// a pick and a composite edit are two commands of the same typed tool.
    #[doc(hidden)]
pub fn test_command_id(command: &TestCommand) -> &'static str {
        match command {
            TestCommand::Increment => "increment",
            TestCommand::SetLabel { .. } => "setLabel",
            TestCommand::StreamLabel { .. } => "streamLabel",
            TestCommand::BadView => "badView",
            TestCommand::Select { .. } => "select",
            TestCommand::Navigate => "navigate",
            TestCommand::NoopMutation => "noopMutation",
            TestCommand::ViewNoScope => "viewNoScope",
            TestCommand::ViewPartialScope => "viewPartialScope",
            TestCommand::IncrementViaCommand => "incrementViaCommand",
            TestCommand::WatchdogOverrun => "watchdogOverrun",
            TestCommand::SetLabelViaCommand { .. } => "setLabelViaCommand",
            TestCommand::SetActiveUtility { .. } => "setActiveUtility",
            TestCommand::TargetWindow { .. } => "targetWindow",
            TestCommand::ModeIncrement => "mode.increment",
            TestCommand::CompositeEdit { .. } => "compositeEdit",
            TestCommand::ProbeChild { .. } => "probeChild",
            TestCommand::SpawnCountTask => "spawnCountTask",
            TestCommand::ApplyCountFromTask { .. } => "applyCountFromTask",
            TestCommand::PickItem { .. } => "compositeEdit",
            TestCommand::BulkEdit { .. } => "compositeEdit",
        }
    }

    /// 🧮️ `TestApp`'s reducer body, shared verbatim by `ArtifactApp::handle` (host route) and
    /// `test_app_command_reduce` (the app-owned retained tool route) so a migrated dispatch
    /// and a direct one can never diverge.
    ///
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: no mutations of its own — the
    /// task's eventual `TaskResolution::Command` follow-up is what mutates the
    /// document, on a LATER dispatch (see `ApplyCountFromTask` below).
    #[doc(hidden)]
pub fn test_app_reduce(command: &TestCommand, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
        match command {
            TestCommand::Increment | TestCommand::IncrementViaCommand | TestCommand::ModeIncrement => Ok(Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: doc.snapshot.count + 1 })], ..Default::default() }),
            TestCommand::TargetWindow { window_id } => Ok(Emit { artifact_mutations: vec![TestMutation::SetLabel(SetLabel { value: window_id.clone() })], ..Default::default() }),
            TestCommand::WatchdogOverrun => {
                let started = std::time::Instant::now();
                while started.elapsed() < std::time::Duration::from_millis(10) {
                    std::hint::spin_loop();
                }
                Ok(Emit::default())
            }
            TestCommand::SetLabel { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })])),
            TestCommand::SetLabelViaCommand { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })])),
            TestCommand::StreamLabel { value, commit } => {
                let transaction = protocol::TransactionRef { id: TEST_STREAM_LABEL_TRANSACTION.into(), tool: format!("{}#streamLabel", TestApp::<false>::APP_ID) };
                let mutations = vec![TestMutation::SetLabel(SetLabel { value: value.clone() })];
                Ok(if *commit { Emit::commit_transaction(transaction, mutations) } else { Emit::stream_transaction(transaction, mutations) })
            }
            TestCommand::BadView => Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: 99 })])),
            TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: json!({ "utilityId": utility_id.clone() }).into() })),
            TestCommand::Select { id } => Ok(Emit::config(vec![ChangeTestConfigSelection { selected: id.clone() }.into()])),
            TestCommand::Navigate => Ok(Emit::effect(Effect::Navigate { uri: "semio://home".into() })),
            TestCommand::NoopMutation => Ok(Emit::default()),
            TestCommand::ViewNoScope => Ok(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }),
            TestCommand::ViewPartialScope => {
                Ok(Emit { ui_scope: UiDirtyScope::Partial { window_bodies: vec!["some.window".into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }, ..Default::default() })
            }
            TestCommand::CompositeEdit { slot, child_id, child_value } => Ok(Emit {
                artifact_mutations: vec![TestMutation::SetLabel(SetLabel { value: "composite".into() })],
                child_preparations:std::collections::VecDeque::from([semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation::of::<TestSnapshot,_>(slot.clone(),child_id.clone(),vec![TestMutation::SetCount(SetCount{value:*child_value})])]),
                ..Default::default()
            }),
            TestCommand::ProbeChild { slot, child_id } => {
                let _snapshot = doc.children.typed_read::<TestSnapshot>(slot, child_id)?;
                Ok(Emit::effect(Effect::DispatchAction { req: RequestId(91_001), action: "probeChildContinuation".into(), args: None, delay_ms: 0 }))
            }
            TestCommand::SpawnCountTask => Ok(Emit::task(AsyncTask::new("spawn-count-task".into(),SpawnCountTask,semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256}).unwrap().0)),
            TestCommand::ApplyCountFromTask { value } => Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: *value })])),
            TestCommand::PickItem { id } => Ok(keyed_pick_emit(id)),
            TestCommand::BulkEdit { rows } => Ok(Emit { artifact_mutations: (1..=*rows).map(|row| TestMutation::SetCount(SetCount { value: row })).collect(), ..Default::default() }),
        }
    }

    #[doc(hidden)]
pub fn test_prepared_operation_wire_source(operation: &TestMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let (_, ordinal, spec) = semio_framework_dsl_record::BorrowedDslVariants::projected_borrowed_variant_identity(operation);
        let tag = u64::try_from(ordinal).ok()?;
        Some(match operation {
            TestMutation::SetCount(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
            TestMutation::SetLabel(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
            TestMutation::SetSlotChildren(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
        })
    }

    #[doc(hidden)]
pub fn test_document_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<TestSnapshot, TestMutation>> {
        store::operation_wire_preparation_factory(store::cold_mutation_apply_preparation_factory::<TestSnapshot, TestMutation>(), test_prepared_operation_wire_source, |operation| {
            let semantics = protocol::SemanticMutation::semantics(operation);
            Some((semantics.entity, semantics.kind))
        })
    }
