//! 📦 `bounds` — the 3D min/max bounding box over every entity's own point-valued fields, folding
//! both top-level `entities` AND every `blocks[].entities` (a block reference's own nested
//! entities are real placed geometry, not opaque — this walk includes them, ignoring
//! `Insert.position`'s own referenced block content since resolving `block_name` back to its
//! `DxfBlock` and applying its own transform (`scale`/`rotation`) is a referential-invariant
//! concern out of scope for this pure structural fold, same rule semio-cad applies to its own
//! `Insert`). `Circle`/`Arc` contribute their FULL circle's bounding box (`center ± radius` on
//! every axis) rather than the arc's own tighter angular sweep — an honest superset, not a
//! heuristic understatement. `Other` (raw-retained unknown entity kinds — `3DFACE`, `POINT`,
//! `DIMENSION`, …) carries no typed point field, so it is counted in `entityCount` but
//! contributes nothing to the fold — never fabricated. A pure whole-snapshot scalar (one min/max
//! fold) — no `InferredField` needed.

use crate::schema::snapshot::{DxfEntity, DxfSnapshot};

//#region 🔖️Bounds
/// 📦️ Dxf's entity-derived 3D bounding box.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DxfBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub entity_count: u32,
}

/// 🩹 Hand-rolled: an empty entity set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute` returns for zero entities (the fold's identity value), keeping the inference-default
/// law correct.
impl Default for DxfBounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], entity_count: 0 }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, p: [f64; 3]) {
    if !*seen {
        *min = p;
        *max = p;
        *seen = true;
        return;
    }
    for i in 0..3 {
        min[i] = min[i].min(p[i]);
        max[i] = max[i].max(p[i]);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_sphere(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, center: [f64; 3], radius: f64) {
    let lo = [center[0] - radius, center[1] - radius, center[2] - radius];
    let hi = [center[0] + radius, center[1] + radius, center[2] + radius];
    expand(min, max, seen, lo);
    expand(min, max, seen, hi);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_entity(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, entity: &DxfEntity) {
    match entity {
        DxfEntity::Line { start, end, .. } => {
            expand(min, max, seen, *start);
            expand(min, max, seen, *end);
        }
        DxfEntity::Circle { center, radius, .. } | DxfEntity::Arc { center, radius, .. } => {
            expand_sphere(min, max, seen, *center, *radius);
        }
        DxfEntity::Polyline { vertices, .. } => {
            for v in vertices {
                expand(min, max, seen, [v.x, v.y, v.z]);
            }
        }
        DxfEntity::Text { position, .. } => expand(min, max, seen, *position),
        DxfEntity::Solid { points, .. } => {
            for p in points {
                expand(min, max, seen, *p);
            }
        }
        DxfEntity::Insert { position, .. } => expand(min, max, seen, *position),
        DxfEntity::Other { .. } => {}
    }
}

/// 📦️ Computes [`DxfBounds`] over every top-level `entities` record plus every block's own
/// nested `entities` — see module doc comment for the per-variant bounding rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_dxf_bounds(snapshot: &DxfSnapshot) -> DxfBounds {
    let mut min = [0.0, 0.0, 0.0];
    let mut max = [0.0, 0.0, 0.0];
    let mut seen = false;
    let mut entity_count = 0u32;

    let entities = snapshot.entities.iter().chain(snapshot.blocks.iter().flat_map(|b| b.entities.iter()));
    for entity in entities {
        entity_count += 1;
        expand_entity(&mut min, &mut max, &mut seen, entity);
    }

    DxfBounds { min, max, entity_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
