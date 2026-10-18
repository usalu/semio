/// 🏭️ Tool factory keys and shared actions: the retained command factory wrapper, the copy draw app and the exact bijection between activated tool factory keys, migrated actions and the shared framework action route.

    struct OtherTestRetainedCommandFactory(TestRetainedCommandFactory);

    impl ToolJobFactory for OtherTestRetainedCommandFactory {
        type Payload = semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>;
        fn keys(&self) -> &[ToolFactoryKey] {
            &self.0.keys
        }
        fn payload_schema_id(&self) -> &str {
            TEST_RETAINED_COMMAND_SCHEMA
        }
        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolJobFactory::execution_contract(&self.0)
        }
        fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            ToolJobFactory::create_job(&mut self.0, operation, payload)
        }
    }

    impl ArtifactOwnedToolJobFactory for OtherTestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TestRetainedCommandFactory::PUBLICATION_CONTRACTS;
    }

    /// 🧬️ A hostile structural copy of Draw's public controller/schema/tool constants. Its
    /// concrete Rust type is deliberately not the audited Draw owner type.
    #[derive(Default)]
    struct CopyDrawApp;

    impl ArtifactApp for CopyDrawApp {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.draw.draw", standard: StandardId("1"), subset: SubsetId::ANY };
        const APP_ID: &'static str = "s.draw.draw@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "draw.document";
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

        async fn initial_snapshot() -> TestSnapshot {
            TestSnapshot::default()
        }

        async fn command_id(_command: &TestCommand) -> &'static str {
            "canvasPointerDown"
        }

        async fn handle(
            command: &TestCommand,
            doc: &ArtifactView<'_, TestSnapshot>,
            cfg: &ConfigView<'_, TestConfig>,
            interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            draft: &DraftView<'_, NoDraft>,
            engines: &semio_framework_2d_compute::EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            TestApp::<false>::handle(command, doc, cfg, interaction, _view_state, draft, engines).await
        }

        async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            TestApp::<false>::render(body_key, doc, cfg, _view_state).await
        }
    }

    /// 🧵️ Source text of the mounted artifact instance: its module tree, the parent file followed by every staged impl slice in declaration order, read at run time from the manifest folder of the hosting crate.
    #[doc(hidden)]
