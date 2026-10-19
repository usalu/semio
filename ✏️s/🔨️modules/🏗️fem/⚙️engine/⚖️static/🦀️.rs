//! ➗️ linear-static solve `Ku = F` with reactions and per-element recovery. `semio-s-fem-analyses` is the sparse
//! ⚖️ Linear static analysis: the closed first-party element set, the assembled structural `Model` and its dense
//! multi-case pipeline for larger models.

use semio_framework_dispatch_macros::dyn_enum_close;
use semio_s_fem_algebra::MatD;
use semio_s_fem_algebra::VecD;
use semio_s_fem_elements2d::Bar2;
use semio_s_fem_elements2d::BeamEb2;
use semio_s_fem_elements2d::PlateDkt;
use semio_s_fem_elements2d::Quad4;
use semio_s_fem_elements2d::Quad8;
use semio_s_fem_elements2d::Tri3Cst;
use semio_s_fem_elements2d::Tri6Lst;
use semio_s_fem_elements3d::Bar3;
use semio_s_fem_elements3d::Frame3;
use semio_s_fem_elements3d::Hex8;
use semio_s_fem_elements3d::ShellFacet3;
use semio_s_fem_elements3d::Tet4;
use semio_s_fem_model::__semio_dispatch_Element;
use semio_s_fem_model::AxialSpring;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_model::ElementContext;
use semio_s_fem_model::ElementResult;
use semio_s_fem_model::FemError;
use semio_s_fem_model::MemberUdl;
use semio_s_fem_model::NodalLoad;
use semio_s_fem_model::Node;
use semio_s_fem_model::NodeDisplacement;
use semio_s_fem_model::NodeReaction;
use semio_s_fem_model::SolutionChecks;
use semio_s_fem_model::StaticResult;
use semio_s_fem_model::Support;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

// #region 🔖️Model
dyn_enum_close! {
    /// 🧬️ O1 closed-set dispatch for `Element` — the 13 first-party implementors (12 real 2D/3D elements of
    /// `semio-s-fem-elements2d`/`-elements3d` plus the contract's `AxialSpring`); see `📓️terra-fem-report.md`. The
    /// trait's exported `__semio_dispatch_Element!` is imported above, so the closed set lives beside the solve.
    pub enum Elements: Element {
        AxialSpring(AxialSpring),
        Bar2(Bar2),
        BeamEb2(BeamEb2),
        Tri3Cst(Tri3Cst),
        Tri6Lst(Tri6Lst),
        Quad4(Quad4),
        Quad8(Quad8),
        PlateDkt(PlateDkt),
        Bar3(Bar3),
        Frame3(Frame3),
        Tet4(Tet4),
        Hex8(Hex8),
        ShellFacet3(ShellFacet3),
    }
}

/// 🏗️ The assembled structural model handed to `solve_linear_static`.
#[derive(Default)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub elements: Vec<Elements>,
    pub supports: Vec<Support>,
    pub nodal_loads: Vec<NodalLoad>,
    pub member_loads: Vec<(String, MemberUdl)>,
}

/// 🔍️ No `Elements` variant derives `Debug`, so print element ids/count instead — this is what lets
/// `Result<Model, _>` be used with `unwrap_err()`/`expect_err()` in caller test code.
impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("nodes", &self.nodes)
            .field("elements", &self.elements.iter().map(|e| e.id()).collect::<Vec<_>>())
            .field("supports", &self.supports)
            .field("nodal_loads", &self.nodal_loads)
            .field("member_loads", &self.member_loads)
            .finish()
    }
}
// #endregion 🔖️Model

// #region 🔖️DofMap
struct DofMap {
    index: HashMap<(String, Dof), usize>,
    order: Vec<(String, Dof)>,
}

impl DofMap {
    fn get(&self, node_id: &str, dof: Dof) -> Option<usize> {
        self.index.get(&(node_id.to_string(), dof)).copied()
    }

    fn len(&self) -> usize {
        self.order.len()
    }
}

/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it),
/// so nodes with no rotational stiffness never get a spurious, singular rotational equation.
fn build_dof_map(model: &Model) -> DofMap {
    let mut order = Vec::new();
    let mut index = HashMap::new();
    for node in &model.nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in &model.elements {
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
            index.insert((node.id.clone(), dof), order.len());
            order.push((node.id.clone(), dof));
        }
    }
    DofMap { index, order }
}
// #endregion 🔖️DofMap

// #region 🔖️Validate
fn validate(model: &Model) -> Result<(), FemError> {
    if model.nodes.is_empty() {
        return Err(FemError::EmptyModel);
    }
    let mut seen = HashSet::new();
    for node in &model.nodes {
        if !seen.insert(node.id.clone()) {
            return Err(FemError::DuplicateNodeId(node.id.clone()));
        }
    }
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for element in &model.elements {
        for id in element.node_ids() {
            if !node_exists(&id) {
                return Err(FemError::DanglingNodeRef(id));
            }
        }
    }
    for support in &model.supports {
        if !node_exists(&support.node_id) {
            return Err(FemError::DanglingNodeRef(support.node_id.clone()));
        }
    }
    for load in &model.nodal_loads {
        if !node_exists(&load.node_id) {
            return Err(FemError::DanglingNodeRef(load.node_id.clone()));
        }
    }
    Ok(())
}
// #endregion 🔖️Validate

// #region 🔖️Assembly
fn positions_of(model: &Model, node_ids: &[String]) -> Vec<[f64; 3]> {
    node_ids.iter().map(|id| model.nodes.iter().find(|n| &n.id == id).map(|n| n.pos).unwrap_or_default()).collect()
}

