//! 🧪️ Run package serialization, admission, and replay laws.
use crate::*;
use protocol::MutationDiff;

async fn sample_run_node_record(node_id: &str, status: RunNodeStatus) -> RunNodeRecord {
    RunNodeRecord {
        node_id: node_id.into(),
        status,
        document_fingerprint: "doc-fp".into(),
        config_fingerprint: "cfg-fp".into(),
        input_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:in:in"), fingerprint: "in-fp".into() }],
        output_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:out:out"), fingerprint: "out-fp".into() }],
        outputs: vec![RunOutputArtifact { port_id: format!("{node_id}:out:out"), artifact_id: format!("artifacts/{node_id}"), path: format!("out/{node_id}.out") }],
        duration_ms: 12.5,
    }
}

async fn sample_run_document() -> RunArtifact {
    let mut document = empty_run_document().await;
    document = apply_run_operation(
        &document,
        &RunMutation::StartRun(StartRun {
            workflow_ref: "space.space".into(),
            workflow_checkpoint_id: "ck-1".into(),
            input_collection_ref: "collections/in".into(),
            input_snapshot_id: "snap-1".into(),
            parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
            output_collection_ref: "collections/out".into(),
            trigger: RunTrigger::Manual { actor: "dev".into() },
        }),
    );
    document = apply_run_operation(&document, &RunMutation::StartRunNode(StartRunNode { node_id: "a".into() }));
    document = apply_run_operation(&document, &RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("a", RunNodeStatus::Computed).await }));
    document
}

#[semio_framework_async_macros::async_test]
async fn empty_run_document_matches_schema() {
    let document = empty_run_document().await;
    assert_eq!(document.schema, S_RUN_SCHEMA);
    assert_eq!(document.status, RunStatus::Pending);
    assert!(!document.sealed);
    assert!(document.node_records.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn run_document_dsl_pack_round_trips() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample_run_document().await);
    store::os_store::test_support::assert_dsl_pack_equivalence(&empty_run_document().await);
}

#[semio_framework_async_macros::async_test]
async fn run_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "ck-1".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snap-1".into(),
        parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Manual { actor: "dev".into() },
    }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "ck-1".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snap-1".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Automation { automation_ref: "os.automation/a1".into(), event_fingerprint: "evt-1".into() },
    }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRunNode(StartRunNode { node_id: "a".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("a", RunNodeStatus::CacheHit).await }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "123".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }));
}

