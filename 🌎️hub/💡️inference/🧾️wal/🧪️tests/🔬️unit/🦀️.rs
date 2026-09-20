//! 🔬️ Literal hostile traces exercise the production retained WAL reader, not a second matcher.

#![cfg(feature = "native-artifact-execution")]

use super::*;
use db::storage::WalStorage;

#[path = "../⛓️chain/🦀️.rs"]
mod chain;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json")).unwrap()
}

fn decode_hex(value: &str) -> Vec<u8> {
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).unwrap()).collect()
}

fn durable_edit<Mutation>(ordinal: i32, mutation_id: &str, actor: &str, timestamp: protocol::HybridLogicalTimestamp, forward: Mutation, inverse: Vec<Mutation>) -> directory::os_spr::Edit<Mutation> {
    let id = format!("inference-store-edit-{ordinal}");
    directory::os_spr::Edit {
        id: id.clone(),
        actor: Some(actor.into()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![directory::os_spr::MutationMeta {
            mutation_id: Some(protocol::MutationId(mutation_id.to_owned())),
            dependencies: Vec::new(),
            base_version: ordinal as u64,
            author_id: Some(protocol::ActorId(actor.into())),
            timestamp,
            undo_policy: directory::os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: Some(protocol::SchemaId(format!("inference-store-member-{ordinal}"))),
            label: Some(format!("inference Store member {ordinal}")),
            group_id: None,
            origin: Default::default(),
        }],
        description: Some(format!("inference Store member {ordinal}")),
        coalesce_key: None,
        sequence_number: ordinal,
        started_at: format!("2026-09-06T00:00:0{ordinal}Z"),
        finished_at: Some(format!("2026-09-06T00:00:1{ordinal}Z")),
    }
}

pub(super) struct DurableFixtureRecord {
    pub(super) record: directory::os_store::durable_group::DurableOwnedGroupJournalRecordV1,
    /// 🎭️ A second, fully re-sealed decision that keeps the approval's `mutation_id` while its
    /// envelope payload is another proposal entirely — what `same-id-altered-envelope` has to mean.
    /// Flipping a byte of the canonical pack only corrupts the footer checksum, so the container is
    /// refused before any binding is examined and the trace proved nothing about identity binding.
    pub(super) altered_record: directory::os_store::durable_group::DurableOwnedGroupJournalRecordV1,
    /// 🎭️ The same decision re-sealed under a different actor — a canonical, admissible pack whose
    /// committed author is not the approval's, so the hostile-command law can offer real durable
    /// bytes instead of a bare envelope encoding the replay never even admits as a decision.
    pub(super) wrong_actor_record: directory::os_store::durable_group::DurableOwnedGroupJournalRecordV1,
    pub(super) command: Vec<u8>,
    pub(super) proposal_hash: String,
    pub(super) mutation_id: String,
    pub(super) command_hash: String,
    pub(super) diff_payload: Vec<u8>,
    pub(super) inverse_payload: Vec<u8>,
}

impl DurableFixtureRecord {
    /// 🖨️ The whole regenerated fixture, produced — not hand-derived. `commandHash` covers the two
    /// payload blobs and `encodedHex`, so when the hash moves those move with it and patching the
    /// three digests alone leaves the fixture internally inconsistent. One failing run therefore
    /// prints every literal `🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` owns, in its own key
    /// order, so the regeneration is a copy of producer output rather than an edit.
    pub(super) fn regeneration_readout(&self) -> String {
        format!(
            "\n  proposalHash        {}\n  command.mutationId  {}\n  commandHash         {}\n  encodedHex          {}\n  command.diff.payloadHex     {}\n  command.inverse.payloadHex  {}",
            self.proposal_hash,
            self.mutation_id,
            self.command_hash,
            encode_hex(&self.command),
            encode_hex(&self.diff_payload),
            encode_hex(&self.inverse_payload)
        )
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
        use std::fmt::Write as _;
        let _ = write!(text, "{byte:02x}");
        text
    })
}

