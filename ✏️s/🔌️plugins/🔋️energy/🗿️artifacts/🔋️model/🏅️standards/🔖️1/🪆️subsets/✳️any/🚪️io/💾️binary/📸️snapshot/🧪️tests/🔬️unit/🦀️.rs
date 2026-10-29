use crate::reference::binary::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = crate::reference::text::parse_dsl(semio_framework_test_fixtures::fixture_text!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("parse semio example");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
