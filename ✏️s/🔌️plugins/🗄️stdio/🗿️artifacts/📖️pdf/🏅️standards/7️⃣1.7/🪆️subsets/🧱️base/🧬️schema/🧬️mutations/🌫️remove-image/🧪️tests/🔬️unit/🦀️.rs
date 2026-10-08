use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveImage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-image");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🌫️remove-image/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🌫️remove-image/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveImage(RemoveImage { id: support::removed(&before.images, &after.images).id }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🌫️remove-image/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🌫️remove-image/➡️after.pdf"));
    let target = support::removed(&base.images, &after.images);
    let mut first = target.clone();
    first.id = "first".to_string();
    let mut last = target.clone();
    last.id = "last".to_string();
    base.images.insert(0, first);
    base.images.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveImage(RemoveImage { id: target.id.clone() }), &base).await;
}
