use super::*;

#[test]
fn canonical_framing_round_trips_and_checks_identity() {
    let mutation = PdfVtMutation::InsertEncryptionDictionary(crate::standards::v1_7::subsets::vt::schema::mutations::InsertEncryptionDictionary { version: 0, revision: 0, placements: Vec::new() });
    let mut bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfVtMutation::decode_op(&bytes).unwrap(), mutation);
    bytes[1] = crate::standards::v1_7::subsets::vt::io::binary::mutations::remove_encryption_dictionary::TAG;
    assert!(PdfVtMutation::decode_op(&bytes).is_err());
}
