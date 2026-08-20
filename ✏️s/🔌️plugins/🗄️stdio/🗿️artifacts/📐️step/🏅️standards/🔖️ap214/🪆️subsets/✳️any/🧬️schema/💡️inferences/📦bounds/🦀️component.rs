//! 📦 `bounds` — the spatial min/max bounding box over every real
//! `CARTESIAN_POINT('label',(x,y,z));` entity in `entities` (ISO 10303-21 AP214, Automotive
//! Design — https://www.iso.org/standard/63151.html). Unlike `🏗️ifc`'s `IFCCARTESIANPOINT`
//! (whose sole EXPRESS attribute IS the coordinate list), AP214's `CARTESIAN_POINT` has a leading
//! `label: STRING` attribute before `coordinates: LIST OF REAL` (confirmed against this
//! subset's own `📸️snapshot/🦀️component.rs` fixture: `CARTESIAN_POINT('',(0.,0.,0.));` — args[0]
//! is the label, args[1] the coordinate aggregate) — so this fold scans `entity.args` for the
//! first `Aggregate` whose members are ALL real/integer-convertible with 2-4 items, rather than
//! assuming `args.first()`, honestly tolerating either arg order a real AP214 exporter might use.
//! `StepValue` has no `as_aggregate`/`as_real` helper methods of its own (unlike `IfcValue`), so
//! this fold matches its variants directly. A point's missing 3rd (z) component defaults to
//! `0.0`, same convention `🏗️ifc`'s bounds fold uses for honestly-2D placements. A pure
//! whole-snapshot scalar (one min/max fold) — no `InferredField` needed.

use crate::artifacts::step::schema::snapshot::StepValue;
use crate::artifacts::step::StepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ STEP AP214's `CARTESIAN_POINT`-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub point_count: u32,
}

/// 🩹 Hand-rolled: an empty entity set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute_step_bounds` returns for zero `CARTESIAN_POINT` entities (the fold's identity value),
/// keeping the inference-default law correct.
impl Default for StepBounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], point_count: 0 }
    }
}

/// 🔢️ `StepValue::Real`/`Integer` -> `f64`, mirroring `IfcValue::as_real`/`Part21Value::as_real`
/// since `StepValue` itself has no such helper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn step_value_as_real(v: &StepValue) -> Option<f64> {
    match v {
        StepValue::Real(r) => Some(*r),
        StepValue::Integer(i) => Some(*i as f64),
        _ => None,
    }
}

/// 🔍️ Finds `entity.args`'s coordinate aggregate: the first `Aggregate` of 2-4 members, every
/// member real/integer-convertible — tolerates AP214's real `('label',(x,y,z))` arg order (the
/// aggregate at index 1) without assuming a fixed position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coordinate_aggregate(args: &[StepValue]) -> Option<Vec<f64>> {
    args.iter().find_map(|v| match v {
        StepValue::Aggregate(items) if (2..=4).contains(&items.len()) => {
            let reals: Vec<f64> = items.iter().filter_map(step_value_as_real).collect();
            (reals.len() == items.len()).then_some(reals)
        }
        _ => None,
    })
}

/// 📦️ Computes [`StepBounds`] by folding every `CARTESIAN_POINT` entity's coordinate aggregate
/// (found via [`coordinate_aggregate`], honest about AP214's real leading-label arg shape).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_step_bounds(snapshot: &StepSnapshot) -> StepBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut point_count = 0u32;

    for entity in &snapshot.entities {
        if !entity.name.eq_ignore_ascii_case("CARTESIAN_POINT") {
            continue;
        }
        let Some(coords) = coordinate_aggregate(&entity.args) else { continue };
        let p = [coords.first().copied().unwrap_or(0.0), coords.get(1).copied().unwrap_or(0.0), coords.get(2).copied().unwrap_or(0.0)];
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

    StepBounds { min, max, point_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::step::schema::snapshot::StepEntity;
    use crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn point_entity(id: u64, x: f64, y: f64, z: f64) -> StepEntity {
        StepEntity { id, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String(String::new()), StepValue::Aggregate(vec![StepValue::Real(x), StepValue::Real(y), StepValue::Real(z)])], complex: Vec::new() }
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_matches_hand_built_entity_extent() {
        let snapshot = StepSnapshot {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: Default::default(),
            entities: vec![point_entity(1, 0.0, 0.0, 0.0), point_entity(2, -5.0, 2.0, 7.0), point_entity(3, 10.0, 3.0, -1.0), StepEntity { id: 4, name: "DIRECTION".into(), args: vec![StepValue::String(String::new())], complex: Vec::new() }],
        };
        let bounds = compute_step_bounds(&snapshot);
        assert_eq!(bounds.min, [-5.0, 0.0, -1.0]);
        assert_eq!(bounds.max, [10.0, 3.0, 7.0]);
        assert_eq!(bounds.point_count, 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header: Default::default(), entities: vec![point_entity(1, 1.0, 1.0, 1.0)] };
        assert_eq!(compute_step_bounds(&snapshot), compute_step_bounds(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_step_bounds(&StepSnapshot::default()), StepBounds::default());
    }
}
//#endregion 🧪️Tests
