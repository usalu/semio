use super::*;
use std::sync::{Arc, Barrier};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).unwrap()
}

fn selected(fixture: &serde_json::Value) -> InferenceIdentityV1 {
    serde_json::from_value(fixture["identity"].clone()).unwrap()
}

fn memory() -> InferenceJobLedgerV1 {
    let connection = Connection::open_in_memory().unwrap();
    connection.execute_batch(SCHEMA).unwrap();
    InferenceJobLedgerV1 { connection: Mutex::new(connection) }
}

fn reader(identity: &InferenceIdentityV1) -> InferenceReaderV1<'_> {
    InferenceReaderV1 { user_id: &identity.user_id, session_id: &identity.session_id, authorization_generation: identity.authorization_generation, space_id: &identity.space_id, document_id: &identity.document_id }
}

#[test]
fn gis_inference_sqlite_ledger_executes_neutral_traces_with_private_first_terminal_wins() {
    let fixture = fixture();
    let selected = selected(&fixture);
    assert_eq!(selected.digest().unwrap(), fixture["identityDigest"].as_str().unwrap());
    let identifiers: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🖥️inference-server-identity-v1/🔣️.json")).unwrap();
    for row in identifiers["cases"].as_array().unwrap() {
        for field in identifiers["fields"].as_array().unwrap() {
            let mut candidate = fixture["identity"].clone();
            candidate["headOrdinal"] = 1.into();
            candidate["headEditId"] = "0".repeat(32).into();
            candidate[field.as_str().unwrap()] = row["value"].clone();
            let value: InferenceIdentityV1 = serde_json::from_value(candidate).unwrap();
            assert_eq!(value.validate().is_ok(), row["accepted"].as_bool().unwrap(), "{}/{}", row["name"], field);
        }
    }
    let maximum_id = "a".repeat(identifiers["maximumBytes"].as_u64().unwrap() as usize);
    assert!(format!("v1:{}:{}:{}{}", maximum_id.len(), maximum_id.len(), maximum_id, maximum_id).len() <= 256);
    assert!(format!("user:{maximum_id}#session:{maximum_id}").len() <= 256);
    for hostile in fixture["hostileIdentities"].as_array().unwrap() {
        let mut candidate = fixture["identity"].clone();
        let path = hostile["path"].as_array().unwrap();
        let mut at = &mut candidate;
        for segment in &path[..path.len() - 1] {
            at = &mut at[segment.as_str().unwrap()];
        }
        at[path.last().unwrap().as_str().unwrap()] = hostile["value"].clone();
        if let Ok(identity) = serde_json::from_value::<InferenceIdentityV1>(candidate) {
            assert!(identity.validate().is_err(), "{}", hostile["name"]);
        }
    }
    let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
    let result = InferencePrivateBytesV1::new(serde_json::to_vec(&fixture["expectedInference"]).unwrap(), RESULT_MAX_BYTES).unwrap();
    let proposal = InferencePrivateBytesV1::new(b"bounded-ledger-payload-not-an-executable-proposal".to_vec(), PROPOSAL_MAX_BYTES).unwrap();
    for trace in fixture["traces"].as_array().unwrap() {
        let ledger = memory();
        let receipt = ledger.accept(&selected, &input, 1000).unwrap();
        let mut epoch = 0;
        for (step, operation) in trace["operations"].as_array().unwrap().iter().enumerate() {
            let at = 1001 + step as u64;
            match operation.as_str().unwrap() {
                "start" => {
                    if let Some(claim) = ledger.start(&receipt.job_id, &selected, at).unwrap() {
                        epoch = claim.run_epoch;
                    }
                }
                "succeed" => {
                    ledger.succeed(&receipt.job_id, &selected, epoch, &result, &proposal, at).unwrap();
                }
                "cancel" => {
                    ledger.cancel(&receipt.job_id, &reader(&selected), at).unwrap();
                }
                _ => panic!("unknown literal trace operation"),
            }
        }
        let view = ledger.read(&receipt.job_id, &reader(&selected), 1010).unwrap();
        assert_eq!(serde_json::to_value(view.state).unwrap(), trace["state"], "{}", trace["name"]);
        assert_eq!(serde_json::to_value(view.proposal_state).unwrap(), trace["proposalState"], "{}", trace["name"]);
        assert_eq!(!view.result.as_slice().is_empty(), trace["hasResult"].as_bool().unwrap());
        let count = ledger.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1", [], |row| read_integer(row, 0)).unwrap();
        assert_eq!(count, trace["eventCount"].as_u64().unwrap());
        let mut foreign = reader(&selected);
        foreign.user_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
        assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1004), Err(InferenceErrorV1::Denied)));
        assert_eq!(ledger.cancel(&receipt.job_id, &foreign, 1004), Err(InferenceErrorV1::Denied));
        foreign = reader(&selected);
        foreign.space_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
        assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1010), Err(InferenceErrorV1::Denied)));
        foreign = reader(&selected);
        foreign.authorization_generation += 1;
        assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1010), Err(InferenceErrorV1::Denied)));
        assert!(ledger.connection.lock().unwrap().execute("UPDATE inference_job_event_v1 SET kind='failed'", []).is_err());
    }
}

