//! playbook -> pdf
use crate::artifacts::playbook::PlaybookSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(<PlaybookSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
