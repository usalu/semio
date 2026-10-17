use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_annotation::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveAnnotation as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-annotation");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveAnnotation(RemoveAnnotation { index: 0, at: 0 }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/➡️after.pdf"));
    let annotation = base.pages[0].annotations[0].clone();
    base.pages[0].annotations.insert(0, annotation.clone());
    base.pages[0].annotations.push(annotation);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveAnnotation(RemoveAnnotation { index: 0, at: 1 }), &base).await;
}
