//! 🧭️ Brep engine algorithm types: `Vec3`/`Aabb`/`ParamDomain`/`FaceGroup`/`MeshTransfer`/
//! `PointClassification` shared by framework-3d's own algorithm modules (`bvh`, `spatial`,
//! `offset`, `mesh_io`, `classify`, `tessellate`, `boolean`) and re-imported across the
//! `stdio → semio-framework-3d` forward edge by the relocated consumer contract
//! (`BrepKernel`/`GeometryHandle`/`Brep`/`SolidExporter`/…, now at
//! `semio_s_plugin_stdio::…::subsets::brep::schema::engine`, ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP).
//!
//! The consumer-contract types (`GeometryKind`, `GeometryHandle`, `ClosestPoint`, `BrepTopology`,
//! `BrepError`, the `BrepKernel` trait, `Brep` + its 93 `_sync` methods, `SolidExporter`/
//! `SolidImporter` + Step/Stl/Obj/Glb codec pairs) were deleted from here wave DEDUP — verified
//! byte-for-byte duplicated at the stdio site (93/93 `_sync` methods). Their `block_on`/
//! `BrepEngineHost`/`BrepDocumentOpEngine`/`BREP_ENGINE_ID` host-engine wiring (formerly
//! `🧮️compute`/`🖥️host` submodules here) was also deleted: zero external callers repo-wide
//! (wave G4 already deleted the sole `static HOST: OnceLock<BrepEngineHost>` call site; every
//! plugin now owns a fresh `Brep::new()` directly — see `cad_brep_kernel()` and
//! `ProcessKernelReplay`, both of which document the replacement).

use serde::{Deserialize, Serialize};

// #region 🔖️Types
/// 📐️ Column vector `[x,y,z]`.
pub type Vec3 = [f64; 3];

/// 📦️ Axis-aligned bounds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// 📏️ Parametric domain `[min, max]`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParamDomain {
    pub min: f64,
    pub max: f64,
}

/// 🧩️ Triangle index range for one B-Rep face.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaceGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🖼️ Tessellated mesh payload for preview upload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshTransfer {
    pub position: Vec<f32>,
    pub normal: Vec<f32>,
    pub index: Vec<u32>,
    pub edges: Vec<f32>,
    #[serde(default)]
    pub points: Vec<f32>,
    pub face_groups: Vec<FaceGroup>,
}

/// 📍️ Point classification relative to a solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointClassification {
    Inside,
    Outside,
    OnBoundary,
}
// #endregion 🔖️Types
