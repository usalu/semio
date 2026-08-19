//! playbook -> md
use crate::artifacts::playbook::PlaybookSnapshot;
pub async fn register() {}
pub async fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(<PlaybookSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
