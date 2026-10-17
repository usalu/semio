use crate::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::RemoveStructTreeRoot;
use crate::standards::v1_7::subsets::ua::io::text::mutations::remove_struct_tree_root::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveStructTreeRoot {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
