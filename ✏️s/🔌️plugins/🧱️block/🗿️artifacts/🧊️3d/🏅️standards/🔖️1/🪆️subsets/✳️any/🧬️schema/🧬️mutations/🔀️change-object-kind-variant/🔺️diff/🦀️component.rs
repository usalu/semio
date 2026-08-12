//! 🔺️ Sparse diff builder for `ChangeObjectKindVariant` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeObjectKindVariant, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { object_kind: Some(BlockKindIdentity { variant: payload.new_variant.clone(), ..base.object_kind.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
