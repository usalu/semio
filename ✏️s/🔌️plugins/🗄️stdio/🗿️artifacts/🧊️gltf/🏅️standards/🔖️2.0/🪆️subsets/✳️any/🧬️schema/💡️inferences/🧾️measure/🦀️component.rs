//! 🧾️ Shared value, quality, policy, diagnostic, and entity contracts for GLTF inference.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfUnit {
    Unitless,
    Metre,
    SquareMetre,
    CubicMetre,
    Radian,
    InverseMetre,
    InverseSquareMetre,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfCoordinateSpace {
    MeshLocal,
    NodeLocal,
    SceneWorld,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfAvailability {
    Available,
    Approximate,
    Unavailable,
    InvalidInput,
    UnsupportedPrimitive,
    OpenSurface,
    NonManifold,
    Degenerate,
    UnresolvedResource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfValidity {
    Valid,
    Invalid,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfComputationMethod {
    Exact,
    DeterministicEstimate,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GltfVec3 {
    pub(crate) fn new(v: [f64; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }
    pub(crate) fn array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBounds3 {
    pub min: GltfVec3,
    pub max: GltfVec3,
    pub dimensions: GltfVec3,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfHistogram {
    pub edges: Vec<f64>,
    pub counts: Vec<u64>,
    pub weights: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_deviation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub median: Option<f64>,
    pub quantiles: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histogram: Option<GltfHistogram>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDirectionScore {
    pub direction: GltfVec3,
    pub score: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfPrincipalFrame {
    pub centroid: GltfVec3,
    pub axes: [GltfVec3; 3],
    pub eigenvalues: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfQuality {
    pub method: GltfComputationMethod,
    pub coverage: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absolute_error: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_error: Option<f64>,
    pub sample_count: u64,
    pub watertight: bool,
    pub manifold: bool,
    pub consistently_oriented: bool,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfProvenance {
    pub algorithm: String,
    pub algorithm_version: u32,
    pub dependency_fingerprints: Vec<String>,
    pub coordinate_space: GltfCoordinateSpace,
    pub tolerance_fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling_seed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pose: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMeasure<T> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
    pub unit: GltfUnit,
    pub availability: GltfAvailability,
    pub validity: GltfValidity,
    pub diagnostic_ids: Vec<String>,
    pub quality: GltfQuality,
    pub provenance: GltfProvenance,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAnalysisPolicy {
    pub schema_version: u32,
    pub absolute_length_tolerance: f64,
    pub relative_tolerance: f64,
    pub angular_tolerance_radians: f64,
    pub contact_tolerance: f64,
    pub sharp_feature_angle_radians: f64,
    pub histogram_edges: Vec<f64>,
    pub sampling_budget: u64,
    pub sampling_seed: String,
    pub static_pose: bool,
    pub unit_density: bool,
    pub fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDiagnostic {
    pub id: String,
    pub severity: GltfSeverity,
    pub code: String,
    pub message: String,
    pub paths: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfEntityScope {
    Document,
    Scene,
    NodeInstance,
    Mesh,
    Primitive,
    Component,
    SurfaceRegion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfEntityAddress {
    pub scope: GltfEntityScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<u32>,
    pub node_path: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primitive: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_region: Option<u32>,
    pub content_fingerprint: String,
}
