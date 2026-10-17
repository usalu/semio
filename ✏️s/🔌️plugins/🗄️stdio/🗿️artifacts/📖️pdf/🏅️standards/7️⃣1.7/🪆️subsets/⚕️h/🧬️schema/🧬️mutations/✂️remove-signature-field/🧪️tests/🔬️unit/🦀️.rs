use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::h::schema::mutations::insert_signature_field::InsertSignatureField;
use crate::standards::v1_7::subsets::h::schema::mutations::PdfHMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_and_can_restore_the_named_signature_field() {
    let catalog = testkit::document_of(vec![testkit::catalog_object()]);
    let base = applied(&catalog, &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }));
    let mutation = RemoveSignatureField { name: "Signature1".to_string() };
    let next = applied(&base, &PdfHMutation::RemoveSignatureField(mutation.clone()));
    assert!(support::signature_field_named(&next, &mutation.name).is_none());
    assert_eq!(next, catalog, "the last field takes the whole form with it");
    assert_eq!(<RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: "Signature1".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: "Signature1".to_string() }), &base).await;
}
