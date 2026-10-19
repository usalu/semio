// 👥️ Interaction, presence and child fixtures: the interaction registry and app, the command page budget, the presence roster, the test children, the close admission fixture and the test task source.

//#region 🗝️RegisteredKeyedDispatchFixture
    /// 🧪️ A registry declaring one `HierarchyProvider::Topology` interaction domain ("items", see
    /// `TestApp::interaction_topology`) — the `🕹️InteractionDispatch` fixture. Auto-injects the six
    /// framework interaction actions via `interaction_action_definitions` (verified by
    /// `build_definition_carries_window_interactions_and_injects_framework_actions` above).
    #[doc(hidden)]
pub async fn interaction_registry() -> AppActionRegistry {
        AppActionRegistry::from_definition(&interaction_app_definition().await)
    }

    /// 🕹️ The synthetic app the interaction laws dispatch against: ONE window kind, ONE declared
    /// domain, and the `window_kind_interactions` ref that binds them — the whole input the framework's
    /// `interaction_declared_refresh_scope` derives a verb's refresh scope from.
    #[doc(hidden)]
pub async fn interaction_app_definition() -> AppDefinition {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), TEST_APP_WINDOW_BODY_KEY, SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .interaction(InteractionDefinition {
                    id: "items".into(),
                    label: LocalizedLabel::data("Items"),
                    granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                    hierarchy: HierarchyProvider::Topology,
                    hover: HoverSpec::default(),
                    selection: SelectionSpec {
                        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                        methods: vec![SelectionMethod::Pick],
                        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                        transitive: false,
                        broadcast: true,
                    },
                })
                .await
                .window_kind_interactions("main", vec![InteractionRef::new("items")])
                .await;
        let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
        app.definition
    }

    #[doc(hidden)]
pub async fn interaction_app_under_test(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractApp {
        contract_app_with(interaction_registry().await, mounted_policy, identity).await
    }

    /// 🕹️ [`interaction_app_under_test`] without the self-closing wrapper — for the local-interaction
    /// laws that move the app into a `PluginRuntime` cell.
    #[doc(hidden)]
pub async fn interaction_app_raw(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp> {
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), interaction_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        app
    }

    /// 🧪️ Builds the JSON `args` an `interactionSelect`/`interactionHover` dispatch carries — the
    /// `targets` arg is itself a JSON-encoded string (an `ActionArgDef::text`, matching
    /// `interaction_action_definitions`'s declaration), not a nested JSON array.
    #[doc(hidden)]
pub fn interaction_target_args(extra: Value, id: &str) -> DslValue {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String(targets)));
        }
        object
    }
    
    /// ⏱️ Turns one command may take to reach its terminal ingress status before the law calls it stuck.
    #[doc(hidden)]
pub const INGRESS_TURNS_PER_COMMAND: usize = 512;
    
    #[doc(hidden)]
pub fn command_page_authority_budget() -> Budget {
        Budget { retained: semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn(), fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
    }

    #[doc(hidden)]
pub async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, PluginAssemblyError> {
        PluginBuilder::<_, TestRuntimeApps>::new("test").label("Synthetic").version("0.0.1").package_id("semio:test").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
    }
    //#endregion 🔖️EphemeralLaneTests

    //#region 🔖️AdoptPresenceTests
    #[doc(hidden)]
pub fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
        PresencePeer {
            actor: actor.to_string(),
            connected_at_ms: 1,
            label: None,
            presence_pack: with_pack.then(|| PublicationPresence::default().encode_pack()),
            user_id: None,
            role: None,
            drag_ghost_json: None,
            interaction: None,
            color,
            surface: None,
            views: Vec::new(),
            ui: None,
            tool_run: None,
            principal_kind: None,
            active_tool: None,
            history_edit: None,
            typing: Vec::new(),
        }
    }

    #[doc(hidden)]
