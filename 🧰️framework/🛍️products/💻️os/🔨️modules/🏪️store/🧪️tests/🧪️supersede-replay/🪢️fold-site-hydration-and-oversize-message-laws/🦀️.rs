// 🪢️ Fold site, hydration and oversize message laws: the member document hydration with supersessions, the retained and cancelled hydration, the input breaking the supersede law, the bounded oversize replay messages, the folded ledger check in and the interior revert.

//#region 🧪️FoldSiteLaws
/// 📨️ A supersession `remote` authored on another replica: one input replacing `target`, stamped past every local clock.
fn remote_supersession(document: &str, target: &MutationId, replacement: protocol::InputReplacement, logical: u64) -> semio_framework_os_spr::MutationEnvelope {
    let transition = semio_framework_os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: target.clone(), replacement }] });
    semio_framework_os_spr::history_transition_envelope(&transition, &ArtifactId(document.into()), &ActorId("remote-editor".into()), vec![target.clone()], HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical })
}

/// 🧵️ Every edit's inverse by edit id: what a load recomputes and every fold site must agree on.
fn inverses(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> BTreeMap<String, Vec<DemoMutation>> {
    store.envelope().vcs.edits.iter().map(|edit| (edit.id.clone(), edit.inverse.iter().cloned().collect())).collect()
}

/// 💧️ A member document whose history holds supersessions hydrates — the retained composition member open — to exactly
/// the store a `.pack`/`.spr` parse builds: the superseded state, the effective supersessions, the recomputed inverses,
/// the durable outcomes and the content revision. The `.ops` text mirror round-trips the same history (audit F-C1).
#[semio_framework_async_macros::async_test]
async fn a_member_document_with_supersessions_hydrates_to_its_superseded_state() {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() };
    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-superseded", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(dialect.clone());
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10))), input(&ids[2], None)] }).await.expect("a clean supersession");
    assert_eq!(store.snapshot_ref().n, Some(16));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reference = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pair parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let history = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.expect("history decodes");
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "member-superseded".into(), dialect };
    let (operation, generation) = (semio_framework_job::OperationId(3), semio_framework_job::Generation(5));
    let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(
        files.pack.clone(),
        history,
        expected,
        None,
        "demo/v1".into(),
        fixture_demo_owners(),
        operation,
        generation,
        u64::MAX,
        PersistedDocumentHydrationTarget::Store { generation: 0 },
        store.local_actor_id().clone(),
    );
    let mut sequence = 0;
    let mut member = None;
    let original_grant=fixture_hydration_grant();let mut hydration_receipts=semio_framework_value::retained_clone::RetainedCloneProgress::default();
    for _ in 0..100_000 {
        let mut receipt=semio_framework_value::retained_clone::RetainedCloneProgress::default();let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4, 999,original_grant), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence,&mut receipt);
        let step=hydration.step(&mut cx,original_grant);let actual=cx.retained_progress();assert!(actual.fits(original_grant));hydration_receipts=hydration_receipts.checked_add(actual).expect("original hydration receipt accumulation");
        match step {
            PersistedDocumentHydrationStep::Pending(_) => {}
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(hydrated)) => {
                member = Some(*hydrated);
                break;
            }
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(_)) => panic!("a store hydration hands out a store"),
            PersistedDocumentHydrationStep::Rejected(diagnostic) => panic!("hydration refused a superseded history: {diagnostic:?}"),
        }
    }
    let member = ArtifactStore(member.unwrap_or_else(|| panic!("hydration converges; it still retains {:?}", hydration.remaining_owner())), Some(close_test_store::<DemoSnapshot, DemoMutation>));
    assert_eq!(member.snapshot_ref().n, Some(16), "the hydrated member folds the effective forwards");
    assert_eq!(member.supersessions(), reference.supersessions());
    assert_eq!(inverses(&member), inverses(&reference));
    assert_eq!(outcomes_by_position(&member, &member.mutation_outcomes().unwrap()), outcomes_by_position(&reference, &reference.mutation_outcomes().unwrap()));
    assert_eq!(member.0.content_revision(), reference.0.content_revision(), "hydration and parse build the same canonical identity");
    assert_eq!(member.0.content_revision(), store.0.content_revision(), "and the live store's");
    let text = print_document_text(store.envelope()).await.expect("text prints");
    assert!(text.ops.lines().any(|line| line.starts_with("supersede")), "the text mirror carries the supersede line");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("the text mirror parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(16));
    assert_eq!(mirrored.supersessions(), store.supersessions());
    assert_eq!(mirrored.0.content_revision(), store.0.content_revision());
}

