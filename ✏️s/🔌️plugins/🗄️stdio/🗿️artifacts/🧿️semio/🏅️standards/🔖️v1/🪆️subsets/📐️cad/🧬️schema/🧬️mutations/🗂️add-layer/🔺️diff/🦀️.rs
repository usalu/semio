//! 🔺️ Diff for `AddLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_layer(base, &payload.layer.name).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Layer \"{}\" already exists.", payload.layer.name), [payload.layer.name.clone()]);
    }
    let super::AddLayer { layer, at } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.layers.len(), |at| at.min(base.layers.len())), item: layer.clone() }] }), blocks: None, entities: None })
}
//#endregion 🔖️Diff
