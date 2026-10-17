use crate::standards::v1_7::subsets::x::schema::mutations::remove_output_intent::RemoveOutputIntent;
use crate::standards::v1_7::subsets::x::io::text::mutations::remove_output_intent::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveOutputIntent {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
