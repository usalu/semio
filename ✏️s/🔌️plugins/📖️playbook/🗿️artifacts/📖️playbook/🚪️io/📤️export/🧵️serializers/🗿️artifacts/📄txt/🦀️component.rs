//! playbook -> txt
use crate::artifacts::playbook::PlaybookSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(<PlaybookSnapshot as store::DocumentDsl>::render_dsl(snapshot).into_bytes())
}
