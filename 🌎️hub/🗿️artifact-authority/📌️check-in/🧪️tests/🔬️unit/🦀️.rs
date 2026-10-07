use super::super::{AcceptedArtifactOperation, AuthorityLimits};
use super::*;
use ::directory::os_directory::{DocumentFrontier, DocumentOwner, DocumentCheckInStatusV1};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 🧮️ A replay codec whose pair is a decimal sum and whose ledger stream is `+n` terms, so a law can
/// read the fold back exactly.
struct SumCodec {
    identity: TrustedArtifactIdentity,
    replays: AtomicUsize,
}

impl TrustedArtifactCodec for SumCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        std::str::from_utf8(&pair.pack).ok().and_then(|text| text.parse::<i64>().ok()).ok_or(AuthorityError::Codec { stage, message: "pack is not a sum".into() })?;
        Ok(())
    }

    async fn apply_operation(&self, pair: ArtifactPair, _operation: &AcceptedArtifactOperation, _context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        Ok(pair)
    }
}

impl TrustedArtifactReplayCodec for SumCodec {
    async fn replay_envelopes(&self, mut pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        self.replays.fetch_add(1, Ordering::SeqCst);
        let base: i64 = std::str::from_utf8(&pair.pack).unwrap().parse().unwrap();
        let mut sum = base;
        for term in std::str::from_utf8(envelopes).map_err(|_| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: "ledger is not text".into() })?.split('+').filter(|term| !term.is_empty()) {
            sum += term.parse::<i64>().map_err(|_| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: "ledger term is not a number".into() })?;
        }
        pair.pack = sum.to_string().into_bytes();
        pair.spr.extend_from_slice(envelopes);
        Ok(pair)
    }
}

struct SumCatalog(SumCodec);

impl TrustedArtifactCatalog for SumCatalog {
    type Codec = SumCodec;

    async fn resolve<'a>(&'a self, _required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        Ok(&self.0)
    }
}

fn descriptor() -> DocumentDescriptor {
    DocumentDescriptor {
        space_id: "raum".to_string(),
        document_id: "karte".to_string(),
        artifact_kind: "s.sum".to_string(),
        artifact_schema: "sum".to_string(),
        owner: DocumentOwner { plugin_id: "sum".to_string(), package_id: "semio:sum".to_string(), version: "1.0.0".to_string(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    }
}

fn head(ordinal: u64) -> ArtifactFrontier {
    ArtifactFrontier { document_id: "karte".to_string(), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: ArtifactHash([ordinal as u8; 32]) }
}

fn materialization(envelopes: &[u8]) -> CheckInMaterialization {
    CheckInMaterialization { descriptor: descriptor(), scope: DocumentScope::new("raum", "karte"), parent_checkpoint_id: ArtifactHash([9; 32]), base_pair: ArtifactPair { pack: b"10".to_vec(), spr: b"seed".to_vec() }, head: head(3), envelopes: envelopes.to_vec() }
}

fn authority() -> ValidatingCanonicalArtifactAuthority<SumCatalog> {
    ValidatingCanonicalArtifactAuthority::new(SumCatalog(SumCodec { identity: TrustedArtifactIdentity::from_descriptor(&descriptor()), replays: AtomicUsize::new(0) }))
}

/// 📌️ The candidate is the fold of the ledger onto the base pair, its baseline is exactly the named
/// head, its parent the active checkpoint, and its identity the canonical checkpoint encoding; a
/// refused fold, a genesis head, a foreign head and a cancelled job transfer nothing.
#[tokio::test]
async fn materialize_check_in_folds_the_ledger_onto_the_active_pair_at_the_named_head() {
    let authority = authority();
    let job = DocumentCheckInJob::new("0a1b2c3d4e5f60718293a4b5c6d7e8f9", "aa");
    let context = OperationContext::stall_bounded(DOCUMENT_CHECK_IN_STALL_BOUND_MS, AuthorityLimits::maximum(), job.as_ref()).unwrap();
    let candidate = authority.materialize_check_in(materialization(b"+5+-2"), &context).await.expect("the ledger folds");
    assert_eq!(candidate.pair.pack, b"13".to_vec());
    assert_eq!(candidate.checkpoint.baseline_frontier, head(3));
    assert_eq!(candidate.checkpoint.parent_checkpoint_id, Some(ArtifactHash([9; 32])));
    assert_eq!(candidate.checkpoint.checkpoint_id, ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&candidate.checkpoint).unwrap())));
    assert_eq!(job.status().progress.completed_units, 5, "codec resolved, input validated, replayed and derived");
    assert_eq!(job.status().phase, DocumentCheckInPhaseV1::Materializing);

    assert!(matches!(authority.materialize_check_in(materialization(b"+x"), &context).await, Err(AuthorityError::Codec { .. })));
    let mut genesis = materialization(b"+1");
    genesis.head = ArtifactFrontier { document_id: "karte".into(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash([0; 32]) };
    assert_eq!(authority.materialize_check_in(genesis, &context).await.unwrap_err(), AuthorityError::InvalidFrontier);
    let mut foreign = materialization(b"+1");
    foreign.head.document_id = "andere".into();
    assert_eq!(authority.materialize_check_in(foreign, &context).await.unwrap_err(), AuthorityError::InvalidFrontier);
    assert_eq!(authority.materialize_check_in(materialization(b""), &context).await.unwrap_err(), AuthorityError::InvalidOperationOrder);
    job.cancel();
    assert_eq!(authority.materialize_check_in(materialization(b"+1"), &context).await.unwrap_err(), AuthorityError::Cancelled);
}