#[test]
fn run_payload_json_uses_exact_camel_case_and_rejects_unknown_fields() {
    let automation = RunTrigger::Automation { automation_ref: "automation".into(), event_fingerprint: "event".into() };
    let encoded = dsl::os_pack::json::parse(&dsl::os_pack::json::to_json_string(&automation)).expect("RunTrigger JSON");
    let expected = dsl::os_pack::json::parse(r#"{"kind":"automation","automationRef":"automation","eventFingerprint":"event"}"#).expect("expected JSON");
    assert!(dsl::os_pack::json::value_eq_ignoring_object_order(&encoded, &expected));
    assert!(dsl::os_pack::json::from_json_str::<RunTrigger>(r#"{"kind":"automation","automation_ref":"automation","event_fingerprint":"event"}"#).is_err());
    assert!(dsl::os_pack::json::from_json_str::<RunTrigger>(r#"{"kind":"manual","actor":"operator","extra":true}"#).is_err());
    assert!(dsl::os_pack::json::from_json_str::<RunNodeRecord>(
        r#"{"nodeId":"node","status":"computed","documentFingerprint":"document","configFingerprint":"config","inputFingerprints":[],"outputFingerprints":[],"outputs":[],"durationMs":1.0,"extra":true}"#
    )
    .is_err());
}

#[semio_framework_async_macros::async_test]
async fn checked_run_admission_matches_the_typed_diff_rejection() {
    let start = RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
    });
    let started = apply_run_operation_checked(&empty_run_document().await, start.clone()).await.expect("first start applies");
    let outcome = protocol::Mutation::diff(&start, &started);
    let expected = MutationDiff::apply(outcome.diff(), &started).expect_err("the direct diff rejects a second start");
    let actual = apply_run_operation_checked(&started, start).await.expect_err("checked admission rejects the same second start");
    assert_eq!(actual, expected);
    assert_eq!(actual.code, "mutation.apply.conflicting-target");
    assert_eq!(actual.target, vec!["status"]);
}

/// 🔒️ The load-bearing law this wave exists to prove: once `Seal` has been applied, every further
/// operation is rejected by `apply_run_operation_checked` (not silently accepted, not a panic) —
/// this is the real write seam `run::SpaceRunner` goes through for every `RunMutation` it emits.
#[semio_framework_async_macros::async_test]
async fn apply_run_operation_checked_rejects_everything_after_seal() {
    let document = sample_run_document().await;
    assert!(!document.sealed);

    let sealed = apply_run_operation_checked(&document, RunMutation::SealRun(SealRun { status: RunStatus::Succeeded })).await.expect("sealing an unsealed run must succeed");
    assert!(sealed.sealed);
    assert_eq!(sealed.status, RunStatus::Succeeded);
    assert!(sealed.finished_at.is_some());

    let rejected_log = apply_run_operation_checked(&sealed, RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "too late".into(), at: "999".into() }));
    assert!(rejected_log.await.is_err(), "a Log after Seal must be rejected, not silently applied");

    let rejected_node_finished = apply_run_operation_checked(&sealed, RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await }));
    assert!(rejected_node_finished.await.is_err(), "a NodeFinished after Seal must be rejected");

    let rejected_reseal = apply_run_operation_checked(&sealed, RunMutation::SealRun(SealRun { status: RunStatus::Failed }));
    assert!(rejected_reseal.await.is_err(), "re-sealing an already-sealed run must be rejected");

    assert_eq!(sealed.node_records.len(), 1, "the rejected NodeFinished must not have been applied");
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_preserves_each_append_in_order() {
    let document = empty_run_document().await;
    let first = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "first".into(), at: "1".into() });
    let first_diff = protocol::Mutation::diff(&first, &document).diff().clone();
    let middle = MutationDiff::apply(&first_diff, &document).expect("first append applies");
    let second = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "second".into(), at: "2".into() });
    let mut combined = first_diff;
    combined.absorb(protocol::Mutation::diff(&second, &middle).diff().clone());
    let after = MutationDiff::apply(&combined, &document).expect("combined appends apply");
    assert_eq!(after.logs.iter().map(|line| line.message.as_str()).collect::<Vec<_>>(), vec!["first", "second"]);
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_preserves_start_before_later_log() {
    let document = empty_run_document().await;
    let start = RunMutation::StartRun(StartRun {
        workflow_ref: "workflow-selected".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "inputs".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "outputs".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
    });
    let start_diff = protocol::Mutation::diff(&start, &document).diff().clone();
    let middle = MutationDiff::apply(&start_diff, &document).expect("start applies");
    let append = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "started".into(), at: "1".into() });
    let mut combined = start_diff;
    combined.absorb(protocol::Mutation::diff(&append, &middle).diff().clone());
    let after = MutationDiff::apply(&combined, &document).expect("combined start and append apply");
    assert_eq!(after.workflow_ref, "workflow-selected");
    assert_eq!(after.status, RunStatus::Running);
    assert_eq!(after.logs.iter().map(|line| line.message.as_str()).collect::<Vec<_>>(), vec!["started"]);
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_is_associative_with_empty_identity() {
    let document = empty_run_document().await;
    let log = |message: &str| RunDiff::Log { node_id: String::new(), level: "info".into(), message: message.into(), at: message.into() };
    let first = log("first");
    let second = log("second");
    let third = log("third");
    let mut left = first.clone();
    left.absorb(second.clone());
    left.absorb(third.clone());
    let mut suffix = second;
    suffix.absorb(third);
    let mut right = first.clone();
    right.absorb(suffix);
    let mut leading_identity = RunDiff::Empty;
    leading_identity.absorb(first.clone());
    let mut trailing_identity = first.clone();
    trailing_identity.absorb(RunDiff::Empty);
    assert_eq!(left, right);
    assert_eq!(leading_identity, first);
    assert_eq!(trailing_identity, first);
    assert_eq!(MutationDiff::apply(&left, &document), MutationDiff::apply(&right, &document));
}

