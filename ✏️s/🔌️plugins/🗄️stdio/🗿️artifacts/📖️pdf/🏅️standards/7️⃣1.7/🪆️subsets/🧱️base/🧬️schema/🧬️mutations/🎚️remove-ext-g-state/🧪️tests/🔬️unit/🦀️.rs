use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveExtGState as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-ext-g-state");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🎚️remove-ext-g-state/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🎚️remove-ext-g-state/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveExtGState(RemoveExtGState { id: support::removed(&before.ext_g_states, &after.ext_g_states).id }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🎚️remove-ext-g-state/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🎚️remove-ext-g-state/➡️after.pdf"));
    let target = support::removed(&base.ext_g_states, &after.ext_g_states);
    let mut first = target.clone();
    first.id = "first".to_string();
    let mut last = target.clone();
    last.id = "last".to_string();
    base.ext_g_states.insert(0, first);
    base.ext_g_states.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveExtGState(RemoveExtGState { id: target.id.clone() }), &base).await;
}
