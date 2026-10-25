/// 📚️ Batch publication and stamped clock laws: the demo batch publication helpers, the reset capacity, the two hundred mutation batch, the single mutation batch, the fold and commit refusals, the mid batch rejection and cancel and the stamped publication clock.

/// 🧺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B: drives one batched publication to its receipt
/// under a bounded per-turn grant, exactly as a host actor tick drives it.
async fn publish_demo_batch(
    store: &mut ArtifactStore<DemoSnapshot, DemoMutation>,
    operation: u64,
    mutations: Vec<DemoMutation>,
) -> (ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>, Result<LaneItemReceipt, VcsError>) {
    publish_demo_batch_with(store, operation, mutations, DemoOneItemPreparationFactory::admissible()).await
}

/// 🧺️ The same drive against an explicitly chosen app preflight, so a footprint declaration can be
/// varied without touching any other admission gate.
async fn publish_demo_batch_with(
    store: &mut ArtifactStore<DemoSnapshot, DemoMutation>,
    operation: u64,
    mutations: Vec<DemoMutation>,
    app_factory: DemoOneItemPreparationFactory,
) -> (ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>, Result<LaneItemReceipt, VcsError>) {
        let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(app_factory);
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), store.local_actor_id().0.clone(), mutations, HistoryLane::Document, Some(&factory), None)
        .unwrap_or_else(|rejected| panic!("batched admission: {}", rejected.reason));
    let grant = batch_turn_grant();
    for _ in 0..1_048_576 {
        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)) {
            Ok(ArtifactStoreOneItemAdvance::Published(receipt, _)) => return (publication, Ok(receipt)),
            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                let reason = publication.fault().unwrap_or("batched preparation refused").to_string();
                return (publication, Err(VcsError::ValidationFailed(reason)));
            }
            Ok(_) => {}
            Err(error) => return (publication, Err(error)),
        }
    }
    panic!("batched publication never reached its receipt inside its bounded turn budget: phase={:?}, checkpoint={:?}, fault={:?}", publication.phase(), publication.progress(), publication.fault());
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_reset_preserves_capacity_for_retained_batch_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔁️reset-publication/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let initial = row["initial"].as_i64().unwrap() as i32;
        let after = row["after"].as_i64().unwrap() as i32;
        let fresh = || create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "reset-publication", DemoSnapshot { n: Some(initial) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = ArtifactStore::bare(fresh()).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        store.reset(fresh()).await.expect("empty-history reset");
        let (mut publication, receipt) = publish_demo_batch(&mut store, 41, vec![DemoMutation::SetN(SetN { n: after })]).await;
        assert!(publication.acknowledge() || receipt.is_err());
        publication.begin_close();
        close_durable_publication(&mut publication);
        let outcome = receipt.map(|_| serde_json::to_value(store.snapshot_ref().n).unwrap());
        close_demo_artifact_store(&mut store);
        assert_eq!(outcome.unwrap(), row["after"]);
    }
}