pub(super) fn durable_fixture_record(fixture: &serde_json::Value) -> DurableFixtureRecord {
    use directory::Inference as _;
    use semio_s_artifact_gis_gismap::{
        mutations::apply_gis_map_mutation,
        schema::{gis_map_descriptor_json, gis_map_document_from_descriptor_json, gis_map_snapshot_to_drawing},
        standards::v1::subsets::any::schema::inferences::GisMapInference,
    };
    use semio_s_artifact_stdio_semio::standards::v1::subsets::{drawing::schema::mutations::apply_semio_drawing_mutation, value::schema::mutations::apply_semio_value_mutation};

    let base = gis_map_document_from_descriptor_json(r#"{"positions":[{"id":"point-a","lon":7,"lat":47}],"routes":[{"id":"route-a","points":[[8,46],[9,48]]}],"regions":[]}"#);
    let drawing_base = gis_map_snapshot_to_drawing(&base);
    let value_base = semio_s_artifact_gis_gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&base));
    let actor = fixture["command"]["actor"].as_str().unwrap();
    let job_id = fixture["jobId"].as_str().unwrap();
    let document_key = fixture["documentKey"].as_str().unwrap();
    let timestamp = protocol::HybridLogicalTimestamp {
        actor: fixture["command"]["timestamp"]["actor"].as_u64().unwrap(),
        physical_ms: fixture["command"]["timestamp"]["physicalMs"].as_u64().unwrap(),
        logical: fixture["command"]["timestamp"]["logical"].as_u64().unwrap(),
    };
    let sealed = |work_job_id: &str, identity: &str, edit_actor: &str| {
        let work = GisMapInference::infer(&base).create_region_group_work(&base, work_job_id).expect("typed fixed-three inference work");
        let mut parent_post = base.clone();
        apply_gis_map_mutation(&mut parent_post, &work.parent).expect("parent inference mutation applies");
        let mut drawing_post = drawing_base.clone();
        apply_semio_drawing_mutation(&mut drawing_post, &work.drawing);
        let mut value_post = value_base.clone();
        apply_semio_value_mutation(&mut value_post, &work.value);
        let proposal = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
        let inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
        let record = directory::os_store::durable_group::durable_owned_group_journal_test_record_from_edits(
            directory::os_io::ArtifactRef { artifact_id: document_key.into(), dialect: directory::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } },
            durable_edit(1, identity, edit_actor, timestamp, work.parent, work.parent_inverse),
            parent_post,
            durable_edit(2, &format!("{identity}:drawing"), edit_actor, protocol::HybridLogicalTimestamp { logical: 1, ..timestamp }, work.drawing, work.drawing_inverse),
            drawing_post,
            durable_edit(3, &format!("{identity}:value"), edit_actor, protocol::HybridLogicalTimestamp { logical: 2, ..timestamp }, work.value, work.value_inverse),
            value_post,
        )
        .expect("Store-owned typed fixed-three decision fixture");
        (record, proposal, inverse)
    };
    let canonical_proposal = directory::os_pack::json::to_json_string(&GisMapInference::infer(&base).create_region_group_work(&base, job_id).expect("typed fixed-three inference work").parent).into_bytes();
    let proposal_hash = crate::inference::sha256(&canonical_proposal);
    let mutation_id = crate::inference::sha256(format!("semio.hub.inference-approval-mutation/v1\0{job_id}\0{proposal_hash}").as_bytes())[..32].to_string();
    let (record, proposal, inverse) = sealed(job_id, &mutation_id, actor);
    let altered_job_id = fixture["bindingMismatches"][0]["jobId"].as_str().unwrap();
    assert_ne!(altered_job_id, job_id, "the altered-envelope decision must propose something else entirely");
    let (altered_record, _, _) = sealed(altered_job_id, &mutation_id, actor);
    let wrong_actor = actor.replacen('a', "e", 1);
    assert_ne!(wrong_actor, actor, "the wrong-actor decision must be committed by somebody else");
    let (wrong_actor_record, _, _) = sealed(job_id, &mutation_id, &wrong_actor);
    let command = super::super::command::encode_server_stamped_command_v1(&super::super::command::CanonicalInferenceCommandPartsV1 {
        mutation_id: &mutation_id,
        document_id: fixture["documentKey"].as_str().unwrap(),
        actor,
        diff_schema: super::super::schema::GIS_DOCUMENT_SCHEMA,
        diff_payload: &proposal,
        inverse_schema: super::super::schema::GIS_DOCUMENT_SCHEMA,
        inverse_payload: &inverse,
        timestamp,
    })
    .expect("canonical command reconstructed from Store parent edit");
    let command_hash = crate::inference::sha256(&command);
    DurableFixtureRecord { record, altered_record, wrong_actor_record, command, proposal_hash, mutation_id, command_hash, diff_payload: proposal, inverse_payload: inverse }
}

