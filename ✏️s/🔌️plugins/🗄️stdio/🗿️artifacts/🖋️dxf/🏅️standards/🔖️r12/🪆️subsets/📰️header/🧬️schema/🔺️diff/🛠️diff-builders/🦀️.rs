//! 🛠️ Builders of the DXF diff: the per-collection field-change helpers and the one-edit diff constructors the mutations use. See [`super`].

use super::*;

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level field-change wrappers, exposed to `🧬️mutations` so `SetLayer`/`SetStyle`/
// `SetLinetype`/`SetEntity`/`SetBlock`'s `diff()` can compute a sparse per-field patch without the
// private `DxfNamedElem`/`DxfIndexElem` traits themselves leaving this module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn header_var_field_changes(a: &DxfHeaderVar, b: &DxfHeaderVar) -> DxfHeaderVarDiff {
    <DxfHeaderVar as DxfNamedElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layer_field_changes(a: &DxfLayer, b: &DxfLayer) -> DxfLayerDiff {
    <DxfLayer as DxfNamedElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn style_field_changes(a: &DxfStyle, b: &DxfStyle) -> DxfStyleDiff {
    <DxfStyle as DxfNamedElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn linetype_field_changes(a: &DxfLinetype, b: &DxfLinetype) -> DxfLinetypeDiff {
    <DxfLinetype as DxfNamedElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn block_field_changes(a: &DxfBlock, b: &DxfBlock) -> DxfBlockDiff {
    <DxfBlock as DxfIndexElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_header_var(index: usize, name: &str, header_var: DxfHeaderVar, existed: bool) -> DxfDiff {
    if existed {
        DxfDiff {
            header_vars: Some(DxfHeaderVarsDiff {
                removed: vec![],
                modified: vec![DxfHeaderVarModified { name: name.to_string(), diff: DxfHeaderVarDiff { group_code: Some(header_var.group_code), value: Some(header_var.value), extra_group_codes: Some(header_var.extra_group_codes) } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![], modified: vec![], added: vec![DxfHeaderVarAdded { index, header_var }] }), ..Default::default() }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_header_var(name: &str) -> DxfDiff {
    DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_layer(index: usize, layer: DxfLayer) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![], added: vec![DxfLayerAdded { index, layer }] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_layer(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_layer(name: &str, diff: DxfLayerDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![DxfLayerModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_style(index: usize, style: DxfStyle) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![], added: vec![DxfStyleAdded { index, style }] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_style(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_style(name: &str, diff: DxfStyleDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![DxfStyleModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_linetype(index: usize, linetype: DxfLinetype) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![], added: vec![DxfLinetypeAdded { index, linetype }] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_linetype(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_linetype(name: &str, diff: DxfLinetypeDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![DxfLinetypeModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_entity(index: usize, entity: DxfEntity) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![], added: vec![DxfEntityAdded { index, entity }] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_entity(index: usize) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_entity(index: usize, diff: DxfEntityDiff) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![DxfEntityModified { index, diff }], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_block(index: usize, block: DxfBlock) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![], added: vec![DxfBlockAdded { index, block }] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_block(index: usize) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_block(index: usize, diff: DxfBlockDiff) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![DxfBlockModified { index, diff }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders
