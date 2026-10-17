use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject, PdfSnapshot};
use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::remove_object::RemoveObject;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::insert_object::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<InsertObject as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-object");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📦️insert-object/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/📦️insert-object/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::InsertObject(InsertObject { id: ObjRef { num: 900, gen: 0 }, value: PdfObject::Int(7), index: None, admitted_stream_roles: None }), &before).await;
}
