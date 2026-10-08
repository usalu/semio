//! ↩️ Inverse for `SetHeadingLevel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetHeadingLevel, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetHeadingLevel { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Heading { level, .. }) => vec![SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path: path.clone(), level: *level })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