pub async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64,cx:&mut semio_framework_job::StepContext<'_>) -> PresenceRosterOutcome {
        let roster = peers.iter().map(|peer| ::semio_framework_async::poll::resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
        let admission = app.reserve_presence_ingress(seq).expect("reserve roster before decode");
        let first = roster.iter().next().map(|bytes| bytes.to_vec()).unwrap_or_default();
        let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority")).map_err(|(error, _)| error).expect("admit first roster page");
        let generation = admission.generation();
        app.admit_presence_ingress(admission, cursor, now_ms);
        let mut next_page = 1usize;
        for _ in 0..512 {
            app.maintenance.maintenance_stage = 7;
            let _ = app_maintenance_turn(app,cx).expect("bounded roster step");
            if next_page < roster.len() {
                let page = FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
                match app.push_presence_ingress(generation, next_page as u32, page) {
                    Ok(()) => next_page += 1,
                    Err((_fault, _page)) => {}
                }
            }
            if let Some(outcome) = app.take_presence_outcome() {
                return outcome;
            }
        }
        panic!("retained roster did not reach an observable terminal outcome");
    }
    //#endregion 🔖️InteractionViewPeersTests

    store::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
        history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum TestMembers, TestMembersOpen {
            Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer),
        }
    }

    /// 📇️ The APP's half of a composition: declares `child_id` in the parent's one owned-child slot beside every member
    /// it already declares. The runtime admits a member only under the identity its parent's snapshot declares
    /// (`declared_child_reference`), so a law that registers a live member declares it first — exactly as a product
    /// parent's own leaf does before its member is opened.
    #[doc(hidden)]
pub async fn declare_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let mut children: Vec<String> = app.stores.test_snapshot().await.slot.iter().map(|child| child.target.to_uri()).collect();
        children.push(ArtifactRef { artifact_id: child_id.into(), dialect: test_child_dialect().await }.to_uri());
        apply_test_mutations(app.stores.test_store_mut().await, vec![TestMutation::SetSlotChildren(SetSlotChildren { children })]).await.expect("the parent declares the member it is about to own");
    }

    /// 🤝️ [`declare_test_child`], then the RUNTIME's half: a fresh [`new_test_child`] registered as that declared member.
    #[doc(hidden)]
