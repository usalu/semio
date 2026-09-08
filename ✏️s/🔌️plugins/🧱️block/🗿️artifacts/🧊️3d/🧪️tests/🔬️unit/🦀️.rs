
use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_3d_block_interchange_kind() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "3d.block");
    assert_eq!(kind.schema, BLOCK_3D_SCHEMA);
    assert_eq!(kind.component_kind, "block3d");
}
