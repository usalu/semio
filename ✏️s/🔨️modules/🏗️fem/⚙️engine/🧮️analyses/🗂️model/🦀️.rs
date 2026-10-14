//! 🗂️ Analysis model: load cases, combinations, the owned and mounted analysis models with their bounded close,
//! modal and buckling result records, the DOF map, reference validation and the RCM equation permutation.

use replication::value::list::PagedList;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use semio_s_fem_algebra::VecD;
use semio_s_fem_model::{Dof, Element, FemError, MemberUdl, NodalLoad, Node, Support};
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::rcm_order;

pub const MOUNTED_OWNER_PAGE_BYTES: usize = 4_096;
pub const ASSEMBLY_TRIPLET_INDEX_SPACE: usize = usize::MAX;

pub fn encode_value<T: semio_framework_value::ToValue>(value: &T) -> Vec<u8> {
    semio_framework_os_pack::pack_rt::encode_wire_value(&value.to_value())
}

pub fn decode_value<T: semio_framework_value::FromValue>(bytes: &[u8]) -> Result<T, String> {
    let value = semio_framework_os_pack::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
    T::from_value(value).map_err(|error| error.to_string())
}

pub fn reserve_exact_owner_page<T>(owner: &mut Vec<T>, additional: usize) -> bool {
    owner.try_reserve_exact(additional).is_ok() && owner.capacity().checked_mul(size_of::<T>()).is_some_and(|bytes| bytes <= MOUNTED_OWNER_PAGE_BYTES)
}

pub fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

pub fn close_paged_owner_step<T, const N: usize>(owner: &mut PagedList<T, N>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    if owner.terminal_is_empty() {
        return Ok(None);
    }
    let progress = owner.release_empty_page(maximum_bytes).map_err(|_| ())?;
    progress.progressed.then_some(Some((1, progress.released_allocation_bytes))).ok_or(())
}

