use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveDictEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-dict-entry");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🚫️remove-dict-entry/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🚫️remove-dict-entry/➡️after.pdf"));
    let catalog = support::catalog_id(&before).expect("the seed has a catalog");
    let before = support::applied(&before, &PdfMutation::SetDictEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetDictEntry { id: catalog, path: Vec::new(), key: "Marker".to_string(), value: PdfObject::Int(7), index: None }));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveDictEntry(RemoveDictEntry { id: support::catalog_id(&before).expect("the seed has a catalog"), path: Vec::new(), key: "Marker".to_string() }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🚫️remove-dict-entry/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🚫️remove-dict-entry/➡️after.pdf"));
    let catalog = support::catalog_id(&base).expect("the seed has a catalog");
    let set = |base: &PdfSnapshot, key: &str, index: Option<usize>| support::applied(base, &PdfMutation::SetDictEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetDictEntry { id: catalog, path: Vec::new(), key: key.to_string(), value: PdfObject::Int(7), index }));
    let base = set(&set(&set(&base, "First", None), "Last", None), "Marker", Some(support::entry_position(&base, catalog, "Type").map_or(0, |at| at + 1)));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveDictEntry(RemoveDictEntry { id: catalog, path: Vec::new(), key: "Marker".to_string() }), &base).await;
}
