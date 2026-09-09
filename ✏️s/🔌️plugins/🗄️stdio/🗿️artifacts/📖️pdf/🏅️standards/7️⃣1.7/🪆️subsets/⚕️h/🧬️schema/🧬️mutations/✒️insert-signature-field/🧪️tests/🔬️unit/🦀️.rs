use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_named_signature_field() {
    let base = PdfSnapshot::default();
    let mutation = InsertSignatureField { name: "Signature1".to_string() };
    let outcome = <InsertSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::signature_field_named(&next, &mutation.name).is_some());
}
