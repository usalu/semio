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
impl DwgLogicalLayer {
    pub fn to_native(&self) -> semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer {
        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer { name: self.name.clone(), color: self.color }
    }
}
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

}
//#endregion 🔖️DrawingModel
