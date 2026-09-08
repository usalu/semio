//! 📤️ `s.stdio.semio/v1/drawing` → `dwg` (ac1024) — mirrors the import leaf and the dxf↔drawing
//! bridge's own structure exactly. Walks each `DrawLayer`'s `DrawNode` tree into one real DWG
//! layer (`DwgDrawing::ensure_layer(&layer.name)`): `Path` segments become `LwPolyline`/`Spline`
//! entities via the relocated (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
//! ARTIFACTS G2) hand-rolled DWG codec's own `paths_to_dwg_drawing`, `Text` nodes become
//! `DwgGeometry::Text` entities directly (this codec's own type — the sibling import leaf's
//! `dwg_geometry_to_path_segments` deliberately has no `Text` arm, since text isn't a path).
//! Populates the shared logical DWG snapshot directly; native bytes are materialized only by the
//! DWG serializer.
//!
//! Honest lossy points (documented, never fabricated): `Image` nodes and per-node `style` have no
//! DWG entity/attribute equivalent and are dropped (matches the dxf↔drawing bridge's own
//! block/insert-less architectural boundary); `Group` transforms are NOT applied to child
//! geometry (flattened by walk order only, not by matrix) — same simplification the dxf↔drawing
//! bridge's own entity walk makes.

use semio_s_artifact_stdio_dwg::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg::{paths_to_dwg_drawing, DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgPathSegment, DwgSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment, SemioDrawingSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };

/// 📐 The semio-authored codec's own file magic — see the codec's own module doc for why this is
/// NOT `"AC1024"` despite living under this standard tier.
const DWG_CODEC_VERSION: &str = "AC1015";

//#region 🔖️SegmentMap
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn path_segment_to_dwg(segment: &PathSegment) -> DwgPathSegment {
    match *segment {
        PathSegment::MoveTo { to } => DwgPathSegment::Move { to: [to.x, to.y] },
        PathSegment::LineTo { to } => DwgPathSegment::Line { to: [to.x, to.y] },
        PathSegment::QuadTo { c, to } => DwgPathSegment::Quad { ctrl: [c.x, c.y], to: [to.x, to.y] },
        PathSegment::CubicTo { c1, c2, to } => DwgPathSegment::Cubic { ctrl1: [c1.x, c1.y], ctrl2: [c2.x, c2.y], to: [to.x, to.y] },
        PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => DwgPathSegment::Arc { rx, ry, rotation: x_rotation, large_arc, sweep, to: [to.x, to.y] },
        PathSegment::Close => DwgPathSegment::Close,
    }
}
//#endregion 🔖️SegmentMap

//#region 🔖️Walk
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_node(node: &DrawNode, paths: &mut Vec<Vec<DwgPathSegment>>, texts: &mut Vec<(SemioPoint2, String)>) {
    match node {
        DrawNode::Group { children, .. } => {
            for child in children {
                collect_node(child, paths, texts);
            }
        }
        DrawNode::Path { segments, .. } => {
            if !segments.is_empty() {
                paths.push(segments.iter().map(path_segment_to_dwg).collect());
            }
        }
        DrawNode::Text { value, at, .. } => texts.push((*at, value.clone())),
        DrawNode::Image { .. } => {}
    }
}
//#endregion 🔖️Walk

//#region 🔖️Serializer
pub struct SemioDrawingToDwg;

impl ArtifactSerializer for SemioDrawingToDwg {
    type From = SemioDrawingSnapshot;
    type Into = DwgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut drawing = DwgDrawing::default();
        for layer in &from.layers {
            let layer_index = drawing.ensure_layer(&layer.name);
            let mut paths = Vec::new();
            let mut texts = Vec::new();
            collect_node(&layer.root, &mut paths, &mut texts);

            let sub = paths_to_dwg_drawing(&paths);
            for mut entity in sub.entities {
                entity.layer = layer_index;
                drawing.entities.push(entity);
            }
            for (at, content) in texts {
                drawing.entities.push(DwgEntity { layer: layer_index, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [at.x, at.y, 0.0], height: 1.0, rotation: 0.0, content } });
            }
        }

        let snapshot = DwgSnapshot { version: DWG_CODEC_VERSION.into(), drawing: DwgLogicalDrawing::from_native(&drawing).map_err(store::PackError::Schema)?, ..DwgSnapshot::default() };
        Ok(snapshot)
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
