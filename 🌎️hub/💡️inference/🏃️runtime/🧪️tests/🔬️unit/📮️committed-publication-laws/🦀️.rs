/// 📮️ Law of a committed approval event reaching the actor frontier and the public checkpoint before the ledger applies it.

#[tokio::test]
async fn gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply() {
    let watchdog = RuntimeLawHangWatchdogV1::arm("gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply", std::time::Duration::from_secs(600));
    let (identity, base) = canonical_identity_and_base();
    let undo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("durable undo fixture");
    let undo_contract = &undo_fixture["genesisFirstUndo"];
    let genesis_snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).expect("exact initial Map snapshot");
    let owner = reader(&identity);
    let ledger = ledger();
    let accepted = ledger.accept(&identity, &base.pack, 1_000).expect("accepted job");
    let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
    let (proposal, command) = canonical_approval(&identity, &accepted.job_id, &base, 1_004);
    let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
    assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
    let proposal_hash = sha256(proposal.as_slice());
    ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");

    let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
    let backend = Arc::new(db::storage::DbBackend::Memory(memory));
    let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    assert_eq!(undo_contract["initialShape"], "scope-bound-zero-frontier");
    assert!(base.frontier.is_genesis_for(&scope), "the first approval begins at the exact scope-bound zero frontier");
    assert_eq!(undo_contract["syntheticGenesisEdit"], false, "the fixture never invents a bootstrap edit");
    let document = protocol::ArtifactId(document_key(&scope));
    let handle = database.ensure_document(&document).await.expect("document actor");
    let initial = handle.checkpoint_publication_snapshot().await.expect("initial actor frontier");
    assert_eq!((initial.authority_generation, initial.frontier.head_seq, initial.frontier.commit_seq, initial.head_edit_id), (0, 0, 0, None));
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let publisher_entered = Arc::new(tokio::sync::Notify::new());
    let publisher_release = Arc::new(tokio::sync::Notify::new());
    let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
        ledger: ledger.clone(),
        identity: identity.clone(),
        job_id: accepted.job_id.clone(),
        attempts: std::sync::atomic::AtomicUsize::new(0),
        order: order.clone(),
        pause: Some((publisher_entered.clone(), publisher_release.clone())),
    });
    let backend_witness = backend.clone();
    let committer = Arc::new(GisRetainedCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
    let committer_port: Arc<dyn GisMapApprovalCommitterV1> = committer.clone();
    let gate = Arc::new(tokio::sync::Mutex::new(()));
    let composed_children = base.composed_children(&GisMapInferenceBindingV1).expect("canonical fixed-three children");
    let expected_mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
    let expected_command_hash = sha256(command.as_slice());
    let approval_actor = approval_actor(&identity);
    let pure_ingress = approval_ingress(&identity);
    let pure_request = GisMapApprovalCommitRequestV1 {
        composed_children: &composed_children,
        scope: &scope,
        actor: &approval_actor,
        mutation_id: &expected_mutation_id,
        command_hash: &expected_command_hash,
        job_id: &accepted.job_id,
        proposal_hash: &proposal_hash,
        command: command.as_slice(),
        base: &base,
        base_frontier: &base.frontier,
        deadline_ms: 60_000,
        now_ms: 1_004,
        document_write: gate.clone(),
        ingress: pure_ingress,
    };
    assert!(committer.validate_prepared_request(&pure_request).is_ok(), "the exact ledger/base tuple passes immutable admission before any owner reservation");
    assert!(GisRetainedCommitterV1::preflight(&pure_request).is_ok(), "the exact fixed-three command passes pure GIS preflight before Store assembly");
    let wrong_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> =
        Arc::new(TestApprovalIngressAuthorityV1 { scope: scope.clone(), user_id: identity.user_id.clone(), session_id: identity.session_id.clone(), authorization_generation: identity.authorization_generation + 1, released: None });
    assert!(
        matches!(commit_prepared_approval(&committer_port, &GisMapInferenceBindingV1, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), wrong_ingress).await, Err(InferenceRouteErrorV1::Denied)),
        "a substituted Hub ingress generation is rejected before document-write acquisition",
    );
    assert!(gate.try_lock().is_ok(), "rejected ingress never acquires the document writer");
    let first_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let first_ingress = tracked_approval_ingress(&identity, Some(first_ingress_released.clone()));
    let first = commit_prepared_approval(&committer_port, &GisMapInferenceBindingV1, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), first_ingress).await;
    assert!(
        matches!(first, Err(InferenceRouteErrorV1::Storage)),
        "public checkpoint refusal remains a retained nonterminal publication, got {:?} refused as {:?}; {:?}",
        first.as_ref().err(),
        last_gis_map_commit_conflict_arm(),
        last_publisher_region_evidence(),
    );
    assert!(first_ingress_released.load(Ordering::Acquire), "a failed request releases its live Hub ingress authority after the durable decision is retained");
    assert!(gate.try_lock().is_err(), "the exact document write authority remains held after publication refusal");
    assert_eq!(ledger.read(&accepted.job_id, &owner, 1_005).expect("retained proposal").proposal_state, semio_hub_inference::schema::InferenceProposalStateV1::Offered);
    let actor = handle.checkpoint_publication_snapshot().await.expect("committed Event actor frontier");
    assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq), (1, 1));
    let expected_mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
    assert_eq!(actor.head_edit_id.as_ref().map(|value| value.0.as_str()), Some(expected_mutation_id.as_str()));
    let (parent_revision, parent_reference, drawing_reference, drawing_owner, value_reference, value_owner) = {
        use directory::os_store::SpaceMember;
        let documents = committer.documents.lock().await;
        match documents.get(&document_key(&scope)) {
            Some(RetainedGisMapDocumentStateV1::Publishing { owners, .. }) => {
                let parent = owners.parent.as_ref().expect("retained parent Store");
                let drawing = owners.drawing.as_ref().expect("retained drawing Store");
                let value = owners.value.as_ref().expect("retained value Store");
                (
                    parent.content_revision_now(),
                    parent.artifact_ref().expect("parent Store identity"),
                    drawing.artifact_ref().expect("drawing Store identity"),
                    drawing.owner_ref().expect("drawing Store owner"),
                    value.artifact_ref().expect("value Store identity"),
                    value.owner_ref().expect("value Store owner"),
                )
            }
            _ => panic!("publisher refusal retains the verified publication owner"),
        }
    };
    let expected_parent = semio_framework_artifact_reference::ArtifactRef { artifact_id: document_key(&scope), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } };
    assert_eq!(parent_reference, expected_parent);
    assert_eq!(drawing_reference.to_uri(), "gismap-drawing!s.stdio.semio@v1/drawing");
    assert_eq!(drawing_owner, directory::os_store::OwnerRef { parent: expected_parent.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() });
    assert_eq!(value_reference.to_uri(), "gismap-value!s.stdio.semio@v1/value");
    assert_eq!(value_owner, directory::os_store::OwnerRef { parent: expected_parent, slot: "value".into(), child_id: "gismap-value".into() });
    assert_ne!(actor.frontier.chain_hash, initial.frontier.chain_hash, "a committed fixed-three decision advances the actor frontier away from its genesis chain hash");
    assert_eq!(actor.frontier.chain_hash, parent_revision, "a committed fixed-three decision anchors the actor frontier chain hash to the parent Store's post content revision");

    let retry_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let retry_ingress = tracked_approval_ingress(&identity, Some(retry_ingress_released.clone()));
    let mut retry = committer.commit(GisMapApprovalCommitRequestV1 {
        composed_children: &composed_children,
        scope: &scope,
        actor: &approval_actor,
        mutation_id: &expected_mutation_id,
        command_hash: &expected_command_hash,
        job_id: &accepted.job_id,
        proposal_hash: &proposal_hash,
        command: command.as_slice(),
        base: &base,
        base_frontier: &base.frontier,
        deadline_ms: 60_000,
        now_ms: 1_006,
        document_write: gate.clone(),
        ingress: retry_ingress,
    });
    watchdog.at("the sole retained driver reaches the paused publisher");
    tokio::select! {
        _ = publisher_entered.notified() => {}
        _ = &mut retry => panic!("fresh retry completed before the retained publisher pause"),
    }
    let mut close = Box::pin(committer.close());
    let close_wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
    let close_waker = std::task::Waker::from(close_wake);
    let mut close_context = std::task::Context::from_waker(&close_waker);
    assert!(matches!(close.as_mut().poll(&mut close_context), std::task::Poll::Pending), "close joins the active post-witness driver instead of starting another verifier or publisher");
    assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt"]);
    drop(close);
    publisher_release.notify_one();
    watchdog.at("fresh retry observes retained publication");
    let terminal = (&mut retry)
        .await
        .unwrap_or_else(|error| panic!("fresh request joins the one publication: {error:?} refused as {:?}, identity mismatch {:?}, prepare conflict {:?}", last_gis_map_commit_conflict_arm(), last_gis_map_identity_mismatch(), last_gis_map_prepare_conflict()));
    assert!(terminal.applied, "a successful committed-witness reconciliation reports the durable approval as applied");
    assert_eq!(terminal.document_generation, 0, "the committed witness retains the live initial actor generation");
    assert_eq!(terminal.frontier.head_edit_ordinal, base.frontier.head_edit_ordinal + undo_contract["firstCommandOrdinalDelta"].as_u64().expect("first command delta"), "the first real command is the first history edit",);
    drop(retry);
    let approved = ledger.read(&accepted.job_id, &owner, 1_007).expect("approved proposal");
    assert_eq!(approved.proposal_state, semio_hub_inference::schema::InferenceProposalStateV1::Approved);
    assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"],);
    assert!(gate.try_lock().is_ok(), "the document write authority releases only after public ACK and ledger apply");
    assert!(retry_ingress_released.load(Ordering::Acquire), "the fresh Hub ingress authority releases after public ACK and ledger apply");

    let after_pack = {
        use directory::os_store::SpaceMember as _;
        let documents = committer.documents.lock().await;
        let owners = match documents.get(&document_key(&scope)) {
            Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners,
            _ => panic!("the approved publication retains its exact three Store owners"),
        };
        ::semio_framework_async::poll::resolve_ready(owners.parent.as_ref().expect("retained parent Store").snapshot_pack()).expect("approved Map snapshot").pack
    };
    let after_base = InferenceMapBaseV1 {
        frontier: ArtifactFrontier {
            document_id: terminal.frontier.document_id.clone(),
            head_edit_ordinal: terminal.frontier.head_edit_ordinal,
            head_edit_id: terminal.frontier.head_edit_id.clone(),
            last_commit_seq: terminal.frontier.last_commit_seq,
            chain_hash: directory::os_directory::io::binary::artifact_hash::parse_artifact_hash_hex(&terminal.frontier.chain_sha256).expect("approved chain hash"),
        },
        descriptor_digest: identity.descriptor_digest.clone(),
        pack: InferencePrivateBytesV1::new(after_pack, INPUT_MAX_BYTES).expect("bounded approved Map pack"),
    };
    let target = ledger.gis_map_approval_undo_target(&terminal.undo.target_id, &owner).expect("owner-retained durable undo target");
    assert_eq!(target.after_frontier, terminal.frontier);
    assert_eq!(target.after_base_digest, after_base.digest());
    let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).expect("retained original command");
    let inverses = semio_framework_pack_json::from_json_str::<Vec<semio_s_artifact_gis_gismap::mutations::GisMapMutation>>(std::str::from_utf8(original.inverse_payload()).expect("canonical inverse text"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical inverse mutations");
    assert_eq!(inverses.len(), 1, "the retained approval owns one exact parent inverse");
    let current = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(after_base.pack.as_slice()).expect("approved Map snapshot");
    let mut before = current.clone();
    semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation(&mut before, &inverses[0]).expect("server inverse applies to the exact current Map");
    use directory::Inference as _;
    let work = semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).expect("server reconstructs the original fixed-three work");
    let undo_diff = semio_framework_pack_json::to_json_string(&inverses[0]).into_bytes();
    let undo_inverse = semio_framework_pack_json::to_json_string(&vec![work.parent]).into_bytes();
    let undo_idempotency_key = "55".repeat(16);
    let undo_operation_id = sha256(format!("semio.hub.gis-map-approval-undo-operation/v1\0{}\0{}", target.target_id, undo_idempotency_key).as_bytes())[..32].to_owned();
    let undo_proposal_hash = sha256(&undo_diff);
    let undo_mutation_id = approval_mutation_id(&undo_operation_id, &undo_proposal_hash);
    let undo_command = InferencePrivateBytesV1::new(
        encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &undo_mutation_id,
            document_id: &document_key(&scope),
            actor: &approval_actor,
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &undo_diff,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &undo_inverse,
            timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 1_010, logical: 0 },
        })
        .expect("server-stamped undo command"),
        semio_hub_inference::command::COMMAND_MAX_BYTES,
    )
    .expect("bounded undo command");
    let undo_command_hash = sha256(undo_command.as_slice());
    assert!(matches!(
        ledger.prepare_gis_map_approval_undo(&target, &undo_idempotency_key, &undo_operation_id, &undo_proposal_hash, &undo_mutation_id, &undo_command_hash, &undo_command,),
        Ok(semio_hub_inference_ledger::GisMapApprovalUndoAdmissionV1::Prepared)
    ));
    let undo_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let undo_ingress = tracked_approval_ingress(&identity, Some(undo_ingress_released.clone()));
    let undo_receipt = committer_port
        .undo(GisMapApprovalUndoCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &approval_actor,
            target: &target,
            idempotency_key: &undo_idempotency_key,
            mutation_id: &undo_mutation_id,
            command_hash: &undo_command_hash,
            operation_id: &undo_operation_id,
            proposal_hash: &undo_proposal_hash,
            command: undo_command.as_slice(),
            base: &after_base,
            deadline_ms: 60_000,
            now_ms: 1_010,
            document_write: gate.clone(),
            ingress: undo_ingress,
        })
        .await
        .unwrap_or_else(|error| {
            panic!(
                "retained durable undo: {error:?}, prepare conflict {:?}, conflict arm {:?}, identity mismatch {:?}",
                last_gis_map_prepare_conflict(),
                last_gis_map_commit_conflict_arm(),
                last_gis_map_identity_mismatch(),
            )
        });
    assert!(undo_receipt.applied, "the first exact undo applies one second durable decision");
    assert_eq!(undo_receipt.frontier.head_edit_id, undo_mutation_id);
    assert_eq!(undo_receipt.frontier.head_edit_ordinal, terminal.frontier.head_edit_ordinal + undo_contract["undoCommandOrdinalDelta"].as_u64().expect("undo command delta"),);
    assert_eq!(undo_receipt.frontier.last_commit_seq, terminal.frontier.last_commit_seq + 1);
    assert!(!undo_contract["restoresGenesisFrontier"].as_bool().expect("frontier contract"));
    assert!(undo_receipt.frontier.head_edit_ordinal > 0 && undo_receipt.frontier.last_commit_seq > 0, "undo restores content through a real second command, never by resetting lineage to genesis");
    assert!(undo_ingress_released.load(Ordering::Acquire), "durable undo releases its retained Hub ingress only after publication ACK");
    let reverted_pair = {
        use directory::os_store::SpaceMember as _;
        let documents = committer.documents.lock().await;
        let owners = match documents.get(&document_key(&scope)) {
            Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners,
            _ => panic!("the undo publication retains its exact three Store owners"),
        };
        ::semio_framework_async::poll::resolve_ready(owners.parent.as_ref().expect("retained reverted parent Store").snapshot_pack()).expect("reverted Map snapshot")
    };
    let reverted = published_pair_snapshot(&reverted_pair.pack, &reverted_pair.spr).await.expect("reverted Map pair folds to its exact head state");
    assert!(undo_contract["restoresExactInitialSnapshot"].as_bool().expect("snapshot contract"));
    assert_eq!(reverted, genesis_snapshot, "the second durable publication restores the exact package-owned initial Map snapshot");
    assert_eq!(
        order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(),
        ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"]
    );
    let replay = ledger.replayed_gis_map_approval_undo(&target.target_id, &undo_idempotency_key, &owner).expect("exact undo replay lookup").expect("committed undo replay receipt");
    assert!(replay.applied && replay.replayed);
    assert_eq!((&replay.mutation_id, &replay.command_hash, &replay.frontier), (&undo_mutation_id, &undo_command_hash, &undo_receipt.frontier));
    assert!(matches!(
        ledger.prepare_gis_map_approval_undo(
            &target,
            &undo_idempotency_key,
            &undo_operation_id,
            &undo_proposal_hash,
            &replay.mutation_id,
            &replay.command_hash,
            &undo_command,
        ),
        Ok(semio_hub_inference_ledger::GisMapApprovalUndoAdmissionV1::Replayed(receipt)) if receipt.frontier == undo_receipt.frontier
    ));

    committer.close().await.unwrap_or_else(|error| panic!("committer close: {error:?}, last close-store step {:?}", last_close_store_step()));
    assert_eq!(Arc::strong_count(&committer.documents), 1, "close joins every maintenance task after its full retained committer owner is physically dropped");
    assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert_eq!(
        ledger.reconcile_committed_approval(&accepted.job_id, &terminal.witness, terminal.document_generation, &terminal.frontier, &identity.descriptor_digest, &"11".repeat(32), 1_008,).map(|value| value.applied),
        Err(InferenceErrorV1::Conflict),
        "an invalidated initial-generation witness cannot reconcile again",
    );
    let approved_events = ledger.events(&accepted.job_id, &owner, 0, 1_009).expect("terminal owner events").events.into_iter().filter(|event| event.kind == "approved").count();
    assert_eq!(approved_events, 1, "initial-generation reconciliation emits one approved event");
    drop(committer_port);
    drop(committer);
    drop(handle);
    let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
    database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
    drop(database);
    let db::storage::DbBackend::Memory(memory) = backend_witness.as_ref() else { panic!("the law mounts the exact memory DB I/O backend") };
    memory.close().await.expect("terminal memory DB I/O backend close");
    drop(backend_witness);
    pool.shutdown().expect("worker pool shutdown");
}
