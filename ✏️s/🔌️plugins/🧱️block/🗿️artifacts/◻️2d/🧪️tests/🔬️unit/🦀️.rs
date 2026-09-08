
use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_2d_block_interchange_kind() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "2d.block");
    assert_eq!(kind.schema, BLOCK_2D_SCHEMA);
    assert_eq!(kind.component_kind, "block2d");
}
