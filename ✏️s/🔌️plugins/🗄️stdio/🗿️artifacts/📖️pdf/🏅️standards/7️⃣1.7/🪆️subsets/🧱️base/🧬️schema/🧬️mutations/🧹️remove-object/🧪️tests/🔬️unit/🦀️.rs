use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveObject as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-object");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🧹️remove-object/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🧹️remove-object/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveObject(RemoveObject { id: before.objects.last().expect("the seed holds objects").id, admitted_stream_roles: None }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🧹️remove-object/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🧹️remove-object/➡️after.pdf"));
    let id = support::next_object_id(&base);
    let base = crate::standards::v1_7::subsets::base::io::mutation_bridge::applied(&base, &PdfMutation::InsertObject(crate::standards::v1_7::subsets::base::schema::mutations::InsertObject { id, value: PdfObject::Int(7), index: Some(1), admitted_stream_roles: None }));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveObject(RemoveObject { id, admitted_stream_roles: None }), &base).await;
}
