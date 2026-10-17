use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::h::schema::mutations::PdfHMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::RemoveSignatureField;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::h::schema::mutations::insert_signature_field::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_named_signature_field() {
    let base = testkit::document_of(vec![testkit::catalog_object()]);
    let mutation = InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None };
    let next = applied(&base, &PdfHMutation::InsertSignatureField(mutation.clone()));
    assert!(support::signature_field_named(&next, &mutation.name).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: Vec::new(), field_index: None, entry_index: None }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfHMutation::InsertSignatureField(InsertSignatureField { name: "Signature1".to_string(), placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)], field_index: None, entry_index: None });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
