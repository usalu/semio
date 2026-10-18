// 🔂️ Replica convergence, cancellation and prefix ring laws: the replicas converging under a supersession, the report replay cancellation, the convergence early exit, the state before projection, the atomic finished replays, the prefix ring evictions, the semantic operation labels and the supersede command round trip.

//#region 🧪️SupersedeLaws
/// 🔀️ A supersession converges on every replica whatever order its events arrive in, also against a concurrent edit.
#[semio_framework_async_macros::async_test]
async fn replicas_converge_on_a_supersession_under_shuffled_arrival() {
    let mut author = demo_store("converge", Some(0)).await;
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let mut peer = demo_store("converge", Some(0)).await;
    for event in author.event_log().expect("log") {
        peer.ingest_remote(event).await.expect("peer ingests the edits");
    }
    apply(&mut peer, vec![add(100)]).await;
    fixture_author(&mut author, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(7)))] }).await.expect("supersede");
    let log = author.event_log().expect("author log");
    for event in peer.event_log().expect("peer log") {
        author.ingest_remote(event).await.expect("author ingests the concurrent edit");
    }
    for event in log.iter().rev().cloned() {
        peer.ingest_remote(event).await.expect("peer ingests out of order");
    }
    assert_eq!(author.snapshot().unwrap(), peer.snapshot().unwrap());
    assert_eq!(author.snapshot_ref().n, Some(112));
    assert_eq!(author.applied_edit_ids().len(), peer.applied_edit_ids().len());
    assert_eq!(author.supersessions().keys().collect::<Vec<_>>(), peer.supersessions().keys().collect::<Vec<_>>());
    assert_eq!(outcomes_by_mutation(&author.mutation_outcomes().unwrap()), outcomes_by_mutation(&peer.mutation_outcomes().unwrap()));
    for rotation in 0..log.len() {
        let mut replica = demo_store("converge", Some(0)).await;
        let mut shuffled = log.clone();
        shuffled.rotate_left(rotation);
        for event in shuffled {
            replica.ingest_remote(event).await.expect("replica ingests");
        }
        assert_eq!(replica.applied_edit_ids().len(), 3, "rotation {rotation}");
        assert_eq!(replica.snapshot_ref().n, Some(12), "rotation {rotation}");
        assert!(replica.supersessions().contains_key(&ids[0]), "rotation {rotation}");
    }
}

/// 🛑️ Cancelling a Report replay after any number of steps leaves the store exactly as it was, and every scratch owner
/// retires (the test store closes to its terminal-empty witness).
#[semio_framework_async_macros::async_test]
async fn cancelling_a_report_replay_at_every_step_leaves_the_store_untouched() {
    let mut store = demo_store("cancel", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2), add(3)]).await;
    apply(&mut store, vec![add(4)]).await;
    let ids = operation_ids(&store);
    let entries = drafts(&[(&ids[0], Some(set(9)))]);
    let mut probe = store.begin_report_replay(&entries, None).expect("probe").with_retirement_factories(Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<DemoMutation>::default()));
    let total = probe.progress().total;
    probe.cancel();
    semio_framework_os_store_testing_doubles::close(&mut probe);
    assert_eq!(total, 7);
    for stop in 0..=total {
        let (generation, revision, snapshot, applied, transitions, outcomes) =
            (store.generation(), store.0.content_revision(), store.snapshot().unwrap(), store.applied_edit_ids().to_vec(), store.envelope().transitions.len(), store.mutation_outcomes().unwrap());
        let mut replay = store.begin_report_replay(&entries, None).expect("replay").with_retirement_factories(Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<DemoMutation>::default()));
        if stop > 0 {
            while replay.progress().done < stop {
                assert!(replay.step(store.replay_edits(), &mut || true).is_ok());
            }
            assert_eq!(replay.progress().done, stop);
        }
        replay.cancel();
        assert_eq!(store.generation(), generation);
        assert_eq!(store.0.content_revision(), revision);
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert_eq!(store.applied_edit_ids(), applied.as_slice());
        assert_eq!(store.envelope().transitions.len(), transitions);
        assert_eq!(store.mutation_outcomes().unwrap(), outcomes);
        semio_framework_os_store_testing_doubles::close(&mut replay);
    }
}

