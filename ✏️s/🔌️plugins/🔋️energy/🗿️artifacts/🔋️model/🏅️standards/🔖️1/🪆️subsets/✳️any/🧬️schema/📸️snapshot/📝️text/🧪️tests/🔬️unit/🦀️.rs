use super::*;

#[semio_framework_async_macros::async_test]
async fn semio_example_dsl_round_trips() {
    let document = parse_dsl(SEMIO_ENERGY_MODEL_EXAMPLE_TEXT).expect("parse semio example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
