
use super::*;
use protocol::MutationDiff;

#[test]
fn removes_and_can_restore_the_named_signature_field() {
    let mut base = PdfSnapshot::default();
    support::insert_signature_field(&mut base, "Signature1");
    let mutation = RemoveSignatureField { name: "Signature1".to_string() };
    let outcome = <RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::signature_field_named(&next, &mutation.name).is_none());
    assert_eq!(<RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).len(), 1);
}
