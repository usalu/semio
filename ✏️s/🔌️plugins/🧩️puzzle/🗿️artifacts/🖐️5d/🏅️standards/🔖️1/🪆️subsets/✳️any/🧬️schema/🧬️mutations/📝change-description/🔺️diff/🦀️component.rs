//! 🔺️ Sparse diff builder for `ChangeDescription` — patches the document `meta.description`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::{Puzzle5dMeta, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDescription, _base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    protocol::MutationOutcome::new(Puzzle5dDiff { meta: Some(Puzzle5dMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
