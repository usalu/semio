//! ↩️ Inverse for `SetSlideLayout`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_slide_layout;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetSlideLayout, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_slide_layout::SetSlideLayout>,
{
    let super::SetSlideLayout { index, .. } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![Op::from(set_slide_layout::SetSlideLayout { index: *index, layout_id: slide.layout_id.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
