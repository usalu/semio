use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Catalog".to_string())), ("ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(true)))])]);
    let mutation = RemoveDisplayDocTitle {};
    let next = applied(&base, &PdfUaMutation::RemoveDisplayDocTitle(mutation.clone()));
    assert!(support::catalog_entry(&next, "ViewerPreferences").is_none());
    assert_eq!(<RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true }));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {}), &base).await;
}
