//! 🔺️ Sparse diff builder for `ConnectNodes`.
use crate::artifacts::dag::diff::{DagDiff, DagEdgesDelta};
use crate::artifacts::dag::{DagFixtureEdge, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectNodes, _base: &DagSnapshot) -> DagDiff {
    let edge = DagFixtureEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), route_style: payload.route_style, properties: payload.properties.clone() };
    DagDiff { edges: Some(DagEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
