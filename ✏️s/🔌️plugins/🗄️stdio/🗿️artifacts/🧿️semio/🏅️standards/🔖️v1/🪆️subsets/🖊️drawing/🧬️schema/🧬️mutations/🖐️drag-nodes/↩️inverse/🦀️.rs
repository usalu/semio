//! ↩️ Inverse for `DragNodes`.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::schema::mutations::SemioDrawingMutation;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DragNodes, _base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioDrawingMutation::DragNodes(super::DragNodes { ats: payload.ats.clone(), offset: SemioPoint2 { x: -payload.offset.x, y: -payload.offset.y } })]

    })())
}
//#endregion 🔖️Inverse
