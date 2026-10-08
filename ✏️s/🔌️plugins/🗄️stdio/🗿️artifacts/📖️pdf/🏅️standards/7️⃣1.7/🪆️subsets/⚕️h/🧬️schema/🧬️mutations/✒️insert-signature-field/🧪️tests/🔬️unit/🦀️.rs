use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_named_signature_field() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None };
    let next = applied(&base, &PdfHMutation::InsertSignatureField(mutation.clone()));
    assert!(support::signature_field_named(&next, &mutation.name).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = support::with_tail(&support::document());
    let mutation = PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: vec![support::placed(98, 1), support::placed(99, 2)], field_index: None, entry_index: None });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