#[semio_framework_async_macros::async_test]
async fn retained_hydration_preserves_the_original_opened_actor_across_stored_authors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️opened-hydration-actor/🔣️.json")).unwrap();
    let opened = fixture["openedActor"].as_str().unwrap();
    let author = fixture["historyAuthor"].as_str().unwrap();
    assert_ne!(opened, author);
    for config in [false, true] {
        let initial = DemoSnapshot { n: Some(fixture["initial"].as_i64().unwrap() as i32) };
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "opened-actor-hydration", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        if config { envelope.history_shape = semio_framework_os_spr::HistoryShape::Config; }
        else { envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() }); }
        let mut source = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(author.into())).await.unwrap();
        source.install_document_store_owners_exact(fixture_demo_owners());
        let mut source = ArtifactStore(source, Some(close_test_store::<DemoSnapshot, DemoMutation>));
        apply(&mut source, vec![set(fixture["stored"].as_i64().unwrap() as i32)]).await;
        let files = print_document_pack(source.envelope()).await.unwrap();
        let history = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.unwrap();
        assert_eq!(history.edits.last().unwrap().actor.as_deref(), Some(author));
        let mut hydrated = None;
        if config {
            let mut hydration = RetainedConfigStoreHydration::<DemoSnapshot, DemoMutation>::from_snapshots(source.envelope().vcs.genesis.clone(), initial.clone(), initial, history, source.envelope().id.clone(), "demo/v1".into(), fixture_demo_owners(), 0, 65536, ActorId(opened.into()));
            for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
                match hydration.advance(fixture_hydration_grant()) {
                    ConfigStoreHydrationStep::Pending(_) => {}
                    ConfigStoreHydrationStep::Ready(store) => { hydrated = Some(*store); break; }
                    ConfigStoreHydrationStep::Rejected(diagnostic) => panic!("original config actor hydration refused: {diagnostic:?}"),
                }
            }
            assert!(hydration.terminal_is_empty());
        } else {
            let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: source.envelope().id.clone(), dialect: source.envelope().dialect.clone().unwrap() };
            let (operation, generation) = (semio_framework_job::OperationId(700), semio_framework_job::Generation(1));
            let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(files.pack, history, expected, None, "demo/v1".into(), fixture_demo_owners(), operation, generation, u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, ActorId(opened.into()));
            let mut sequence = 0;
            let original_grant=fixture_hydration_grant();let mut hydration_receipts=semio_framework_value::retained_clone::RetainedCloneProgress::default();
            for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
                let mut receipt=semio_framework_value::retained_clone::RetainedCloneProgress::default();let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4, u64::MAX,original_grant), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence,&mut receipt);
                let step=hydration.step(&mut cx,original_grant);let actual=cx.retained_progress();assert!(actual.fits(original_grant));hydration_receipts=hydration_receipts.checked_add(actual).expect("original opened-actor hydration receipt accumulation");
                match step {
                    PersistedDocumentHydrationStep::Pending(_) => {}
                    PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(store)) => { hydrated = Some(*store); break; }
                    PersistedDocumentHydrationStep::Rejected(diagnostic) => panic!("original member actor hydration refused: {diagnostic:?}"),
                    _ => panic!("original member actor hydration returned another output"),
                }
            }
            assert!(hydration.terminal_is_empty(), "hydration still retains {:?}", hydration.remaining_owner());
        }
        let hydrated = ArtifactStore(hydrated.expect("original actor hydration converges"), Some(close_test_store::<DemoSnapshot, DemoMutation>));
        assert_eq!(hydrated.snapshot_ref().n, source.snapshot_ref().n);
        assert_eq!(serde_json::to_value(hydrated.local_actor_id()).unwrap(), fixture["openedActor"]);
        assert_eq!(semio_framework_pack_json::to_json_string(hydrated.local_actor_id()), serde_json::to_string(&fixture["openedActor"]).unwrap());
        assert_eq!(hydrated.envelope().vcs.edits.last().unwrap().actor.as_deref(), Some(author));
    }
}

