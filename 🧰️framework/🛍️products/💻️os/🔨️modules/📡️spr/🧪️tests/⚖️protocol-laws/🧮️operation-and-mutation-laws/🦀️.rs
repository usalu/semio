//! 🧮️ Operation text and mutation laws of the protocol laws: the operation text round trip and the diff absorption, inverse and inverse-sum laws with their cold variants and the diff algebra inverse law. See [`super`].


//#region 🔖️Laws
/// ✅️ LAW: `OpText::print_op` output never contains `'\n'`, and `parse_op(op.print_op())` recovers
/// an equal operation.
pub async fn assert_op_text_round_trip<Op>(op: &Op)
where
    Op: crate::OpText + Clone + PartialEq + std::fmt::Debug,
{
    let line = op.print_op();
    assert!(!line.contains('\n'), "OpText::print_op output must never contain a newline, got {line:?}");
    let parsed = Op::parse_op(&line).unwrap_or_else(|error| panic!("OpText::parse_op failed to parse its own print_op output {line:?}: {error:?}"));
    assert_eq!(&parsed, op, "OpText::parse_op(op.print_op()) must recover an equal operation");
}

/// ✅️ LAW: `absorb(d1, d2).apply(base) == d2.apply(&d1.apply(base))` — the sequential-coalesce
/// contract every `ColdMutationDiff::absorb` impl must satisfy (`📡️spr/🎮️command/🦀️.rs`'s
/// `🔖️Mutation` region doc). Every `🧬️mutations/<kind>/🔺️diff` leaf's test region should call this
/// with two diffs of its own artifact known to have been produced by sequential mutations.
///
/// 🧊️ Every intermediate projection goes through [`crate::MutationDiff::retire_projection`] and the absorbed diff through
/// [`crate::MutationDiff::retire_cold`], so one law serves plain and fail-closed (`OrderedMap`) artifacts alike.
pub async fn assert_mutation_diff_absorb_law<P, D>(base: &P, d1: D, d2: D)
where
    P: Clone + PartialEq + std::fmt::Debug,
    D: crate::ColdMutationDiff<P> + Clone,
{
    let mid = protocol::apply_diff_cold(&d1, base).expect("first valid diff must apply");
    let sequential = protocol::apply_diff_cold(&d2, &mid);
    D::retire_projection(mid);
    let mut absorbed = d1;
    absorbed.absorb(d2);
    let composed = protocol::apply_diff_cold(&absorbed, base);
    absorbed.retire_cold();
    let matches = composed == sequential;
    let report = matches.then(String::new).unwrap_or_else(|| format!("composed={composed:?} sequential={sequential:?}"));
    for projection in [composed, sequential].into_iter().flatten() {
        D::retire_projection(projection);
    }
    assert!(matches, "absorb(d1, d2).apply(base) must equal d2.apply(&d1.apply(base)); {report}");
}

/// ✅️ LAW: applying `mutation`'s `Mutation::inverse(base)` (in reverse order, matching
/// `ArtifactStore::replay_mutations`'s own `back.reverse()`) after `mutation` restores `base`. The
/// per-`MutationKind` version of `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`'s
/// core requirement: every handcrafted mutation implements a real inverse, not a sentinel.
///
/// 🧊️ This law is itself a generic seam that MANUFACTURES operations, diffs and projections and throws them away, so it
/// retires every one of them exactly like the store folds do: each raised diff through
/// [`crate::MutationDiff::retire_cold`], each intermediate or restored projection through
/// [`crate::MutationDiff::retire_projection`] and every minted inverse operation through
/// [`crate::Mutation::retire_cold`]. Their defaults ARE a plain drop, so nothing changes for a technology built out of
/// plain values; one whose projection or diff owns a fail-closed root (an `OrderedMap`, a neural `Dictionary`) overrides them and
/// this one law is correct for it without a separate variant. `base` belongs to the caller and is never retired here.
pub async fn assert_mutation_inverse_law<P, Op>(base: &P, mutation: &Op)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P> + crate::ColdMutation<P>,
{
    use crate::MutationDiff;
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let applied = (!rejected).then(|| protocol::apply_diff_cold(&forward, base));
    forward.retire_cold();
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let mut state = applied.expect("an unrejected forward outcome is applied").expect("valid forward diff must apply");
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        let (delta, _) = undo.diff(&state).into_parts();
        let next = protocol::apply_diff_cold(&delta, &state).expect("valid inverse diff must apply");
        delta.retire_cold();
        <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(std::mem::replace(&mut state, next));
    }
    let restored = state == *base;
    let report = restored.then(String::new).unwrap_or_else(|| format!("restored={state:?} base={base:?}"));
        <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(state);
