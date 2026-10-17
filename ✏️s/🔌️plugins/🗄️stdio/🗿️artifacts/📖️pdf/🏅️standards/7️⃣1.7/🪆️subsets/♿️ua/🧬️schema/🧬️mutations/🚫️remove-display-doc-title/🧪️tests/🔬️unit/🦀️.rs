use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle;
use crate::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = testkit::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Catalog".to_string())), ("ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(true)))])]);
    let mutation = RemoveDisplayDocTitle {};
    let next = applied(&base, &PdfUaMutation::RemoveDisplayDocTitle(mutation.clone()));
    assert!(support::catalog_entry(&next, "ViewerPreferences").is_none());
    assert_eq!(<RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true, entry_index: None })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true, entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true, entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {}), &base).await;
}
