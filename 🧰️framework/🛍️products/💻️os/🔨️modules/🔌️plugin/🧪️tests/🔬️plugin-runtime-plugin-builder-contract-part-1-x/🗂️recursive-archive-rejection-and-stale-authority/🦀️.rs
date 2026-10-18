// 🗂️ Recursive archive, rejection and stale authority: the recursive document archive round trip and cancellation, rejection cases and stale authority cases.

//#region 🧬️ComposedParentFixture
#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_document_archive_round_trips_the_complete_owned_closure() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let archive = recursive_document_archive("archive-parent").await;
    let expected = archive.clone();
    let mut app = live_recursive_replacement_app().await;
    PluginApp::begin_document_archive_load(app.as_mut(), 81, archive).expect("recursive archive admission");
    let status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 81, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 81).expect("recursive archive acknowledgement");
    assert_eq!(app.stores.store.envelope().id, "archive-parent");
    assert_eq!(app.stores.store.snapshot().expect("archive parent current snapshot").revision, 7);
    assert_eq!(app.child_composition.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("archive branch content").count, 29);
    assert_eq!(app.child_composition.child_content_root.typed_read_at::<RecursiveBranchSnapshot>(MemberKeyRef { owner: "child-1", slot: "nested", child_id: "grandchild-1" }).expect("archive leaf content").count, 53);
    let persisted = Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("recursive archive read");
    assert_eq!(persisted.members.len(), 2);
    assert_eq!(persisted.members[0].owner.parent.artifact_id, "archive-parent");
    assert_eq!(persisted.members[1].owner.parent.artifact_id, "child-1");
    assert_eq!(persisted, expected);

    let mut malformed = persisted.clone();
    malformed.parent_pack.push(0);
    PluginApp::begin_document_archive_load(app.as_mut(), 82, malformed).expect("malformed recursive archive retains admission owner");
    let malformed_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 82, &mut cx)).await;
    assert_eq!(malformed_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 82).expect("malformed recursive archive acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after malformed parent"), persisted);

    let mut malformed_history = persisted.clone();
    let mut history = Box::pin(protocol::decode_history(
        &malformed_history.parent_spr,
        &protocol::os_spr::history::DecodeOptions::default(),
    ))
        .await
        .expect("decode recursive archive history fixture");
    history.transitions.push(protocol::HistoryTransitionRecord {
        id: "malformed-after-valid-prefix".into(),
        actor: "missing-edit-after-valid-prefix".into(),
        hlt: (1, 1, 0),
        dependencies: Vec::new(),
        observed: None,
        payload: vec![0xff],
    });
    malformed_history.parent_spr = Box::pin(protocol::encode_history(
        &history,
        &protocol::os_spr::history::EncodeOptions {
            write_backwards_section: true,
            ..protocol::os_spr::history::EncodeOptions::default()
        },
    ))
    .await
    .expect("encode semantically malformed retained history");
    PluginApp::begin_document_archive_load(app.as_mut(), 85, malformed_history).expect("semantic recursive archive refusal retains admission owner");
    let malformed_history_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 85, &mut cx)).await;
    assert_eq!(malformed_history_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 85).expect("semantic recursive archive refusal acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after semantic history refusal"), persisted);

    let mut malformed_child_history = persisted.clone();
    Box::pin(append_malformed_transition_to_member_history(&mut malformed_child_history.members[0])).await;
    PluginApp::begin_document_archive_load(app.as_mut(), 86, malformed_child_history).expect("semantic recursive child refusal retains admission owner");
    let malformed_child_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 86, &mut cx)).await;
    assert_eq!(malformed_child_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 86).expect("semantic recursive child refusal acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after semantic child history refusal"), persisted);

    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef {
        artifact_id: "child-1".into(),
        dialect: test_child_dialect().await,
    };
    let foreign = Box::pin(recursive_parent_document_files(
        "foreign-schema-parent",
        "semio.foreign-composed-test/v1",
        parent_dialect,
        ComposedParentSnapshot { slot: Some(ParentFixtureChild::new("child-1".into(), child_reference)), revision: 0 },
    ))
    .await;
    let mut refused = persisted.clone();
    refused.parent_pack = foreign.pack;
    refused.parent_spr = foreign.spr;
    PluginApp::begin_document_archive_load(app.as_mut(), 83, refused).expect("foreign recursive archive retains admission owner");
    let refused_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 83, &mut cx)).await;
    assert_eq!(refused_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 83).expect("foreign recursive archive acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after foreign parent"), persisted);
    close_recursive_replacement_app(app.as_mut(), &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🧭️ One poll spends up to `DOCUMENT_ARCHIVE_POLL_WALL_US` driving the same rotation, so whether it
/// comes back still `Running` or already `Cancelled` is a scheduling coincidence, not a product
/// property — a quiet machine fits more bounded steps into that budget than a loaded one, and this
/// clause used to demand the loaded outcome. What the law is named for is the ORDER: a cancelled
/// archive never publishes or faults, and it is not acknowledgeable until its exact input owner has
/// been retired. Both are asserted directly.
#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let archive = recursive_document_archive("cancelled-archive-parent").await;
    let mut app = live_recursive_replacement_app().await;
    let previous = app.stores.store.envelope().id.clone();
    PluginApp::begin_document_archive_load(app.as_mut(), 84, archive).expect("recursive archive cancellation admission");
    assert!(PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).is_err());
    PluginApp::cancel_document_archive_load(app.as_mut(), 84).expect("recursive archive cancellation request");
    let requested = Box::pin(PluginApp::poll_document_archive_load(app.as_mut(), 84, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut cx)).await.expect("recursive archive cancellation request status");
    assert!(!matches!(requested.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Fault), "a cancelled recursive archive published or faulted: {:?}", requested.state);
    if requested.state != protocol::DocumentArchiveLoadState::Cancelled {
        assert!(PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).is_err(), "a cancelled archive short of its terminal status was acknowledgeable");
    }
    let status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 84, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Cancelled);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).expect("recursive archive cancelled acknowledgement");
    assert_eq!(app.stores.store.envelope().id, previous);
    close_recursive_replacement_app(app.as_mut(), &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

async fn verify_recursive_rejection_case(id: &'static str, operation: u64, cx:&mut semio_framework_job::StepContext<'_>) {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, id);
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let old_content_generation = app.child_composition.child_content_generation;
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, operation, &format!("rejected-{id}"), row["mode"].as_str().expect("recursive rejection mode"), cx)).await;
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, cx)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, cx)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault, "{id}");
    assert_eq!(app.stores.store.envelope().id, old_parent_id, "{id}");
    assert_eq!(app.child_composition.child_content_root.slots(), vec![("slot".into(), "child-1".into())], "{id}");
    assert_eq!(app.child_composition.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("rejection retained live recursive branch").count, 0, "{id}");
    assert_eq!(app.child_composition.child_content_generation, old_content_generation, "{id}");
    assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation, "{id}");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded rejected replacement acknowledgement"), "{id}");
    close_recursive_replacement_app(app.as_mut(), cx);
    assert_eq!(row["published"], false, "{id}");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_rejects_missing_extra_and_duplicate_members_after_bounded_retirement() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    for (offset, id) in ["missing-grandchild", "extra-unreachable-member", "duplicate-member"].into_iter().enumerate() {
        Box::pin(verify_recursive_rejection_case(id, 750 + offset as u64, &mut cx)).await;
    }

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

