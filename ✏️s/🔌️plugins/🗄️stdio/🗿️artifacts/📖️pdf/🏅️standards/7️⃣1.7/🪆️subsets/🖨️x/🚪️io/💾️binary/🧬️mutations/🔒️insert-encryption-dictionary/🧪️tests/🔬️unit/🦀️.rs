use crate::standards::v1_7::subsets::x::schema::mutations::insert_encryption_dictionary::InsertEncryptionDictionary;
use crate::standards::v1_7::subsets::x::io::binary::mutations::insert_encryption_dictionary::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertEncryptionDictionary { version: 0, revision: 0, placements: Vec::new() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