/// 🚦️ Progress is monotonic and bounded, exactly one terminal outcome wins, a refusal after the
/// client's cancellation reads as cancelled, and every status the job emits is canonical wire.
#[test]
fn check_in_job_status_is_monotonic_single_terminal_and_canonical() {
    let job = DocumentCheckInJob::new("0a1b2c3d4e5f60718293a4b5c6d7e8f9", "aa");
    assert_eq!(job.status().phase, DocumentCheckInPhaseV1::Accepted);
    job.advance(DocumentCheckInPhaseV1::Materializing, 3);
    job.advance(DocumentCheckInPhaseV1::Materializing, 1);
    assert_eq!(job.status().progress.completed_units, 3, "progress never goes back");
    job.advance(DocumentCheckInPhaseV1::Publishing, 99);
    assert_eq!(job.status().progress.completed_units, DOCUMENT_CHECK_IN_TOTAL_UNITS - 1, "only ready completes the last unit");
    let baseline = EditedArtifactFrontierV1::of_artifact_frontier(&head(3)).unwrap();
    job.finish_ready(ArtifactHash([5; 32]), ArtifactHash([6; 32]), baseline.clone());
    job.finish_refused(DocumentCheckInRefusalV1::Unavailable);
    job.finish_cancelled();
    let ready = job.status();
    assert_eq!(ready.phase, DocumentCheckInPhaseV1::Ready);
    assert_eq!(ready.progress.completed_units, DOCUMENT_CHECK_IN_TOTAL_UNITS);
    assert_eq!(ready.ready.as_ref().unwrap().baseline, baseline);
    assert_eq!(DocumentCheckInStatusV1::parse_canonical_json(&ready.canonical_json().unwrap()), Some(ready));

    let refused = DocumentCheckInJob::new("00000000000000000000000000000001", "aa");
    refused.finish_refused(DocumentCheckInRefusalV1::StaleHead);
    assert_eq!((refused.status().phase, refused.status().refusal), (DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::StaleHead)));
    assert!(refused.status().canonical_json().is_some());

    let cancelled = DocumentCheckInJob::new("00000000000000000000000000000002", "aa");
    cancelled.cancel();
    cancelled.finish_refused(DocumentCheckInRefusalV1::Unavailable);
    assert_eq!((cancelled.status().phase, cancelled.status().refusal), (DocumentCheckInPhaseV1::Cancelled, None));
    assert!(cancelled.status().canonical_json().is_some());

    let revoked = DocumentCheckInJob::new("00000000000000000000000000000003", "aa");
    revoked.revoke();
    assert!(revoked.is_cancelled(), "revocation stops the job at its next checkpoint");
    revoked.finish_cancelled();
    assert_eq!((revoked.status().phase, revoked.status().refusal), (DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)));
}