#[semio_framework_async_macros::async_test]
async fn cancelled_hydration_retires_the_same_original_opened_actor_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️opened-hydration-actor/🔣️.json")).unwrap();
    let source = demo_store("cancelled-opened-actor", Some(0)).await;
    let files = print_document_pack(source.envelope()).await.unwrap();
    let history = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.unwrap();
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: source.envelope().id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
    let mut receipts = Vec::new();
    for actor in fixture["retiredActors"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()) {
        let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(files.pack.clone(), history.clone(), expected.clone(), None, "demo/v1".into(), fixture_demo_owners(), semio_framework_job::OperationId(701), semio_framework_job::Generation(1), u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, ActorId(actor.into()));
        let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
        assert_eq!(hydration.close_step(zero).unwrap(),RetainedCloneStep::Progress(Default::default()));
        let grant=|key:&str|{let n=|axis:&str|fixture[key][axis].as_u64().unwrap()as usize;RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")}};
        let mut charged = 0;
        for _ in 0..32 {
            let admitted=grant("grant");let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||hydration.close_step(admitted).unwrap());let receipt=step.progress();assert!(receipt.fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));charged+=receipt.released_bytes;
        }
        assert!(!hydration.terminal_is_empty(),"three release bytes cannot physically release the original actor allocation");
        for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
            let admitted=grant("cleanupGrant");let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||hydration.close_step(admitted).unwrap());let receipt=step.progress();assert!(receipt.fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));charged+=receipt.released_bytes;if matches!(step,RetainedCloneStep::Complete(_)){break}
        }
        assert!(hydration.terminal_is_empty());
        receipts.push(charged);
    }
    assert_eq!(receipts[1] - receipts[0], fixture["retiredActorByteDifference"].as_u64().unwrap() as usize);
    let oracle: Vec<String> = serde_json::from_value(fixture["retiredActors"].clone()).unwrap();
    assert_eq!(oracle[1].len() - oracle[0].len(), receipts[1] - receipts[0]);
}

/// 🧯️ A replacement input breaking the supersede law — garbage bytes and a foreign schema from a remote replica — folds
/// its target as a no-op with one `Fatal` `mutation.invariant` outcome, identically at every fold site: ingest, the
/// durable outcomes, `state_before`, both persisted forms and `materialize_document_snapshot`. Hub check-in refuses it as
/// a blocking report — never a decode failure — and takes the ledger once a later supersession repairs it (audit F-M2).
#[semio_framework_async_macros::async_test]
async fn an_input_breaking_the_supersede_law_folds_as_a_fatal_no_op_at_every_site() {
    let mut store = demo_store("garbage", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3)]).await;
    let ids = operation_ids(&store);
    let baseline = print_document_pack(store.envelope()).await.expect("baseline pair prints");
    let garbage = remote_supersession("garbage", &ids[1], protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: vec![0xff, 0x13, 0x37] }, 0);
    let foreign = remote_supersession("garbage", &ids[2], protocol::InputReplacement::Input { schema: "other/v1".into(), payload: add(30).encode_op().unwrap() }, 1);
    for envelope in [garbage.clone(), foreign] {
        let report = store.ingest_remote(envelope).await.expect("a replica never refuses a supersession it cannot decode");
        assert!(report.accepted && report.conflict.is_none());
    }
    assert_eq!(store.snapshot_ref().n, Some(1), "both superseded operations fold as no-ops");
    let outcomes = outcomes_by_mutation(&store.mutation_outcomes().expect("durable outcomes"));
    for target in [&ids[1], &ids[2]] {
        assert_eq!(outcomes[target], (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.invariant".to_string()], true, false));
    }
    assert_eq!(store.state_before(&ids[2], &protocol::HistoryInputDrafts::new()).expect("state before").n, Some(1));
    assert_eq!(materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("materialized").n, Some(1));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("the pair loads").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.snapshot_ref().n, Some(1));
    assert_eq!(outcomes_by_mutation(&reloaded.mutation_outcomes().unwrap()), outcomes);
    let text = print_document_text(store.envelope()).await.expect("text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("the text mirror loads").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(1));
    let refused =
        replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &semio_framework_os_spr::encode_envelopes(std::slice::from_ref(&garbage)), || Ok(fixture_demo_owners())).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.invariant")), "{refused:?}");
    let repair = remote_supersession("garbage", &ids[1], draft(Some(add(5))), 2);
    let repaired = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &semio_framework_os_spr::encode_envelopes(&[garbage, repair]), || Ok(fixture_demo_owners()))
        .await
        .expect("a repaired ledger checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&repaired.pack, &repaired.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(9));
    test_support::retire_parsed_document(parsed);
}

