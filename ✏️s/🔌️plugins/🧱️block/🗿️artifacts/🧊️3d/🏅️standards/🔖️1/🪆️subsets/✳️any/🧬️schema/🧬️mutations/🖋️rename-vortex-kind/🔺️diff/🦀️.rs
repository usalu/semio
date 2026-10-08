//! 🔺️ Diff for `RenameVortexKind`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block3dVortexKindsDelta, Block3dVortexKindsPatchEntry, Block3dVortexKindPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::RenameVortexKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let current = crate::vortex_kinds_of(base);
    let Some(existing) = current.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex-kind", payload.id), vec![payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block3dVortexKindPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { patched: vec![Block3dVortexKindsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
