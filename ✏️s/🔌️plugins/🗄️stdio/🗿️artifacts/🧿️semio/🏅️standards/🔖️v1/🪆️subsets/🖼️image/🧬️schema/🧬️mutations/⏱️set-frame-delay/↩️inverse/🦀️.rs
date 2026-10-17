//! ↩️ Inverse for `SetFrameDelay`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_frame_delay;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetFrameDelay, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_frame_delay::SetFrameDelay>,
{
    let super::SetFrameDelay { index, .. } = payload;
    Ok(vec![match base.frames.get(*index) {
        Some(frame) => Op::from(set_frame_delay::SetFrameDelay { index: *index, delay_ms: frame.delay_ms }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