pub async fn register_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        declare_test_child(app, child_id).await;
        app.register_child("slot", child_id, test_child_dialect().await, new_test_child(child_id).await.expect("construct child"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register the declared child");
    }

    /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestMutation>`, wrapped as `TestMembers` —
    /// the shape `VcsArtifactApp::register_child`/`open_child` expect. Built directly (no
    /// runtime `ChildStoreFactory`/`MemberFactory` registration needed — `TestMembers::open`
    /// dispatches by kind at compile time) — the SAME `TestSnapshot`/`TestMutation` pair
    /// `TestApp` itself uses, so a "child" here is just a second, independently-owned instance
    /// of the identical document shape.
    #[doc(hidden)]
pub async fn new_test_child(id: &str) -> Result<TestMembers, store::VcsError> {
        let mut envelope = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
        envelope.dialect = Some(test_child_dialect().await);
        let mut child = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await?;
        store::install_funded_member_store_owners(&mut child,&semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer)?;
        Ok(TestMembers::Child(Box::new(child)))
    }

    /// 🫙️ The owner-less twin of {@link new_test_child}, for the laws whose whole subject is a
    /// member that has NOT yet adopted its bounded retirement authority. Every product path mints a
    /// member through `create_member_store`/`open_member_store`, which install the catalog, so this
    /// shape exists only to drive the refusal those laws assert.
    #[doc(hidden)]
pub async fn new_bare_test_child(id: &str) -> Result<TestMembers, store::VcsError> {
        let mut envelope = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
        envelope.dialect = Some(test_child_dialect().await);
        Ok(TestMembers::Child(Box::new(store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await?)))
    }

    #[doc(hidden)]
pub async fn test_child_dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
    }

    #[doc(hidden)]
pub fn close_member_admission_fixture(member: &mut TestMembers) {
        for _ in 0..65_536 {
            let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
            let demand=member.close_owned_demands(grant.maximum_copy_bytes).expect("actual member close demand");
            assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"member demand exceeds its original fixture close policy");
            let step = member.close_owned_step(grant).expect("actual member original close");
            assert!(step.progress().fits(grant));
            if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                assert!(member.close_owned_terminal_is_empty());
                return;
            }
        }
        panic!("member admission fixture did not close");
    }

    #[doc(hidden)]
pub fn close_member_admission_app<A: ArtifactApp>(app: &mut VcsArtifactApp<A, TestMembers>,cx:&mut semio_framework_job::StepContext<'_>) {
        artifact_app_laws::close_registered_fixture_app(app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    }

    /// 🧪️ One reactor-turn continuation unit, polled exactly the way the guest bridges it: ONCE, with
    /// a no-op waker. A unit that suspends is the `resolve_ready: future was not ready on first poll`
    /// trap that killed the gis actor on its first auto check-in.
    #[doc(hidden)]
pub fn original_publication_turn<P:PluginApp>(app:&mut P,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,caller:&mut semio_framework_job::OriginalJobFixtureCaller,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(),semio_framework_job::OriginalTurnFailure>{
        caller.turn(grant,|original|{
            let mut task=std::task::Context::from_waker(std::task::Waker::noop());
            let mut unit=std::pin::pin!(PluginApp::advance_typed_operation_publication(app,identity,original));
            match std::future::Future::poll(unit.as_mut(),&mut task){std::task::Poll::Ready(result)=>result.map_err(Into::into),std::task::Poll::Pending=>panic!("a reserved commit unit suspended inside its original turn")}
        })
    }

    /// 🧪️ The child's current `count`, read through the same `ChildContentView` seam an app uses.
    #[doc(hidden)]
pub async fn reads_child_count(app: &VcsArtifactApp<TestApp, TestMembers>) -> i32 {
        let view = ChildContentView::clone(&app.child_composition.child_content_root);
        view.typed_read::<TestSnapshot>("slot", "child-1").expect("child readable through the view").count
    }

    /// 🔀️ The exact effect every app's `interaction_select_effect` builds — a guest asking the host
    /// to replay one framework interaction verb.
    #[doc(hidden)]
pub fn inline_interaction_select(args: DslValue) -> Effect {
        Effect::ReplayShellCommand { action_id: INTERACTION_SELECT_ACTION_ID.into(), args: Some(args) }
    }

    /// 🧵️ Test-only typed task source: the closure is the whole captured state and retires as one leaf.
    #[doc(hidden)]
pub struct TestTaskSource<F>(F);

    #[doc(hidden)]
pub struct TestTaskDrop<F>(std::mem::ManuallyDrop<Option<F>>);

    impl<F: Send + 'static> semio_framework_value::retirement::RetirementCursor for TestTaskDrop<F> {
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}

        fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
            use semio_framework_value::retirement::RetirementStep;
            if self.0.is_none() { return RetirementStep::Complete; }
            if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
            drop(self.0.take());
            RetirementStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })
        }
        fn terminal_is_empty(&self) -> bool { self.0.is_none() }
        fn next_work_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
        fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
        fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }
        fn allows_admitted_narrow_work(&self) -> bool { true }
        fn terminal_release_bytes(&self) -> Option<usize> { self.0.is_none().then_some(std::mem::size_of::<Self>()) }
    }

    impl<F: Send + 'static> semio_framework_value::retirement::RetireOwned for TestTaskSource<F> {
        fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { Box::new(TestTaskDrop(std::mem::ManuallyDrop::new(Some(self.0)))) }
        fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<TestTaskDrop<F>>()) }
        fn controlled_retirement_supported() -> bool { true }
    }

    #[doc(hidden)]
pub fn test_task<F, Fut>(label: &str, run: F) -> AsyncTask<TestMutation, TestConfigMutation, NoDraftMutation>
    where
        F: FnOnce(TaskCtx) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<TaskResolution<TestMutation, TestConfigMutation, NoDraftMutation>, Fault>> + 'static,
    {
        AsyncTask::new(label.to_string(), TestTaskSource(run), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).unwrap_or_else(|(error, _, _)| panic!("test task constructor refused: {error}")).0
    }