fn scope(fixture: &serde_json::Value) -> DocumentScope {
    DocumentScope::new(fixture["scope"]["spaceId"].as_str().unwrap(), fixture["scope"]["documentId"].as_str().unwrap())
}

pub(in crate::inference) fn envelope(fixture: &serde_json::Value) -> protocol::MutationEnvelope {
    let source = &fixture["command"];
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(source["mutationId"].as_str().unwrap().into()),
        document_id: protocol::ArtifactId(source["documentId"].as_str().unwrap().into()),
        actor: protocol::ActorId(source["actor"].as_str().unwrap().into()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(source["diff"]["schema"].as_str().unwrap().into()), payload: decode_hex(source["diff"]["payloadHex"].as_str().unwrap()) },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(source["inverse"]["schema"].as_str().unwrap().into()), payload: decode_hex(source["inverse"]["payloadHex"].as_str().unwrap()) },
        timestamp: protocol::HybridLogicalTimestamp { actor: source["timestamp"]["actor"].as_u64().unwrap(), physical_ms: source["timestamp"]["physicalMs"].as_u64().unwrap(), logical: source["timestamp"]["logical"].as_u64().unwrap() },
    }
}

fn target(fixture: &serde_json::Value, trace: &serde_json::Value, durable: &DurableFixtureRecord) -> InferenceWalTargetV1 {
    let mut scope = scope(fixture);
    if let Some(space_id) = trace["spaceId"].as_str() {
        scope.space_id = space_id.into();
    }
    // 🧾️ The receipt names the decision the log actually committed, which for the altered-envelope
    // trace is the re-sealed one. The target still names the approval's own proposal and command, so
    // the replay finds an admissible decision at its receipt that does not bind the approval —
    // "absent", the answer the trace asserts — instead of a corrupt container.
    let altered = trace["records"].as_array().is_some_and(|records| records.iter().any(|record| record["bytes"] == "altered-target"));
    let committed = if altered { &durable.altered_record } else { &durable.record };
    InferenceWalTargetV1 {
        scope,
        generation: fixture["generation"].as_u64().unwrap(),
        job_id: fixture["jobId"].as_str().unwrap().into(),
        proposal_hash: durable.proposal_hash.clone(),
        mutation_id: durable.mutation_id.clone(),
        command_hash: durable.command_hash.clone(),
        actor: fixture["command"]["actor"].as_str().unwrap().into(),
        maximum_records: trace["maximumRecords"].as_u64().unwrap_or(fixture["maximumRecords"].as_u64().unwrap()),
        receipt: directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1 {
            anchor_sha256: committed.anchor_sha256().to_string(),
            decision_sha256: committed.decision_sha256().to_string(),
            transaction_id: trace["receiptTransactionId"].as_u64().unwrap_or(1),
            segment_index: 0,
        },
    }
}

async fn storage(fixture: &serde_json::Value, trace: &serde_json::Value, durable: &DurableFixtureRecord) -> Arc<db::storage::DbBackend> {
    storage_with_event(fixture, trace, durable, None).await
}

