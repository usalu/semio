use crate::standards::v1_7::subsets::x::schema::mutations::remove_encryption_dictionary::RemoveEncryptionDictionary;
use crate::standards::v1_7::subsets::x::io::binary::mutations::remove_encryption_dictionary::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveEncryptionDictionary { version: 0, revision: 0 };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
