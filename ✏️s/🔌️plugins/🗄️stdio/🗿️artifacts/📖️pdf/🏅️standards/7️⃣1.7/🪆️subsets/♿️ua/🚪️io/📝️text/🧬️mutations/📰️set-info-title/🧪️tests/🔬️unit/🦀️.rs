use crate::standards::v1_7::subsets::ua::schema::mutations::set_info_title::SetInfoTitle;
use crate::standards::v1_7::subsets::ua::io::text::mutations::set_info_title::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetInfoTitle { title: "accessible title".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
