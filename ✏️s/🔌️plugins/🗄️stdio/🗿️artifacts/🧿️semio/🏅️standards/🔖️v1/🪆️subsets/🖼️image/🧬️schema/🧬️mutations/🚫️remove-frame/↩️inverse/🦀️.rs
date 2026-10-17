//! ↩️ Inverse for `RemoveFrame`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_frame;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveFrame, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_frame::InsertFrame>,
{
    let super::RemoveFrame { index } = payload;
    Ok(vec![match base.frames.get(*index) {
        Some(frame) => Op::from(insert_frame::InsertFrame { index: *index, frame: frame.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
