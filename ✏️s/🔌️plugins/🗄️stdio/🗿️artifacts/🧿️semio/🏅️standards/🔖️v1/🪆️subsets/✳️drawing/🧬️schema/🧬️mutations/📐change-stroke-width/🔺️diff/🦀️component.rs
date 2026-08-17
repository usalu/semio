//! 🔺️ `change-stroke-width` — sparse diff construction; an absent `style_name` is
//! `mutation.target-missing` (Error, empty diff); a `new_width` identical to the style's current
//! stroke width is `mutation.no-op` (Warning, empty diff); a non-finite `new_width` is
//! `mutation.invariant` (Fatal, empty diff).

use super::mutation::ChangeStrokeWidth;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawStyleDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrokeWidth, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(old) = base.styles.iter().find(|s| s.name == payload.style_name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Style \"{}\" does not exist.", payload.style_name), [payload.style_name.clone()]);
    };
    if let Some(w) = payload.new_width {
        if !w.is_finite() {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Style \"{}\" new stroke width is not finite.", payload.style_name), [payload.style_name.clone()]);
        }
    }
    if old.stroke_width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Style \"{}\" already has that stroke width.", payload.style_name));
    }
    protocol::MutationOutcome::new(SemioDrawingDiff {
        canvas: None,
        styles: Some(NamedTripleDiff {
            removed: Vec::new(),
            modified: vec![NamedModified { key: payload.style_name.clone(), diff: DrawStyleDiff { fill: None, stroke: None, stroke_width: Some(payload.new_width), opacity: None } }],
            added: Vec::new(),
        }),
        layers: None,
    })
}
//#endregion 🔖️Diff
