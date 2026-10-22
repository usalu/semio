//! ↩️ Inverse for `ApplyDrawing`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_drawing;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyDrawing, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_drawing::ApplyDrawing>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Drawing(b) => <SemioDrawingMutation as ColdMutation<SemioDrawingSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyDrawing { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
