//! 🏛️ Policy, merge and corpus laws of the protocol laws: the policy matrix, merge convergence, modify-versus-delete, chronological determinism, quarantine, ledger replay, conflict round trip and channel frame corpus laws. See [`super`].

use super::*;

//#region 🔖️Laws
//#region 🔖️Policy
pub(super) const POLICY_MATRIX_POLICIES: [crate::MergePolicy; 3] = [crate::MergePolicy::LaissezFaire, crate::MergePolicy::Normal, crate::MergePolicy::Vigilant];

pub(super) const POLICY_MATRIX_LEVELS: [semio_framework_diagnostic::Severity; 4] = [semio_framework_diagnostic::Severity::Info, semio_framework_diagnostic::Severity::Warning, semio_framework_diagnostic::Severity::Error, semio_framework_diagnostic::Severity::Fatal];

/// 📐️ The frozen 3×4 table (`📋️contract-freeze.md` "The three merge policies"): `LaissezFaire`
/// rejects only `Fatal`; `Normal` rejects `Error`+`Fatal`; `Vigilant` rejects
/// `Warning`+`Error`+`Fatal`.
pub(super) async fn policy_matrix_expected_reject(policy: crate::MergePolicy, level: semio_framework_diagnostic::Severity) -> bool {
    match policy {
        crate::MergePolicy::LaissezFaire => level == semio_framework_diagnostic::Severity::Fatal,
        crate::MergePolicy::Normal => level >= semio_framework_diagnostic::Severity::Error,
        crate::MergePolicy::Vigilant => level >= semio_framework_diagnostic::Severity::Warning,
    }
}

/// ✅️ LAW: `rejects` (typically `MergePolicy::rejects`) and `is_applicable` (typically a
/// single-message `MutationOutcome::is_applicable` probe at `level`) must both agree with the
/// frozen 3×4 policy matrix for every `(policy, level)` pair.
pub async fn assert_policy_matrix(rejects: impl AsyncFn(crate::MergePolicy, semio_framework_diagnostic::Severity) -> bool, is_applicable: impl AsyncFn(crate::MergePolicy, semio_framework_diagnostic::Severity) -> bool) {
    for policy in POLICY_MATRIX_POLICIES {
        for level in POLICY_MATRIX_LEVELS {
            let expected_reject = policy_matrix_expected_reject(policy, level).await;
            assert_eq!(rejects(policy, level).await, expected_reject, "rejects({policy:?}, {level:?}) diverged from the frozen 3x4 policy matrix");
            assert_eq!(is_applicable(policy, level).await, !expected_reject, "is_applicable({policy:?}, {level:?}) diverged from the frozen 3x4 policy matrix");
        }
    }
}

/// ✅️ LAW (1-D variant of [`assert_policy_matrix`]): a single real outcome (`mutation.diff(base)`,
/// hence "1-D" — one `(op, base)` pair, not the synthetic 3×4 grid) must still agree with the
/// frozen policy matrix at its own `worst_level()`: for every [`crate::os_spr::MergePolicy`],
/// `policy.rejects(level)` and `outcome.is_applicable(policy)` must match
/// `policy_matrix_expected_reject(policy, level)`/its negation, and an outcome with no messages
/// (`worst_level() == None`) must never be rejected. Also asserts [`assert_fatal_never_applies`]
/// (§C2 law 1) on the same outcome, since a `Fatal` outcome — rejected by every policy — must carry
/// no diff at all.
pub async fn assert_outcome_policy_matrix<P, Op>(base: &P, mutation: &Op)
where
    Op: crate::Mutation<P> + crate::ColdMutation<P>,
    Op::ReferenceDiff: PartialEq + std::fmt::Debug + Default,
{
    let outcome = mutation.diff(base);
    let worst = outcome.worst_level();
    for policy in POLICY_MATRIX_POLICIES {
        let expected_reject = match worst {
            Some(level) => policy_matrix_expected_reject(policy, level).await,
            None => false,
        };
        if let Some(level) = worst {
            assert_eq!(policy.rejects(level), expected_reject, "MergePolicy::rejects({policy:?}, {level:?}) diverged from the frozen 3x4 policy matrix for this outcome");
        }
        assert_eq!(outcome.is_applicable(policy), !expected_reject, "MutationOutcome::is_applicable({policy:?}) diverged from the frozen 3x4 policy matrix for this outcome (worst_level = {worst:?})");
    }
    assert_fatal_never_applies(&outcome).await;
}
//#endregion 🔖️Policy

