//! 🔺️ Sparse diff builder for `ChangeFastenerKind` — patches the one addressed fastener in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dFastenerPatch, Puzzle5dFastenersDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFastenerKind, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "fastener", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle5dFastenerPatch {
        fastener_kind: (payload.new_fastener_kind != item.fastener_kind).then(|| payload.new_fastener_kind.clone()),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        fasteners: Some(Puzzle5dFastenersDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