pub fn cold_paged_owner<T, const N: usize>(capacity: usize) -> Result<PagedList<T, N>, FemError> {
    let mut owner = PagedList::default();
    while owner.capacity() < capacity {
        if !owner.reserve_capacity_one(capacity, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?.progressed {
            return Err(FemError::Singular);
        }
    }
    Ok(owner)
}

// #region 🔖️Model
/// 📦️ A named load case: nodal loads, member UDLs, and an optional self-weight contribution.
pub struct LoadCase {
    pub id: String,
    pub nodal_loads: Vec<NodalLoad>,
    pub member_loads: Vec<(String, MemberUdl)>,
    pub self_weight: bool,
}

/// 📦️ A linear combination of load cases — `Σ factor_i * case_i`, superposed from already-solved
/// case results (no re-solve).
pub struct Combination {
    pub id: String,
    pub terms: Vec<(String, f64)>,
}

/// 🏗️ Model geometry for multi-case/modal/buckling analysis — no loads (those come from `LoadCase`).
pub struct AnalysisModel {
    pub nodes: Vec<Node>,
    pub elements: Vec<Elements>,
    pub supports: Vec<Support>,
}

pub const MOUNTED_ANALYSIS_NODE_SLOTS: usize = 128;
pub const MOUNTED_ANALYSIS_ELEMENT_SLOTS: usize = 128;
pub const MOUNTED_ANALYSIS_SUPPORT_SLOTS: usize = 64;
pub const MOUNTED_ANALYSIS_BACKING_BYTES: usize = MOUNTED_OWNER_PAGE_BYTES;

pub struct MountedAnalysisSupport {
    pub node_id: String,
    pub fixed: [Option<Dof>; 6],
    pub fixed_len: usize,
}

impl MountedAnalysisSupport {
    pub fn new(node_id: String) -> Self {
        Self { node_id, fixed: [None; 6], fixed_len: 0 }
    }

    pub fn push_fixed(&mut self, dof: Dof) -> Result<(), Dof> {
        if self.fixed_len == self.fixed.len() {
            return Err(dof);
        }
        self.fixed[self.fixed_len] = Some(dof);
        self.fixed_len += 1;
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.node_id.capacity() != 0 {
            let bytes = self.node_id.capacity();
            if bytes > maximum_bytes {
                return (false, 0, 0);
            }
            self.node_id = String::new();
            return (false, 1, bytes);
        }
        if self.fixed_len != 0 {
            self.fixed_len -= 1;
            self.fixed[self.fixed_len] = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}

/// 🚧️ A mounted model retains allocation failures separately from logical admission limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedAnalysisFault {
    Capacity { requested: usize, maximum: usize },
    Allocation { allocated_bytes: usize, reason: &'static str },
    Closing,
}

/// 🎟️ One model admission opportunity reports only its actual newly retained backing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MountedAnalysisAdmission {
    pub complete: bool,
    pub allocated_bytes: usize,
}

pub struct MountedModelItems<T, const N: usize> {
    pub values: PagedList<T, N>,
    admitted: usize,
}

impl<T, const N: usize> Default for MountedModelItems<T, N> {
    fn default() -> Self {
        Self { values: PagedList::default(), admitted: 0 }
    }
}

impl<T, const N: usize> MountedModelItems<T, N> {
    fn next_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        if target > N { return Err(MountedAnalysisFault::Capacity { requested: target, maximum: N }); }
        self.values.next_capacity_allocation_bytes(target).map_err(|error| MountedAnalysisFault::Allocation { allocated_bytes: 0, reason: error.reason })
    }

    fn admit_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        if self.next_allocation_bytes(target)?.is_some() {
            let progress = self.values.reserve_capacity_one(target, maximum_bytes.min(MOUNTED_OWNER_PAGE_BYTES)).map_err(|error| MountedAnalysisFault::Allocation { allocated_bytes: error.allocated_bytes, reason: error.reason })?;
            return Ok(MountedAnalysisAdmission { complete: false, allocated_bytes: progress.allocated_bytes });
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(MountedAnalysisAdmission::default());
        }
        Ok(MountedAnalysisAdmission { complete: true, allocated_bytes: 0 })
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.values.len() >= self.admitted { return Err(value); }
        self.values.push_reserved(value)
    }
}

pub trait MountedAnalysisItem {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize);
}

impl MountedAnalysisItem for Node {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let bytes = self.id.capacity();
        if bytes == 0 { return (true, 0, 0); }
        if bytes > maximum_bytes { return (false, 0, 0); }
        self.id = String::new();
        (false, 1, bytes)
    }
}

impl MountedAnalysisItem for Elements {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(bytes) = self.mounted_next_string_bytes() {
            if bytes > maximum_bytes { return (false, 0, 0); }
            return (false, 1, self.close_mounted_string_step().expect("admitted mounted element string"));
        }
        (self.mounted_strings_terminal_is_empty(), 0, 0)
    }
}

impl MountedAnalysisItem for MountedAnalysisSupport {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        self.close_step(maximum_bytes)
    }
}

impl<T: MountedAnalysisItem, const N: usize> MountedModelItems<T, N> {
    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(value) = self.values.len().checked_sub(1).and_then(|index| self.values.get_mut(index)) {
            let progress = value.close_item_step(maximum_bytes);
            if !progress.0 { return progress; }
            self.values.pop();
            return (false, 1, 0);
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return (false, 1, 0);
        }
        match close_paged_owner_step(&mut self.values, maximum_bytes) {
            Ok(Some((items, bytes))) => (false, items, bytes),
            Ok(None) => (true, 0, 0),
            Err(()) => (false, 0, 0),
        }
    }
}

/// 🧱 Mounted model metadata moves directly while each payload page retains its own byte grant.
pub struct MountedAnalysisModel {
    pub nodes: MountedModelItems<Node, MOUNTED_ANALYSIS_NODE_SLOTS>,
    pub elements: MountedModelItems<Elements, MOUNTED_ANALYSIS_ELEMENT_SLOTS>,
    pub supports: MountedModelItems<MountedAnalysisSupport, MOUNTED_ANALYSIS_SUPPORT_SLOTS>,
    pub fault: Option<MountedAnalysisFault>,
    pub closing: bool,
    pub close_lane: u8,
}

