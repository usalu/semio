use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetPageMediaBox as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-media-box");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/📐️set-page-media-box/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/📐️set-page-media-box/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetPageMediaBox(SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 300.0, 400.0] }), &before).await;
}
