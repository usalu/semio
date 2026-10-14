use crate::standards::v1::subsets::any::io::binary::snapshot::*;


#[semio_framework_async_macros::async_test]
async fn selected_child_parent_pack_round_trips_and_agrees_with_dsl() {
    let document = <crate::test_support::FixtureComposition as crate::JackComposition>::initial_snapshot();
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn selected_child_pack_schema_identity_preserves_address_without_content_copy() {
    let document = <crate::test_support::FixtureComposition as crate::JackComposition>::initial_snapshot();
    ::store::os_store::test_support::assert_pack_schema_identity(&document);
    let decoded = decode(&encode(&document)).expect("decode");
    assert_eq!(decoded.content,document.content);
    assert!(crate::jack_content_for_handle(&document.content).is_ok());
    assert!(crate::jack_content_for_handle(&decoded.content).is_err());
}
