use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry::SetTrailerEntry;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_trailer_entry::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveTrailerEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-trailer-entry");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧽️remove-trailer-entry/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧽️remove-trailer-entry/➡️after.pdf"));
    let before = crate::standards::v1_7::subsets::base::schema::testkit::applied(&before, &PdfMutation::SetTrailerEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetTrailerEntry { key: "Marker".to_string(), value: PdfObject::Bool(true), index: None }));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: "Marker".to_string() }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧽️remove-trailer-entry/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🧽️remove-trailer-entry/➡️after.pdf"));
    let set = |base: &PdfSnapshot, key: &str, index: Option<usize>| crate::standards::v1_7::subsets::base::schema::testkit::applied(base, &PdfMutation::SetTrailerEntry(crate::standards::v1_7::subsets::base::schema::mutations::SetTrailerEntry { key: key.to_string(), value: PdfObject::Bool(true), index }));
    let base = set(&set(&set(&base, "First", None), "Last", None), "Marker", Some(1));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: "Marker".to_string() }), &base).await;
}
