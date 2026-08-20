//! 📦 `bounds` — one named inference: the planar min/max bounding box over every entity's own
//! point-valued fields, folding both top-level `entities` AND every `blocks[].entities` (a block
//! reference's own nested entities are real placed geometry, not opaque — this walk includes
//! them, ignoring `Insert.insertion_point`'s own block content since resolving `block_name` back
//! to its `CadBlock` and applying its transform is a referential-invariant concern the composer's
//! `SemioCadValidator` owns, not this pure structural fold). `Arc`/`Circle` contribute their
//! full circle's bounding box (`center ± radius`) rather than the arc's own tighter sweep — an
//! honest superset, not a heuristic understatement; `Ellipse` likewise uses
//! `center ± |major_axis_end - center|` as its bounding radius. A pure whole-snapshot scalar (one
//! min/max fold) — no `InferredField` needed.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadEntity, CadEntityRecord, SemioCadSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Semio cad's entity-derived planar bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioCadBounds {
    pub min: SemioPoint2,
    pub max: SemioPoint2,
    pub entity_count: u32,
}

/// 🩹 Hand-rolled: `SemioPoint2` has no `Default` bounding-box meaning on its own, and an empty
/// entity set has no honest min/max — `[0,0]`/`[0,0]` matches what `compute` returns for zero
/// entities (the fold's identity value), keeping the inference-default law correct.
impl Default for SemioCadBounds {
    fn default() -> Self {
        Self { min: SemioPoint2 { x: 0.0, y: 0.0 }, max: SemioPoint2 { x: 0.0, y: 0.0 }, entity_count: 0 }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand(min: &mut SemioPoint2, max: &mut SemioPoint2, seen: &mut bool, p: SemioPoint2) {
    if !*seen {
        *min = p;
        *max = p;
        *seen = true;
        return;
    }
    min.x = min.x.min(p.x);
    min.y = min.y.min(p.y);
    max.x = max.x.max(p.x);
    max.y = max.y.max(p.y);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_circle(min: &mut SemioPoint2, max: &mut SemioPoint2, seen: &mut bool, center: SemioPoint2, radius: f64) {
    expand(min, max, seen, SemioPoint2 { x: center.x - radius, y: center.y - radius });
    expand(min, max, seen, SemioPoint2 { x: center.x + radius, y: center.y + radius });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_entity(min: &mut SemioPoint2, max: &mut SemioPoint2, seen: &mut bool, entity: &CadEntity) {
    match entity {
        CadEntity::Line { a, b } => {
            expand(min, max, seen, *a);
            expand(min, max, seen, *b);
        }
        CadEntity::Arc { center, radius, .. } | CadEntity::Circle { center, radius } => {
            expand_circle(min, max, seen, *center, *radius);
        }
        CadEntity::Ellipse { center, major_axis_end, .. } => {
            let radius = ((major_axis_end.x - center.x).powi(2) + (major_axis_end.y - center.y).powi(2)).sqrt();
            expand_circle(min, max, seen, *center, radius);
        }
        CadEntity::Polyline { vertices, .. } => {
            for v in vertices {
                expand(min, max, seen, *v);
            }
        }
        CadEntity::Text { position, .. } => expand(min, max, seen, *position),
        CadEntity::Insert { insertion_point, .. } => expand(min, max, seen, *insertion_point),
        CadEntity::Solid { p1, p2, p3, p4 } => {
            for p in [p1, p2, p3, p4] {
                expand(min, max, seen, *p);
            }
        }
        CadEntity::Dimension { def_point, text_position, .. } => {
            expand(min, max, seen, *def_point);
            expand(min, max, seen, *text_position);
        }
    }
}

/// 📦️ Computes [`SemioCadBounds`] over every top-level `entities` record plus every block's own
/// nested `entities` — see module doc comment for the per-variant bounding rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_cad_bounds(snapshot: &SemioCadSnapshot) -> SemioCadBounds {
    let mut min = SemioPoint2 { x: 0.0, y: 0.0 };
    let mut max = SemioPoint2 { x: 0.0, y: 0.0 };
    let mut seen = false;
    let mut entity_count = 0u32;

    let records: Vec<&CadEntityRecord> = snapshot.entities.iter().chain(snapshot.blocks.iter().flat_map(|b| b.entities.iter())).collect();
    for record in &records {
        entity_count += 1;
        expand_entity(&mut min, &mut max, &mut seen, &record.entity);
    }

    SemioCadBounds { min, max, entity_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntityRecord, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn point(x: f64, y: f64) -> SemioPoint2 {
        SemioPoint2 { x, y }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn record(handle: &str, layer: &str, entity: CadEntity) -> CadEntityRecord {
        CadEntityRecord { handle: handle.into(), layer: layer.into(), entity }
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_matches_hand_built_entity_extent() {
        let snapshot = SemioCadSnapshot {
            schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: Vec::new(),
            blocks: vec![CadBlock { name: "b1".into(), base_point: point(0.0, 0.0), entities: vec![record("h2", "0", CadEntity::Circle { center: point(5.0, 7.5), radius: 1.0 })] }],
            entities: vec![record("h0", "0", CadEntity::Line { a: point(-2.0, 1.0), b: point(0.0, 2.0) }), record("h1", "0", CadEntity::Polyline { vertices: vec![point(1.0, 1.0), point(3.0, 4.0)], closed: false })],
        };
        let bounds = compute_semio_cad_bounds(&snapshot);
        assert_eq!(bounds.min, point(-2.0, 1.0));
        assert_eq!(bounds.max, point(6.0, 8.5));
        assert_eq!(bounds.entity_count, 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = SemioCadSnapshot { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers: Vec::new(), blocks: Vec::new(), entities: vec![record("h0", "0", CadEntity::Line { a: point(0.0, 0.0), b: point(1.0, 1.0) })] };
        assert_eq!(compute_semio_cad_bounds(&snapshot), compute_semio_cad_bounds(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_cad_bounds(&SemioCadSnapshot::default()), SemioCadBounds::default());
    }
}
//#endregion 🧪️Tests
