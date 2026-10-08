//! 🔺️ Diff for `RemoveContent`. A missing entry ⇒ Error `mutation.target-missing`; `from` past the stored
//! leaf count ⇒ Error `mutation.target-mismatch`; `from` equal to the stored leaf count ⇒ Warning `mutation.no-op`.
use crate::diff::{RemodelingContentRow, RemodelingDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveContent, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let target = [payload.content_id.clone()];
    let Some(artifact) = base.durable_artifacts.get(&payload.content_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Content \"{}\" does not exist.", payload.content_id), target);
    };
    let stored = artifact.chunks.len() as u64;
    if payload.from > stored {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Content \"{}\" stores only {stored} leaves.", payload.content_id), target);
    }
    if payload.from == stored {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Content \"{}\" stores no leaf from {stored} on.", payload.content_id));
    }
    protocol::MutationOutcome::new(RemodelingDiff::content_rows(vec![RemodelingContentRow::Truncate { id: payload.content_id.clone(), from: payload.from }]))
}
//#endregion 🔖️Diff
