use crate::standards::v1_7::subsets::ua::schema::mutations::set_info_title::SetInfoTitle;
use crate::standards::v1_7::subsets::ua::io::binary::mutations::set_info_title::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetInfoTitle { title: "accessible title".to_string() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
