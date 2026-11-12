    /// 📌️ Document cancellation and latest-wins keys: the retained document cancellation, exact keys against the serde oracle, producer child claims, contended finish, rebase and full registry reclaim.

use semio_plugin_command::command_admission::ToolLatestWinsRegistry;
use semio_plugin_testing_laws::artifact_app_laws::KeyedCancellationFixture;
use semio_os_store_command::HistoryLane;
pub(super) async fn retained_document_cancellation<A: ArtifactApp + Default>(factory: std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>, mutation: fn() -> A::Mutation, observe: fn(&A::Snapshot) -> i32) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        for case in fixture["publicationCases"].as_array().unwrap() {
            for ack_phase in 0..3 {
                let delayed_ack = ack_phase == 1;
                if ack_phase == 2 && case["cancelAt"] != "awaitingAck" { continue; }
                let boundary = case["cancelAt"].as_str().unwrap();
                let mounted_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("original typed fixture supplied identity authority");
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),mounted_policy,&mut identity).await.expect("the reference snapshot births");
                let before = app.stores.store.snapshot_root();
                let revision = app.stores.store.content_revision_now();
                let generation = app.stores.store.generation_now();
                let operation = semio_framework_job::Operation::new(if ack_phase == 2 { semio_framework_job::OperationId(41) } else { semio_framework_job::allocate_operation_id() }, semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(generation), 1);
                let lease = fixture_cancellation_lease(&mut app, &ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }, cx);
                let mut mounted = MountedTypedCommandFullOperation::<A> { reserved_construction: Default::default(),
                    source_identity: semio_framework_job::OriginalSourceBinding::borrow(cx),
                    original_emission_caller: None,
                    original_emission_decoder: None,
                    original_emission_body: None,
                    verb: "setGraphParameter".into(),
                    meta: ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None },
                    operation,
                    canonical_revision: revision,
                    artifact_generation: generation,
                    config_generation: 0,
                    draft_generation: 0,
                    presence_generation: 0,
                    transient_generation: 0,
                    window_config_authority: None,
                    window_transient_authority: None,
                    publication_lanes: &[ArtifactToolPublicationLane::Artifact], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
                    session: None,
                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                    reserved_producer: None, completion: None,
                    completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},
                    worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None,
                    terminal_seen: true,
                worker_resume_pending: false,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    tool_stage:semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Capture,tool_press_result:None,pending_artifact_publication:None,publication_failure:Default::default(), pending_stream_abort:None,stream_abort_resident:Default::default(), stream_abort_completed:false, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                    pending_child_publication: None,
                    owned_child_group: None,
                    owned_child_committed: false,
                    owned_child_result_pending: false,
                    captured_child_content: Some(semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage::from_cold(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    page_publication:semio_plugin_retained_command::typed_operation::MountedPagePublication::new(),result_page:None, diagnostics:None,
                    result_page_presented: false,
                    result_sequence: 0,
                    publication_progress: 0,
                    publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,actor_capture_pending:semio_plugin_mounted_owner::actor_capture::ActorCapture::new(),
                    publication_attempt: 0,
                    ui_pending: true,
                    progress: None,
                    progress_pending: false,
                    user_cancel_requested: false,
                    published_artifact: false,
                    published_config: false,
                    published_window_config: false,
                    command_logged: false,
                interaction_revalidated: false,
                    retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault: None,
                    stage: MountedTypedCommandFullOperationStage::Publishing,
                };
                if boundary != "producer" {
                    let pending =
                        app.stores.store.begin_apply_batch(operation.operation, generation, revision, "fixture".into(), vec![mutation()], HistoryLane::Document, Some(&factory), None).unwrap_or_else(|_| panic!("exact scalar document preparation admission"));
                    mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Artifact(pending));
                    let target = match boundary {
                        "preparation" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Preparing,
                        "preflight" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::PreflightingCommit,
                        "publishing" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing,
                        "awaitingAck" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::AwaitingAck,
                        _ => unreachable!(),
                    };
                    for _ in 0..100_000 {
                        let Some(PendingArtifactStorePublication::Artifact(pending)) = mounted.pending_artifact_publication.as_ref() else {
                            panic!("exact document pending owner");
                        };
                        if pending.phase() == target {
                            break;
                        }
                        publish_mounted_fixture_unit(&mut app, &mut mounted, mounted_policy.maintenance, cx);
                    }
                    let Some(PendingArtifactStorePublication::Artifact(pending)) = mounted.pending_artifact_publication.as_ref() else {
                        panic!("exact document pending owner");
                    };
                    assert_eq!(pending.phase(), target);
                }

                if ack_phase == 2 {
                    let ack_fixture: Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/📬️ack/🔣️.json")).unwrap();
                    let row = ack_fixture["cases"].as_array().unwrap().iter().find(|row| row["lane"] == "artifact").unwrap();
                    let mut encoded_receipt = mounted.result_page.take().expect("actual document publisher retained its committed receipt");
                    assert_eq!(ack_fixture_token(&row["initialToken"]), encoded_receipt.token, "the original page retains the exact actual committed receipt token");
                    close_ack_receiver(&mut encoded_receipt, mounted_policy.close, cx);
                    mounted.result_sequence = row["initialToken"]["sequence"].as_u64().unwrap() as u32;
                    mounted.result_page = Some(original_ack_page(&ack_fixture, row, TypedOperationResultLane::Artifact, mounted_policy.preparation, cx));
                    verify_original_ack_case(&mut mounted, row, &ack_fixture, mounted_policy.close, cx);
                    assert_eq!(observe(&app.stores.store.snapshot_root()), 42);
                    drop(before);
                    close_app(&mut app, "original artifact ACK regression", cx);
                    assert!(app.close_terminal_is_empty());
                    continue;
                }

                mounted.cancellation_lease.as_ref().unwrap().cancel();
                if boundary == "awaitingAck" {
                    let receipt = {let original=mounted.result_page.as_ref().unwrap();(original.token,original.bytes().to_vec())};
                    assert!(!mounted.reject_cancelled_publication().unwrap());
                    assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), receipt.1.as_slice());
                    if delayed_ack {
                        let mut presented = present_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation, cx).unwrap();
                        assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                        assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
                        for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                            assert!(present_mounted_page(&mut mounted, crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation, cx).is_none());
                        }
                        assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut presented,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
                retire_acknowledged_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close, cx);
                    } else {
                        assert!(mounted.acknowledge_result_page(receipt.0).unwrap());
                retire_acknowledged_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close, cx);
                    }
                }
                publish_mounted_fixture_unit(&mut app, &mut mounted, mounted_policy.maintenance, cx);
                let mut final_page = if delayed_ack {
                    let mut presented = present_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation, cx).unwrap();
                    let mut deliveries = 1;
                    for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                        deliveries += usize::from(present_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation, cx).is_some());
                    }
                    assert_eq!(deliveries, fixture["resultAck"]["deliveries"].as_u64().unwrap() as usize);
                    assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                    assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
                    presented
                } else {
                    present_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation, cx).unwrap()
                };
                assert_eq!(final_page.lane, TypedOperationResultLane::Fault);
                let committed = case["committed"].as_bool().unwrap();
                let actual = serde_json::json!({ "count": observe(&app.stores.store.snapshot_root()), "generation": app.stores.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.stores.store.snapshot_root()) });
                let expected = serde_json::json!({ "count": if committed { 42 } else { 0 }, "generation": u64::from(committed), "sameRoot": !committed });
                assert_eq!(actual, expected, "{boundary}, delayed ACK={delayed_ack}");
                let final_token = artifact_app_laws::close_fixture_result_page(&mut final_page, mounted_policy.close);
                 assert!(mounted.acknowledge_result_page(final_token).unwrap());
                close_mounted(&mut mounted, "document cancellation close", mounted_policy.close, cx);
                assert!(mounted.terminal_is_empty());
                drop(before);
                close_app(&mut app, "document cancellation app close", cx);
                assert!(app.close_terminal_is_empty());
                eprintln!("real mounted Document publication {boundary}, delayed ACK={delayed_ack}: count/revision/root retained and close terminal");
            }
        }
    }

    /// 🗝️ Admits one latest-wins operation through a funded original turn and reports whether the registry accepted it.
    fn begin_registry(registry: &mut ToolLatestWinsRegistry, operation: u64, key: &semio_framework_value_ordered::ordered::SharedOwner<String>, lease: &semio_plugin_operation::ToolCancellationLease) -> bool {
        let quoted = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        let mut bank = semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::BankWallet::new(quoted, 1 << 20));
        let (poll, _) = semio_framework_value_testing_admission::bank_run(&mut bank, quoted, 1 << 20, |effect| { let result = registry.begin(operation, key, lease, effect); semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result, work: effect.performed_work() }) });
        assert!(bank.into_wallet().is_ok());
        let semio_framework_value::RetainedWorkPoll::Complete(turn) = poll else { panic!("latest-wins admission left its funded turn pending") };
        turn.result.unwrap().is_some()
    }

    /// 🔓️ Releases one non-final alias of a fixture key through its quoted funded retirement.
    fn release_key_alias(key: &mut semio_framework_value_ordered::ordered::SharedOwner<String>) {
        let grant = semio_framework_value_testing_admission::demand_grant(key.retire_demand().unwrap());
        assert!(matches!(key.retire_step(grant, key.retire_work_demand().unwrap()).result.unwrap(), Some((semio_framework_value::retirement::arc_custody::original::SharedOriginalRelease::Alias, _))));
    }

    #[test]
    fn retained_latest_wins_full_domain_exact_keys_match_serde_oracle_and_retire() {
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let first = &fixture["first"];
        assert_eq!(first["target"].as_str().unwrap().len(), 8_192);
        for case in fixture["cases"].as_array().unwrap() {
            let next = &case["next"];
            assert_eq!(next["target"].as_str().unwrap().len(), 8_192);
            let oracle = first == next;
            assert_eq!(oracle, case["superseded"].as_bool().unwrap());
            let mut cancellations = KeyedCancellationFixture::open();
            let mut registry = ToolLatestWinsRegistry::new();
            let first_operation = semio_framework_job::allocate_operation_id();
            let mut first_lease = cancellations
                .begin_keyed(ToolOperationKey {
                    app_instance_id: first["instance"].as_u64().unwrap() as u32,
                    document: ArtifactDocumentAuthority(7),
                    operation_id: first_operation,
                    base_revision: semio_framework_job::RevisionId(1),
                    generation: semio_framework_job::Generation(0),
                });
            let mut first_key=fixture_latest_wins_key(first);
            assert!(begin_registry(&mut registry, first_operation.0, &first_key, &first_lease));
            release_key_alias(&mut first_key);
            for _ in 0..100_000 {
                if registry.take_outcome(first_operation.0) == Some(true) {
                    break;
                }
                advance_registry(&mut registry, "exact key admission");
            }
            assert!(!first_lease.token.is_cancelled_now());
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                advance_registry(&mut registry, "exact key supersession");
            }
            assert!(registry.can_begin());
            let next_operation = semio_framework_job::allocate_operation_id();
            let mut next_lease = cancellations
                .begin_keyed(ToolOperationKey {
                    app_instance_id: next["instance"].as_u64().unwrap() as u32,
                    document: ArtifactDocumentAuthority(7),
                    operation_id: next_operation,
                    base_revision: semio_framework_job::RevisionId(1),
                    generation: semio_framework_job::Generation(0),
                });
            let mut next_key=fixture_latest_wins_key(next);
            assert!(begin_registry(&mut registry, next_operation.0, &next_key, &next_lease));
            release_key_alias(&mut next_key);
            let mut accepted = false;
            for _ in 0..100_000 {
                if let Some(result) = registry.take_outcome(next_operation.0) {
                    accepted = result;
                    break;
                }
                advance_registry(&mut registry, "exact key comparison");
            }
            assert!(accepted);
            assert_eq!(first_lease.token.is_cancelled_now(), oracle, "{}", case["id"]);
            assert!(!next_lease.token.is_cancelled_now());
            first_lease.finish();
            next_lease.finish();
            cancellations.close_lease(first_lease);
            cancellations.close_lease(next_lease);
            assert_eq!(cancellations.handle().active_operation_count(), 0);
            registry.begin_close();
            for _ in 0..300_000 {
                if registry.terminal_is_empty() {
                    break;
                }
                advance_registry(&mut registry, "exact key close");
            }
            assert!(registry.terminal_is_empty());
            cancellations.close();
            eprintln!("exact latest-wins key {} matched independent serde scope equality and retired its 8192-byte identity", case["id"]);
        }
    }

    #[test]
    fn retained_latest_wins_producer_child_cannot_bypass_document_or_app_publication_claim() {
        let mut cancellations = KeyedCancellationFixture::open();
        let key = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(0),
        };
        let mut lease = cancellations.begin_keyed(key.clone());
        let producer = lease.cancel_token();
        producer.cancel_now();
        assert!(!lease.token.is_cancelled_now());
        assert!(lease.try_claim_publication().is_some());
        assert!(cancellations.handle().cancel_document(key.document).unwrap());
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        cancellations.close_lease(lease);
        let mut lease = cancellations.begin_keyed(ToolOperationKey { operation_id: semio_framework_job::allocate_operation_id(), ..key });
        let prior_claim = lease.try_claim_publication().unwrap();
        cancellations.handle().cancel_scope_generation();
        drop(prior_claim);
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        cancellations.close_lease(lease);
        assert_eq!(cancellations.handle().active_operation_count(), 0);
        cancellations.close();
    }

    #[test]
    fn retained_latest_wins_contended_finish_is_deferred_and_cannot_release_replacement() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let mut cancellations = KeyedCancellationFixture::open();
        let key = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(0),
        };
        let mut lease = cancellations.begin_keyed(key.clone());
        let cancellation_state = std::sync::Arc::clone(&cancellations.handle().state);
        let lock = cancellation_state.try_lock().unwrap();
        lease.finish();
        assert_eq!(serde_json::json!(cancellations.handle().active_operation_count()), fixture["deferredFinish"]["activeBeforeSweep"]);
        drop(lock);
        drop(cancellation_state);
        let index = TOOL_CANCELLATION_SLOTS + key.operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        assert!(cancellations.cleanup(index, true));
        cancellations.close_lease(lease);
        assert_eq!(serde_json::json!(cancellations.handle().active_operation_count()), fixture["deferredFinish"]["activeAfterSweep"]);
        let mut old = cancellations.begin_keyed(key.clone());
        assert!(cancellations.cleanup(index, false));
        let mut replacement = cancellations.begin_keyed(ToolOperationKey { generation: semio_framework_job::Generation(1), ..key });
        old.finish();
        assert_eq!(serde_json::json!(cancellations.handle().active_operation_count() == 1 && !replacement.token.is_cancelled_now()), fixture["deferredFinish"]["replacementPreserved"]);
        replacement.finish();
        cancellations.close_lease(old);
        cancellations.close_lease(replacement);
        assert_eq!(cancellations.handle().active_operation_count(), 0);
        cancellations.close();
    }

    #[test]
    fn retained_latest_wins_rebase_rebinds_exact_registered_cancellation_authority() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let mut cancellations = KeyedCancellationFixture::open();
        let old = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(fixture["rebase"]["beforeGeneration"].as_u64().unwrap()),
        };
        let fresh = ToolOperationKey { base_revision: semio_framework_job::RevisionId(2), generation: semio_framework_job::Generation(fixture["rebase"]["afterGeneration"].as_u64().unwrap()), ..old.clone() };
        let mut lease = cancellations.begin_keyed(old.clone());
        let cancellation_state = std::sync::Arc::clone(&cancellations.handle().state);
        let busy = cancellation_state.try_lock().unwrap();
        assert!(!lease.rebind_keyed(fresh.base_revision, fresh.generation).unwrap());
        drop(busy);
        drop(cancellation_state);
        assert!(lease.rebind_keyed(fresh.base_revision, fresh.generation).unwrap());
        assert_eq!(*lease.original_key(), fresh);
        assert_eq!(serde_json::json!({ "old": cancellations.handle().cancel(&old).unwrap(), "fresh": cancellations.handle().cancel(&fresh).unwrap() }), serde_json::json!({ "old": fixture["rebase"]["cancelOldKey"], "fresh": fixture["rebase"]["cancelFreshKey"] }));
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        cancellations.close_lease(lease);
        assert_eq!(cancellations.handle().active_operation_count(), 0);
        cancellations.close();
    }

    #[test]
    fn retained_latest_wins_full_registry_reclaims_completed_targets_before_admission() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let mut cancellations = KeyedCancellationFixture::open();
        let mut registry = ToolLatestWinsRegistry::new();
        let count = fixture["reclamation"]["sequentialCompletedTargets"].as_u64().unwrap();
        let mut accepted = 0;
        for index in 0..count {
            let operation = semio_framework_job::allocate_operation_id();
            let mut lease = cancellations
                .begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation, base_revision: semio_framework_job::RevisionId(1), generation: semio_framework_job::Generation(0) });
            let mut key = semio_framework_value_ordered::ordered::SharedOwner::from_cold(format!("target-{index:04}"));
            let mut begun = false;
            let mut result = None;
            for _ in 0..100_000 {
                if !begun {
                    begun = begin_registry(&mut registry, operation.0, &key, &lease);
                }
                if begun {
                    result = registry.take_outcome(operation.0);
                }
                if result.is_some() {
                    break;
                }
                advance_registry(&mut registry, "full-map retained admission");
            }
            assert_eq!(result, Some(true), "completed target {index} must not consume permanent admission capacity");
            release_key_alias(&mut key);
            accepted += 1;
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                advance_registry(&mut registry, "full-map retained reclamation");
            }
            assert!(registry.can_begin());
            lease.finish();
            cancellations.close_lease(lease);
        }
        assert_eq!(serde_json::json!(accepted), fixture["reclamation"]["acceptedTargets"]);
        assert_eq!(cancellations.handle().active_operation_count(), 0);
        registry.begin_close();
        for _ in 0..100_000 {
            if registry.terminal_is_empty() {
                break;
            }
            advance_registry(&mut registry, "full-map retained close");
        }
        assert!(registry.terminal_is_empty());
        cancellations.close();
    }
