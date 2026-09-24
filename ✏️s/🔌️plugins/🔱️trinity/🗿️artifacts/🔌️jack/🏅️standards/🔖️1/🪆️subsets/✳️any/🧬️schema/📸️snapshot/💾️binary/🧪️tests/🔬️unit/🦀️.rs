use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, NAKAGIN_EXAMPLE_TEXT};

#[semio_framework_async_macros::async_test]
async fn nakagin_example_pack_round_trips_and_agrees_with_dsl() {
    let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_pack_schema_identity_is_derived_and_keeps_the_graph() {
    let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
    ::store::os_store::test_support::assert_pack_schema_identity(&document);
    let decoded = decode(&encode(&document)).expect("decode");
    let (before, after) = (crate::jack_working_scene(&document), crate::jack_working_scene(&decoded));
    assert!(!before.nodes.is_empty());
    assert_eq!((after.nodes, after.edges), (before.nodes, before.edges));
}
