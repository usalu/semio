//! puzzle5d -> zip
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle5dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