/// 🧺️ ONE gesture of 200 mutations is ONE `Edit` in ONE ledger slot and ONE undo step — the
/// `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` ceiling that used to fault a `setActiveExample` load at
/// its 65th mutation is structurally out of reach, and the staged edit is byte-for-byte the edit
/// `ArtifactCommand::Apply` records for the same mutation list.
///
/// 🧮️ `SetN`, not `AddN`: `AddN` emits one info message per operation and 200 of them exceed the
/// edit-message byte authority `ArtifactCommand::Apply` records — a limit of the message lane, not
/// of the staged edit, and the oracle below must apply the very same list.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step() {
    const ITEMS: usize = 200;
    const { assert!(ITEMS > semio_framework_os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY, "the fixture must exceed the fixed edit ledger capacity to prove the ceiling is gone") };
    let mutations = (0..ITEMS).map(|index| DemoMutation::SetN(SetN { n: index as i32 + 1 })).collect::<Vec<_>>();
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-gesture", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let (mut publication, receipt) = publish_demo_batch(&mut store, 1, mutations.clone()).await;
    let receipt = receipt.expect("a two-hundred item gesture publishes once");
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(publication.admitted_items(), ITEMS);
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32), "every admitted mutation folded against the running post root");
    assert_eq!(store.applied_edit_ids().len(), 1, "one gesture consumes exactly one applied-edit slot");
    assert_eq!(store.log.envelope.vcs.edits.len(), 1, "one gesture consumes exactly one history ledger slot");
    let staged = store.log.envelope.vcs.edits.last().expect("staged gesture edit").clone();
    assert_eq!(staged.forwards.len(), ITEMS);
    assert_eq!(staged.mutation_meta.len(), ITEMS);
    assert_eq!(staged.inverse.len(), ITEMS);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let mut oracle = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-oracle", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    oracle.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    test_support::dispatch_test_command(&mut oracle, ArtifactCommand::Apply { mutations, transaction: None }).await.expect("the batched command oracle applies the same list");
    let expected = oracle.log.envelope.vcs.edits.last().expect("oracle edit");
    assert_eq!(staged.forwards, expected.forwards, "a staged gesture records the same forwards ArtifactCommand::Apply does");
    assert_eq!(staged.inverse, expected.inverse, "a staged gesture records the same inverse ordering ArtifactCommand::Apply does");
    assert_eq!(oracle.snapshot_ref().n, store.snapshot_ref().n);
    close_demo_artifact_store(&mut oracle);

    let post = store.snapshot_ref().clone();
    let mut backwards = post.clone();
    for operation in staged.inverse.iter().rev() {
        backwards = protocol::apply_diff_cold(operation.diff(&backwards).diff(), &backwards).expect("staged inverse applies tail-first");
    }
    assert_eq!(backwards.n, Some(0), "the staged inverse is applied in reverse order of the forwards");
    let mut forwards = post;
    for operation in &staged.inverse {
        forwards = protocol::apply_diff_cold(operation.diff(&forwards).diff(), &forwards).expect("staged inverse applies");
    }
    assert_ne!(forwards.n, Some(0), "consuming the staged inverse head-first is NOT the undo of the gesture");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("one gesture is one undo step");
    assert_eq!(store.snapshot_ref().n, Some(0), "one undo reverts the whole gesture");
    assert!(store.applied_edit_ids().is_empty());
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("one gesture is one redo step");
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32));
    close_demo_artifact_store(&mut store);
}

/// 🧺️ The single-mutation publication is literally the `N = 1` case of the same machine: same
/// phases, same receipt, one ledger slot — and the same identity rule as `ArtifactCommand::Apply` (audit W1G-8): its one
/// mutation is named after its edit, so a remote replica's one-forward edit resolves every transition naming it.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_of_one_mutation_is_the_single_item_case() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-single", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch(&mut store, 2, vec![DemoMutation::SetN(SetN { n: 7 })]).await;
    receipt.expect("a single-item gesture publishes");
    assert_eq!(publication.admitted_items(), 1);
    assert_eq!(store.snapshot_ref().n, Some(7));
    let staged = store.log.envelope.vcs.edits.last().expect("staged edit");
    assert_eq!(staged.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
    assert_eq!(staged.inverse.iter().cloned().collect::<Vec<_>>(), vec![DemoMutation::AssignN(AssignN { n: Some(0) })]);
    assert_eq!(staged.mutation_meta[0].mutation_id, Some(MutationId(staged.id.clone())), "a one-mutation gesture names its mutation after its edit on the batched route as on every other");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    let (mut pair, receipt) = publish_demo_batch(&mut store, 3, vec![DemoMutation::SetN(SetN { n: 8 }), DemoMutation::SetN(SetN { n: 9 })]).await;
    receipt.expect("a two-item gesture publishes");
    let staged = store.log.envelope.vcs.edits.last().expect("staged two-item edit");
    assert_eq!(staged.mutation_meta.iter().map(|meta| meta.mutation_id.clone()).collect::<Vec<_>>(), vec![Some(MutationId(format!("{}#0", staged.id))), Some(MutationId(format!("{}#1", staged.id)))], "a genuine multi-mutation gesture keeps its positional ids");
    assert!(pair.acknowledge());
    close_durable_publication(&mut pair);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ The fold contract, stated from both sides — the law the whole procedural 3d app boot broke.
/// `ArtifactStoreOneItemFootprint::work_items` counts staged edit ROWS (one forward plus every
/// inverse row), so a durable preflight that declares ONE work item for a point-invertible mutation
/// fail-closes its own single-item gesture inside `fold_batch_item`, with the exact message the
/// runtime reported; the very same gesture through
/// [`ArtifactStoreOneItemFootprint::for_leaf`] publishes and stages both rows. The
/// store's contract is therefore exactly what its own fixtures declare — the `work_items: 1` that ~40
/// app preflights carried was the defect (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_fold_refuses_a_one_work_item_declaration_and_accepts_the_invertible_one() {
    assert_eq!(ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 1 }), 512), ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 512 });
    assert_eq!(ArtifactStoreOneItemFootprint::for_ephemeral_item(512), ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 });

    let mut under = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-under-declared", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    under.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = under.generation_now();
    let (mut publication, receipt) = publish_demo_batch_with(&mut under, 7, vec![DemoMutation::SetN(SetN { n: 9 })], DemoOneItemPreparationFactory::under_declared()).await;
    assert_eq!(receipt.expect_err("a one-work-item declaration cannot carry a point-invertible item"), VcsError::TooLarge { rows: 2, capacity: 1 });
    assert_eq!(under.generation_now(), generation, "a refused fold publishes nothing");
    assert_eq!(under.snapshot_ref().n, Some(0));
    assert!(under.applied_edit_ids().is_empty());
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut under);

    let mut exact = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-invertible", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    exact.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch_with(&mut exact, 8, vec![DemoMutation::SetN(SetN { n: 9 })], DemoOneItemPreparationFactory::admissible()).await;
    receipt.expect("the invertible declaration carries the same item");
    assert_eq!(exact.snapshot_ref().n, Some(9));
    let staged = exact.log.envelope.vcs.edits.last().expect("staged edit");
    assert_eq!(staged.forwards.len() + staged.inverse.len(), ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 0 }), 0).work_items, "the exact declaration is the rows the item actually folds");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut exact);
}

