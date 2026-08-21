//! 🔺️ Sparse diff builder for `CreateHandle` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::Block2dHandlesDelta;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.handles.iter().any(|item| item.id == payload.handle.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "handle", payload.handle.id), vec![payload.handle.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Some(Block2dHandlesDelta { added: vec![payload.handle.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
