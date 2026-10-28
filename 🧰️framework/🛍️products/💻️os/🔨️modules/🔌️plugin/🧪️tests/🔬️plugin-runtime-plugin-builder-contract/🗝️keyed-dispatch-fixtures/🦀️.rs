// 🗝️ Keyed dispatch fixtures: the keyed test app, its disposer, job, sources and factory, the keyed registry and the native reactor budget.

    //#region 🗝️RegisteredKeyedDispatchFixture
    #[doc(hidden)]
#[derive(Default)]
    pub struct KeyedTestApp;

    /// 🪟️ The keyed fixture's one window body — the carrier's own partial refresh scope names it.
    #[doc(hidden)]
pub const KEYED_WINDOW_BODY_KEY: &str = "synthetic.main";

    /// 🔀️ Exactly what `canvas_gesture::pointer_up` (fem2d) returns for a pick: ONE
    /// `interaction_select_effect` (`Effect::ReplayShellCommand { interactionSelect }`), a partial
    /// window refresh, and NO store lane — the emit whose verb reached the host on the install pump.
    #[doc(hidden)]
pub fn keyed_pick_emit(id: &str) -> Emit<TestMutation, TestConfigMutation> {
        Emit {
            effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), id))],
            ui_scope: keyed_pick_carrier_scope(),
            ..Default::default()
        }
    }

    #[doc(hidden)]
pub fn keyed_pick_carrier_scope() -> UiDirtyScope {
        UiDirtyScope::Partial { window_bodies: vec![KEYED_WINDOW_BODY_KEY.into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    #[doc(hidden)]
pub struct KeyedTestCommandDisposer {
        pub pending: Option<TestCommand>,
        pub active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub moved: bool,
    }

    impl KeyedTestCommandDisposer {
        pub fn new() -> Self { Self { pending: None, active: None, moved: false } }
    }

    impl ArtifactOwnedDisposer<TestCommand> for KeyedTestCommandDisposer {
        fn retirement_demands(&self, _: &TestCommand, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if let Some(active) = self.active.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.pending.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending); }
            Ok(if self.moved { Default::default() } else { semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<TestCommand>(), depth: 1, ..Default::default() } })
        }

        fn close_step(&mut self, command: &mut TestCommand, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, Fault> {
            use semio_framework_value::retained_clone::RetainedCloneProgress;
            if grant.maximum_items == 0 { return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default())); }
            let demand = self.retirement_demands(command, grant.maximum_copy_bytes).map_err(semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault)?;
            if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default())); }
            if !self.moved {
                self.pending = Some(std::mem::replace(command, TestCommand::NoopMutation));
                self.moved = true;
                return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<TestCommand>(), ..Default::default() }));
            }
            let step = if self.active.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant) } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, grant) }.map_err(semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault)?;
            Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::retained(step, self.terminal_is_empty(command)))
        }

        fn terminal_is_empty(&self, command: &TestCommand) -> bool { self.moved && self.pending.is_none() && self.active.is_none() && matches!(command, TestCommand::NoopMutation) }
        fn terminal_frame_release_bytes(&self) -> Option<usize> { (self.moved && self.pending.is_none() && self.active.is_none()).then_some(std::mem::size_of::<Self>()) }
    }

    #[doc(hidden)]
pub struct KeyedTestJobSources {
        controller_id:Option<String>,
        tool_id:Option<String>,
        schema_id:Option<String>,
        pub read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
        pub snapshot: Option<std::sync::Arc<TestSnapshot>>,
        pub config: Option<std::sync::Arc<TestConfig>>,
        pub history: Option<std::sync::Arc<HistoryView>>,
        pub window_config: Option<semio_framework_plugin_window_config::WindowConfigSnapshot>,
        pub interaction: Option<std::sync::Arc<InteractionState>>,
        pub hover: Option<std::sync::Arc<InteractionHoverState>>,
        pub context: Option<ArtifactOwnedContextHandle<KeyedTestApp>>,
        pub instance_owner: Option<semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle>,
        pub output: Option<ArtifactOutputChunks>,
        pub parent_document_id: Option<String>,
        pub authoring_seed: Option<String>,
    }

    impl KeyedTestJobSources {
        /// 🧹️ The next original request input to retire, quoted as the typed retirement lane that owns it: `None` once every input is physically returned.
        pub fn demands(&self) -> Result<Option<semio_framework_value::RetirementDemand>, semio_framework_value::ValueError> {
            macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.$field).map(Some); } }; }
            macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.$field).map(Some); } }; }


            owned!(controller_id);owned!(tool_id);owned!(schema_id);owned!(window_config);owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
            lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
            Ok(None)
        }

        pub fn step(&mut self, active: &mut Option<Box<dyn store::ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<semio_framework_value::retained_clone::RetainedCloneStep>, semio_framework_value::ValueError> {
            macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.$field, active, grant).map(Some); } }; }
            macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.$field, active, grant).map(Some); } }; }

            owned!(controller_id);owned!(tool_id);owned!(schema_id);owned!(window_config);owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
            lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
            Ok(None)
        }
    }

    #[doc(hidden)]
