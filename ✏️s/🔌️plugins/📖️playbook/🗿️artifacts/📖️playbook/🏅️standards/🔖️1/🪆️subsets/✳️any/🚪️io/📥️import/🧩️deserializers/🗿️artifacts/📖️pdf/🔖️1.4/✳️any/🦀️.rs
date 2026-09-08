//! playbook <- pdf
use crate::empty_playbook_snapshot;
use crate::PlaybookSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaybookSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_playbook_snapshot();
    snap.id = "playbook-import".into();
    let _ = ("pdf-import", b"pdf");
    snap.title = Some(format!("Imported pdf"));
    Ok(snap)
}
