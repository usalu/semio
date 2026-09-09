//! ⚙️ FEM 3D module engine — headless compute (pure FE algorithm, moved out of the artifact tree: an
//! artifact is a schema + io system, never an engine). Errors and the top-level `build_model`/
//! `fem3d_solve`/`fem3d_solve_all` entry points live here. Solid meshing lives in `🕸️meshing`,
//! modal/buckling in `🎵️modal-buckling`, mesh preview + nodal stress in `🗺️mesh-preview`.
//! `empty_fem3d_snapshot` lives in the artifact's own `🧬️schema/🦀️component.rs` (pure document
//! helper); `fem3d_io`/its ports and the scene-render bridge (`fem3d_scene_parts`/`fem3d_camera_json`
//! and their helpers — app-facing, reference `crate::app_surface` and shared by the model/results
//! windows) live in `🎛️apps/🧊️3d/🦀️.rs`.

use crate::analyses;
use crate::fem3d_engine::meshing;
use crate::Fem3dSnapshot;
use std::collections::HashMap;

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem3dSnapshot`.
#[derive(Clone, Debug, PartialEq)]
pub enum Fem3dError {
    MaterialNotFound(String),
    SectionNotFound(String),
    NodeNotFound(String),
    UnknownSolidId(String),
    MeshFailed { solid_id: String, reason: String },
    LoadCaseNotFound(String),
    ModeIndexOutOfRange(usize),
    Fem(crate::model::FemError),
}

impl std::fmt::Display for Fem3dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaterialNotFound(id) => write!(formatter, "material not found: {id}"),
            Self::SectionNotFound(id) => write!(formatter, "section not found: {id}"),
            Self::NodeNotFound(id) => write!(formatter, "node not found: {id}"),
            Self::UnknownSolidId(id) => write!(formatter, "unknown solid id: {id}"),
            Self::MeshFailed { solid_id, reason } => write!(formatter, "solid {solid_id} failed to mesh: {reason}"),
            Self::LoadCaseNotFound(id) => write!(formatter, "load case not found: {id}"),
            Self::ModeIndexOutOfRange(index) => write!(formatter, "mode index out of range: {index}"),
            Self::Fem(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for Fem3dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fem(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::model::FemError> for Fem3dError {
    fn from(error: crate::model::FemError) -> Self {
        Self::Fem(error)
    }
}
// #endregion 🔖️Errors

// #region 🔖️Bridge
/// 🌉️ Resolves a `Fem3dSnapshot` load case into a `crate::model::Model`: nodes, `Bar3`/`Frame3`/`Tet4`
/// elements (materials/sections looked up by id), supports, and the named load case's translated loads.
pub fn build_model(doc: &Fem3dSnapshot, case_id: &str) -> Result<crate::model::Model, Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = meshing::translate_loads(&case.loads, &solids)?;
    Ok(crate::model::Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🚀️ Frozen entry point: builds the model for `case_id` and runs `crate::model::solve_linear_static`.
/// Consumed directly by `fem-plugin`; do not rename or change this signature.
pub fn fem3d_solve(doc: &Fem3dSnapshot, case_id: &str) -> Result<crate::model::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    crate::model::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Builds an `AnalysisModel` plus one `analyses::LoadCase` per `doc.load_cases` entry and one
/// `analyses::Combination` per `doc.combinations` entry, solving them ALL at once via
/// `crate::analyses::solve_multi_case` (self-weight honored via `doc.materials`' `rho`, gravity
/// fixed at `[0.0, 0.0, -9.81]` — this crate is Z-up, per `FemNode`'s `{x,y,z}` fields and the existing
/// cantilever test's `Dof::Tz` tip load). Returns results keyed by case id ∪ combination id.
pub fn fem3d_solve_all(doc: &Fem3dSnapshot) -> Result<HashMap<String, crate::model::StaticResult>, Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for case in &doc.load_cases {
        let (nodal_loads, member_loads) = meshing::translate_loads(&case.loads, &solids)?;
        cases.push(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight });
    }
    let combinations: Vec<analyses::Combination> = doc.combinations.iter().map(|combination| analyses::Combination { id: combination.id.clone(), terms: combination.terms.iter().map(|(id, factor)| (id.clone(), *factor)).collect() }).collect();
    analyses::solve_multi_case(&model, &cases, &combinations, [0.0, 0.0, -9.81]).map_err(Fem3dError::from)
}
// #endregion 🔖️Bridge

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
