// 🏁️ Replacement close, cancellation and nested window: composed replacement cancellation, close query, terminal store box, candidate projection, live child change and the nested recursive window input.

//#region 🧬️ComposedParentFixture
#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    for (case, target) in [
        ("member-open", semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::OpeningMembers),
        ("closure", semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::ValidatingClosure),
        ("views", semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::PreparingCandidateViews),
    ] {
        let mut app = live_composed_replacement_app().await;
        let old_parent_id = app.stores.store.envelope().id.clone();
        let old_window_generation = app.stores.window_transient_store.document_generation();
        let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 710 + case.len() as u64, &format!("cancel-{case}"), 31, &mut cx).await;
        drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;
        assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("cancel member registry"));
        app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("cancel candidate ingress"));
        app.seal_owned_document_members(handle).expect("cancel candidate seal");
        drive_composed_replacement_to(&mut app, handle, target, &mut cx).await;
        if case == "member-open" {
            app.maintenance.maintenance_stage = 14;
            let _ = composed_maintenance_turn(&mut app, 4096, &mut cx).expect("begin exact member open before cancellation");
            assert!(app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()));
        }
        app.cancel_artifact_store_replacement(handle).expect("cancel retained composed replacement");
        drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, &mut cx).await;
        assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Cancelled, "{case}");
        assert_eq!(app.stores.store.envelope().id, old_parent_id, "{case}");
        assert_eq!(app.child_composition.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0, "{case}");
        assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation, "{case}");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("cancel acknowledgement"));
        close_member_admission_app(&mut app, &mut cx);
    }

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_close_query_forwards_whole_member_page_extent() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/🧫️fixtures/📏️replacement-demand/🔣️.json")).expect("closed neutral replacement demand");
    let ordinary = fixture["ordinaryGrantBytes"].as_u64().unwrap() as usize;
    assert_eq!(ordinary,COMPOSITION_PAGE_GRANT.maximum_release_bytes);
    let extent = fixture["expectedCloseDemandBytes"].as_u64().unwrap() as usize;
    let ceiling = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    assert_eq!(ceiling,COMPOSITION_CLEANUP_GRANT.maximum_release_bytes);
    let mut app = live_composed_replacement_app().await;
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 799, "exact-page-close", 31, &mut cx).await;
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("member ingress registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("exact member ingress"));
    app.seal_owned_document_members(handle).expect("sealed member set");
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::OpeningMembers, &mut cx).await;
    app.maintenance.maintenance_stage = 14;
    composed_maintenance_turn(&mut app, ordinary, &mut cx).expect("begin retained member open");
    app.cancel_artifact_store_replacement(handle).expect("cancel candidate");
    for _ in 0..100_000 {
        if app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::RetiringRejectedMembers && active.active_member_open.as_ref().is_some_and(|open| store::MemberOpenOperation::next_release_byte_demand(open).is_ok_and(|bytes| bytes == extent))) { break; }
        app.close_step(&mut cx,COMPOSITION_PAGE_GRANT).expect("close prior owners and bounded logical request retirement"); let _ = app.take_constructor_receipt();
        semio_framework_async::yield_once().await;
    }
    let demand = app.ingest.store_replacement_jobs.get(handle.operation.0).and_then(|active| active.active_member_open.as_ref()).map(|open| store::MemberOpenOperation::next_release_byte_demand(open).expect("retained member release quote")).expect("retained member operation");
    let denied = app.close_step(&mut cx,COMPOSITION_PAGE_GRANT).expect("physical undergrant retains owner"); let _ = app.take_constructor_receipt();
    let demand_after_denial = app.ingest.store_replacement_jobs.get(handle.operation.0).and_then(|active| active.active_member_open.as_ref()).map(|open| store::MemberOpenOperation::next_release_byte_demand(open).expect("denied member release quote")).expect("denied owner remains retained");
    let caller_demand = app.close_retirement_demands(ordinary).expect("caller close quote").release_bytes;
    let mut terminal = false;
    for _ in 0..100_000 {
        let step = app.close_step(&mut cx,COMPOSITION_CLEANUP_GRANT).expect("explicit admitted cleanup extent"); let _ = app.take_constructor_receipt();
        if matches!(step, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)) { terminal = true; break; }
        semio_framework_async::yield_once().await;
    }
    assert!(terminal && PluginApp::close_terminal_is_empty(&app));
    assert_eq!(demand, extent);
    assert_eq!(demand_after_denial, extent);
    assert!(matches!(denied, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress.released_bytes == 0) || matches!(denied, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { .. }));
    assert_eq!(caller_demand, extent);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_terminal_store_box_requires_whole_physical_grant() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/🧫️fixtures/📏️replacement-terminal/🔣️.json")).expect("closed neutral terminal frame");
    let ceiling = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    assert_eq!(ceiling,COMPOSITION_CLEANUP_GRANT.maximum_release_bytes);
    let mut app = live_composed_replacement_app().await;
    let handle = composed_replacement_candidate(&mut app, 798, "terminal-box-close", 0, &mut cx).await;
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;
    app.cancel_artifact_store_replacement(handle).expect("cancel candidate before member ingress");
    let mut extent = None;
    for _ in 0..100_000 {
        if let Some(active) = app.ingest.store_replacement_jobs.get(handle.operation.0) {
            if let Some(bytes) = active.terminal_store_frame_bytes() {
                extent = Some(bytes);
                break;
            }
        }
        app.close_step(&mut cx,COMPOSITION_CLEANUP_GRANT).expect("close exact preceding candidate owners"); let _ = app.take_constructor_receipt();
        semio_framework_async::yield_once().await;
    }
    let extent = extent.expect("terminal candidate store frame remains retained");
    assert!(extent > 0 && extent <= ceiling);
    let before_demand = app.close_retirement_demands(extent).expect("terminal frame close quote").release_bytes;
    let (denied, denied_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(&mut cx,store::RetainedCloneGrant{maximum_release_bytes:extent-1,..COMPOSITION_CLEANUP_GRANT}).expect("terminal frame one below")); let _ = app.take_constructor_receipt();
    let retained = app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.terminal_store_frame_bytes() == Some(extent));
    let exact_demand = app.close_retirement_demands(extent).expect("terminal frame close quote after the undergrant").release_bytes;
    let (funded, funded_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(&mut cx,store::RetainedCloneGrant{maximum_release_bytes:extent,..COMPOSITION_CLEANUP_GRANT}).expect("terminal frame exact funding")); let _ = app.take_constructor_receipt();
    let mut terminal = false;
    for _ in 0..100_000 {
        let step = app.close_step(&mut cx,COMPOSITION_CLEANUP_GRANT).expect("complete separately admitted app cleanup"); let _ = app.take_constructor_receipt();
        if matches!(step, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)) { terminal = true; break; }
        semio_framework_async::yield_once().await;
    }
    assert!(terminal && PluginApp::close_terminal_is_empty(&app));
    assert_eq!(exact_demand, extent);
    assert!(retained);
    assert_eq!((denied_heap.requested_bytes, denied_heap.released_bytes), (0, 0));
    assert!(matches!(denied, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress.copied_items <= 1 && progress.released_bytes == 0));
    assert_eq!((funded_heap.requested_bytes, funded_heap.released_bytes), (0, extent));
    assert!(matches!(funded, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress.copied_items == 1 && progress.released_bytes == extent));

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🧩️ A candidate parent's child projection is the APP's answer, never the structural one.
///
/// The replacement pump used to call `semio_framework_composition_admission::ChildRestoreProjection::from_snapshot` on the candidate
/// and `map_err` the `ChildRestoreProjectionError` into one flat `candidate parent child projection is
/// invalid`, so a live-load refusal was unattributable from the plugin side: gismap's
/// `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` failed with that
/// string while its own projection law was green over every snapshot the crate can build (ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, engineering §9). This law pins the seam from the far
/// side: the fixture's snapshot is the very one the sibling replacement laws drive to publication, so
/// the structural projection accepts it; only `ComposedParentApp`'s own projection refuses this
/// revision, and a pump that bypassed the app would answer progress here instead of a fault.
#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_candidate_projection_is_the_apps_own_answer() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let handle = composed_replacement_candidate(&mut app, 741, "app-projection-parent", COMPOSED_PARENT_PROJECTION_REFUSAL_REVISION, &mut cx).await;
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;

    app.maintenance.maintenance_stage = 14;
    let fault = composed_maintenance_turn(&mut app, 4096, &mut cx).expect_err("the app's refusal must reach the caller, never be answered as progress");
    let semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(original)=&fault.primary else{panic!("original app diagnostic remains the primary")};
    assert_eq!(original.code.0, "test.parent-projection", "the app's own fault code survives the pump");
    assert_eq!(original.message, COMPOSED_PARENT_PROJECTION_REFUSAL_MESSAGE, "the app's own diagnostic survives the pump, not a flat framework string");
    close_composition_refusal(fault,&mut cx);

    app.cancel_artifact_store_replacement(handle).expect("cancel the refused candidate");
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, &mut cx).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Cancelled);
    assert_eq!(app.stores.store.envelope().id, old_parent_id, "a refused candidate never displaces the live parent");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("refused candidate acknowledgement"));
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 729, "stale-parent", 47, &mut cx).await;
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("stale member registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("stale candidate ingress"));
    app.seal_owned_document_members(handle).expect("stale candidate seal");
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::CandidateReady, &mut cx).await;
    let generation = app.child_composition.admit_child_content_publication().expect("real live child publication authority");
    app.child_composition.publish_child_content_member(generation, "slot", "child-1").await.expect("real live child generation change");
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::Complete, &mut cx).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.stores.store.envelope().id, old_parent_id);
    assert_eq!(app.child_composition.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0);
    assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("stale replacement acknowledgement"));
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_publishes_the_complete_nested_bundle_atomically() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, "recursive-unordered-success");
    let mut app = live_recursive_replacement_app().await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let old_content_generation = app.child_composition.child_content_generation;
    let child_dialect = test_child_dialect().await;
    app.child_composition.pending_child_pins.push(vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect }, checkpoint_id: "displaced-checkpoint".into() });
    let parent_id = vectors["root"]["artifactId"].as_str().expect("recursive replacement root id");
    let (handle, ingress) = retained_recursive_replacement_fixture(&mut app, 741, parent_id, row["mode"].as_str().expect("recursive replacement mode"), &mut cx).await;
    eprintln!(
        "recursive replacement success: admitting complete member set active={} app={} ingress={}",
        std::mem::size_of::<semio_framework_plugin_document_media::ActiveArtifactStoreReplacement<ComposedParentSnapshot, RecursiveFixtureMutation, RecursiveTestMembers>>(),
        std::mem::size_of::<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>>(),
        std::mem::size_of::<semio_framework_plugin_child::OwnedDocumentMemberIngress>(),
    );
    Box::pin(drive_recursive_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);

    for _ in 0..100_000 {
        let committed = app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.committed);
        if committed {
            assert_eq!(app.stores.store.envelope().id, parent_id);
            assert_eq!(app.child_composition.child_content_root.keys().len(), 2);
            assert_eq!(app.child_composition.child_content_root.slots(), vec![("slot".into(), "child-1".into())], "the document itself owns the branch alone");
            assert_eq!(app.child_composition.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("published recursive branch").count, 23);
            assert_eq!(app.child_composition.child_content_root.typed_read_at::<RecursiveBranchSnapshot>(MemberKeyRef { owner: "child-1", slot: "nested", child_id: "grandchild-1" }).expect("published recursive leaf").count, 47);
            assert_eq!(app.child_composition.composition.graph_mut().await.owner_of("child-1").await, Some(parent_id));
            assert_eq!(app.child_composition.composition.graph_mut().await.owner_of("grandchild-1").await, Some("child-1"));
            assert!(app.child_composition.pending_child_pins.is_empty());
            assert_eq!(app.child_composition.child_content_generation, old_content_generation + 1);
            assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation + 1);
        } else {
            assert_eq!(app.stores.store.envelope().id, old_parent_id);
            assert_eq!(app.child_composition.child_content_root.slots(), vec![("slot".into(), "child-1".into())]);
            assert_eq!(app.child_composition.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("retained live recursive branch").count, 0);
            assert!(app.child_composition.child_content_root.dialect_at(MemberKeyRef { owner: "child-1", slot: "nested", child_id: "grandchild-1" }).is_none());
            assert_eq!(app.child_composition.child_content_generation, old_content_generation);
            assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation);
        }
        if app.poll_artifact_store_replacement(handle) == semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        let _ = drive_recursive_replacement_step(&mut app, &mut cx).expect("atomic recursive replacement drive");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready);
    let packs = PluginApp::child_packs(app.as_ref()).await.expect("published recursive member registry packs");
    assert_eq!(packs.iter().map(|entry| entry.child_id.as_str()).collect::<std::collections::BTreeSet<_>>(), std::collections::BTreeSet::from(["child-1", "grandchild-1"]));
    assert!(app.child_composition.test_child_admission_state(1).roots_retiring_empty);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("recursive terminal replacement acknowledgement"));
    close_recursive_replacement_app(app.as_mut(), &mut cx);
    assert_eq!(row["published"], true);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}
//#endregion 🧬️ComposedParentFixture
