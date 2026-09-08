
use super::*;

//#region 🔖️Gen
#[semio_framework_async_macros::async_test]
async fn history_log_gen_is_deterministic_for_a_fixed_seed() {
    let profile = GenProfile { edit_count: 5, max_ops_per_edit: 3, checkpoint_every: 2, adversarial: false };
    let a = HistoryLogGen::new(42).await.generate(&profile).await;
    let b = HistoryLogGen::new(42).await.generate(&profile).await;
    assert_eq!(a, b, "the same seed must generate the same HistoryLog");
}

#[semio_framework_async_macros::async_test]
async fn history_log_gen_different_seeds_usually_differ() {
    let profile = GenProfile { edit_count: 5, max_ops_per_edit: 3, checkpoint_every: 2, adversarial: false };
    let a = HistoryLogGen::new(1).await.generate(&profile).await;
    let b = HistoryLogGen::new(2).await.generate(&profile).await;
    assert_ne!(a, b);
}

#[semio_framework_async_macros::async_test]
async fn history_log_gen_respects_edit_count_and_checkpoint_cadence() {
    let profile = GenProfile { edit_count: 9, max_ops_per_edit: 2, checkpoint_every: 3, adversarial: false };
    let log = HistoryLogGen::new(7).await.generate(&profile).await;
    assert_eq!(log.edits.len(), 9);
    assert_eq!(log.changes.len(), 3, "9 edits at checkpoint_every=3 must produce 3 changes");
    assert_eq!(log.checkpoints.len(), 3);
    for edit in &log.edits {
        assert!(edit.ops.len() <= 2);
        assert!(edit.meta.is_none(), "this generator never populates the derived-data meta slot");
    }
}

#[semio_framework_async_macros::async_test]
async fn history_log_gen_zero_checkpoint_every_produces_no_changes_or_checkpoints() {
    let profile = GenProfile { edit_count: 4, max_ops_per_edit: 2, checkpoint_every: 0, adversarial: false };
    let log = HistoryLogGen::new(3).await.generate(&profile).await;
    assert!(log.changes.is_empty());
    assert!(log.checkpoints.is_empty());
    assert!(log.alternatives.is_empty(), "no checkpoints -> no alternatives to reference them");
}

