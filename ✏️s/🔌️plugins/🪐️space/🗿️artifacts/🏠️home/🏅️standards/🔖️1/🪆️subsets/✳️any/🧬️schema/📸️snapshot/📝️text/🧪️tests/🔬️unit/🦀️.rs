
use super::*;

#[semio_framework_async_macros::async_test]
async fn home_dsl_round_trips_default_and_populated_documents() {
    store::os_store::test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 });
    store::os_store::test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 });
}

#[semio_framework_async_macros::async_test]
async fn home_dsl_round_trips_bundled_default_example() {
    let document = parse_dsl(HOME_EXAMPLE_TEXT).expect("parse default example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
