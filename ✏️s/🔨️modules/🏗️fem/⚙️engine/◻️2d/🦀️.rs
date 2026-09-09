//! ⚙️ FEM 2D module engine — headless compute (pure FE algorithm, moved out of the artifact tree: an
//! artifact is a schema + io system, never an engine). Sibling files `🕸️meshing`/`🎵️modal-buckling`/
//! `🗺️mesh-preview` hold the region-meshing, modal/buckling and mesh-preview bridges respectively; this
//! file keeps the `Errors` region and the top-level `build_model`/solve entry points that aren't
//! specific to any of those three. `empty_fem2d_snapshot` lives in the artifact's own
//! `🧬️schema/🦀️component.rs` (pure document helper); `fem2d_io`/its ports live in
//! `🎛️apps/◻️2d/🦀️.rs` (app-facing `AppIo` surface).

use crate::fem2d_engine::meshing::{area_load_nodal_loads, build_nodes_and_elements, self_weight_nodal_loads, GRAVITY_G};
use crate::model::{MemberUdl, NodalLoad, Support};
use crate::{Fem2dSnapshot, FemLoad};
use std::collections::HashMap;

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem2dSnapshot`.
#[derive(Clone, Debug, PartialEq)]
pub enum Fem2dError {
    UnknownNodeId(String),
    UnknownMaterialId(String),
    UnknownSectionId(String),
    UnknownRegionId(String),
    MeshFailed { region_id: String, reason: String },
    LoadCaseNotFound(String),
    ModeIndexOutOfRange(usize),
    Fem(crate::model::FemError),
}

impl std::fmt::Display for Fem2dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownNodeId(id) => write!(formatter, "unknown node id: {id}"),
            Self::UnknownMaterialId(id) => write!(formatter, "unknown material id: {id}"),
            Self::UnknownSectionId(id) => write!(formatter, "unknown section id: {id}"),
            Self::UnknownRegionId(id) => write!(formatter, "unknown region id: {id}"),
            Self::MeshFailed { region_id, reason } => write!(formatter, "region {region_id} failed to mesh: {reason}"),
            Self::LoadCaseNotFound(id) => write!(formatter, "load case not found: {id}"),
            Self::ModeIndexOutOfRange(index) => write!(formatter, "mode index out of range: {index}"),
            Self::Fem(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for Fem2dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fem(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::model::FemError> for Fem2dError {
    fn from(error: crate::model::FemError) -> Self {
        Self::Fem(error)
    }
}
// #endregion 🔖️Errors

// #region 🔖️Solve
/// 🌉️ Resolves a `Fem2dSnapshot` plus a named load case into a `crate::model::Model`, erroring
/// descriptively on any dangling material/section/node/region reference.
pub fn build_model(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::model::Model, Fem2dError> {
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }));
            }
            FemLoad::Area { region_id, pressure, .. } => {
                let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads(region, *pressure));
            }
        }
    }
    if load_case.self_weight {
        nodal_loads.extend(self_weight_nodal_loads(doc, &regions));
    }

    Ok(crate::model::Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🌉️ Frozen public entry point: solves a `Fem2dSnapshot`'s named load case for linear-static
/// equilibrium. Signature is a contract consumed directly by the plugin host; do not rename or
/// change it.
pub fn fem2d_solve(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::model::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    crate::model::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Richer entry point: resolves EVERY `doc.load_cases`/`doc.combinations` entry at once (regions
/// meshed via the same `build_nodes_and_elements` resolution as `build_model`) and solves them all
/// together via `crate::analyses::solve_multi_case` — self-weight honored per-case through
/// `doc.materials`' `rho` (see `self_weight_nodal_loads`'s doc for the `Tri3Cst` caveat), gravity
/// fixed at `[0.0, -9.81, 0.0]`. Returns results keyed by case id ∪ combination id.
pub fn fem2d_solve_all(doc: &Fem2dSnapshot) -> Result<HashMap<String, crate::model::StaticResult>, Fem2dError> {
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };

    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for load_case in &doc.load_cases {
        let mut nodal_loads = Vec::new();
        let mut member_loads = Vec::new();
        for load in &load_case.loads {
            match load {
                FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
                FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                    member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }));
                }
                FemLoad::Area { region_id, pressure, .. } => {
                    let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                    nodal_loads.extend(area_load_nodal_loads(region, *pressure));
                }
            }
        }
        cases.push(crate::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight });
    }

    let combinations: Vec<crate::analyses::Combination> = doc.combinations.iter().map(|c| crate::analyses::Combination { id: c.id.clone(), terms: c.terms.iter().map(|t| (t.case_id.clone(), t.factor)).collect() }).collect();

    crate::analyses::solve_multi_case(&model, &cases, &combinations, [0.0, -GRAVITY_G, 0.0]).map_err(Fem2dError::from)
}
// #endregion 🔖️Solve

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
