use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let catalog = support::document_of(vec![support::catalog_object()]);
    let base = support::after_rows(&catalog, support::dpart_root_rows(&catalog, "run 4711", &[], None));
    let mutation = RemoveDpartRoot {};
    let next = applied(&base, &PdfVtMutation::RemoveDpartRoot(mutation.clone()));
    assert!(support::catalog_entry(&next, "DPartRoot").is_none());
    assert_eq!(next, catalog, "the partition objects leave with the root entry");
    assert_eq!(<RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {}), &base).await;
}