/// 🗂️ A known request joins its job, a fresh one owns a new job, live jobs are bounded, terminal
/// statuses are retained oldest-first, and cancelling a document reaches every job of it.
#[test]
fn check_in_jobs_join_bound_retain_and_cancel_by_document() {
    let jobs = DocumentCheckInJobs::default();
    let key = |request: u32, document: &str| DocumentCheckInKey { user_id: "ada".into(), space_id: "raum".into(), document_id: document.into(), request_id: format!("{request:032x}") };
    let DocumentCheckInAdmission::Owner(first) = jobs.admit(key(1, "karte"), "aa") else { panic!("a fresh request owns its job") };
    let DocumentCheckInAdmission::Existing(joined) = jobs.admit(key(1, "karte"), "aa") else { panic!("a known request joins") };
    assert!(matches!(jobs.admit(key(1, "karte"), "bb"), DocumentCheckInAdmission::Conflict), "the same request id naming another request conflicts");
    assert!(Arc::ptr_eq(&first, &joined));
    for request in 2..=DOCUMENT_CHECK_IN_MAX_LIVE as u32 {
        assert!(matches!(jobs.admit(key(request, "karte"), "aa"), DocumentCheckInAdmission::Owner(_)));
    }
    assert!(matches!(jobs.admit(key(9_999, "karte"), "aa"), DocumentCheckInAdmission::Unavailable), "live jobs are bounded");
    jobs.cancel_document("raum", "karte");
    assert!(first.is_cancelled() && jobs.get(&key(2, "karte")).unwrap().is_cancelled());
    first.finish_cancelled();
    assert!(matches!(jobs.admit(key(9_999, "karte"), "aa"), DocumentCheckInAdmission::Owner(_)), "a terminal job frees a live slot");
    jobs.forget(&key(9_999, "karte"));
    assert!(jobs.get(&key(9_999, "karte")).is_none());
    assert_eq!(check_in_refusal_of_authority_error(&AuthorityError::Cancelled), None);
    assert_eq!(check_in_refusal_of_authority_error(&AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: String::new() }), Some(DocumentCheckInRefusalV1::CodecRefused));
    assert_eq!(check_in_refusal_of_authority_error(&AuthorityError::Publication(String::new())), Some(DocumentCheckInRefusalV1::ActiveCheckpointChanged));
}

//#region 🔖️Supersede
/// 🗺️ A linked GIS Map codec — the shape `PluginHostArtifactCodec` and a linked `VerifiedNativeArtifactCodec` give a
/// hub: `codec.replay-envelopes` is `store::replay_envelopes_onto_pair`, a replica's own fold.
#[cfg(feature = "native-artifact-execution")]
struct LinkedGisMapCodec {
    identity: TrustedArtifactIdentity,
    codec: ::directory::os_store::ArtifactCodec,
}

#[cfg(feature = "native-artifact-execution")]
impl TrustedArtifactCodec for LinkedGisMapCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        (self.codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: error.to_string() })?;
        Ok(())
    }

    async fn apply_operation(&self, pair: ArtifactPair, _operation: &AcceptedArtifactOperation, _context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        Ok(pair)
    }
}

#[cfg(feature = "native-artifact-execution")]
impl TrustedArtifactReplayCodec for LinkedGisMapCodec {
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let (pack, spr, _) = (self.codec.replay_envelopes)(&pair.pack, &pair.spr, envelopes).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: error.to_string() })?;
        Ok(ArtifactPair { pack, spr })
    }
}

#[cfg(feature = "native-artifact-execution")]
struct LinkedGisMapCatalog(LinkedGisMapCodec);

#[cfg(feature = "native-artifact-execution")]
impl TrustedArtifactCatalog for LinkedGisMapCatalog {
    type Codec = LinkedGisMapCodec;

    async fn resolve<'a>(&'a self, _required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        Ok(&self.0)
    }
}

