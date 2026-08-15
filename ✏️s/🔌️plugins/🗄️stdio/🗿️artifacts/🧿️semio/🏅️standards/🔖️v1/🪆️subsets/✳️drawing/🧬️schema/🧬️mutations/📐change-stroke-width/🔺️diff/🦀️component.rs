//! 🔺️ `change-stroke-width` — sparse diff construction; a no-op when `style_name` is absent.

use super::mutation::ChangeStrokeWidth;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawStyleDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrokeWidth, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) if old.stroke_width != payload.new_width => SemioDrawingDiff {
            canvas: None,
            styles: Some(NamedTripleDiff {
                removed: Vec::new(),
                modified: vec![NamedModified { key: payload.style_name.clone(), diff: DrawStyleDiff { fill: None, stroke: None, stroke_width: Some(payload.new_width), opacity: None } }],
                added: Vec::new(),
            }),
            layers: None,
        },
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
