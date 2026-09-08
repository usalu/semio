
use super::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_representative_document() {
    let document = Block5dSnapshot::default();
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
