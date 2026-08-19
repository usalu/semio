//! 📦 `bounds` — the spatial min/max bounding box over every real `IFCCARTESIANPOINT((x,y,z));`
//! entity in `entities` (buildingSMART IFC4, ISO 10303-21 Part-21 syntax over IFC4's own EXPRESS
//! schema — https://www.iso.org/standard/70303.html). `IfcSnapshot` owns its own typed model
//! (`IfcEntity`/`IfcValue`, never the shared `Part21Document` — see the sibling
//! `📸️snapshot/🦀️component.rs` doc comment), so this fold matches on `entity.name` directly and
//! reads coordinates via `IfcValue::as_aggregate`/`IfcValue::as_real`. A point's missing 3rd (z)
//! component defaults to `0.0` — the same convention `engine::spatial::cartesian_point` already
//! uses for honestly-2D placements. A pure whole-snapshot scalar (one min/max fold) — no
//! `InferredField` needed.

use crate::artifacts::ifc::schema::snapshot::IfcValue;
use crate::artifacts::ifc::IfcSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ IFC4's `IFCCARTESIANPOINT`-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub point_count: u32,
}

/// 🩹 Hand-rolled: an empty entity set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute_ifc_bounds` returns for zero `IFCCARTESIANPOINT` entities (the fold's identity
/// value), keeping the inference-default law correct.
impl Default for IfcBounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], point_count: 0 }
    }
}

/// 📦️ Computes [`IfcBounds`] by folding every `IFCCARTESIANPOINT` entity's coordinate aggregate
/// (`args.first()`, no leading label arg — `IfcCartesianPoint.Coordinates` is IFC4's own sole
/// attribute, unlike STEP AP214's `CARTESIAN_POINT('label',(x,y,z))` shape). `name` is matched
/// case-insensitively even though this format's own convention always persists it uppercase.
pub async fn compute_ifc_bounds(snapshot: &IfcSnapshot) -> IfcBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut point_count = 0u32;

    for entity in &snapshot.entities {
        if !entity.name.eq_ignore_ascii_case("IFCCARTESIANPOINT") {
            continue;
        }
        let Some(coords) = entity.args.first().and_then(IfcValue::as_aggregate) else { continue };
        let p = [coords.first().and_then(IfcValue::as_real).unwrap_or(0.0), coords.get(1).and_then(IfcValue::as_real).unwrap_or(0.0), coords.get(2).and_then(IfcValue::as_real).unwrap_or(0.0)];
        point_count += 1;
        if !seen {
            min = p;
            max = p;
            seen = true;
        } else {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
    }

    IfcBounds { min, max, point_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::ifc::schema::snapshot::IfcEntity;
    use crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA;

    async fn point_entity(id: u64, x: f64, y: f64, z: f64) -> IfcEntity {
        IfcEntity { id, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(x), IfcValue::Real(y), IfcValue::Real(z)])], complex: Vec::new() }
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_matches_hand_built_entity_extent() {
        let snapshot = IfcSnapshot {
            schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
            header: Default::default(),
            entities: vec![point_entity(1, 0.0, 0.0, 0.0), point_entity(2, -3.0, 6.0, 12.0), point_entity(3, 9.0, -1.0, 4.0), IfcEntity { id: 4, name: "IFCOWNERHISTORY".into(), args: vec![IfcValue::Unset], complex: Vec::new() }],
        };
        let bounds = compute_ifc_bounds(&snapshot);
        assert_eq!(bounds.min, [-3.0, -1.0, 0.0]);
        assert_eq!(bounds.max, [9.0, 6.0, 12.0]);
        assert_eq!(bounds.point_count, 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), header: Default::default(), entities: vec![point_entity(1, 1.0, 1.0, 1.0)] };
        assert_eq!(compute_ifc_bounds(&snapshot), compute_ifc_bounds(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_ifc_bounds(&IfcSnapshot::default()), IfcBounds::default());
    }
}
//#endregion 🧪️Tests
