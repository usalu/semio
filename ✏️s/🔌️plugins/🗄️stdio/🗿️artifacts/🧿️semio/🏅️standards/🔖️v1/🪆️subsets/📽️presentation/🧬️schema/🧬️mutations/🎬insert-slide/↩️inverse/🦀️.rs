//! ↩️ Inverse for `InsertSlide`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_slide;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertSlide, _base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_slide::RemoveSlide>,
{
    let super::InsertSlide { index, .. } = payload;
    Ok(vec![Op::from(remove_slide::RemoveSlide { index: *index })])
}
//#endregion 🔖️Inverse
