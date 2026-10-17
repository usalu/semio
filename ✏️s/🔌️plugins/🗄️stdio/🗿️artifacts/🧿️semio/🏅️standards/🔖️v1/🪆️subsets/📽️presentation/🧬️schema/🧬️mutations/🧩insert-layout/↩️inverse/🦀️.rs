//! ↩️ Inverse for `InsertLayout`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_layout;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertLayout, _base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_layout::RemoveLayout>,
{
    let super::InsertLayout { layout, .. } = payload;
    Ok(vec![Op::from(remove_layout::RemoveLayout { id: layout.id.clone() })])
}
//#endregion 🔖️Inverse
