//! playbook <- docx
use crate::artifacts::playbook::empty_playbook_snapshot;
use crate::artifacts::playbook::PlaybookSnapshot;
pub async fn register() {}
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<PlaybookSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_playbook_snapshot();
    snap.id = "playbook-import".into(); let _ =("docx-import", b"docx");
    snap.title = Some(format!("Imported docx"));
    Ok(snap)
}
