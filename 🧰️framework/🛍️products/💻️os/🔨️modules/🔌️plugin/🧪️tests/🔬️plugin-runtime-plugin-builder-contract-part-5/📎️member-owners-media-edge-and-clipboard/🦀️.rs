/// 📎️ Member owners, media edge and clipboard: the test member owners, the composed document media edge, child retirement rejection, config lane edits and the copy, cut and paste laws.

    /// 🔐️ Adopts the exact member owner catalog a product child carries from
    /// `create_member_store`, for a law that first drove the refusal an owner-less member owes and
    /// then has to close the app it registered that member into.
    use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers;
fn install_test_member_owners(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), child_id.to_string())).expect("exact child owner").member;
        store::install_funded_member_store_owners(child,&semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer).expect("actual funded member catalog");
    }

    /// 🪆️ LAW (design §20.15, S4-LOAD wave 4): a COMPOSED document crosses a media edge whole — `produce_media("artifact:out")` carries the
    /// recursive archive (parent pack + `.spr` + members) as text that survives a UTF-8 host round trip, and the consumer's stepped load
    /// adopts the members: the child comes back as its own live store at the value its history ended on.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_document_crosses_a_media_edge_with_its_children() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut source = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut source, "child-1").await;
        source.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut source, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the composite gesture settles");
        let media_fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎞️media/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let media_grant=serde_json::from_value(media_fixture["grant"].clone()).unwrap();let mut media_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let original=identity.pause().expect("same original composed media admission receipt");assert_eq!(original.maximum_bytes(),media_fixture["originalNativeMaximumBytes"].as_u64().unwrap()as usize);let original=match original.with_retirement_recipient(&mut media_recipient){Ok(original)=>original,Err((error,_original))=>panic!("{error}")};let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume_retirement(original,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let mut artifact = identity.encode(|native| { let mut owner=semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner::new(native,media_grant); ::semio_framework_async::poll::resolve_ready(PluginApp::produce_media(&mut source, "artifact:out", &mut owner)) }).expect("the composed source hands out its whole document through its original authority");
        artifact.data = String::from_utf8(artifact.data).expect("a text-only host edge carries the carrier").into_bytes();

        let mut consumer = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let semio_framework_plugin_decl_app::MediaConsumption::DocumentLoad(archive) = PluginApp::consume_media(&mut consumer, "artifact:in", artifact).await.expect("consumed") else {
            panic!("a whole document is answered as a document archive load");
        };
        assert_eq!(archive.members.iter().map(|entry| (entry.owner.slot.as_str(), entry.owner.child_id.as_str())).collect::<Vec<_>>(), vec![("slot", "child-1")]);
        PluginApp::begin_document_archive_load(&mut consumer, 9, archive).expect("admit");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(&mut consumer, 9, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(&mut consumer, 9).expect("acknowledge");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
        let child = consumer.child_store("slot", "child-1").await.expect("the member came along");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the child arrives at the value its own history ended on");
        drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut source, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎡️ The rotation is fair: one call runs every stage that still has something to give and
    /// reports a blocked stage only once nothing else can progress, so the block is the rotation's
    /// EVENTUAL answer rather than the answer of the one call that lands on stage 4. A lost
    /// authority never reports a block at all, so nothing here is weakened.
    #[semio_framework_async_macros::async_test]
    async fn child_snapshot_retirement_rejection_preserves_exact_erased_owner() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "child-a").await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child-a");
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        assert_maintenance_reports_block(&mut app, 4, "the rejected child snapshot transfer",cx);
        let retirement = app.child_composition.child_content_retirements.get(2).expect("retirement remains registered after rejected transfer");
        let entry = retirement.pending.as_ref().expect("exact rejected snapshot remains pending");
        assert!(entry.snapshot.typed::<TestSnapshot>().is_some(), "rejection preserves the exact erased owner and type identity");
        install_test_member_owners(&mut app, "child-a");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⚖️ LAW (design §20.13, L4): a config-lane edit is never a history row and undo never steps over it — the history
    /// lists the document row alone (every row, no `edit_id` filter), and `undo` retracts the newest DOCUMENT edit while the
    /// config the select wrote stays.
    #[semio_framework_async_macros::async_test]
    async fn a_config_lane_edit_is_never_a_history_row_and_undo_never_steps_over_it() {
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
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "the select wrote the config lane");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.iter().map(|entry| entry.action_id.as_str()).collect::<Vec<_>>(), vec!["increment"], "only the document row is listed");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 0, "undo retracts the document edit, never the config one");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "undo never steps over the config lane");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🫧️ The plugin cannot touch shell-owned state itself — it bubbles the inverse out as an effect
    /// instead of replaying anything locally, and does NOT append a new log entry on its own.
    #[semio_framework_async_macros::async_test]
    async fn shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "inverseCommandId": "os.setThemeId", "inverseArgs": { "themeId": "light" } }))), &mut identity).await;

        let history = app.render_ctx().test_history().await;
        let entry = history.commands.first().expect("one logged shell row");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(entry.revertible, "a Shell row with a stored inverse must be revertible");
        let seq = entry.seq;

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": seq }))), &mut identity).await;

        assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: Some(semio_framework::dsl_value!({ "themeId": "light" })) }]);
        assert_eq!(app.render_ctx().test_history().await.commands.len(), history.commands.len(), "bubbling the effect logs nothing new by itself");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_emits_clipboard_write_effect_with_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        let result = reserved_action(&mut app, "copy", None, &mut identity).await;
        assert!(result.mutations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.dsl_text, "hello");
        assert_eq!(fragment.source_app, TestApp::<false>::APP_ID);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_on_empty_selection_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "copy", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ✂️ One undo restores the cut label — cut is a single coalesced edit, not two.
    #[semio_framework_async_macros::async_test]
    async fn cut_removes_label_and_emits_clipboard_write_as_one_undo_unit() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        let result = reserved_action(&mut app, "cut", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");
        assert_eq!(result.requested_effects.len(), 1);
        assert!(matches!(&result.requested_effects[0], Effect::ClipboardWrite { fragment } if fragment.dsl_text == "hello"));
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "hello");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_materializes_fragment_at_original_anchor() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "original" }));
        reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "pasted");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_non_original_anchor_reaches_the_app_placement() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "centroid" }));
        reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, format!("pasted-{:?}", PasteAnchor::Centroid));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "paste", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.stores.test_snapshot().await.label, "");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
