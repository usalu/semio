use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetShading as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-shading");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🌅️set-shading/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🌅️set-shading/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetShading(SetShading { shading: support::added(&before.shadings, &after.shadings), index: None }), &before).await;
}
