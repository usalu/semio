use crate::standards::v1_7::subsets::x::schema::mutations::remove_trim_box::RemoveTrimBox;
use crate::standards::v1_7::subsets::x::io::binary::mutations::remove_trim_box::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveTrimBox { page_index: 0 };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