/// 🧺️ The SAME law for the multi-item gesture, and the reason the defect stayed invisible until a
/// one-mutation command ran it: `fold_batch_item` compares each candidate against the GESTURE-WIDE
/// merged declaration, so `N` under-declared items still fold (2 rows ≤ `N` for every `N ≥ 2`) and
/// only the `PreflightingCommit` gate catches the overrun — with the sibling message
/// `batched prepared candidate failed its exact fixed commit contract`, `2N` staged rows against a
/// declared `N`. The single-item case is the one with zero slack, which is why the boot-time
/// `setActiveExample` was the first thing to die. Both gates pass exactly when every item declares
/// through [`ArtifactStoreOneItemFootprint::for_leaf`]
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture_and_accepts_the_invertible_one() {
    const ITEMS: usize = 3;
    let mutations = || (0..ITEMS).map(|index| DemoMutation::SetN(SetN { n: index as i32 + 1 })).collect::<Vec<_>>();
    assert_eq!(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 }.merged(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 }).work_items, 2, "a merged declaration sums the items' rows");

    let mut under = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-batch-under-declared", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    under.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = under.generation_now();
    let (mut publication, receipt) = publish_demo_batch_with(&mut under, 9, mutations(), DemoOneItemPreparationFactory::under_declared()).await;
    assert_eq!(
        receipt.expect_err("an under-declared multi-item gesture cannot commit its inverse rows"),
        VcsError::TooLarge { rows: 6, capacity: 3 },
        "the per-item fold admits every candidate at N >= 2; the gesture-wide commit gate is what refuses"
    );
    assert_eq!(under.generation_now(), generation, "a refused commit publishes nothing");
    assert_eq!(under.snapshot_ref().n, Some(0));
    assert!(under.applied_edit_ids().is_empty());
    assert!(under.log.envelope.vcs.edits.is_empty(), "a refused gesture consumes no ledger slot");
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut under);

    let mut exact = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-batch-invertible", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    exact.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch_with(&mut exact, 10, mutations(), DemoOneItemPreparationFactory::admissible()).await;
    receipt.expect("the invertible declaration carries the same gesture");
    assert_eq!(exact.snapshot_ref().n, Some(ITEMS as i32));
    let staged = exact.log.envelope.vcs.edits.last().expect("staged gesture edit");
    assert_eq!(staged.forwards.len(), ITEMS);
    assert_eq!(staged.inverse.len(), ITEMS);
    assert_eq!(staged.forwards.len() + staged.inverse.len(), ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 0 }), 0).work_items * ITEMS, "the merged invertible declaration is exactly the rows the gesture folds");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut exact);
}