impl MountedAnalysisModel {
    pub fn new() -> Self {
        Self { nodes: MountedModelItems::default(), elements: MountedModelItems::default(), supports: MountedModelItems::default(), fault: None, closing: false, close_lane: 0 }
    }

    pub fn admission_ready(&self) -> Result<(), MountedAnalysisFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        if self.closing { return Err(MountedAnalysisFault::Closing); }
        Ok(())
    }

    pub fn retain_admission(&mut self, result: Result<MountedAnalysisAdmission, MountedAnalysisFault>) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        if let Err(fault) = result { self.fault = Some(fault); }
        result
    }

    pub fn next_node_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.nodes.next_allocation_bytes(target)
    }

    pub fn next_element_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.elements.next_allocation_bytes(target)
    }

    pub fn next_support_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.supports.next_allocation_bytes(target)
    }

    pub fn admit_node_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.nodes.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn admit_element_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.elements.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn admit_support_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.supports.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn push_node(&mut self, node: Node) -> Result<(), Node> {
        if self.admission_ready().is_err() { return Err(node); }
        self.nodes.push(node)
    }

    pub fn push_element(&mut self, element: Elements) -> Result<(), Elements> {
        if self.admission_ready().is_err() { return Err(element); }
        self.elements.push(element)
    }

    pub fn push_support(&mut self, support: MountedAnalysisSupport) -> Result<(), MountedAnalysisSupport> {
        if self.admission_ready().is_err() { return Err(support); }
        self.supports.push(support)
    }

    pub fn nodes_len(&self) -> usize { self.nodes.values.len() }
    pub fn elements_len(&self) -> usize { self.elements.values.len() }
    pub fn node(&self, index: usize) -> Option<&Node> { self.nodes.values.get(index) }
    pub fn element(&self, index: usize) -> Option<&Elements> { self.elements.values.get(index) }
    pub fn support(&self, index: usize) -> Option<&MountedAnalysisSupport> { self.supports.values.get(index) }

    pub fn physical_backing_bytes(&self) -> [usize; 3] {
        [self.nodes.values.allocated_bytes(), self.elements.values.allocated_bytes(), self.supports.values.allocated_bytes()]
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.nodes.values.terminal_is_empty() && self.elements.values.terminal_is_empty() && self.supports.values.terminal_is_empty() && self.nodes.admitted == 0 && self.elements.admitted == 0 && self.supports.admitted == 0
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        self.closing = true;
        loop {
            let progress = match self.close_lane {
                0 => self.nodes.close_step(maximum_bytes),
                1 => self.elements.close_step(maximum_bytes),
                2 => self.supports.close_step(maximum_bytes),
                _ => return (self.terminal_is_empty(), 0, 0),
            };
            if !progress.0 { return progress; }
            self.close_lane += 1;
        }
    }
}

impl Default for MountedAnalysisModel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct AnalysisModelCloseCursor {
    pub lane: u8,
}

