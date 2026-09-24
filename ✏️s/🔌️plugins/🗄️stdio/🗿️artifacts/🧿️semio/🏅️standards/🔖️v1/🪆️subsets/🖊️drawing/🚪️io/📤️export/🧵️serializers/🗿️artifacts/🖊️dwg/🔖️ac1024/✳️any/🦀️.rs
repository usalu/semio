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
//! A path in the two-arc circle normal form under a similarity transform is written as a real
//! `Circle` entity (the import leaf reads it back to the same form). `Group` transforms are applied
//! to child geometry — points are mapped and arcs become cubics under a general affine map.
//!
//! Honest lossy points (documented, never fabricated): `Image` nodes and per-node `style` have no
//! DWG entity/attribute equivalent and are dropped (matches the dxf↔drawing bridge's own
//! block/insert-less architectural boundary).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment, SemioDrawingSnapshot};
use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{circle_normal_form, compose_affine, semio_transform_affine, similarity_scale, transformed_segments};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_dwg::{paths_to_dwg_drawing, DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgPathSegment, DwgSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };

/// 📐 The semio-authored codec's own file magic — see the codec's own module doc for why this is
/// NOT `"AC1024"` despite living under this standard tier.

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
fn collect_node(node: &DrawNode, matrix: &[f64; 6], paths: &mut Vec<Vec<DwgPathSegment>>, circles: &mut Vec<([f64; 2], f64)>, texts: &mut Vec<(SemioPoint2, String)>) {
    match node {
        DrawNode::Group { transform, children } => {
            let inner = compose_affine(matrix, &semio_transform_affine(transform));
            for child in children {
                collect_node(child, &inner, paths, circles, texts);
            }
        }
        DrawNode::Path { segments, .. } => {
            if let (Some((centre, radius)), Some(scale)) = (circle_normal_form(segments), similarity_scale(matrix)) {
                circles.push(([matrix[0] * centre[0] + matrix[2] * centre[1] + matrix[4], matrix[1] * centre[0] + matrix[3] * centre[1] + matrix[5]], radius * scale));
            } else if !segments.is_empty() {
                paths.push(transformed_segments(segments, matrix).iter().map(path_segment_to_dwg).collect());
            }
        }
        DrawNode::Text { value, at, .. } => texts.push((SemioPoint2 { x: matrix[0] * at.x + matrix[2] * at.y + matrix[4], y: matrix[1] * at.x + matrix[3] * at.y + matrix[5] }, value.clone())),
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
            let mut circles = Vec::new();
            let mut texts = Vec::new();
            collect_node(&layer.root, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &mut paths, &mut circles, &mut texts);

            let sub = paths_to_dwg_drawing(&paths);
            for mut entity in sub.entities {
                entity.layer = layer_index;
                drawing.entities.push(entity);
            }
            for (centre, radius) in circles {
                drawing.entities.push(DwgEntity { layer: layer_index, color: DwgColor::ByLayer, geometry: DwgGeometry::Circle { center: [centre[0], centre[1], 0.0], radius, normal: [0.0, 0.0, 1.0] } });
            }
            for (at, content) in texts {
                drawing.entities.push(DwgEntity { layer: layer_index, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [at.x, at.y, 0.0], height: 1.0, rotation: 0.0, content } });
            }
        }

        let snapshot = DwgSnapshot::from_drawing(&drawing).map_err(store::PackError::Schema)?;
        Ok(snapshot)
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
