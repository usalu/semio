use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemovePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-page");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🗑️remove-page/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🗑️remove-page/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemovePage(RemovePage { index: 0 }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🗑️remove-page/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🗑️remove-page/➡️after.pdf"));
    for (offset, page) in base.pages.iter_mut().enumerate() {
        page.media_box[2] += offset as f64;
    }
    let mut first = base.pages[0].clone();
    first.media_box[2] += 100.0;
    let mut last = base.pages[0].clone();
    last.media_box[2] += 200.0;
    base.pages.insert(0, first);
    base.pages.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemovePage(RemovePage { index: 1 }), &base).await;
}