/// ✂️ A replay never yields an unstorable message ledger: an edit whose outcomes exceed one entry's byte authority keeps
/// its worst messages and summarizes the rest deterministically — at local apply, at a local supersession's install, at
/// a remote supersession's ingest and at load — so no supersession is refused or wedged for size, and an alternative
/// branched over such an edit is authored exactly once (audit F-M3).
#[semio_framework_async_macros::async_test]
async fn oversize_replay_messages_are_bounded_deterministically_everywhere() {
    const OPERATIONS: usize = 200;
    let bytes = |entry: &[semio_framework_os_spr::MutationMessage], edit_id: &str| entry.iter().fold(edit_id.len(), |total, message| total + message.code.0.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>());
    let dropped = |entry: &[semio_framework_os_spr::MutationMessage]| -> usize {
        let summary = entry.last().expect("a bounded entry ends with its summary");
        assert_eq!((summary.code.0.as_str(), summary.level), ("mutation.cascade", semio_framework_diagnostic::Severity::Info));
        summary.message.strip_suffix(" more messages").expect("the summary counts what it drops").parse().expect("a count")
    };
    let mut store = demo_store("bounded", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, (0..OPERATIONS).map(|_| add(1)).collect()).await;
    let ids = operation_ids(&store);
    let bulk = store.applied_edit_ids()[1].clone();
    let applied = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&applied, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert_eq!(applied.len() - 1 + dropped(&applied), OPERATIONS, "kept and summarized messages account for every operation");
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(DemoMutation::AssignN(AssignN { n: Some(7) })))] }).await.expect("an oversize but clean replay installs");
    assert_eq!(store.snapshot_ref().n, Some(207));
    assert_eq!(store.messages_for_edit(&bulk), applied.as_slice(), "the replayed entry bounds exactly like the applied one");
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "bounded-variant".into(), inputs: vec![input(&ids[0], Some(DemoMutation::AssignN(AssignN { n: Some(9) })))] }).await.expect("an oversize replay branches");
    assert_eq!(store.envelope().vcs.alternatives.iter().filter(|alternative| alternative.name == "bounded-variant").count(), 1);
    assert_eq!(store.snapshot_ref().n, Some(209));
    let withdraw = remote_supersession("bounded", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let report = store.ingest_remote(withdraw).await.expect("a remote supersession is never refused for size");
    assert!(report.accepted && report.conflict.is_none());
    assert_eq!(store.snapshot_ref().n, None);
    let failed = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&failed, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert!(failed[..failed.len() - 1].iter().all(|message| message.code.0 == "mutation.target-missing" && message.level == semio_framework_diagnostic::Severity::Error), "the worst messages are the ones kept");
    let kept = failed.len() - 1;
    assert_eq!(kept + dropped(&failed), OPERATIONS);
    assert_eq!(failed.last().unwrap().op_index, Some(kept as u32), "the summary sits at the first dropped operation");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("a bounded ledger loads").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.messages_for_edit(&bulk), failed.as_slice(), "load bounds the recomputed messages identically");
    assert_eq!(outcomes_by_mutation(&reloaded.mutation_outcomes().unwrap()), outcomes_by_mutation(&store.mutation_outcomes().unwrap()));
}

