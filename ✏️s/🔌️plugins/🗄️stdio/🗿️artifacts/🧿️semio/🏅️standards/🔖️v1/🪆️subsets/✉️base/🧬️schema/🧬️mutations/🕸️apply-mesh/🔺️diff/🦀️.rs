//! 🔺️ Diff for `ApplyMesh`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyMesh, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Mesh(b) => <SemioMeshMutation as ColdMutation<SemioMeshSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Mesh),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
