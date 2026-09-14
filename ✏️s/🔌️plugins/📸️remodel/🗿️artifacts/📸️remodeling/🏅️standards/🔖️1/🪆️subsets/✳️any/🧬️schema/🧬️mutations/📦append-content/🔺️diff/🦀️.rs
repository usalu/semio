//! 🔺️ Diff for `AppendContent`. Refusals: no leaves or an undecodable/oversized leaf ⇒
//! `mutation.invalid-content-chunk`; a leaf index past the stored length ⇒ `mutation.content-gap`; an
//! entry of another kind or presentation ⇒ `mutation.content-kind-mismatch`; overlapping leaves that
//! differ ⇒ `mutation.content-conflict`; a result beyond the kind's envelope ⇒ `mutation.content-capacity`.
//! Leaves that are already stored byte for byte ⇒ Warning `mutation.no-op`, so an identical reconstruction
//! re-lands on the same content-addressed entry without rewriting it.
use crate::diff::RemodelingDiff;
use crate::{decode_remodeling_durable_chunk, RemodelingDurableArtifact, RemodelingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::AppendContent, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let target = [payload.content_id.clone()];
    let mut appended_bytes = 0usize;
    for chunk in &payload.chunks {
        match decode_remodeling_durable_chunk(chunk) {
            Some(bytes) => appended_bytes += bytes.len(),
            None => return protocol::MutationOutcome::error("mutation.invalid-content-chunk", "A content leaf is not base64 or exceeds the 4 KiB leaf envelope.", target),
        }
    }
    if payload.chunks.is_empty() {
        return protocol::MutationOutcome::error("mutation.invalid-content-chunk", "An append carries no content leaves.", target);
    }
    let existing = base.durable_artifacts.get(&payload.content_id);
    let stored = existing.map_or(&[][..], |artifact| artifact.chunks.as_slice());
    let Ok(first) = usize::try_from(payload.first) else { return protocol::MutationOutcome::error("mutation.content-gap", "The first leaf index is out of range.", target) };
    if first > stored.len() {
        return protocol::MutationOutcome::error("mutation.content-gap", format!("Leaf {first} would leave a gap after {} stored leaves.", stored.len()), target);
    }
    if let Some(artifact) = existing {
        if artifact.kind != payload.kind.wire() || artifact.mime != payload.mime || artifact.width != payload.width || artifact.height != payload.height {
            return protocol::MutationOutcome::error("mutation.content-kind-mismatch", format!("Content \"{}\" is stored as another kind or presentation.", payload.content_id), target);
        }
    }
    let overlap = stored.len().saturating_sub(first).min(payload.chunks.len());
    if stored[first..first + overlap] != payload.chunks[..overlap] {
        return protocol::MutationOutcome::error("mutation.content-conflict", "An appended leaf differs from the leaf already stored at its index.", target);
    }
    if overlap == payload.chunks.len() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Content \"{}\" already stores these leaves.", payload.content_id));
    }
    let stored_bytes: usize = stored[..first + overlap].iter().filter_map(|chunk| decode_remodeling_durable_chunk(chunk)).map(|bytes| bytes.len()).sum();
    let total_chunks = first + payload.chunks.len();
    if total_chunks > payload.kind.max_chunks() || stored_bytes + appended_bytes > payload.kind.max_bytes() {
        return protocol::MutationOutcome::error("mutation.content-capacity", format!("Content \"{}\" would exceed its {} envelope.", payload.content_id, payload.kind.wire()), target);
    }
    let mut durable_artifacts = base.durable_artifacts.clone();
    let entry = durable_artifacts.entry(payload.content_id.clone()).or_insert_with(|| RemodelingDurableArtifact { kind: payload.kind.wire().into(), mime: payload.mime.clone(), width: payload.width, height: payload.height, chunks: Vec::new() });
    entry.chunks.extend(payload.chunks[overlap..].iter().cloned());
    protocol::MutationOutcome::new(RemodelingDiff { durable_artifacts: Some(durable_artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
