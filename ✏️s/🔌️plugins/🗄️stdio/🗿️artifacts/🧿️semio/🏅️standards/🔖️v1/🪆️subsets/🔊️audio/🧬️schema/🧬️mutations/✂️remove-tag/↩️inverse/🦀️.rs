//! ↩️ Inverse for `RemoveTag`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioAudioOp>(payload: &super::RemoveTag, base: &SemioAudioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let super::RemoveTag { index } = payload;
    Ok(vec![match base.tags.get(*index) {
        Some(t) => Op::from(insert_tag::InsertTag { index: *index, tag: t.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
