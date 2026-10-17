use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::x::schema::mutations::remove_trim_box::RemoveTrimBox;
use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::x::schema::mutations::set_trim_box::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let base = testkit::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string()))])]);
    let page = ObjRef { num: 1, gen: 0 };
    let mutation = SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0], entry_index: None };
    let next = applied(&base, &PdfXMutation::SetTrimBox(mutation.clone()));
    assert_eq!(support::page_box(&next, page, "TrimBox"), Some([1.0, 2.0, 300.0, 400.0]));
    assert_eq!(<SetTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfXMutation::RemoveTrimBox(RemoveTrimBox { page_index: 0 })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string()))])]);
    assert_mutation_inverse_sum_law(&PdfXMutation::SetTrimBox(SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0], entry_index: None }), &base).await;
}
