
use super::*;
use crate::inference::schema::{INPUT_MAX_BYTES, InferenceIdentityV1 as Identity, PROGRESS_MAX_CURSOR};
use std::future::Future;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
}

fn ledger_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture")
}

fn identity() -> Identity {
    serde_json::from_value(ledger_fixture()["identity"].clone()).expect("accepted identity")
}

fn ledger() -> Arc<InferenceJobLedgerV1> {
    let path = std::env::temp_dir().join(format!("semio-gis-map-proposal-{}.sqlite", directory::os_identity::time_ordered_id()));
    Arc::new(InferenceJobLedgerV1::open(&path).expect("bounded private ledger"))
}

fn input(identity: &Identity) -> InferencePrivateBytesV1 {
    let bytes = ledger_fixture()["input"].as_str().expect("literal base").as_bytes().to_vec();
    assert_eq!(sha256(&bytes), identity.input_hash, "the literal base is exactly the frozen input");
    InferencePrivateBytesV1::new(bytes, INPUT_MAX_BYTES).expect("bounded base")
}

fn canonical_map_pack() -> InferencePrivateBytesV1 {
    use directory::ArtifactPack as _;
    let descriptor = ledger_fixture()["input"].as_str().expect("literal Map descriptor").to_owned();
    let snapshot = semio_s_artifact_gis_gismap::schema::gis_map_document_from_descriptor_json(&descriptor);
    InferencePrivateBytesV1::new(snapshot.encode_pack(), INPUT_MAX_BYTES).expect("bounded canonical Map pack")
}

fn canonical_map_base(identity: &Identity) -> InferenceMapBaseV1 {
    InferenceMapBaseV1 {
        frontier: directory::os_directory::ArtifactFrontier {
            document_id: identity.document_id.clone(),
            head_edit_ordinal: identity.head_ordinal,
            head_edit_id: identity.head_edit_id.clone(),
            last_commit_seq: identity.last_commit_seq,
            chain_hash: directory::os_directory::ArtifactHash([0; 32]),
        },
        descriptor_digest: identity.descriptor_digest.clone(),
        pack: canonical_map_pack(),
    }
}

fn canonical_identity_and_base() -> (Identity, InferenceMapBaseV1) {
    let mut identity = identity();
    let base = canonical_map_base(&identity);
    identity.input_hash = base.digest();
    (identity, base)
}

fn canonical_approval(identity: &Identity, job_id: &str, base: &InferenceMapBaseV1, now_ms: u64) -> (InferencePrivateBytesV1, InferencePrivateBytesV1) {
    let (snapshot, inference) = deterministic_map_inference(base, job_id).expect("real GIS Map inference");
    let work = inference.create_region_group_work(&snapshot, job_id).expect("server-derived fixed-three CreateRegion work");
    let proposal_bytes = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
    let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES).expect("bounded canonical proposal");
    let proposal_hash = sha256(proposal.as_slice());
    let inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    let mutation_id = approval_mutation_id(job_id, &proposal_hash);
    let document_id = document_key(&scope);
    let actor = approval_actor(identity);
    let command = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
        mutation_id: &mutation_id,
        document_id: &document_id,
        actor: &actor,
        diff_schema: GIS_DOCUMENT_SCHEMA,
        diff_payload: proposal.as_slice(),
        inverse_schema: GIS_DOCUMENT_SCHEMA,
        inverse_payload: &inverse,
        timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
    })
    .expect("server-stamped approval command");
    (proposal, InferencePrivateBytesV1::new(command, super::super::command::COMMAND_MAX_BYTES).expect("bounded approval command"))
}

struct TestApprovalIngressAuthorityV1 {
    scope: DocumentScope,
    user_id: String,
    session_id: String,
    authorization_generation: u64,
    released: Option<Arc<std::sync::atomic::AtomicBool>>,
}

impl GisMapApprovalIngressAuthorityV1 for TestApprovalIngressAuthorityV1 {
    fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn session_id(&self) -> &str {
        &self.session_id
    }

    fn authorization_generation(&self) -> u64 {
        self.authorization_generation
    }
}

impl Drop for TestApprovalIngressAuthorityV1 {
    fn drop(&mut self) {
        if let Some(released) = &self.released {
            released.store(true, Ordering::Release);
        }
    }
}

fn approval_ingress(identity: &Identity) -> Arc<dyn GisMapApprovalIngressAuthorityV1> {
    tracked_approval_ingress(identity, None)
}

fn tracked_approval_ingress(identity: &Identity, released: Option<Arc<std::sync::atomic::AtomicBool>>) -> Arc<dyn GisMapApprovalIngressAuthorityV1> {
    Arc::new(TestApprovalIngressAuthorityV1 {
        scope: DocumentScope::new(identity.space_id.clone(), identity.document_id.clone()),
        user_id: identity.user_id.clone(),
        session_id: identity.session_id.clone(),
        authorization_generation: identity.authorization_generation,
        released,
    })
}