async fn storage_with_event(fixture: &serde_json::Value, trace: &serde_json::Value, durable: &DurableFixtureRecord, replacement: Option<&[u8]>) -> Arc<db::storage::DbBackend> {
    let config = db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2);
    let pool = Arc::new(db::semio_framework_async::process_worker_pool(config));
    let storage = db::storage::MemoryStorage::new(pool).await.unwrap();
    let document = db::ArtifactId(fixture["documentKey"].as_str().unwrap().into());
    let permit = storage.acquire_writer(&document).await.unwrap();
    storage.create_segment(&permit, 0).await.unwrap();
    let mut writer = protocol::SprWriter::begin(Vec::<u8>::new(), &protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 }).await.unwrap();
    let mut header = Vec::new();
    protocol::write_str(&mut header, &document.0);
    header.extend_from_slice(&0u64.to_le_bytes());
    header.push(0);
    writer.write_record(db::wal::WAL_SEGMENT_HEADER, true, &header, protocol::codec::ids::CodecId(0)).await.unwrap();
    writer.commit().await.unwrap();
    if trace["flushed"] != false {
        for record in trace["records"].as_array().unwrap() {
            let kind = record["kind"].as_str().unwrap();
            let (tag, bytes) = match kind {
                "begin" => (db::wal::WAL_TX_BEGIN, record["txId"].as_u64().unwrap().to_le_bytes().to_vec()),
                "abort" => (db::wal::WAL_TX_ABORT, record["txId"].as_u64().unwrap().to_le_bytes().to_vec()),
                "commit" => {
                    let mut bytes = record["txId"].as_u64().unwrap().to_le_bytes().to_vec();
                    bytes.extend_from_slice(&(record["recordCount"].as_u64().unwrap() as u32).to_le_bytes());
                    (db::wal::WAL_TX_COMMIT, bytes)
                }
                "command" => {
                    let mut bytes = durable.record.canonical_pack().to_vec();
                    match record["bytes"].as_str().unwrap() {
                        "different" => bytes = b"different non-Pack durable event".to_vec(),
                        "altered-target" => bytes = durable.altered_record.canonical_pack().to_vec(),
                        "target" => {
                            if let Some(replacement) = replacement {
                                bytes = replacement.to_vec();
                            }
                        }
                        _ => panic!("unrecognized literal durable event"),
                    }
                    (db::wal::WAL_EVENT, bytes)
                }
                _ => panic!("unrecognized literal WAL record"),
            };
            writer.write_record(tag, true, &bytes, protocol::codec::ids::CodecId(0)).await.unwrap();
        }
        writer.commit().await.unwrap();
    }
    let mut bytes = writer.into_sink().await;
    if trace["tornTail"] == true {
        bytes.pop();
    }
    let mut pages = db::storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db::storage::DB_IO_PAGE_BYTES)).unwrap();
    for fragment in bytes.chunks(db::storage::DB_IO_PAGE_BYTES) {
        assert_eq!(pages.write_fragment(fragment).unwrap(), fragment.len());
    }
    storage.append(&permit, 0, pages.seal_retained().await.unwrap()).await.unwrap();
    storage.sync(&permit, 0, db::DurabilityClass::Fsync).await.unwrap();
    permit.release().await.unwrap();
    Arc::new(db::storage::DbBackend::Memory(storage))
}

