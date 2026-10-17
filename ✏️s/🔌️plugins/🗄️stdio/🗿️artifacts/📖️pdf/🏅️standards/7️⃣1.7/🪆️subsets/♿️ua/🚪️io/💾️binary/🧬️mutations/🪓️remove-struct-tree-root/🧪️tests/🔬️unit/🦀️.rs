use crate::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::RemoveStructTreeRoot;
use crate::standards::v1_7::subsets::ua::io::binary::mutations::remove_struct_tree_root::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveStructTreeRoot {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
