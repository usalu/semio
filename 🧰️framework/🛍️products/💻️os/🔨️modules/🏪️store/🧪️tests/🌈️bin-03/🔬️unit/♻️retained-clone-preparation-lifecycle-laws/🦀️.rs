/// ♻️ Retained clone preparation lifecycle laws: the one item grant fixture, the neutral lifecycle oracle, the interrupted publication handoff, the contiguous capacity refusal and the single retry ack with a move only root.

fn fixture_one_item_grant(row: &serde_json::Value) -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items:row["maximumItems"].as_u64().expect("fixture supplied items") as usize,maximum_copy_bytes:row["maximumCopyBytes"].as_u64().expect("fixture supplied copy") as usize,maximum_capacity_bytes:row["maximumCapacityBytes"].as_u64().expect("fixture supplied capacity") as usize,maximum_release_bytes:row["maximumReleaseBytes"].as_u64().expect("fixture supplied release") as usize,maximum_depth:row["maximumDepth"].as_u64().expect("fixture supplied depth") as usize }
}

fn close_retained_clone_preparation_publication<P: Send + Sync + 'static, M: Send + 'static>(publication: &mut ArtifactStoreBatchPublication<P, M>, grant: RetainedCloneGrant) -> usize {
    publication.begin_close();
    for turn in 1..10_000 {
        publication.next_copy_byte_demand().expect("current publication copy frontier");
        publication.next_capacity_byte_demand(grant.maximum_copy_bytes).expect("current publication capacity frontier");
        publication.next_release_byte_demand().expect("current publication release frontier");
        publication.next_depth_demand().expect("current publication depth frontier");
        let step = publication.close_step(grant).expect("retained clone publication consumes the supplied caller policy");
        assert!(step.progress().fits(grant) && step.progress().copied_items <= grant.maximum_items, "one close turn stays inside every independently granted axis");
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(publication.terminal_is_empty());return turn; }
    }
    panic!("retained clone publication did not reach terminal empty under the supplied policy");
}

