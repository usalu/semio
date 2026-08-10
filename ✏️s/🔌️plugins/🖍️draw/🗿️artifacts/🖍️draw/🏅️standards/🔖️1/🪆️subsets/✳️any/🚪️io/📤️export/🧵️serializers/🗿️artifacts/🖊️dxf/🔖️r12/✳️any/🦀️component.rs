//! draw -> dxf
use crate::artifacts::draw::DrawSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &DrawSnapshot) -> Result<Vec<u8>, String> {
    Ok(<DrawSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
