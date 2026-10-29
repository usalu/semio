// 📔️ Command log laws: the command log entries of operations, gestures, undo, redo, remote edits, history filters and shell noted rows.

    //#region 🔖️CommandLogTests
    /// 🏷️ `increment` IS declared by `migrated_contract_registry` (the fixture every law here now uses),
    /// so the row carries its DECLARED label — still `LocalizedLabel::data`, so still locale-invariant
    /// rather than untranslated English, which is the property this clause exists for.
    #[semio_framework_async_macros::async_test]
    async fn an_operation_action_appends_one_command_log_entry_linked_to_its_edit() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "increment");
        assert_eq!(label_in(&entry.label, Locale::En), "Increment");
        assert_eq!(label_in(&entry.label, Locale::De), "Increment", "a declared label is locale-invariant data, never untranslated English");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_some());
        assert!(!entry.op_lines.is_empty(), "operation entry must carry printed op-text");
        assert!(entry.applied && entry.revertible);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_streamed_gesture_appends_exactly_one_command_log_entry() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for (value, commit) in [("a", false), ("ab", false), ("abc", true)] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit }, &meta(), &mut identity,cx).await.expect("streamLabel");
        }
        let history = app.render_ctx().test_history().await;
        let stream_entries: Vec<&CommandView> = history.commands.iter().filter(|entry| entry.action_id == "streamLabel").collect();
        assert_eq!(stream_entries.len(), 1, "a streamed gesture must grow one entry's op_lines, not append new entries");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📜️ LAW (ticket 26/09/23 C12): a streamed gesture's history row previews only its edit's newest
    /// `HISTORY_ROW_OPERATION_PREVIEW` operations — the newest one last — however long the gesture runs, so reading the history
    /// costs the same per tick at the end of a long gesture as at its start.
    #[semio_framework_async_macros::async_test]
    async fn a_long_streamed_gesture_previews_its_newest_operations_only() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let mut value = String::new();
        for key in 0..64u8 {
            value.push(char::from(b'a' + key % 26));
            app.dispatch_typed(TestCommand::StreamLabel { value: value.clone(), commit: key == 63 }, &meta(), &mut identity,cx).await.expect("streamLabel");
            let _ = app.render_ctx().test_history().await;
        }
        let history = app.render_ctx().test_history().await;
        let row = history.commands.iter().find(|entry| entry.action_id == "streamLabel").expect("the gesture's row");
        assert_eq!(row.op_count, 64, "the row counts every operation of its edit");
        assert_eq!(row.op_lines.len(), HISTORY_ROW_OPERATION_PREVIEW, "the row previews a bounded tail of its operations");
        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
        let exported = app.history_snapshot().await.expect("history snapshot");
        let exported = exported.upserts.iter().find(|entry| entry.action_id == "streamLabel").expect("the gesture's exported row");
        assert_eq!((exported.op_count, exported.op_lines.len()), (64, HISTORY_ROW_OPERATION_PREVIEW), "the exported row carries the bounded preview and its edit's operation count");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_and_redo_append_entries_and_never_shrink_the_log() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        assert_eq!(app.render_ctx().test_history().await.commands.len(), 1);
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        let after_undo = app.render_ctx().test_history().await;
        assert_eq!(after_undo.commands.len(), 2, "undo appends, it does not remove the increment entry");
        assert!(after_undo.commands.iter().any(|entry| entry.action_id == "increment"));
        reserved_action(&mut app, "redo", None, &mut identity, cx).await;
        let after_redo = app.render_ctx().test_history().await;
        assert_eq!(after_redo.commands.len(), 3, "redo appends a third entry");
        assert!(after_redo.commands.iter().any(|entry| entry.action_id == "undo"));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ The receiver never dispatched anything itself — any log entry it has must come from backfill.
    #[semio_framework_async_macros::async_test]
    async fn ingested_remote_edits_are_backfilled_into_the_command_log() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut sender = contract_app(mounted_policy, &mut identity).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc-history-backfill", "mem://doc-history-backfill").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = contract_app(mounted_policy, &mut identity).await;
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest");
        let history = receiver.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        assert!(history.commands[0].edit_id.is_some());
        assert!(!history.commands[0].op_lines.is_empty());

        artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");

        artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_emits_no_operations_and_updates_the_view() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" }))), &mut identity, cx).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.render_ctx().test_history().await.command_filter, HistoryCommandFilter::OnlyMutations);
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn interleaved_config_only_views_leave_only_the_document_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta(), &mut identity,cx).await.expect("select a");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta(), &mut identity,cx).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("c".into()) }, &meta(), &mut identity,cx).await.expect("select c");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.count)).collect::<Vec<_>>(), vec![("increment", 1)]);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🌐️ A shell-noted row's text is the shell's own already-rendered copy, so the row stores it as
    /// locale-INVARIANT data (`LocalizedLabel::data`) — every cell of the matrix carries it. The row
    /// therefore never claims to be English, and the History panel renders it unchanged in a German
    /// shell instead of leaking a locale it does not have.
    #[semio_framework_async_macros::async_test]
    async fn a_shell_noted_row_carries_the_same_text_in_every_locale() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme" }))), &mut identity, cx).await;
        let history = app.render_ctx().test_history().await;
        let label = &history.commands[0].label;
        for terminology in Terminology::ALL {
            for locale in Locale::ALL {
                assert_eq!(label.resolve(terminology, locale), "Set Theme", "{terminology:?}/{locale:?}");
            }
        }
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🙈️ A `View` verb that edited no store and carries no inverse is no history row, while a mutation next to it is.
    #[semio_framework_async_macros::async_test]
    async fn a_pure_view_verb_is_no_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::ViewNoScope, &meta(), &mut identity,cx).await.expect("viewNoScope");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::ViewPartialScope, &meta(), &mut identity,cx).await.expect("viewPartialScope");
        let history = app.render_ctx().test_history().await;
        let rows: Vec<&str> = history.commands.iter().map(|entry| entry.action_id.as_str()).collect();
        assert_eq!(rows, vec!["increment"], "pure view verbs never become rows");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let before_len = app.render_ctx().test_history().await.commands.len();
        let result = reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(result.ui_scope, UiDirtyScope::None, "nothing was logged, so the scope must not be upgraded either");
        assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len);
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_is_never_logged() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let before_len = app.render_ctx().test_history().await.commands.len();
        reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" }))), &mut identity, cx).await;
        assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len, "the filter's own chrome must not fill the list it filters");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render before");
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme" }))), &mut identity, cx).await;
        let rendered = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render after");
        assert_eq!(rendered.root.children.len(), 2, "Actions + Commands");
        assert_eq!(rendered.root.children[1].children.len(), 1, "a log-only cache key change (no store generation bump) must still refresh the rendered panel");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn an_operation_kind_action_with_zero_operations_still_logs_one_entry() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::NoopMutation, &meta(), &mut identity,cx).await.expect("noopMutation");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "noopMutation");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_none(), "no operations means no VCS edit, even though the action is Mutation-kind");
        assert_eq!(entry.count, 1);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    //#endregion 🔖️CommandLogTests
