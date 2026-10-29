use crate::reference::text::*;

#[semio_framework_async_macros::async_test]
async fn semio_example_dsl_round_trips() {
    let document = parse_dsl(semio_framework_test_fixtures::fixture_text!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("parse semio example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
