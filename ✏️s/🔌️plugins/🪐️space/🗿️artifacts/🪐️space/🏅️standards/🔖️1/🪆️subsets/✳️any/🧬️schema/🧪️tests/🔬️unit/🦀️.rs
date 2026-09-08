
use super::*;

#[semio_framework_async_macros::async_test]
async fn descriptor_carries_the_space_index_schema_id() {
    assert_eq!(sspace_index_schema_descriptor().id, "s.space.space");
}
