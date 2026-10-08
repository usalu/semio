use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_and_can_restore_the_named_signature_field() {
    let catalog = support::document_of(vec![support::catalog_object()]);
    let base = applied(&catalog, &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }));
    let mutation = RemoveSignatureField { name: "Signature1".to_string() };
    let next = applied(&base, &PdfHMutation::RemoveSignatureField(mutation.clone()));
    assert!(support::signature_field_named(&next, &mutation.name).is_none());
    assert_eq!(next, catalog, "the last field takes the whole form with it");
    assert_eq!(<RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: "Signature1".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: "Signature1".to_string() }), &base).await;
}
