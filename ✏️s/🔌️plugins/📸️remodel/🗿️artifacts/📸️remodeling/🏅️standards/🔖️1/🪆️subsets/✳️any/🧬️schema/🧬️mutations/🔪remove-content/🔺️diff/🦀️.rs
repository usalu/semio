//! 🔺️ Diff for `RemoveContent`. A missing entry ⇒ Error `mutation.target-missing`; `from` past the stored
//! leaf count ⇒ Error `mutation.content-gap`; `from` equal to the stored leaf count ⇒ Warning `mutation.no-op`.
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveContent, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let target = [payload.content_id.clone()];
    let Some(artifact) = base.durable_artifacts.get(&payload.content_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Content \"{}\" does not exist.", payload.content_id), target);
    };
    let stored = artifact.chunks.len() as u64;
    if payload.from > stored {
        return protocol::MutationOutcome::error("mutation.content-gap", format!("Content \"{}\" stores only {stored} leaves.", payload.content_id), target);
    }
    if payload.from == stored {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Content \"{}\" stores no leaf from {stored} on.", payload.content_id));
    }
    let mut durable_artifacts = base.durable_artifacts.clone();
    if payload.from == 0 {
        durable_artifacts.remove(&payload.content_id);
    } else if let Some(entry) = durable_artifacts.get_mut(&payload.content_id) {
        entry.chunks.truncate(payload.from as usize);
    }
    protocol::MutationOutcome::new(RemodelingDiff { durable_artifacts: Some(durable_artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
