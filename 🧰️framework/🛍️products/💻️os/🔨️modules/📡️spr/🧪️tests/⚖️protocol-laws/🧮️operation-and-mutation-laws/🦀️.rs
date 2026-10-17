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
/// contract every `MutationDiff::absorb` impl must satisfy (`📡️spr/🎮️command/🦀️.rs`'s
/// `🔖️Mutation` region doc). Every `🧬️mutations/<kind>/🔺️diff` leaf's test region should call this
/// with two diffs of its own artifact known to have been produced by sequential mutations.
pub async fn assert_mutation_diff_absorb_law<P, D>(base: &P, d1: D, d2: D)
where
    P: PartialEq + std::fmt::Debug,
    D: crate::MutationDiff<P> + Clone,
{
    let mid = crate::apply_diff(&d1, base).expect("first valid diff must apply");
    let sequential = crate::apply_diff(&d2, &mid);
    let mut absorbed = d1;
    absorbed.absorb(d2);
    let composed = crate::apply_diff(&absorbed, base);
    assert_eq!(composed, sequential, "absorb(d1, d2).apply(base) must equal d2.apply(&d1.apply(base))");
}

/// 🧊️ Cold twin of [`assert_mutation_diff_absorb_law`] for a projection whose owned form rejects a
/// bare drop (an `OrderedMap` root, a retirement ladder) — `retire` CLOSES every intermediate
/// projection this law materialises, and `retire_diff` closes every diff, so the law itself never
/// leaks. Same shape as `store::test_support::assert_dsl_round_trip_cold`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §2.6).
pub async fn assert_mutation_diff_absorb_law_cold<P, D>(base: &P, d1: D, d2: D, retire: impl Fn(P), retire_diff: impl Fn(D))
where
    P: PartialEq + std::fmt::Debug,
    D: crate::MutationDiff<P> + Clone,
{
    let mid = crate::apply_diff(&d1, base).expect("first valid diff must apply");
    let sequential = crate::apply_diff(&d2, &mid);
    retire(mid);
    let mut absorbed = d1;
    absorbed.absorb(d2);
    let composed = crate::apply_diff(&absorbed, base);
    retire_diff(absorbed);
    let matches = composed == sequential;
    let report = matches.then(String::new).unwrap_or_else(|| format!("composed={composed:?} sequential={sequential:?}"));
    for projection in [composed, sequential].into_iter().flatten() {
        retire(projection);
    }
    assert!(matches, "absorb(d1, d2).apply(base) must equal d2.apply(&d1.apply(base)); {report}");
}

/// ✅️ LAW: applying `mutation`'s `Mutation::inverse(base)` (in reverse order, matching
/// `ArtifactStore::replay_mutations`'s own `back.reverse()`) after `mutation` restores `base`. The
/// per-`MutationKind` version of `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`'s
/// core requirement: every handcrafted mutation implements a real inverse, not a sentinel.
///
/// 🧊️ This law is itself a generic seam that MANUFACTURES operations and diffs and throws them
/// away: `mutation.inverse(base)` mints a whole `Vec<Op>` and every step raises a
/// `MutationOutcome<Op::Diff>`. Both are routed through [`crate::os_spr::Mutation::retire_cold`] /
/// [`crate::os_spr::MutationDiff::retire_cold`] instead of being dropped — exactly what those two
/// methods' contracts require of "every generic replay/fold seam that builds a delta and throws it
/// away". Their defaults ARE a plain drop, so nothing changes for a technology built out of plain
/// values; for one whose operation owns a fail-closed root (an `OrderedMap`, a neural
/// `Dictionary`), the bare drop used to abort the process INSIDE the law
/// (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`,
/// ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, bucket XCUT-DICT). The PROJECTION `P` is
/// still plain-dropped here — a technology whose projection also rejects a bare drop uses
/// [`assert_mutation_inverse_law_cold`].
pub async fn assert_mutation_inverse_law<P, Op>(base: &P, mutation: &Op)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P>,
{
    use crate::MutationDiff;
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let applied = (!rejected).then(|| crate::apply_diff(&forward, base));
    forward.retire_cold();
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let mut state = applied.expect("an unrejected forward outcome is applied").expect("valid forward diff must apply");
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        let (delta, _) = undo.diff(&state).into_parts();
        let next = crate::apply_diff(&delta, &state).expect("valid inverse diff must apply");
        delta.retire_cold();
        state = next;
    }
    let restored = state == *base;
    let report = restored.then(String::new).unwrap_or_else(|| format!("restored={state:?} base={base:?}"));
    for undo in backward {
        Op::retire_cold(undo);
    }
    assert!(restored, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
}

