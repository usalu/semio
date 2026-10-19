/// ↩️ Reserved undo and note shell command: reserved undo ingress and chrome steps, revert snapshots, config-only dispatches and the intercepted note shell command.

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_actor_ingress_admits_undeclared_window_kind() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation, ViewWindowInstance};
        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let before = app.stores.test_store().await.applied_edit_ids().len();
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_instances: vec![ViewWindowInstance { id: "world-1".into(), window_kind_id: "world".into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "missing-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        assert!(semio_framework_plugin_reactor_app::addressed_action_view(&view, &invocation.address).is_err(), "strict window projection rejects a missing instance");
        let admitted = semio_framework_plugin_reactor_app::admit_addressed_action_view(&view, &invocation.address).expect("reserved undo is admitted without window ownership");
        let meta = ActionMeta { view_state: Some(admitted), ..meta() };
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;let admitted=artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta,&mut identity,cx).await.expect("actor reserved undo");
        assert!(admitted.mutations.is_empty());
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)), "first turn must admit spawn-job");
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first turn must not commit history");
        let result = settle_reserved(&mut app, admitted).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.stores.test_snapshot().await.count, 0, "history order must regress through actor ingress");
        assert!(app.stores.test_store().await.applied_edit_ids().len() < before);
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 Browser #37 mixed stack: document example under chrome-top Resize Window.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_shell_then_falls_through_to_document_store() {
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
            .expect("document example");
        let _ = app.render_ctx().test_history().await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
         &mut identity)
        .await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let before = app.render_ctx().test_history().await;
        assert!(before.commands.iter().any(|entry| label_in(&entry.label, Locale::En).contains("Set Active Example")));
        assert_eq!(before.commands.first().map(|entry| label_in(&entry.label, Locale::En)), Some("Resize Window"));

        let first = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(
            first.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first undo must pop the chrome-top shell, not the document example");
        assert!(first.history_patch.as_ref().is_some_and(|patch| patch.can_undo && !patch.upserts.is_empty()));

        let second = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(second.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })));
        assert_eq!(app.stores.test_snapshot().await.count, 0, "second undo must apply the document inverse");
        assert!(second.history_patch.as_ref().is_some_and(|patch| patch.can_redo && !patch.upserts.is_empty()));

        reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1, "redo must restore the document example before the shell");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 §3.2's user-visible half: clicking a window between two edits noted `shell.windowActivate`,
    /// which the chrome branch then popped instead of the user's own edit — undo looked like it did
    /// nothing and the host replayed the chrome id into the guest. A note with no declared inverse is
    /// not an undo target, so the DOCUMENT edit is what `undo` pops and no replay leaves the guest.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_steps_over_undeclared_chrome_and_pops_the_document_edit() {
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
            .expect("document edit");
        let _ = app.render_ctx().test_history().await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({ "commandId": "shell.windowActivate", "label": "Activate Window", "detail": { "windowId": "trinity-jack-graph" } }))),
         &mut identity)
        .await;
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.first().map(|entry| label_in(&entry.label, Locale::En)), Some("Activate Window"), "the window activation is the newest row");
        assert!(before.commands.iter().find(|entry| entry.action_id == "shell.windowActivate").is_some_and(|entry| !entry.revertible), "a note that declared no inverse is not an undo target");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let undone = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(
            undone.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })),
            "undo must not replay a window activation into the guest"
        );
        assert_eq!(app.stores.test_snapshot().await.count, 0, "undo must pop the document edit, not the window activation");
        let after = app.render_ctx().test_history().await;
        assert!(after.commands.iter().any(|entry| entry.action_id == "shell.windowActivate"), "the chrome row stays in the append-only log");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ `commands` is newest-first — take the MINIMUM seq among "increment" entries to target inc1, not inc2.
    ///
    /// 🧾️ `commands` is newest-first — the just-appended revert entry is the FIRST element, not the last.
    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_restores_the_snapshot_and_appends_one_entry() {
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
        let first_increment_seq = app.render_ctx().test_history().await.commands.iter().filter(|entry| entry.action_id == "increment").map(|entry| entry.seq).min().expect("first increment entry");
        let before_len = app.render_ctx().test_history().await.commands.len();

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first_increment_seq }))), &mut identity).await;
        assert!(result.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1, "revert leaves the target edit applied, undoing only what came after it");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), before_len + 1, "exactly one entry appended for the revert itself");
        assert_eq!(history.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn consecutive_config_only_view_dispatches_add_no_history_rows() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for id in ["node-1", "node-2", "node-3"] {
            app.dispatch_typed(TestCommand::Select { id: Some(id.into()) }, &meta(), &mut identity,cx).await.expect("select");
        }
        assert!(app.render_ctx().test_history().await.commands.is_empty(), "three config-only Views add no row");
        assert_eq!(app.stores.test_config().await.selected, Some("node-3".to_string()));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn note_shell_command_is_intercepted_before_the_app_and_records_each_repeat() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let args = dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "detail": "dark" }));
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &mut identity).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty(), "interception must happen before the app ever sees noteShellCommand");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "os.setThemeId");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(label_in(&entry.label, Locale::En).contains("dark"));

        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &mut identity).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty());
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 2);
        assert!(history.commands.iter().all(|entry| entry.count == 1));
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
