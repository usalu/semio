//! 🔺️ Sparse diff builder for `ChangeTitle` — a real single-field patch (never a whole-snapshot
//! capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeTitle, base: &PlaybookSnapshot) -> protocol::MutationOutcome<PlaybookDiff> {
    if payload.new_title == base.title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Playbook title already has this value.");
    }
    protocol::MutationOutcome::new(PlaybookDiff { title: Some(payload.new_title.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
