/// 🔚️ Laws of terminal close fencing new admission and of the frozen comparison limits and error vocabulary.

#[tokio::test]
async fn gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission() {
    let (identity, base) = canonical_identity_and_base();
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
    let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
        ledger: ledger.clone(),
        identity: identity.clone(),
        job_id: accepted.job_id.clone(),
        attempts: std::sync::atomic::AtomicUsize::new(0),
        order: Arc::new(std::sync::Mutex::new(Vec::new())),
        pause: None,
    });
    let backend_witness = backend.clone();
    let committer = Arc::new(GisRetainedCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
    let gate = Arc::new(tokio::sync::Mutex::new(()));
    let children = base.composed_children(&GisMapInferenceBindingV1).expect("exact composed children");
    let actor = approval_actor(&identity);
    let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
    let command_hash = sha256(command.as_slice());
    let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let unpolled = committer.commit(GisMapApprovalCommitRequestV1 {
        composed_children: &children,
        scope: &scope,
        actor: &actor,
        mutation_id: &mutation_id,
        command_hash: &command_hash,
        job_id: &accepted.job_id,
        proposal_hash: &proposal_hash,
        command: command.as_slice(),
        base: &base,
        base_frontier: &base.frontier,
        deadline_ms: 60_000,
        now_ms: 1_004,
        document_write: gate.clone(),
        ingress: tracked_approval_ingress(&identity, Some(ingress_released.clone())),
    });
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
    let mut close = Box::pin(committer.close());
    let close_wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
    let close_waker = std::task::Waker::from(close_wake);
    let mut close_context = std::task::Context::from_waker(&close_waker);
    assert!(matches!(close.as_mut().poll(&mut close_context), std::task::Poll::Pending), "terminal close waits for the validated unpolled cleanup owner");
    let refused_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(
        matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_005,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(refused_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Unavailable)
        ),
        "terminal close atomically fences later admission",
    );
    assert!(refused_ingress_released.load(Ordering::Acquire));
    drop(unpolled);
    tokio::time::timeout(std::time::Duration::from_secs(5), &mut close).await.expect("cleanup decrement wakes terminal close").expect("terminal close");
    drop(close);
    assert!(ingress_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("terminal outbox query").rows.is_empty());
    assert!(gate.try_lock().is_ok());
    drop(committer);
    let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
    database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
    drop(database);
    assert_eq!(Arc::strong_count(&backend_witness), 1, "every retained owner of the I/O backend is released before the pool closes — a surviving clone is the pool use `shutdown` reports as busy");
    let db::storage::DbBackend::Memory(memory) = backend_witness.as_ref() else { panic!("the law mounts the exact memory DB I/O backend") };
    memory.close().await.expect("terminal memory DB I/O backend close");
    drop(backend_witness);
    pool.shutdown().expect("worker pool shutdown");
}