pub struct KeyedTestJob {
        pub sources: Option<KeyedTestJobSources>,
        pub rejected: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection<KeyedTestApp>>,
        pub admitted: bool,
        pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub command: Option<Box<TestCommand>>,
        pub completion: Option<ArtifactToolCompletion<KeyedTestApp>>,
        pub raw: Option<action_bus::RetainedToolWireInput>,
        pub page: usize,
        pub base_count: i32,
        pub closing: bool,
    }

    impl KeyedTestJob {
        pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if let Some(raw) = self.raw.as_ref() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? }); }
            if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.command.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.command); }
            if self.completion.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion); }
            if let Some(sources) = self.sources.as_ref() { return Ok(sources.demands()?.unwrap_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })); }
            if self.rejected.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.rejected); }
            Ok(Default::default())
        }
    }

    impl semio_framework_job::InteractiveJob for KeyedTestJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            if self.admitted { return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "keyed fixture original completion remains rejected").into()); }
            if cx.is_cancelled() {
                return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if cx.should_yield() {
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let Some(now_us) = cx.now_us() else {
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            };
            assert!(cx.deadline_us().saturating_sub(now_us) <= 500, "registered factory deadline must preserve its exact 500us contract");
            if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
                self.page += 1;
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let emit = match self.command.as_deref().unwrap() {
                TestCommand::CompositeEdit { slot, child_id, child_value } => {
                    let child_emits = (!slot.is_empty()).then(|| test_child_emit(slot.clone(), child_id.clone(), &[TestMutation::SetCount(SetCount { value: *child_value })])).into_iter().collect();
                    Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: self.base_count + child_value })], child_emits, ..Default::default() }
                }
                TestCommand::PickItem { id } => keyed_pick_emit(id),
                TestCommand::BulkEdit { rows } => Emit { artifact_mutations: (1..=*rows).map(|row| TestMutation::SetCount(SetCount { value: self.base_count + row })).collect(), ..Default::default() },
                _ => panic!("exact keyed fixture command"),
            };
            if let Err(original) = self.completion.as_ref().unwrap().complete(Ok(emit), EphemeralEmit::default()) {
                self.rejected = Some(original);
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "keyed fixture retains original rejected completion").into());
            }
            self.admitted = true;
            semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
            let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
            let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep};
                if let Some(raw)=self.raw.as_mut(){
                    let step=raw.close_step(grant)?;if raw.terminal_is_empty(){self.raw.take();}
                    return Ok(RetainedCloneStep::Progress(step.progress()))
                }
                if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                else if self.command.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.command,&mut self.retirement,grant)}
                else if self.completion.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.retirement,grant)}
                else if let Some(sources)=self.sources.as_mut(){
                    match sources.step(&mut self.retirement,grant){
                        Ok(Some(step))=>Ok(step),
                        Ok(None)=>{self.sources=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
                        Err(error)=>Err(error),
                    }
                }
                else if self.rejected.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected,&mut self.retirement,grant)}
                else{Ok(RetainedCloneStep::Complete(Default::default()))}
            })?;
            step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.sources.is_none() && self.rejected.is_none() && self.retirement.is_none() && self.raw.is_none() && self.command.is_none() && self.completion.is_none()
        }
    }

    #[doc(hidden)]
