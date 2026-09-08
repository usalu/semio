
use super::*;

#[test]
fn canonical_framing_round_trips_and_checks_identity() {
    let mutation = PdfVtMutation::InsertEncryptionDictionary(super::super::InsertEncryptionDictionary { version: 0, revision: 0 });
    let mut bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfVtMutation::decode_op(&bytes).unwrap(), mutation);
    bytes[1] = super::super::remove_encryption_dictionary::binary::TAG;
    assert!(PdfVtMutation::decode_op(&bytes).is_err());
}
