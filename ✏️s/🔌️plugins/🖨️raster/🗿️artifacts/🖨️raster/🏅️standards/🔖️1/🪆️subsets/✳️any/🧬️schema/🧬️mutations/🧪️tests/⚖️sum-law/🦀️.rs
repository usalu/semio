//! ⚖️ The Raster twin of `assert_mutation_inverse_sum_law`: the same four assertions, but every displaced projection and every
//! cold diff is closed through its owner's retirement route, because a populated `RasterOwnedMap` aborts on a bare drop.
use crate::diff::RasterDiff;
use crate::{RasterMutation, RasterSnapshot};
use protocol::{DiffAlgebra, Mutation, MutationDiff};

fn canon(diff: RasterDiff) -> RasterDiff {
    let mut canonical = RasterDiff::default();
    canonical.absorb(diff);
    canonical
}

fn retire(snapshot: RasterSnapshot) {
    <RasterDiff as MutationDiff<RasterSnapshot>>::retire_projection(snapshot);
}

/// ✅️ L3 for one Raster mutation on `base`: the concrete inverse is non-empty when state changes, replays back to `base`, its
/// diffs absorb into a diff that restores `base` from the applied state, and that sum equals `forward.inverse(base)`.
pub(crate) async fn assert_raster_inverse_sum_law(mutation: &RasterMutation, base: &RasterSnapshot) {
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {messages:?}");
    let after = protocol::apply_diff(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    MutationDiff::<RasterSnapshot>::retire_cold(forward);
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state = protocol::apply_diff(&RasterDiff::default(), &after).expect("empty diff applies");
    let mut sum = RasterDiff::default();
    for undo in &backward {
        let (step, _) = undo.diff(&state).into_parts();
        let next = protocol::apply_diff(&step, &state).expect("valid inverse diff must apply");
        sum.absorb(step);
        retire(std::mem::replace(&mut state, next));
    }
    for undo in backward {
        <RasterMutation as Mutation<RasterSnapshot>>::retire_cold(undo);
    }
    let replayed = state == *base;
    let summed = protocol::apply_diff(&sum, &after);
    let summed_restores = summed.as_ref().is_ok_and(|restored| restored == base);
    let canonical_sum = canon(sum);
    let canonical_negative = canon(negative);
    let matches_negative = !changed || canonical_sum == canonical_negative;
    let report = format!("sum={canonical_sum:?} negative={canonical_negative:?}");
    MutationDiff::<RasterSnapshot>::retire_cold(canonical_sum);
    MutationDiff::<RasterSnapshot>::retire_cold(canonical_negative);
    retire(state);
    retire(after);
    if let Ok(restored) = summed {
        retire(restored);
    }
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state");
    assert!(replayed, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed_restores, "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(matches_negative, "the summed inverse diffs must equal the negative of the forward diff, d.inverse(base); {report}");
}
