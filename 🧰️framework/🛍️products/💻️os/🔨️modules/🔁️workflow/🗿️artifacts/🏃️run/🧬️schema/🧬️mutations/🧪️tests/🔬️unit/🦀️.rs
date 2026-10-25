use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use super::*;
use protocol::{Mutation, SemanticMutation};

#[test]
fn descriptors_follow_the_canonical_run_roster() {
    assert_eq!(RunMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(), vec!["start-run", "start-run-node", "finish-run-node", "append-run-log", "seal-run", "set-run-header", "set-run-seal", "remove-run-log", "remove-run-node"]);
    assert_eq!(<RunMutation as Mutation<RunArtifact>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(), vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]);
}

/// ➕️ L3 for every run kind: the concrete inverse's diffs sum to the negative of the forward diff. Ordered collections (node rows, log lines) get
/// their first, a middle and their last row removed, replaced and inserted.
#[semio_framework_async_macros::async_test]
async fn every_run_kind_inverse_diffs_sum_to_the_negative_diff() {
    use crate::tests::{apply_run_operation, sample_run_document, sample_run_node_record, set_run_header_of};
    use crate::*;
    let law = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law::<RunArtifact, RunMutation>;
    let fresh = empty_run_document().await;
    for trigger in [RunTrigger::Manual { actor: "dev".into() }, RunTrigger::Automation { automation_ref: "os.automation/a1".into(), event_fingerprint: "evt-1".into() }] {
        law(
            &RunMutation::StartRun(StartRun {
                workflow_ref: "space.space".into(),
                workflow_checkpoint_id: "ck-1".into(),
                input_collection_ref: "collections/in".into(),
                input_snapshot_id: "snap-1".into(),
                parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
                output_collection_ref: "collections/out".into(),
                trigger,
            }),
            &fresh,
        )
        .await;
    }
    let started = sample_run_document().await;
    let mut three = started.clone();
    three.node_records = vec![sample_run_node_record("a", RunNodeStatus::Computed).await, sample_run_node_record("b", RunNodeStatus::CacheHit).await, sample_run_node_record("c", RunNodeStatus::Failed).await];
    three.logs = ["one", "two", "three"].iter().map(|message| RunLogLine { node_id: "a".into(), level: "info".into(), message: (*message).into(), at: "1".into() }).collect();
    law(&RunMutation::StartRunNode(StartRunNode { node_id: "b".into() }), &started).await;
    law(&RunMutation::StartRunNode(StartRunNode { node_id: "b".into() }), &three).await;
    for id in ["a", "b", "c"] {
        law(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record(id, RunNodeStatus::Computed).await, index: None }), &three).await;
        law(&RunMutation::RemoveRunNode(RemoveRunNode { node_id: id.into() }), &three).await;
    }
    for index in [None, Some(0), Some(2), Some(3)] {
        law(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("d", RunNodeStatus::Computed).await, index }), &three).await;
    }
    law(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await, index: None }), &started).await;
    law(&RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "2".into() }), &started).await;
    law(&RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "2".into() }), &three).await;
    for count in [1, 2, 3] {
        law(&RunMutation::RemoveRunLog(RemoveRunLog { count }), &three).await;
    }
    law(&RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }), &started).await;
    let sealed = apply_run_operation(&started, &RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }));
    law(&RunMutation::SetRunSeal(SetRunSeal { sealed: false, status: RunStatus::Running, finished_at: None }), &sealed).await;
    law(&RunMutation::SetRunHeader(SetRunHeader { status: RunStatus::Pending, started_at: String::new(), ..set_run_header_of(&started) }), &started).await;
    law(&RunMutation::SetRunHeader(SetRunHeader { workflow_ref: "other.space".into(), status: RunStatus::Pending, started_at: String::new(), ..set_run_header_of(&started) }), &started).await;
}

/// 📍️ A node row removed from the middle comes back at its own index, and a row inserted at an index lands there.
#[semio_framework_async_macros::async_test]
async fn node_rows_are_position_exact() {
    use crate::tests::sample_run_node_record;
    use crate::*;
    let mut base = empty_run_document().await;
    base.node_records = vec![sample_run_node_record("a", RunNodeStatus::Computed).await, sample_run_node_record("b", RunNodeStatus::Computed).await, sample_run_node_record("c", RunNodeStatus::Computed).await];
    let removal = RunMutation::RemoveRunNode(RemoveRunNode { node_id: "b".into() });
    let diff = protocol::ColdMutation::diff(&removal, &base);
    assert_eq!(diff.diff().steps, vec![RunStep::Node { node_id: "b".into(), record: None, index: Some(1) }]);
    let inverse = protocol::ColdMutation::inverse(&removal, &base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [RunMutation::FinishRunNode(FinishRunNode { index: Some(1), .. })]), "{inverse:?}");
    let insert = RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("z", RunNodeStatus::Computed).await, index: Some(1) });
    let inserted = protocol::apply_diff_cold(protocol::ColdMutation::diff(&insert, &base).diff(), &base).expect("insert applies");
    assert_eq!(inserted.node_records.iter().map(|record| record.node_id.as_str()).collect::<Vec<_>>(), ["a", "z", "b", "c"]);
    let past = RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("z", RunNodeStatus::Computed).await, index: Some(4) });
    assert_eq!(protocol::ColdMutation::diff(&past, &base).messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), ["mutation.target-missing"]);
}
