//! block3d -> glb
use crate::artifacts::block3d::Block3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Block3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Block3dSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
