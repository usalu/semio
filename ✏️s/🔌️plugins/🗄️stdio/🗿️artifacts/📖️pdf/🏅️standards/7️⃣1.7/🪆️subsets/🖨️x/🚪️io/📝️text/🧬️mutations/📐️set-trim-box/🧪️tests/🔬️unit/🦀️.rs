use crate::standards::v1_7::subsets::x::schema::mutations::set_trim_box::SetTrimBox;
use crate::standards::v1_7::subsets::x::io::text::mutations::set_trim_box::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetTrimBox { page_index: 0, trim_box: [0.0, 0.0, 100.0, 100.0], entry_index: None };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
