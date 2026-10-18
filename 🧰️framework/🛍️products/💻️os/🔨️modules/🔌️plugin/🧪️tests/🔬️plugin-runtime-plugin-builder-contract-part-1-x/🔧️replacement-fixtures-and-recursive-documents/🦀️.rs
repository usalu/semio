// 🔧️ Replacement fixtures and recursive documents: the composed and recursive replacement fixtures, drivers and closers, the member envelopes, the member history and the parent document files.

//#region 🧬️ComposedParentFixture
async fn retained_composed_replacement_fixture(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    parent_id: &str,
    child_count: i32, cx:&mut semio_framework_job::StepContext<'_>) -> (semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle, semio_framework_plugin_child::OwnedDocumentMemberIngress) {
    retained_composed_replacement_fixture_at_revision(app, operation, parent_id, child_count, 0, cx).await
}

async fn retained_composed_replacement_fixture_at_revision(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    parent_id: &str,
    child_count: i32,
    parent_revision: i32, cx:&mut semio_framework_job::StepContext<'_>) -> (semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle, semio_framework_plugin_child::OwnedDocumentMemberIngress) {
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let handle = composed_replacement_candidate(app, operation, parent_id, parent_revision, cx).await;
    let generation = handle.generation;
    let operation = handle.operation;

    let owner = store::OwnerRef {
        parent: ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect },
        slot: "slot".into(),
        child_id: "child-1".into(),
    };
    let mut member = TestMembers::create("child-1", &child_dialect, &TestSnapshot { count: child_count, label: "replacement".into(), slot: Vec::new() }.encode_pack(), protocol::ActorId("actor:composition-opening-fixture".into())).await.expect("candidate child");
    member.set_owner(Some(owner.clone())).await;
    let bytes = member.envelope_pack_bytes().await.expect("candidate child full envelope");
    close_member_admission_fixture(&mut member);
    let chunks = bytes.chunks(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes.len() }).expect("candidate member page credits");
    for chunk in chunks {
        pages.admit_page(store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded member page")).unwrap_or_else(|_| panic!("pre-admitted candidate member page"));
    }
    pages.seal().expect("candidate member page set");
    let request = store::MemberOpenRequest::new(operation, generation, u64::MAX, child_reference.clone(), Some(owner.clone()), pages, protocol::ActorId("actor:composition-opening-fixture".into())).admit(1).unwrap_or_else(|_| panic!("candidate member request"));
    let ingress = semio_framework_plugin_child::OwnedDocumentMemberIngress::try_new(0, child_reference, owner, request).unwrap_or_else(|_| panic!("candidate member ingress"));
    (handle, ingress)
}

/// 🧬️ The candidate parent half of a retained replacement alone — no member ingress, so a law that
/// never reaches member admission leaves no ingress owner to hand off.
async fn composed_replacement_candidate(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    parent_id: &str,
    parent_revision: i32, cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle {
    let generation = semio_framework_job::Generation(app.stores.store.generation_now());
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect };
    let snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new("child-1".into(), child_reference)), revision: parent_revision };
    let mut envelope = store::os_store::document_materialize::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(parent_dialect);
    let mut candidate = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, app.stores.store.local_actor_id().clone()).await.expect("candidate parent store");
    store::install_unscheduled_catalog(&mut candidate, funded_recursive_fixture_owners::<ComposedParentSnapshot>("recursive-parent")).expect("candidate parent catalog installs");
    let job = semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::from_candidate(candidate)));
    let operation = semio_framework_job::OperationId(operation);
    app.ingest.store_replacement_jobs.insert_admitted(operation.0, semio_framework_plugin_document_media::ActiveArtifactStoreReplacement::new(operation, generation, app.child_composition.child_content_generation, job, (cx.operation(),cx.generation())));
    semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle { operation, generation }
}

/// 🎟️ The fixture lends its authored maintenance policy without deriving authority from quotes.
fn composed_maintenance_turn<A: PluginApp>(app: &mut A, body: usize, cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    assert_eq!(body,COMPOSITION_MAINTENANCE_GRANT.maximum_copy_bytes);
    app.maintenance_step(cx,COMPOSITION_MAINTENANCE_GRANT)
}

/// ♻️ Returns the complete original fixture refusal through the initiating physical recipient.
fn close_composition_refusal(source:semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure,cx:&mut semio_framework_job::StepContext<'_>){
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(source).unwrap_or_else(|_|panic!("complete original lifecycle refusal supports controlled retirement"));
 for _ in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){return}cx.with_retained_loan(COMPOSITION_CLEANUP_GRANT,|same|semio_framework_job::close_original_retained_step(same,owner.selected_demand(COMPOSITION_CLEANUP_GRANT.maximum_copy_bytes)?,|grant|owner.step(grant))).expect("original composition refusal paid retirement");}
 panic!("original composition refusal exceeded its initiating supplied turn bound")
}

