use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::a::schema::mutations::set_af_relationship::SetAfRelationship;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_and_can_restore_the_relationship() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"attached payload for measurements.csv".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("Filespec".to_string())), ("F", support::literal("measurements.csv")), ("UF", support::literal("measurements.csv")), ("EF", support::single_entry_dict("F", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))), ("AFRelationship", PdfObject::Name("Data".to_string()))])]);
    let mutation = RemoveAfRelationship { file_name: "measurements.csv".to_string() };
    let next = applied(&base, &PdfAMutation::RemoveAfRelationship(mutation.clone()));
    let next_id = support::file_spec_named(&next, &mutation.file_name).unwrap();
    assert!(support::object(&next, next_id).and_then(|value| support::dict_name(value, "AFRelationship")).is_none());
    assert_eq!(<RemoveAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"attached payload for measurements.csv".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("Filespec".to_string())), ("F", support::literal("measurements.csv")), ("UF", support::literal("measurements.csv")), ("EF", support::single_entry_dict("F", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))), ("AFRelationship", PdfObject::Name("Data".to_string()))])]);
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveAfRelationship(RemoveAfRelationship { file_name: "measurements.csv".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"attached payload for measurements.csv".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("Filespec".to_string())), ("F", support::literal("measurements.csv")), ("UF", support::literal("measurements.csv")), ("EF", support::single_entry_dict("F", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))), ("AFRelationship", PdfObject::Name("Data".to_string()))])]);
    let base = testkit::with_trailing_entry(&base, support::file_spec_named(&base, "measurements.csv").expect("the fixture holds the file spec"));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveAfRelationship(RemoveAfRelationship { file_name: "measurements.csv".to_string() }), &base).await;
}
