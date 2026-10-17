//! ↩️ Inverse for `CreateMesh`.

use crate::schema::mutations::delete_mesh;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateMesh, _base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_mesh::DeleteMesh>,
{
    Ok((|| {
    vec![Op::from(delete_mesh::DeleteMesh { id: payload.mesh.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