struct OrderedApprovalCheckpointPublisherV1 {
    ledger: Arc<InferenceJobLedgerV1>,
    identity: Identity,
    job_id: String,
    attempts: std::sync::atomic::AtomicUsize,
    order: Arc<std::sync::Mutex<Vec<&'static str>>>,
    pause: Option<(Arc<tokio::sync::Notify>, Arc<tokio::sync::Notify>)>,
}

impl GisMapApprovalCheckpointPublisherV1 for OrderedApprovalCheckpointPublisherV1 {
    fn publish<'a>(
        &'a self,
        request: GisMapApprovalCheckpointRequestV1,
        document_write: Arc<GisMapDocumentWriteAuthorityV1>,
        _attempt_lifetime_ms: u64,
        decision_now_ms: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async move {
            let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(&request.pair.pack).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let expected_region = format!("inference-{}", self.job_id);
            let attempt = self.attempts.fetch_add(1, Ordering::AcqRel);
            let has_expected_region = snapshot
                .regions
                .iter()
                .any(|region| region.id == expected_region && region.data.get("id").and_then(|value| value.as_str()) == Some(expected_region.as_str()) && region.data.get("kind").and_then(|value| value.as_str()) == Some("inference-bounds"));
            if has_expected_region != (attempt < 2) {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let view = self.ledger.read(&self.job_id, &reader(&self.identity), decision_now_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let expected_proposal_state = if attempt < 2 { super::super::schema::InferenceProposalStateV1::Offered } else { super::super::schema::InferenceProposalStateV1::Approved };
            if view.proposal_state != expected_proposal_state || document_write.gate.try_lock().is_ok() {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-attempt");
            if attempt == 0 {
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            if attempt == 1 {
                if let Some((entered, release)) = &self.pause {
                    entered.notify_one();
                    release.notified().await;
                }
            }
            let frontier = RetainedGisMapApprovalCommitterV1::publication_frontier(&request.scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            let pack_hash = directory::os_directory::ArtifactHash::parse_hex(&sha256(&request.pair.pack)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            let spr_hash = directory::os_directory::ArtifactHash::parse_hex(&sha256(&request.pair.spr)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-ack");
            Ok(directory::os_directory::PublishedArtifactCheckpoint {
                scope: request.scope,
                checkpoint_id: directory::os_directory::ArtifactHash([0x45; 32]),
                parent_checkpoint_id: None,
                descriptor_digest_v1: directory::os_directory::ArtifactHash::parse_hex(&request.descriptor_digest).ok_or(GisMapApprovalCommitErrorV1::Storage)?,
                baseline_frontier: frontier,
                pack: directory::os_directory::PublishedArtifactBlob { sha256: pack_hash, byte_length: u64::try_from(request.pair.pack.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)? },
                spr: directory::os_directory::PublishedArtifactBlob { sha256: spr_hash, byte_length: u64::try_from(request.pair.spr.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)? },
                aggregate_sha256: directory::os_directory::ArtifactHash([0x46; 32]),
                published_at_ms: decision_now_ms,
            })
        })
    }

    fn checkpoint_applied(&self, checkpoint: &directory::os_directory::PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1> {
        let view = self.ledger.read(&self.job_id, &reader(&self.identity), checkpoint.published_at_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
        if view.proposal_state != super::super::schema::InferenceProposalStateV1::Approved {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("peer-rebootstrap");
        Ok(())
    }
}

enum AbandonedApprovalPhaseV1 {
    Preflight,
    Assembly,
    Journal,
    Committed,
}

struct ApprovalPollWakeV1 {
    ready: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for ApprovalPollWakeV1 {
    fn wake(self: Arc<Self>) {
        self.ready.store(true, Ordering::Release);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.ready.store(true, Ordering::Release);
    }
}

async fn poll_approval_to_phase<F>(future: &mut std::pin::Pin<Box<F>>, committer: &RetainedGisMapApprovalCommitterV1, key: &str, phase: AbandonedApprovalPhaseV1)
where
    F: std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + ?Sized,
{
    let wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
    let waker = std::task::Waker::from(wake.clone());
    for _ in 0..4_096 {
        while !wake.ready.swap(false, Ordering::AcqRel) {
            tokio::task::yield_now().await;
        }
        let pending = {
            let mut context = std::task::Context::from_waker(&waker);
            matches!(future.as_mut().poll(&mut context), std::task::Poll::Pending)
        };
        assert!(pending, "approval must remain retained before its cancellation phase");
        let reached = {
            let documents = committer.documents.lock().await;
            match (documents.get(key), &phase) {
                (Some(RetainedGisMapDocumentStateV1::Ready { pending: Some(_), document_write: Some(_), .. }), AbandonedApprovalPhaseV1::Preflight)
                | (Some(RetainedGisMapDocumentStateV1::Assembly { .. }), AbandonedApprovalPhaseV1::Assembly)
                | (Some(RetainedGisMapDocumentStateV1::Journal { receipt: None, .. }), AbandonedApprovalPhaseV1::Journal)
                | (Some(RetainedGisMapDocumentStateV1::Journal { receipt: Some(_), .. }), AbandonedApprovalPhaseV1::Committed) => true,
                _ => false,
            }
        };
        if reached {
            return;
        }
    }
    panic!("approval did not reach its exact retained cancellation phase");
}

async fn wait_for_abandoned_approval_handoff(committer: &RetainedGisMapApprovalCommitterV1, key: &str, gate: &Arc<tokio::sync::Mutex<()>>, ingress_released: &std::sync::atomic::AtomicBool) {
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
    .expect("abandoned approval returns every Store, ingress and document writer");
}

async fn wait_for_abandoned_prepared_handoff(
    committer: &RetainedGisMapApprovalCommitterV1,
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

async fn resume_parked_cleanup(committer: &RetainedGisMapApprovalCommitterV1) {
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
    assert!(page.progress.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS && page.events.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS);
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
    let command = InferencePrivateBytesV1::new((0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect(), super::super::command::COMMAND_MAX_BYTES).expect("bounded command");
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
                commit_prepared_approval(&committer, &identity, &receipt.job_id, &proposal_hash, &command, &base, 60_000, 1_005 + attempt, Arc::new(tokio::sync::Mutex::new(())), approval_ingress(&identity),).await,
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

#[tokio::test]
async fn gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply() {
    let (identity, base) = canonical_identity_and_base();
    let undo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("durable undo fixture");
    let undo_contract = &undo_fixture["genesisFirstUndo"];
    let genesis_snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).expect("exact initial Map snapshot");
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
    let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
    let committer_port: Arc<dyn GisMapApprovalCommitterV1> = committer.clone();
    let gate = Arc::new(tokio::sync::Mutex::new(()));
    let composed_children = base.composed_children().expect("canonical fixed-three children");
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
    assert!(RetainedGisMapApprovalCommitterV1::preflight(&pure_request).is_ok(), "the exact fixed-three command passes pure GIS preflight before Store assembly");
    let wrong_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> =
        Arc::new(TestApprovalIngressAuthorityV1 { scope: scope.clone(), user_id: identity.user_id.clone(), session_id: identity.session_id.clone(), authorization_generation: identity.authorization_generation + 1, released: None });
    assert!(
        matches!(commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), wrong_ingress).await, Err(InferenceRouteErrorV1::Denied)),
        "a substituted Hub ingress generation is rejected before document-write acquisition",
    );
    assert!(gate.try_lock().is_ok(), "rejected ingress never acquires the document writer");
    let first_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let first_ingress = tracked_approval_ingress(&identity, Some(first_ingress_released.clone()));
    let first = commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), first_ingress).await;
    assert!(matches!(first, Err(InferenceRouteErrorV1::Storage)), "public checkpoint refusal remains a retained nonterminal publication");
    assert!(first_ingress_released.load(Ordering::Acquire), "a failed request releases its live Hub ingress authority after the durable decision is retained");
    assert!(gate.try_lock().is_err(), "the exact document write authority remains held after publication refusal");
    assert_eq!(ledger.read(&accepted.job_id, &owner, 1_005).expect("retained proposal").proposal_state, super::super::schema::InferenceProposalStateV1::Offered);
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
    let expected_parent = directory::os_io::ArtifactRef { artifact_id: document_key(&scope), dialect: directory::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } };
    assert_eq!(parent_reference, expected_parent);
    assert_eq!(drawing_reference.to_uri(), "gismap-drawing!s.stdio.semio@v1/drawing");
    assert_eq!(drawing_owner, directory::os_store::OwnerRef { parent: expected_parent.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() });
    assert_eq!(value_reference.to_uri(), "gismap-value!s.stdio.semio@v1/value");
    assert_eq!(value_owner, directory::os_store::OwnerRef { parent: expected_parent, slot: "value".into(), child_id: "gismap-value".into() });
    assert_ne!(actor.frontier.chain_hash, parent_revision, "the actor WAL chain and Store content revision are intentionally distinct hash domains");

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
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        tokio::select! {
            _ = publisher_entered.notified() => {}
            _ = &mut retry => panic!("fresh retry completed before the retained publisher pause"),
        }
    })
    .await
    .expect("the sole retained driver reaches the paused publisher");
    let mut close = Box::pin(committer.close());
    let close_wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
    let close_waker = std::task::Waker::from(close_wake);
    let mut close_context = std::task::Context::from_waker(&close_waker);
    assert!(matches!(close.as_mut().poll(&mut close_context), std::task::Poll::Pending), "close joins the active post-witness driver instead of starting another verifier or publisher");
    assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt"]);
    drop(close);
    publisher_release.notify_one();
    let terminal = tokio::time::timeout(std::time::Duration::from_secs(5), &mut retry).await.expect("fresh retry observes retained publication").expect("fresh request joins the one publication");
    assert!(!terminal.applied, "the joining request observes the already-applied terminal state");
    assert_eq!(terminal.document_generation, 0, "the committed witness retains the live initial actor generation");
    assert_eq!(terminal.frontier.head_edit_ordinal, base.frontier.head_edit_ordinal + undo_contract["firstCommandOrdinalDelta"].as_u64().expect("first command delta"), "the first real command is the first history edit",);
    drop(retry);
    let approved = ledger.read(&accepted.job_id, &owner, 1_007).expect("approved proposal");
    assert_eq!(approved.proposal_state, super::super::schema::InferenceProposalStateV1::Approved);
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
        semio_framework::io::resolve_ready(owners.parent.as_ref().expect("retained parent Store").snapshot_pack()).expect("approved Map snapshot").pack
    };
    let after_base = InferenceMapBaseV1 {
        frontier: ArtifactFrontier {
            document_id: terminal.frontier.document_id.clone(),
            head_edit_ordinal: terminal.frontier.head_edit_ordinal,
            head_edit_id: terminal.frontier.head_edit_id.clone(),
            last_commit_seq: terminal.frontier.last_commit_seq,
            chain_hash: directory::os_directory::ArtifactHash::parse_hex(&terminal.frontier.chain_sha256).expect("approved chain hash"),
        },
        descriptor_digest: identity.descriptor_digest.clone(),
        pack: InferencePrivateBytesV1::new(after_pack, INPUT_MAX_BYTES).expect("bounded approved Map pack"),
    };
    let target = ledger.gis_map_approval_undo_target(&terminal.undo.target_id, &owner).expect("owner-retained durable undo target");
    assert_eq!(target.after_frontier, terminal.frontier);
    assert_eq!(target.after_base_digest, after_base.digest());
    let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).expect("retained original command");
    let inverses = directory::os_pack::json::from_json_str::<Vec<semio_s_artifact_gis_gismap::mutations::GisMapMutation>>(std::str::from_utf8(original.inverse_payload()).expect("canonical inverse text")).expect("canonical inverse mutations");
    assert_eq!(inverses.len(), 1, "the retained approval owns one exact parent inverse");
    let current = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(after_base.pack.as_slice()).expect("approved Map snapshot");
    let mut before = current.clone();
    semio_s_artifact_gis_gismap::mutations::apply_gis_map_mutation(&mut before, &inverses[0]).expect("server inverse applies to the exact current Map");
    use directory::Inference as _;
    let work = semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).expect("server reconstructs the original fixed-three work");
    let undo_diff = directory::os_pack::json::to_json_string(&inverses[0]).into_bytes();
    let undo_inverse = directory::os_pack::json::to_json_string(&vec![work.parent]).into_bytes();
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
        super::super::command::COMMAND_MAX_BYTES,
    )
    .expect("bounded undo command");
    let undo_command_hash = sha256(undo_command.as_slice());
    assert!(matches!(
        ledger.prepare_gis_map_approval_undo(&target, &undo_idempotency_key, &undo_operation_id, &undo_proposal_hash, &undo_mutation_id, &undo_command_hash, &undo_command,),
        Ok(super::super::sqlite::GisMapApprovalUndoAdmissionV1::Prepared)
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
        .expect("retained durable undo");
    assert!(undo_receipt.applied, "the first exact undo applies one second durable decision");
    assert_eq!(undo_receipt.frontier.head_edit_id, undo_mutation_id);
    assert_eq!(undo_receipt.frontier.head_edit_ordinal, terminal.frontier.head_edit_ordinal + undo_contract["undoCommandOrdinalDelta"].as_u64().expect("undo command delta"),);
    assert_eq!(undo_receipt.frontier.last_commit_seq, terminal.frontier.last_commit_seq + 1);
    assert!(!undo_contract["restoresGenesisFrontier"].as_bool().expect("frontier contract"));
    assert!(undo_receipt.frontier.head_edit_ordinal > 0 && undo_receipt.frontier.last_commit_seq > 0, "undo restores content through a real second command, never by resetting lineage to genesis");
    assert!(undo_ingress_released.load(Ordering::Acquire), "durable undo releases its retained Hub ingress only after publication ACK");
    let reverted = {
        use directory::os_store::SpaceMember as _;
        let documents = committer.documents.lock().await;
        let owners = match documents.get(&document_key(&scope)) {
            Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners,
            _ => panic!("the undo publication retains its exact three Store owners"),
        };
        let pack = semio_framework::io::resolve_ready(owners.parent.as_ref().expect("retained reverted parent Store").snapshot_pack()).expect("reverted Map snapshot").pack;
        <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(&pack).expect("reverted Map pack")
    };
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
        Ok(super::super::sqlite::GisMapApprovalUndoAdmissionV1::Replayed(receipt)) if receipt.frontier == undo_receipt.frontier
    ));

