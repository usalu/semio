use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<ReplaceContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "replace-content");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🔁️replace-content/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🔁️replace-content/➡️after.pdf"));
    let (at, _, inserted) = support::content_edit(&before, &after);
    assert_mutation_inverse_sum_law(&PdfMutation::ReplaceContent(ReplaceContent { index: 0, at, op: inserted[0].clone() }), &before).await;
}
