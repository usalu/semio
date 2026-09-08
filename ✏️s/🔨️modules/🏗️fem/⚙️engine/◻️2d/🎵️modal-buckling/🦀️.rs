//! 🎵️ FEM 2D module engine — modal/buckling analysis bridge (pure FE algorithm, moved out of the
//! artifact tree).

use crate::{Fem2dSnapshot, FemLoad};
use crate::fem2d_engine::meshing::build_nodes_and_elements;
use crate::fem2d_engine::Fem2dError;
use crate::model::{Dof, Element, Elements, MemberUdl, Node, Support};
use std::collections::HashMap;

/// 🔢️ Node-major, active-DOF-filtered ordering matching `crate::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — a small local reimplementation (mirrors
/// `analyses::build_dof_map`, which isn't `pub`, following the same precedent that module's own doc
/// comment sets for `lib.rs`'s private `build_dof_map`) used to unpack a raw mode-shape `VecD` back
/// into per-node `[f64;6]` values.
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
pub fn fem2d_modal(doc: &Fem2dSnapshot) -> Result<crate::analyses::ModalResult, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    crate::analyses::modal(&model, doc.analysis.modal_count).map_err(Fem2dError::from)
}

/// 🌉️ Richer modal entry point: solves the same modal analysis as `fem2d_modal` but also unpacks mode
/// `mode_index`'s shape `VecD` into a friendly per-node `[f64;6]` displacement map (see `mode_dof_order`),
/// ready to feed the same deformed-shape rendering the results window already uses for static results.
/// Returns `(frequency_hz, node_id -> displacement values)`.
pub fn fem2d_modal_mode_values(doc: &Fem2dSnapshot, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    let result = crate::analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🧩️ `buckling_inputs`'s resolved `(nodes, elements, supports, load case)` quadruple.
type BucklingInputs = (Vec<Node>, Vec<Elements>, Vec<Support>, crate::analyses::LoadCase);

/// 🌉️ Shared buckling-case resolution for `fem2d_buckling`/`fem2d_buckling_mode_values`: builds the
/// geometry plus the ONE named `case_id`'s `analyses::LoadCase`, mirroring `fem2d_solve_all`'s
/// per-case load translation (nodal/member-UDL/area loads), erroring `"load case not found: {case_id}"`
/// if `case_id` isn't in `doc.load_cases`.
fn buckling_inputs(doc: &Fem2dSnapshot, case_id: &str) -> Result<BucklingInputs, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let member_node_ids: std::collections::HashSet<String> = doc.nodes.iter().map(|n| n.id.clone()).collect();
    let nodes: Vec<Node> = nodes.into_iter().filter(|n| member_node_ids.contains(&n.id)).collect();
    let elements: Vec<Elements> = elements.into_iter().take(doc.elements.len()).collect();
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(crate::model::NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 })),
            FemLoad::Area { .. } => {}
        }
    }
    let case = crate::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight };
    Ok((nodes, elements, supports, case))
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem2d_buckling(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::analyses::BucklingResult, Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    crate::analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem2dError::from)
}

/// 🌉️ Richer buckling entry point: mirrors `fem2d_modal_mode_values` — solves the same buckling
/// analysis as `fem2d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem2d_buckling_mode_values(doc: &Fem2dSnapshot, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    let result = crate::analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
    let factor = *result.factors.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
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
