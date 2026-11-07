//! 🫴️ Every run mutation variant is received by its original cursor into the value its decode oracle returns.
use crate::*;

#[test]
fn every_run_mutation_variant_is_received_by_its_original_cursor_like_its_decode_oracle() {
    let parameters = vec![RunParameterValue { parameter_id: "p".into(), value: "1".into() }];
    let record = RunNodeRecord {
        node_id: "n".into(),
        status: RunNodeStatus::CacheHit,
        document_fingerprint: "d".into(),
        config_fingerprint: "c".into(),
        input_fingerprints: vec![PortFingerprint { port_id: "i".into(), fingerprint: "f".into() }],
        output_fingerprints: Vec::new(),
        outputs: vec![RunOutputArtifact { port_id: "o".into(), artifact_id: "a".into(), path: "x/y".into() }],
        duration_ms: 1.5,
    };
    let start = |trigger: RunTrigger| StartRun { workflow_ref: "w".into(), workflow_checkpoint_id: "k".into(), input_collection_ref: "ic".into(), input_snapshot_id: "is".into(), parameter_values: parameters.clone(), output_collection_ref: "oc".into(), trigger };
    let operations = [
        RunMutation::StartRun(start(RunTrigger::Manual { actor: "me".into() })),
        RunMutation::StartRun(start(RunTrigger::Automation { automation_ref: "auto".into(), event_fingerprint: "fp".into() })),
        RunMutation::StartRunNode(StartRunNode { node_id: "n".into() }),
        RunMutation::FinishRunNode(FinishRunNode { node_record: record, index: Some(2) }),
        RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "m".into(), at: "t".into() }),
        RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }),
        RunMutation::SetRunSeal(SetRunSeal { sealed: true, status: RunStatus::Failed, finished_at: None }),
        RunMutation::SetRunHeader(SetRunHeader { workflow_ref: "w".into(), workflow_checkpoint_id: "k".into(), input_collection_ref: "ic".into(), input_snapshot_id: "is".into(), parameter_values: parameters.clone(), output_collection_ref: "oc".into(), trigger: RunTrigger::Manual { actor: "me".into() }, status: RunStatus::Running, started_at: "s".into() }),
        RunMutation::RemoveRunLog(RemoveRunLog { count: 3 }),
        RunMutation::RemoveRunNode(RemoveRunNode { node_id: "n".into() }),
    ];
    dsl::original_receiving::check_receiving_round_trip(&operations).unwrap();
}