/// 🧺️ A gesture whose fourth mutation cannot prepare against the running post root commits
/// NOTHING: no ledger slot, no applied edit, no root replacement, no generation bump — and the
/// half-staged edit still closes to its exact terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_rejection_mid_batch_leaves_no_partial_edit_and_still_retires_exactly() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-rejection", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let mutations = vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::AddN(AddN { delta: 2 }), DemoMutation::DeleteN(DeleteN {}), DemoMutation::SetN(SetN { n: 5 })];
    let (mut publication, outcome) = publish_demo_batch(&mut store, 3, mutations).await;
    let error = outcome.expect_err("the fourth mutation cannot prepare against a deleted target");
    assert!(error.to_string().contains("demo retained mutation rejected against its base"), "{error}");
    assert!(publication.staged_items() < publication.admitted_items(), "the batch faulted before every item was staged");
    assert_eq!(store.generation_now(), generation, "a rejected gesture never bumps the generation");
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "a rejected gesture never replaces the root");
    assert!(store.applied_edit_ids().is_empty(), "a rejected gesture leaves no partial applied edit");
    assert!(store.log.envelope.vcs.edits.is_empty(), "a rejected gesture consumes no history ledger slot");
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ Cancelling a gesture mid-flight retires the already-staged forwards, inverses, metadata and
/// running post root one owner per bounded turn, and never publishes.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-cancel", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let root = store.snapshot_root();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(4), generation, store.content_revision_now(), "retained-test".into(), (0..32).map(|_| DemoMutation::AddN(AddN { delta: 1 })).collect(), HistoryLane::Document, Some(&factory), None)
        .expect("a thirty-two item gesture admits");
    let grant = batch_turn_grant();
    while publication.staged_items() < 3 {
        receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("bounded staging turn");
    }
    assert!(store.cancel_apply_batch(&mut publication));
    assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Blocked)));
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation, "a cancelled gesture never publishes");
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));
    assert!(store.log.envelope.vcs.edits.is_empty());
    close_demo_artifact_store(&mut store);
}


#[semio_framework_async_macros::async_test]
async fn retained_latest_wins_cold_rebase_preserves_admitted_cursor_capacity_for_next_publication() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔌️plugin/🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "cold-retained-rebase", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let capacity = store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 13 })], transaction: None }).await.unwrap();
    assert_eq!(store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert_eq!(serde_json::json!(store.generation_now()), fixture["rebase"]["afterGeneration"]);
    let revision = store.content_revision_now();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store.begin_apply_batch(semio_framework_job::OperationId(93), store.generation_now(), revision, "fixture".into(), vec![DemoMutation::SetN(SetN { n: 97 })], HistoryLane::Document, Some(&factory), None).unwrap();
    let grant = fixture_one_item_grant(&fixture);
    for _ in 0..16_384 {
        if matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)).unwrap(), ArtifactStoreOneItemAdvance::Published(_, _)) {
            break;
        }
    }
    assert_eq!(serde_json::to_value(store.snapshot_root().as_ref()).unwrap(), serde_json::json!({ "n": 97 }));
    assert_eq!(store.generation_now(), 2);
    assert_eq!(store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-digest-authority", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let forged: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::forged_digest());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(11), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 12 })], HistoryLane::Document, Some(&forged), None)
        .expect("Store-minted immutable authority admits the domain owner");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    while publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
        assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
    }
    let authority = publication.authority.as_ref().expect("publication retains Store authority");
    let prepared = publication.preparation.as_ref().and_then(|owner| owner.prepared()).expect("domain prepared candidate");
    assert_eq!(authority.operation(), semio_framework_job::OperationId(11));
    assert_eq!(authority.generation(), semio_framework_job::Generation(generation));
    assert_eq!(authority.base_revision(), revision);
    assert_eq!(authority.base_applied_edit_count(), 0);
    let canonical = authority.prepared_edit_digest(&prepared.edit()).expect("Store helper accepts its exact semantic edit");
    assert_eq!(canonical, CursorRevisionAccumulator::edit_digest(&prepared.edit()), "public helper and private Store validation share one canonical digest law");
    assert_ne!(prepared.edit_digest(), canonical, "hostile domain altered only its exposed candidate digest");
    let mut rejected = false;
    for _ in 0..65_536 {
        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)) {
            Err(_) | Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => { rejected = true; break; }
            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("a domain-forged digest published"),
            Ok(_) => {}
        }
    }
    assert!(rejected, "Store recomputation rejects a domain-forged digest before publication");
    publication.begin_close();
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "forged cursor/history authority cannot replace the root");
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-rejections", DemoSnapshot { n: Some(3) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let admissible: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let stale = store.begin_apply_batch(semio_framework_job::OperationId(2), generation + 1, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 9 })], HistoryLane::Document, Some(&admissible), None);
    assert!(stale.is_err());

    let mut cancelled =
        store.begin_apply_batch(semio_framework_job::OperationId(3), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 8 })], HistoryLane::Document, Some(&admissible), None).expect("fresh publication admits");
    assert!(store.cancel_apply_batch(&mut cancelled));
    close_durable_publication(&mut cancelled);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));

    for n in 1..=65 {
        test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: None }).await.expect("an edit past one history page");
    }
    assert_eq!(store.applied_edit_ids().len(), 65);
    assert_eq!(serde_json::to_value(store.snapshot_root().as_ref()).unwrap(), serde_json::json!({ "n": 65 }));
    assert_ne!(store.generation_now(), generation);
    close_demo_artifact_store(&mut store);
}

