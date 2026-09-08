//! 📥️ `dwg` (ac1024) → `s.stdio.semio/v1/drawing` — reads the shared logical `DwgSnapshot`
//! drawing model and maps it with `dwg_geometry_to_path_segments`. One `DrawLayer` per DWG layer, one
//! `DrawNode` per entity, in entity order — the exact inverse of the sibling export leaf's own
//! per-layer `paths_to_dwg_drawing` + `DwgGeometry::Text` push order.
//!
//! Honest lossy points (documented, never fabricated):
//! - `z` is dropped (drawing is 2D-only, matching the cad↔dxf/dxf↔drawing bridges' own
//!   dimensionality note).
//! - Every `DwgGeometry` kind `dwg_geometry_to_path_segments` doesn't cover (`Line`/`Point`/`Arc`/
//!   `Ellipse`/`Polyline3d`/`PolyfaceMesh`/`Face3d`) has no `DrawNode` equivalent here and is
//!   dropped — mesh-shaped content is the `🔺️mesh` bridge's job, not this one's.
//! - Malformed logical geometry is a hard `Err`, not a fabricated empty drawing.

use semio_s_artifact_stdio_dwg::{dwg_geometry_to_path_segments, DwgDrawing, DwgGeometry, DwgSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

//#region 🔖️SegmentMap
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_segment_to_path(segment: &semio_s_artifact_stdio_dwg::DwgPathSegment) -> PathSegment {
    use semio_s_artifact_stdio_dwg::DwgPathSegment;
    match *segment {
        DwgPathSegment::Move { to } => PathSegment::MoveTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        DwgPathSegment::Line { to } => PathSegment::LineTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        DwgPathSegment::Quad { ctrl, to } => PathSegment::QuadTo { c: SemioPoint2 { x: ctrl[0], y: ctrl[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        DwgPathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::CubicTo { c1: SemioPoint2 { x: ctrl1[0], y: ctrl1[1] }, c2: SemioPoint2 { x: ctrl2[0], y: ctrl2[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        DwgPathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::ArcTo { rx, ry, x_rotation: rotation, large_arc, sweep, to: SemioPoint2 { x: to[0], y: to[1] } },
        DwgPathSegment::Close => PathSegment::Close,
    }
}
//#endregion 🔖️SegmentMap

//#region 🔖️EntityMap
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn draw_node_from_entity(geometry: &DwgGeometry) -> Option<DrawNode> {
    if let DwgGeometry::Text { at, content, .. } = geometry {
        return Some(DrawNode::Text { value: content.clone(), at: SemioPoint2 { x: at[0], y: at[1] }, style: None });
    }
    dwg_geometry_to_path_segments(geometry).map(|segments| DrawNode::Path { segments: segments.iter().map(dwg_segment_to_path).collect(), style: None })
}
//#endregion 🔖️EntityMap

//#region 🔖️Deserializer
pub struct SemioDrawingFromDwg;

impl ArtifactDeserializer for SemioDrawingFromDwg {
    type From = DwgSnapshot;
    type Into = SemioDrawingSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let drawing: DwgDrawing = from.drawing.to_native().map_err(store::PackError::Schema)?;
        let layers = drawing
            .layers
            .iter()
            .enumerate()
            .filter_map(|(layer_index, layer)| {
                let children: Vec<DrawNode> = drawing.entities.iter().filter(|e| e.layer == layer_index).filter_map(|e| draw_node_from_entity(&e.geometry)).collect();
                if children.is_empty() {
                    return None;
                }
                Some(DrawLayer { id: layer.name.clone(), name: layer.name.clone(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } })
            })
            .collect();
        Ok(SemioDrawingSnapshot { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas: DrawCanvas::default(), styles: Vec::new(), layers })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
