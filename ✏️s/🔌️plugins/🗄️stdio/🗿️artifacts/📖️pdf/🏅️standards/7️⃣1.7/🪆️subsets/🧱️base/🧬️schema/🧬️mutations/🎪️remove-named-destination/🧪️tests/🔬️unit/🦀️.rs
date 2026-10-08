use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveNamedDestination as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-named-destination");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🎪️remove-named-destination/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🎪️remove-named-destination/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name: support::removed(&before.named_destinations, &after.named_destinations).name }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🎪️remove-named-destination/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🎪️remove-named-destination/➡️after.pdf"));
    let target = support::removed(&base.named_destinations, &after.named_destinations);
    let mut first = target.clone();
    first.name = "first".to_string();
    let mut last = target.clone();
    last.name = "last".to_string();
    base.named_destinations.insert(0, first);
    base.named_destinations.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name: target.name.clone() }), &base).await;
}
