//! 🔺️ Sparse diff builder for `DisconnectNodes`.
use crate::artifacts::dag::diff::{DagDiff, DagEdgesDelta};
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectNodes, _base: &DagSnapshot) -> DagDiff {
    DagDiff { edges: Some(DagEdgesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
