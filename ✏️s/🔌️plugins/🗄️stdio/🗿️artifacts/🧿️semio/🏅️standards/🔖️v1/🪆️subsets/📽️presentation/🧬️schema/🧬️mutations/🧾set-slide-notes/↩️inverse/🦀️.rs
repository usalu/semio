//! ↩️ Inverse for `SetSlideNotes`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_slide_notes;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetSlideNotes, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_slide_notes::SetSlideNotes>,
{
    let super::SetSlideNotes { index, .. } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![Op::from(set_slide_notes::SetSlideNotes { index: *index, notes: slide.notes.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
