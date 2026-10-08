use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveTrailerEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-trailer-entry");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🧽️remove-trailer-entry/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🧽️remove-trailer-entry/➡️after.pdf"));
    let before = crate::standards::v1_7::subsets::base::io::mutation_bridge::applied(&before, &PdfMutation::SetTrailerEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetTrailerEntry { key: "Marker".to_string(), value: PdfObject::Bool(true), index: None }));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: "Marker".to_string() }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🧽️remove-trailer-entry/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🧽️remove-trailer-entry/➡️after.pdf"));
    let set = |base: &PdfSnapshot, key: &str, index: Option<usize>| crate::standards::v1_7::subsets::base::io::mutation_bridge::applied(base, &PdfMutation::SetTrailerEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetTrailerEntry { key: key.to_string(), value: PdfObject::Bool(true), index }));
    let base = set(&set(&set(&base, "First", None), "Last", None), "Marker", Some(1));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: "Marker".to_string() }), &base).await;
}