fn element_global_indices(dof_map: &DofMap, node_ids: &[String], dofs: &[Dof]) -> Option<Vec<usize>> {
    let mut indices = Vec::with_capacity(node_ids.len() * dofs.len());
    for node_id in node_ids {
        for &dof in dofs {
            indices.push(dof_map.get(node_id, dof)?);
        }
    }
    Some(indices)
}

fn extract_submatrix(k: &MatD, rows: &[usize], cols: &[usize]) -> MatD {
    let mut out = MatD::zeros(rows.len(), cols.len());
    for (i, &row) in rows.iter().enumerate() {
        for (j, &col) in cols.iter().enumerate() {
            out.set(i, j, k.get(row, col));
        }
    }
    out
}
// #endregion 🔖️Assembly

// #region 🔖️Solve
/// 🧮️ Assembles and solves the model for linear-static equilibrium `Ku = F`, partitioned by
/// support restraints (dense elimination — small/single-case scale; `analyses::solve_multi_case`
/// is the sparse multi-case/combination pipeline for larger models), then recovers reactions and
/// per-element internal forces from the solved displacement vector.
pub fn solve_linear_static(model: &Model) -> Result<StaticResult, FemError> {
    validate(model)?;
    let dof_map = build_dof_map(model);
    let ndof = dof_map.len();
    let mut k = MatD::zeros(ndof, ndof);
    let mut f = VecD::zeros(ndof);

    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices = match element_global_indices(&dof_map, &node_ids, dofs) {
            Some(indices) => indices,
            None => continue,
        };
        let ctx = ElementContext { positions: positions_of(model, &node_ids) };
        let ke = element.stiffness_global(&ctx);
        for (local_row, &global_row) in indices.iter().enumerate() {
            for (local_col, &global_col) in indices.iter().enumerate() {
                k.add_at(global_row, global_col, ke.get(local_row, local_col));
            }
        }
        if let Some((_, udl)) = model.member_loads.iter().find(|(id, _)| id.as_str() == element.id()) {
            if let Some(fe) = element.equivalent_nodal_loads(&ctx, udl) {
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fe.get(local_row));
                }
            }
        }
    }

    for load in &model.nodal_loads {
        if let Some(idx) = dof_map.get(&load.node_id, load.dof) {
            f.add_at(idx, load.value);
        }
    }

    let mut constrained_set = BTreeSet::new();
    for support in &model.supports {
        for &dof in &support.fixed {
            if let Some(idx) = dof_map.get(&support.node_id, dof) {
                constrained_set.insert(idx);
            }
        }
    }
    let free: Vec<usize> = (0..ndof).filter(|i| !constrained_set.contains(i)).collect();
    let constrained: Vec<usize> = constrained_set.into_iter().collect();

    let kff = extract_submatrix(&k, &free, &free);
    let ff = VecD::from_vec(free.iter().map(|&i| f.get(i)).collect());
    let uf = kff.lu_solve(&ff).ok_or(FemError::Singular)?;

    let mut u = VecD::zeros(ndof);
    for (i, &idx) in free.iter().enumerate() {
        u.set(idx, uf.get(i));
    }

    let mut reactions = Vec::with_capacity(constrained.len());
    for &c in &constrained {
        let mut r = 0.0;
        for j in 0..ndof {
            r += k.get(c, j) * u.get(j);
        }
        r -= f.get(c);
        let (node_id, dof) = dof_map.order[c].clone();
        reactions.push(NodeReaction { node_id, dof, value: r });
    }

    let mut displacements: Vec<NodeDisplacement> = model.nodes.iter().map(|node| NodeDisplacement { node_id: node.id.clone(), values: [0.0; 6] }).collect();
    for (idx, (node_id, dof)) in dof_map.order.iter().enumerate() {
        if let Some(entry) = displacements.iter_mut().find(|d| &d.node_id == node_id) {
            entry.values[dof.index()] = u.get(idx);
        }
    }

    let mut elements = Vec::with_capacity(model.elements.len());
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices = match element_global_indices(&dof_map, &node_ids, dofs) {
            Some(indices) => indices,
            None => continue,
        };
        let ctx = ElementContext { positions: positions_of(model, &node_ids) };
        let u_local = VecD::from_vec(indices.iter().map(|&idx| u.get(idx)).collect());
        let udl = model.member_loads.iter().find(|(id, _)| id.as_str() == element.id()).map(|(_, udl)| udl);
        elements.push((element.id().to_string(), element.recover(&ctx, &u_local, udl)));
    }

    let mut reaction_sum = [0.0; 6];
    for r in &reactions {
        reaction_sum[r.dof.index()] += r.value;
    }
    for (idx, (_, dof)) in dof_map.order.iter().enumerate() {
        reaction_sum[dof.index()] += f.get(idx);
    }
    let ku = k.mul_vec(&u);
    let ku_free = VecD::from_vec(free.iter().map(|&i| ku.get(i)).collect());
    let residual_norm = ku_free.sub(&ff).norm2() / ff.norm2().max(1e-9);

    Ok(StaticResult { displacements, reactions, elements, checks: SolutionChecks { residual_norm, reaction_sum } })
}
// #endregion 🔖️Solve


// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️elements2d/🦀️.rs"]
mod elements2d_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️continuum/🦀️.rs"]
mod continuum_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️plate/🦀️.rs"]
mod plate_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️elements3d/🦀️.rs"]
mod elements3d_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️shell/🦀️.rs"]
mod shell_tests;
// #endregion 🔖️Tests
