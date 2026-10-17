use crate::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::SetStructTreeRoot;
use crate::standards::v1_7::subsets::ua::io::binary::mutations::set_struct_tree_root::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetStructTreeRoot { placements: Vec::new(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
