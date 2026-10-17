//! ↩️ Inverse for `DragNodes`.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DragNodes, _base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::drag_nodes::DragNodes>,
{
    Ok((|| {
    vec![Op::from(super::DragNodes { ats: payload.ats.clone(), offset: SemioPoint2 { x: -payload.offset.x, y: -payload.offset.y } })]

    })())
}
//#endregion 🔖️Inverse
