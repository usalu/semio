use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let catalog = support::document_of(vec![support::catalog_object()]);
    let base = support::after_rows(&catalog, support::dpart_root_rows(&catalog, "before"));
    let mutation = SetDpartMetadata { job: "after".to_string() };
    let next = applied(&base, &PdfVtMutation::SetDpartMetadata(mutation.clone()));
    assert_eq!(support::dpart_job(&next).as_deref(), Some("after"));
    assert_eq!(<SetDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "before".to_string() })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = { let catalog = support::document_of(vec![support::catalog_object()]); support::after_rows(&catalog, support::dpart_root_rows(&catalog, "before")) };
    assert_mutation_inverse_sum_law(&PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "after".to_string() }), &base).await;
}
