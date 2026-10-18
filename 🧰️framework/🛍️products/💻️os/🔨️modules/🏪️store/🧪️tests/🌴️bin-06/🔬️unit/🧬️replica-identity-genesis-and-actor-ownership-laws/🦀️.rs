/// 🧬️ Replica identity, genesis and actor ownership laws: the fresh replicas with identical gestures, the borrowed child sources, the stored genesis neutral law, the opened actor ownership, the bounded genesis capture and the actorless history refusal.

/// 🪪️ Fixture law `🧫️fixtures/🪪️replica-edit-identity.json`: fresh replicas that author identical
/// gestures never share an edit or wire operation id (the hub ledger dedupes by id).
#[semio_framework_async_macros::async_test]
async fn fresh_replicas_authoring_identical_gestures_never_share_an_edit_or_operation_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🪪️replica-edit-identity.json")).expect("replica identity fixture");
    let mut edit_ids = std::collections::HashSet::new();
    let mut operation_ids = std::collections::HashSet::new();
    for _ in 0..fixture["replicas"].as_u64().expect("replica count") {
        let genesis = DemoSnapshot { n: Some(fixture["genesisN"].as_i64().expect("genesis n") as i32) };
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>(fixture["artifactSchema"].as_str().expect("schema"), fixture["documentId"].as_str().expect("document"), genesis, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        for gesture in fixture["gestures"].as_array().expect("gestures") {
            let mutations = gesture["ops"]
                .as_array()
                .expect("gesture ops")
                .iter()
                .map(|op| match (op.get("set"), op.get("add")) {
                    (Some(n), None) => DemoMutation::SetN(SetN { n: n.as_i64().expect("set n") as i32 }),
                    (None, Some(delta)) => DemoMutation::AddN(AddN { delta: delta.as_i64().expect("add delta") as i32 }),
                    _ => panic!("fixture op must be exactly one of set/add"),
                })
                .collect();
            store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, &mut identity).await.expect("fixture gesture applies");
        }
        let edits = &store.log.envelope.vcs.edits;
        assert_eq!(edits.len() as u64, fixture["expect"]["editsPerReplica"].as_u64().expect("edits per replica"));
        let operations: Vec<String> = edits.iter().flat_map(|edit| semio_framework_os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(edit)).map(|id| id.0).collect();
        assert_eq!(operations.len() as u64, fixture["expect"]["operationIdsPerReplica"].as_u64().expect("operations per replica"));
        edit_ids.extend(edits.iter().map(|edit| edit.id.clone()));
        operation_ids.extend(operations);
        close_demo_artifact_store(&mut store);
    }
    assert_eq!(edit_ids.len() as u64, fixture["expect"]["distinctEditIds"].as_u64().expect("distinct edits"));
    assert_eq!(operation_ids.len() as u64, fixture["expect"]["distinctOperationIds"].as_u64().expect("distinct operations"));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


