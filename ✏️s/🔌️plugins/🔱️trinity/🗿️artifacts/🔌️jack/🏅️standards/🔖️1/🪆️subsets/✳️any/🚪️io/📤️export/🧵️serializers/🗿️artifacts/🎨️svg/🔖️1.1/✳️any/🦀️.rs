//! jack -> svg
use crate::JackSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
