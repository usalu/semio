use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<InsertPage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-page");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/📥️insert-page/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/📥️insert-page/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = crate::standards::v1_7::subsets::base::io::text_document(&[(200.0, 100.0, "a"), (300.0, 100.0, "b"), (400.0, 100.0, "c")]);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }), &base).await;
}
