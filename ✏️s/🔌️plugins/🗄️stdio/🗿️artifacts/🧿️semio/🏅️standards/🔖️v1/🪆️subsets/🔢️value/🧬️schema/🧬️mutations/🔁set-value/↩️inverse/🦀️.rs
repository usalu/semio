//! ↩️ Inverse for `SetValue`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetValue, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::SetValue { path, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(old) => vec![SemioValueMutation::SetValue(set_value::SetValue { path: path.clone(), value: old.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
