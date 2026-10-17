//! ↩️ Inverse for `SetShapeFrame`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::frame_of;
#[allow(unused_imports)]
use crate::schema::mutations::set_shape_frame;
#[allow(unused_imports)]
use crate::schema::mutations::support::shape_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetShapeFrame, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_shape_frame::SetShapeFrame>,
{
    let super::SetShapeFrame { slide_index, shape_index, .. } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(shape) => vec![Op::from(set_shape_frame::SetShapeFrame { slide_index: *slide_index, shape_index: *shape_index, frame: *frame_of(shape) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
