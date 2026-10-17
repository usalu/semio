/// 🧰️ Law fixtures: ledger and identity builders, canonical approval and map inputs, ordered publishers and the approval poll harness.

type GisRetainedCommitterV1 = RetainedGisMapApprovalCommitterV1<GisMapInferenceBindingV1>;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
}

fn ledger_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture")
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
    use pack::snapshot::ArtifactPack as _;
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
    let (_snapshot, work) = <GisMapInferenceBindingV1 as ComposedMapCommitBindingV1>::executed_approval_work(base.pack.as_slice(), job_id, &inference_budget()).expect("real GIS Map inference and server-derived fixed-three CreateRegion work");
    let proposal_bytes = work.parent_json;
    let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES).expect("bounded canonical proposal");
    let proposal_hash = sha256(proposal.as_slice());
    let inverse = work.parent_inverse_json;
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
    (proposal, InferencePrivateBytesV1::new(command, semio_hub_inference::command::COMMAND_MAX_BYTES).expect("bounded approval command"))
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

static LAST_PUBLISHER_REGION_EVIDENCE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

fn record_publisher_region_evidence(evidence: String) {
    *LAST_PUBLISHER_REGION_EVIDENCE.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(evidence);
}

fn last_publisher_region_evidence() -> Option<String> {
    LAST_PUBLISHER_REGION_EVIDENCE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

/// 🧩️ The state an `ArtifactPair` materializes to. The `pack` half of every pair in this product is
/// the document's stored GENESIS pack, never its current fold: `print_document_pack` shares
/// `vcs.genesis.pack()`, the native codec's `apply_ops_binary` retains exactly those bytes while
/// appending to the `spr`, and every load path (`parse_decoded_document_spr`, `open_member_store`)
/// folds the `spr` log back onto it. So a committed mutation is visible in a published checkpoint
/// only when BOTH halves are folded — reading `pair.pack` alone asks the genesis whether an approval
/// happened, which it can never answer.
async fn published_pair_snapshot(pack: &[u8], spr: &[u8]) -> Result<semio_s_artifact_gis_gismap::GisMapSnapshot, GisMapApprovalCommitErrorV1> {
    let parsed = directory::os_store::text_format::parse_document_pack::<semio_s_artifact_gis_gismap::GisMapSnapshot, semio_s_artifact_gis_gismap::mutations::GisMapMutation>(pack, spr)
        .await
        .map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
    let snapshot = parsed.snapshot.clone();
    directory::os_store::test_support::retire_parsed_document(parsed);
    Ok(snapshot)
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
            let snapshot = published_pair_snapshot(&request.pair.pack, &request.pair.spr).await?;
            let expected_region = format!("inference-{}", self.job_id);
            let attempt = self.attempts.fetch_add(1, Ordering::AcqRel);
            let has_expected_region = snapshot
                .regions
                .iter()
                .any(|region| region.id == expected_region && region.data.get("id").and_then(|value| value.as_str()) == Some(expected_region.as_str()) && region.data.get("kind").and_then(|value| value.as_str()) == Some("inference-bounds"));
            if has_expected_region != (attempt < 2) {
                record_publisher_region_evidence(format!(
                    "attempt {attempt} wanted {expected_region} present={}, folded pair carried {:?}",
                    attempt < 2,
                    snapshot.regions.iter().map(|region| (region.id.clone(), region.data.get("id").and_then(|value| value.as_str()).map(str::to_owned), region.data.get("kind").and_then(|value| value.as_str()).map(str::to_owned))).collect::<Vec<_>>()
                ));
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublisherRegionDiffers));
            }
            let view = self.ledger.read(&self.job_id, &reader(&self.identity), decision_now_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let expected_proposal_state = if attempt < 2 { semio_hub_inference::schema::InferenceProposalStateV1::Offered } else { semio_hub_inference::schema::InferenceProposalStateV1::Approved };
            if view.proposal_state != expected_proposal_state || document_write.gate.try_lock().is_ok() {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublisherProposalStateOrGate));
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
            let frontier = GisRetainedCommitterV1::publication_frontier(&request.scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            let pack_hash = directory::os_directory::io::binary::artifact_hash::parse_artifact_hash_hex(&sha256(&request.pair.pack)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            let spr_hash = directory::os_directory::io::binary::artifact_hash::parse_artifact_hash_hex(&sha256(&request.pair.spr)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-ack");
            Ok(directory::os_directory::PublishedArtifactCheckpoint {
                scope: request.scope,
                checkpoint_id: directory::os_directory::ArtifactHash([0x45; 32]),
                parent_checkpoint_id: None,
                descriptor_digest_v1: directory::os_directory::io::binary::artifact_hash::parse_artifact_hash_hex(&request.descriptor_digest).ok_or(GisMapApprovalCommitErrorV1::Storage)?,
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
        if view.proposal_state != semio_hub_inference::schema::InferenceProposalStateV1::Approved {
            return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublisherCheckpointApplyState));
        }
        self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("peer-rebootstrap");
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

async fn poll_approval_to_phase<F>(future: &mut std::pin::Pin<Box<F>>, committer: &GisRetainedCommitterV1, key: &str, phase: AbandonedApprovalPhaseV1)
where
    F: std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + ?Sized,
{
    let wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
    let waker = std::task::Waker::from(wake.clone());
    for _ in 0..4_096 {
        while !wake.ready.swap(false, Ordering::AcqRel) {
            tokio::task::yield_now().await;
        }
        let outcome = {
            let mut context = std::task::Context::from_waker(&waker);
            future.as_mut().poll(&mut context).map(|result| result.map(|receipt| receipt.applied))
        };
        if let std::task::Poll::Ready(result) = outcome {
            panic!(
                "approval must remain retained before its cancellation phase {phase:?}: it completed with {result:?}; last prepare conflict {:?}, last conflict arm {:?}, last identity mismatch {:?}, last abandoned turn {:?}, last storage refusal {:?}",
                last_gis_map_prepare_conflict(),
                last_gis_map_commit_conflict_arm(),
                last_gis_map_identity_mismatch(),
                last_abandoned_assembly_turn(),
                last_gis_map_storage_refusal(),
            );
        }
        let reached = {
            let documents = committer.documents.try_lock().expect("a retained approval never parks holding the committer document map");
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

/// ⏰️ An OS-thread hang guard for one law in this module. Every bound in here is a
/// `tokio::time::timeout`, which cannot fire when the current-thread runtime itself is wedged — the
/// law then burns nextest's whole 300 s kill budget and names nothing. This thread watches the wall
/// clock instead: on expiry it prints the last phase the law entered plus the last abandoned
/// `Assembly` turn and aborts, so a wedge costs seconds and arrives with a name. It is declared
/// FIRST in the law so it drops LAST, after the committer and its pool.
struct RuntimeLawHangWatchdogV1 {
    phase: Arc<std::sync::Mutex<String>>,
    finished: Arc<std::sync::atomic::AtomicBool>,
}

impl RuntimeLawHangWatchdogV1 {
    fn arm(law: &'static str, budget: std::time::Duration) -> Self {
        let phase = Arc::new(std::sync::Mutex::new(String::from("armed, before the first step")));
        let finished = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let watched_phase = phase.clone();
        let watched_finished = finished.clone();
        std::thread::spawn(move || {
            let deadline = std::time::Instant::now() + budget;
            while std::time::Instant::now() < deadline {
                if watched_finished.load(Ordering::Acquire) {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            if watched_finished.load(Ordering::Acquire) {
                return;
            }
            let entered = watched_phase.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
            eprintln!(
                "law hang watchdog: {law} exceeded {budget:?}; last phase entered: {entered}; last abandoned turn {:?}; last close-store step {:?}; last identity mismatch {:?}; last prepare conflict {:?}; last conflict arm {:?}",
                last_abandoned_assembly_turn(),
                last_close_store_step(),
                last_gis_map_identity_mismatch(),
                last_gis_map_prepare_conflict(),
                last_gis_map_commit_conflict_arm(),
            );
            std::process::abort();
        });
        Self { phase, finished }
    }

    fn at(&self, phase: impl Into<String>) {
        *self.phase.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = phase.into();
    }
}

impl Drop for RuntimeLawHangWatchdogV1 {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::Release);
    }
}
