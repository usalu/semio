//! ↩️ Inverse for `InsertShape`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_shape;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertShape, _base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_shape::RemoveShape>,
{
    let super::InsertShape { slide_index, shape_index, .. } = payload;
    Ok({
        vec![Op::from(remove_shape::RemoveShape { slide_index: *slide_index, shape_index: *shape_index })]
    })
}
//#endregion 🔖️Inverse
