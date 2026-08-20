//! 🔺️ `replace-fill` — sparse diff construction; an absent `style_name` is
//! `mutation.target-missing` (Error, empty diff); a `new_fill` identical to the style's current
//! fill is `mutation.no-op` (Warning, empty diff); a non-finite `new_fill` component is
//! `mutation.invariant` (Fatal, empty diff).

use super::mutation::ReplaceFill;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawStyleDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ReplaceFill, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(old) = base.styles.iter().find(|s| s.name == payload.style_name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Style \"{}\" does not exist.", payload.style_name), [payload.style_name.clone()]);
    };
    if let Some(c) = payload.new_fill {
        if !c.r.is_finite() || !c.g.is_finite() || !c.b.is_finite() || !c.a.is_finite() {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Style \"{}\" new fill has a non-finite component.", payload.style_name), [payload.style_name.clone()]);
        }
    }
    if old.fill == payload.new_fill {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Style \"{}\" already has that fill.", payload.style_name));
    }
    protocol::MutationOutcome::new(SemioDrawingDiff {
        canvas: None,
        styles: Some(NamedTripleDiff {
            removed: Vec::new(),
            modified: vec![NamedModified { key: payload.style_name.clone(), diff: DrawStyleDiff { fill: Some(payload.new_fill), stroke: None, stroke_width: None, opacity: None } }],
            added: Vec::new(),
        }),
        layers: None,
    })
}
//#endregion 🔖️Diff