#[test]
fn gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary() {
    let fixture = fixture();
    let preview = gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"].as_str().expect("sample job"), fixture["proposalHash"].as_str().expect("proposal hash"), fixture["proposalCanonical"].as_str().expect("canonical proposal").as_bytes())
        .expect("typed owner preview");
    assert_eq!(serde_json::to_value(preview).expect("preview wire"), fixture["preview"]);
    assert_eq!(gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"].as_str().expect("sample job"), &"0".repeat(64), fixture["proposalCanonical"].as_str().expect("proposal").as_bytes()).err(), Some(InferenceRouteErrorV1::Conflict));
    let substituted = b"{\"DeleteRegion\":{\"id\":\"inference-11111111111111111111111111111111\"}}";
    assert_eq!(gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"].as_str().expect("sample job"), &sha256(substituted), substituted).err(), Some(InferenceRouteErrorV1::Conflict));
    let proposal = || serde_json::from_str::<serde_json::Value>(fixture["proposalCanonical"].as_str().expect("proposal")).expect("proposal value");
    let rejected = [
        ("wrong-region-id", serde_json::json!("substituted"), "/CreateRegion/item/id", InferenceRouteErrorV1::Conflict),
        ("wrong-kind", serde_json::json!("route"), "/CreateRegion/item/data/kind", InferenceRouteErrorV1::Conflict),
        ("short-ring", serde_json::json!([[7, 46], [9, 46], [9, 48], [7, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Bounds),
        ("reordered-ring", serde_json::json!([[7, 46], [7, 48], [9, 48], [9, 46], [7, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Conflict),
        ("out-of-range-ring", serde_json::json!([[-181, 46], [9, 46], [9, 48], [-181, 48], [-181, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Conflict),
    ];
    for (name, value, pointer, expected) in rejected {
        let mut candidate = proposal();
        *candidate.pointer_mut(pointer).unwrap_or_else(|| panic!("{name} pointer")) = value;
        let bytes = serde_json::to_vec(&candidate).expect("candidate bytes");
        assert_eq!(gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"].as_str().expect("sample job"), &sha256(&bytes), &bytes).err(), Some(expected), "{name}");
    }
    for (name, bytes) in [
        ("malformed", b"{".as_slice()),
        (
            "non-finite",
            br#"{"CreateRegion":{"index":0,"item":{"id":"inference-11111111111111111111111111111111","data":{"id":"inference-11111111111111111111111111111111","kind":"inference-bounds","ring":[[1e999,46],[9,46],[9,48],[1e999,48],[1e999,46]]}}}}"#
                .as_slice(),
        ),
    ] {
        assert_eq!(gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"].as_str().expect("sample job"), &sha256(bytes), bytes).err(), Some(InferenceRouteErrorV1::Invalid), "{name}");
    }
    let identity = identity();
    let frozen: semio_hub_inference::schema::InferenceBindingIdentityV1 = serde_json::from_value(fixture["binding"].clone()).expect("frozen binding identity");
    assert_eq!(frozen, identity.binding);
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    let base = InferenceMapBaseV1 {
        frontier: directory::os_directory::ArtifactFrontier {
            document_id: identity.document_id.clone(),
            head_edit_ordinal: identity.head_ordinal,
            head_edit_id: identity.head_edit_id.clone(),
            last_commit_seq: identity.last_commit_seq,
            chain_hash: directory::os_directory::ArtifactHash([0; 32]),
        },
        descriptor_digest: identity.descriptor_digest.clone(),
        pack: input(&identity),
    };
    assert_eq!(compare_frozen_identity(&frozen, &identity, &scope, &base), Ok(()));
    let drift: Vec<(&str, Box<dyn Fn(&mut Identity)>)> = vec![
        ("changed-frontier", Box::new(|value: &mut Identity| value.last_commit_seq += 1)),
        ("changed-base-pack", Box::new(|value: &mut Identity| value.input_hash = "9".repeat(64))),
        ("changed-binding-digest", Box::new(|value: &mut Identity| value.binding.digest = "9".repeat(64))),
        ("changed-catalog-generation", Box::new(|value: &mut Identity| value.binding.catalog_generation_id = "9".repeat(64))),
        ("changed-parent-dialect", Box::new(|value: &mut Identity| value.binding.parent_dialect.subset = "lite".into())),
        ("changed-surface", Box::new(|value: &mut Identity| value.binding.surface_id = "s.gis.gismap@1/*#viewer".into())),
        ("changed-granted-mode", Box::new(|value: &mut Identity| value.binding.granted_mode = "read-observe".into())),
    ];
    for (name, mutate) in &drift {
        let mut candidate = identity.clone();
        mutate(&mut candidate);
        assert_eq!(compare_frozen_identity(&frozen, &candidate, &scope, &base), Err(InferenceRouteErrorV1::Conflict), "{name} was admitted against the frozen binding");
        assert!(fixture["approvalRejections"].as_array().expect("rejections").iter().any(|row| row["name"] == *name && row["code"] == "inference.conflict"), "{name} is not pinned by the neutral corpus");
    }
    let mut cross_space = identity.clone();
    cross_space.space_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".into();
    assert!(compare_frozen_identity(&frozen, &cross_space, &scope, &base).is_err());
    let limits = &fixture["limits"];
    assert_eq!(limits["requestMaxBytes"], semio_hub_inference::schema::REQUEST_MAX_BYTES as u64);
    assert_eq!(limits["inputMaxBytes"], INPUT_MAX_BYTES as u64);
    let default_map = {
        use pack::snapshot::ArtifactPack as _;
        semio_s_artifact_gis_gismap::schema::default_document().encode_pack()
    };
    assert!(InferencePrivateBytesV1::new(default_map.clone(), INPUT_MAX_BYTES).is_ok(), "the map the hub creates by default ({} B) must be inferable within inputMaxBytes", default_map.len());
    assert_eq!(limits["resultMaxBytes"], RESULT_MAX_BYTES as u64);
    assert_eq!(limits["proposalMaxBytes"], PROPOSAL_MAX_BYTES as u64);
    assert_eq!(limits["commandMaxBytes"], semio_hub_inference::command::COMMAND_MAX_BYTES as u64);
    assert_eq!(limits["identityJsonMaxBytes"], semio_hub_inference::schema::IDENTITY_JSON_MAX_BYTES as u64);
    assert_eq!(limits["jobCapacity"], semio_hub_inference::schema::JOB_CAPACITY as u64);
    assert_eq!(limits["operationCapacity"], OPERATION_CAPACITY as u64);
    assert_eq!(limits["documentGateCapacity"], DOCUMENT_GATE_CAPACITY as u64);
    assert_eq!(limits["progressMaxCursor"], PROGRESS_MAX_CURSOR);
    assert_eq!(limits["eventPageMaxItems"], semio_hub_inference::schema::EVENT_PAGE_MAX_ITEMS as u64);
    assert_eq!(limits["claimLeaseMaxMs"], semio_hub_inference::schema::CLAIM_LEASE_MAX_MS);
    assert_eq!(limits["jobMaxLifetimeMs"], semio_hub_inference::schema::JOB_MAX_LIFETIME_MS);
    assert_eq!(limits["workUnitLimit"], WORK_UNIT_LIMIT);
    assert_eq!(limits["recursionDepth"], u64::from(RECURSION_DEPTH));
    assert_eq!(limits["allocationBytes"], ALLOCATION_BYTES);
    assert_eq!(limits["approvalMaxRecords"], APPROVAL_MAX_RECORDS);
    for row in fixture["errors"].as_array().expect("error vocabulary") {
        let code = row["code"].as_str().expect("code");
        let status = row["status"].as_u64().expect("status");
        let published = [
            InferenceRouteErrorV1::Unavailable,
            InferenceRouteErrorV1::Denied,
            InferenceRouteErrorV1::NotFound,
            InferenceRouteErrorV1::Invalid,
            InferenceRouteErrorV1::Bounds,
            InferenceRouteErrorV1::Conflict,
            InferenceRouteErrorV1::Capacity,
            InferenceRouteErrorV1::Expired,
            InferenceRouteErrorV1::Cancelled,
            InferenceRouteErrorV1::CommitUnavailable,
            InferenceRouteErrorV1::Storage,
        ]
        .into_iter()
        .find(|candidate| candidate.code() == code)
        .unwrap_or_else(|| panic!("{code} is not a published inference route code"));
        assert_eq!(u64::from(published.status()), status, "{code}");
    }
    assert_eq!(document_key(&scope), format!("v1:{}:{}:{}{}", identity.space_id.len(), identity.document_id.len(), identity.space_id, identity.document_id));
    assert_eq!(approval_actor(&identity), format!("user:{}#session:{}", identity.user_id, identity.session_id));
}
