use crate::standards::v1_7::subsets::x::schema::mutations::insert_media_annotation::InsertMediaAnnotation;
use crate::standards::v1_7::subsets::x::io::binary::mutations::insert_media_annotation::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string(), placements: Vec::new() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
