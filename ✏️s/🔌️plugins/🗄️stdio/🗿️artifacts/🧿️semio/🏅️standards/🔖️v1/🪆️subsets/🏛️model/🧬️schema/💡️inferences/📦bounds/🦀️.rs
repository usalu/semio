//! 📦 `bounds` — one named inference: the model's own POSITION envelope, folded over every
//! `SpatialNode.placement.translation` and every `SemioModelElement.placement.translation` this
//! subset owns outright. `GeometryRef` only resolves BY ID into the sibling `brep`/`mesh` subsets
//! (this facet's own module doc comment) — inlining THEIR geometry here would violate the
//! composition boundary those subsets' own snapshots enforce, so this is honestly a placement
//! envelope, not a geometry bounding box. A plain whole-snapshot fold — no `InferredField`/
//! incremental caching needed for a single min/max pass (same ruling `cad`'s own `📦bounds` facet
//! reaches for its own entity-point fold).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

//#region 🔖️Bounds
/// 📦️ Semio model position envelope.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioModelBounds {
    pub min: SemioPoint3,
    pub max: SemioPoint3,
    pub entity_count: u32,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand(min: &mut SemioPoint3, max: &mut SemioPoint3, p: &SemioPoint3, seen_any: &mut bool) {
    if !*seen_any {
        *min = *p;
        *max = *p;
        *seen_any = true;
        return;
    }
    min.x = min.x.min(p.x);
    min.y = min.y.min(p.y);
    min.z = min.z.min(p.z);
    max.x = max.x.max(p.x);
    max.y = max.y.max(p.y);
    max.z = max.z.max(p.z);
}

/// 📦️ Computes [`SemioModelBounds`] — pure, total, O(spatial + elements). An entity-less snapshot
/// returns `SemioModelBounds::default()` (min == max == origin, `entity_count: 0`), matching the
/// derived zero struct — the same degenerate-empty convention `image`'s own header-fold facet
/// documents.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_model_bounds(snapshot: &SemioModelSnapshot) -> SemioModelBounds {
    let mut min = SemioPoint3::default();
    let mut max = SemioPoint3::default();
    let mut seen_any = false;
    let mut entity_count = 0u32;
    for node in &snapshot.spatial {
        expand(&mut min, &mut max, &node.placement.translation, &mut seen_any);
        entity_count += 1;
    }
    for element in &snapshot.elements {
        expand(&mut min, &mut max, &element.placement.translation, &mut seen_any);
        entity_count += 1;
    }
    SemioModelBounds { min, max, entity_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
