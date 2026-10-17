/// 🤝️ Laws of abandoned approval handoff and cleanup, proposal ownership and the fail-closed composition transaction.

async fn wait_for_abandoned_approval_handoff(committer: &GisRetainedCommitterV1, key: &str, gate: &Arc<tokio::sync::Mutex<()>>, ingress_released: &std::sync::atomic::AtomicBool) {
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let ready = {
                let documents = committer.documents.lock().await;
                matches!(documents.get(key), Some(RetainedGisMapDocumentStateV1::Ready { pending: None, document_write: None, .. }))
            };
            let maintenance_idle = !committer.maintenance_owns_document(key);
            if ready && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap_or_else(|_| {
        let state = committer.documents.try_lock().map_or_else(
            |_| String::from("document map held across an await"),
            |documents| match documents.get(key) {
                None => String::from("absent"),
                Some(RetainedGisMapDocumentStateV1::Recovery { .. }) => String::from("Recovery"),
                Some(RetainedGisMapDocumentStateV1::Recovered { .. }) => String::from("Recovered"),
                Some(RetainedGisMapDocumentStateV1::Ready { pending, document_write, .. }) => format!("Ready pending={} document_write={}", pending.is_some(), document_write.is_some()),
                Some(RetainedGisMapDocumentStateV1::Assembly { owner, .. }) => format!("Assembly phase={:?} [{}]", owner.phase(), owner.closing_witness()),
                Some(RetainedGisMapDocumentStateV1::Journal { receipt, .. }) => format!("Journal receipt={}", receipt.is_some()),
                Some(RetainedGisMapDocumentStateV1::Verification { .. }) => String::from("Verification"),
                Some(RetainedGisMapDocumentStateV1::Publishing { .. }) => String::from("Publishing"),
                Some(RetainedGisMapDocumentStateV1::Published { .. }) => String::from("Published"),
                Some(RetainedGisMapDocumentStateV1::Closing(_)) => String::from("Closing"),
            },
        );
        panic!(
            "abandoned approval returns every Store, ingress and document writer: state {state}, last abandoned turn {:?}, maintenance_owns={}, gate_free={}, ingress_released={}",
            last_abandoned_assembly_turn(),
            committer.maintenance_owns_document(key),
            gate.try_lock().is_ok(),
            ingress_released.load(Ordering::Acquire),
        );
    });
}

async fn wait_for_abandoned_prepared_handoff(
    committer: &GisRetainedCommitterV1,
    ledger: &InferenceJobLedgerV1,
    identity: &Identity,
    job_id: &str,
    key: &str,
    gate: &Arc<tokio::sync::Mutex<()>>,
    ingress_released: &std::sync::atomic::AtomicBool,
) {
    let maintenance_key = format!("prepared:{job_id}");
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let document_absent = !committer.documents.lock().await.contains_key(key);
            let maintenance_idle = !committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains(&maintenance_key);
            let outbox_abandoned = ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).is_ok_and(|page| page.rows.is_empty());
            if document_absent && maintenance_idle && outbox_abandoned && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("unpolled or rejected preflight returns its exact ingress and prepared outbox");
}