#[tokio::test]
async fn inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces() {
    let fixture = fixture();
    let durable = durable_fixture_record(&fixture);
    let mut bytes = Vec::new();
    protocol::encode_envelope(&envelope(&fixture), &mut bytes);
    assert_eq!(bytes, decode_hex(fixture["encodedHex"].as_str().unwrap()));
    assert_eq!(crate::inference::sha256(&bytes), fixture["commandHash"].as_str().unwrap());
    let identifiers: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🖥️inference-server-identity-v1/🔣️.json")).unwrap();
    for row in identifiers["cases"].as_array().unwrap() {
        for field in ["userId", "sessionId", "spaceId", "documentId"] {
            let mut candidate = target(&fixture, &fixture["traces"][0], &durable);
            let value = row["value"].as_str().unwrap();
            match field {
                "userId" => candidate.actor = format!("user:{value}#session:valid"),
                "sessionId" => candidate.actor = format!("user:valid#session:{value}"),
                "spaceId" => candidate.scope.space_id = value.into(),
                "documentId" => candidate.scope.document_id = value.into(),
                _ => unreachable!(),
            }
            let accepted = InferenceDocumentFenceV1::new(candidate.scope.clone(), candidate.generation).and_then(|fence| candidate.validate(&fence)).is_ok();
            assert_eq!(accepted, row["accepted"].as_bool().unwrap(), "WAL {}/{}", row["name"], field);
        }
    }
    for trace in fixture["traces"].as_array().unwrap() {
        let storage = tokio::time::timeout(Duration::from_secs(2), storage(&fixture, trace, &durable)).await.unwrap_or_else(|_| panic!("WAL trace {} timed out during storage admission", trace["name"]));
        let verifier = InferenceWalVerifierV1::new(storage);
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), fixture["generation"].as_u64().unwrap()).unwrap());
        let control = Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap());
        if trace["cancelAfterRecords"] == 0 {
            control.cancel();
        }
        let cancellation = trace["cancelAfterRecords"].as_u64().filter(|value| *value > 0);
        let invalidate = trace["observedGeneration"].as_u64().is_some();
        let watcher = if cancellation.is_some() || invalidate {
            let control = control.clone();
            let fence = fence.clone();
            Some(tokio::spawn(async move {
                let threshold = cancellation.unwrap_or(3);
                tokio::time::timeout(Duration::from_secs(2), async {
                    while control.progress().0 < threshold {
                        tokio::task::yield_now().await;
                    }
                })
                .await
                .unwrap();
                if invalidate {
                    fence.invalidate();
                } else {
                    control.cancel();
                }
            }))
        } else {
            None
        };
        let result = tokio::time::timeout(Duration::from_secs(4), verifier.verify(target(&fixture, trace, &durable), fence.clone(), control)).await.unwrap_or_else(|_| panic!("WAL trace {} timed out during retained verification", trace["name"]));
        if let Some(watcher) = watcher {
            watcher.await.unwrap();
        }
        let outcome = match &result {
            Ok(Some(witness)) => {
                assert!(
                    witness.matches(
                        &scope(&fixture),
                        fixture["generation"].as_u64().unwrap(),
                        fixture["jobId"].as_str().unwrap(),
                        fixture["proposalHash"].as_str().unwrap(),
                        fixture["command"]["mutationId"].as_str().unwrap(),
                        fixture["commandHash"].as_str().unwrap()
                    ),
                    "frozen fixture literals disagree with the producer — proposalHash {} vs {}, command.mutationId {} vs {}, commandHash {} vs {}. Regenerate the fixture from this producer read-out, never by hand:{}",
                    fixture["proposalHash"],
                    durable.proposal_hash,
                    fixture["command"]["mutationId"],
                    durable.mutation_id,
                    fixture["commandHash"],
                    durable.command_hash,
                    durable.regeneration_readout(),
                );
                for mismatch in fixture["bindingMismatches"].as_array().unwrap() {
                    assert!(!witness.matches(
                        &scope(&fixture),
                        fixture["generation"].as_u64().unwrap(),
                        mismatch["jobId"].as_str().unwrap(),
                        mismatch["proposalHash"].as_str().unwrap(),
                        fixture["command"]["mutationId"].as_str().unwrap(),
                        fixture["commandHash"].as_str().unwrap()
                    ));
                }
                "verified"
            }
            Ok(None) => "absent",
            Err(InferenceErrorV1::Conflict) => "stale",
            Err(InferenceErrorV1::Cancelled) => "cancelled",
            Err(InferenceErrorV1::Bounds) => "bounds",
            Err(InferenceErrorV1::Invalid | InferenceErrorV1::Storage) => "invalid",
            Err(other) => panic!("unexpected WAL outcome {other:?}"),
        };
        assert_eq!(outcome, trace["expected"].as_str().unwrap(), "{}", trace["name"]);
        if let Some(reusable) = trace["reusableAfterInvalidation"].as_bool() {
            fence.invalidate();
            assert_eq!(
                result.as_ref().unwrap().as_ref().unwrap().matches(
                    &scope(&fixture),
                    fixture["generation"].as_u64().unwrap(),
                    fixture["jobId"].as_str().unwrap(),
                    fixture["proposalHash"].as_str().unwrap(),
                    fixture["command"]["mutationId"].as_str().unwrap(),
                    fixture["commandHash"].as_str().unwrap()
                ),
                reusable
            );
        }
        tokio::time::timeout(Duration::from_secs(2), async {
            while verifier.active() != 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(verifier.active(), 0, "retained replay closes before its slot is released");
        if trace["spaceId"].is_null() && trace["cancelAfterRecords"] != 0 {
            assert!(verifier.close_steps() > 0, "{}", trace["name"]);
        }
    }
}

pub(in crate::inference) async fn committed_fixture_witness() -> (CommittedInferenceWalWitnessV1, Arc<InferenceDocumentFenceV1>) {
    let fixture = fixture();
    let durable = durable_fixture_record(&fixture);
    let trace = &fixture["traces"][0];
    let verifier = InferenceWalVerifierV1::new(storage(&fixture, trace, &durable).await);
    let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), fixture["generation"].as_u64().unwrap()).unwrap());
    let witness = verifier.verify(target(&fixture, trace, &durable), fence.clone(), Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap())).await.unwrap().unwrap();
    assert_eq!(verifier.active(), 0);
    (witness, fence)
}