/// 🧊️ Cold twin of [`assert_mutation_inverse_law`] for a projection whose owned form rejects a bare
/// drop. Every intermediate `state` is CLOSED through `retire` and every raised
/// `MutationOutcome` is routed through `into_parts()` into `retire_diff` — the outcome OWNS its
/// diff (`📡️replication/🎮️mutation/🦀️.rs`), so dropping it leaks exactly like dropping the
/// projection does (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §2.3).
pub async fn assert_mutation_inverse_law_cold<P, Op>(base: &P, mutation: &Op, retire: impl Fn(P), retire_diff: impl Fn(Op::Diff))
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P>,
{
    use crate::MutationDiff;
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let applied = (!rejected).then(|| crate::apply_diff(&forward, base));
    retire_diff(forward);
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let mut state = applied.expect("an unrejected forward outcome is applied").expect("valid forward diff must apply");
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        let (delta, _) = undo.diff(&state).into_parts();
        let next = crate::apply_diff(&delta, &state).expect("valid inverse diff must apply");
        retire_diff(delta);
        retire(std::mem::replace(&mut state, next));
    }
    let matches = state == *base;
    let report = matches.then(String::new).unwrap_or_else(|| format!("{state:?}"));
    retire(state);
    // 🧊️ The minted inverse operations are owners too — see `assert_mutation_inverse_law`'s note.
    for undo in backward {
        Op::retire_cold(undo);
    }
    assert!(matches, "applying mutation.inverse(base) (reversed) after mutation must restore base; restored:\n{report}");
}

