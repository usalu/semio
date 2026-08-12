//! 🔺️ Sparse diff builder for `CreateHandleKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandleKindsDelta};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateHandleKind, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { handle_kinds: Some(Block2dHandleKindsDelta { added: vec![payload.handle_kind.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
