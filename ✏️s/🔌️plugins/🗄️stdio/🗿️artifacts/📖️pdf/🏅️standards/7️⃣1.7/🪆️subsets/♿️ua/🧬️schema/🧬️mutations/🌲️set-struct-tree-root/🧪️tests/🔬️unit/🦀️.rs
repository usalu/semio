use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = SetStructTreeRoot {};
    let next = applied(&base, &PdfUaMutation::SetStructTreeRoot(mutation.clone()));
    let Some(PdfObject::Ref(id)) = support::catalog_entry(&next, "StructTreeRoot") else { panic!("structure tree root reference") };
    assert_eq!(support::object(&next, *id), Some(&support::struct_tree_root_object()));
    assert_eq!(<SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {}), &base).await;
}
