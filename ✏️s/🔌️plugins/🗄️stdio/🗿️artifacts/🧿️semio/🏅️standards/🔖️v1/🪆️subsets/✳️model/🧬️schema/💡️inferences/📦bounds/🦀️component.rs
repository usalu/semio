//! 📦 `bounds` — one named inference: the model's own POSITION envelope, folded over every
//! `SpatialNode.placement.translation` and every `SemioModelElement.placement.translation` this
//! subset owns outright. `GeometryRef` only resolves BY ID into the sibling `brep`/`mesh` subsets
//! (this facet's own module doc comment) — inlining THEIR geometry here would violate the
//! composition boundary those subsets' own snapshots enforce, so this is honestly a placement
//! envelope, not a geometry bounding box. A plain whole-snapshot fold — no `InferredField`/
//! incremental caching needed for a single min/max pass (same ruling `cad`'s own `📦bounds` facet
//! reaches for its own entity-point fold).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Semio model position envelope.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioQuaternion, SemioTransform};
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, SemioModelElement, SpatialKind, SpatialNode, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn placed(x: f64, y: f64, z: f64) -> SemioTransform {
        SemioTransform { translation: SemioPoint3 { x, y, z }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn populated() -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
            spatial: vec![
                SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: placed(-5.0, 0.0, 0.0) },
                SpatialNode { id: "storey-1".into(), kind: SpatialKind::Storey, name: "Ground".into(), parent_id: Some("site-1".into()), placement: placed(0.0, 0.0, 3.0) },
            ],
            elements: vec![SemioModelElement { id: "wall-1".into(), class: ElementClass::Wall, placement: placed(10.0, -2.0, 1.0), geometry: GeometryRef::None, spatial_id: Some("storey-1".into()), psets: Vec::new() }],
            relations: Vec::new(),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn folds_min_max_across_spatial_and_element_placements() {
        let bounds = compute_semio_model_bounds(&populated());
        assert_eq!(bounds.entity_count, 3);
        assert_eq!(bounds.min, SemioPoint3 { x: -5.0, y: -2.0, z: 0.0 });
        assert_eq!(bounds.max, SemioPoint3 { x: 10.0, y: 0.0, z: 3.0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = populated();
        assert_eq!(compute_semio_model_bounds(&snapshot), compute_semio_model_bounds(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_model_bounds(&SemioModelSnapshot::default()), SemioModelBounds::default());
    }
}
//#endregion 🧪️Tests
