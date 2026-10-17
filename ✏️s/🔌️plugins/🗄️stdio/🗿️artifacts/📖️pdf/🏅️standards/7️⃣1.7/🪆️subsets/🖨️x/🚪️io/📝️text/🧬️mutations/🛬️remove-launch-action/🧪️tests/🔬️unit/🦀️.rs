use crate::standards::v1_7::subsets::x::schema::mutations::remove_launch_action::RemoveLaunchAction;
use crate::standards::v1_7::subsets::x::io::text::mutations::remove_launch_action::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveLaunchAction { target: "sample".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
