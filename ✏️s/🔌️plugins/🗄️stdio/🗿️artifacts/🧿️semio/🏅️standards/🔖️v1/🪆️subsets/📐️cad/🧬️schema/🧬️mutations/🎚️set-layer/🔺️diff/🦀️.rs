//! 🔺️ Diff for `SetLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_layer(base, &payload.name).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    }
    let super::SetLayer { name, color_index, line_type, visible } = payload;
    protocol::MutationOutcome::new(wrap_layer_diff(name, CadLayerDiff { color_index: *color_index, line_type: line_type.clone(), visible: *visible }))
}
//#endregion 🔖️Diff
