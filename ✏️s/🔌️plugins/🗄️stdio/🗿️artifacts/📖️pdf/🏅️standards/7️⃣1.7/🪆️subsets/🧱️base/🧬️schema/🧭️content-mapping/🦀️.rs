//! 🧭️ Resource identity projection over decoded PDF operators.
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfPropertyList};
use std::collections::HashMap;

/// 📚 A resource dictionary's name → id maps per category (what a content stream's names
/// resolve to).
#[derive(Clone, Debug, Default)]
pub struct ResourceMap {
    pub fonts: HashMap<String, String>,
    pub x_objects: HashMap<String, String>,
    pub ext_g_states: HashMap<String, String>,
    pub shadings: HashMap<String, String>,
    pub patterns: HashMap<String, String>,
    pub color_spaces: HashMap<String, String>,
    pub properties: HashMap<String, String>,
}

/// 🔁 Rewrites every resource name in `ops` through `map` (names the map does not know stay).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rename_content(ops: Vec<PdfOp>, map: &ResourceMap) -> Vec<PdfOp> {
    ops.into_iter()
        .map(|mut op| {
            if let Some((name, id)) = rename_target(&mut op, map) { *name = id.to_owned(); }
            op
        })
        .collect()
}

/// 🎯️ The resource name `op` selects and the id `map` renames it to, when the map knows that name; [`rename_content`] applies it to every operator, a bounded lifter one operator per unit.
pub fn rename_target<'a, 'm>(op: &'a mut PdfOp, map: &'m ResourceMap) -> Option<(&'a mut String, &'m str)> {
    let (table, name) = match op {
        PdfOp::SetFont { name, .. } => (&map.fonts, name),
        PdfOp::PaintXObject { name } => (&map.x_objects, name),
        PdfOp::SetExtGState { name } => (&map.ext_g_states, name),
        PdfOp::PaintShading { name } => (&map.shadings, name),
        PdfOp::SetStrokeColorN { pattern: Some(name), .. } | PdfOp::SetFillColorN { pattern: Some(name), .. } => (&map.patterns, name),
        PdfOp::SetStrokeColorSpace { name } | PdfOp::SetFillColorSpace { name } => (&map.color_spaces, name),
        PdfOp::MarkedContentPointWithProperties { properties: PdfPropertyList::Named { name }, .. } | PdfOp::BeginMarkedContentWithProperties { properties: PdfPropertyList::Named { name }, .. } => (&map.properties, name),
        _ => return None,
    };
    let id = table.get(name.as_str())?;
    Some((name, id.as_str()))
}