pub fn close_analysis_model_step(owner: &mut Arc<AnalysisModel>, cursor: &mut AnalysisModelCloseCursor, maximum_bytes: usize) -> (bool, usize, usize) {
    let Some(model) = Arc::get_mut(owner) else {
        return (false, 0, 0);
    };
    loop {
        match cursor.lane {
            0 => {
                let Some(node) = model.nodes.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if node.id.capacity() != 0 {
                    let bytes = node.id.capacity();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    node.id = String::new();
                    return (false, 1, bytes);
                }
                model.nodes.pop();
                return (false, 1, 0);
            }
            1 => {
                let bytes = model.nodes.capacity() * size_of::<Node>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.nodes = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            2 => {
                let Some(element) = model.elements.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    return (false, 1, element.close_mounted_string_step().expect("mounted model close witness changed without mutation"));
                }
                if !element.mounted_strings_terminal_is_empty() {
                    return (false, 0, 0);
                }
                model.elements.pop();
                return (false, 1, 0);
            }
            3 => {
                let bytes = model.elements.capacity() * size_of::<Elements>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.elements = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            4 => {
                let Some(support) = model.supports.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if support.node_id.capacity() != 0 {
                    let bytes = support.node_id.capacity();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    support.node_id = String::new();
                    return (false, 1, bytes);
                }
                if support.fixed.pop().is_some() {
                    return (false, 1, 0);
                }
                let bytes = support.fixed.capacity() * size_of::<Dof>();
                if bytes != 0 {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    support.fixed = Vec::new();
                    return (false, 1, bytes);
                }
                model.supports.pop();
                return (false, 1, 0);
            }
            5 => {
                let bytes = model.supports.capacity() * size_of::<Support>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.supports = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            _ => return (true, 0, 0),
        }
    }
}

/// 📐️ The lowest modes of a `modal` analysis — `shapes[i]` is node-major matching `model.nodes`
/// order, DOF sub-order `Tx,Ty,Tz,Rx,Ry,Rz` filtered to each node's active DOFs (the same layout
/// `StaticResult`'s node list implies), zero at every constrained DOF.
pub struct ModalResult {
    pub frequencies_hz: Vec<f64>,
    pub shapes: Vec<VecD>,
}

/// 📐️ The lowest linear-buckling load factors of a `buckling` analysis — `factors[i] * reference_case`
/// is the critical load; `shapes[i]` uses the same layout as `ModalResult::shapes`.
pub struct BucklingResult {
    pub factors: Vec<f64>,
    pub shapes: Vec<VecD>,
}
// #endregion 🔖️Model

// #region 🔖️DofMap
/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it) —
/// a small, self-contained reimplementation of `lib.rs`'s private `build_dof_map`/`DofMap` (not
/// `pub`, so not importable here), kept byte-for-byte equivalent in ordering behavior.
pub struct DofMap {
    pub order: Vec<(String, Dof)>,
    pub slots: HashMap<String, [Option<usize>; 6]>,
}

impl DofMap {
    /// 🔢️ Wraps an ordered `(node, dof)` run with its per-node slot index, so a lookup is one hash
    /// probe instead of a scan of the whole order — the scan made every assembly and recovery pass
    /// quadratic in the mesh (`elements × dofs × order`), which is what dominated a two-thousand-node
    /// continuum model before the factorization even started.
    pub fn from_order(order: Vec<(String, Dof)>) -> Self {
        let mut slots: HashMap<String, [Option<usize>; 6]> = HashMap::new();
        for (index, (node_id, dof)) in order.iter().enumerate() {
            slots.entry(node_id.clone()).or_insert([None; 6])[dof.index()] = Some(index);
        }
        Self { order, slots }
    }

    pub fn get(&self, node_id: &str, dof: Dof) -> Option<usize> {
        self.slots.get(node_id)?[dof.index()]
    }

    /// 🔢️ Appends one `(node, dof)` owner at the end of the order and indexes it — the mounted
    /// construction emits the order one owner per step.
    pub fn push(&mut self, owner: (String, Dof)) {
        let index = self.order.len();
        self.slots.entry(owner.0.clone()).or_insert([None; 6])[owner.1.index()] = Some(index);
        self.order.push(owner);
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }
}

pub fn build_dof_map(nodes: &[Node], elements: &[Elements]) -> DofMap {
    let mut active_of: HashMap<String, Vec<Dof>> = HashMap::new();
    for element in elements {
        for id in element.node_ids() {
            let active = active_of.entry(id).or_default();
            for &dof in element.dofs_per_node() {
                if !active.contains(&dof) {
                    active.push(dof);
                }
            }
        }
    }
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = active_of.get(node.id.as_str()).cloned().unwrap_or_default();
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    DofMap::from_order(order)
}

/// 📍️ World positions of `node_ids` through a node-id index — built once per model, never a scan
/// of the node list per element.
pub fn positions_of(node_slot: &HashMap<String, usize>, nodes: &[Node], node_ids: &[String]) -> Vec<[f64; 3]> {
    node_ids.iter().map(|id| node_slot.get(id).map(|&slot| nodes[slot].pos).unwrap_or_default()).collect()
}

/// 📍️ The node-id → slot index `positions_of` resolves through.
pub fn node_index_of(nodes: &[Node]) -> HashMap<String, usize> {
    nodes.iter().enumerate().map(|(slot, node)| (node.id.clone(), slot)).collect()
}

pub fn element_global_indices(dof_map: &DofMap, node_ids: &[String], dofs: &[Dof]) -> Option<Vec<usize>> {
    let mut indices = Vec::with_capacity(node_ids.len() * dofs.len());
    for node_id in node_ids {
        for &dof in dofs {
            indices.push(dof_map.get(node_id, dof)?);
        }
    }
    Some(indices)
}
// #endregion 🔖️DofMap

// #region 🔖️Validate
pub fn validate(model: &AnalysisModel) -> Result<(), FemError> {
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
    Ok(())
}

pub fn validate_case(model: &AnalysisModel, case: &LoadCase) -> Result<(), FemError> {
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for load in &case.nodal_loads {
        if !node_exists(&load.node_id) {
            return Err(FemError::DanglingNodeRef(load.node_id.clone()));
        }
    }
    Ok(())
}
// #endregion 🔖️Validate

// #region 🔖️Rcm
/// 🌀️ Node-index RCM permutation, expanded to DOF granularity: each node's active DOFs stay
/// contiguous, positioned at its node's new RCM slot. `inv_perm[old_idx] = new_idx` (the only
/// direction callers need — un-permuting walks `old_idx` and looks up its new slot).
pub struct RcmPermutation {
    pub inv_perm: Vec<usize>,
}

pub fn build_rcm_permutation(nodes: &[Node], elements: &[Elements], dof_map: &DofMap) -> RcmPermutation {
    let n_nodes = nodes.len();
    let node_index: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n_nodes];
    let mut seen_edges: HashSet<(usize, usize)> = HashSet::new();
    for element in elements {
        let ids = element.node_ids();
        let idxs: Vec<usize> = ids.iter().filter_map(|id| node_index.get(id.as_str()).copied()).collect();
        for i in 0..idxs.len() {
            for j in (i + 1)..idxs.len() {
                let (a, b) = (idxs[i], idxs[j]);
                if a != b {
                    let key = (a.min(b), a.max(b));
                    if seen_edges.insert(key) {
                        adjacency[a].push(b);
                        adjacency[b].push(a);
                    }
                }
            }
        }
    }
    let node_perm = rcm_order(&adjacency);

    // `dof_map.order` is grouped by node in `nodes`' own iteration order (see `build_dof_map`), so
    // each original node index owns one contiguous run — walk it once to find each run's bounds.
    let mut node_dof_ranges: Vec<(usize, usize)> = vec![(0, 0); n_nodes];
    let mut cursor = 0usize;
    for (i, node) in nodes.iter().enumerate() {
        let mut count = 0;
        while cursor + count < dof_map.order.len() && dof_map.order[cursor + count].0 == node.id {
            count += 1;
        }
        node_dof_ranges[i] = (cursor, count);
        cursor += count;
    }

    let ndof = dof_map.len();
    let mut rcm_perm = Vec::with_capacity(ndof);
    for &old_node_idx in &node_perm {
        let (start, count) = node_dof_ranges[old_node_idx];
        for k in 0..count {
            rcm_perm.push(start + k);
        }
    }
    let mut inv_perm = vec![0usize; ndof];
    for (new_idx, &old_idx) in rcm_perm.iter().enumerate() {
        inv_perm[old_idx] = new_idx;
    }
    RcmPermutation { inv_perm }
}
// #endregion 🔖️Rcm

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
