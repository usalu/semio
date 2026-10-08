use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetObjectValue as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-object-value");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🔧️set-object-value/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🔧️set-object-value/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetObjectValue(SetObjectValue { id: before.objects.last().expect("the seed holds objects").id, value: PdfObject::Int(7), index: None }), &before).await;
}