pub struct KeyedTestFactory {
        pub keys: Vec<ToolFactoryKey>,
    }

    impl ToolJobFactory for KeyedTestFactory {
        type Payload = KeyedTestJob;
        type Job = KeyedTestJob;
        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }
        fn payload_schema_id(&self) -> &str {
            "semio.test.keyed-command.v1"
        }
        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1)
        }
        fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework_action_bus::RetainedToolWireInput>,checkpoint:Option<&semio_framework_action_bus::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework_action_bus::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework_action_bus::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}

    }

    impl ArtifactOwnedToolJobFactory for KeyedTestFactory {
        type Owner = KeyedTestApp;
        const TOOL_IDS: &'static [&'static str] = &["compositeEdit"];
        const DOCUMENT_SCHEMA: &'static str = KeyedTestApp::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child] }];
        fn latest_wins_target(command: &TestCommand) -> Option<&str> {
            match command {
                TestCommand::CompositeEdit { child_id, .. } => Some(child_id),
                _ => None,
            }
        }
        fn build_latest_wins_command_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<TestCommand>>> {
            Some(Box::new(KeyedTestCommandDisposer::new()))
        }
    }

    //#region 🧹️KeyedNoStateFixtureOwners
    //#endregion 🧹️KeyedNoStateFixtureOwners

    impl ArtifactApp for KeyedTestApp {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.test.keyed", standard: StandardId("1"), subset: SubsetId::ANY };
        const APP_ID: &'static str = "s.test.keyed@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "semio.test/v1";
        type Snapshot = TestSnapshot;
        type Mutation = TestMutation;
        type Config = TestConfig;
        type ConfigMutation = TestConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = NoTransient;
        type TransientMutation = NoTransientMutation;
        type Command = TestCommand;
        semio_framework_plugin_decl_app::bounded_first_step_tool_proofs! {
            owner: KeyedTestApp, owner_file: "plugin/🦀️.rs", controller: "s.test.keyed@1/*#editor", artifact_schema: "semio.test/v1",
            factory: "KeyedTestFactory", factory_type: KeyedTestFactory,
            contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
        }
        fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
            registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
        }
        fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::job_close_fixture::admit_fixture_tool_job(request,output,cx,|mut request|{
            assert!(request.snapshot.label.is_empty());
            let base_count = request.snapshot.count;
            let sources = KeyedTestJobSources { controller_id:Some(request.controller_id),tool_id:Some(request.tool_id),schema_id:Some(request.payload_schema_id), read: Some(request.snapshot_read), snapshot: Some(request.snapshot), config: Some(request.config), history: Some(request.history), window_config: request.window_config, interaction: Some(request.interaction_state), hover: Some(request.interaction_hover), context: Some(request.context), instance_owner: Some(request.instance_operation_owner), output: Some(request.output_chunks), parent_document_id: Some(request.parent_document_id), authoring_seed: Some(request.authoring_seed) };
            let job = KeyedTestJob { command: Some(request.command), completion: Some(request.completion), raw: request.input.take(), page: 0, base_count, closing: false, sources: Some(sources), rejected: None, admitted: false, retirement: None };

job})}
        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            Some(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>())
        }
        fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            TestApp::<false>::build_document_store_disposer()
        }
        fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            TestApp::<false>::build_config_store_disposer()
        }
        fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            TestApp::<false>::build_draft_store_disposer()
        }
        fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_store_disposer())
        }
        fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_store_disposer())
        }
        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_peer_retirement_factory())
        }
        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_local_root_retirement_factory())
        }
        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_local_root_retirement_factory())
        }
        fn reference_birth()->Option<fn()->TestSnapshot>{Some(||{
            TestSnapshot::default()
        })}
        async fn command_id(command: &TestCommand) -> &'static str {
            TestApp::<false>::command_id(command).await
        }
        async fn handle(
            _command: &TestCommand,
            _doc: &ArtifactView<'_, TestSnapshot>,
            _cfg: &ConfigView<'_, TestConfig>,
            _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            _draft: &DraftView<'_, NoDraft>,
            _engines: &semio_framework_2d_compute::EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            Err(Fault::from("keyed fixture requires its actual retained factory"))
        }
        async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            TestApp::<false>::render(body, doc, cfg, _view_state).await
        }
    }

    #[doc(hidden)]
pub async fn keyed_test_registry() -> AppActionRegistry {
        let manifest = App::from_builder(
            App::builder(KeyedTestApp::APP_ID, LocalizedLabel::data("Keyed Fixture"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .app_command("compositeEdit", LocalizedLabel::data("Set Parameter"), "fixture", ActionKind::Mutation)
                .await
                .interactive_jobs(InteractiveJobClassification::Migrated)
                .await,
        )
        .await;
        AppActionRegistry::from_definition(&manifest.definition)
    }

    /// ⛽️ The per-turn ceiling a browser turn actually carries — `DEFAULT_SHARD_BUDGET` lifted through
    /// `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`'s `poll`, which stamps `maxFrames: 8` — so a law that
    /// drives the continuation reads the exact grant production does, not a generous test one.
    #[doc(hidden)]
pub fn reactor_native_budget(cx:&semio_framework_job::StepContext<'_>) -> semio_framework::kernel::Budget {
        semio_framework::kernel::Budget { retained: cx.retained_grant(), fuel: cx.fuel_remaining(), deadline_us: cx.deadline_us(), max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 8 }
    }
//#endregion 🗝️RegisteredKeyedDispatchFixture
