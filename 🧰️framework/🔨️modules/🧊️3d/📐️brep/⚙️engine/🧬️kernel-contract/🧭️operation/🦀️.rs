//! 🧭️ Parameters and stable names for resumable B-Rep operations.

use super::{GeometryHandle,Vec3};

/// 🧭️ One expensive kernel operation, named by its input handles and parameters. The language-agnostic
/// fixture `🧫️fixtures/⏱️operation-jobs/🔣️.json` names the same operations and parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum BrepOperation {
    Fillet { shape: GeometryHandle, radius: f64 },
    FilletVariable { shape: GeometryHandle, radius_start: f64, radius_end: f64 },
    FilletEdges { shape: GeometryHandle, edges: Vec<GeometryHandle>, radius: f64 },
    Chamfer { shape: GeometryHandle, distance: f64 },
    ChamferAsymmetric { shape: GeometryHandle, first: f64, second: f64 },
    ChamferEdges { shape: GeometryHandle, edges: Vec<GeometryHandle>, distance: f64 },
    Shell { shape: GeometryHandle, thickness: f64, open_faces: Vec<GeometryHandle> },
    Draft { shape: GeometryHandle, faces: Vec<GeometryHandle>, pull_direction: Vec3, neutral_point: Vec3, angle: f64 },
    OffsetSolid { shape: GeometryHandle, distance: f64 },
    OffsetFace { face: GeometryHandle, distance: f64 },
    ThickenFace { face: GeometryHandle, thickness: f64 },
    Defeature { shape: GeometryHandle, faces: Vec<GeometryHandle> },
    Extrude { face: GeometryHandle, direction: Vec3, distance: f64 },
    ExtrudeWire { wire: GeometryHandle, vector: Vec3 },
    Revolve { face: GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64 },
    Sweep { profile: GeometryHandle, path: GeometryHandle },
    Pipe { profile: GeometryHandle, path: GeometryHandle, guide: Option<GeometryHandle> },
    HelicalSweep { profile: GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, radius: f64, pitch: f64, turns: f64 },
    Loft { profiles: Vec<GeometryHandle>, smooth: bool },
    Section { solid: GeometryHandle, plane_origin: Vec3, plane_normal: Vec3 },
    Split { solid: GeometryHandle, plane_origin: Vec3, plane_normal: Vec3 },
    CompoundCut { target: GeometryHandle, tools: Vec<GeometryHandle> },
    LinearPattern { shape: GeometryHandle, direction: Vec3, spacing: f64, count: usize },
    CircularPattern { shape: GeometryHandle, axis: Vec3, count: usize },
    GridPattern { shape: GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize },
    ConvexHull { points: Vec<Vec3> },
}

impl BrepOperation {
    /// 🏷️ The stable camelCase name every host and fixture calls this operation by.
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Fillet { .. } => "fillet",
            Self::FilletVariable { .. } => "filletVariable",
            Self::FilletEdges { .. } => "filletEdges",
            Self::Chamfer { .. } => "chamfer",
            Self::ChamferAsymmetric { .. } => "chamferAsymmetric",
            Self::ChamferEdges { .. } => "chamferEdges",
            Self::Shell { .. } => "shell",
            Self::Draft { .. } => "draft",
            Self::OffsetSolid { .. } => "offsetSolid",
            Self::OffsetFace { .. } => "offsetFace",
            Self::ThickenFace { .. } => "thickenFace",
            Self::Defeature { .. } => "defeature",
            Self::Extrude { .. } => "extrude",
            Self::ExtrudeWire { .. } => "extrudeWire",
            Self::Revolve { .. } => "revolve",
            Self::Sweep { .. } => "sweep",
            Self::Pipe { .. } => "pipe",
            Self::HelicalSweep { .. } => "helicalSweep",
            Self::Loft { .. } => "loft",
            Self::Section { .. } => "section",
            Self::Split { .. } => "split",
            Self::CompoundCut { .. } => "compoundCut",
            Self::LinearPattern { .. } => "linearPattern",
            Self::CircularPattern { .. } => "circularPattern",
            Self::GridPattern { .. } => "gridPattern",
            Self::ConvexHull { .. } => "convexHull",
        }
    }
}
