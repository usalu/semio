/// ↩️ View action, undo and history delivery: view and shell actions without operations, reserved undo within the host drive contract, isolated spawn jobs and the history delivery scopes.

    /// 👁️ A view command never advances the document.
    #[semio_framework_async_macros::async_test]
    async fn view_action_emits_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_action_emits_host_effect_without_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Navigate, &meta(), &mut identity,cx).await.expect("navigate");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }]);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 #38 host-drive contract: spawn-admit undo reaches Done in 2 Isolated steps with the browser budget.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_reaches_done_within_host_drive_contract() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
         &mut identity, cx)
        .await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
         &mut identity, cx)
        .await;
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        let (settled, steps) = semio_framework_plugin_instance_app::plugin_app_impl::drive_framework_reserved_spawn_like_host(&mut app, admitted).await.expect("host-like Isolated drive");
        assert!(steps > 1, "the reserved dummy job is a multi-turn bounded job; every turn is one funded step ({steps})");
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert!(settled.history_patch.is_some_and(|patch| patch.can_undo && patch.can_redo && !patch.upserts.is_empty()));
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪️ The addressed borrowed receiver retains one original invocation and caller through pending admission.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_addressed_receiver_admits_isolated_spawn_job() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::ViewWindowInstance;
        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        app.bind_instance_id(1).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.expect("host-wire bundle"));
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_push_instance(&runtime, semio_framework_plugin_plugin_program::AppInstance { id: 1, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, 1, "local");
        let action_json = format!(
            r#"{{"address":{{"pluginId":"test","appId":"{}","modeId":"edit","windowKindId":"puzzle3d-main","windowInstanceId":"puzzle3d-main-perspective","actionId":"undo"}},"arguments":{{"windowId":"puzzle3d-main-perspective"}}}}"#,
            TestApp::<false>::APP_ID,
        );
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_id: Some("puzzle3d-main-perspective".into()),
            window_instances: vec![ViewWindowInstance { id: "puzzle3d-main-perspective".into(), window_kind_id: "puzzle3d-main".into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let mut invocation:ManifestActionInvocation=semio_framework_pack_json::from_json_str(&action_json,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed original addressed invocation");
        let meta=ActionMeta{actor:"local".into(),instance_id:1,view_state:Some(view)};
        let cell=runtime.instances.borrow().get(1).cloned().expect("original mounted receiver");
        let mut original_args=Some(DslValue::Object(std::mem::take(&mut invocation.arguments).into_iter().collect()));let mut original_fault=None;
        let mut admitted=None;
        for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){
            let mut instance=cell.host_instance().expect("original exclusive receiver");
            admitted=instance.app.handle_action_invocation(&invocation.address,&mut original_args,&mut original_fault,Some("edit"),&meta,&mut identity,cx).await.expect("actual original addressed receiver");
            if admitted.is_some(){break}
            instance.app.maintenance_step(cx,cx.retained_grant()).expect("original receiving maintenance");
        }
        let admitted=admitted.expect("finite original addressed admission reached acceptance");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "export first turn must admit Isolated framework.reserved.tool, got {:?}",
            admitted.requested_effects,
        );
        assert!(admitted.output.get("operationId").and_then(semio_framework_value::DslValue::as_str).is_some(), "first-turn output names the spawn-job");
        let cell = runtime.instances.borrow().get(1).cloned().expect("live export instance");
        let mut instance = cell.instance.lock().expect("export instance");
        artifact_app_laws::close_registered_fixture_app(&mut instance.app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_config_only_view_action_is_not_a_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert!(app.render_ctx().test_history().await.commands.is_empty(), "a config-only View is never a row (design §20.13, L4)");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "while the config lane holds its edit");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_does_not_widen_a_none_ui_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::ViewNoScope, &meta(), &mut identity,cx).await.expect("viewNoScope");
        assert_eq!(result.ui_scope, UiDirtyScope::None);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_preserves_partial_ui_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::ViewPartialScope, &meta(), &mut identity,cx).await.expect("viewPartialScope");
        let UiDirtyScope::Partial { window_bodies, panel_bodies, .. } = result.ui_scope else { panic!("expected a Partial scope") };
        assert_eq!(window_bodies, vec!["some.window".to_string()]);
        assert!(panel_bodies.is_empty());

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn scope_upgrade_full_stays_full() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert_eq!(result.ui_scope, UiDirtyScope::Full);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn view_action_emitting_ops_is_rejected() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let error = app.dispatch_typed(TestCommand::BadView, &meta(), &mut identity,cx).await.expect_err("a View command emitting operations must be rejected");
        assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
