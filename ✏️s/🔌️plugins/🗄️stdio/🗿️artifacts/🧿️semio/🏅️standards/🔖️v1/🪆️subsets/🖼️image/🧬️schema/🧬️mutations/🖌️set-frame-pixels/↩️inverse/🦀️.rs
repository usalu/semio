//! ↩️ Inverse for `SetFramePixels`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_frame_pixels;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetFramePixels, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_frame_pixels::SetFramePixels>,
{
    let super::SetFramePixels { index, .. } = payload;
    Ok(vec![match base.frames.get(*index) {
        Some(frame) => Op::from(set_frame_pixels::SetFramePixels { index: *index, rgba8: frame.rgba8.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
