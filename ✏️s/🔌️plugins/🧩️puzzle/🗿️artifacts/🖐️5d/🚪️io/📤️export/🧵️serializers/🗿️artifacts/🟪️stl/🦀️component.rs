//! puzzle5d -> stl
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle5dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
