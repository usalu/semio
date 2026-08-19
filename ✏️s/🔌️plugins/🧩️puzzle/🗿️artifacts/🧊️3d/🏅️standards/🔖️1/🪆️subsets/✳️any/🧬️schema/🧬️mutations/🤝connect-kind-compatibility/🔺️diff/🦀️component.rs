//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — patches the document `meta.kindCompatibility`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::{Puzzle3dKindCompatibility, Puzzle3dSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "already connected").at(vec![payload.source.clone(), payload.target.clone()])]);
    }
    let mut meta = base.meta.clone();
    meta.kind_compatibility.push(Puzzle3dKindCompatibility {
        source: payload.source.clone(),
        target: payload.target.clone(),
        bidirectional: payload.bidirectional,
        important: payload.important,
        specificity: payload.specificity,
    });
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
