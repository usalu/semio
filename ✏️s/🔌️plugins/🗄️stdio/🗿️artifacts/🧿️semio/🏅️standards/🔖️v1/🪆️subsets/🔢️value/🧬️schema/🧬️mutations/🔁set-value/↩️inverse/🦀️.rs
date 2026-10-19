//! ↩️ Inverse for `SetValue`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use crate::path::resolve;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetValue, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::set_value::SetValue>,
{
    let super::SetValue { path, .. } = payload;
    Ok(match resolve(&base.root, path) {
        Some(old) => vec![Op::from(crate::set_value::SetValue { path: path.clone(), value: old.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
