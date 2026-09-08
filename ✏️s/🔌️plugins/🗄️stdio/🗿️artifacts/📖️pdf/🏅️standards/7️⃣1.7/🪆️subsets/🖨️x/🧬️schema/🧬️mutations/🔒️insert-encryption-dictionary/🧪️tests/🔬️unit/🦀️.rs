
use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_requested_security_handler() {
    let base = PdfSnapshot::default();
    let mutation = InsertEncryptionDictionary { version: 2, revision: 3 };
    let outcome = <InsertEncryptionDictionary as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_some());
}
