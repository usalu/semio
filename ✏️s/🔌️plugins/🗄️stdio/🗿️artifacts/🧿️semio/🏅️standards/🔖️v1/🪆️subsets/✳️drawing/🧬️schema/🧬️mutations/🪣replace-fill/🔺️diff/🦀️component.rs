//! 🔺️ `replace-fill` — sparse diff construction; a no-op when `style_name` is absent.

use super::mutation::ReplaceFill;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawStyleDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceFill, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) if old.fill != payload.new_fill => SemioDrawingDiff {
            canvas: None,
            styles: Some(NamedTripleDiff { removed: Vec::new(), modified: vec![NamedModified { key: payload.style_name.clone(), diff: DrawStyleDiff { fill: Some(payload.new_fill), stroke: None, stroke_width: None, opacity: None } }], added: Vec::new() }),
            layers: None,
        },
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