/// 🌱️ The GIS Map genesis pair of the check-in document and the ledger a real map editor emits for `mutations`, one
/// one-operation edit each — exactly its store's own event log.
#[cfg(feature = "native-artifact-execution")]
async fn gis_map_ledger(mutations: Vec<semio_s_artifact_gis_gismap::GisMapMutation>) -> (ArtifactPair, Vec<protocol::MutationEnvelope>) {
    use ::directory::os_store::{ArtifactCommand, ArtifactPack, ArtifactStore, SnapshotRetirementStep};
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot, GIS_MAP_SCHEMA};
    let pack = <GisMapSnapshot as ArtifactPack>::encode_pack(&GisMapSnapshot::default());
    let spr = ::directory::os_store::empty_document_spr("karte", GIS_MAP_SCHEMA).await;
    let parsed = ::directory::os_store::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&pack, &spr).await.expect("genesis pair parses");
    let mut store = ArtifactStore::<GisMapSnapshot, GisMapMutation>::new(parsed.into_envelope(), directory::ActorId(directory::os_spr::LOCAL_ACTOR_ID.into())).await.expect("editor store");
    store.install_document_store_owners_exact(::directory::os_store::bounded_artifact_store_owners());
    let mut applied = Ok(());
    for mutation in mutations {
        if let Err(error) = store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await {
            applied = Err(error);
            break;
        }
    }
    let events = store.event_log();
    loop {
        match store.close_owned_step(1, ::directory::os_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("editor store closes") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { .. } => {}
            SnapshotRetirementStep::Blocked => panic!("editor store close blocked"),
        }
    }
    drop(store);
    applied.expect("editor edits apply");
    (ArtifactPair { pack, spr }, events.expect("editor ledger"))
}

/// ✏️ A collaborator's supersession of `targets` authored after the whole `ledger`: its envelope depends on exactly its
/// targets, and a replacement input is the aggregate op's canonical `OpBinary` bytes under the artifact schema.
#[cfg(feature = "native-artifact-execution")]
fn gis_map_supersede(ledger: &[protocol::MutationEnvelope], inputs: Vec<(usize, Option<semio_s_artifact_gis_gismap::GisMapMutation>)>) -> protocol::MutationEnvelope {
    use protocol::OpBinary;
    let last = ledger.last().expect("a ledger to supersede").timestamp;
    let inputs: Vec<protocol::SupersededInput> = inputs
        .into_iter()
        .map(|(position, replacement)| protocol::SupersededInput {
            target: ledger[position].mutation_id.clone(),
            replacement: match replacement {
                Some(mutation) => protocol::InputReplacement::Input { schema: semio_s_artifact_gis_gismap::GIS_MAP_SCHEMA.to_string(), payload: mutation.encode_op().expect("canonical op bytes") },
                None => protocol::InputReplacement::Withdrawn,
            },
        })
        .collect();
    let supersede = protocol::TransitionSupersede { scope: None, inputs };
    let dependencies = supersede.targets();
    let timestamp = protocol::HybridLogicalTimestamp { actor: last.actor.wrapping_add(1), physical_ms: last.physical_ms + 1, logical: 0 };
    protocol::history_transition_envelope(&protocol::HistoryTransition::Supersede(supersede), &ledger[0].document_id, &protocol::ActorId("collaborator".into()), dependencies, timestamp)
}

/// 📍️ A GIS Map position `id` at longitude `lon`.
#[cfg(feature = "native-artifact-execution")]
fn gis_map_position(index: usize, id: &str, lon: f64) -> semio_s_artifact_gis_gismap::GisMapMutation {
    use semio_s_artifact_gis_gismap::mutations::create_position::CreatePosition;
    let data = ::directory::DslValue::object([("lon".into(), ::directory::DslValue::float(lon)), ("lat".into(), ::directory::DslValue::float(47.0))]);
    semio_s_artifact_gis_gismap::GisMapMutation::CreatePosition(CreatePosition { index, item: semio_s_artifact_gis_gismap::MapFeature { id: id.into(), data } })
}

/// 🏗️ The check-in authority over the linked GIS Map codec.
#[cfg(feature = "native-artifact-execution")]
fn gis_map_authority() -> ValidatingCanonicalArtifactAuthority<LinkedGisMapCatalog> {
    let codec = ::directory::os_store::ArtifactCodec::of::<semio_s_artifact_gis_gismap::GisMapSnapshot, semio_s_artifact_gis_gismap::GisMapMutation>(semio_s_artifact_gis_gismap::GIS_MAP_SCHEMA);
    ValidatingCanonicalArtifactAuthority::new(LinkedGisMapCatalog(LinkedGisMapCodec { identity: TrustedArtifactIdentity::from_descriptor(&descriptor()), codec }))
}

