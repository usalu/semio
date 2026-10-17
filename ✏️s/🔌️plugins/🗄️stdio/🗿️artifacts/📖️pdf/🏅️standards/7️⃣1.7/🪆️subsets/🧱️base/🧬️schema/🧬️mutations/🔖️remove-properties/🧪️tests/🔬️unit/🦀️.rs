use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_properties::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveProperties as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-properties");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🔖️remove-properties/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🔖️remove-properties/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveProperties(RemoveProperties { name: testkit::removed(&before.properties, &after.properties).name }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🔖️remove-properties/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🔖️remove-properties/➡️after.pdf"));
    let target = testkit::removed(&base.properties, &after.properties);
    let mut first = target.clone();
    first.name = "first".to_string();
    let mut last = target.clone();
    last.name = "last".to_string();
    base.properties.insert(0, first);
    base.properties.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveProperties(RemoveProperties { name: target.name.clone() }), &base).await;
}
