/// 🧶️ Streamed ticks and reserved undo: streamed ticks as one undo step, plain emissions, history actions and the reserved undo invocations.

    /// 🪡️ A streamed tool transaction is ONE edit (design §15): every tick joins the open edit, the commit tick closes it,
    /// and one undo reverts the whole gesture back to the empty label.
    #[semio_framework_async_macros::async_test]
    async fn streamed_ticks_and_their_commit_are_one_undo_step() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for value in ["a", "ab"] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel tick");
        }
        app.dispatch_typed(TestCommand::StreamLabel { value: "abc".into(), commit: true }, &meta(), &mut identity,cx).await.expect("streamLabel commit");
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧱️ Outside a tool transaction every emission is its own plain edit (design §20.1): nothing coalesces, so one undo
    /// reverts exactly the last dispatch.
    #[semio_framework_async_macros::async_test]
    async fn plain_emissions_never_coalesce() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for value in ["a", "ab", "abc"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        }
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "ab");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_actions_round_trip_through_the_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc1");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc2");
        assert_eq!(app.stores.test_snapshot().await.count, 2);

        let undo = reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert!(undo.mutations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1);

        reserved_action(&mut app, "redo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 2);

        let checkpoint = reserved_action(&mut app, "commitCheckpoint", None, &mut identity, cx).await;
        assert!(checkpoint.mutations.is_empty());
        assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_invocation_does_not_require_window_ownership() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let mut app = contract_app(mounted_policy, &mut identity).await;
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "main-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;let admitted=artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta(),&mut identity,cx).await.expect("reserved undo without window ownership");
        let result = settle_reserved(&mut app, admitted, cx).await;
        assert!(result.mutations.is_empty());
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit undo");
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)));
        assert!(admitted.events.iter().all(|event| event.kind != "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first turn answers without committing");
        let settled = settle_reserved(&mut app, admitted, cx).await;
        assert!(settled.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 0, "driving the spawned job must run commit_framework_history_route");
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