    committer.close().await.expect("committer close");
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
    pool.shutdown().expect("worker pool shutdown");
}

#[tokio::test]
async fn gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer() {
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
    let document = protocol::ArtifactId(document_key(&scope));
    let handle = database.ensure_document(&document).await.expect("document actor");
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> =
        Arc::new(OrderedApprovalCheckpointPublisherV1 { ledger: ledger.clone(), identity: identity.clone(), job_id: accepted.job_id.clone(), attempts: std::sync::atomic::AtomicUsize::new(0), order: order.clone(), pause: None });
    let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
    let key = document_key(&scope);
    let gate = Arc::new(tokio::sync::Mutex::new(()));
    let composed_children = base.composed_children().expect("exact composed children");
    let actor = approval_actor(&identity);
    let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
    let command_hash = sha256(command.as_slice());
    let invalid_children = vec!["gismap-drawing".to_owned(), "substituted-value".to_owned()];
    let foreign_scope_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let foreign_scope_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> = Arc::new(TestApprovalIngressAuthorityV1 {
        scope: DocumentScope::new("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.document_id.clone()),
        user_id: identity.user_id.clone(),
        session_id: identity.session_id.clone(),
        authorization_generation: identity.authorization_generation,
        released: Some(foreign_scope_ingress_released.clone()),
    });
    assert!(matches!(
        committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
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
                ingress: foreign_scope_ingress,
            })
            .await,
        Err(GisMapApprovalCommitErrorV1::Conflict)
    ));
    assert!(foreign_scope_ingress_released.load(Ordering::Acquire));
    let mut substituted_bases = Vec::new();
    let mut descriptor = canonical_map_base(&identity);
    descriptor.descriptor_digest = "9".repeat(64);
    substituted_bases.push(("descriptor", descriptor));
    let mut pack = canonical_map_base(&identity);
    pack.pack = InferencePrivateBytesV1::new(b"substituted-pack".to_vec(), INPUT_MAX_BYTES).expect("bounded substituted pack");
    substituted_bases.push(("pack", pack));
    let mut head_ordinal = canonical_map_base(&identity);
    head_ordinal.frontier.head_edit_ordinal += 1;
    substituted_bases.push(("head-ordinal", head_ordinal));
    let mut head_edit = canonical_map_base(&identity);
    head_edit.frontier.head_edit_id = "substituted-edit".to_owned();
    substituted_bases.push(("head-edit", head_edit));
    let mut commit = canonical_map_base(&identity);
    commit.frontier.last_commit_seq += 1;
    substituted_bases.push(("commit", commit));
    let mut chain = canonical_map_base(&identity);
    chain.frontier.chain_hash = directory::os_directory::ArtifactHash([1; 32]);
    substituted_bases.push(("chain", chain));
    for (name, candidate) in &substituted_bases {
        let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(
            matches!(
                committer
                    .commit(GisMapApprovalCommitRequestV1 {
                        composed_children: &composed_children,
                        scope: &scope,
                        actor: &actor,
                        mutation_id: &mutation_id,
                        command_hash: &command_hash,
                        job_id: &accepted.job_id,
                        proposal_hash: &proposal_hash,
                        command: command.as_slice(),
                        base: candidate,
                        base_frontier: &candidate.frontier,
                        deadline_ms: 60_000,
                        now_ms: 1_004,
                        document_write: gate.clone(),
                        ingress: tracked_approval_ingress(&identity, Some(ingress_released.clone())),
                    })
                    .await,
                Err(GisMapApprovalCommitErrorV1::Conflict)
            ),
            "{name} substitution reached cleanup admission",
        );
        assert!(ingress_released.load(Ordering::Acquire), "{name} substitution retained ingress");
    }
    let mut substituted_frontier = base.frontier.clone();
    substituted_frontier.head_edit_ordinal += 1;
    let frontier_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(matches!(
        committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &actor,
                mutation_id: &mutation_id,
                command_hash: &command_hash,
                job_id: &accepted.job_id,
                proposal_hash: &proposal_hash,
                command: command.as_slice(),
                base: &base,
                base_frontier: &substituted_frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: gate.clone(),
                ingress: tracked_approval_ingress(&identity, Some(frontier_ingress_released.clone())),
            })
            .await,
        Err(GisMapApprovalCommitErrorV1::Conflict)
    ));
    assert!(frontier_ingress_released.load(Ordering::Acquire));
    let substituted_actor = format!("user:{}#session:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.user_id);
    let actor_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(matches!(
        committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &substituted_actor,
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
                ingress: tracked_approval_ingress(&identity, Some(actor_ingress_released.clone())),
            })
            .await,
        Err(GisMapApprovalCommitErrorV1::Conflict)
    ));
    assert!(actor_ingress_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("pure admission refusal lookup").is_some(), "pure admission refusals preserve the prepared outbox");
    let substituted_command_hash = sha256(b"substituted-command");
    let substituted_proposal_hash = sha256(b"substituted-proposal");
    let substituted_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(
        matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &substituted_command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &substituted_proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(substituted_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ),
        "the public committer refuses a same-job substituted command/proposal tuple before cleanup admission",
    );
    assert!(substituted_ingress_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let prepared_after_substitution = ledger.approval_recovery_by_mutation(&mutation_id).expect("substitution outbox lookup").expect("exact prepared outbox remains");
    assert_eq!(
        (prepared_after_substitution.0.command_hash.as_str(), prepared_after_substitution.0.proposal_hash.as_str(), prepared_after_substitution.1.authorization_generation,),
        (command_hash.as_str(), proposal_hash.as_str(), identity.authorization_generation),
    );
    let cleanup_reservations = (0..super::super::schema::JOB_CAPACITY).map(|index| format!("{index:064x}")).collect::<Vec<_>>();
    for job_id in &cleanup_reservations {
        committer.reserve_cleanup_job(job_id).expect("bounded cleanup reservation");
    }
    let capacity_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let capacity_refused = committer
        .commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
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
            ingress: tracked_approval_ingress(&identity, Some(capacity_ingress_released.clone())),
        })
        .await;
    assert!(matches!(capacity_refused, Err(GisMapApprovalCommitErrorV1::Capacity)));
    assert!(capacity_ingress_released.load(Ordering::Acquire));
    assert_eq!(
        ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("capacity outbox query").rows.len(),
        1,
        "cleanup-capacity refusal preserves the caller's exact durable prepared owner",
    );
    for job_id in &cleanup_reservations {
        committer.release_cleanup_job(job_id);
    }
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let prepared_runtime_shutdown_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    committer.reserve_cleanup_job(&accepted.job_id).expect("prepared cleanup reservation");
    let prepared_runtime_shutdown_owner = GisMapPreparedApprovalV1 {
        scope: scope.clone(),
        job_id: accepted.job_id.clone(),
        mutation_id: mutation_id.clone(),
        command_hash: command_hash.clone(),
        proposal_hash: proposal_hash.clone(),
        ingress: tracked_approval_ingress(&identity, Some(prepared_runtime_shutdown_released.clone())),
    };
    let prepared_runtime_shutdown_key = format!("prepared:{}", accepted.job_id);
    assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(prepared_runtime_shutdown_key.clone()));
    let prepared_runtime_shutdown_task = GisMapPreparedApprovalTaskV1 { committer: committer.as_ref().clone(), maintenance_key: prepared_runtime_shutdown_key, owner: Some(prepared_runtime_shutdown_owner) };
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary prepared-cleanup runtime");
        runtime.spawn(async move {
            let _retained = prepared_runtime_shutdown_task;
            std::future::pending::<()>().await;
        });
        runtime.block_on(tokio::task::yield_now());
    })
    .join()
    .expect("temporary prepared-cleanup runtime shutdown");
    assert!(committer.parked_prepared.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
    assert!(!prepared_runtime_shutdown_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact prepared ingress cursor instead of dropping it");
    resume_parked_cleanup(&committer).await;
    assert!(prepared_runtime_shutdown_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("prepared runtime-shutdown query").rows.is_empty());
    ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after prepared cleanup runtime shutdown");
    let runtime_shutdown_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    committer.reserve_cleanup_job(&accepted.job_id).expect("request cleanup reservation");
    let runtime_shutdown_ingress = tracked_approval_ingress(&identity, Some(runtime_shutdown_ingress_released.clone()));
    let runtime_shutdown_request = Arc::new(GisMapApprovalRequestTokenV1);
    let (runtime_shutdown_identity, _) = RetainedGisMapApprovalCommitterV1::preflight(&GisMapApprovalCommitRequestV1 {
        composed_children: &composed_children,
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
        ingress: runtime_shutdown_ingress,
    })
    .expect("runtime-shutdown cursor preflight");
    let runtime_shutdown_owner = GisMapAbandonedRequestV1 { key: key.clone(), identity: runtime_shutdown_identity, request: runtime_shutdown_request };
    let runtime_shutdown_maintenance_key = RetainedGisMapApprovalCommitterV1::abandoned_request_maintenance_key(&runtime_shutdown_owner);
    assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(runtime_shutdown_maintenance_key.clone()));
    let runtime_shutdown_task = GisMapAbandonedRequestTaskV1 { committer: committer.as_ref().clone(), maintenance_key: runtime_shutdown_maintenance_key, owner: Some(runtime_shutdown_owner) };
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary cleanup runtime");
        runtime.spawn(async move {
            let _retained = runtime_shutdown_task;
            std::future::pending::<()>().await;
        });
        runtime.block_on(tokio::task::yield_now());
    })
    .join()
    .expect("temporary cleanup runtime shutdown");
    assert!(committer.parked_requests.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
    assert!(!runtime_shutdown_ingress_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact ingress cursor instead of dropping it");
    resume_parked_cleanup(&committer).await;
    assert!(runtime_shutdown_ingress_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("runtime-shutdown outbox query").rows.is_empty());
    ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after temporary cleanup runtime shutdown");
    let thread_committer = committer.clone();
    let thread_identity = identity.clone();
    let thread_scope = scope.clone();
    let thread_frontier = base.frontier.clone();
    let thread_descriptor = base.descriptor_digest.clone();
    let thread_pack = base.pack.as_slice().to_vec();
    let thread_command = command.as_slice().to_vec();
    let thread_children = composed_children.clone();
    let thread_actor = actor.clone();
    let thread_mutation_id = mutation_id.clone();
    let thread_command_hash = command_hash.clone();
    let thread_job_id = accepted.job_id.clone();
    let thread_proposal_hash = proposal_hash.clone();
    let thread_gate = gate.clone();
    let thread_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_released = thread_ingress_released.clone();
    std::thread::spawn(move || {
        let thread_base = InferenceMapBaseV1 { frontier: thread_frontier, descriptor_digest: thread_descriptor, pack: InferencePrivateBytesV1::new(thread_pack, INPUT_MAX_BYTES).expect("bounded thread base") };
        let thread_command = InferencePrivateBytesV1::new(thread_command, super::super::command::COMMAND_MAX_BYTES).expect("bounded thread command");
        let future = thread_committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &thread_children,
            scope: &thread_scope,
            actor: &thread_actor,
            mutation_id: &thread_mutation_id,
            command_hash: &thread_command_hash,
            job_id: &thread_job_id,
            proposal_hash: &thread_proposal_hash,
            command: thread_command.as_slice(),
            base: &thread_base,
            base_frontier: &thread_base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: thread_gate,
            ingress: tracked_approval_ingress(&thread_identity, Some(thread_released)),
        });
        drop(future);
    })
    .join()
    .expect("no-runtime unpolled owner thread");
    assert!(thread_ingress_released.load(Ordering::Acquire), "no-runtime future Drop synchronously returns its ingress after exact outbox abandonment");
    assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("no-runtime outbox query").rows.is_empty());
    ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after no-runtime Drop");
    let unpolled_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let unpolled = committer.commit(GisMapApprovalCommitRequestV1 {
        composed_children: &composed_children,
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
        ingress: tracked_approval_ingress(&identity, Some(unpolled_ingress_released.clone())),
    });
    drop(unpolled);
    wait_for_abandoned_prepared_handoff(&committer, &ledger, &identity, &accepted.job_id, &key, &gate, unpolled_ingress_released.as_ref()).await;
    ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after an unpolled future");
    let rejected_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(matches!(
        committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &invalid_children,
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
                now_ms: 1_006,
                document_write: gate.clone(),
                ingress: tracked_approval_ingress(&identity, Some(rejected_ingress_released.clone())),
            })
            .await,
        Err(GisMapApprovalCommitErrorV1::Rejected)
    ),);
    assert!(rejected_ingress_released.load(Ordering::Acquire));
    assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("rejected preflight lookup").is_some(), "a pure preflight rejection leaves the prepared outbox unchanged");
    for (phase_index, phase) in [AbandonedApprovalPhaseV1::Preflight, AbandonedApprovalPhaseV1::Assembly, AbandonedApprovalPhaseV1::Journal].into_iter().enumerate() {
        let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
        let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
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
            ingress,
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, phase)).await.expect("approval reaches its retained cancellation phase");
        drop(future);
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        wait_for_abandoned_approval_handoff(&committer, &key, &gate, ingress_released.as_ref()).await;
        let actor = handle.checkpoint_publication_snapshot().await.expect("unchanged actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id), (0, 0, None));
        let pending = ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("abandoned outbox query");
        assert!(pending.rows.is_empty(), "the autonomous cancellation owner clears its exact prepared outbox row before releasing Hub ingress");
        let revived = ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005 + u64::try_from(phase_index).expect("bounded phase")).expect("same owner revives its exact abandoned outbox row");
        assert_eq!((revived.mutation_id.as_str(), revived.command_hash.as_str(), revived.proposal_hash.as_str()), (mutation_id.as_str(), command_hash.as_str(), proposal_hash.as_str()));
    }
    let prepared_events = ledger.events(&accepted.job_id, &reader(&identity), 0, 1_008).expect("owner event page").events.into_iter().filter(|event| event.kind == "approval-prepared").count();
    assert_eq!(prepared_events, 1, "three exact cancellation/revival cycles preserve one immutable preparation event");
    assert!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty(), "pre-witness cancellation emits neither a public checkpoint nor a peer rebootstrap signal");
    assert_eq!(ledger.read(&accepted.job_id, &reader(&identity), 1_005).expect("offered proposal").proposal_state, super::super::schema::InferenceProposalStateV1::Offered);
    let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
    let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
        composed_children: &composed_children,
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
        now_ms: 1_006,
        document_write: gate.clone(),
        ingress,
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, AbandonedApprovalPhaseV1::Committed)).await.expect("approval reaches its committed receipt cutover");
    drop(future);
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let approved = ledger.read(&accepted.job_id, &reader(&identity), 1_007).is_ok_and(|view| view.proposal_state == super::super::schema::InferenceProposalStateV1::Approved);
            let maintenance_idle = !committer.maintenance_owns_document(&key);
            if approved && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("receipt cutover autonomously reaches public checkpoint and ledger apply");
    assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"],);
    let actor = handle.checkpoint_publication_snapshot().await.expect("committed actor frontier");
    assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id.as_ref().map(|id| id.0.as_str())), (1, 1, Some(mutation_id.as_str())));
    committer.close().await.expect("committer close");
    drop(committer);
    drop(handle);
    let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
    database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
    drop(database);
    pool.shutdown().expect("worker pool shutdown");
}

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
    let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
    let gate = Arc::new(tokio::sync::Mutex::new(()));
    let children = base.composed_children().expect("exact composed children");
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
    pool.shutdown().expect("worker pool shutdown");
}

