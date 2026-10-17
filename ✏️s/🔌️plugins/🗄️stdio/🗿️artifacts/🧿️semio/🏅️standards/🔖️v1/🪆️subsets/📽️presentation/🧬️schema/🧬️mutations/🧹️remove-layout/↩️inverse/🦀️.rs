//! ↩️ Inverse for `RemoveLayout`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_layout;
#[allow(unused_imports)]
use crate::schema::mutations::support::layout_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveLayout, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_layout::InsertLayout>,
{
    let super::RemoveLayout { id } = payload;
    Ok(match layout_at(base, id) {
        Some(l) => vec![Op::from(insert_layout::InsertLayout { layout: l.clone(), at: base.layouts.iter().position(|layout| layout.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
