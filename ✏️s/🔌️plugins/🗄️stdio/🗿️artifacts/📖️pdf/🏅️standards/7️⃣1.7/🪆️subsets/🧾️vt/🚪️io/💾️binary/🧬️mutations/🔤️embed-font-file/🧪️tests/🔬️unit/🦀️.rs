
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef { num: 1, gen: 0 }, entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
