
use super::*;
use protocol::MutationDiff;

#[test]
fn removes_only_a_present_security_handler() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::encryption_dictionary(2, 3));
    let mutation = RemoveEncryptionDictionary { version: 2, revision: 3 };
    let outcome = <RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
    assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).len(), 1);
}
