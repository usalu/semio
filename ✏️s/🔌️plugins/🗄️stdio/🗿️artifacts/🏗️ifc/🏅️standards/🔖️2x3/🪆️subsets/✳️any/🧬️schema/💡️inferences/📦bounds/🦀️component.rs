//! 📦 `bounds` — the spatial min/max bounding box over every real `IFCCARTESIANPOINT((x,y,z));`
//! instance in `document.instances` (buildingSMART IFC2X3 / ISO-PAS 16739:2005, ISO 10303-21
//! Part-21 syntax). `document` still wraps the shared generic `Part21Document` graph directly
//! (this standard's real, current, on-disk shape — see the sibling `📸️snapshot/🦀️component.rs`
//! doc comment), so this fold reads it through `Part21Instance::entity`/`Part21Value::as_list`/
//! `Part21Value::as_real` rather than a typed accessor. A point's missing 3rd (z) component
//! defaults to `0.0` — the same convention IFC4's own `engine::spatial::cartesian_point` uses for
//! honestly-2D placements. A pure whole-snapshot scalar (one min/max fold) — no `InferredField`
//! needed.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::Part21Value;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ IFC2X3's `IFCCARTESIANPOINT`-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub fn compute_ifc2x3_bounds(snapshot: &Ifc2x3Snapshot) -> Ifc2x3Bounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut point_count = 0u32;

    for instance in &snapshot.document.instances {
        let Some(args) = instance.entity("IFCCARTESIANPOINT") else { continue };
        let Some(coords) = args.first().and_then(Part21Value::as_list) else { continue };
        let p = [
            coords.first().and_then(Part21Value::as_real).unwrap_or(0.0),
            coords.get(1).and_then(Part21Value::as_real).unwrap_or(0.0),
            coords.get(2).and_then(Part21Value::as_real).unwrap_or(0.0),
        ];
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
mod tests {
    use super::*;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance};

    fn point_instance(id: u64, x: f64, y: f64, z: f64) -> Part21Instance {
        Part21Instance {
            id,
            entities: vec![("IFCCARTESIANPOINT".into(), vec![Part21Value::List(vec![Part21Value::Real(x.into()), Part21Value::Real(y.into()), Part21Value::Real(z.into())])])],
        }
    }

    #[test]
    fn bounds_matches_hand_built_point_extent() {
        let snapshot = Ifc2x3Snapshot {
            schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(),
            document: Part21Document {
                header: Part21Header::default(),
                instances: vec![
                    point_instance(1, 0.0, 0.0, 0.0),
                    point_instance(2, -2.0, 5.0, 10.0),
                    point_instance(3, 8.0, 1.0, -4.0),
                    Part21Instance { id: 4, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset])] },
                ],
            },
            edm_preamble: None,
        };
        let bounds = compute_ifc2x3_bounds(&snapshot);
        assert_eq!(bounds.min, [-2.0, 0.0, -4.0]);
        assert_eq!(bounds.max, [8.0, 5.0, 10.0]);
        assert_eq!(bounds.point_count, 3);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = Ifc2x3Snapshot {
            schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(),
            document: Part21Document { header: Part21Header::default(), instances: vec![point_instance(1, 1.0, 1.0, 1.0)] },
            edm_preamble: None,
        };
        assert_eq!(compute_ifc2x3_bounds(&snapshot), compute_ifc2x3_bounds(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_ifc2x3_bounds(&Ifc2x3Snapshot::default()), Ifc2x3Bounds::default());
    }
}
//#endregion 🧪️Tests