/// ✅️ LAW (L3): the concrete inverse of `mutation` sums to the negative of its diff. With `d = mutation.diff(base)`, `after =
/// apply_diff(d, base)`, the inverse operations replayed in storage-reversed order build `d_k = inv_k.diff(s_{k-1})` and
/// `s_k = apply_diff(d_k, s_{k-1})`, then `Σ = d_1 ⊕ … ⊕ d_n` by [`crate::os_spr::MutationDiff::absorb`]. Asserts (1) the inverse is
/// non-empty whenever `after != base` (no `V2-EMPTY-INVERSE`), (2) the sequential replay restores `base`, (3)
/// `apply_diff(Σ, after) == base`, and (4) `canon(Σ) == canon(d.inverse(base))` with `canon(x) = absorb(default, x)` — the inverse
/// operations are concrete, not derived from the forward diff, yet their diffs sum to exactly the negative diff.
pub async fn assert_mutation_inverse_sum_law<P, Op>(mutation: &Op, base: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P>,
{
    use crate::{DiffAlgebra, MutationDiff};
    use semio_framework_value::ToValue;
    fn canon<P, D: MutationDiff<P>>(diff: D) -> D {
        let mut canonical = D::default();
        canonical.absorb(diff);
        canonical
    }
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let after = crate::apply_diff(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    forward.retire_cold();
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state = after.clone();
    let mut sum = <Op::Diff as Default>::default();
    for undo in &backward {
        let (step, _) = undo.diff(&state).into_parts();
        let next = crate::apply_diff(&step, &state).expect("valid inverse diff must apply");
        sum.absorb(step);
        state = next;
    }
    for undo in backward {
        Op::retire_cold(undo);
    }
    let replayed = state == *base;
    let summed = crate::apply_diff(&sum, &after);
    let summed_restores = summed.as_ref() == Ok(base);
    let canonical_sum = canon::<P, Op::Diff>(sum);
    let canonical_negative = canon::<P, Op::Diff>(negative);
    let matches_negative = !changed || canonical_sum == canonical_negative;
    let report = format!("sum={:?} negative={:?} restored={state:?} base={base:?}", canonical_sum.to_value(), canonical_negative.to_value());
    canonical_sum.retire_cold();
    canonical_negative.retire_cold();
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state; base={base:?}");
    assert!(replayed, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed_restores, "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(matches_negative, "the summed inverse diffs must equal the negative of the forward diff, d.inverse(base); {report}");
}

/// 🧊️ Cold twin of [`assert_mutation_inverse_sum_law`] for a projection whose owned form rejects a bare drop (an `OrderedMap`
/// root, a retirement ladder): every intermediate projection is CLOSED through `retire` and every diff this law raises through
/// `retire_diff`, and the minted inverse operations through `Mutation::retire_cold`, so the law itself never plain-drops a fail-closed
/// owner. The assertions are the same four as the plain law; they run after everything is retired.
pub async fn assert_mutation_inverse_sum_law_cold<P, Op>(mutation: &Op, base: &P, retire: impl Fn(P), retire_diff: impl Fn(Op::Diff))
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::Mutation<P>,
{
    use crate::{DiffAlgebra, MutationDiff};
    use semio_framework_value::ToValue;
    fn canon<P, D: MutationDiff<P>>(diff: D) -> D {
        let mut canonical = D::default();
        canonical.absorb(diff);
        canonical
    }
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    if rejected {
        retire_diff(forward);
        panic!("a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    }
    let after = crate::apply_diff(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    retire_diff(forward);
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state = None;
    let mut sum = <Op::Diff as Default>::default();
    for undo in &backward {
        let current = state.as_ref().unwrap_or(&after);
        let (step, _) = undo.diff(current).into_parts();
        let next = crate::apply_diff(&step, current).expect("valid inverse diff must apply");
        sum.absorb(step);
        if let Some(displaced) = state.replace(next) {
            retire(displaced);
        }
    }
    for undo in backward {
        Op::retire_cold(undo);
    }
    let restored = state.unwrap_or_else(|| after.clone());
    let replayed = restored == *base;
    let report_state = format!("{restored:?}");
    retire(restored);
    let summed = crate::apply_diff(&sum, &after);
    let summed_restores = summed.as_ref() == Ok(base);
    if let Ok(projection) = summed {
        retire(projection);
    }
    retire(after);
    let canonical_sum = canon::<P, Op::Diff>(sum);
    let canonical_negative = canon::<P, Op::Diff>(negative);
    let matches_negative = !changed || canonical_sum == canonical_negative;
    let report = format!("sum={:?} negative={:?} restored={report_state} base={base:?}", canonical_sum.to_value(), canonical_negative.to_value());
    retire_diff(canonical_sum);
    retire_diff(canonical_negative);
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state; base={base:?}");
    assert!(replayed, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed_restores, "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(matches_negative, "the summed inverse diffs must equal the negative of the forward diff, d.inverse(base); {report}");
}

/// ✅️ LAW: `d.inverse(base).apply(&d.apply(base)) == *base` — [`crate::os_spr::DiffAlgebra`]'s
/// diff-level undo, independent of any `Mutation` that might have produced `d`.
pub async fn assert_diff_algebra_inverse_law<P, D>(base: &P, d: &D)
where
    P: Clone + PartialEq + std::fmt::Debug,
    D: crate::DiffAlgebra<P> + crate::MutationDiff<P>,
{
    let after = crate::apply_diff(d, base).expect("valid diff must apply");
    let restored = crate::apply_diff(&d.inverse(base), &after);
    assert_eq!(restored.as_ref(), Ok(base), "d.inverse(base).apply(&d.apply(base)) must equal base");
}
//#endregion 🔖️Laws
