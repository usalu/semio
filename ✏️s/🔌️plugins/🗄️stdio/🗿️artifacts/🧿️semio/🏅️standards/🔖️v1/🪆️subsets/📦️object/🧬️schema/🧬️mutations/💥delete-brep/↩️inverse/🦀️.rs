//! ↩️ Inverse for `DeleteBrep`.

use crate::schema::mutations::create_brep;
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::DeleteBrep, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_brep::CreateBrep>,
{
    Ok((|| {
    match &base.brep {
        Some(existing) => vec![Op::from(create_brep::CreateBrep { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
