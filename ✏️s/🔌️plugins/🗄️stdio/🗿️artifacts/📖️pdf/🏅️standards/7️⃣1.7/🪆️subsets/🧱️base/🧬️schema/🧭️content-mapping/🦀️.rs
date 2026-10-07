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
    let rename = |table: &HashMap<String, String>, name: &mut String| {
        if let Some(id) = table.get(name.as_str()) {
            *name = id.clone();
        }
    };
    ops.into_iter()
        .map(|mut op| {
            match &mut op {
                PdfOp::SetFont { name, .. } => rename(&map.fonts, name),
                PdfOp::PaintXObject { name } => rename(&map.x_objects, name),
                PdfOp::SetExtGState { name } => rename(&map.ext_g_states, name),
                PdfOp::PaintShading { name } => rename(&map.shadings, name),
                PdfOp::SetStrokeColorN { pattern: Some(name), .. } | PdfOp::SetFillColorN { pattern: Some(name), .. } => rename(&map.patterns, name),
                PdfOp::SetStrokeColorSpace { name } | PdfOp::SetFillColorSpace { name } => rename(&map.color_spaces, name),
                PdfOp::MarkedContentPointWithProperties { properties: PdfPropertyList::Named { name }, .. } | PdfOp::BeginMarkedContentWithProperties { properties: PdfPropertyList::Named { name }, .. } => rename(&map.properties, name),
                _ => {}
            }
            op
        })
        .collect()
}
