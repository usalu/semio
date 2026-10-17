use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::RemoveDisplayDocTitle;
use crate::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = testkit::document_of(vec![testkit::catalog_object()]);
    let mutation = SetDisplayDocTitle { display: true, entry_index: None };
    let next = applied(&base, &PdfUaMutation::SetDisplayDocTitle(mutation.clone()));
    assert_eq!(support::catalog_flag(&next, "ViewerPreferences", "DisplayDocTitle"), Some(true));
    assert_eq!(<SetDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true, entry_index: None }), &base).await;
}
