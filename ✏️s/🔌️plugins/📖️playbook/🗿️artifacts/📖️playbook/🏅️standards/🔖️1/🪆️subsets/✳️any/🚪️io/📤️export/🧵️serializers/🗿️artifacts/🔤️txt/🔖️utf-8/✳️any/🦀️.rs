//! playbook -> txt
use crate::PlaybookSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(<PlaybookSnapshot as store::os_store::serialization::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
