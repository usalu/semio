//! ↩️ Inverse for `RemoveSlide`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_slide;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveSlide, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_slide::InsertSlide>,
{
    let super::RemoveSlide { index } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![Op::from(insert_slide::InsertSlide { index: *index, slide: slide.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