for undo in backward {
        Op::retire_cold(undo);
    }
    assert!(restored, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
}



/// ✅️ LAW (L3): the concrete inverse of `mutation` sums to the negative of its diff. With `d = mutation.diff(base)`, `after =
/// apply_diff_cold(d, base)`, the inverse operations replayed in storage-reversed order build `d_k = inv_k.diff(s_{k-1})` and
/// `s_k = apply_diff_cold(d_k, s_{k-1})`, then `Σ = d_1 ⊕ … ⊕ d_n` by [`crate::ColdMutationDiff::absorb`]. Asserts (1) the inverse is
/// non-empty whenever `after != base` (no `V2-EMPTY-INVERSE`), (2) the sequential replay restores `base`, (3)
/// `apply_diff_cold(Σ, after) == base`, and (4) `canon(Σ) == canon(d.inverse(base))` with `canon(x) = absorb(default, x)` — the inverse
/// operations are concrete, not derived from the forward diff, yet their diffs sum to exactly the negative diff.
///
/// 🧊️ Every diff, intermediate projection and minted inverse operation this law builds is retired like
/// [`assert_mutation_inverse_law`] does, so one law serves plain and fail-closed (`OrderedMap`) artifacts alike. The assertions
/// run after everything is retired.
pub async fn assert_mutation_inverse_sum_law<P, Op>(mutation: &Op, base: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P> + crate::ColdMutation<P>,
    Op::ReferenceDiff: crate::ColdDiffAlgebra<P>,
{
    use crate::{ColdDiffAlgebra, MutationDiff};
    use semio_framework_value::ToValue;
    fn canon<P, D: MutationDiff<P>>(diff: D) -> D {
        let mut canonical = D::default();
        canonical.absorb(diff);
        canonical
    }
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    if rejected {
        forward.retire_cold();
        panic!("a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    }
    let after = protocol::apply_diff_cold(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    forward.retire_cold();
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state: Option<P> = None;
    let mut sum = <Op::ReferenceDiff as Default>::default();
    for undo in &backward {
        let current = state.as_ref().unwrap_or(&after);
        let (step, _) = undo.diff(current).into_parts();
        let next = protocol::apply_diff_cold(&step, current).expect("valid inverse diff must apply");
        sum.absorb(step);
        if let Some(displaced) = state.replace(next) {
            <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(displaced);
        }
    }
    for undo in backward {
        Op::retire_cold(undo);
    }
    let restored = state.unwrap_or_else(|| after.clone());
    let replayed = restored == *base;
    let report_state = format!("{restored:?}");
    <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(restored);
    let summed = protocol::apply_diff_cold(&sum, &after);
    let summed_restores = summed.as_ref() == Ok(base);
    if let Ok(projection) = summed {
        <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(projection);
    }
    <Op::ReferenceDiff as MutationDiff<P>>::retire_projection(after);
    let canonical_sum = canon::<P, Op::ReferenceDiff>(sum);
    let canonical_negative = canon::<P, Op::ReferenceDiff>(negative);
    let matches_negative = !changed || canonical_sum == canonical_negative;
    let report = format!("sum={:?} negative={:?} restored={report_state} base={base:?}", canonical_sum.to_value(), canonical_negative.to_value());
    canonical_sum.retire_cold();
    canonical_negative.retire_cold();
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state; base={base:?}");
    assert!(replayed, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed_restores, "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(matches_negative, "the summed inverse diffs must equal the negative of the forward diff, d.inverse(base); {report}");
}

/// ✅️ LAW: `d.inverse(base).apply(&d.apply(base)) == *base` — [`crate::DiffAlgebra`]'s
/// diff-level undo, independent of any `Mutation` that might have produced `d`.
pub async fn assert_diff_algebra_inverse_law<P, D>(base: &P, d: &D)
where
    P: Clone + PartialEq + std::fmt::Debug,
    D: crate::ColdDiffAlgebra<P> + crate::ColdMutationDiff<P>,
{
    let after = protocol::apply_diff_cold(d, base).expect("valid diff must apply");
    let restored = protocol::apply_diff_cold(&d.inverse(base), &after);
    assert_eq!(restored.as_ref(), Ok(base), "d.inverse(base).apply(&d.apply(base)) must equal base");
}
//#endregion 🔖️Laws