async fn drive_composed_replacement_to(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    handle: semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle,
    target: semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState, cx:&mut semio_framework_job::StepContext<'_>) {
    for _ in 0..100_000 {
        if app.ingest.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == target) {
            return;
        }
        app.maintenance.maintenance_stage = 14;
        let _ = composed_maintenance_turn(app, 4096, cx).expect("retained composed replacement step");
        semio_framework_async::yield_once().await;
    }
    let active = app.ingest.store_replacement_jobs.get(handle.operation.0);
    panic!("retained composed replacement did not reach {target:?}; state {:?} refusal {:?}", active.map(|active| active.state), active.and_then(|active| active.refusal_fault()));
}

async fn live_composed_replacement_app() -> VcsArtifactApp<ComposedParentApp, TestMembers> {
    let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    let dialect = test_child_dialect().await;
    let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack(), app.stores.store.local_actor_id().clone()).await.expect("live child");
    app.register_child("slot", "child-1", dialect, member, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("live child publication");
    app
}

fn recursive_replacement_vectors() -> Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧩️composition/🔣️.json"))).expect("recursive replacement vectors")
}

fn recursive_replacement_case<'a>(vectors: &'a Value, id: &str) -> &'a Value {
    vectors["cases"].as_array().expect("recursive replacement cases").iter().find(|row| row["id"] == id).expect("recursive replacement case")
}

fn close_recursive_member_fixture(member: &mut RecursiveTestMembers) {
    let grant=COMPOSITION_MAINTENANCE_GRANT;
    for _ in 0..65_536 {
        let step = member.close_owned_step(grant).expect("recursive fixture member closes under its exact grant");
        assert!(step.progress().fits(grant));
        if matches!(step, store::RetainedCloneStep::Complete(_)) {
            assert!(member.close_owned_terminal_is_empty());
            return;
        }
    }
    panic!("recursive fixture member did not close");
}

fn close_recursive_parent_fixture(store: &mut store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>) {
    let grant=COMPOSITION_MAINTENANCE_GRANT;
    for _ in 0..65_536 {
        let step = store.close_owned_step(grant).expect("recursive archive parent closes under its exact grant");
        assert!(step.progress().fits(grant));
        if matches!(step, store::RetainedCloneStep::Complete(_)) {
            assert!(store.close_owned_terminal_is_empty());
            return;
        }
    }
    panic!("recursive archive parent did not close");
}

fn close_recursive_replacement_app(app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>, cx:&mut semio_framework_job::StepContext<'_>) {
    for _ in 0..100_000 {
        let demand = app.close_retirement_demands(4096).expect("original fixture close demand");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            assert!(grant.maximum_copy_bytes <= 4096 && grant.maximum_release_bytes <= 4096, "original fixture body and physical grant limits remain unchanged {demand:?}");
            match { let step = app.close_step(cx,grant).expect("recursive fixture app closes under its exact grant"); let _ = app.take_constructor_receipt(); step } {
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason } => panic!("recursive fixture app close awaited input: {reason}"),
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason } => panic!("recursive fixture app has no external owner: {reason}"),
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant));
                assert!(app.close_terminal_is_empty());
                return;
            }
        }
    }
    panic!("recursive fixture app did not close");
}

async fn recursive_member_envelope(reference: &ArtifactRef, owner: &store::OwnerRef, snapshot_pack: &[u8]) -> Vec<u8> {
    let mut member = Box::pin(RecursiveTestMembers::create(&reference.artifact_id, &reference.dialect, snapshot_pack, protocol::ActorId("actor:composition-opening-fixture".into()))).await.expect("recursive candidate member");
    Box::pin(member.set_owner(Some(owner.clone()))).await;
    let bytes = Box::pin(member.envelope_pack_bytes()).await.expect("recursive candidate full envelope");
    close_recursive_member_fixture(&mut member);
    bytes
}

async fn recursive_member_envelope_with_history(reference: &ArtifactRef, owner: &store::OwnerRef, snapshot_pack: &[u8], value: i32) -> Vec<u8> {
    let mut member = Box::pin(RecursiveTestMembers::create(&reference.artifact_id, &reference.dialect, snapshot_pack, protocol::ActorId("actor:composition-opening-fixture".into()))).await.expect("recursive historical member");
    Box::pin(member.set_owner(Some(owner.clone()))).await;
    match &mut member {
        RecursiveTestMembers::Branch(store) => {
            semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![RecursiveFixtureMutation { value }], transaction: None }, &mut identity)).await)
                .expect("recursive historical member mutation");
        }
    }
    let bytes = Box::pin(member.envelope_pack_bytes()).await.expect("recursive historical member full envelope");
    close_recursive_member_fixture(&mut member);
    bytes
}