#[semio_framework_async_macros::async_test]
async fn run_diff_sequence_rejects_later_steps_without_mutating_the_base() {
    let document = empty_run_document().await;
    let mut seal_then_log = RunDiff::Seal { status: RunStatus::Succeeded };
    seal_then_log.absorb(RunDiff::Log { node_id: String::new(), level: "info".into(), message: "late".into(), at: "1".into() });
    let sealed_error = MutationDiff::apply(&seal_then_log, &document).expect_err("log after a composed seal rejects");
    assert_eq!(sealed_error.code, "mutation.apply.sealed");
    assert_eq!(sealed_error.target, vec!["sealed"]);
    assert_eq!(document, empty_run_document().await);

    let first_start = RunDiff::Start {
        workflow_ref: "workflow".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "inputs".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "outputs".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
    };
    let mut double_start = first_start.clone();
    double_start.absorb(first_start);
    let start_error = MutationDiff::apply(&double_start, &document).expect_err("second composed start rejects");
    assert_eq!(start_error.code, "mutation.apply.conflicting-target");
    assert_eq!(start_error.target, vec!["status"]);
    assert_eq!(document, empty_run_document().await);
}

#[semio_framework_async_macros::async_test]
async fn finish_run_node_replacement_inverse_restores_the_original_node_order() {
    let mut document = empty_run_document().await;
    document.node_records = vec![sample_run_node_record("a", RunNodeStatus::CacheHit).await, sample_run_node_record("b", RunNodeStatus::CacheHit).await, sample_run_node_record("c", RunNodeStatus::CacheHit).await];
    let operation = RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await });
    let inverse = protocol::Mutation::inverse(&operation, &document);
    let mut restored = apply_run_operation(&document, &operation);
    assert_eq!(restored.node_records.iter().map(|record| record.node_id.as_str()).collect::<Vec<_>>(), vec!["a", "b", "c"]);
    for step in inverse.iter().rev() {
        restored = apply_run_operation(&restored, step);
    }
    assert_eq!(restored, document);
}

#[semio_framework_async_macros::async_test]
async fn run_node_record_dsl_pack_round_trips_nested_tables() {
    let record = sample_run_node_record("a", RunNodeStatus::Failed).await;
    let mut document = empty_run_document().await;
    document.node_records.push(record);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_package_cases_match_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("📦️package-contract/📜️cases.json")).expect("language-neutral fixture");
    assert_eq!(fixture["package"], env!("CARGO_PKG_NAME"));
    for case in fixture["cases"].as_array().expect("cases") {
        let mut document = empty_run_document().await;
        let mut rejections = Vec::new();
        for value in case["operations"].as_array().expect("operations") {
            let encoded = serde_json::to_string(value).expect("third-party JSON oracle");
            let operation: RunMutation = dsl::os_pack::json::from_json_str(&encoded).expect("domain operation JSON");
            let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&operation)).expect("domain JSON output");
            assert_eq!(&actual, value);
            match apply_run_operation_checked(&document, operation).await {
                Ok(next) => document = next,
                Err(error) => rejections.push(error.code),
            }
        }
        let status: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&document.status)).expect("status JSON");
        let actual = serde_json::json!({"schema":document.schema,"status":status,"sealed":document.sealed,"logs":document.logs.iter().map(|line| &line.message).collect::<Vec<_>>(),"rejections":rejections});
        assert_eq!(actual, case["expected"], "{}", case["name"]);
        println!("[DEBUG] Workflow run package fixture passed: {}", case["name"]);
    }
}
