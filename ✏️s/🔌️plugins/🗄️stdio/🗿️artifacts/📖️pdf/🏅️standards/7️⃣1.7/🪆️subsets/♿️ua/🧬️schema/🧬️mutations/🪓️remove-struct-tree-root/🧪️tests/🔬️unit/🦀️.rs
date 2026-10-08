use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Catalog".to_string())), ("StructTreeRoot", PdfObject::Ref(ObjRef { num: 2, gen: 0 }))]), support::struct_tree_root_object()]);
    let mutation = RemoveStructTreeRoot {};
    let next = applied(&base, &PdfUaMutation::RemoveStructTreeRoot(mutation.clone()));
    assert!(support::catalog_entry(&next, "StructTreeRoot").is_none());
    assert_eq!(next.objects.len(), 1, "the root leaves with its catalog entry");
    assert_eq!(<RemoveStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {}));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {}), &base).await;
}
