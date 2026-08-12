//! 🔺️ Sparse diff builder for `AddRepresentationAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta, Block5dRepresentationsPatch, Block5dRepresentationsPatchEntry};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockRepresentation};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddRepresentationAttribute, base: &Block5dSnapshot) -> Block5dDiff {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else { return Block5dDiff::default(); };
    let mut attributes = existing.attributes.clone();
    attributes.push(payload.attribute.clone());
    let replacement = BlockRepresentation { attributes, ..existing.clone() };
    Block5dDiff { representations: Some(Block5dRepresentationsDelta { patched: vec![Block5dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block5dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
