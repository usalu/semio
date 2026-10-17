use crate::standards::v1_7::subsets::x::schema::mutations::remove_font_file::RemoveFontFile;
use crate::standards::v1_7::subsets::x::io::binary::mutations::remove_font_file::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveFontFile { descriptor_ordinal: 0 };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
