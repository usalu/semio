//! 📥️ `dwg` (ac1024) → `s.stdio.semio/v1/cad` — the DWG codec's logical drawing (`DwgSnapshot.drawing`,
//! decoded from real AC1015/AC1018 bytes) read entity by entity, the inverse of the sibling export:
//! line, circle, arc (radians back to degrees), ellipse, lightweight and 3D polylines (flattened to
//! the plan), text and 3D faces (as solids) become CAD entities on their DWG layer.
//!
//! 🔖 Documented lossiness: points, splines and polyface meshes have no CAD entity here and are
//! dropped; polyline bulges are not kept (their vertices are).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::cad::schema::snapshot::{CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_dwg::{DwgGeometry, DwgSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

//#region 🔖️EntityMap
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cad_entity_from_dwg(geometry: &DwgGeometry) -> Option<CadEntity> {
    let p = |v: &[f64]| SemioPoint2 { x: v[0], y: v[1] };
    Some(match geometry {
        DwgGeometry::Line { start, end } => CadEntity::Line { a: p(start), b: p(end) },
        DwgGeometry::Circle { center, radius, .. } => CadEntity::Circle { center: p(center), radius: *radius },
        DwgGeometry::Arc { center, radius, start_angle, end_angle, .. } => CadEntity::Arc { center: p(center), radius: *radius, start_angle: start_angle.to_degrees(), end_angle: end_angle.to_degrees() },
        DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, .. } => CadEntity::Ellipse { center: p(center), major_axis_end: p(major_axis), ratio: *ratio, start_param: *start_param, end_param: *end_param },
        DwgGeometry::LwPolyline { closed, vertices, .. } => CadEntity::Polyline { vertices: vertices.iter().map(|v| p(v)).collect(), closed: *closed },
        DwgGeometry::Polyline3d { closed, vertices } => CadEntity::Polyline { vertices: vertices.iter().map(|v| p(v)).collect(), closed: *closed },
        DwgGeometry::Text { at, height, rotation, content } => CadEntity::Text { position: p(at), height: *height, rotation: rotation.to_degrees(), content: content.clone() },
        DwgGeometry::Face3d { corners } => CadEntity::Solid { p1: p(&corners[0]), p2: p(&corners[1]), p3: p(&corners[2]), p4: p(&corners[3]) },
        DwgGeometry::Point { .. } | DwgGeometry::Spline { .. } | DwgGeometry::PolyfaceMesh { .. } => return None,
    })
}
//#endregion 🔖️EntityMap

//#region 🔖️Deserializer
pub struct SemioCadFromDwg;

impl ArtifactDeserializer for SemioCadFromDwg {
    type From = DwgSnapshot;
    type Into = SemioCadSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.version.is_empty() {
            return Err(store::PackError::Schema("dwg→semio/cad: missing AC10xx version sentinel — not a real DWG file".into()));
        }
        let drawing = from.drawing.to_native().map_err(store::PackError::Schema)?;
        let layers = drawing.layers.iter().map(|layer| CadLayer { name: layer.name.clone(), color_index: i32::from(layer.color), line_type: "CONTINUOUS".into(), visible: true }).collect();
        let entities = drawing
            .entities
            .iter()
            .enumerate()
            .filter_map(|(index, entity)| cad_entity_from_dwg(&entity.geometry).map(|cad| CadEntityRecord { handle: format!("{:X}", index + 1), layer: drawing.layers.get(entity.layer).map_or_else(|| "0".to_string(), |layer| layer.name.clone()), entity: cad }))
            .collect();
        Ok(SemioCadSnapshot { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers, blocks: Vec::new(), entities })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
