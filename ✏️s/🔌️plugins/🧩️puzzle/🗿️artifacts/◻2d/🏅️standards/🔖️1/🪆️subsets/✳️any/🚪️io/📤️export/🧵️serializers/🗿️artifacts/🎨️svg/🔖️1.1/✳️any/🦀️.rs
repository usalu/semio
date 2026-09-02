//! puzzle2d -> svg
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Puzzle2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
