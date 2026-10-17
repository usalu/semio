use crate::standards::v1_7::subsets::ua::schema::mutations::remove_mark_info::RemoveMarkInfo;
use crate::standards::v1_7::subsets::ua::io::binary::mutations::remove_mark_info::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveMarkInfo {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
