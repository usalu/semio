
use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn home_dsl_round_trips_default_and_populated_documents() {
    semio_os_store_test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 });
    semio_os_store_test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 });
}

#[semio_framework_async_macros::async_test]
async fn home_dsl_round_trips_bundled_default_example() {
    let document = parse_dsl(HOME_EXAMPLE_TEXT).expect("parse default example");
    semio_os_store_test_support::assert_dsl_round_trip(&document);
}
