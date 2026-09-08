//! 🎵️ FEM 3D module engine — modal + linear buckling analyses (pure FE algorithm, moved out of the
//! artifact tree).

use crate::analyses;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::fem3d_engine::{meshing, Fem3dError};
use crate::model::{Dof, Element, Elements, Node};

/// 🔢️ Node-major, active-DOF-filtered ordering matching `crate::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — mirrors `fem_2d`'s identically named helper
/// (both are small local reimplementations of `analyses::build_dof_map`, which isn't `pub`).
fn mode_dof_order(nodes: &[Node], elements: &[Elements]) -> Vec<(String, Dof)> {
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in elements {
            if element.node_ids().iter().any(|id| id == &node.id) {
                for &dof in element.dofs_per_node() {
                    if !active.contains(&dof) {
                        active.push(dof);
                    }
                }
            }
        }
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    order
}

/// 🎵️ Modal analysis: lowest `doc.analysis.modal_count` natural frequencies/mode shapes.
pub fn fem3d_modal(doc: &Fem3dSnapshot) -> Result<analyses::ModalResult, Fem3dError> {
    let (nodes, elements, _solids, supports) = meshing::resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::modal(&model, doc.analysis.modal_count).map_err(Fem3dError::from)
}

/// 🌉️ Richer modal entry point: solves the same modal analysis as `fem3d_modal` but also unpacks mode
/// `mode_index`'s shape into a per-node `[f64;6]` displacement map. Returns
/// `(frequency_hz, node_id -> displacement values)`.
pub fn fem3d_modal_mode_values(doc: &Fem3dSnapshot, mode_index: usize) -> Result<(f64, std::collections::HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, _solids, supports) = meshing::resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: std::collections::HashMap<String, [f64; 6]> = std::collections::HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🌉️ Shared buckling-case resolution for `fem3d_buckling`/`fem3d_buckling_mode_values`, mirroring
/// `fem2d`'s `buckling_inputs` — translates the named case's loads (incl. `Area` against `solids`).
fn buckling_case(doc: &Fem3dSnapshot, case_id: &str, solids: &[meshing::MeshedSolid]) -> Result<analyses::LoadCase, Fem3dError> {
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = meshing::translate_loads(&case.loads, solids)?;
    Ok(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight })
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem3d_buckling(doc: &Fem3dSnapshot, case_id: &str) -> Result<analyses::BucklingResult, Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let case = buckling_case(doc, case_id, &solids)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem3dError::from)
}

/// 🌉️ Richer buckling entry point: mirrors `fem3d_modal_mode_values` — solves the same buckling
/// analysis as `fem3d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem3d_buckling_mode_values(doc: &Fem3dSnapshot, case_id: &str, mode_index: usize) -> Result<(f64, std::collections::HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let case = buckling_case(doc, case_id, &solids)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
    let factor = *result.factors.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: std::collections::HashMap<String, [f64; 6]> = std::collections::HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((factor, values))
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
