// 📦️ Document archive and replacement publication: the recursive document archive, the retained recursive replacement fixture, the maintenance idle answer and the composed replacement publication.

//#region 🧬️ComposedParentFixture
async fn recursive_document_archive(parent_id: &str) -> protocol::DocumentArchivePack {
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let root_reference = ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect.clone() };
    let branch_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let leaf_reference = ArtifactRef { artifact_id: "grandchild-1".into(), dialect: child_dialect };
    let branch_owner = store::OwnerRef { parent: root_reference, slot: "slot".into(), child_id: branch_reference.artifact_id.clone() };
    let leaf_owner = store::OwnerRef { parent: branch_reference.clone(), slot: "nested".into(), child_id: leaf_reference.artifact_id.clone() };
    let branch_snapshot = RecursiveBranchSnapshot {
        count: 23,
        label: "archive-branch".into(),
        nested: Some(RecursiveFixtureChild::new(leaf_reference.artifact_id.clone(), leaf_reference.clone())),
    };
    let leaf_snapshot = RecursiveBranchSnapshot { count: 47, label: "archive-leaf".into(), nested: None };
    let parent_snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new(branch_reference.artifact_id.clone(), branch_reference.clone())), revision: 0 };
    let parent = Box::pin(recursive_parent_document_files(parent_id, ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_dialect, parent_snapshot)).await;
    let branch_envelope = Box::pin(recursive_member_envelope_with_history(&branch_reference, &branch_owner, &branch_snapshot.encode_pack(), 29)).await;
    let leaf_envelope = Box::pin(recursive_member_envelope_with_history(&leaf_reference, &leaf_owner, &leaf_snapshot.encode_pack(), 53)).await;
    protocol::DocumentArchivePack {
        parent_pack: parent.pack,
        parent_spr: parent.spr,
        members: vec![
            protocol::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: recursive_archive_reference(&branch_reference),
                owner: protocol::DocumentArchiveOwnerRef { parent: recursive_archive_reference(&branch_owner.parent), slot: branch_owner.slot, child_id: branch_owner.child_id },
                envelope_pack: branch_envelope,
            },
            protocol::OwnedDocumentMemberPackEntry {
                ordinal: 1,
                reference: recursive_archive_reference(&leaf_reference),
                owner: protocol::DocumentArchiveOwnerRef { parent: recursive_archive_reference(&leaf_owner.parent), slot: leaf_owner.slot, child_id: leaf_owner.child_id },
                envelope_pack: leaf_envelope,
            },
        ],
    }
}

async fn drive_recursive_document_archive_load(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    operation: u64, cx:&mut semio_framework_job::StepContext<'_>) -> protocol::DocumentArchiveLoadStatus {
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(app, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx)).await.expect("recursive archive operation status");
        if matches!(
            status.state,
            protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault
        ) {
            return status;
        }
        let _ = composed_maintenance_turn(app, store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES, cx).expect("recursive archive public maintenance step");
        semio_framework_async::yield_once().await;
    }
    panic!("recursive archive operation exceeded its public maintenance progress authority")
}