/// 🏁️ Convergence early exit reports and finalizes exactly what the full replay does.
#[semio_framework_async_macros::async_test]
async fn convergence_early_exit_equals_the_full_replay() {
    let mut stores = Vec::new();
    for early in [true, false] {
        let mut store = demo_store(if early { "early" } else { "full" }, Some(0)).await;
        if early {
            store.enable_convergence_early_exit();
        }
        apply(&mut store, vec![set(1)]).await;
        apply(&mut store, vec![set(2)]).await;
        for _ in 0..5 {
            apply(&mut store, vec![add(1)]).await;
        }
        let ids = operation_ids(&store);
        store.state_before(&ids[6], &protocol::HistoryInputDrafts::new()).map(|preview| drive_retirement_terminal(admit_demo_alias_retirement(&store,preview))).expect("the preview populates the prefix ring");
        stores.push((store, ids));
    }
    let mut reports = Vec::new();
    for (store, ids) in &stores {
        let replay = store.begin_report_replay(&drafts(&[(&ids[0], Some(set(50)))]), None).expect("replay");
        let result = finish(store, replay);
        reports.push((result.converged_at(), result.state().map(|state| state.n), outcomes_by_position(store, &store.replay_report(&result).unwrap().outcomes)));
        result.retire_unscheduled().expect("the inspected replay retires from its own quoted demands");
    }
    assert!(reports[0].0.is_some_and(|length| length < 7), "the early store converges before the head: {:?}", reports[0].0);
    assert_eq!(reports[1].0, None, "the full store replays every edit");
    assert_eq!(reports[0].1, reports[1].1);
    assert_eq!(reports[0].2, reports[1].2);
    for (store, ids) in &mut stores {
        let target = ids[0].clone();
        fixture_author(store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&target, Some(set(50)))] }).await.expect("supersede");
    }
    assert_eq!(stores[0].0.snapshot().unwrap(), stores[1].0.snapshot().unwrap());
    assert_eq!(outcomes_by_position(&stores[0].0, &stores[0].0.mutation_outcomes().unwrap()), outcomes_by_position(&stores[1].0, &stores[1].0.mutation_outcomes().unwrap()));
    test_support::assert_live_equals_replay(&stores[0].0).await;
}

/// 🕰️ `state_before` is the projection before an operation, with drafts over the effective inputs and nothing downstream.
#[semio_framework_async_macros::async_test]
async fn state_before_answers_the_projection_before_an_operation_under_drafts() {
    let mut store = demo_store("before", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    let mut observe = |target: &MutationId, entries: protocol::HistoryInputDrafts| {
        let preview = store.state_before(target, &entries).expect("preview");
        let n = preview.n;
        drive_retirement_terminal(admit_demo_alias_retirement(&store,preview));
        n
    };
    assert_eq!(observe(&ids[0], protocol::HistoryInputDrafts::new()), Some(0));
    assert_eq!(observe(&ids[3], protocol::HistoryInputDrafts::new()), Some(6));
    assert_eq!(observe(&ids[3], drafts(&[(&ids[0], Some(set(10)))])), Some(15));
    assert_eq!(observe(&ids[2], drafts(&[(&ids[1], None)])), Some(1));
    assert!(store.live.prefix_ring.iter().all(|entry| entry.length <= 2), "only live prefixes are retained");
    assert!(!store.live.prefix_ring.is_empty());
    assert_eq!(store.snapshot_ref().n, Some(10), "a preview never touches the live projection");
}

/// 🏁️ A finished session replay commits without replaying again, refuses when stale or blocking, and finalizes as a new
/// alternative just like an overwrite.
#[semio_framework_async_macros::async_test]
async fn finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones() {
    let mut store = demo_store("finish", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    let stale = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(set(10)))]), None).expect("stale replay"));
    apply(&mut store, vec![add(3)]).await;
    assert!(matches!(fixture_finalize(&mut store,stale, HistoryFinalization::Overwrite).await, Err(VcsError::Stale { .. })));
    let blocking = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(DemoMutation::DeleteN(DeleteN {})))]), None).expect("blocking replay"));
    let generation = store.generation();
    assert!(matches!(fixture_finalize(&mut store,blocking, HistoryFinalization::Overwrite).await, Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, .. })));
    assert_eq!(store.generation(), generation);
    let clean = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(set(10)))]), None).expect("clean replay"));
    let receipt = fixture_finalize(&mut store,clean, HistoryFinalization::Overwrite).await.expect("overwrite");
    assert!(receipt.generation > generation);
    assert_eq!(store.snapshot_ref().n, Some(15));
    test_support::assert_live_equals_replay(&store).await;
    let variant = finish(&store, store.begin_report_replay(&drafts(&[(&ids[1], Some(add(20)))]), None).expect("variant replay"));
    fixture_finalize(&mut store,variant, HistoryFinalization::Alternative { name: "variant".into() }).await.expect("new alternative");
    assert_eq!(store.snapshot_ref().n, Some(33));
    assert!(store.envelope().active_alternative_id.is_some());
    test_support::assert_live_equals_replay(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
}