pub fn instance_source() -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&fixture_path!("../../../../🧵️instance/🦀️.rs")).expect("the instance module tree reads")
    }

    /// 🧭️ The bijection has exactly two named residues, and both are asserted as exact sets so a
    /// drift in either direction fails on the clause that names its cause.
    ///
    /// 1. Framework-reserved SURFACE verbs the framework registers a factory for unconditionally
    ///    (`register_framework_reserved_tool_factories`) while this fixture's manifest declares
    ///    none of them — it has no interaction topology and no window kit to mint them. A verb
    ///    here is registered but undeclared, so `require_ui_safe_declaration` refuses it by name
    ///    before its factory is ever reached.
    /// 2. Migrated verbs the framework serves without a tool factory, so a missing registration
    ///    is not the dead-action defect it would be for any other migrated verb: `setActiveUtility`
    ///    (injected whenever the app declares utilities; `handle_action_invocation` routes it
    ///    DIRECTLY to `dispatch_emit`) and `cancelTypedOperation` (every app's operation-progress
    ///    cancel; the head of `dispatch_action` routes it DIRECTLY to `dispatch_operation_cancellation`).
    ///
    /// 🚦️ `not-ui-safe`, not `missing-factory`: `contract_registry` declares every verb
    /// `BatchOnlyPendingRewrite`, and the UI-safety backstop is read from the manifest declaration
    /// at the HEAD of `admit_command_wire`, before any proof is resolved. A non-migrated verb has
    /// no factory precisely BECAUSE it is not migrated, so `missing-factory` was the consequence,
    /// not the cause — the clause above still proves there is no factory to find.
    #[semio_framework_async_macros::async_test]
    async fn activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let declared = registry.test_migrated_tool_ids();
        let controller_id = registry.test_controller_id().to_string();
        let mut app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry_on_bus(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), registry.clone(), platform.action_bus.clone(), protocol::ActorId(LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        let registered: std::collections::BTreeSet<String> = app.tools.test_registered_tool_keys()
            .into_iter()
            .map(|key| {
                assert_eq!(key.0, controller_id);
                key.1
            })
            .collect();
        let framework_registered_without_declaration: std::collections::BTreeSet<String> =
            ["clearSelection", "configuration-binary", "import-media", "interactionHover", "interactionSelect", "selectAll", "setInteractionGranularity", "setSelectionMode"].into_iter().map(String::from).collect();
        let framework_directly_routed_migrated: std::collections::BTreeSet<String> = ["cancelTypedOperation", "setActiveUtility", semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, semio_framework::HOST_EVENT_ACTION_ID]
            .into_iter()
            .chain(semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS)
            .map(String::from)
            .collect();
        assert_eq!(registered.difference(&declared).cloned().collect::<std::collections::BTreeSet<_>>(), framework_registered_without_declaration, "an activated factory without a manifest declaration must be one of the framework's own reserved surface verbs");
        assert_eq!(declared.difference(&registered).cloned().collect::<std::collections::BTreeSet<_>>(), framework_directly_routed_migrated, "a migrated declaration with no activated factory is a dead action unless the framework routes it directly");
        assert!(
            instance_source().contains("} else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {"),
            "the only reason `setActiveUtility` may carry no factory is its direct `dispatch_emit` arm in `handle_action_invocation`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifaction==CANCEL_TYPED_OPERATION_ACTION_ID{returnself.dispatch_operation_cancellation(args,meta).await;}"),
            "the only reason `cancelTypedOperation` may carry no factory is its direct `dispatch_operation_cancellation` arm at the head of `dispatch_action`"
        );
        let runtime_source: String = instance_source().split_whitespace().collect();
        assert!(
            runtime_source.contains("ifInstanceExtensions::claims_action(action){returnBox::pin(InstanceExtensions::dispatch_action(self,action,args,meta,identity,cx)).await"),
            "the only reason the history-edit verbs may carry no factory is their boxed instance-extension route at the head of `dispatch_action`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifaction==semio_framework::HOST_EVENT_ACTION_ID{returnBox::pin(self.dispatch_host_event_action(args,meta,identity)).await;}"),
            "the only reason `hostEvent` may carry no factory is its host-driven, boxed arm at the head of `dispatch_action`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifmatches!(action,semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID|semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID){returnErr(Fault::new(FaultOrigin::Framework,FaultCode::new(\"framework.document-transfer.shell-owned\")"),
            "the only reason the document transfer verbs may carry no factory is their shell-owned refusal at the head of `dispatch_action`"
        );
        let platform_visible = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(platform_visible, registered, "every activated factory key is joined on the platform bus under this controller, and nothing else is");
        let before = platform.action_bus.dispatch_count();
        let verb = <TestApp<false, TEST_APP_TOOLS_NONE> as ArtifactApp>::command_id(&TestCommand::IncrementViaCommand).await;
        assert!(!registered.contains(verb), "the unproved verb has no activated factory of its own");
        assert!(!declared.contains(verb), "and this registry does not declare it migrated either");
        let error = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity).await.expect_err("an unproved typed command must remain fail closed");
        assert_eq!(error.code.0, "interactive-job.not-ui-safe");
        assert_eq!(platform.action_bus.dispatch_count(), before);
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    //#region 🧪️SharedFrameworkActionRouteTests
    /// 🪪️ Every framework-reserved factory is parameterized by its OWNING app type, so the
    /// identity this law joins must name the exact owner the wrapper above was built on — a bare
    /// `TestApp` is `TestApp<false, TEST_APP_TOOLS_FULL>`, a DIFFERENT owner and a different TypeId.
    ///
    /// 📏️ The last column is `ToolExecutionContract::max_raw_wire_bytes`, i.e. the `$raw` column of
    /// this verb's own `framework_reserved_job!(..)` line — NOT its output cap. `copy`/`cut` carry
    /// an 8 KiB raw wire and a 1 MiB OUTPUT; recording the output cap here made the law demand a
    /// 1 MiB wire admission that the bus has never granted them.
    #[semio_framework_async_macros::async_test]
    async fn shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let controller_id = registry.test_controller_id().to_string();
        let app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry_on_bus(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), registry, platform.action_bus.clone(), protocol::ActorId(LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        type UnprovedFrameworkOwner = TestApp<false, TEST_APP_TOOLS_NONE>;
        let expected: [(&str, &str, std::any::TypeId, &'static str, usize); 12] = [
            ("copy", "framework.reserved.copy.v1", std::any::TypeId::of::<FrameworkCopyJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkCopyJobFactory<UnprovedFrameworkOwner>>(), 8_192),
            ("cut", "framework.reserved.cut.v1", std::any::TypeId::of::<FrameworkCutJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkCutJobFactory<UnprovedFrameworkOwner>>(), 8_192),
            ("paste", "framework.reserved.paste.v1", std::any::TypeId::of::<FrameworkPasteJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkPasteJobFactory<UnprovedFrameworkOwner>>(), 1_048_576),
            ("noteShellCommand", "framework.reserved.noteShellCommand.v1", std::any::TypeId::of::<FrameworkNoteShellCommandJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkNoteShellCommandJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            (
                "setHistoryCommandFilter",
                "framework.reserved.setHistoryCommandFilter.v1",
                std::any::TypeId::of::<FrameworkSetHistoryCommandFilterJobFactory<UnprovedFrameworkOwner>>(),
                std::any::type_name::<FrameworkSetHistoryCommandFilterJobFactory<UnprovedFrameworkOwner>>(),
                4_096,
            ),
            ("recordTutorial", "framework.reserved.recordTutorial.v1", std::any::TypeId::of::<FrameworkRecordTutorialJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkRecordTutorialJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            ("clearSelection", "framework.reserved.clearSelection.v1", std::any::TypeId::of::<FrameworkClearSelectionJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkClearSelectionJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            ("interactionHover", "framework.reserved.interactionHover.v1", std::any::TypeId::of::<FrameworkInteractionHoverJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkInteractionHoverJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            ("interactionSelect", "framework.reserved.interactionSelect.v1", std::any::TypeId::of::<FrameworkInteractionSelectJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkInteractionSelectJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            ("selectAll", "framework.reserved.selectAll.v1", std::any::TypeId::of::<FrameworkSelectAllJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkSelectAllJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            (
                "setInteractionGranularity",
                "framework.reserved.setInteractionGranularity.v1",
                std::any::TypeId::of::<FrameworkSetInteractionGranularityJobFactory<UnprovedFrameworkOwner>>(),
                std::any::type_name::<FrameworkSetInteractionGranularityJobFactory<UnprovedFrameworkOwner>>(),
                16_384,
            ),
            ("setSelectionMode", "framework.reserved.setSelectionMode.v1", std::any::TypeId::of::<FrameworkSetSelectionModeJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkSetSelectionModeJobFactory<UnprovedFrameworkOwner>>(), 16_384),
        ];
        let joined = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        let registrations = app.tools.test_framework_tool_registrations();
        for (tool_id, schema_id, factory_type_id, factory_type_name, maximum) in expected {
            let (key, registered_schema_id, registered_factory_type_id, registered_factory_type_name, contract) = registrations.get(tool_id).expect("shared framework registration");
            assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
            assert_eq!(registered_schema_id, schema_id);
            assert_eq!(*registered_factory_type_id, factory_type_id);
            assert_eq!(*registered_factory_type_name, factory_type_name);
            assert_eq!(contract.max_raw_wire_bytes, maximum);
            assert!(joined.contains(tool_id));
            let exact = vec![0_u8; maximum];
            let admission = platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &exact).expect("exact maximum admission");
            assert_eq!(admission.factory_type_id, factory_type_id);
            assert_eq!(admission.factory_type_name, factory_type_name);
            assert!(platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &vec![0_u8; maximum + 1]).is_err());
        }
        let mut app = app;
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    //#endregion 🧪️SharedFrameworkActionRouteTests
