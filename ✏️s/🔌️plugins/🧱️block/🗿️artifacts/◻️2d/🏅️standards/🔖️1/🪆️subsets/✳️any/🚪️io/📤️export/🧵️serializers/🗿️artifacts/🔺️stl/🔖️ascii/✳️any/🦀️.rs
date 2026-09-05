//! block2d -> stl
use crate::artifacts::block2d::Block2dSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &Block2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
