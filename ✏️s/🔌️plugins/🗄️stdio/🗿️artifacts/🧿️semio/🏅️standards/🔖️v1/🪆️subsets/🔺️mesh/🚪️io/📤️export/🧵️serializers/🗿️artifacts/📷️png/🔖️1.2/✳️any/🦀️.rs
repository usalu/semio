//! 📤️ `s.stdio.semio/v1/mesh` → `png` (1.2) — a shaded isometric view of the mesh.
//!
//! The view is an orthographic projection along `(-1, -1, -1)` with `+z` as screen-up (the CAD/GIS
//! convention), fitted into a [`MESH_VIEW_EDGE`]-pixel square with a margin, on a white ground.
//! Triangles (`Triangles`, `TriangleStrip`, `TriangleFan`, indexed or not) are painted far to near
//! (painter's algorithm) with two-sided Lambert shading of their colour: the mean of their vertex
//! colours when every vertex has one, else their material's base colour, else a neutral grey.
//! `Lines`/`LineStrip` primitives are drawn as one-pixel dark strokes. The projected scene is a real
//! `s.stdio.semio/v1/drawing` ([`mesh_view_drawing`]), painted by that subset's own rasterizer.
//!
//! 🔖 Documented lossiness (`IoFidelity::Lossy`, a picture of the mesh, not the mesh): one fixed
//! view; `Points` primitives, textures, normals and UVs are not drawn; intersecting triangles are
//! ordered by centroid depth, not split.

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::SemioDrawingToPng;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioPrimitive, SemioTopology};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_png::PngSnapshot;
use std::collections::BTreeMap;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 📏️ Edge of the square view in pixels.
pub const MESH_VIEW_EDGE: f64 = 512.0;
const MESH_VIEW_MARGIN: f64 = 16.0;
const NEUTRAL_GREY: SemioRgba = SemioRgba { r: 0.72, g: 0.74, b: 0.78, a: 1.0 };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalized(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    if length > 0.0 { [a[0] / length, a[1] / length, a[2] / length] } else { a }
}