#[test]
fn gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary() {
    let fixture = fixture();
    let preview = gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), fixture["proposalHash"].as_str().expect("proposal hash"), fixture["proposalCanonical"].as_str().expect("canonical proposal").as_bytes())
        .expect("typed owner preview");
    assert_eq!(serde_json::to_value(preview).expect("preview wire"), fixture["preview"]);
    assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &"0".repeat(64), fixture["proposalCanonical"].as_str().expect("proposal").as_bytes()).err(), Some(InferenceRouteErrorV1::Conflict));
    let substituted = b"{\"DeleteRegion\":{\"id\":\"inference-11111111111111111111111111111111\"}}";
    assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(substituted), substituted).err(), Some(InferenceRouteErrorV1::Conflict));
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
        assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(&bytes), &bytes).err(), Some(expected), "{name}");
    }
    for (name, bytes) in [
        ("malformed", b"{".as_slice()),
        (
            "non-finite",
            br#"{"CreateRegion":{"index":0,"item":{"id":"inference-11111111111111111111111111111111","data":{"id":"inference-11111111111111111111111111111111","kind":"inference-bounds","ring":[[1e999,46],[9,46],[9,48],[1e999,48],[1e999,46]]}}}}"#
                .as_slice(),
        ),
    ] {
        assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(bytes), bytes).err(), Some(InferenceRouteErrorV1::Invalid), "{name}");
    }
    let identity = identity();
    let frozen: super::super::schema::InferenceBindingIdentityV1 = serde_json::from_value(fixture["binding"].clone()).expect("frozen binding identity");
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
    assert_eq!(limits["requestMaxBytes"], super::super::schema::REQUEST_MAX_BYTES as u64);
    assert_eq!(limits["inputMaxBytes"], INPUT_MAX_BYTES as u64);
    assert_eq!(limits["resultMaxBytes"], RESULT_MAX_BYTES as u64);
    assert_eq!(limits["proposalMaxBytes"], PROPOSAL_MAX_BYTES as u64);
    assert_eq!(limits["commandMaxBytes"], super::super::command::COMMAND_MAX_BYTES as u64);
    assert_eq!(limits["identityJsonMaxBytes"], super::super::schema::IDENTITY_JSON_MAX_BYTES as u64);
    assert_eq!(limits["jobCapacity"], super::super::schema::JOB_CAPACITY as u64);
    assert_eq!(limits["operationCapacity"], OPERATION_CAPACITY as u64);
    assert_eq!(limits["documentGateCapacity"], DOCUMENT_GATE_CAPACITY as u64);
    assert_eq!(limits["progressMaxCursor"], PROGRESS_MAX_CURSOR);
    assert_eq!(limits["eventPageMaxItems"], super::super::schema::EVENT_PAGE_MAX_ITEMS as u64);
    assert_eq!(limits["claimLeaseMaxMs"], super::super::schema::CLAIM_LEASE_MAX_MS);
    assert_eq!(limits["jobMaxLifetimeMs"], super::super::schema::JOB_MAX_LIFETIME_MS);
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
