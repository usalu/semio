use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveProperties as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-properties");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🔖️remove-properties/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🔖️remove-properties/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveProperties(RemoveProperties { name: support::removed(&before.properties, &after.properties).name }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🔖️remove-properties/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🔖️remove-properties/➡️after.pdf"));
    let target = support::removed(&base.properties, &after.properties);
    let mut first = target.clone();
    first.name = "first".to_string();
    let mut last = target.clone();
    last.name = "last".to_string();
    base.properties.insert(0, first);
    base.properties.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveProperties(RemoveProperties { name: target.name.clone() }), &base).await;
}