/// 🕰️ ticket 26/09/18 HT3a: a durable committer stamps a server-derived HLC on the edit it will
/// hash into its canonical command, so the Store must mint the publication authority at that exact
/// tick — the semantic-edit contract stays an exact equality and a stamp that is not strictly ahead
/// of the Store's own clock is refused before any owner is reserved.
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_behind() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-clock", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let stamp = HybridLogicalTimestamp { actor: 7, physical_ms: store.log.clock.physical_ms + 1_000, logical: 0 };
    let ahead: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(11), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 5 })], HistoryLane::Document, Some(&ahead), None)
        .expect("a stamped clock strictly ahead of the Store clock admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    let receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded stamped step") {
            break receipt;
        }
    };
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(store.log.clock, stamp, "the committed Store clock is the stamp the committer hashed into its command");
    let published = store.log.envelope.vcs.edits.last().expect("one published stamped edit");
    assert_eq!(published.mutation_meta.first().expect("stamped authority metadata").timestamp, stamp);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let behind: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let refused = store
        .begin_apply_batch(semio_framework_job::OperationId(12), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 6 })], HistoryLane::Document, Some(&behind), None)
        .err()
        .expect("a stamp at or behind the Store clock is refused");
    assert_eq!(refused.reason, "stamped publication clock is not strictly after this Store's own last published edit");
    assert_eq!(store.generation_now(), generation + 1, "a refused stamp never publishes");
    assert_eq!(store.log.clock, stamp);

    let same_millisecond = HybridLogicalTimestamp { actor: 7, physical_ms: stamp.physical_ms, logical: stamp.logical + 1 };
    let tied: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(same_millisecond));
    let mut tied_publication = store
        .begin_apply_batch(semio_framework_job::OperationId(13), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 7 })], HistoryLane::Document, Some(&tied), None)
        .expect("a second approval in the same millisecond carries the next logical tick and admits");
    let tied_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut tied_publication, 4096, grant,receiving)).expect("one bounded same-millisecond step") {
            break receipt;
        }
    };
    assert_eq!(tied_receipt, LaneItemReceipt { generation_before: generation + 1, generation_after: generation + 2 });
    assert_eq!(store.log.clock, same_millisecond, "the same-millisecond stamp is the committed clock, so the command hash still rebuilds");
    assert!(tied_publication.acknowledge());
    close_durable_publication(&mut tied_publication);
    close_demo_artifact_store(&mut store);
}

