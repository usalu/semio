//! 🔺️ Sparse diff builder for `EditWorkingGraph` — names only the working-graph document fields the new graph changes.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;
use protocol::DiffAlgebra;
use semio_s_artifact_trinity_jack::{JackDiff, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::EditWorkingGraph, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let (next, current) = (&payload.new_working_graph, &base.working_graph);
    let patch = JackDiff {
        schema: (next.schema != current.schema).then(|| next.schema.clone()),
        name: (next.name != current.name).then(|| next.name.clone()),
        manifest_id: (next.manifest_id != current.manifest_id).then(|| next.manifest_id.clone()),
        manifest: (next.manifest != current.manifest).then(|| next.manifest.clone()),
        camera: (next.camera != current.camera).then(|| next.camera.clone()),
        content: (next.content != current.content).then(|| next.content.clone()),
        root_node_id: (next.root_node_id != current.root_node_id).then(|| next.root_node_id.clone()),
        query: (next.query != current.query).then(|| next.query.clone()),
    };
    if DiffAlgebra::<JackSnapshot>::is_empty(&patch) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Working graph is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { working_graph: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
