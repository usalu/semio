//! block5d -> stl
use crate::artifacts::block5d::Block5dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Block5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block5dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
