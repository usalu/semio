
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::text::{NAKAGIN_EXAMPLE_TEXT, parse_dsl};

#[semio_framework_async_macros::async_test]
async fn nakagin_example_pack_round_trips_and_agrees_with_dsl() {
    let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