#[semio_framework_async_macros::async_test]
async fn dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🫳️child-dispatch/🔣️.json")).expect("neutral borrowed child declaration");
    let parent_ref=semio_framework_artifact_reference::ArtifactRef{artifact_id:fixture["parent"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.demoparent".into(),standard:"1".into(),subset:"*".into()}};
    let child_ref=semio_framework_artifact_reference::ArtifactRef{artifact_id:fixture["child"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.demochild".into(),standard:"1".into(),subset:"*".into()}};
    let mut parent=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&parent_ref.artifact_id,DemoSnapshot{n:Some(0)},None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&child_ref.artifact_id,DemoSnapshot{n:Some(0)},None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator=CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id,fixture["slot"].as_str().unwrap(),&child_ref.artifact_id).await.unwrap();
    let source=vec![ValidatedMutation::SetN(ValidatedSetN{n:fixture["invalidN"].as_i64().unwrap() as i32}).encode_op().unwrap()];
    let expected=source[0].clone();
    let bytes=source[0].as_ptr();
    let cells=source.as_ptr();
    let schema=SchemaId(fixture["opSchema"].as_str().unwrap().into());
    let labels=[semio_framework_ui_locale::LocalizedLabel::data(fixture["label"].as_str().unwrap().to_owned())];
    let dispatch=ChildDispatch::borrowed(child_ref,&source,&schema,&labels);
    assert_eq!(dispatch.ops.as_ptr(),cells);
    assert_eq!(dispatch.ops[0].as_ptr(),bytes);
    assert!(std::ptr::eq(dispatch.op_schema,&schema));
    assert_eq!(dispatch.labels.as_ptr(),labels.as_ptr());
    let mut children=[(&mut child,dispatch)];
    let result=coordinator.dispatch_group(&parent_ref,&mut parent,&mut children,Vec::new(),Vec::new(),GroupMeta::default(), &mut identity).await;
    assert!(matches!(result,Err(VcsError::Rejected{policy:semio_framework_os_spr::MergePolicy::Normal,..})));
    assert_eq!(children[0].1.ops.as_ptr(),cells);
    assert_eq!(children[0].1.ops[0].as_ptr(),bytes);
    assert_eq!(children[0].1.ops[0],expected);
    drop(children);
    assert!(parent.envelope().vcs.edits.is_empty()&&child.envelope().vcs.edits.is_empty());
    assert_eq!(source[0].as_ptr(),bytes);
    assert_eq!(source[0],expected);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[test]
fn immutable_stored_genesis_obeys_the_neutral_law() {
    #[derive(Debug, PartialEq)]
    struct GenesisLawSnapshot { n: i64 }
    static ENCODED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static DECODED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    struct GenesisLawProducer;
    impl semio_framework_os_vcs::io::binary::genesis::ArtifactGenesisProducer<GenesisLawSnapshot> for GenesisLawProducer {
        fn encode_genesis_pack(&self,snapshot:&GenesisLawSnapshot) -> Result<Vec<u8>, ValueError> { ENCODED.fetch_add(1, std::sync::atomic::Ordering::SeqCst); Ok(format!("{{\"n\":{}}}", snapshot.n).into_bytes()) }
        fn decode_genesis_pack(&self,pack: &[u8]) -> Result<GenesisLawSnapshot, ValueError> { DECODED.fetch_add(1, std::sync::atomic::Ordering::SeqCst); let value: serde_json::Value = serde_json::from_slice(pack).unwrap(); Ok(GenesisLawSnapshot { n: value["n"].as_i64().unwrap() }) }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    let born = AdmittedArtifactGenesis::born(GenesisLawSnapshot { n: 0 },&GenesisLawProducer).unwrap();
    assert_eq!(ENCODED.load(std::sync::atomic::Ordering::SeqCst), 1);
    let mut digests = Vec::new();
    for row in fixture["packs"].as_array().unwrap() {
        let raw = row["hex"].as_str().unwrap();
        let bytes: Vec<u8> = raw.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
        let stored:AdmittedArtifactGenesis<GenesisLawSnapshot> = semio_framework_os_vcs::io::text::genesis::native_genesis_from_value(DslValue::String(raw.into()),&GenesisLawProducer).unwrap();
        assert_eq!(stored.facts().snapshot().n, row["decoded"]["n"].as_i64().unwrap());
        assert_eq!(stored.pack(), bytes);
        assert_eq!(stored.to_value(), DslValue::String(raw.into()));
        assert_eq!(stored.facts().digest(), *semio_framework_hash::hash(&bytes).as_bytes());
        let alias = stored.facts().share_snapshot();
        let cloned = stored.clone();
        assert!(Arc::ptr_eq(&alias, &cloned.facts().share_snapshot()));
        assert!(Arc::ptr_eq(&stored.share_pack(), &cloned.share_pack()));
        assert_eq!(stored.facts().digest(), cloned.facts().digest());
        digests.push(stored.facts().digest());
    }
    assert_eq!(ENCODED.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(DECODED.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert_eq!(born.facts().digest(), digests[0]);
    assert_ne!(digests[0], digests[1]);
    assert!(semio_framework_os_vcs::io::text::genesis::decode_genesis_hex_controlled("7B",&mut semio_framework_value::NativeDecodeControl::new(4096,&mut |_|true)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn opened_actor_ownership_obeys_the_neutral_law() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    let actors = &fixture["actors"];
    let actor = ActorId(actors["opened"].as_str().unwrap().into());
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "actor-genesis-law", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut real = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, actor.clone()).await.unwrap();
    real.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert!(Arc::ptr_eq(&*real.live.current, &real.envelope().vcs.genesis.facts().share_snapshot()));
    let mut live = ArtifactStore(real, Some(close_test_store::<DemoSnapshot, DemoMutation>));
    live.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.unwrap();
    let first = live.envelope().vcs.edits.iter().next().unwrap();
    assert_eq!(first.actor.as_deref(), actors["bareAuthor"].as_str());
    assert_eq!(first.mutation_meta[0].author_id.as_ref(), Some(&actor));
    let remote = foreign_mutation_envelope(actors["explicitAuthor"].as_str().unwrap(), DemoMutation::SetN(SetN { n: 2 })).await;
    live.dispatch(ArtifactCommand::IngestRemote { envelope: remote }, &mut identity).await.unwrap();
    let foreign = live.envelope().vcs.edits.iter().last().unwrap();
    assert_eq!(foreign.actor.as_deref(), actors["explicitAuthor"].as_str());
    assert_eq!(live.local_actor_id(), &actor);
    live.dispatch(ArtifactCommand::Undo, &mut identity).await.unwrap();
    assert_eq!(live.envelope().transitions.last().unwrap().actor.0.as_str(), actors["transitionAuthor"].as_str().unwrap());
    let files = print_document_pack(live.envelope()).await.unwrap();
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    let mut reopened = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(parsed.into_envelope(), actor.clone()).await.unwrap();
    reopened.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert_eq!(reopened.local_actor_id().0.as_str(), actors["reopened"].as_str().unwrap());
    close_test_store(&mut reopened);
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    let mut candidate = parsed.into_envelope();
    candidate.vcs.edits.iter_mut().next().unwrap().actor = None;
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(candidate, actor).await.is_err());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[test]
fn stored_genesis_capture_is_bounded_and_cancellable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    for row in fixture["packs"].as_array().unwrap() {
        let hex = row["hex"].as_str().unwrap();
        for fuel in [1, 2, 7] {
            let raw = format!("{{\"schema\":\"{hex}\",\"id\":\"capture\"}}");
            let mut cursor = owned_schema_test_cursor(&[raw.as_bytes()]);
            let cancel = semio_framework_job::root_cancel_token();
            let mut sequence = 0;
            let token = loop {
                let mut original_retained_progress_8=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(7, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_8);
                match cursor.step(&mut cx) { OwnedSchemaRecordStep::FieldToken { field_id: 1, token, .. } => break token, OwnedSchemaRecordStep::Pending => {}, _ => panic!("canonical Pack token") }
            };
            let mut original_retained_progress_9=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_9);
            let mut capture = ArtifactGenesisPackCapture::new(token, &cx).unwrap();
            for _ in 0..hex.len()+2 {
                let before = capture.relative;
                let mut original_retained_progress_10=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_10);
                let complete = capture.step(&cursor, &mut cx).unwrap();
                assert!(capture.relative-before <= fuel as usize);
                if complete { break; }
            }
            assert!(capture.complete);
            assert_eq!(capture.bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), hex);
            assert!(capture.hasher.finalize() == semio_framework_hash::hash(&capture.bytes));
            let mut original_retained_progress_11=semio_framework_value::RetainedCloneProgress::default();
        let mut stale = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(2), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_11);
            let before = capture.bytes.len();
            assert!(capture.step(&cursor, &mut stale).is_err());
            assert_eq!(capture.bytes.len(), before);
            cancel.cancel_now();
            let mut original_retained_progress_12=semio_framework_value::RetainedCloneProgress::default();
        let mut cancelled = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel, semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_12);
            assert!(capture.step(&cursor, &mut cancelled).is_err());
            assert_eq!(capture.bytes.len(), before);
            let grant=physical_test_close_grant();
            let(retirement,receipt)=semio_framework_value::factory::OwnedValueRetirementFactory::<Vec<u8>>::default().retire_owned(capture.bytes,grant).unwrap_or_else(|(error,_)|panic!("original captured bytes admission refused: {error}"));
            assert!(receipt.fits(grant));drive_retirement_terminal(retirement);
            let grant=schema_test_close_grant();
            for _ in 0..8 { if cursor.terminal_is_empty() { break; }let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes)); }
            assert!(cursor.terminal_is_empty());
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_hydrators_refuse_actorless_history_from_the_neutral_law() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    for config in [false, true] {
        for row in fixture["retainedAdmissions"].as_array().unwrap() {
            let initial = DemoSnapshot { n: Some(0) };
            let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "actorless-hydration", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
            if config { envelope.history_shape = semio_framework_os_spr::HistoryShape::Config; }
            else { envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() }); }
            let mut live = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
            live.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.unwrap();
            let files = print_document_pack(live.envelope()).await.unwrap();
            let mut history = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.unwrap();
            history.edits[0].actor = row["editAuthor"].as_str().map(semio_framework_value::SharedUtf8::from);
            history.edits[0].meta.as_mut().unwrap()[0].author_id = row["operationAuthor"].as_str().map(semio_framework_value::SharedUtf8::from);
            let actor = ActorId(fixture["actors"]["opened"].as_str().unwrap().into());
            let mut rejected = false;
            if config {
                let mut hydration = RetainedConfigStoreHydration::<DemoSnapshot, DemoMutation>::from_snapshots(live.envelope().vcs.genesis.clone(), initial.clone(), initial.clone(), history, live.envelope().id.clone(), "demo/v1".into(), complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners(physical_test_close_grant())), 0, 65536, actor);
                for _ in 0..1_000_000 { match hydration.advance(physical_test_close_grant()) { ConfigStoreHydrationStep::Pending(_) => {}, ConfigStoreHydrationStep::Rejected(diagnostic) => {  rejected = true; break; }, ConfigStoreHydrationStep::Ready(mut store) => { close_test_store(&mut store); break; } } }
                let mut closed = false;
                for _ in 0..100000 { if hydration.terminal_is_empty() { closed = true; break; } let step=hydration.close_step(physical_test_close_grant()).unwrap();assert!(step.progress().fits(physical_test_close_grant())); }
                assert!(closed, "retained hydration closes exactly");
            } else {
                let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: live.envelope().id.clone(), dialect: live.envelope().dialect.clone().unwrap() };
                let (operation, generation) = (semio_framework_job::OperationId(801), semio_framework_job::Generation(1));
                let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_decoded_pack(initial, files.pack, live.envelope().vcs.genesis.facts().digest(), history, expected, None, "demo/v1".into(), complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners(physical_test_close_grant())), operation, generation, u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, actor);
                let mut sequence = 0;
                for _ in 0..1_000_000 {
                    let mut original_retained_progress_13=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX, physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_13);
                    match hydration.step(&mut cx, physical_test_close_grant()) { PersistedDocumentHydrationStep::Pending(_) => {}, PersistedDocumentHydrationStep::Rejected(diagnostic) => {  rejected = true; break; }, PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(mut store)) => { close_test_store(&mut store); break; }, _ => panic!("requested store output") }
                }
                let mut closed = false;
                for _ in 0..100000 { if hydration.terminal_is_empty() { closed = true; break; } let step=hydration.close_step(physical_test_close_grant()).unwrap();assert!(step.progress().fits(physical_test_close_grant())); }
                assert!(closed, "retained hydration closes exactly");
            }
            assert_eq!(!rejected, row["accepted"].as_bool().unwrap(), "config={config}, authors={row}");
        }
    }
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}
