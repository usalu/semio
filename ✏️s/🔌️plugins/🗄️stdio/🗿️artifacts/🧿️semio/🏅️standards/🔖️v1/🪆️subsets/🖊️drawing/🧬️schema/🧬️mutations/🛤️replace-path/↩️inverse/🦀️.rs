//! ↩️ Inverse for `ReplacePath`.

use crate::schema::diff::node_at;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReplacePath, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::replace_path::ReplacePath>,
{
    Ok((|| {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { segments, .. }) => vec![Op::from(super::ReplacePath { at: payload.at.clone(), new_segments: segments.clone() })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