#[semio_framework_async_macros::async_test]
async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let grant = fixture_one_item_grant(&fixture["grant"]);
    for (index, row) in fixture["cases"].as_array().expect("lifecycle cases").iter().enumerate() {
        let initial = row["initial"].as_i64().expect("initial") as i32;
        let value = row["value"].as_i64().expect("value") as i32;
        let kind = row["kind"].as_str().expect("kind");
        let mode = match kind {
            "rejection" => DemoRetainedCloneEditMode::Reject,
            "fault" => DemoRetainedCloneEditMode::Fault,
            "overBudget" => DemoRetainedCloneEditMode::OverBudget,
            _ => DemoRetainedCloneEditMode::Apply,
        };
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", row["id"].as_str().expect("case id"), DemoSnapshot { n: Some(initial) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let generation = store.generation_now();
        let revision = store.content_revision_now();
        let factory = retained_clone_preparation_factory(mode);
        let admitted = store.begin_apply_batch(
            semio_framework_job::OperationId(index as u64 + 1),
            generation,
            revision,
            store.local_actor_id().0.clone(),
            vec![DemoMutation::SetN(SetN { n: value })],
            HistoryLane::Document,
            Some(&factory),
            None,
        );
        let mut published = false;
        let mut terminal_empty = true;
        if kind == "rejection" {
            if let Ok(mut publication) = admitted {
                let mut refused = false;
                for _ in 0..262_144 {
                    match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
                        Err(_) | Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => { refused = true; break; }
                        Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("{} published a rejected gesture", row["id"]),
                        Ok(_) => {}
                    }
                }
                assert!(refused, "{} rejects before a SnapshotRead owner is transferred", row["id"]);
                publication.begin_close();
                close_retained_clone_preparation_publication(&mut publication, grant.retained_grant());
                terminal_empty = publication.terminal_is_empty();
            }
        } else {
            let mut publication = admitted.unwrap_or_else(|rejected| panic!("{} admission: {}", row["id"], rejected.reason));
            let interrupt = row["interruptAfterTurns"].as_u64().expect("interrupt turns") as usize;
            for _ in 0..interrupt {
                match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
                    Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => {
                        published = true;
                        break;
                    }
                    Ok(ArtifactStoreOneItemAdvance::PreparationRefused) if kind == "fault" => {
                        assert!(publication.fault().expect("a refused preparation retains its fault").to_string().contains("fixture edit fault"));
                        break;
                    }
                    Ok(_) => {}
                    Err(error) if kind == "fault" => {
                        assert!(error.to_string().contains("fixture edit fault"));
                        break;
                    }
                    Err(error) => panic!("{} interrupted prefix: {error}", row["id"]),
                }
            }
            match kind {
                "success" => {
                    let mut last = None;
                    for _ in 0..262_144 {
                        let advance = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)).expect("retained clone success turn");
                        last = Some(advance);
                        match advance {
                            ArtifactStoreOneItemAdvance::Published(_, _) => {
                                published = true;
                                break;
                            }
                            ArtifactStoreOneItemAdvance::PreparationProgress(_, progress) => assert!(progress.fits(grant.retained_grant())),
                            ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::Blocked => {}
                            ArtifactStoreOneItemAdvance::AwaitingAck(_) | ArtifactStoreOneItemAdvance::Complete | ArtifactStoreOneItemAdvance::PreparationRefused => panic!("success reached a terminal state before publishing"),
                        }
                    }
                    assert!(published, "success publishes within its declared retained turn envelope; last advance {last:?}, grant {grant:?}, phase {:?}", publication.phase());
                    assert!(publication.acknowledge());
                }
                "cancel" => assert!(store.cancel_apply_batch(&mut publication)),
                "stale" => {
                    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: initial + 1 })], transaction: None }).await.expect("superseding edit publishes");
                    let error = match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
                        Err(error) => error,
                        Ok(_) => panic!("retained clone publication must reject its stale base"),
                    };
                    assert!(error.to_string().contains("stale"));
                }
                "fault" => {
                    let mut faulted = publication.fault().is_some();
                    for _ in 0..262_144 {
                        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
                            Err(error) => {
                                assert!(error.to_string().contains("fixture edit fault"));
                                faulted = true;
                                break;
                            }
                            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("faulting edit published"),
                            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                                assert!(publication.fault().expect("a refused preparation retains its fault").to_string().contains("fixture edit fault"));
                                faulted = true;
                                break;
                            }
                            Ok(_) => {}
                        }
                    }
                    assert!(faulted, "fault case reaches its injected retained edit fault");
                    publication.begin_close();
                }
                "overBudget" => {
                    let mut refusal = None;
                    for _ in 0..262_144 {
                        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
                            Err(error) => {
                                refusal = Some(error);
                                break;
                            }
                            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("over-budget retained edit published"),
                            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                                refusal = Some(VcsError::ValidationFailed(publication.fault().expect("a refused preparation retains its fault").to_string()));
                                break;
                            }
                            Ok(_) => {}
                        }
                    }
                    let error = refusal.expect("over-budget retained edit is refused before publication");
                    assert!(error.to_string().contains("exceeded its item, copy, capacity, or release grant"), "over-budget refusal text: {error}");
                    publication.begin_close();
                }
                _ => {}
            }
            let close_turns = close_retained_clone_preparation_publication(&mut publication, grant.retained_grant());
            assert!(close_turns > 1, "the retained owner lifecycle remains interruptible through terminal close");
            terminal_empty = publication.terminal_is_empty();
        }
        let actual = serde_json::json!({
            "published": published,
            "value": store.snapshot_ref().n,
            "history": store.applied_edit_ids().len(),
            "terminalEmpty": terminal_empty,
        });
        assert_eq!(actual, row["expected"], "{} matches the language-neutral lifecycle oracle", row["id"]);
        if kind == "success" {
            test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("retained clone publication undo");
            assert_eq!(store.snapshot_ref().n, Some(initial));
            test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("retained clone publication redo");
            assert_eq!(store.snapshot_ref().n, Some(value));
        }
        close_demo_artifact_store(&mut store);
    }
}

