
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_definition_matches_default() {
    assert_eq!(empty_block3d_snapshot(), Block3dSnapshot::default());
}
