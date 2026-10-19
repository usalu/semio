//! ↩️ Inverse for `RemoveMapEntry`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use crate::path::resolve;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveMapEntry, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::set_map_entry::SetMapEntry>,
{
    let super::RemoveMapEntry { path, key } = payload;
    Ok(match resolve(&base.root, path) {
        Some(SemioValue::Map { entries }) => match entries.iter().position(|e| &e.key == key) {
            Some(pos) => {
                vec![Op::from(crate::set_map_entry::SetMapEntry { path: path.clone(), key: key.clone(), value: entries[pos].value.clone(), at: Some(pos) })]
            }
            None => Vec::new(),
        },
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
