//! 🔀 `set-primitive-topology` — Error `mutation.target-missing` when the
//! (`mesh_id`,`primitive_id`) pair is absent, Warning `mutation.no-op` when `topology` already
//! equals the current value. No `mutation.invariant` check: whether a topology is structurally
//! consistent with a primitive's `indices`/`positions` count is not validated anywhere else in
//! this subset (no such rule exists in `🚪️io/🦀️component.rs`'s `check_mesh_referential_invariants`
//! or elsewhere), so inventing one here would fabricate an undocumented domain rule.

use super::mutation::SetPrimitiveTopology;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &SetPrimitiveTopology, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]).await;
    };
    if primitive.topology == payload.topology {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Primitive \"{}\" topology is unchanged.", payload.primitive_id)).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_primitive_topology(base, &payload.mesh_id, &payload.primitive_id, payload.topology))
}
//#endregion 🔖️Diff