async fn retained_recursive_replacement_fixture(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    operation: u64,
    parent_id: &str,
    mode: &str, cx:&mut semio_framework_job::StepContext<'_>) -> (semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle, Vec<semio_framework_plugin_child::OwnedDocumentMemberIngress>) {
    let generation = semio_framework_job::Generation(app.stores.store.generation_now());
    let child_dialect = test_child_dialect().await;
    let leaf_dialect = child_dialect.clone();
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let branch_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let leaf_reference = ArtifactRef { artifact_id: "grandchild-1".into(), dialect: leaf_dialect.clone() };
    let root_reference = ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect.clone() };
    let snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new(branch_reference.artifact_id.clone(), branch_reference.clone())), revision: 0 };
    let mut envelope = store::os_store::document_materialize::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(parent_dialect);
    let mut candidate = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, app.stores.store.local_actor_id().clone()).await.expect("recursive candidate parent store");
    store::install_unscheduled_catalog(&mut candidate, funded_recursive_fixture_owners::<ComposedParentSnapshot>("recursive-parent")).expect("recursive candidate parent catalog installs");
    let job = semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::from_candidate(candidate)));
    let operation = semio_framework_job::OperationId(operation);
    app.ingest.store_replacement_jobs.insert_admitted(
        operation.0,
        semio_framework_plugin_document_media::ActiveArtifactStoreReplacement::new(operation, generation, app.child_composition.child_content_generation, job, (cx.operation(),cx.generation())),
    );

    let branch_owner = store::OwnerRef { parent: root_reference.clone(), slot: "slot".into(), child_id: branch_reference.artifact_id.clone() };
    let leaf_owner = store::OwnerRef { parent: branch_reference.clone(), slot: "nested".into(), child_id: leaf_reference.artifact_id.clone() };
    let branch_snapshot = RecursiveBranchSnapshot {
        count: 23,
        label: "replacement-branch".into(),
        nested: Some(RecursiveFixtureChild::new(leaf_reference.artifact_id.clone(), leaf_reference.clone())),
    };
    let leaf_snapshot = RecursiveBranchSnapshot { count: 47, label: "replacement-leaf".into(), nested: None };
    let mut ingress = Vec::new();
    ingress.push(recursive_member_ingress(operation, generation, 0, branch_reference.clone(), branch_owner.clone(), branch_snapshot.encode_pack()).await);
    match mode {
        "complete" => ingress.push(recursive_member_ingress(operation, generation, 1, leaf_reference, leaf_owner, leaf_snapshot.encode_pack()).await),
        "missing" => {}
        "extra" => {
            ingress.push(recursive_member_ingress(operation, generation, 1, leaf_reference, leaf_owner, leaf_snapshot.encode_pack()).await);
            let extra_reference = ArtifactRef { artifact_id: "orphan-1".into(), dialect: leaf_dialect };
            let extra_owner = store::OwnerRef { parent: root_reference, slot: "orphan".into(), child_id: extra_reference.artifact_id.clone() };
            ingress.push(recursive_member_ingress(operation, generation, 2, extra_reference, extra_owner, RecursiveBranchSnapshot { count: 59, label: "unreachable".into(), nested: None }.encode_pack()).await);
        }
        "duplicate" => ingress.push(recursive_member_ingress(operation, generation, 1, branch_reference, branch_owner, branch_snapshot.encode_pack()).await),
        other => panic!("unknown recursive replacement fixture mode {other}"),
    }
    (semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle { operation, generation }, ingress)
}

fn admit_recursive_member_set(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    handle: semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle,
    row: &Value,
    ingress: Vec<semio_framework_plugin_child::OwnedDocumentMemberIngress>,
) {
    assert!(app.try_begin_owned_document_members(handle, ingress.len(), u64::MAX).expect("recursive member ingress registry"));
    let mut ingress = ingress.into_iter().map(Some).collect::<Vec<_>>();
    for ordinal in row["admissionOrder"].as_array().expect("recursive admission order") {
        let ordinal = ordinal.as_u64().expect("recursive admission ordinal") as usize;
        let member = ingress.get_mut(ordinal).and_then(Option::take).expect("recursive admission ordinal is unique and in range");
        app.admit_owned_document_member(handle, member).unwrap_or_else(|_| panic!("recursive exact candidate member admission"));
    }
    assert!(ingress.iter().all(Option::is_none));
    app.seal_owned_document_members(handle).expect("recursive complete candidate member set");
}

fn drive_recursive_replacement_step(app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    composed_maintenance_turn(app, 4096, cx)
}

async fn drive_recursive_replacement_to(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    handle: semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle,
    target: semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState, cx:&mut semio_framework_job::StepContext<'_>) {
    for _ in 0..100_000 {
        if app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == target) {
            return;
        }
        if let Err(source)=drive_recursive_replacement_step(app,cx){close_composition_refusal(source,cx)}
        semio_framework_async::yield_once().await;
    }
    let active = app.ingest.store_replacement_jobs.get(handle.operation.0);
    panic!("recursive retained replacement did not reach {target:?}; state {:?} refusal {:?}", active.map(|active| active.state), active.and_then(|active| active.refusal_fault()));
}

