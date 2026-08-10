//! playbook <- md
use crate::artifacts::playbook::PlaybookSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaybookSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}
