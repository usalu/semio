use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetProperties as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-properties");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🏷️set-properties/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🏷️set-properties/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetProperties(SetProperties { properties: support::added(&before.properties, &after.properties), index: None }), &before).await;
}
