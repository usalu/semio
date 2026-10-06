use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::empty_playbook_snapshot;

#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_the_empty_snapshot() {
    let document = empty_playbook_snapshot();
    let text = print_dsl(&document);
    assert_eq!(parse_dsl(&text).expect("parse"), document);
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_dsl_round_trips() {
    let document = parse_dsl(FACADE_GENERATOR_EXAMPLE_TEXT).expect("parse example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_matches_the_handcrafted_spec() {
    let document = parse_dsl(FACADE_GENERATOR_EXAMPLE_TEXT).expect("parse example");
    assert_eq!(document.title.as_deref(), Some("Facade Panel Generator"));
    assert_eq!(document.flow.child_id, crate::examples::demo::FLOW_ID);
    assert_eq!(print_dsl(&document).trim_end(), FACADE_GENERATOR_EXAMPLE_TEXT.trim_end());
}
