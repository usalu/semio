//! 🪪️ Canonical checkpoint identity representation and hashing.
use super::super::super::{ArtifactHistoryLedger,Change,Author,CompositionPin};

struct PendingChangeRef<'a> {
    id: &'a str,
    edit_ids: &'a [String],
    description: Option<&'a str>,
    saved_at: &'a str,
}
fn pending_change_ref_json(pending: &PendingChangeRef<'_>) -> String {
    let mut object = semio_framework_pack_json::Object::new();
    object.insert("id", semio_framework_pack_json::Value::String(pending.id.to_string()));
    object.insert("editIds", semio_framework_pack_json::Value::Array(pending.edit_ids.iter().map(|id| semio_framework_pack_json::Value::String(id.clone())).collect()));
    object.insert("description", pending.description.map_or(semio_framework_pack_json::Value::Null, |text| semio_framework_pack_json::Value::String(text.to_string())));
    object.insert("savedAt", semio_framework_pack_json::Value::String(pending.saved_at.to_string()));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(object))
}

fn content_addressed_checkpoint_id_core(
    parent_id: Option<&str>,
    change_ids: &[String],
    changes: &ArtifactHistoryLedger<Change>,
    pending: Option<PendingChangeRef<'_>>,
    message: Option<&str>,
    authors: &[Author],
    timestamp: &str,
    pins: &[CompositionPin],
) -> String {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut input = Vec::new();
    input.extend_from_slice(parent_id.unwrap_or("").as_bytes());
    input.push(0);
    for change_id in change_ids {
        let change_hash = if let Some(change) = changes.iter().find(|change| change.id == *change_id) {
            *semio_framework_hash::hash(semio_framework_pack_json::to_json_string(change).as_bytes()).as_bytes()
        } else if let Some(change) = pending.as_ref().filter(|change| change.id == change_id.as_str()) {
            *semio_framework_hash::hash(pending_change_ref_json(change).as_bytes()).as_bytes()
        } else {
            [0u8; 32]
        };
        input.extend_from_slice(&change_hash);
    }
    input.push(0);
    input.extend_from_slice(message.unwrap_or("").as_bytes());
    input.push(0);
    for author in authors {
        input.extend_from_slice(author.id.as_bytes());
        input.push(0);
    }
    input.push(0);
    input.extend_from_slice(timestamp.as_bytes());
    if !pins.is_empty() {
        let mut ordered: Vec<(String, &CompositionPin)> = Vec::with_capacity(pins.len());
        for pin in pins {
            ordered.push((pin.child_ref.to_uri(), pin));
        }
        ordered.sort_by(|(a, _), (b, _)| a.cmp(b));
        input.push(0);
        for (uri, pin) in ordered {
            input.extend_from_slice(uri.as_bytes());
            input.push(0);
            input.extend_from_slice(pin.checkpoint_id.as_bytes());
            input.push(0);
        }
    }
    let digest = *semio_framework_hash::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("ck-{hex16}")
}

/// 🔒️ Mints a checkpoint identity from its canonical representation commitments.
pub async fn content_addressed_checkpoint_id(parent_id: Option<&str>, change_ids: &[String], changes: &ArtifactHistoryLedger<Change>, message: Option<&str>, authors: &[Author], timestamp: &str, pins: &[CompositionPin]) -> String {
    content_addressed_checkpoint_id_core(parent_id, change_ids, changes, None, message, authors, timestamp, pins)
}

/// 🪡️ Admits a pending change's canonical commitment before its ledger insertion.
pub fn content_addressed_checkpoint_id_with_pending_change(
    parent_id: Option<&str>,
    change_ids: &[String],
    changes: &ArtifactHistoryLedger<Change>,
    pending_change_id: &str,
    pending_edit_ids: &[String],
    pending_description: Option<&str>,
    pending_saved_at: &str,
    message: Option<&str>,
    authors: &[Author],
    timestamp: &str,
    pins: &[CompositionPin],
) -> String {
    content_addressed_checkpoint_id_core(parent_id, change_ids, changes, Some(PendingChangeRef { id: pending_change_id, edit_ids: pending_edit_ids, description: pending_description, saved_at: pending_saved_at }), message, authors, timestamp, pins)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