/// 🧭️ Ring entries a supersession makes stale are evicted through the snapshot retirement factory — never dropped — and
/// the store still closes to its exact terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn prefix_ring_evictions_retire_through_the_snapshot_factory() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "ring", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(complete_test_catalog(DocumentStoreOwners::admit_source_constructor(physical_test_close_grant(), || (ExactDemoSnapshotRetirementFactory(Arc::clone(&completed)), DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new())))).unwrap_or_else(|(error,_)|panic!("original catalog installation refused: {error}"));
    for n in 1..=6 {
        apply(&mut store, vec![set(n)]).await;
    }
    let ids = operation_ids(&store);
    let preview = store.state_before(&ids[5], &protocol::HistoryInputDrafts::new()).expect("preview");
    drive_retirement_terminal(admit_demo_alias_retirement(&store,preview));
    let retained: Vec<usize> = store.live.prefix_ring.iter().map(|entry| entry.length).collect();
    assert!(!retained.is_empty() && retained.iter().all(|length| *length >= 1));
    let displaced = store.owners.displaced_retirements.owners.len();
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(40)))] }).await.expect("supersede");
    assert!(store.owners.displaced_retirements.owners.len() > displaced, "stale ring entries retire through displaced owners");
    let live = forward_prefix_digests::<DemoSnapshot, DemoMutation, _, _>(store.cursor.revision_accumulator.identity_digest, store.applied_edit_ids(), &store.envelope().vcs.edits, store.supersessions()).expect("digests");
    assert!(store.live.prefix_ring.iter().all(|entry| live.get(entry.length) == Some(&entry.digest)), "no stale entry survives");
    close_demo_artifact_store(&mut store);
    assert!(completed.load(std::sync::atomic::Ordering::SeqCst) > retained.len(), "every evicted and live root retired exactly");
}
//#endregion 🧪️SupersedeLaws

//#region 🧪️Metadata
/// 🧾️ Local and remote operations carry their semantic kind, descriptor label and tool transaction, on the wire and
/// through `.spr` persistence.
#[semio_framework_async_macros::async_test]
async fn operations_carry_semantic_kind_label_and_transaction_everywhere() {
    let transaction = protocol::TransactionRef::mint(&ActorId("author".into()), &HybridLogicalTimestamp::new(1, 1), "demo#drag");
    let mut store = demo_store("metadata", Some(0)).await;
    fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![set(3), add(1)], transaction: Some(transaction.clone()) }).await.expect("apply in a transaction");
    let expect = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| {
        let operations = store.mutation_ops().expect("operations");
        assert_eq!(operations.len(), 2);
        for (operation, (kind, label)) in operations.iter().zip([("demo/v1#set-n", "Set N"), ("demo/v1#add-n", "Add N")]) {
            let meta = operation.meta.expect("metadata");
            assert_eq!(meta.semantic_kind.as_ref().map(|kind| kind.0.as_str()), Some(kind));
            assert_eq!(meta.label.as_deref(), Some(label));
            assert_eq!(operation.transaction, Some(&transaction));
        }
    };
    expect(&store);
    let mut peer = demo_store("metadata", Some(0)).await;
    for event in store.event_log().expect("log") {
        assert_eq!(event.transaction.as_ref(), Some(&transaction), "the wire carries the transaction");
        peer.ingest_remote(event).await.expect("peer ingests");
    }
    expect(&peer);
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pack parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    expect(&reloaded);
}

/// 🎞️ `Supersede`, `CreateAlternativeWithSupersede` and a transaction-carrying `Apply` round-trip the value, text and
/// binary command codecs.
#[semio_framework_async_macros::async_test]
async fn supersede_commands_round_trip_every_codec() {
    let transaction = protocol::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "demo#drag".into() };
    let commands: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Supersede { scope: Some("alternative-1".into()), inputs: vec![input(&MutationId("m-1".into()), Some(set(3))), input(&MutationId("m-2".into()), None)] },
        ArtifactCommand::Supersede { scope: None, inputs: vec![input(&MutationId("m-3".into()), None)] },
        ArtifactCommand::CreateAlternativeWithSupersede { name: "variant".into(), inputs: vec![input(&MutationId("m-1".into()), Some(add(2)))] },
        ArtifactCommand::Apply { mutations: vec![set(1)], transaction: Some(transaction.clone()) },
        ArtifactCommand::ApplyInLane { mutations: vec![add(1)], lane: HistoryLane::Interaction, transaction: Some(transaction) },
    ];
    for command in commands {
        let text = print_command(&command).await.expect("prints");
        assert_eq!(parse_command::<DemoSnapshot, DemoMutation>(&text).await.expect("parses"), command, "{text}");
        let bytes = command.encode_command().expect("encodes");
        assert_eq!(ArtifactCommand::<DemoMutation>::decode_command::<DemoSnapshot>(&bytes).expect("decodes"), command);
        assert_eq!(ArtifactCommand::<DemoMutation>::from_value(command.to_value()).expect("value decodes"), command);
    }
    assert!(parse_command::<DemoSnapshot, DemoMutation>("supersede\n  replace m-1\n").await.is_err(), "a replace line needs its replacement");
    assert!(parse_command::<DemoSnapshot, DemoMutation>("supersede\n").await.is_err(), "a supersede needs an input");
}
//#endregion 🧪️Metadata