//#region 🔖️Merge
/// ✅️ LAW: two peers that independently insert the same closed `envelopes` set (in independently
/// shuffled order) into a fresh [`crate::os_spr::MutationDag`] and then fold the drained batch
/// through `fold` converge on the same state — `fold` is responsible for its own canonicalization
/// (typically an HLC sort, mirroring `ingest_remote`'s own §C6 step 3) since
/// `take_next_applied`'s own order is only causally valid, not canonical.
pub async fn assert_merge_convergence<P: PartialEq + std::fmt::Debug>(seed: u64, peer_count: usize, envelopes: &[crate::MutationEnvelope], fold: impl Fn(&[crate::MutationEnvelope]) -> P) {
    let mut rng = SplitMix64(seed);
    let mut expected: Option<P> = None;
    for _ in 0..peer_count.max(2) {
        let mut order: Vec<usize> = (0..envelopes.len()).collect();
        fisher_yates_shuffle(&mut rng, &mut order).await;
        let mut dag = crate::MutationDag::new();
        for index in order {
            dag.insert(envelopes[index].clone()).expect("assert_merge_convergence requires a closed dependency set with unique ids");
        }
        let batch = take_all_applied(&mut dag);
        retire_dag(dag);
        let state = fold(&batch);
        match &expected {
            None => expected = Some(state),
            Some(reference) => assert_eq!(&state, reference, "peers that exchange the same edit set (different arrival/insertion order) must converge on the same state"),
        }
    }
}

/// ✅️ LAW: the headline modify-vs-delete scenario (`📋️contract-freeze.md` "Testkit laws"). Under
/// `Normal`/`Vigilant`, `report.accepted` must be `false` and `post_state` must equal `pre_state`
/// (quarantined, nothing applied), and `report.conflict` must resolve (in `conflicts`) to a
/// `ConflictKind::Quarantined`. Under `LaissezFaire`, `report.accepted` must be `true`,
/// `report.replayed` must carry an `Error` message, `report.conflict` must resolve to a
/// `ConflictKind::Degraded`, and `part_present(post_state)` must be `false`.
pub async fn assert_modify_vs_delete<P: PartialEq + std::fmt::Debug>(policy: crate::MergePolicy, pre_state: &P, post_state: &P, report: &crate::MergeReport, conflicts: &[crate::Conflict], part_present: impl Fn(&P) -> bool) {
    let has_error = report.replayed.iter().flat_map(|edit| &edit.messages).any(|message| message.level == semio_framework_diagnostic::Severity::Error);
    match policy {
        crate::MergePolicy::Normal | crate::MergePolicy::Vigilant => {
            assert!(!report.accepted, "under {policy:?}, a modify-vs-delete remote merge must be quarantined (MergeReport::accepted == false)");
            assert_eq!(post_state, pre_state, "under {policy:?}, a quarantined merge must leave the state unchanged");
            let quarantined = report.conflict.as_ref().and_then(|id| conflicts.iter().find(|conflict| &conflict.id == id)).expect("a quarantined MergeReport must reference an existing Conflict");
            assert!(matches!(quarantined.kind, crate::ConflictKind::Quarantined { .. }), "a rejected modify-vs-delete merge must raise a Quarantined conflict, got {:?}", quarantined.kind);
        }
        crate::MergePolicy::LaissezFaire => {
            assert!(report.accepted, "under LaissezFaire, a modify-vs-delete remote merge must be applied (MergeReport::accepted == true)");
            assert!(has_error, "under LaissezFaire, an applied modify-vs-delete merge must carry an Error message");
            let degraded = report.conflict.as_ref().and_then(|id| conflicts.iter().find(|conflict| &conflict.id == id)).expect("an applied-but-messy modify-vs-delete merge must raise a Degraded conflict");
            assert!(matches!(degraded.kind, crate::ConflictKind::Degraded { .. }), "an applied modify-vs-delete merge must raise a Degraded conflict, got {:?}", degraded.kind);
            assert!(!part_present(post_state), "under LaissezFaire, the deleted part must remain absent after the merge");
        }
    }
}

