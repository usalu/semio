/// 📚️ Archive load, reload and revert: the test archive load, the child persist and reload cycle, the child content view across undo, group undo and revert to command.

    /// 📥️ Drives one stepped document archive load on `app` to its terminal status and acknowledges it.
    use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers;
async fn drive_test_archive_load(app: &mut VcsArtifactApp<TestApp, TestMembers>, operation: u64, archive: protocol::DocumentArchivePack) -> protocol::DocumentArchiveLoadStatus {
        PluginApp::begin_document_archive_load(app, operation, archive).expect("the runtime admits the archive");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(app, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(app, operation).expect("acknowledge");
        status
    }

    /// 🧒️ Registering a member is the RUNTIME's job; declaring it on the parent snapshot is the
    /// APP's: the runtime admits a member only under the identity its parent's snapshot declares.
    ///
    /// 📤️ Persist exactly what the host does: the document archive (`ReadDocumentArchive`) — the
    /// parent pack and `.spr` plus one owner-stamped entry per live member (design §21.6: the
    /// recursive archive is the single carrier).
    ///
    /// 📥️ Reload into a FRESH app through the stepped archive load (`LoadDocumentArchive`). The
    /// child comes back as its OWN live store, at the value its own history ended on, owned by the
    /// reloaded document.
    ///
    /// 🚫️ The members travel WITH the parent: the same archive without them is refused by the
    /// ownership closure (`closure-rejected`), so a composed parent never loads half — there is no
    /// "parent first, children later" route.
    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the migrated composite gesture settles before it is persisted");
        assert_eq!(app.stores.test_snapshot().await.slot.len(), 1, "the live parent declares exactly its one member");

        let archive = PluginApp::document_archive(&app).await.expect("the composed document archives its closure");
        let document = app.stores.store.envelope().id.clone();
        assert_eq!(
            archive.members.iter().map(|entry| (entry.owner.parent.artifact_id.as_str(), entry.owner.slot.as_str(), entry.owner.child_id.as_str(), entry.reference.artifact_id.as_str())).collect::<Vec<_>>(),
            vec![(document.as_str(), "slot", "child-1", "child-1")],
            "the archive carries the one member, owner-stamped to the document"
        );

        let mut reloaded = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let status = drive_test_archive_load(&mut reloaded, 21, archive.clone()).await;
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
        assert_eq!(reloaded.test_snapshot().await.slot.len(), 1, "the reloaded parent carries its own declaration through the pack");
        let child = reloaded.child_store("slot", "child-1").await.expect("child restored");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the reloaded child lost its own edit history");
        assert_eq!(child.owner_ref().map(|owner| (owner.parent.artifact_id, owner.slot, owner.child_id)), Some((document.clone(), "slot".to_string(), "child-1".to_string())), "the reloaded child is owned by the reloaded document");

        let mut halved = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let mut parent_alone = archive;
        parent_alone.members.clear();
        let refused = drive_test_archive_load(&mut halved, 22, parent_alone).await;
        let refusal = format!("{:?}", semio_framework_diagnostic::decode_fault_bytes(&refused.fault));
        assert_eq!(refused.state, protocol::DocumentArchiveLoadState::Fault, "a parent that declares a member never loads without it: {refusal}");
        assert!(refusal.contains("closure-rejected"), "the ownership closure names the refusal: {refusal}");
        assert!(halved.test_snapshot().await.slot.is_empty() && halved.child_store("slot", "child-1").await.is_none(), "a refused load leaves the open document as it was");

        drain_and_close_composed_fixture(&mut halved, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut reloaded, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ↩️ Store-level undo bypasses `ArtifactApp::handle` entirely — this is exactly where the
    /// `thread_local!` child caches this view replaces used to go stale.
    #[semio_framework_async_macros::async_test]
    async fn the_child_content_view_never_goes_stale_across_undo_and_redo() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        assert_eq!(reads_child_count(&app).await, 7);

        app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("undo");
        assert_eq!(reads_child_count(&app).await, 0, "the view must reflect the child's undone state");
        app.dispatch_action("redo", None, &meta(), &mut identity,cx).await.expect("redo");
        assert_eq!(reads_child_count(&app).await, 7, "the view must reflect the child's redone state");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪢️ `child-b` is registered but NEVER targeted by the composite gesture below — its
    /// `tail_group_id()` stays `None`, the textbook "foreign tail" `GroupUndoReport` must
    /// skip rather than abort the whole group over.
    ///
    /// The parent reverted...
    ///
    /// ...child-a (the real group member) reverted too...
    ///
    /// ...and child-b — a genuine foreign tail, never touched by this group — is reported as
    /// SKIPPED, not silently dropped nor allowed to abort the rest of the group.
    #[semio_framework_async_macros::async_test]
    async fn group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-a").await;
        register_test_child(&mut app, "child-b").await;

        let before = app.stores.test_snapshot().await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-a".into(), child_value: 5 }, &meta(), &mut identity,cx).await.expect("composite edit");
        assert_eq!(app.stores.test_snapshot().await.label, "composite");

        let result = app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("group undo");

        assert_eq!(app.stores.test_snapshot().await, before);
        let TestMembers::Child(child_a_store) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("child-a").member;
        assert_eq!(child_a_store.snapshot().expect("child-a snapshot").count, 0);
        assert!(!result.diagnostics.is_empty(), "child-b's foreign tail must surface a diagnostic, not vanish silently");
        assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");


        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }


    /// ⏪️ Revert-to-command walks the DOCUMENT lane alone (design §20.13, L4): "leave the TARGET row applied, undo everything
    /// after it" — reverting to the first increment undoes the second and leaves the config the interleaved selects wrote.
    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_walks_the_document_lane_and_leaves_config_alone() {
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
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta(), &mut identity,cx).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment again");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 2, "two document rows and no config row");
        let first = history.commands.iter().min_by_key(|entry| entry.seq).expect("the first increment row");
        assert!(first.revertible, "a document row is revertible");
        reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first.seq }))), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1, "reverting to the first increment undoes the second");
        assert_eq!(app.stores.test_config().await.selected, Some("b".to_string()), "the config lane is never walked");
        let after = app.render_ctx().test_history().await;
        assert_eq!(after.commands.len(), 3, "the revert appends one History-kind row");
        assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