/// 🌱️ ticket 26/09/18 HT4: a Store seeds `clock` from `now_ms()` at construction, which is a fact
/// about the machine and not about the document. A document with no published edit of its own is
/// therefore after nothing, and a server-minted stamp far behind that seed — a hub preparing the
/// first approval on a genesis document — must publish rather than be refused against a wall clock
/// the document never saw.
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_clock_admits_a_genesis_document_behind_its_construction_seed() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-genesis-clock", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let stamp = HybridLogicalTimestamp { actor: 1, physical_ms: 1_004, logical: 0 };
    assert!(store.log.clock.physical_ms > stamp.physical_ms, "the construction seed is a wall-clock tick far ahead of the minted stamp");
    let genesis: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(21), generation, store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 3 })], HistoryLane::Document, Some(&genesis), None)
        .expect("a genesis document is after nothing, so the minted stamp admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded genesis step") {
            break;
        }
    }
    assert_eq!(store.log.clock, stamp, "the committed clock is the stamp, never the construction seed");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let behind: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let refused = store
        .begin_apply_batch(semio_framework_job::OperationId(22), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 4 })], HistoryLane::Document, Some(&behind), None)
        .err()
        .expect("once this Store has published an edit, its clock is a document fact and the replay is refused");
    assert_eq!(refused.reason, "stamped publication clock is not strictly after this Store's own last published edit");
    close_demo_artifact_store(&mut store);
}

/// 🔏 ticket 26/09/18 HT5: a durable committer hashes its canonical command over BOTH the stamped
/// clock and the stamped mutation id, then persists that hash. If the Store mints its own
/// content-addressed `edit_id()` for the publication anyway, the committed edit carries an identity
/// no verifier can bind back to the approval it executed — which is exactly what the hub's WAL
/// reported as "committed decision does not bind its approval target — mutation-id" and what the
/// actor frontier reported as `edit-…` where the approval's mutation id was required. So a stamped
/// publication publishes under the stamped identity, as `Edit.id` AND as the folded item's
/// `mutation_id`, while every unstamped gesture keeps the store's content-addressed mint (its one mutation named
/// after it, audit W1G-8).
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_carries_its_committed_identity_into_the_published_edit() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-identity", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };

    let approval = "7d6bd4f3246ae47f074ed7a02011c0cf";
    let stamp = HybridLogicalTimestamp { actor: 1, physical_ms: 1_004, logical: 0 };
    let stamped: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped_identity(stamp, approval));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(31), generation, store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 5 })], HistoryLane::Document, Some(&stamped), None)
        .expect("a stamped identity admits with its stamped clock");
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded stamped-identity step") {
            break;
        }
    }
    let published = store.log.envelope.vcs.edits.last().expect("one published stamped edit");
    assert_eq!(published.id, approval, "the published edit publishes under the identity the durable decision hashed");
    assert_eq!(published.mutation_meta.first().expect("stamped authority metadata").mutation_id.as_ref().map(|mutation| mutation.0.as_str()), Some(approval), "the folded item keeps the approval's mutation id, never the store's `#position` rewrite");
    assert_eq!(store.log.applied_edit_ids.last().map(String::as_str), Some(approval), "the applied cursor names the stamped identity, so the actor frontier's head edit is the approval");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let local: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut gesture = store
        .begin_apply_batch(semio_framework_job::OperationId(32), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 6 })], HistoryLane::Document, Some(&local), None)
        .expect("an unstamped local gesture admits unchanged");
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut gesture, 4096, grant,receiving)).expect("one bounded unstamped step") {
            break;
        }
    }
    let gestured = store.log.envelope.vcs.edits.last().expect("one published local edit");
    assert_ne!(gestured.id, approval, "an unstamped gesture keeps the identity its own preparation minted, never a stamped one, got {}", gestured.id);
    assert!(gestured.id.starts_with("retained-one-item-"), "the preparation factory mints the unstamped identity, got {}", gestured.id);
    assert_eq!(gestured.mutation_meta.first().expect("local authority metadata").mutation_id.as_ref().map(|mutation| mutation.0.clone()), Some(gestured.id.clone()), "an unstamped one-mutation gesture names its mutation after its edit");
    assert!(gesture.acknowledge());
    close_durable_publication(&mut gesture);
    close_demo_artifact_store(&mut store);
}
