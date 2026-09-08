
use super::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = crate::document_dsl::parse_dsl(crate::document_dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT).expect("parse semio example");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