#[semio_framework_async_macros::async_test]
async fn history_log_gen_adversarial_op_text_never_breaks_the_ops_line_grammar() {
    let profile = GenProfile { edit_count: 40, max_ops_per_edit: 6, checkpoint_every: 0, adversarial: true };
    for seed in 0..20u64 {
        let log = HistoryLogGen::new(seed).await.generate(&profile).await;
        for edit in &log.edits {
            for op in &edit.ops {
                let text = op.text.as_deref().expect("generator always sets text");
                assert!(!text.is_empty(), "generated op text must never be empty");
                assert!(!text.contains('\n'), "generated op text must never contain a newline");
                assert_eq!(text.trim(), text, "generated op text must have no leading/trailing whitespace");
                assert!(!text.starts_with('#'), "generated op text must never look like a comment line");
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn op_dag_gen_produces_a_closed_topologically_orderable_set() {
    let envelopes = OpDagGen::new(11).await.generate(30).await;
    assert_eq!(envelopes.len(), 30);
    let known: std::collections::HashSet<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
    for (index, envelope) in envelopes.iter().enumerate() {
        assert_eq!(envelope.mutation_id.0, format!("op-{index}"));
        for dependency in &envelope.dependencies {
            assert!(known.contains(&dependency.0), "every dependency must reference a node within the generated set");
            let dep_index: usize = dependency.0.strip_prefix("op-").unwrap().parse().unwrap();
            assert!(dep_index < index, "every dependency must reference a strictly earlier index");
        }
    }
}
//#endregion 🔖️Gen

//#region 🔖️Laws
async fn tiny_profile() -> GenProfile {
    GenProfile { edit_count: 3, max_ops_per_edit: 3, checkpoint_every: 0, adversarial: false }
}

async fn typical_profile() -> GenProfile {
    GenProfile { edit_count: 20, max_ops_per_edit: 5, checkpoint_every: 4, adversarial: false }
}

async fn adversarial_profile() -> GenProfile {
    GenProfile { edit_count: 15, max_ops_per_edit: 8, checkpoint_every: 5, adversarial: true }
}

#[semio_framework_async_macros::async_test]
async fn history_encode_decode_identity_across_profiles() {
    for (seed, profile) in [(1u64, tiny_profile().await), (2, typical_profile().await), (3, adversarial_profile().await)] {
        let log = HistoryLogGen::new(seed).await.generate(&profile).await;
        assert_history_encode_decode_identity(&log).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn history_canonical_stable_across_profiles() {
    for (seed, profile) in [(4u64, tiny_profile().await), (5, typical_profile().await), (6, adversarial_profile().await)] {
        let log = HistoryLogGen::new(seed).await.generate(&profile).await;
        assert_history_canonical_stable(&log).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn history_streamed_equals_buffered_across_profiles() {
    for (seed, profile) in [(7u64, tiny_profile().await), (8, typical_profile().await), (9, adversarial_profile().await)] {
        let log = HistoryLogGen::new(seed).await.generate(&profile).await;
        assert_streamed_equals_buffered(&log).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn history_encode_decode_identity_handles_empty_edits_and_history() {
    assert_history_encode_decode_identity(&HistoryLogGen::new(10).await.generate(&GenProfile { edit_count: 0, max_ops_per_edit: 0, checkpoint_every: 0, adversarial: false }).await).await;
    let mut zero_op_profile = tiny_profile().await;
    zero_op_profile.max_ops_per_edit = 0;
    assert_history_encode_decode_identity(&HistoryLogGen::new(11).await.generate(&zero_op_profile).await).await;
}

#[semio_framework_async_macros::async_test]
async fn ops_protocol_bidirectional_on_a_hand_written_sample() {
    assert_ops_protocol_bidirectional(
        "doc \"doc-1\" schema=\"schema-1\"\n\
             edit \"e0\" started=\"2026-07-27T00:00:00Z\" actor=\"actor-1\" description=\"first edit\"\n\
             \x20\x20set foo = 1\n\
             \x20\x20set bar = 2\n\
             edit \"e1\" started=\"2026-07-27T00:00:01Z\" finished=\"2026-07-27T00:00:05Z\"\n\
             \x20\x20noop\n",
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ops_protocol_bidirectional_skips_comments_and_blank_lines() {
    assert_ops_protocol_bidirectional("doc \"doc-2\" schema=\"schema-2\"\n\n# a comment before active\nactive \"alt-1\"\n").await;
}

#[semio_framework_async_macros::async_test]
async fn ops_protocol_bidirectional_on_generated_logs() {
    for (seed, profile) in [(12u64, tiny_profile().await), (13, typical_profile().await)] {
        let log = HistoryLogGen::new(seed).await.generate(&profile).await;
        let bytes = write_history_log(&log, false).await;
        let ops_text = crate::os_spr::decompile_ops(&bytes, &crate::os_spr::DecodeOptions::default()).await.expect("decompile_ops");
        assert_ops_protocol_bidirectional(&ops_text).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn zero_copy_holds_across_a_generated_history() {
    let log = HistoryLogGen::new(14).await.generate(&typical_profile().await).await;
    assert_zero_copy(&write_history_log(&log, false).await).await;
}

#[semio_framework_async_macros::async_test]
async fn chain_detects_tamper_on_a_generated_history() {
    let log = HistoryLogGen::new(15).await.generate(&typical_profile().await).await;
    assert_chain_detects_tamper(&write_history_log(&log, false).await).await;
}

#[semio_framework_async_macros::async_test]
async fn recovery_truncates_to_commit_quick() {
    let log = HistoryLogGen::new(16).await.generate(&typical_profile().await).await;
    assert_recovery_truncates_to_commit(&write_history_log(&log, true).await, CorruptionLevel::Quick).await;
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn compaction_identity_on_a_generated_history() {
    let log = HistoryLogGen::new(17).await.generate(&typical_profile().await).await;
    assert_compaction_identity(&write_history_log(&log, false).await).await;
}

//#region 🏃️quick
mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn op_dag_convergence_holds_on_a_small_generated_dag() {
        let envelopes = OpDagGen::new(20).await.generate(6).await;
        assert_op_dag_convergence(&envelopes, 100, 8).await;
    }
}
//#endregion 🏃️quick

//#region 🏃️exhaustive
mod exhaustive {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn op_dag_convergence_holds_on_a_larger_generated_dag() {
        let envelopes = OpDagGen::new(21).await.generate(60).await;
        assert_op_dag_convergence(&envelopes, 200, 40).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn recovery_truncates_to_commit_exhaustive_on_a_small_fixture() {
        let log = HistoryLogGen::new(22).await.generate(&tiny_profile().await).await;
        assert_recovery_truncates_to_commit(&write_history_log(&log, true).await, CorruptionLevel::Exhaustive).await;
    }
}
//#endregion 🏃️exhaustive

//#region 🧸️Fixtures
use super::mutation_law_fixture::{AddCounter, AddMissingCounter, AddObservedCounter, AddRejectedCounter, AddUncheckedCounter, CounterDiff, CounterMutation};

async fn sample_conflict(id: &str, kind: crate::os_spr::ConflictKind) -> crate::os_spr::Conflict {
    crate::os_spr::Conflict { id: crate::os_spr::ConflictId(id.to_string()), kind, status: crate::os_spr::ConflictStatus::Open, messages: Vec::new(), actors: Vec::new(), timestamp: crate::os_spr::HybridLogicalTimestamp::new(1, 100) }
}
//#endregion 🧸️Fixtures

//#region 🔖️Laws (continued)
#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_holds_and_rejects_a_broken_impl() {
    assert_op_text_round_trip(&CounterMutation::AddCounter(AddCounter { delta: -7 })).await;
    assert_op_text_round_trip(&CounterMutation::AddCounter(AddCounter { delta: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn operation_diff_apply_matches_backwards_inverse() {
    use crate::os_spr::{Mutation, MutationDiff};
    let base: i64 = 10;
    let op = CounterMutation::AddCounter(AddCounter { delta: 5 });
    let forward = op.diff(&base).diff().apply(&base).expect("valid forward diff");
    assert_eq!(forward, 15);
    let [undo] = <[CounterMutation; 1]>::try_from(op.inverse(&base)).unwrap();
    assert_eq!(undo.diff(&forward).diff().apply(&forward), Ok(base));
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_absorb_law_holds_for_add() {
    assert_mutation_diff_absorb_law(&10i64, CounterDiff::delta(3), CounterDiff::delta(4)).await;
}

#[semio_framework_async_macros::async_test]
async fn mutation_inverse_law_holds_for_add() {
    assert_mutation_inverse_law(&10i64, &CounterMutation::AddCounter(AddCounter { delta: 5 })).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must not have been rejected")]
async fn mutation_inverse_law_panics_when_forward_outcome_is_rejected() {
    assert_mutation_inverse_law(&10i64, &CounterMutation::AddRejectedCounter(AddRejectedCounter {})).await;
}

#[semio_framework_async_macros::async_test]
async fn diff_algebra_between_law_holds_for_add() {
    assert_diff_algebra_between_law::<i64, CounterDiff>(&10, &17).await;
}

#[semio_framework_async_macros::async_test]
async fn diff_algebra_inverse_law_holds_for_add() {
    assert_diff_algebra_inverse_law(&10i64, &CounterDiff::delta(5)).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must recover an equal operation")]
async fn op_text_round_trip_panics_on_a_lossy_impl() {
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct LossyOp {
        delta: i64,
    }
    impl crate::os_spr::OpText for LossyOp {
        fn print_op(&self) -> String {
            "lossy".to_string()
        }
        fn parse_op(_line: &str) -> Result<Self, crate::os_dsl::TextError> {
            Ok(LossyOp { delta: 0 })
        }
    }
    assert_op_text_round_trip(&LossyOp { delta: 42 }).await;
}

#[semio_framework_async_macros::async_test]
async fn wire_frame_round_trip_holds_for_client_and_server_samples() {
    assert_wire_frame_round_trip(&WireFrameSample::Client(crate::os_spr::ClientFrame::Bye, crate::os_spr::Lane::Command)).await;
    assert_wire_frame_round_trip(&WireFrameSample::Client(crate::os_spr::ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, crate::os_spr::Lane::Preview)).await;
    let frontier = crate::os_spr::RuntimeFrontierSummary { document_id: crate::os_spr::ArtifactId("doc-1".to_string()), head_edit_ordinal: 5, head_edit_id: "edit-5".to_string(), last_commit_seq: 2, chain_hash: [7u8; 32] };
    assert_wire_frame_round_trip(&WireFrameSample::Server(
        crate::os_spr::ServerFrame::Welcome { session_id: "s1".to_string(), resume_token: "r1".to_string(), server_frontier: frontier, bootstrap: crate::os_spr::Bootstrap::Tail },
        crate::os_spr::Lane::Command,
    ))
    .await;
}

#[semio_framework_async_macros::async_test]
async fn channel_frame_round_trip_holds_for_command_and_frame_samples() {
    assert_channel_frame_round_trip(&ChannelFrameSample::Command(&crate::os_spr::AppCommand::ReadConflicts { seq: 1 })).await;
    assert_channel_frame_round_trip(&ChannelFrameSample::Command(&crate::os_spr::AppCommand::ConfigCommand { seq: 1, command: vec![1, 2, 3] })).await;
    assert_channel_frame_round_trip(&ChannelFrameSample::Frame(&crate::os_spr::AppFrame::Done { in_reply_to: 1 })).await;
    assert_channel_frame_round_trip(&ChannelFrameSample::Frame(&crate::os_spr::AppFrame::Error { in_reply_to: None, fault: b"e:m".to_vec(), report: Vec::new() })).await;
}
//#endregion 🔖️Laws (continued)

//#region 🔖️Outcome
#[semio_framework_async_macros::async_test]
async fn missing_target_is_error_holds_for_a_correct_impl() {
    assert_missing_target_is_error(&10i64, &CounterMutation::AddMissingCounter(AddMissingCounter {})).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "mutation.target-missing")]
async fn missing_target_is_error_panics_on_a_buggy_impl() {
    assert_missing_target_is_error(&10i64, &CounterMutation::AddUncheckedCounter(AddUncheckedCounter {})).await;
}

#[semio_framework_async_macros::async_test]
async fn fatal_never_applies_holds_for_a_correct_outcome() {
    let outcome: crate::os_spr::MutationOutcome<CounterDiff> = crate::os_spr::MutationOutcome::fatal("mutation.invariant", "boom", ["x"]);
    assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "Fatal outcome must carry diff == D::default()")]
async fn fatal_never_applies_panics_on_a_non_empty_diff() {
    let outcome = crate::os_spr::MutationOutcome::new(CounterDiff::delta(3)).absorb_messages([crate::os_spr::MutationMessage::fatal("mutation.invariant", "boom")]);
    assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
async fn outcome_deterministic_holds_for_add() {
    assert_outcome_deterministic(&10i64, &CounterMutation::AddCounter(AddCounter { delta: 4 })).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must be deterministic")]
async fn outcome_deterministic_panics_on_a_nondeterministic_impl() {
    assert_outcome_deterministic(&10i64, &CounterMutation::AddObservedCounter(AddObservedCounter::default())).await;
}
//#endregion 🔖️Outcome

//#region 🔖️Policy
async fn message_at_level(level: crate::os_dsl::Severity) -> crate::os_spr::MutationMessage {
    match level {
        crate::os_dsl::Severity::Info => crate::os_spr::MutationMessage::info("mutation.cascade", "probe"),
        crate::os_dsl::Severity::Warning => crate::os_spr::MutationMessage::warn("mutation.no-op", "probe"),
        crate::os_dsl::Severity::Error => crate::os_spr::MutationMessage::error("mutation.target-missing", "probe"),
        crate::os_dsl::Severity::Fatal => crate::os_spr::MutationMessage::fatal("mutation.invariant", "probe"),
    }
}

#[semio_framework_async_macros::async_test]
async fn policy_matrix_holds_for_the_real_apis() {
    assert_policy_matrix(
        async |policy, level| policy.rejects(level),
        async |policy, level| {
            let outcome: crate::os_spr::MutationOutcome<()> = crate::os_spr::MutationOutcome::new(()).absorb_messages([message_at_level(level).await]);
            outcome.is_applicable(policy)
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "diverged from the frozen 3x4 policy matrix")]
async fn policy_matrix_panics_on_a_wrong_impl() {
    assert_policy_matrix(async |_, _| false, async |_, _| true).await;
}
//#endregion 🔖️Policy

//#region 🔖️Merge
#[semio_framework_async_macros::async_test]
async fn merge_convergence_holds_for_a_commutative_fold() {
    let envelopes = OpDagGen::new(30).await.generate(10).await;
    assert_merge_convergence(300, 5, &envelopes, |batch| {
        let mut batch = batch.to_vec();
        batch.sort_by_key(|envelope| envelope.timestamp);
        batch.iter().fold(0i64, |state, envelope| {
            let payload = std::str::from_utf8(&envelope.diff.payload).unwrap();
            let index: i64 = payload.strip_prefix("index:").unwrap().parse().unwrap();
            state + index
        })
    })
    .await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must converge on the same state")]
async fn merge_convergence_panics_for_an_order_dependent_fold() {
    let envelopes = OpDagGen::new(31).await.generate(10).await;
    assert_merge_convergence(301, 6, &envelopes, |batch| batch.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>().join(",")).await;
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_holds_for_normal_quarantine() {
    let pre = Some("part".to_string());
    let post = pre.clone();
    let conflict = sample_conflict("c1", crate::os_spr::ConflictKind::Quarantined { envelopes: Vec::new() }).await;
    let report = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::Normal, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: Some(crate::os_dsl::Severity::Error), conflict: Some(conflict.id.clone()) };
    assert_modify_vs_delete(crate::os_spr::MergePolicy::Normal, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some()).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must be quarantined")]
async fn modify_vs_delete_panics_when_normal_wrongly_accepts() {
    let pre = Some("part".to_string());
    let post = pre.clone();
    let report = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::Normal, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: None };
    assert_modify_vs_delete(crate::os_spr::MergePolicy::Normal, &pre, &post, &report, &[], |state| state.is_some()).await;
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_holds_for_laissez_faire_apply() {
    let pre = Some("part".to_string());
    let post: Option<String> = None;
    let conflict = sample_conflict("c2", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] }).await;
    let report = crate::os_spr::MergeReport {
        policy: crate::os_spr::MergePolicy::LaissezFaire,
        accepted: true,
        insertion_index: 0,
        replayed: vec![crate::os_spr::EditMessages { edit_id: "e1".to_string(), messages: vec![crate::os_spr::MutationMessage::error("mutation.target-missing", "part gone")] }],
        worst: Some(crate::os_dsl::Severity::Error),
        conflict: Some(conflict.id.clone()),
    };
    assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some()).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must remain absent")]
async fn modify_vs_delete_panics_when_laissez_faire_part_still_present() {
    let pre = Some("part".to_string());
    let post = pre.clone();
    let conflict = sample_conflict("c3", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] }).await;
    let report = crate::os_spr::MergeReport {
        policy: crate::os_spr::MergePolicy::LaissezFaire,
        accepted: true,
        insertion_index: 0,
        replayed: vec![crate::os_spr::EditMessages { edit_id: "e1".to_string(), messages: vec![crate::os_spr::MutationMessage::error("mutation.target-missing", "part gone")] }],
        worst: Some(crate::os_dsl::Severity::Error),
        conflict: Some(conflict.id.clone()),
    };
    assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some()).await;
}

#[semio_framework_async_macros::async_test]
async fn chronological_determinism_holds_for_an_order_independent_run() {
    assert_chronological_determinism(5, 400, 6, async |order| {
        let mut sorted = order.to_vec();
        sorted.sort_unstable();
        (sorted.clone(), sorted.iter().map(|i| format!("edit-{i}")).collect(), Vec::new())
    })
    .await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must not change the final state")]
async fn chronological_determinism_panics_for_an_order_dependent_run() {
    assert_chronological_determinism(5, 401, 6, async |order| (order.to_vec(), order.iter().map(|i| format!("edit-{i}")).collect(), Vec::new())).await;
}

#[semio_framework_async_macros::async_test]
async fn quarantine_accept_equals_laissez_faire_holds_when_equal() {
    assert_quarantine_accept_equals_laissez_faire(&5i64, &5i64).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must produce exactly the state LaissezFaire would have produced")]
async fn quarantine_accept_equals_laissez_faire_panics_when_unequal() {
    assert_quarantine_accept_equals_laissez_faire(&5i64, &6i64).await;
}

#[semio_framework_async_macros::async_test]
async fn quarantine_discard_preserves_state_holds() {
    assert_quarantine_discard_preserves_state(&5i64, &5i64, &["e1".to_string()], &["e2".to_string()]).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must never be relayed")]
async fn quarantine_discard_preserves_state_panics_when_relayed() {
    assert_quarantine_discard_preserves_state(&5i64, &5i64, &["e1".to_string()], &["e1".to_string()]).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must leave the state untouched")]
async fn quarantine_discard_preserves_state_panics_when_state_changes() {
    assert_quarantine_discard_preserves_state(&5i64, &6i64, &["e1".to_string()], &[]).await;
}

#[semio_framework_async_macros::async_test]
async fn ledger_matches_replay_holds_when_equal() {
    let mut ledger = std::collections::HashMap::new();
    ledger.insert("e1".to_string(), vec![crate::os_spr::MutationMessage::info("mutation.cascade", "note")]);
    let replayed = ledger.clone();
    assert_ledger_matches_replay(&ledger, &replayed).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must equal a fresh replay")]
async fn ledger_matches_replay_panics_when_unequal() {
    let mut ledger = std::collections::HashMap::new();
    ledger.insert("e1".to_string(), vec![crate::os_spr::MutationMessage::info("mutation.cascade", "note")]);
    let mut replayed = std::collections::HashMap::new();
    replayed.insert("e1".to_string(), Vec::new());
    assert_ledger_matches_replay(&ledger, &replayed).await;
}
//#endregion 🔖️Merge

//#region 🔖️Conflict
#[semio_framework_async_macros::async_test]
async fn conflict_spr_round_trip_holds_for_an_identity_codec() {
    let conflict = sample_conflict("c4", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] }).await;
    let for_decode = conflict.clone();
    assert_conflict_spr_round_trip(&conflict, async |_c| Vec::new(), async move |_bytes| for_decode.clone()).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must equal conflict")]
async fn conflict_spr_round_trip_panics_for_a_lossy_codec() {
    let conflict = sample_conflict("c5", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] });
    assert_conflict_spr_round_trip(&conflict.await, async |_c| Vec::new(), async |_bytes| sample_conflict("different", crate::os_spr::ConflictKind::Degraded { edit_ids: Vec::new() }).await).await;
}
//#endregion 🔖️Conflict

//#region 🔖️Channel
#[semio_framework_async_macros::async_test]
async fn frame_corpus_round_trip_holds_for_the_real_app_command_codec() {
    let corpus = vec![crate::os_spr::AppCommand::ReadConflicts { seq: 1 }, crate::os_spr::AppCommand::ConfigCommand { seq: 1, command: vec![1, 2, 3] }];
    assert_channel_frame_corpus(
        &corpus,
        async |command| {
            let encoded = crate::os_spr::encode_app_command(command).await.unwrap();
            assert_eq!(encoded.page_len(), 1);
            encoded.front_page().unwrap().as_slice().to_vec()
        },
        async |bytes| crate::os_spr::channel::decode_app_command(bytes).await.unwrap(),
    )
    .await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must equal sample")]
async fn frame_corpus_round_trip_panics_for_a_lossy_codec() {
    let corpus = vec![crate::os_spr::AppCommand::ReadConflicts { seq: 1 }];
    assert_channel_frame_corpus(&corpus, async |_command| Vec::new(), async |_bytes| crate::os_spr::AppCommand::ReadConflicts { seq: 0 }).await;
}
//#endregion 🔖️Channel

//#region 🔖️Corrupt
#[semio_framework_async_macros::async_test]
async fn fuzz_truncation_never_panics_history_reader_open() {
    let log = HistoryLogGen::new(23).await.generate(&typical_profile().await).await;
    let bytes = write_history_log(&log, true).await;
    let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| crate::os_io::resolve_ready(open_and_log(candidate, &crate::os_spr::DecodeOptions::default())).map(|_| ()).map_err(|error| error.to_string()));
    assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a truncated buffer: {:?}", report.cases_panicked);
}

#[semio_framework_async_macros::async_test]
async fn fuzz_bit_flips_never_panics_history_reader_open() {
    let log = HistoryLogGen::new(24).await.generate(&typical_profile().await).await;
    let bytes = write_history_log(&log, true).await;
    let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, |candidate| crate::os_io::resolve_ready(open_and_log(candidate, &crate::os_spr::DecodeOptions::default())).map(|_| ()).map_err(|error| error.to_string()));
    assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a bit-flipped buffer: {:?}", report.cases_panicked);
}

#[semio_framework_async_macros::async_test]
async fn fuzz_truncation_never_panics_recover() {
    let log = HistoryLogGen::new(25).await.generate(&typical_profile().await).await;
    let bytes = write_history_log(&log, true).await;
    let limits = crate::os_spr::ProtocolLimits::default();
    let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| crate::os_io::resolve_ready(crate::os_spr::format::recover(&candidate, &limits, crate::os_spr::RecoveryMode::LastCommit)).map(|_| ()).map_err(|error| error.to_string()));
    assert!(report.cases_panicked.is_empty(), "crate::os_spr::format::recover must never panic on a truncated buffer: {:?}", report.cases_panicked);
}
//#endregion 🔖️Corrupt

//#region 🔖️Golden
#[semio_framework_async_macros::async_test]
async fn golden_hash_hex_is_deterministic_and_hex_encoded() {
    let a = golden_hash_hex(b"protocol_testkit golden fixture").await;
    let b = golden_hash_hex(b"protocol_testkit golden fixture").await;
    assert_eq!(a, b);
    assert_eq!(a.len(), 64);
    assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
}
//#endregion 🔖️Golden
