//! 📦 `bounds` — the spatial min/max bounding box over every real `IFCCARTESIANPOINT((x,y,z));`
//! instance in `document.instances` (buildingSMART IFC2X3 / ISO-PAS 16739:2005, ISO 10303-21
//! Part-21 syntax). `document` still wraps the shared generic `Part21Document` graph directly
//! (this standard's real, current, on-disk shape — see the sibling `📸️snapshot/🦀️.rs`
//! doc comment), so this fold reads it through `Part21Instance::entity`/`Part21Value::as_list`/
//! `Part21Value::as_real` rather than a typed accessor. A point's missing 3rd (z) component
//! defaults to `0.0` — the same convention IFC4's own `engine::spatial::cartesian_point` uses for
//! honestly-2D placements. A pure whole-snapshot scalar (one min/max fold) — no `InferredField`
//! needed.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_step::engine::part21::Part21Value;

//#region 🔖️Bounds
/// 📦️ IFC2X3's `IFCCARTESIANPOINT`-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Ifc2x3Bounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub point_count: u32,
}

/// 🩹 Hand-rolled: an empty instance set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute_ifc2x3_bounds` returns for zero `IFCCARTESIANPOINT` instances (the fold's identity
/// value), keeping the inference-default law correct.
impl Default for Ifc2x3Bounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], point_count: 0 }
    }
}

/// 📦️ Computes [`Ifc2x3Bounds`] by folding every real `IFCCARTESIANPOINT` instance's coordinate
/// aggregate (`args.first()`, no leading label arg — unlike STEP AP214's `CARTESIAN_POINT`, IFC's
/// own EXPRESS schema declares `IfcCartesianPoint.Coordinates` as the sole attribute).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_ifc2x3_bounds(snapshot: &Ifc2x3Snapshot) -> Ifc2x3Bounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut point_count = 0u32;

    for instance in &snapshot.document.instances {
        let Some(args) = instance.entity("IFCCARTESIANPOINT") else { continue };
        let Some(coords) = args.first().and_then(Part21Value::as_list) else { continue };
        let p = [coords.first().and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(1).and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)];
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

    Ifc2x3Bounds { min, max, point_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
