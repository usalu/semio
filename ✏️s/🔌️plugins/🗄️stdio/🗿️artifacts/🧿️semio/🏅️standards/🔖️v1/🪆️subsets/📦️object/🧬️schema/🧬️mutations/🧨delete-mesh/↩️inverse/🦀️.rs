//! ↩️ Inverse for `DeleteMesh`.

use crate::schema::mutations::create_mesh;
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::DeleteMesh, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_mesh::CreateMesh>,
{
    Ok((|| {
    match &base.mesh {
        Some(existing) => vec![Op::from(create_mesh::CreateMesh { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
