//! 📤️ `s.stdio.semio/v1/cad` → `dwg` (ac1024) — every layer becomes a DWG layer and every model-space
//! entity its DWG counterpart in the logical drawing model (`DwgLogicalDrawing`), which the DWG
//! codec writes as real AC1015 bytes: line, circle, arc and text keep their geometry (DXF-style
//! degree angles become DWG radians), an ellipse keeps its relative major axis, a polyline becomes an
//! `LWPOLYLINE`, a solid a `3DFACE` and a dimension its text at the text position.
//!
//! 🔖 Documented lossiness: block definitions and `INSERT`s have no entity in the logical model and
//! are dropped; layer colours outside 0–255 clamp; line types are not written.

use crate::standards::v1::subsets::cad::schema::snapshot::{CadEntity, CadEntityRecord, SemioCadSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_dwg::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgLayer, DwgSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const NORMAL: [f64; 3] = [0.0, 0.0, 1.0];

//#region 🔖️EntityMap
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_geometry_from_cad(entity: &CadEntity) -> Option<DwgGeometry> {
    Some(match entity {
        CadEntity::Line { a, b } => DwgGeometry::Line { start: [a.x, a.y, 0.0], end: [b.x, b.y, 0.0] },
        CadEntity::Circle { center, radius } => DwgGeometry::Circle { center: [center.x, center.y, 0.0], radius: *radius, normal: NORMAL },
        CadEntity::Arc { center, radius, start_angle, end_angle } => DwgGeometry::Arc { center: [center.x, center.y, 0.0], radius: *radius, start_angle: start_angle.to_radians(), end_angle: end_angle.to_radians(), normal: NORMAL },
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => DwgGeometry::Ellipse { center: [center.x, center.y, 0.0], major_axis: [major_axis_end.x, major_axis_end.y, 0.0], ratio: *ratio, start_param: *start_param, end_param: *end_param, normal: NORMAL },
        CadEntity::Polyline { vertices, closed } => DwgGeometry::LwPolyline { closed: *closed, elevation: 0.0, vertices: vertices.iter().map(|v| [v.x, v.y]).collect(), bulges: vec![0.0; vertices.len()] },
        CadEntity::Text { position, height, rotation, content } => DwgGeometry::Text { at: [position.x, position.y, 0.0], height: *height, rotation: rotation.to_radians(), content: content.clone() },
        CadEntity::Solid { p1, p2, p3, p4 } => DwgGeometry::Face3d { corners: [[p1.x, p1.y, 0.0], [p2.x, p2.y, 0.0], [p3.x, p3.y, 0.0], [p4.x, p4.y, 0.0]] },
        CadEntity::Dimension { text_position, measurement, text, .. } => DwgGeometry::Text { at: [text_position.x, text_position.y, 0.0], height: 1.0, rotation: 0.0, content: if text.is_empty() { measurement.to_string() } else { text.clone() } },
        CadEntity::Insert { .. } => return None,
    })
}

/// 🗺️ The CAD document as a logical DWG drawing (see the module doc).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn cad_to_dwg_drawing(from: &SemioCadSnapshot) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    drawing.layers = from.layers.iter().map(|layer| DwgLayer { name: layer.name.clone(), color: layer.color_index.clamp(0, 255) as u8 }).collect();
    let entity = |record: &CadEntityRecord, drawing: &mut DwgDrawing| dwg_geometry_from_cad(&record.entity).map(|geometry| DwgEntity { layer: drawing.ensure_layer(&record.layer), color: DwgColor::ByLayer, geometry });
    for record in &from.entities {
        if let Some(entity) = entity(record, &mut drawing) {
            drawing.entities.push(entity);
        }
    }
    drawing
}
//#endregion 🔖️EntityMap

//#region 🔖️Serializer
pub struct SemioCadToDwg;

impl ArtifactSerializer for SemioCadToDwg {
    type From = SemioCadSnapshot;
    type Into = DwgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(DwgSnapshot { version: "AC1015".into(), drawing: DwgLogicalDrawing::from_native(&cad_to_dwg_drawing(from)).map_err(store::PackError::Schema)?, ..DwgSnapshot::default() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
