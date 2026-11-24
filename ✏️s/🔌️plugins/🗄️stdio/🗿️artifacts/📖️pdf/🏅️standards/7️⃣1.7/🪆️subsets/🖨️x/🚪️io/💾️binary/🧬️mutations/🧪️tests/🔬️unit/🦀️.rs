use crate::standards::v1_7::subsets::x::schema::mutations::{InsertEncryptionDictionary, PdfXMutation};
use protocol::OpBinary;

#[test]
fn pack_document_round_trips_and_refuses_damaged_bytes() {
    let mutation = PdfXMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 0, revision: 0, placements: Vec::new() });
    let bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfXMutation::decode_op(&bytes).unwrap(), mutation);
    assert_eq!(PdfXMutation::decode_op(&bytes).unwrap().encode_op().unwrap(), bytes);
    let mut damaged = bytes.clone();
    damaged[0] ^= 0xff;
    assert!(PdfXMutation::decode_op(&damaged).is_err());
    assert!(PdfXMutation::decode_op(&bytes[..bytes.len() - 1]).is_err());
}
