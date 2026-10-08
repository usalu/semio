use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<InsertContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-content");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🖋️insert-content/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🖋️insert-content/➡️after.pdf"));
    let (at, _, content) = support::content_edit(&before, &after);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertContent(InsertContent { index: 0, at, content }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🖋️insert-content/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🖋️insert-content/➡️after.pdf"));
    let (at, _, content) = support::content_edit(&base, &after);
    let op = content[0].clone();
    base.pages[0].content.insert(0, op.clone());
    base.pages[0].content.push(op);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertContent(InsertContent { index: 0, at: at + 1, content }), &base).await;
}