async fn resume_parked_cleanup(committer: &GisRetainedCommitterV1) {
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            committer.resume_parked_requests();
            committer.resume_parked_prepared();
            let maintenance_is_empty = committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
            let cleanup_is_empty = committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
            if maintenance_is_empty && cleanup_is_empty {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("fresh runtime drains every parked cleanup owner");
}

#[test]
fn gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation() {
    let fixture = fixture();
    let identity = identity();
    let ledger = ledger();
    let owner = reader(&identity);
    let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
    assert_eq!(ledger.accept(&identity, &input(&identity), 1_001).expect("scoped idempotency"), receipt);
    let claim = ledger.start(&receipt.job_id, &identity, 1_002).expect("claim").expect("first owned epoch");
    assert_eq!(claim.run_epoch, 1);
    assert_eq!(ledger.start(&receipt.job_id, &identity, 1_003).expect("second claim"), None, "a live lease is never stolen");
    for step in 1..=4_u64 {
        let now_ms = if step == 4 { 30_000 } else { 1_003 + step };
        assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, step, WORK_UNIT_LIMIT, now_ms).expect("progress heartbeat"), step);
    }
    assert_eq!(ledger.start(&receipt.job_id, &identity, 45_000).expect("heartbeat-protected claim"), None, "a fresh checkpoint lease cannot be stolen after its original lease elapsed");
    assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch + 1, 5, WORK_UNIT_LIMIT, 45_001), Err(InferenceErrorV1::Conflict), "a foreign epoch cannot append progress");
    assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, 3, WORK_UNIT_LIMIT, 45_002), Err(InferenceErrorV1::Conflict), "progress never regresses");
    let page = ledger.events(&receipt.job_id, &owner, 0, 45_003).expect("owner page");
    assert_eq!(page.progress.iter().map(|row| row.cursor).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    assert_eq!(page.next_cursor, 4);
    assert!(page.progress.len() <= semio_hub_inference::schema::EVENT_PAGE_MAX_ITEMS && page.events.len() <= semio_hub_inference::schema::EVENT_PAGE_MAX_ITEMS);
    assert_eq!(ledger.events(&receipt.job_id, &owner, 4, 45_004).expect("tail page").progress.len(), 0);
    assert_eq!(ledger.events(&receipt.job_id, &owner, PROGRESS_MAX_CURSOR + 1, 45_004).err(), Some(InferenceErrorV1::Bounds));
    assert!(ledger.request_cancel(&receipt.job_id, &owner, 45_005).expect("durable cancel request"));
    let cancelled = ledger.events(&receipt.job_id, &owner, 0, 45_006).expect("cancelled page");
    assert!(cancelled.cancel_requested);
    assert_eq!(
        cancelled.events.iter().map(|row| (row.ordinal, row.kind.as_str())).collect::<Vec<_>>(),
        fixture["cancelLifecycle"].as_array().expect("cancel trace").iter().map(|row| (row["ordinal"].as_u64().expect("ordinal"), row["kind"].as_str().expect("kind"))).collect::<Vec<_>>()
    );
    let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
    let proposal = InferencePrivateBytesV1::new(b"bounded-proposal".to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
    assert_eq!(ledger.succeed(&receipt.job_id, &identity, claim.run_epoch, &result, &proposal, 45_007), Ok(false), "a retired job never publishes a late offer");
    assert!(ledger.request_cancel(&receipt.job_id, &owner, 45_008).is_ok(), "cancellation is idempotent");
}

#[test]
fn gis_map_proposal_is_private_to_its_original_author_owner() {
    let identity = identity();
    let ledger = ledger();
    let owner = reader(&identity);
    let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
    ledger.start(&receipt.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
    let peer_user = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string();
    let foreign = [
        ("peer-author-same-space", InferenceReaderV1 { user_id: peer_user.as_str(), ..reader(&identity) }),
        ("cross-space-author", InferenceReaderV1 { space_id: peer_user.as_str(), ..reader(&identity) }),
        ("wrong-document", InferenceReaderV1 { document_id: peer_user.as_str(), ..reader(&identity) }),
        ("stale-session", InferenceReaderV1 { session_id: peer_user.as_str(), ..reader(&identity) }),
        ("stale-authorization-generation", InferenceReaderV1 { authorization_generation: identity.authorization_generation + 1, ..reader(&identity) }),
    ];
    for (role, candidate) in &foreign {
        assert_eq!(ledger.identity_of(&receipt.job_id, candidate).err(), Some(InferenceErrorV1::Denied), "{role} read the frozen identity");
        assert_eq!(ledger.events(&receipt.job_id, candidate, 0, 1_002).err(), Some(InferenceErrorV1::Denied), "{role} read the private stream");
        assert_eq!(ledger.read(&receipt.job_id, candidate, 1_002).err(), Some(InferenceErrorV1::Denied), "{role} read the private proposal");
        assert_eq!(ledger.request_cancel(&receipt.job_id, candidate, 1_002), Err(InferenceErrorV1::Denied), "{role} cancelled another owner's job");
        assert_eq!(ledger.progress(&receipt.job_id, candidate, 1, 1, WORK_UNIT_LIMIT, 1_002), Err(InferenceErrorV1::Denied), "{role} appended progress");
    }
    assert!(ledger.events(&receipt.job_id, &owner, 0, 1_003).is_ok(), "the original owner still reads its own stream");
}

#[tokio::test]
async fn gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies() {
    let fixture = fixture();
    let identity = identity();
    let ledger = ledger();
    let owner = reader(&identity);
    let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
    let claim = ledger.start(&receipt.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
    let ledger_corpus = ledger_fixture();
    let outbox = &ledger_corpus["outbox"];
    let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().expect("literal proposal").as_bytes().to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
    let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
    assert!(ledger.succeed(&receipt.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
    let hex = outbox["commandHex"].as_str().expect("literal command");
    let command = InferencePrivateBytesV1::new((0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect(), semio_hub_inference::command::COMMAND_MAX_BYTES).expect("bounded command");
    let proposal_hash = sha256(proposal.as_slice());
    let prepared = ledger.prepare_approval(&receipt.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
    assert_eq!(prepared.mutation_id, approval_mutation_id(&receipt.job_id, &proposal_hash));
    assert_eq!(prepared.mutation_id, outbox["mutationId"].as_str().expect("literal mutation"));
    let again = ledger.prepare_approval(&receipt.job_id, &identity, &proposal_hash, &command, 1_004).expect("duplicate approval");
    assert_eq!((again.mutation_id, again.command_hash, again.prepared_at_ms), (prepared.mutation_id.clone(), prepared.command_hash.clone(), prepared.prepared_at_ms), "a duplicate approval reconciles to exactly one prepared envelope");
    let base = canonical_map_base(&identity);
    let committer: Arc<dyn GisMapApprovalCommitterV1> = Arc::new(UnavailableGisMapApprovalCommitterV1);
    for attempt in 0..2 {
        assert!(
            matches!(
                commit_prepared_approval(&committer, &GisMapInferenceBindingV1, &identity, &receipt.job_id, &proposal_hash, &command, &base, 60_000, 1_005 + attempt, Arc::new(tokio::sync::Mutex::new(())), approval_ingress(&identity),).await,
                Err(InferenceRouteErrorV1::CommitUnavailable)
            ),
            "no composition transaction is registered, so approval must fail closed"
        );
    }
    assert_eq!(InferenceRouteErrorV1::CommitUnavailable.code(), "approval.commit-unavailable");
    assert_eq!(InferenceRouteErrorV1::CommitUnavailable.status(), 503);
    let view = ledger.read(&receipt.job_id, &owner, 1_007).expect("owner view");
    assert_eq!(serde_json::to_value(view.proposal_state).expect("proposal state"), "offered", "a refused publication never marks the proposal approved");
    let page = ledger.events(&receipt.job_id, &owner, 0, 1_008).expect("owner page");
    assert_eq!(
        page.events.iter().map(|row| (row.ordinal, row.kind.as_str())).collect::<Vec<_>>(),
        fixture["lifecycle"].as_array().expect("lifecycle").iter().take(4).map(|row| (row["ordinal"].as_u64().expect("ordinal"), row["kind"].as_str().expect("kind"))).collect::<Vec<_>>()
    );
    assert!(!page.events.iter().any(|row| row.kind == "approved"), "no witness, no approved event");
}
