use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetAnnotation as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-annotation");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/📝️set-annotation/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/📝️set-annotation/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetAnnotation(SetAnnotation { index: 0, at: 0, annotation: after.pages[0].annotations[0].clone() }), &before).await;
}
