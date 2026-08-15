//! 🔺️ `change-stroke-color` — sparse diff construction; a no-op when `style_name` is absent.

use super::mutation::ChangeStrokeColor;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawStyleDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrokeColor, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) if old.stroke != payload.new_color => SemioDrawingDiff {
            canvas: None,
            styles: Some(NamedTripleDiff {
                removed: Vec::new(),
                modified: vec![NamedModified { key: payload.style_name.clone(), diff: DrawStyleDiff { fill: None, stroke: Some(payload.new_color), stroke_width: None, opacity: None } }],
                added: Vec::new(),
            }),
            layers: None,
        },
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
