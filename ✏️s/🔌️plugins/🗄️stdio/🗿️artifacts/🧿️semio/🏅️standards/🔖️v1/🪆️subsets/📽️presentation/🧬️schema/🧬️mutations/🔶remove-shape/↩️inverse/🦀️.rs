//! ↩️ Inverse for `RemoveShape`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_shape;
#[allow(unused_imports)]
use crate::schema::mutations::support::shape_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveShape, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_shape::InsertShape>,
{
    let super::RemoveShape { slide_index, shape_index } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(shape) => vec![Op::from(insert_shape::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
