//! block5d -> stl
use crate::artifacts::block5d::Block5dSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &Block5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block5dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
