use crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root::SetDpartRoot;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_root::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDpartRoot { job: "sample".to_string(), placements: Vec::new(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
