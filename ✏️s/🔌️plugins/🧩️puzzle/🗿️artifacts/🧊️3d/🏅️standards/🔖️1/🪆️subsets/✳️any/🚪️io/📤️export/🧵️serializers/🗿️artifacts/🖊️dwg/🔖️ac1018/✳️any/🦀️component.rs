//! puzzle3d -> dwg
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Puzzle3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
