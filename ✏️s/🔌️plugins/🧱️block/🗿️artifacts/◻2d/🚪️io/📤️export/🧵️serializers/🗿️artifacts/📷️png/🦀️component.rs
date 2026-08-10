//! block2d -> png
use crate::artifacts::block2d::Block2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Block2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block2dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
