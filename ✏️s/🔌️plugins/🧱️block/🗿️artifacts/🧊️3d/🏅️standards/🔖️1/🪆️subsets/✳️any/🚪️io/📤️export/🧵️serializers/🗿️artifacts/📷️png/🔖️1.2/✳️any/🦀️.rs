//! block3d -> png
use crate::artifacts::block3d::Block3dSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &Block3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
