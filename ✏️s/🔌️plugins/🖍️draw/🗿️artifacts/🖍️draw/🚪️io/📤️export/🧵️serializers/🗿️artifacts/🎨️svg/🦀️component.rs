//! draw -> svg
use crate::artifacts::draw::DrawSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &DrawSnapshot) -> Result<Vec<u8>, String> {
    Ok(<DrawSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
