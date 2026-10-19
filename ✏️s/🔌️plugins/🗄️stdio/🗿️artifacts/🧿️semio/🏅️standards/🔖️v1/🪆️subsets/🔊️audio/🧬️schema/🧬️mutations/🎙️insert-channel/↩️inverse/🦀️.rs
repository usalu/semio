//! ↩️ Inverse for `InsertChannel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioAudioOp>(payload: &super::InsertChannel, base: &SemioAudioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let super::InsertChannel { index, .. } = payload;
    Ok(vec![Op::from(remove_channel::RemoveChannel { index: (*index).min(base.channels.len()) })])
}
//#endregion 🔖️Inverse
