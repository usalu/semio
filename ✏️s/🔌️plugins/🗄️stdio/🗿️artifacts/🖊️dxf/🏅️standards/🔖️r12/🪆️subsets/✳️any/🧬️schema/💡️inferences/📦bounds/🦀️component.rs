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

use crate::artifacts::dxf::schema::snapshot::{DxfEntity, DxfSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Dxf's entity-derived 3D bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub entity_count: u32,
}

/// 🩹 Hand-rolled: an empty entity set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute` returns for zero entities (the fold's identity value), keeping the inference-default
/// law correct.
impl Default for DxfBounds {
    async fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], entity_count: 0 }
    }
}

async fn expand(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, p: [f64; 3]) {
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

async fn expand_sphere(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, center: [f64; 3], radius: f64) {
    let lo = [center[0] - radius, center[1] - radius, center[2] - radius];
    let hi = [center[0] + radius, center[1] + radius, center[2] + radius];
    expand(min, max, seen, lo);
    expand(min, max, seen, hi);
}

async fn expand_entity(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, entity: &DxfEntity) {
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
pub async fn compute_dxf_bounds(snapshot: &DxfSnapshot) -> DxfBounds {
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
mod tests {
    use super::*;
    use crate::artifacts::dxf::schema::snapshot::DxfBlock;

    #[test]
    async fn bounds_matches_hand_built_entity_extent() {
        let snapshot = DxfSnapshot {
            schema: "s.stdio.dxf".into(),
            header_vars: Vec::new(),
            tables: Default::default(),
            other_tables: Vec::new(),
            blocks: vec![DxfBlock { name: "b1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Circle { center: [5.0, 7.5, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
            entities: vec![DxfEntity::Line { start: [-2.0, 1.0, 0.0], end: [0.0, 2.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![] }],
        };
        let bounds = compute_dxf_bounds(&snapshot);
        assert_eq!(bounds.min, [-2.0, 1.0, -1.0]);
        assert_eq!(bounds.max, [6.0, 8.5, 1.0]);
        assert_eq!(bounds.entity_count, 3);
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = DxfSnapshot {
            schema: "s.stdio.dxf".into(),
            header_vars: Vec::new(),
            tables: Default::default(),
            other_tables: Vec::new(),
            blocks: Vec::new(),
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 1.0], layer: "0".into(), unknown_group_codes: vec![] }],
        };
        assert_eq!(compute_dxf_bounds(&snapshot), compute_dxf_bounds(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_dxf_bounds(&DxfSnapshot::default()), DxfBounds::default());
    }
}
//#endregion 🧪️Tests