#[semio_framework_async_macros::async_test]
async fn interrupted_retained_clone_publication_handoffs_to_store_maintenance() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let row = &fixture["handoff"];
    let grant = fixture_one_item_grant(&row["closeGrant"]);
    let dropped = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> =
        Arc::new(ProbedRetainedClonePreparationFactory { inner: retained_clone_preparation_factory(DemoRetainedCloneEditMode::Apply), dropped: Arc::clone(&dropped) });
    let initial = DemoSnapshot { n: Some(31) };
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-driver-loss", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut publication = Some(
        store
            .begin_apply_batch(
                semio_framework_job::OperationId(71),
                store.generation_now(),
                store.content_revision_now(),
                "retained-driver-loss".into(),
                vec![DemoMutation::SetN(SetN { n: 32 })],
                HistoryLane::Document,
                Some(&factory),
                None,
            )
            .expect("retained clone publication is admitted"),
    );
    for _ in 0..65_536 {
        let driver = publication.as_mut().expect("live publication driver");
        assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(driver, row["closeGrant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
        if driver.preparation.is_some() { break; }
    }
    assert!(publication.as_ref().is_some_and(|driver| driver.preparation.is_some()), "the driver is interrupted while it owns the retained preparation cursor");
    assert!(store.handoff_batch_publication_close(&mut publication).expect("Store accepts publication close handoff"));
    assert!(publication.is_none(), "the interrupted driver retains no cursor owner after handoff");
    assert_eq!(dropped.load(std::sync::atomic::Ordering::SeqCst), 0, "the retained preparation remains unfinished after driver loss inside Store maintenance");
    let mut close_turns = 0usize;
    loop {
        close_turns += 1;
        let demand = store.maintenance_retirements_demands(grant.maximum_copy_bytes).expect("Store maintenance quotes the handed-off publication frontier");
        let turn = grant.retained_grant();
        assert!(demand.copy_bytes <= turn.maximum_copy_bytes && demand.capacity_bytes <= turn.maximum_capacity_bytes && demand.release_bytes <= turn.maximum_release_bytes && demand.depth <= turn.maximum_depth);
        let step = store.maintenance_retirements_step(turn).expect("Store maintenance retires the handed-off publication");
        assert!(step.progress().fits(turn) && step.progress().copied_items <= grant.maximum_items);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        assert!(close_turns < 4_096, "Store maintenance reaches terminal emptiness");
    }
    let actual = serde_json::json!({
        "driverOwnsCursor": publication.is_none() && dropped.load(std::sync::atomic::Ordering::SeqCst) == 1,
        "sourcePreserved": store.snapshot_ref() == &initial && store.applied_edit_ids().is_empty(),
        "closeRequiresMultipleTurns": close_turns > 1,
        "terminalEmpty": store.maintenance_retirements_terminal_is_empty(),
    });
    assert_eq!(actual, row["expected"], "Store driver-loss handoff matches the language-neutral oracle");
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn retained_clone_preparation_refuses_contiguous_capacity_larger_than_one_store_grant() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let grant = fixture_one_item_grant(&fixture["grant"]);
    let text_bytes = fixture["largeCapacity"]["stringByteLength"].as_u64().expect("large string byte length") as usize;
    let expected_code = fixture["largeCapacity"]["expectedCode"].as_str().expect("large capacity refusal code");
    let initial = RetainedTextSnapshot { text: "x".repeat(text_bytes) };
    let mut store = ArtifactStore::bare(create_document_envelope::<RetainedTextSnapshot, RetainedTextMutation>("retained.text/v1", "retained-text-capacity", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(complete_test_catalog(DocumentStoreOwners::admit_source_constructor(physical_test_close_grant(), || (semio_framework_value::factory::SharedValueRetirementFactory::<RetainedTextSnapshot>::default(), semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextSnapshot>::default(), semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextMutation>::default(), ArtifactStoreCursorDisposer::<RetainedTextSnapshot, RetainedTextMutation>::new())))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<RetainedTextSnapshot, RetainedTextMutation>> = Arc::new(
        RetainedClonePreparationFactory::new(
            Arc::new(RetainedTextEdit),
            Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextMutation>::default()),
            Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<RetainedTextSnapshot>::default()),
            64,
        )
        .expect("retained text preparation factory"),
    );
    let mut publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(41),
            store.generation_now(),
            store.content_revision_now(),
            "retained-text-capacity".into(),
            vec![RetainedTextMutation::SetRetainedText(SetRetainedText { text: "y".into() })],
            HistoryLane::Document,
            Some(&factory),
            None,
        )
        .expect("the operation-wide footprint is admitted before per-turn contiguous allocation");
    let mut refusal = None;
    let mut last = None;
    for _ in 0..65_536 {
        let advance = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving));
        last = Some(format!("{advance:?}"));
        match advance {
            Err(error) => {
                refusal = Some(error);
                break;
            }
            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                refusal = Some(VcsError::ValidationFailed(publication.fault().expect("a refused preparation retains its fault").to_string()));
                break;
            }
            Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..) | ArtifactStoreOneItemAdvance::Blocked) => {}
            Ok(_) => panic!("oversized contiguous owner reached an invalid state before refusal"),
        }
    }
    let error = refusal.unwrap_or_else(|| panic!("a contiguous string larger than the turn capacity grant cannot be allocated; last advance {last:?}, grant {grant:?}"));
    assert!(error.to_string().contains(expected_code), "capacity refusal exposes the stable language-neutral code: {error}");
    assert_eq!(store.snapshot_ref(), &initial);
    assert!(store.applied_edit_ids().is_empty());
    assert!(close_retained_clone_preparation_publication(&mut publication, grant.retained_grant()) > 1);
    close_test_store(&mut store);
    assert!(SpaceMember::close_owned_terminal_is_empty(&store));
}

