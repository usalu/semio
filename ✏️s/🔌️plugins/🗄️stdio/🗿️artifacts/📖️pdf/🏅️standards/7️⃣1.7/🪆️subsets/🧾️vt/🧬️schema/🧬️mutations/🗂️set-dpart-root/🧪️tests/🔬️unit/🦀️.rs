use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = SetDpartRoot { job: "run 4711".to_string() };
    let next = applied(&base, &PdfVtMutation::SetDpartRoot(mutation.clone()));
    assert!(support::catalog_entry(&next, "DPartRoot").is_some());
    assert_eq!(support::dpart_job(&next).as_deref(), Some("run 4711"));
    assert_eq!(<SetDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string() }), &base).await;
}
