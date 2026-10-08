use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveShading as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-shading");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🌄️remove-shading/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🌄️remove-shading/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveShading(RemoveShading { id: support::removed(&before.shadings, &after.shadings).id }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🌄️remove-shading/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🌄️remove-shading/➡️after.pdf"));
    let target = support::removed(&base.shadings, &after.shadings);
    let mut first = target.clone();
    first.id = "first".to_string();
    let mut last = target.clone();
    last.id = "last".to_string();
    base.shadings.insert(0, first);
    base.shadings.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveShading(RemoveShading { id: target.id.clone() }), &base).await;
}
