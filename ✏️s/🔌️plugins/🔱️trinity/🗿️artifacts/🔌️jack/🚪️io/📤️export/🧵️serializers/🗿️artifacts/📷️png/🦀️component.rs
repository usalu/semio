//! jack -> png
use crate::artifacts::jack::JackSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JackSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
