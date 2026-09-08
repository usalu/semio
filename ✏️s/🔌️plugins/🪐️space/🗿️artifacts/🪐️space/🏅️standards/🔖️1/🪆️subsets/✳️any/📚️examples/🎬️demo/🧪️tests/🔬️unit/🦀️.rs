
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

#[semio_framework_async_macros::async_test]
async fn bundled_example_parses_as_a_valid_space_index() {
    let document = <SSpaceSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("bundled example parses");
    assert_eq!(document.space_id, "demo-space");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
