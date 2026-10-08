use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string())), ("TrimBox", support::box_object([1.0, 2.0, 300.0, 400.0]))])]);
    let page = ObjRef { num: 1, gen: 0 };
    let mutation = RemoveTrimBox { page_index: 0 };
    let next = applied(&base, &PdfXMutation::RemoveTrimBox(mutation.clone()));
    assert!(support::page_box(&next, page, "TrimBox").is_none());
    assert_eq!(<RemoveTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfXMutation::SetTrimBox(SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0] })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string())), ("TrimBox", support::box_object([1.0, 2.0, 300.0, 400.0]))])]);
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveTrimBox(RemoveTrimBox { page_index: 0 }), &base).await;
}
