use crate::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation::RemoveMediaAnnotation;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::remove_media_annotation::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