/// 🔺️ The corner index triples a primitive's topology denotes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn primitive_triangles(primitive: &SemioPrimitive) -> Vec<[usize; 3]> {
    let order: Vec<usize> = if primitive.indices.is_empty() { (0..primitive.positions.len()).collect() } else { primitive.indices.iter().map(|i| *i as usize).collect() };
    let triangles: Vec<[usize; 3]> = match primitive.topology {
        SemioTopology::Triangles => order.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect(),
        SemioTopology::TriangleStrip => order.windows(3).enumerate().map(|(i, t)| if i % 2 == 0 { [t[0], t[1], t[2]] } else { [t[1], t[0], t[2]] }).collect(),
        SemioTopology::TriangleFan => order.windows(2).skip(1).map(|t| [order[0], t[0], t[1]]).collect(),
        SemioTopology::Points | SemioTopology::Lines | SemioTopology::LineStrip => Vec::new(),
    };
    triangles.into_iter().filter(|t| t.iter().all(|i| *i < primitive.positions.len())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn primitive_lines(primitive: &SemioPrimitive) -> Vec<[usize; 2]> {
    let order: Vec<usize> = if primitive.indices.is_empty() { (0..primitive.positions.len()).collect() } else { primitive.indices.iter().map(|i| *i as usize).collect() };
    let lines: Vec<[usize; 2]> = match primitive.topology {
        SemioTopology::Lines => order.chunks_exact(2).map(|l| [l[0], l[1]]).collect(),
        SemioTopology::LineStrip => order.windows(2).map(|l| [l[0], l[1]]).collect(),
        _ => Vec::new(),
    };
    lines.into_iter().filter(|l| l.iter().all(|i| *i < primitive.positions.len())).collect()
}

/// 🖼️ The mesh's isometric view as a drawing in pixel units (see the module doc).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_view_drawing(mesh: &SemioMeshSnapshot) -> Result<SemioDrawingSnapshot, String> {
    let forward = normalized([-1.0, -1.0, -1.0]);
    let right = normalized(cross(forward, [0.0, 0.0, 1.0]));
    let up = cross(right, forward);
    let light = normalized([-forward[0] * 0.6 + up[0] * 0.6 - right[0] * 0.3, -forward[1] * 0.6 + up[1] * 0.6 - right[1] * 0.3, -forward[2] * 0.6 + up[2] * 0.6 - right[2] * 0.3]);
    let point = |p: &SemioPoint3| [p.x, p.y, p.z];
    let mut faces: Vec<(f64, [[f64; 3]; 3], SemioRgba)> = Vec::new();
    let mut edges: Vec<[[f64; 3]; 2]> = Vec::new();
    for primitive in mesh.meshes.iter().flat_map(|m| &m.primitives) {
        let material = primitive.material_id.as_deref().and_then(|id| mesh.materials.iter().find(|m| m.id == id)).map(|m| m.base_color);
        let vertex_colours = primitive.colors.len() == primitive.positions.len();
        for [a, b, c] in primitive_triangles(primitive) {
            let corners = [point(&primitive.positions[a]), point(&primitive.positions[b]), point(&primitive.positions[c])];
            let colour = if vertex_colours {
                let (ca, cb, cc) = (primitive.colors[a], primitive.colors[b], primitive.colors[c]);
                SemioRgba { r: (ca.r + cb.r + cc.r) / 3.0, g: (ca.g + cb.g + cc.g) / 3.0, b: (ca.b + cb.b + cc.b) / 3.0, a: (ca.a + cb.a + cc.a) / 3.0 }
            } else {
                material.unwrap_or(NEUTRAL_GREY)
            };
            let depth = corners.iter().map(|c| dot(*c, forward)).sum::<f64>() / 3.0;
            faces.push((depth, corners, colour));
        }
        for [a, b] in primitive_lines(primitive) {
            edges.push([point(&primitive.positions[a]), point(&primitive.positions[b])]);
        }
    }
    let projected = |p: [f64; 3]| [dot(p, right), -dot(p, up)];
    let all: Vec<[f64; 2]> = faces.iter().flat_map(|(_, corners, _)| corners.iter().map(|c| projected(*c))).chain(edges.iter().flat_map(|e| e.iter().map(|c| projected(*c)))).collect();
    if all.is_empty() {
        return Err("semio/mesh→png: the mesh has no triangle or line to draw".into());
    }
    let (min_x, max_x) = all.iter().fold((f64::MAX, f64::MIN), |(lo, hi), p| (lo.min(p[0]), hi.max(p[0])));
    let (min_y, max_y) = all.iter().fold((f64::MAX, f64::MIN), |(lo, hi), p| (lo.min(p[1]), hi.max(p[1])));
    let span = (max_x - min_x).max(max_y - min_y);
    let scale = if span > 0.0 { (MESH_VIEW_EDGE - 2.0 * MESH_VIEW_MARGIN) / span } else { 1.0 };
    let offset = [(MESH_VIEW_EDGE - (max_x - min_x) * scale) / 2.0, (MESH_VIEW_EDGE - (max_y - min_y) * scale) / 2.0];
    let screen = |p: [f64; 3]| {
        let q = projected(p);
        SemioPoint2 { x: (q[0] - min_x) * scale + offset[0], y: (q[1] - min_y) * scale + offset[1] }
    };
    faces.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut styles: BTreeMap<String, DrawStyle> = BTreeMap::new();
    let mut children = Vec::with_capacity(faces.len() + edges.len());
    for (_, corners, colour) in &faces {
        let normal = normalized(cross([corners[1][0] - corners[0][0], corners[1][1] - corners[0][1], corners[1][2] - corners[0][2]], [corners[2][0] - corners[0][0], corners[2][1] - corners[0][1], corners[2][2] - corners[0][2]]));
        let shade = (0.35 + 0.65 * dot(normal, light).abs()) as f32;
        let quantize = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() / 255.0;
        let shaded = SemioRgba { r: quantize(colour.r * shade), g: quantize(colour.g * shade), b: quantize(colour.b * shade), a: quantize(colour.a) };
        let name = format!("{:02x}{:02x}{:02x}{:02x}", (shaded.r * 255.0) as u8, (shaded.g * 255.0) as u8, (shaded.b * 255.0) as u8, (shaded.a * 255.0) as u8);
        styles.entry(name.clone()).or_insert_with(|| DrawStyle { name: name.clone(), fill: Some(shaded), stroke: Some(shaded), stroke_width: Some(0.5), opacity: None });
        children.push(DrawNode::Path { segments: vec![PathSegment::MoveTo { to: screen(corners[0]) }, PathSegment::LineTo { to: screen(corners[1]) }, PathSegment::LineTo { to: screen(corners[2]) }, PathSegment::Close], style: Some(name) });
    }
    if !edges.is_empty() {
        let dark = SemioRgba { r: 0.15, g: 0.15, b: 0.18, a: 1.0 };
        styles.insert("mesh-line".into(), DrawStyle { name: "mesh-line".into(), fill: None, stroke: Some(dark), stroke_width: Some(1.0), opacity: None });
        children.extend(edges.iter().map(|[a, b]| DrawNode::Path { segments: vec![PathSegment::MoveTo { to: screen(*a) }, PathSegment::LineTo { to: screen(*b) }], style: Some("mesh-line".into()) }));
    }
    Ok(SemioDrawingSnapshot {
        canvas: DrawCanvas { width: MESH_VIEW_EDGE, height: MESH_VIEW_EDGE, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: styles.into_values().collect(),
        layers: vec![DrawLayer { id: "view".into(), name: "view".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    })
}

//#region 🔖️Serializer
pub struct SemioMeshToPng;

impl ArtifactSerializer for SemioMeshToPng {
    type From = SemioMeshSnapshot;
    type Into = PngSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        SemioDrawingToPng::serialize(&mesh_view_drawing(from).map_err(store::PackError::Schema)?).await
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
