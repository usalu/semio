/// 🔀️ Merge, heads and media import: authoritative merge policy conflicts, the ephemeral lane, composed child heads and the stepped whole-document media import.

    #[semio_framework_async_macros::async_test]
    async fn merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let before_snapshot = app.stores.test_snapshot().await;
        let before_edits = app.stores.test_store().await.applied_edit_ids().len();

        let set = semio_framework_plugin_runtime_instances::inst_wire_frames::set_merge_policy_frames(&mut app, 7, protocol::MergePolicy::Vigilant.as_u8()).await.expect("known policy accepts");
        assert_eq!(set, vec![protocol::AppFrame::Done { in_reply_to: 7 }]);
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant);
        assert_eq!(app.stores.test_snapshot().await, before_snapshot, "local policy changes no document state");
        assert_eq!(app.stores.test_store().await.applied_edit_ids().len(), before_edits, "local policy changes no shared history");

        let invalid_policy = semio_framework_plugin_runtime_instances::inst_wire_frames::set_merge_policy_frames(&mut app, 8, 3).await.expect_err("unknown policy ordinal rejects");
        assert_eq!(invalid_policy.code.0, "merge.invalid-policy");
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant, "invalid policy never changes the active policy");

        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity).await.expect("seed one durable edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the seeded durable edit reaches its publication");
        let (mut envelope, applied_edit_ids) = {
            let store = app.stores.test_store().await;
            let files = store::os_store::text_format::print_document_pack(store.envelope()).await.expect("print retained test envelope");
            (store::os_store::text_format::parse_document_pack::<TestSnapshot, TestMutation>(&files.pack, &files.spr).await.expect("parse retained test envelope").envelope, store.applied_edit_ids().to_vec())
        };
        let kind = protocol::ConflictKind::Degraded { edit_ids: applied_edit_ids.clone() };
        let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
        let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
        let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
        envelope.conflicts.push(protocol::Conflict {
            id: conflict_id.clone(),
            kind,
            status: protocol::ConflictStatus::Open,
            messages: vec![protocol::MutationMessage::warning("mutation.partial", "test conflict").at_op(0)],
            actors: vec![ActorId("local".into())],
            timestamp,
        });
        app.stores.test_store_mut().await.reset(envelope).await.expect("seed valid open degraded conflict");

        let read = semio_framework_plugin_runtime_instances::inst_wire_frames::read_conflicts_frames(&app, 9).await;
        let [protocol::AppFrame::Conflicts { in_reply_to: Some(9), conflicts }] = read.as_slice() else { panic!("read must yield exactly one correlated conflict projection") };
        let open: Vec<protocol::Conflict> = semio_framework_plugin_plugin_program::decode_wire_serialized(conflicts).await.expect("conflict payload decodes canonically");
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, conflict_id);
        assert_eq!(open[0].status, protocol::ConflictStatus::Open);

        let invalid_resolution = semio_framework_plugin_runtime_instances::inst_wire_frames::resolve_conflict_frames(&mut app, 10, conflict_id.0.clone(), 2).await.expect_err("unknown resolution ordinal rejects");
        assert_eq!(invalid_resolution.code.0, "merge.invalid-resolution");
        assert_eq!(app.open_conflicts().await.len(), 1, "invalid resolution never changes conflict state");

        let resolved = semio_framework_plugin_runtime_instances::inst_wire_frames::resolve_conflict_frames(&mut app, 11, conflict_id.0.clone(), 0).await.expect("accept resolves the open degraded conflict");
        let [protocol::AppFrame::MergeReport { in_reply_to: Some(11), report }, protocol::AppFrame::Conflicts { in_reply_to: Some(11), conflicts }] = resolved.as_slice() else {
            panic!("resolve must emit correlated merge report then open-conflict projection")
        };
        let report: protocol::MergeReport = semio_framework_plugin_plugin_program::decode_wire_serialized(report).await.expect("merge report payload decodes canonically");
        assert!(report.accepted);
        assert_eq!(report.conflict.as_ref(), Some(&conflict_id));
        let open: Vec<protocol::Conflict> = semio_framework_plugin_plugin_program::decode_wire_serialized(conflicts).await.expect("post-resolution conflict payload decodes canonically");
        assert!(open.is_empty(), "accepted conflict leaves the open projection");
        assert!(app.open_conflicts().await.is_empty());
        drain_and_close_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "x".into() }, &meta(), &mut identity,cx).await.expect("set label");
        assert_eq!(app.stores.presence_store.generation().await, 0);
        assert_eq!(app.stores.transient_store.generation().await, 0);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪆️ LAW (design §20.15, W-b): `child_head_packs` — what `AppCommand::ReadChildHeads` answers and the MCP gateway injects as
    /// the `child:<slot>/<childId>` dependencies of an inference request — is every owned child's CURRENT head snapshot pack with
    /// its slot, id and dialect: a reader that composes on read decodes the child's live value from it, no history fold.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_documents_child_heads_answer_each_owned_childs_current_head_snapshot() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the composite gesture settles");

        let heads = PluginApp::child_head_packs(&app).await.expect("child heads");
        assert_eq!(heads.iter().map(|entry| (entry.slot.as_str(), entry.child_id.as_str(), entry.dialect.clone())).collect::<Vec<_>>(), vec![("slot", "child-1", test_child_dialect().await.to_coordinate())]);
        let head: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&heads[0].head_pack).expect("the head pack is the child's snapshot pack");
        assert_eq!(head.count, 7, "the head pack carries the child's current value");
        assert_eq!(semio_framework_plugin_extension::artifact_inference::inference_child_dependency(&heads[0].slot, &heads[0].child_id), "child:slot/child-1");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎞️ LAW (W2A-6 P5, `📓️api-stepped-document-load.md` §4/§9): consuming a whole document of the app's own schema never folds
    /// inside the call — `consume_media` answers the carrier's archive (the source document verbatim) and leaves the document untouched;
    /// only the stepped archive load (admit → polls → acknowledge) makes it the consumer's document. The carrier is `pk:` base64 text, so
    /// a text-only host edge carries it losslessly. A foreign document schema is refused under the localized framework code.
    #[semio_framework_async_macros::async_test]
    async fn a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut source = contract_composed_app_raw(mounted_policy, &mut identity).await;
        source
            .test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 41 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("the source edits its document");
        let media_fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎞️media/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let media_grant=serde_json::from_value(media_fixture["grant"].clone()).unwrap();let mut media_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let original=identity.pause().expect("same original media admission receipt");assert_eq!(original.maximum_bytes(),media_fixture["originalNativeMaximumBytes"].as_u64().unwrap()as usize);let original=match original.with_retirement_recipient(&mut media_recipient){Ok(original)=>original,Err((error,_original))=>panic!("{error}")};let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume_retirement(original,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let artifact = identity.encode(|native| { let mut owner=semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner::new(native,media_grant); ::semio_framework_async::poll::resolve_ready(PluginApp::produce_media(&mut source, "artifact:out", &mut owner)) }).expect("the source hands out its whole document through its original authority");
        let mut consumer = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let before = consumer.test_snapshot().await;
        let source_id = PluginApp::document_identity(&source).expect("the source's identity");
        assert!(std::str::from_utf8(&artifact.data).is_ok_and(|text| text.starts_with("pk:")), "the whole-document carrier is pk: base64 text");

        let semio_framework_plugin_decl_app::MediaConsumption::DocumentLoad(archive) = PluginApp::consume_media(&mut consumer, "artifact:in", artifact.clone()).await.expect("a whole document of the own schema is consumed") else {
            panic!("a whole document is answered as a document archive load");
        };
        assert_eq!(consumer.test_snapshot().await, before, "consuming folded nothing: the document is unchanged until the load runs");
        assert!(archive.members.is_empty() && !archive.parent_pack.is_empty() && !archive.parent_spr.is_empty());
        PluginApp::begin_document_archive_load(&mut consumer, 7, archive).expect("the runtime admits the archive");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(&mut consumer, 7, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(&mut consumer, 7).expect("acknowledge");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready);
        assert_eq!(consumer.test_snapshot().await, source.test_snapshot().await, "the stepped load lands the source's head");
        assert_eq!(PluginApp::document_identity(&consumer).as_deref(), Some(source_id.as_str()), "a whole-document transfer loads the source document verbatim");

        let mut foreign = artifact;
        foreign.descriptor.wire = semio_framework::MediaWireFormat::Document { schema: "foreign.document".into() };
        let refusal = PluginApp::consume_media(&mut consumer, "artifact:in", foreign).await.expect_err("a foreign document schema is refused");
        let fault = semio_framework_plugin_decl_app::media_artifact_fault(refusal);
        assert_eq!(fault.code.0, semio_framework_plugin_decl_app::MEDIA_SCHEMA_MISMATCH_CODE);
        let notice = semio_framework::kernel::fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).expect("the refusal earns its localized notice");
        assert!(notice.text.contains("foreign.document") && notice.text.contains(<TestApp as ArtifactApp>::DOCUMENT_SCHEMA), "{}", notice.text);
        drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut source, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