/// ✏️ A committed ledger holding a collaborator's `Supersede` of another author's operation materializes through Check In
/// into a pair that keeps the supersession and whose reload folds it: the position carries the replacement input, the
/// superseded edit keeps its slot and identity (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING `📋️design.md` §2, §3.1).
#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn materialize_check_in_folds_a_supersession_of_the_ledger() {
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot};
    let (genesis, mut ledger) = gis_map_ledger(vec![gis_map_position(0, "p", 7.0)]).await;
    ledger.push(gis_map_supersede(&ledger, vec![(0, Some(gis_map_position(0, "p", 9.0)))]));
    let authority = gis_map_authority();
    let job = DocumentCheckInJob::new("0a1b2c3d4e5f60718293a4b5c6d7e8f9", "aa");
    let context = OperationContext::stall_bounded(DOCUMENT_CHECK_IN_STALL_BOUND_MS, AuthorityLimits::maximum(), job.as_ref()).unwrap();
    let request = CheckInMaterialization { base_pair: genesis, envelopes: ::directory::os_spr::encode_envelopes(&ledger), ..materialization(b"+0") };
    let candidate = authority.materialize_check_in(request, &context).await.expect("a ledger holding a supersession checks in");
    let history = ::directory::os_spr::decode_history(&candidate.pair.spr, &::directory::os_spr::DecodeOptions::default()).await.expect("checked-in history");
    assert_eq!((history.edits.len(), history.transitions.len()), (1, 1), "the superseded edit keeps its slot and the supersession is kept, not applied away");
    let head = ::directory::os_store::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&candidate.pair.pack, &candidate.pair.spr).await.expect("the checked-in pair reloads").into_snapshot();
    assert_eq!(head.positions.iter().map(|position| (position.id.clone(), position.data.clone())).collect::<Vec<_>>(), vec![("p".to_string(), ::directory::DslValue::object([("lat".into(), ::directory::DslValue::float(47.0)), ("lon".into(), ::directory::DslValue::float(9.0))]))], "the reload folds the effective input (in the pack's canonical key order)");
}

/// ⛔️ A supersession that makes the ledger's replay block under `Normal` (a withdrawn create leaves the later delete of
/// the same position `mutation.target-missing`, an `Error`) fails Check In with the codec's typed output refusal, the
/// same on every attempt, while the same ledger without it checks in (design §9.2: Error and Fatal block, the check-in
/// strictness).
#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn materialize_check_in_refuses_a_supersession_that_blocks_the_replay() {
    use semio_s_artifact_gis_gismap::mutations::delete_position::DeletePosition;
    let (genesis, ledger) = gis_map_ledger(vec![gis_map_position(0, "p", 7.0), semio_s_artifact_gis_gismap::GisMapMutation::DeletePosition(DeletePosition { id: "p".into() })]).await;
    let authority = gis_map_authority();
    let job = DocumentCheckInJob::new("0a1b2c3d4e5f60718293a4b5c6d7e8f9", "aa");
    let context = OperationContext::stall_bounded(DOCUMENT_CHECK_IN_STALL_BOUND_MS, AuthorityLimits::maximum(), job.as_ref()).unwrap();
    let request = |ledger: &[protocol::MutationEnvelope]| CheckInMaterialization { base_pair: genesis.clone(), envelopes: ::directory::os_spr::encode_envelopes(ledger), ..materialization(b"+0") };
    authority.materialize_check_in(request(&ledger), &context).await.expect("the ledger without the supersession checks in");
    let mut blocked = ledger.clone();
    blocked.push(gis_map_supersede(&ledger, vec![(0, None)]));
    let first = authority.materialize_check_in(request(&blocked), &context).await.expect_err("a supersession that blocks the replay never checks in");
    assert!(matches!(&first, AuthorityError::Codec { stage: ArtifactValidationStage::Output, message } if message.contains("rejected by merge policy Normal")), "the codec refuses the replayed ledger under Normal: {first:?}");
    assert_eq!(check_in_refusal_of_authority_error(&first), Some(DocumentCheckInRefusalV1::CodecRefused));
    let second = authority.materialize_check_in(request(&blocked), &context).await.expect_err("the refusal is deterministic");
    assert_eq!(first, second, "the same ledger is refused the same way every time");
}
//#endregion 🔖️Supersede
