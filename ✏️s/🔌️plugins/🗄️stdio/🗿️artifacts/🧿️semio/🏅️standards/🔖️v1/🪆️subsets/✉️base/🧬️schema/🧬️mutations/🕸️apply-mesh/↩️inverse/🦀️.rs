//! ↩️ Inverse for `ApplyMesh`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_mesh;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyMesh, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_mesh::ApplyMesh>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Mesh(b) => <SemioMeshMutation as ColdMutation<SemioMeshSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyMesh { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