#[tokio::test]
async fn inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands() {
    let fixture = fixture();
    let durable = durable_fixture_record(&fixture);
    let commands: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/✉️inference-command-v1/🔣️.json")).unwrap();
    let trace = &fixture["traces"][0];
    let mut selected = 0;
    for vector in commands["vectors"].as_array().unwrap() {
        let change = vector["change"].as_str().unwrap();
        if !["trailing", "overlong-varint", "different-actor"].contains(&change) {
            continue;
        }
        // 🎭️ The hostile bytes are a DURABLE DECISION PACK, not a bare command envelope. An event in
        // the receipt transaction that is not a pack is exactly the `missing-command` trace, whose
        // frozen answer is "absent" — so a bare envelope could never prove this law's claim. What the
        // law does claim is that matching `command_hash` against the committed bytes buys nothing:
        // the canonical container and the receipt's own decision hash still refuse.
        let mut bytes = match change {
            "different-actor" => durable.wrong_actor_record.canonical_pack().to_vec(),
            _ => durable.record.canonical_pack().to_vec(),
        };
        if change == "trailing" {
            bytes.push(0);
        }
        if change == "overlong-varint" {
            let offset = directory::os_store::semio_format::BINARY_MAGIC.len();
            bytes[offset] |= 128;
            bytes.insert(offset + 1, 0);
        }
        let mut target = target(&fixture, trace, &durable);
        target.command_hash = crate::inference::sha256(&bytes);
        let backend = tokio::time::timeout(Duration::from_secs(2), storage_with_event(&fixture, trace, &durable, Some(&bytes))).await.expect("bounded committed hostile storage");
        let verifier = InferenceWalVerifierV1::new(backend);
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), 17).unwrap());
        let result = tokio::time::timeout(Duration::from_secs(4), verifier.verify(target, fence, Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap()))).await.expect("bounded committed hostile verification");
        assert!(matches!(result, Err(InferenceErrorV1::Invalid)), "a matching durable hash cannot bypass {}", vector["name"]);
        assert_eq!(verifier.active(), 0, "rejected bytes retire before admission returns");
        selected += 1;
    }
    assert_eq!(selected, 3);
}

mod quick {
    use super::*;

    #[tokio::test]
    async fn inference_wal_proof_dropped_caller_cancels_and_finishes_retained_replay_before_release() {
        let fixture = fixture();
        let durable = durable_fixture_record(&fixture);
        let trace = &fixture["traces"][0];
        for owner in fixture["ownership"].as_array().unwrap() {
            let mut verifier = InferenceWalVerifierV1::new(storage(&fixture, trace, &durable).await);
            let gate = Arc::new(tokio::sync::Semaphore::new(0));
            Arc::get_mut(&mut verifier.state).unwrap().replay_gate = Some(gate.clone());
            let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), 17).unwrap());
            let control = Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap());
            let mut future = Box::pin(verifier.verify(target(&fixture, trace, &durable), fence, control.clone()));
            assert!(futures::poll!(future.as_mut()).is_pending());
            tokio::time::timeout(Duration::from_secs(2), async {
                while control.progress().0 == 0 {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
            let expected = if owner["expected"] == "expired" { InferenceErrorV1::Expired } else { InferenceErrorV1::Cancelled };
            match owner["interrupt"].as_str().unwrap() {
                "drop" => drop(future),
                "cancel" => {
                    control.cancel();
                    assert!(matches!(tokio::time::timeout(Duration::from_secs(1), future).await.unwrap(), Err(error) if error == expected));
                }
                "deadline" => {
                    assert!(matches!(tokio::time::timeout(Duration::from_secs(3), future).await.unwrap(), Err(error) if error == expected));
                }
                _ => panic!("unrecognized literal ownership interrupt"),
            }
            assert_eq!(control.checkpoint(control.progress().0), Err(expected));
            assert_eq!(verifier.active() as u64, owner["heldActive"].as_u64().unwrap());
            assert_eq!(verifier.close_steps(), 0, "blocked replay still owns its admitted slot and pages");
            gate.add_permits(1);
            tokio::time::timeout(Duration::from_secs(2), async {
                while verifier.active() != 0 {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
            assert_eq!(verifier.active() as u64, owner["releasedActive"].as_u64().unwrap());
            assert_eq!(control.progress().0, owner["stoppedProgress"].as_u64().unwrap());
            assert!(verifier.close_steps() > 0, "slot release follows actual replay close");
        }
    }
}
