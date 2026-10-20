//! 🧮️ The logical geometry projection: layers, entities and geometry records and their native drawing conversions.

//#region 🔖️DrawingModel
use std::sync::Arc;
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum DwgLogicalGeometryKind {
    #[default]
    Point,
    Line,
    Circle,
    Arc,
    Ellipse,
    LwPolyline,
    Spline,
    Text,
    Face3d,
    Polyline3d,
    PolyfaceMesh,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalGeometry {
    pub kind: DwgLogicalGeometryKind,
    #[value(default)]
    pub values: Vec<f64>,
    #[value(default)]
    pub indices: Vec<i32>,
    #[value(default)]
    pub text: String,
    #[value(default)]
    pub closed: bool,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalLayer {
    pub name: String,
    pub color: u8,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalEntity {
    pub layer: usize,
    pub color: i16,
    pub geometry: DwgLogicalGeometry,
}
impl DwgLogicalEntity {
    pub(crate) fn to_native(&self) -> Result<semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgEntity, String> {
        let color = match self.color {
            -1 => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::ByLayer,
            -2 => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::ByBlock,
            value if (0..=255).contains(&value) => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::Index(value as u8),
            value => return Err(format!("invalid logical DWG color {value}")),
        };
        Ok(semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgEntity { layer: self.layer, color, geometry: self.geometry.to_native()? })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec2(values: &[f64]) -> Result<[f64; 2], String> {
    values.try_into().map_err(|_| format!("expected 2 values, got {}", values.len()))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn vec3(values: &[f64]) -> Result<[f64; 3], String> {
    values.try_into().map_err(|_| format!("expected 3 values, got {}", values.len()))
}
impl DwgLogicalGeometry {
    pub(crate) fn from_native(geometry: &semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry) -> Self {
        use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::*;
        match geometry {
            Point { at } => Self { kind: DwgLogicalGeometryKind::Point, values: at.to_vec(), ..Default::default() },
            Line { start, end } => Self { kind: DwgLogicalGeometryKind::Line, values: start.iter().chain(end).copied().collect(), ..Default::default() },
            Circle { center, radius, normal } => Self { kind: DwgLogicalGeometryKind::Circle, values: center.iter().chain([radius]).chain(normal).copied().collect(), ..Default::default() },
            Arc { center, radius, start_angle, end_angle, normal } => Self { kind: DwgLogicalGeometryKind::Arc, values: center.iter().chain([radius, start_angle, end_angle]).chain(normal).copied().collect(), ..Default::default() },
            Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
                Self { kind: DwgLogicalGeometryKind::Ellipse, values: center.iter().chain(major_axis).chain([ratio, start_param, end_param]).chain(normal).copied().collect(), ..Default::default() }
            }
            LwPolyline { closed, elevation, vertices, bulges } => Self {
                kind: DwgLogicalGeometryKind::LwPolyline,
                values: std::iter::once(*elevation).chain(vertices.iter().flatten().copied()).chain(bulges.iter().copied()).collect(),
                indices: vec![vertices.len() as i32],
                closed: *closed,
                ..Default::default()
            },
            Spline { degree, control_points, knots, weights } => Self {
                kind: DwgLogicalGeometryKind::Spline,
                values: control_points.iter().flatten().copied().chain(knots.iter().copied()).chain(weights.iter().copied()).collect(),
                indices: vec![*degree as i32, control_points.len() as i32, knots.len() as i32],
                ..Default::default()
            },
            Text { at, height, rotation, content } => Self { kind: DwgLogicalGeometryKind::Text, values: at.iter().copied().chain([*height, *rotation]).collect(), text: content.clone(), ..Default::default() },
            Face3d { corners } => Self { kind: DwgLogicalGeometryKind::Face3d, values: corners.iter().flatten().copied().collect(), ..Default::default() },
            Polyline3d { closed, vertices } => Self { kind: DwgLogicalGeometryKind::Polyline3d, values: vertices.iter().flatten().copied().collect(), closed: *closed, ..Default::default() },
            PolyfaceMesh { vertices, faces } => {
                Self { kind: DwgLogicalGeometryKind::PolyfaceMesh, values: vertices.iter().flatten().copied().collect(), indices: std::iter::once(vertices.len() as i32).chain(faces.iter().flatten().copied()).collect(), ..Default::default() }
            }
        }
    }

    fn to_native(&self) -> Result<semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry, String> {
        use DwgLogicalGeometryKind::*;
        Ok(match self.kind {
            Point => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Point { at: vec3(&self.values)? },
            Line => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Line { start: vec3(&self.values[0..3])?, end: vec3(&self.values[3..6])? },
            Circle => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Circle { center: vec3(&self.values[0..3])?, radius: self.values[3], normal: vec3(&self.values[4..7])? },
            Arc => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Arc { center: vec3(&self.values[0..3])?, radius: self.values[3], start_angle: self.values[4], end_angle: self.values[5], normal: vec3(&self.values[6..9])? },
            Ellipse => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Ellipse { center: vec3(&self.values[0..3])?, major_axis: vec3(&self.values[3..6])?, ratio: self.values[6], start_param: self.values[7], end_param: self.values[8], normal: vec3(&self.values[9..12])? },
            LwPolyline => {
                let count = *self.indices.first().ok_or("polyline vertex count missing")? as usize;
                let vertices = self.values[1..1 + count * 2].as_chunks::<2>().0.iter().map(|chunk| vec2(chunk)).collect::<Result<_, _>>()?;
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::LwPolyline { closed: self.closed, elevation: self.values[0], vertices, bulges: self.values[1 + count * 2..].to_vec() }
            }
            Spline => {
                let degree = self.indices[0] as u32;
                let point_count = self.indices[1] as usize;
                let knot_count = self.indices[2] as usize;
                let point_end = point_count * 3;
                let control_points = self.values[..point_end].as_chunks::<3>().0.iter().map(|chunk| vec3(chunk)).collect::<Result<_, _>>()?;
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Spline { degree, control_points, knots: self.values[point_end..point_end + knot_count].to_vec(), weights: self.values[point_end + knot_count..].to_vec() }
            }
            Text => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Text { at: vec3(&self.values[0..3])?, height: self.values[3], rotation: self.values[4], content: self.text.clone() },
            Face3d => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Face3d { corners: [vec3(&self.values[0..3])?, vec3(&self.values[3..6])?, vec3(&self.values[6..9])?, vec3(&self.values[9..12])?] },
            Polyline3d => semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Polyline3d { closed: self.closed, vertices: self.values.as_chunks::<3>().0.iter().map(|chunk| vec3(chunk)).collect::<Result<_, _>>()? },
            PolyfaceMesh => {
                let vertex_count = self.indices[0] as usize;
                let vertices = self.values.as_chunks::<3>().0.iter().take(vertex_count).map(|chunk| vec3(chunk)).collect::<Result<_, _>>()?;
                let faces = self.indices[1..].as_chunks::<4>().0.to_vec();
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::PolyfaceMesh { vertices, faces }
            }
        })
    }
}
//#endregion 🔖️DrawingModel
