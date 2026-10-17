use crate::standards::v1_7::subsets::vt::schema::mutations::insert_launch_action::InsertLaunchAction;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::insert_launch_action::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertLaunchAction { target: "sample".to_string(), placements: Vec::new() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
