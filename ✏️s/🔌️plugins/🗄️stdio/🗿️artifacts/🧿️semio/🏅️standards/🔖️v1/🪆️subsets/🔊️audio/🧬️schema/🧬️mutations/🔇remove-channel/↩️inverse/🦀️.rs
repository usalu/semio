//! ↩️ Inverse for `RemoveChannel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioAudioOp>(payload: &super::RemoveChannel, base: &SemioAudioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let super::RemoveChannel { index } = payload;
    Ok(vec![match base.channels.get(*index) {
        Some(c) => Op::from(insert_channel::InsertChannel { index: *index, channel: c.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