/// 🎟️ Supplies the immutable plain test cleanup policy independently of the owner's frontier.
fn ephemeral_test_close_grant() -> ArtifactStoreOneItemGrant {
    let grant = physical_test_close_grant();
    ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth }
}

fn close_ephemeral_publication(publication: &mut ArtifactEphemeralOneItemPublication<DemoSnapshot, DemoMutation>) {
    for _ in 0..64 {
        let grant = ephemeral_test_close_grant();
        publication.retirement_demands(grant.maximum_copy_bytes).expect("actual cleanup frontier is fallible, not grant authority");
        let step = publication.close_step(grant).expect("ephemeral publication closes");
        assert!(step.progress().fits(grant.retained_grant()));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(publication.terminal_is_empty());
            return;
        }
    }
    panic!("ephemeral publication did not reach terminal empty");
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_single_retry_ack_and_move_only_root_preserve_generation_revision_and_history() {
    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-single", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let factory = Arc::new(DemoOneItemPreparationFactory::admissible());
    let admitted: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factory.clone();
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 7 })], HistoryLane::Document, Some(&admitted), None)
        .expect("explicit domain factory admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: PREPARATION_TURN_COPY_BYTES,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    let receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded durable step") {
            break receipt;
        }
    };
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(store.snapshot().expect("published snapshot"), DemoSnapshot { n: Some(7) });
    assert_eq!(store.applied_edit_ids().len(), 1);
    assert_ne!(store.content_revision_now(), revision);
    let prepared_root = factory.published_root.lock().expect("root witness lock").as_ref().and_then(std::sync::Weak::upgrade).expect("published root remains owned");
    assert!(Arc::ptr_eq(&prepared_root, &store.snapshot_root()), "commit moves the exact prepared Arc root");
    assert!(publication.retry());
    assert!(publication.retry());
    assert!(!publication.retry());
    assert_eq!(store.generation_now(), generation + 1, "retry never republishes");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}
