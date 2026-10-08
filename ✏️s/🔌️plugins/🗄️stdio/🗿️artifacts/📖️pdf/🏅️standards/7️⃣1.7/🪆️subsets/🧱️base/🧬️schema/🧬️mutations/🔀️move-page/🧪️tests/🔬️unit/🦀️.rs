use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<MovePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "move-page");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🔀️move-page/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🔀️move-page/➡️after.pdf"));
    let before = crate::standards::v1_7::subsets::base::io::mutation_bridge::applied(&before, &PdfMutation::InsertPage(crate::standards::v1_7::subsets::base::schema::mutations::InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }));
    assert_mutation_inverse_sum_law(&PdfMutation::MovePage(MovePage { from: 0, to: 1 }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = crate::standards::v1_7::subsets::base::io::text_document(&[(200.0, 100.0, "a"), (300.0, 100.0, "b"), (400.0, 100.0, "c")]);
    assert_mutation_inverse_sum_law(&PdfMutation::MovePage(MovePage { from: 0, to: 1 }), &base).await;
    assert_mutation_inverse_sum_law(&PdfMutation::MovePage(MovePage { from: 2, to: 0 }), &base).await;
}
