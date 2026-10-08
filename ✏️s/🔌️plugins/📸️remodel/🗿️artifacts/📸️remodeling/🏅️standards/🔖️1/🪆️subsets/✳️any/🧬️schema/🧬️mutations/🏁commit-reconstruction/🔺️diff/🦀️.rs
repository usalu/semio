//! 🔺️ Atomic diff for a reconstruction result. Every durable content handle the payload names must be
//! complete in the BASE document: a `remodeling-content:` sparse buffer ⇒ otherwise
//! `mutation.target-mismatch`; a `remodeling-mesh-content:` mesh ⇒ otherwise
//! `mutation.target-mismatch`; a bound asset content id ⇒ otherwise
//! `mutation.target-mismatch`. Inline buffers and constant meshes are plain values. A payload
//! equal to the stored lanes and bindings ⇒ Warning `mutation.no-op`.
use crate::diff::{RemodelingAssetEntry, RemodelingAssigned, RemodelingDiff, RemodelingResultsDiff, RemodelingRow, RemodelingRows};
use crate::{committed_remodeling_asset_handle, remodeling_content_is_complete, remodeling_mesh_content_handle_parts, RemodelingContentKind, RemodelingSnapshot};
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::CommitReconstruction, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if let Some((content_id, chunk_count)) = payload.sparse.as_ref().and_then(|sparse| sparse.points.content_reference()) {
        if !remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Sparse, chunk_count) {
            return protocol::MutationOutcome::error("mutation.target-mismatch", "The sparse cloud names durable content that is not complete.", ["sparse".to_string()]);
        }
    }
    if let Some((content_id, chunk_count)) = payload.mesh.as_ref().and_then(|mesh| remodeling_mesh_content_handle_parts(&mesh.mesh)) {
        if !remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Mesh, chunk_count) {
            return protocol::MutationOutcome::error("mutation.target-mismatch", "The mesh names durable content that is not complete.", [content_id.to_string()]);
        }
    }
    let bindings: BTreeMap<&str, Option<&str>> = payload.assets.iter().map(|binding| (binding.id.as_str(), binding.content_id.as_deref())).collect();
    let mut rows = Vec::new();
    for (id, content_id) in bindings {
        match content_id {
            Some(content_id) => {
                let complete = base.durable_artifacts.get(content_id).is_some_and(|artifact| artifact.kind == RemodelingContentKind::Image.wire() && remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Image, artifact.chunks.len() as u64));
                if !complete {
                    return protocol::MutationOutcome::error("mutation.target-mismatch", "A bound asset names durable image content that is not complete.", [id.to_string()]);
                }
                let child = committed_remodeling_asset_handle(id, content_id);
                match base.assets.get(id) {
                    Some(held) if held == &child => {}
                    Some(_) => rows.push(RemodelingRow::Replace { entity: RemodelingAssetEntry { key: id.to_string(), child } }),
                    None => rows.push(RemodelingRow::Insert { entity: RemodelingAssetEntry { key: id.to_string(), child } }),
                }
            }
            None if base.assets.contains_key(id) => rows.push(RemodelingRow::Remove { key: id.to_string() }),
            None => {}
        }
    }
    let results = RemodelingResultsDiff {
        sparse: (payload.sparse != base.results.sparse).then(|| RemodelingAssigned::new(payload.sparse.clone())),
        mesh: payload.mesh.as_ref().filter(|mesh| ***mesh != base.results.mesh).map(|mesh| (**mesh).clone()),
        trajectory: (payload.trajectory != base.results.trajectory).then(|| RemodelingAssigned::new(payload.trajectory.clone())),
        geo: (payload.geo != base.results.geo).then(|| RemodelingAssigned::new(payload.geo.clone())),
        qc: (payload.qc != base.results.qc).then(|| RemodelingAssigned::new(payload.qc.clone())),
        ..Default::default()
    };
    if rows.is_empty() && results.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The reconstruction result is already committed.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { assets: (!rows.is_empty()).then_some(RemodelingRows { rows }), results: (!results.is_empty()).then_some(results), ..Default::default() })
}
//#endregion 🔖️Diff
