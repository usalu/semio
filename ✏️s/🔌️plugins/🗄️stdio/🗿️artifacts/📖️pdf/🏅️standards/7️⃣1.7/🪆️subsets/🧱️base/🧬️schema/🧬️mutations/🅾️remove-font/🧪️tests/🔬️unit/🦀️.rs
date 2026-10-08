use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveFont as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-font");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🅾️remove-font/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🅾️remove-font/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveFont(RemoveFont { id: support::removed(&before.fonts, &after.fonts).id }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🅾️remove-font/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🅾️remove-font/➡️after.pdf"));
    let target = support::removed(&base.fonts, &after.fonts);
    let mut first = target.clone();
    first.id = "first".to_string();
    let mut last = target.clone();
    last.id = "last".to_string();
    base.fonts.insert(0, first);
    base.fonts.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveFont(RemoveFont { id: target.id.clone() }), &base).await;
}
