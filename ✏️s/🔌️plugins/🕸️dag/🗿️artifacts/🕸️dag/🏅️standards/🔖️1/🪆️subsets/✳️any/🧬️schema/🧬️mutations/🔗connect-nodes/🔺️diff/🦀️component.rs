//! 🔺️ Sparse diff builder for `ConnectNodes`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagFixtureEdge, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectNodes, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let edge = DagFixtureEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), route_style: payload.route_style, properties: payload.properties.clone() };
    let mut edges = scene.edges;
    edges.push(edge);
    diff_replace_content(scene.nodes, edges)
}
//#endregion 🔖️Diff
