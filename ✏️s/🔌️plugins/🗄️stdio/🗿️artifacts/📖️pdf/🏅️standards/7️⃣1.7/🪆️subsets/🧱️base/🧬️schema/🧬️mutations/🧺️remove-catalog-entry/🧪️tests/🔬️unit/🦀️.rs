use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveCatalogEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-catalog-entry");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧺️remove-catalog-entry/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧺️remove-catalog-entry/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key: testkit::removed(&before.catalog_extra, &after.catalog_extra).key }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧺️remove-catalog-entry/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧺️remove-catalog-entry/➡️after.pdf"));
    let set = |base: &PdfSnapshot, key: &str, index: Option<usize>| crate::standards::v1_7::subsets::base::schema::testkit::applied(base, &PdfMutation::SetCatalogEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetCatalogEntry { key: key.to_string(), value: PdfObject::Int(7), index }));
    let base = set(&set(&set(&base, "First", None), "Last", None), "Marker", Some(1));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key: "Marker".to_string() }), &base).await;
}