fn recursive_member_history_offset(bytes: &[u8]) -> usize {
    let mut value = 0usize;
    let mut shift = 0usize;
    for (index, byte) in bytes.iter().copied().take(10).enumerate() {
        value |= usize::from(byte & 127) << shift;
        if byte & 128 == 0 {
            return index + 1 + value;
        }
        shift += 7;
    }
    panic!("recursive member envelope lacks its bounded snapshot frame")
}

async fn append_malformed_transition_to_member_history(entry: &mut protocol::OwnedDocumentMemberPackEntry) {
    let history_offset = recursive_member_history_offset(&entry.envelope_pack);
    let mut history = Box::pin(protocol::decode_history(
        &entry.envelope_pack[history_offset..],
        &protocol::os_spr::history::DecodeOptions::default(),
    ))
    .await
    .expect("decode recursive member history fixture");
    history.transitions.push(protocol::HistoryTransitionRecord {
        id: "malformed-child-after-valid-prefix".into(),
        actor: "missing-child-edit-after-valid-prefix".into(),
        hlt: (1, 1, 0),
        dependencies: Vec::new(),
        observed: None,
        payload: vec![0xff],
    });
    let encoded = Box::pin(protocol::encode_history(
        &history,
        &protocol::os_spr::history::EncodeOptions {
            write_backwards_section: true,
            ..protocol::os_spr::history::EncodeOptions::default()
        },
    ))
    .await
    .expect("encode semantically malformed retained member history");
    entry.envelope_pack.truncate(history_offset);
    entry.envelope_pack.extend(encoded);
}

async fn recursive_member_ingress(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    ordinal: usize,
    reference: ArtifactRef,
    owner: store::OwnerRef,
    snapshot_pack: Vec<u8>,
) -> semio_framework_plugin_child::OwnedDocumentMemberIngress {
    let bytes = Box::pin(recursive_member_envelope(&reference, &owner, &snapshot_pack)).await;
    let chunks = bytes.chunks(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes.len() }).expect("recursive candidate member page credits");
    for chunk in chunks {
        pages.admit_page(store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(chunk).expect("recursive bounded member page")).unwrap_or_else(|_| panic!("recursive candidate member page admission"));
    }
    pages.seal().expect("recursive candidate member page set");
    let request = store::MemberOpenRequest::new(operation, generation, u64::MAX, reference.clone(), Some(owner.clone()), pages, protocol::ActorId("actor:composition-opening-fixture".into())).admit(1).unwrap_or_else(|_| panic!("recursive candidate member request"));
    semio_framework_plugin_child::OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request).unwrap_or_else(|_| panic!("recursive candidate member ingress"))
}

fn recursive_archive_reference(reference: &ArtifactRef) -> protocol::DocumentArchiveArtifactRef {
    protocol::DocumentArchiveArtifactRef {
        artifact_id: reference.artifact_id.clone(),
        artifact_kind: reference.dialect.artifact_kind.clone(),
        standard: reference.dialect.standard.clone(),
        subset: reference.dialect.subset.clone(),
    }
}

async fn recursive_parent_document_files(
    parent_id: &str,
    schema: &str,
    dialect: ArtifactDialect,
    snapshot: ComposedParentSnapshot,
) -> store::os_store::serialization::ArtifactPackFiles {
    let mut envelope = store::os_store::document_materialize::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(schema, parent_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(dialect);
    let mut parent_store = Box::new(Box::pin(store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("recursive archive parent store"));
    store::install_unscheduled_catalog(&mut parent_store, funded_recursive_fixture_owners::<ComposedParentSnapshot>("recursive-parent")).expect("recursive archive parent catalog installs");
    let generation = parent_store.generation_now();
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(parent_store.apply_one(
        generation,
        RecursiveFixtureMutation { value: 7 },
        protocol::os_store::store_schema::HistoryLane::Document,
        &mut identity,
    ))
    .await)
    .expect("recursive archive parent mutation");
    let files = Box::pin(store::os_store::text_format::print_document_pack(parent_store.envelope())).await.expect("recursive archive parent pack");
    close_recursive_parent_fixture(&mut parent_store);
    files
}
//#endregion 🧬️ComposedParentFixture