/// ✅️ LAW: any arrival order of the same envelope batch must yield the same final state, the same
/// `applied_edit_ids` order, and the same set of raised conflicts. `run` builds a fresh authority
/// from scratch and ingests `order` (a permutation of `0..envelope_count`), returning
/// `(final_state, applied_edit_ids, conflict_ids)`.
pub async fn assert_chronological_determinism<P: PartialEq + std::fmt::Debug>(envelope_count: usize, seed: u64, permutation_count: usize, mut run: impl AsyncFnMut(&[usize]) -> (P, Vec<String>, Vec<crate::ConflictId>)) {
    let mut rng = SplitMix64(seed);
    let mut expected: Option<(P, Vec<String>, Vec<crate::ConflictId>)> = None;
    for _ in 0..permutation_count.max(1) {
        let mut order: Vec<usize> = (0..envelope_count).collect();
        fisher_yates_shuffle(&mut rng, &mut order).await;
        let result = run(&order).await;
        match &expected {
            None => expected = Some(result),
            Some((state, applied, conflicts)) => {
                assert_eq!(&result.0, state, "arrival order must not change the final state");
                assert_eq!(&result.1, applied, "arrival order must not change applied_edit_ids order");
                assert_eq!(&result.2, conflicts, "arrival order must not change which conflicts get raised");
            }
        }
    }
}

/// ✅️ LAW: resolving a `Quarantined` conflict with `ConflictResolution::Accept` produces exactly
/// the state a `LaissezFaire` peer ingesting the same envelopes directly would have produced.
pub async fn assert_quarantine_accept_equals_laissez_faire<P: PartialEq + std::fmt::Debug>(state_after_accept: &P, state_under_laissez_faire: &P) {
    assert_eq!(state_after_accept, state_under_laissez_faire, "resolving a Quarantined conflict with Accept must produce exactly the state LaissezFaire would have produced");
}

/// ✅️ LAW: resolving a `Quarantined` conflict with `ConflictResolution::Discard` leaves the state
/// untouched, and none of `discarded_edit_ids` ever appears in `relayed` (the local edit ids a
/// `flush_outbound`-shaped call would ship).
pub async fn assert_quarantine_discard_preserves_state<P: PartialEq + std::fmt::Debug>(pre_state: &P, post_state: &P, discarded_edit_ids: &[String], relayed: &[String]) {
    assert_eq!(post_state, pre_state, "discarding a Quarantined conflict must leave the state untouched");
    for edit_id in discarded_edit_ids {
        assert!(!relayed.contains(edit_id), "a discarded edit ({edit_id}) must never be relayed");
    }
}

/// ✅️ LAW: the persisted `edit_messages` ledger (`edit_id -> Vec<MutationMessage>`, typically
/// collected via `ArtifactStore::messages_for_edit`) equals what a fresh history replay produces.
pub async fn assert_ledger_matches_replay(ledger: &std::collections::HashMap<String, Vec<crate::MutationMessage>>, replayed: &std::collections::HashMap<String, Vec<crate::MutationMessage>>) {
    assert_eq!(ledger, replayed, "the persisted edit_messages ledger must equal a fresh replay's messages");
}
//#endregion 🔖️Merge

//#region 🔖️Conflict
/// ✅️ LAW (§C7): `decode(encode(conflict)) == conflict` through the `.spr` conflict ledger codec
/// (`REC_CONFLICT`) — `encode`/`decode` are typically `crate::os_spr::history::encode_conflicts`/
/// `decode_conflicts` sliced to one entry.
pub async fn assert_conflict_spr_round_trip(conflict: &crate::Conflict, encode: impl AsyncFn(&crate::Conflict) -> Vec<u8>, decode: impl AsyncFn(&[u8]) -> crate::Conflict) {
    let bytes = encode(conflict).await;
    let decoded = decode(&bytes).await;
    assert_eq!(&decoded, conflict, "decode(encode(conflict)) must equal conflict");
}
//#endregion 🔖️Conflict

//#region 🔖️Channel
/// ✅️ LAW: a corpus round-trip sweep — `decode(encode(sample)) == sample` for every `sample` in
/// `corpus`. Point `encode`/`decode` at `crate::os_spr::{encode,decode}_app_command` or
/// `_app_frame` to sweep the C8 new-frame corpus
/// (`AppCommand::{SetMergePolicy,ResolveConflict,ReadConflicts}`,
/// `AppFrame::{MergeReport,Conflicts}`) once 1-C lands those variants.
pub async fn assert_channel_frame_corpus<T: PartialEq + std::fmt::Debug>(corpus: &[T], encode: impl AsyncFn(&T) -> Vec<u8>, decode: impl AsyncFn(&[u8]) -> T) {
    for sample in corpus {
        let bytes = encode(sample).await;
        let decoded = decode(&bytes).await;
        assert_eq!(&decoded, sample, "decode(encode(sample)) must equal sample for every entry in the corpus");
    }
}
//#endregion 🔖️Channel
//#endregion 🔖️Laws