#[test]
fn gis_inference_sqlite_request_identity_capacity_expiry_and_progress_are_bounded() {
    let fixture = fixture();
    let selected = selected(&fixture);
    let integers = Connection::open_in_memory().unwrap();
    for case in fixture["sqliteIntegers"].as_array().unwrap() {
        let decimal = case["decimal"].as_str().unwrap();
        let expected = case["accepted"].as_bool().unwrap();
        let parsed = decimal.parse::<u64>();
        assert_eq!(parsed.ok().and_then(|value| sql_integer(value).ok()).is_some(), expected, "write {decimal}");
        if let Ok(signed) = decimal.parse::<i64>() {
            let read = integers.query_row("SELECT ?1", [signed], |row| read_integer(row, 0));
            assert_eq!(read.is_ok(), expected, "read {decimal}");
            if expected {
                assert_eq!(read.unwrap().to_string(), decimal);
            }
        }
    }
    assert!(InferenceRequestV1::decode(&serde_json::to_vec(&selected.request).unwrap()).is_ok());
    for hostile in fixture["hostileRequests"].as_array().unwrap() {
        let mut candidate = fixture["identity"]["request"].clone();
        candidate[hostile["field"].as_str().unwrap()] = hostile["value"].clone();
        assert!(InferenceRequestV1::decode(&serde_json::to_vec(&candidate).unwrap()).is_err(), "{}", hostile["name"]);
    }
    let duplicate = serde_json::to_string(&selected.request).unwrap().replacen("{", "{\"version\":1,", 1);
    assert_eq!(InferenceRequestV1::decode(duplicate.as_bytes()), Err(InferenceErrorV1::Invalid));
    assert_eq!(InferenceRequestV1::decode(&vec![b' '; REQUEST_MAX_BYTES + 1]), Err(InferenceErrorV1::Bounds));
    let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
    let ledger = memory();
    assert_eq!(ledger.accept(&selected, &input, SAFE_INTEGER_MAX), Err(InferenceErrorV1::Bounds));
    assert_eq!(ledger.accept(&selected, &input, u64::MAX), Err(InferenceErrorV1::Bounds));
    let receipt = ledger.accept(&selected, &input, 1000).unwrap();
    assert_eq!(ledger.accept(&selected, &input, 2000).unwrap(), receipt);
    let mut changed = selected.clone();
    changed.binding.catalog_generation_id = "7".repeat(64);
    assert_eq!(ledger.accept(&changed, &input, 1001), Err(InferenceErrorV1::Conflict));
    assert_eq!(ledger.start(&receipt.job_id, &changed, 1001), Err(InferenceErrorV1::Conflict));
    assert_eq!(ledger.read(&receipt.job_id, &reader(&selected), 1002).unwrap().state, InferenceJobStateV1::Cancelled);
    for index in 1..JOB_CAPACITY {
        let mut next = selected.clone();
        next.request.request_id = format!("{index:032x}");
        ledger.accept(&next, &input, 1000).unwrap();
    }
    let mut extra = selected.clone();
    extra.request.request_id = "ffffffffffffffffffffffffffffffff".into();
    assert_eq!(ledger.accept(&extra, &input, 1000), Err(InferenceErrorV1::Capacity));
    let short = memory();
    let receipt = short.accept(&selected, &input, 1000).unwrap();
    assert_eq!(short.start(&receipt.job_id, &selected, receipt.expires_at_ms), Err(InferenceErrorV1::Expired));
    assert_eq!(short.start(&receipt.job_id, &selected, receipt.expires_at_ms), Ok(None));
    let failed = memory();
    let receipt = failed.accept(&selected, &input, 1000).unwrap();
    assert!(failed.fail(&receipt.job_id, &reader(&selected), 1001).unwrap());
    assert!(!failed.cancel(&receipt.job_id, &reader(&selected), 1002).unwrap());
    assert_eq!(failed.read(&receipt.job_id, &reader(&selected), 1003).unwrap().state, InferenceJobStateV1::Failed);
    assert!(matches!(InferencePrivateBytesV1::new(vec![0; INPUT_MAX_BYTES + 1], INPUT_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
    assert!(matches!(InferencePrivateBytesV1::new(vec![0; RESULT_MAX_BYTES + 1], RESULT_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
    assert!(matches!(InferencePrivateBytesV1::new(vec![0; PROPOSAL_MAX_BYTES + 1], PROPOSAL_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
    let control = crate::inference::InferenceOperationControlV1::new(1000, 2).unwrap();
    control.checkpoint(2).unwrap();
    control.checkpoint(1).unwrap();
    assert_eq!(control.progress(), (2, 2));
    assert_eq!(control.checkpoint(3), Err(InferenceErrorV1::Bounds));
    control.cancel();
    assert_eq!(control.checkpoint(2), Err(InferenceErrorV1::Cancelled));
}

#[test]
fn inference_request_reconciliation_is_existing_only_reader_bound_and_expiry_independent() {
    let fixture = fixture();
    let reconcile: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧭️inference-job-reconcile-v1/🔣️.json")).unwrap();
    let request = &reconcile["request"];
    assert!(InferenceJobReconcileRequestV1::decode(&serde_json::to_vec(request).unwrap()).is_ok());
    for row in reconcile["requestCases"].as_array().unwrap() {
        let mut candidate = request.clone();
        if let Some(field) = row["path"].as_array().unwrap().first() {
            candidate[field.as_str().unwrap()] = row["value"].clone();
        }
        assert_eq!(InferenceJobReconcileRequestV1::decode(&serde_json::to_vec(&candidate).unwrap()).is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
    }
    let mut boundary = serde_json::to_vec(request).unwrap();
    boundary.resize(RECONCILE_REQUEST_MAX_BYTES, b' ');
    assert!(InferenceJobReconcileRequestV1::decode(&boundary).is_ok());
    boundary.push(b' ');
    assert_eq!(InferenceJobReconcileRequestV1::decode(&boundary), Err(InferenceErrorV1::Bounds));

    let selected = selected(&fixture);
    let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
    let ledger = memory();
    let accepted = ledger.accept(&selected, &input, 1000).unwrap();
    let live = ledger.reconcile_request(&selected.request.request_id, &reader(&selected), 1001).unwrap().unwrap();
    assert_eq!(live.receipt.job_id, accepted.job_id);
    assert_eq!(live.receipt.state, InferenceJobStateV1::Accepted);
    assert!(!live.page.expired);
    assert!(live.approval.is_none());
    let live_result = InferenceJobReconcileResultV1 { schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, request_id: selected.request.request_id.clone(), found: true, job: Some(live) };
    let expected_live = reconcile["results"].as_array().unwrap().iter().find(|row| row["name"] == "accepted-live").unwrap();
    assert_eq!(serde_json::to_value(live_result).unwrap(), expected_live["value"]);

    let session = InferenceReaderV1 { session_id: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", ..reader(&selected) };
    assert!(matches!(ledger.reconcile_request(&selected.request.request_id, &session, 1001), Err(InferenceErrorV1::Denied)));
    let user = InferenceReaderV1 { user_id: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", ..reader(&selected) };
    assert!(ledger.reconcile_request(&selected.request.request_id, &user, 1001).unwrap().is_none());
    assert!(ledger.reconcile_request(&"22".repeat(16), &reader(&selected), 1001).unwrap().is_none());
    assert_eq!(ledger.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_v1", [], |row| read_integer(row, 0)).unwrap(), 1);

    assert!(ledger.request_cancel(&accepted.job_id, &reader(&selected), accepted.expires_at_ms).unwrap());
    assert!(matches!(ledger.events(&accepted.job_id, &reader(&selected), 0, accepted.expires_at_ms), Err(InferenceErrorV1::Expired)));
    let terminal = ledger.reconcile_request(&selected.request.request_id, &reader(&selected), accepted.expires_at_ms).unwrap().unwrap();
    assert_eq!(terminal.receipt.state, InferenceJobStateV1::Cancelled);
    assert!(terminal.page.expired);
    let terminal_result = InferenceJobReconcileResultV1 { schema: "semio.hub.inference-job-reconcile-result/v1", version: 1, request_id: selected.request.request_id.clone(), found: true, job: Some(terminal) };
    let expected_terminal = reconcile["results"].as_array().unwrap().iter().find(|row| row["name"] == "cancelled-expired").unwrap();
    assert_eq!(serde_json::to_value(terminal_result).unwrap(), expected_terminal["value"]);
}

#[test]
fn gis_inference_sqlite_concurrent_connections_have_one_durable_request_winner() {
    let fixture = fixture();
    let selected = selected(&fixture);
    let path = std::env::temp_dir().join(format!("semio-gis-ledger-{}.sqlite", directory::os_identity::time_ordered_id()));
    let first = InferenceJobLedgerV1::open(&path).unwrap();
    let second = InferenceJobLedgerV1::open(&path).unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let input = fixture["input"].as_str().unwrap().as_bytes().to_vec();
    let receipts = std::thread::scope(|scope| {
        let submit = |ledger: InferenceJobLedgerV1, barrier: Arc<Barrier>| {
            let selected = &selected;
            let input = &input;
            scope.spawn(move || {
                barrier.wait();
                ledger.accept(selected, &InferencePrivateBytesV1::new(input.clone(), INPUT_MAX_BYTES).unwrap(), 1000).unwrap()
            })
        };
        let a = submit(first, barrier.clone());
        let b = submit(second, barrier);
        (a.join().unwrap(), b.join().unwrap())
    });
    assert_eq!(receipts.0, receipts.1);
    let reopened = InferenceJobLedgerV1::open(&path).unwrap();
    let count = reopened.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1 WHERE kind='accepted'", [], |row| read_integer(row, 0)).unwrap();
    assert_eq!(count, 1);
    assert_eq!(reopened.read(&receipts.0.job_id, &reader(&selected), 1001).unwrap().state, InferenceJobStateV1::Accepted);
    drop(reopened);
    std::fs::remove_file(&path).unwrap();
}

#[tokio::test]
async fn gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once() {
    let fixture = fixture();
    let selected = selected(&fixture);
    let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
    let result = InferencePrivateBytesV1::new(serde_json::to_vec(&fixture["expectedInference"]).unwrap(), RESULT_MAX_BYTES).unwrap();
    let outbox = &fixture["outbox"];
    let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().unwrap().as_bytes().to_vec(), PROPOSAL_MAX_BYTES).unwrap();
    let command_hex = outbox["commandHex"].as_str().unwrap();
    let command = InferencePrivateBytesV1::new((0..command_hex.len()).step_by(2).map(|index| u8::from_str_radix(&command_hex[index..index + 2], 16).unwrap()).collect(), 8192).unwrap();
    let path = std::env::temp_dir().join(format!("semio-gis-outbox-{}.sqlite", directory::os_identity::time_ordered_id()));
    let ledger = InferenceJobLedgerV1::open(&path).unwrap();
    let receipt = ledger.accept(&selected, &input, 1000).unwrap();
    let claim = ledger.start(&receipt.job_id, &selected, 1001).unwrap().expect("owned run claim");
    ledger.succeed(&receipt.job_id, &selected, claim.run_epoch, &result, &proposal, 1002).unwrap();
    let hash = sha256(proposal.as_slice());
    let mut trailing = command.as_slice().to_vec();
    trailing.push(0);
    let trailing = InferencePrivateBytesV1::new(trailing, 8192).unwrap();
    assert!(matches!(ledger.prepare_approval(&receipt.job_id, &selected, &hash, &trailing, 1003), Err(InferenceErrorV1::Invalid)));
    let empty = ledger.pending_approvals(None, &super::super::InferenceOperationControlV1::new(1000, 5).unwrap()).unwrap();
    assert!(empty.rows.is_empty(), "prefix-valid trailing bytes never create a prepared outbox row");
    let prepared = ledger.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1003).unwrap();
    assert_eq!(prepared.job_id, outbox["jobId"].as_str().unwrap());
    assert_eq!(prepared.mutation_id, outbox["mutationId"].as_str().unwrap());
    assert_eq!(prepared.command_hash, outbox["commandHash"].as_str().unwrap());
    assert_eq!(prepared.proposal_hash, outbox["proposalHash"].as_str().unwrap());
    let repeated = ledger.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1004).unwrap();
    assert_eq!(prepared.mutation_id, repeated.mutation_id);
    assert_eq!(prepared.prepared_at_ms, repeated.prepared_at_ms);
    assert_eq!(ledger.cancel(&receipt.job_id, &reader(&selected), 1005), Err(InferenceErrorV1::Conflict));
    let foreign = InferenceReaderV1 { authorization_generation: selected.authorization_generation + 1, ..reader(&selected) };
    assert_eq!(ledger.abandon_prepared_approval(&foreign, &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash), Err(InferenceErrorV1::Denied),);
    assert_eq!(ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &"0".repeat(64), &prepared.proposal_hash), Err(InferenceErrorV1::Conflict),);
    assert!(ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).unwrap());
    assert!(!ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).unwrap());
    assert!(ledger.pending_approvals(None, &super::super::InferenceOperationControlV1::new(1000, 5).unwrap()).unwrap().rows.is_empty());
    drop(ledger);
    let reopened = InferenceJobLedgerV1::open(&path).unwrap();
    let control = super::super::InferenceOperationControlV1::new(1000, 5).unwrap();
    assert!(reopened.pending_approvals(None, &control).unwrap().rows.is_empty(), "an abandoned request survives restart without blocking the document");
    let revived = reopened.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1006).unwrap();
    assert_eq!((revived.mutation_id.as_str(), revived.command_hash.as_str(), revived.proposal_hash.as_str()), (prepared.mutation_id.as_str(), prepared.command_hash.as_str(), prepared.proposal_hash.as_str()));
    assert_eq!(revived.prepared_at_ms, 1006);
    let pending = reopened.pending_approvals(None, &control).unwrap();
    assert_eq!(pending.rows.len() as u64, outbox["preparedCount"].as_u64().unwrap());
    assert_eq!(pending.rows[0].mutation_id, prepared.mutation_id);
    assert_eq!(pending.rows[0].command.as_slice(), command.as_slice());
    assert!(pending.next_cursor.is_none());
    let prepared_events = reopened.events(&receipt.job_id, &reader(&selected), 0, 1007).unwrap().events.into_iter().filter(|event| event.kind == "approval-prepared").count();
    assert_eq!(prepared_events, 1, "reviving the exact abandoned row never fabricates a second prepared event");
    assert!(matches!(reopened.read(&receipt.job_id, &reader(&selected), receipt.expires_at_ms), Err(InferenceErrorV1::Expired)));
    #[cfg(feature = "native-artifact-execution")]
    {
        let (witness, fence) = super::super::wal::tests::committed_fixture_witness().await;
        let frontier = directory::os_directory::CheckpointPublicationFrontierV1 {
            document_id: selected.document_id.clone(),
            head_edit_ordinal: selected.head_ordinal + 1,
            head_edit_id: prepared.mutation_id.clone(),
            last_commit_seq: selected.last_commit_seq + 1,
            chain_sha256: "44".repeat(32),
        };
        let descriptor_digest = selected.descriptor_digest.clone();
        let after_base_digest = "11".repeat(32);
        assert_eq!(reopened.reconcile_committed_approval(&receipt.job_id, &witness, 18, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).map(|value| value.applied), Err(InferenceErrorV1::Conflict));
        let reconciled = reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).unwrap();
        assert!(reconciled.applied);
        let recovered = reopened.reconcile_request(&selected.request.request_id, &reader(&selected), receipt.expires_at_ms).unwrap().unwrap();
        assert!(recovered.page.expired);
        let recovered_approval = recovered.approval.expect("approved job recovers its private approval authority");
        assert_eq!(recovered_approval.state, InferenceJobReconcileApprovalStateV1::Available);
        let recovered_receipt = recovered_approval.receipt.expect("available approval carries an actionable undo receipt");
        assert!(recovered_receipt.applied);
        assert_eq!(recovered_receipt.job_id, receipt.job_id);
        assert_eq!(recovered_receipt.mutation_id, prepared.mutation_id);
        assert_eq!(recovered_receipt.command_hash, prepared.command_hash);
        assert_eq!(recovered_receipt.proposal_hash, prepared.proposal_hash);
        assert_eq!(recovered_receipt.undo, reconciled.undo);
        let undo_target = reopened.gis_map_approval_undo_target(&reconciled.undo.target_id, &reader(&selected)).expect("original owner reads the retained command");
        assert_eq!(undo_target.original_command.as_slice(), command.as_slice());
        assert_eq!(undo_target.after_frontier, frontier);
        let foreign_target = InferenceReaderV1 { session_id: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", ..reader(&selected) };
        assert!(matches!(reopened.gis_map_approval_undo_target(&reconciled.undo.target_id, &foreign_target), Err(InferenceErrorV1::Denied)));
        assert!(!reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).unwrap().applied);
        let undo_command = InferencePrivateBytesV1::new(vec![9], 8192).unwrap();
        let undo_command_hash = sha256(undo_command.as_slice());
        assert!(matches!(reopened.prepare_gis_map_approval_undo(&undo_target, &"55".repeat(16), &"66".repeat(16), &"77".repeat(32), &"88".repeat(16), &undo_command_hash, &undo_command), Ok(GisMapApprovalUndoAdmissionV1::Prepared)));
        let recovered = reopened.reconcile_request(&selected.request.request_id, &reader(&selected), receipt.expires_at_ms).unwrap().unwrap();
        let recovered_approval = recovered.approval.expect("approved job retains its private recovery phase");
        assert_eq!(recovered_approval.state, InferenceJobReconcileApprovalStateV1::UndoPrepared);
        assert!(recovered_approval.receipt.is_none(), "an in-flight undo is not re-exposed as available");
        assert!(
            matches!(reopened.prepare_gis_map_approval_undo(&undo_target, &"aa".repeat(16), &"66".repeat(16), &"77".repeat(32), &"88".repeat(16), &undo_command_hash, &undo_command), Err(InferenceErrorV1::Conflict)),
            "a distinct retry identity cannot take over the durable target",
        );
        fence.invalidate();
        assert_eq!(reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).map(|value| value.applied), Err(InferenceErrorV1::Conflict));
        assert!(reopened.pending_approvals(None, &control).unwrap().rows.is_empty());
        assert_eq!(
            reopened.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash),
            Err(InferenceErrorV1::Conflict),
            "a committed decision cannot be demoted back to abandoned",
        );
        let count = reopened.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1 WHERE kind='approved'", [], |row| read_integer(row, 0)).unwrap();
        assert_eq!(count, outbox["reconciledCount"].as_u64().unwrap());
        reopened
            .connection
            .lock()
            .unwrap()
            .execute(
                "UPDATE inference_approval_undo_v1 SET phase='committed',original_command=X'',undo_command=X'',undo_frontier_head_ordinal=?2,undo_frontier_head_edit_id=?3,undo_frontier_commit_seq=?4,undo_frontier_chain_sha256=?5 WHERE target_id=?1",
                params![reconciled.undo.target_id, frontier.head_edit_ordinal + 1, "99".repeat(16), frontier.last_commit_seq + 1, "aa".repeat(32)],
            )
            .unwrap();
        let recovered = reopened.reconcile_request(&selected.request.request_id, &reader(&selected), receipt.expires_at_ms).unwrap().unwrap();
        let recovered_approval = recovered.approval.expect("approved job retains its terminal undo phase");
        assert_eq!(recovered_approval.state, InferenceJobReconcileApprovalStateV1::Undone);
        assert!(recovered_approval.receipt.is_none(), "a consumed undo target is never re-exposed");
    }
    drop(reopened);
    std::fs::remove_file(path).unwrap();
}