/// ⚖️ Hub check-in judges the history a ledger folds to, never the states in between: a supersession that makes a
/// downstream operation fail is refused alone, and checks in once a later supersession heals it (audit F-M4).
#[semio_framework_async_macros::async_test]
async fn check_in_judges_the_folded_ledger_not_its_intermediate_states() {
    let mut store = demo_store("intermediate", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let breaking = remote_supersession("intermediate", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let healing = remote_supersession("intermediate", &ids[0], draft(Some(DemoMutation::AssignN(AssignN { n: Some(7) }))), 1);
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_spr::encode_envelopes(std::slice::from_ref(&breaking)), || Ok(fixture_demo_owners())).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    let healed = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_spr::encode_envelopes(&[breaking, healing]), || Ok(fixture_demo_owners()))
        .await
        .expect("a ledger whose final history is clean checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&healed.pack, &healed.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(8));
    test_support::retire_parsed_document(parsed);
}

/// 🔁️ A replica with an interior revert — an undo of an edit a later foreign edit follows — at the given early-exit
/// setting: `restore 5`, `delete`, then the peer's `add 1` (kept with its error under a laissez-faire policy), then undo.
async fn interior_revert_store(early_exit: bool) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    let mut store = demo_store("interior", None).await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    if early_exit {
        store.enable_convergence_early_exit();
    }
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![DemoMutation::DeleteN(DeleteN {})]).await;
    let peer = semio_framework_os_spr::MutationEnvelope {
        mutation_id: MutationId("peer-add".into()),
        document_id: ArtifactId("interior".into()),
        actor: ActorId("peer".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: SchemaId("demo/v1".into()), payload: add(1).encode_op().unwrap() },
        inverse: semio_framework_os_spr::InverseMutation { schema: SchemaId("demo/v1".into()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp { actor: 4, physical_ms: u64::MAX / 2, logical: 0 },
        transaction: None,
        verb: None,
        line: None,
    };
    let report = store.ingest_remote(peer).await.expect("the peer edit ingests");
    assert!(report.accepted);
    assert_eq!(store.snapshot_ref().n, None);
    fixture_author(&mut store, ArtifactCommand::Undo).await.expect("the interior delete reverts");
    store
}

/// 🏁️ An interior re-projection keeps the durable history replay-consistent: after an interior revert the downstream
/// edit's durable outcome is what a replay says, so a supersession replay converging early reports exactly what the full
/// replay reports (audit F-M6).
#[semio_framework_async_macros::async_test]
async fn an_interior_revert_keeps_the_early_exit_equal_to_the_full_replay() {
    let mut early = interior_revert_store(true).await;
    let mut full = interior_revert_store(false).await;
    for store in [&early, &full] {
        assert_eq!(store.snapshot_ref().n, Some(6), "the peer edit applies once the delete is reverted");
        let ids = operation_ids(store);
        assert_eq!(outcomes_by_mutation(&store.mutation_outcomes().unwrap())[&ids[1]].1, vec!["mutation.cascade".to_string()], "the reverted interior refreshed the downstream outcome");
    }
    let restate = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| drafts(&[(&operation_ids(store)[0], Some(DemoMutation::AssignN(AssignN { n: Some(5) })))]);
    let converged = finish(&early, early.begin_report_replay(&restate(&early), None).expect("early replay"));
    assert!(converged.converged_at().is_some(), "an equal prefix state converges");
    let replayed = finish(&full, full.begin_report_replay(&restate(&full), None).expect("full replay"));
    assert!(replayed.converged_at().is_none());
    assert_eq!(outcomes_by_position(&early, &early.replay_report(&converged).unwrap().outcomes), outcomes_by_position(&full, &full.replay_report(&replayed).unwrap().outcomes));
    converged.retire_unscheduled().expect("the converged replay retires from its own quoted demands");
    replayed.retire_unscheduled().expect("the full replay retires from its own quoted demands");
    fixture_author(&mut early, ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
    fixture_author(&mut full, ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
    for store in [&early, &full] {
        assert_eq!(store.snapshot_ref().n, None);
        let ids = operation_ids(store);
        assert_eq!(outcomes_by_mutation(&store.mutation_outcomes().unwrap())[&ids[2]].1, vec!["mutation.target-missing".to_string()], "the interior redo refreshed it again");
    }
}

/// 🔏️ Two replicas holding the same events report the same content revision whatever order a supersession of a redo
/// edit and the reverts that moved it there arrived in (audit F-m1).
#[semio_framework_async_macros::async_test]
async fn a_supersession_of_a_redo_edit_keeps_revisions_order_independent() {
    let mut author = demo_store("redo-revision", Some(0)).await;
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let log = author.event_log().expect("edit log");
    fixture_author(&mut author, ArtifactCommand::Undo).await.expect("undo add 3");
    fixture_author(&mut author, ArtifactCommand::Undo).await.expect("undo add 2");
    let reverts: Vec<_> = author.event_log().expect("log").into_iter().filter(|event| !log.iter().any(|known| known.mutation_id == event.mutation_id)).collect();
    assert_eq!(reverts.len(), 2);
    let supersede = remote_supersession("redo-revision", &ids[2], draft(Some(add(30))), 0);
    let mut late = demo_store("redo-revision", Some(0)).await;
    for event in log.iter().cloned().chain(reverts.iter().cloned()).chain([supersede.clone()]) {
        late.ingest_remote(event).await.expect("reverts first, then the supersession");
    }
    let mut early = demo_store("redo-revision", Some(0)).await;
    for event in log.iter().cloned().chain([supersede]).chain(reverts.iter().cloned()) {
        early.ingest_remote(event).await.expect("the supersession first, then the reverts");
    }
    assert_eq!(late.redo_edit_ids(), early.redo_edit_ids());
    assert_eq!(late.redo_edit_ids().len(), 2);
    assert_eq!(late.0.content_revision(), early.0.content_revision());
    let files = print_document_pack(late.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pair parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.0.content_revision(), late.0.content_revision(), "and a fresh load agrees");
}
//#endregion 🧪️FoldSiteLaws
