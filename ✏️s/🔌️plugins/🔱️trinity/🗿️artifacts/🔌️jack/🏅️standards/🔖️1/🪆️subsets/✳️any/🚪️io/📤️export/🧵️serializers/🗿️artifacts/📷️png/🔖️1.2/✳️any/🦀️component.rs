//! jack -> png
use crate::artifacts::jack::JackSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