async fn live_recursive_replacement_app() -> Box<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>> {
    let mut app = Box::pin(VcsArtifactApp::<ComposedParentApp, RecursiveTestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let dialect = test_child_dialect().await;
    let member = Box::pin(RecursiveTestMembers::create("child-1", &dialect, &RecursiveBranchSnapshot { count: 0, label: "live".into(), nested: None }.encode_pack(), app.stores.store.local_actor_id().clone())).await.expect("live recursive child");
    Box::pin(app.register_child("slot", "child-1", dialect, member, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await.expect("live recursive child publication");
    Box::new(app)
}

/// 🎡️ `maintenance_step` answers its idle `Pending { 0, 0 }` ONLY when every stage of the fixed
/// rotation is idle. The rotation runs one bounded unit per stage; when an EMPTY stage also consumed
/// the whole call, a caller that drives one maintenance unit per turn released one owner per
/// `MAINTENANCE_STAGES` turns — measured on the assembled `s.flow.flow@1/*#editor` surface as 2_051
/// items in 100_000 close turns, 96_588 of them releasing nothing — and a real editor could not finish
/// closing inside any committed turn budget. This law prices the guarantee without a wall-clock or
/// call-count ceiling: while a displaced child root is still queued, no call may answer idle.
#[semio_framework_async_macros::async_test]
async fn maintenance_answers_idle_only_when_every_stage_is_idle() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let generation = app.child_composition.admit_child_content_publication().expect("child content publication authority");
    app.child_composition.publish_child_content_member(generation, "slot", "child-1").await.expect("second child content generation");
    assert!(!app.child_composition.child_content_retirements.is_empty(), "a second child-content publication queues its displaced root");
    for call in 0..4_096 {
        let grant = COMPOSITION_MAINTENANCE_GRANT;
        match app.maintenance_step(&mut cx,grant).expect("bounded maintenance call") {
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) if progress == semio_framework_value::retained_clone::RetainedCloneProgress::default() => {
                assert!(app.child_composition.child_content_retirements.is_empty(), "maintenance answered idle on call {call} while a displaced child root was still queued");
                close_member_admission_app(&mut app, &mut cx);
                return;
            }
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant), "maintenance call {call} released {progress:?} against its quoted grant {grant:?}");
            }
            step => panic!("maintenance call {call} answered {step:?} while draining one displaced child root"),
        }
    }
    panic!("maintenance never reached its idle answer while draining one displaced child root");

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.stores.store.envelope().id.clone();
    let child_dialect = test_child_dialect().await;
    app.child_composition.pending_child_pins.push(vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect }, checkpoint_id: "old-checkpoint".into() });
    let old_window_generation = app.stores.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 701, "replacement-parent", 23, &mut cx).await;
    drive_composed_replacement_to(&mut app, handle, semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState::AwaitingMembers, &mut cx).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("member ingress registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("exact candidate member admission"));
    app.seal_owned_document_members(handle).expect("complete candidate member set");

    for _ in 0..100_000 {
        let committed = app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.committed);
        if committed {
            assert_eq!(app.stores.store.envelope().id, "replacement-parent");
            assert_eq!(app.child_composition.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("new content view").count, 23);
            assert_eq!(app.child_composition.composition.graph_mut().await.owner_of("child-1").await, Some("replacement-parent"));
            assert!(app.child_composition.pending_child_pins.is_empty());
            assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation + 1);
        } else {
            assert_eq!(app.stores.store.envelope().id, old_parent_id);
            assert_eq!(app.child_composition.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("old content view").count, 0);
            assert_eq!(app.stores.window_transient_store.document_generation(), old_window_generation);
        }
        if app.poll_artifact_store_replacement(handle) == semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        app.maintenance.maintenance_stage = 14;
        let _ = composed_maintenance_turn(&mut app, 4096, &mut cx).expect("atomic composed replacement drive");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(app.poll_artifact_store_replacement(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("terminal replacement acknowledgement"));
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}
//#endregion 🧬️ComposedParentFixture