async fn verify_recursive_cancellation_case(id: &'static str, operation: u64, cx:&mut semio_framework_job::StepContext<'_>) {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, id);
    let target = match row["cancelAt"].as_str().expect("recursive cancellation phase") {
        "memberOpen" => semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::OpeningMembers,
        "closure" => semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::ValidatingClosure,
        "viewPreparation" => semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::PreparingCandidateViews,
        other => panic!("unknown recursive cancellation phase {other}"),
    };
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let old_content_generation = app.child_composition.child_content_generation;
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, operation, &format!("cancelled-{id}"), row["mode"].as_str().expect("recursive cancellation mode"), cx)).await;
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, cx)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    Box::pin(drive_recursive_replacement_to(&mut app, handle, target, cx)).await;
    if target == semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::OpeningMembers {
        for _ in 0..semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES {
            if app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()) {
                break;
            }
            let _ = drive_recursive_replacement_step(&mut app, cx).expect("begin recursive member open before cancellation");
        }
        assert!(app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()));
    }
    app.cancel_artifact_store_replacement(handle).expect("cancel recursive retained replacement");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, cx)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Cancelled, "{id}");
    assert_eq!(app.stores.store.envelope().id, old_parent_id, "{id}");
    assert_eq!(app.child_composition.child_content_root.slots(), vec![("slot".into(), "child-1".into())], "{id}");
    assert_eq!(app.child_composition.child_content_generation, old_content_generation, "{id}");
    assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation, "{id}");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded cancelled replacement acknowledgement"), "{id}");
    close_recursive_replacement_app(app.as_mut(), cx);
}

async fn verify_recursive_stale_authority_case( cx:&mut semio_framework_job::StepContext<'_>) {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, "stale-child-content-authority");
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let old_content_generation = app.child_composition.child_content_generation;
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, 769, "stale-recursive-parent", row["mode"].as_str().expect("recursive stale mode"), cx)).await;
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, cx)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::CandidateReady, cx)).await;
    let generation = app.child_composition.admit_child_content_publication().expect("real concurrent child-content publication authority");
    app.child_composition.publish_child_content_member(generation, "slot", "child-1").await.expect("real concurrent child-content publication");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, cx)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.stores.store.envelope().id, old_parent_id);
    assert_eq!(app.child_composition.child_content_root.slots(), vec![("slot".into(), "child-1".into())]);
    assert_eq!(app.child_composition.child_content_generation, old_content_generation + 1);
    assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded stale replacement acknowledgement"));
    close_recursive_replacement_app(app.as_mut(), cx);
    assert_eq!(row["staleAuthority"], true);
    assert_eq!(row["published"], false);
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_cancellation_and_stale_authority_preserve_the_live_bundle() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    for (offset, id) in ["cancel-member-open", "cancel-closure", "cancel-view-preparation"].into_iter().enumerate() {
        Box::pin(verify_recursive_cancellation_case(id, 760 + offset as u64, &mut cx)).await;
    }
    Box::pin(verify_recursive_stale_authority_case( &mut cx)).await;

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}
//#endregion 🧬️ComposedParentFixture
