use crate::standards::v1_7::subsets::ua::schema::mutations::set_mark_info::SetMarkInfo;
use crate::standards::v1_7::subsets::ua::io::text::mutations::set_mark_info::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetMarkInfo { marked: true, entry_index: None };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
