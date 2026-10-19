//! 🧷️ Assembly job stages, checkpoints, plan and the bounded job construction.

use super::*;

// #region 🔖️Assembly
/// 🧱️ Bounded phases of deterministic stiffness assembly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind")]
pub enum AssemblyJobStage {
    ElementTriplets,
    MergeFull,
    MergeFree,
    Complete,
}

impl AssemblyJobStage {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::ElementTriplets => "fem.assembly.element-triplets",
            Self::MergeFull => "fem.assembly.merge-full",
            Self::MergeFree => "fem.assembly.merge-free",
            Self::Complete => "fem.assembly.complete",
        }
    }
}

/// 👁️ Replaceable assembly progress for live element-mark rendering.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct AssemblyPreview {
    pub stage: AssemblyJobStage,
    pub completed_elements: usize,
    pub total_elements: usize,
    pub full_triplets: usize,
    pub free_triplets: usize,
    /// 🧩️ Ids assembled since the previous published preview — a bounded delta, never the whole
    /// roster, so a preview step stays under the interactive ceiling for any element count.
    pub assembled_element_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AssemblyTriplet {
    pub(super) sequence: u64,
    pub(super) row: u32,
    pub(super) col: u32,
    pub(super) value: f64,
}

#[derive(Clone, Default)]
pub(super) struct AssemblyPartitionBuffer {
    pub(super) full: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) free: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
}

#[derive(Clone)]
pub(super) struct PendingElementAssembly {
    pub(super) element_index: usize,
    pub(super) side: usize,
    pub(super) cell_cursor: usize,
    pub(super) reclaim_lane: u8,
    pub(super) complete: bool,
    pub(super) indices_new: Vec<usize>,
    pub(super) positions: Vec<[f64; 3]>,
    pub(super) stiffness: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum PendingElementBuildStage {
    ReserveIndices,
    Indices,
    PublishIndex,
    ReservePositions,
    Positions,
    PublishPosition,
    ReserveStiffnessCredit,
    AllocateStiffness,
    ReferenceQuadraturePoint,
    ShapeFunctionDerivativeScalar,
    JacobianCell,
    DeterminantInverseCell,
    StrainDisplacementCell,
    ConstitutiveCell,
    LocalStiffnessMultiplyCell,
    BodyTractionLoadCell,
    LocalToGlobalTripletCell,
    ObserveStiffnessBacking,
    AdmitStiffnessBacking,
    Complete,
}

impl PendingElementBuildStage {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::ReserveIndices => "fem.element.reserve-indices",
            Self::Indices => "fem.element.local-to-global-index",
            Self::PublishIndex => "fem.element.publish-global-index",
            Self::ReservePositions => "fem.element.reserve-positions",
            Self::Positions => "fem.element.position",
            Self::PublishPosition => "fem.element.publish-position",
            Self::ReserveStiffnessCredit => "fem.element.reserve-stiffness-credit",
            Self::AllocateStiffness => "fem.element.allocate-stiffness",
            Self::ReferenceQuadraturePoint => "fem.element.reference-quadrature-point",
            Self::ShapeFunctionDerivativeScalar => "fem.element.shape-function-derivative-scalar",
            Self::JacobianCell => "fem.element.jacobian-cell",
            Self::DeterminantInverseCell => "fem.element.determinant-inverse-cell",
            Self::StrainDisplacementCell => "fem.element.strain-displacement-cell",
            Self::ConstitutiveCell => "fem.element.constitutive-cell",
            Self::LocalStiffnessMultiplyCell => "fem.element.local-stiffness-multiply-cell",
            Self::BodyTractionLoadCell => "fem.element.body-traction-load-cell",
            Self::LocalToGlobalTripletCell => "fem.element.local-to-global-triplet-cell",
            Self::ObserveStiffnessBacking => "fem.element.observe-stiffness-backing",
            Self::AdmitStiffnessBacking => "fem.element.admit-stiffness-backing",
            Self::Complete => "fem.element.publish-candidate",
        }
    }
}

#[derive(Clone)]
pub(super) struct PendingElementBuild {
    pub(super) element_index: usize,
    pub(super) node_count: usize,
    pub(super) dof_count: usize,
    pub(super) scalar_cursor: usize,
    pub(super) lookup_cursor: usize,
    pub(super) lookup_match: Option<usize>,
    pub(super) stage: PendingElementBuildStage,
    pub(super) indices_new: Vec<usize>,
    pub(super) positions: Vec<[f64; 3]>,
    pub(super) stiffness: Vec<f64>,
    pub(super) stiffness_dimensions: [usize; 2],
    pub(super) stiffness_observed_bytes: usize,
    pub(super) stiffness_credit_reserved: bool,
    pub(super) stiffness_admitted: bool,
}

#[derive(Clone)]
pub(super) struct AssemblyCheckpoint {
    pub(super) stage: AssemblyJobStage,
    pub(super) total_elements: usize,
    pub(super) element_cursor: usize,
    pub(super) pending_build: Option<PendingElementBuild>,
    pub(super) pending: Option<PendingElementAssembly>,
    pub(super) partitions: Vec<AssemblyPartitionBuffer>,
    pub(super) full_merge_cursors: Vec<usize>,
    pub(super) free_merge_cursors: Vec<usize>,
    pub(super) merged_full: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) merged_free: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) checkpoint_due: bool,
    pub(super) preview_due: bool,
    pub(super) preview_cursor: usize,
    pub(super) resume_target: usize,
    pub(super) merge_scan_partition: usize,
    pub(super) merge_candidate: Option<(usize, SparseTriplet)>,
}

#[derive(ToValue, FromValue, semio_framework_value::RetireOwned)]
pub(super) struct AssemblyResumeCheckpoint {
    pub(super) version: u8,
    pub(super) model_signature: u64,
    pub(super) total_elements: usize,
    pub(super) completed_elements: usize,
    pub(super) partition_count: usize,
}

pub(super) struct AssemblyPlan {
    pub(super) dof_map: DofMap,
    pub(super) inv_perm: AssemblyOrder<usize>,
    pub(super) ndof: usize,
    pub(super) free_new: AssemblyOrder<usize>,
    pub(super) compact_of_new: AssemblyOrder<Option<usize>>,
    pub(super) node_slot: HashMap<String, usize>,
}

/// #️⃣️ Stable identity for rejecting a checkpoint against different FEM inputs.
pub(super) fn assembly_model_signature(model: &AnalysisModel) -> u64 {
    fn fold(mut hash: u64, bytes: &[u8]) -> u64 {
        for byte in bytes {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100_0000_01b3);
        }
        hash
    }

    let mut hash = 0xcbf2_9ce4_8422_2325;
    for node in &model.nodes {
        hash = fold(hash, node.id.as_bytes());
        for coordinate in node.pos {
            hash = fold(hash, &coordinate.to_bits().to_le_bytes());
        }
    }
    for element in &model.elements {
        hash = fold(hash, element.id().as_bytes());
        for node_id in element.node_ids() {
            hash = fold(hash, node_id.as_bytes());
        }
        for dof in element.dofs_per_node() {
            hash = fold(hash, &[*dof as u8]);
        }
    }
    for support in &model.supports {
        hash = fold(hash, support.node_id.as_bytes());
        for dof in &support.fixed {
            hash = fold(hash, &[*dof as u8]);
        }
    }
    hash
}

impl AssemblyPlan {
    pub(super) fn prepare(model: &AnalysisModel) -> Result<Self, FemError> {
        validate(model)?;
        let dof_map = build_dof_map(&model.nodes, &model.elements);
        let ndof = dof_map.len();
        let permutation = build_rcm_permutation(&model.nodes, &model.elements, &dof_map);
        let mut constrained_old = HashSet::new();
        for support in &model.supports {
            for &dof in &support.fixed {
                if let Some(index) = dof_map.get(&support.node_id, dof) {
                    constrained_old.insert(index);
                }
            }
        }
        let constrained_new: HashSet<usize> = constrained_old.iter().map(|&old| permutation.inv_perm[old]).collect();
        let free_new: Vec<usize> = (0..ndof).filter(|new_index| !constrained_new.contains(new_index)).collect();
        let mut compact_of_new = vec![None; ndof];
        for (compact, &new_index) in free_new.iter().enumerate() {
            compact_of_new[new_index] = Some(compact);
        }
        Ok(Self { dof_map, inv_perm: permutation.inv_perm.into_iter().collect(), ndof, free_new: free_new.into_iter().collect(), compact_of_new: compact_of_new.into_iter().collect(), node_slot: node_index_of(&model.nodes) })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssemblyConstructionStage {
    ReservePartitionTripletCounts,
    InitializePartitionTripletCounts,
    ReserveDofs,
    ReserveDofIndex,
    InitializeDofIndex,
    ValidateNodePairs,
    ValidateElementReferences,
    ValidateSupportReferences,
    DiscoverDofs,
    EmitDofs,
    CommitDofOwner,
    ReservePermutation,
    BuildPermutation,
    ReserveConstraints,
    InitializeConstraints,
    MarkConstraints,
    ReserveFree,
    BuildFree,
    ReserveCompact,
    InitializeCompact,
    BuildCompact,
    ReservePartitions,
    BuildPartitions,
    ReserveMergedFull,
    ReserveMergedFree,
    ReserveFullMergeCursors,
    ReserveFreeMergeCursors,
    BuildMergeCursors,
    Complete,
}

/// 🧵️ Retained mounted-session assembly-plan construction. Each call performs one reference
/// comparison, DOF insertion, scalar initialization or fixed-capacity allocation opportunity.
enum AssemblyConstructionModel {
    Dynamic(Arc<AnalysisModel>),
    Mounted(MountedAnalysisModel),
}

impl AssemblyConstructionModel {
    fn nodes_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.nodes.len(),
            Self::Mounted(model) => model.nodes_len(),
        }
    }

    pub(super) fn elements_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.elements.len(),
            Self::Mounted(model) => model.elements_len(),
        }
    }

    fn supports_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.supports.len(),
            Self::Mounted(model) => model.supports.values.len(),
        }
    }

    pub(super) fn node(&self, index: usize) -> Option<&Node> {
        match self {
            Self::Dynamic(model) => model.nodes.get(index),
            Self::Mounted(model) => model.node(index),
        }
    }

    pub(super) fn element(&self, index: usize) -> Option<&Elements> {
        match self {
            Self::Dynamic(model) => model.elements.get(index),
            Self::Mounted(model) => model.element(index),
        }
    }

    fn support_node_id(&self, index: usize) -> Option<&str> {
        match self {
            Self::Dynamic(model) => model.supports.get(index).map(|support| support.node_id.as_str()),
            Self::Mounted(model) => model.support(index).map(|support| support.node_id.as_str()),
        }
    }

    fn support_fixed_len(&self, index: usize) -> Option<usize> {
        match self {
            Self::Dynamic(model) => model.supports.get(index).map(|support| support.fixed.len()),
            Self::Mounted(model) => model.support(index).map(|support| support.fixed_len),
        }
    }

    fn support_fixed(&self, support: usize, index: usize) -> Option<Dof> {
        match self {
            Self::Dynamic(model) => model.supports.get(support)?.fixed.get(index).copied(),
            Self::Mounted(model) => model.support(support)?.fixed.get(index).copied().flatten(),
        }
    }
}

pub struct AssemblyJobConstruction {
    pub(super) model: Option<AssemblyConstructionModel>,
    pub(super) model_close: AnalysisModelCloseCursor,
    pub(super) operation: Operation,
    pub(super) partition_count: usize,
    pub(super) stage: AssemblyConstructionStage,
    node_outer: usize,
    node_inner: usize,
    pub(super) element_cursor: usize,
    reference_cursor: usize,
    reference_node_cursor: usize,
    support_cursor: usize,
    support_node_cursor: usize,
    dof_node_cursor: usize,
    dof_element_cursor: usize,
    dof_reference_cursor: usize,
    dof_emit_cursor: usize,
    pending_dof_owner: Option<(String, Dof)>,
    active_dofs: [bool; 6],
    constraint_support_cursor: usize,
    constraint_dof_cursor: usize,
    constraint_order_cursor: usize,
    pub(super) scalar_cursor: usize,
    maximum_triplets: usize,
    partition_triplet_counts: Vec<usize>,
    partition_reserve_cursor: usize,
    partition_reserve_lane: u8,
    pub(super) plan: AssemblyPlan,
    constrained_old: AssemblyOrder<bool>,
    pub(super) partitions: Vec<AssemblyPartitionBuffer>,
    pub(super) merged_full: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) merged_free: PagedList<SparseTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) full_merge_cursors: Vec<usize>,
    pub(super) free_merge_cursors: Vec<usize>,
    pub(super) job: Option<AssemblyJob<'static>>,
}

impl AssemblyJobConstruction {
    pub fn new_owned(model: Arc<AnalysisModel>, operation: Operation, partition_count: usize) -> Self {
        Self::from_model(AssemblyConstructionModel::Dynamic(model), operation, partition_count)
    }

    fn from_model(model: AssemblyConstructionModel, operation: Operation, partition_count: usize) -> Self {
        Self {
            model: Some(model),
            model_close: AnalysisModelCloseCursor::default(),
            operation,
            partition_count,
            stage: AssemblyConstructionStage::ReservePartitionTripletCounts,
            node_outer: 0,
            node_inner: 1,
            element_cursor: 0,
            reference_cursor: 0,
            reference_node_cursor: 0,
            support_cursor: 0,
            support_node_cursor: 0,
            dof_node_cursor: 0,
            dof_element_cursor: 0,
            dof_reference_cursor: 0,
            dof_emit_cursor: 0,
            pending_dof_owner: None,
            active_dofs: [false; 6],
            constraint_support_cursor: 0,
            constraint_dof_cursor: 0,
            constraint_order_cursor: 0,
            scalar_cursor: 0,
            maximum_triplets: 0,
            partition_triplet_counts: Vec::new(),
            partition_reserve_cursor: 0,
            partition_reserve_lane: 0,
            plan: AssemblyPlan { dof_map: DofMap::from_order(Vec::new()), inv_perm: AssemblyOrder::default(), ndof: 0, free_new: AssemblyOrder::default(), compact_of_new: AssemblyOrder::default(), node_slot: HashMap::new() },
            constrained_old: AssemblyOrder::default(),
            partitions: Vec::new(),
            merged_full: PagedList::default(),
            merged_free: PagedList::default(),
            full_merge_cursors: Vec::new(),
            free_merge_cursors: Vec::new(),
            job: None,
        }
    }

    /// 🧱 Retains an already-admitted fixed mounted model without contiguous materialization.
    pub fn new_mounted(model: MountedAnalysisModel, operation: Operation, partition_count: usize) -> Self {
        Self::from_model(AssemblyConstructionModel::Mounted(model), operation, partition_count)
    }

    /// 🎟️ Funds one original order page before changing its assembly cursor.
    pub fn step_one_funded(&mut self, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), FemError> {
        fn reserve<T>(owner: &mut AssemblyOrder<T>, target: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, FemError> {
            owner.reserve_capacity_one_funded(target, RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes.min(MOUNTED_OWNER_PAGE_BYTES), ..grant }).map_err(|_| FemError::ResourceAdmission)
        }
        let maximum_dofs = self.model.as_ref().map_or(Some(0), |model| model.nodes_len().checked_mul(6)).ok_or(FemError::Singular)?;
        let (target, capacity, receipt) = match self.stage {
            AssemblyConstructionStage::ReserveDofs => {
                let receipt = reserve(&mut self.plan.dof_map.order, maximum_dofs, grant)?;
                (maximum_dofs, self.plan.dof_map.order.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveDofIndex => {
                let target = DofMap::slot_count(maximum_dofs).ok_or(FemError::Singular)?;
                let receipt = reserve(&mut self.plan.dof_map.slots, target, grant)?;
                (target, self.plan.dof_map.slots.capacity(), receipt)
            }
            AssemblyConstructionStage::ReservePermutation => {
                let target = self.plan.dof_map.len();
                let receipt = reserve(&mut self.plan.inv_perm, target, grant)?;
                (target, self.plan.inv_perm.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveConstraints => {
                let receipt = reserve(&mut self.constrained_old, self.plan.ndof, grant)?;
                (self.plan.ndof, self.constrained_old.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveFree => {
                let receipt = reserve(&mut self.plan.free_new, self.plan.ndof, grant)?;
                (self.plan.ndof, self.plan.free_new.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveCompact => {
                let receipt = reserve(&mut self.plan.compact_of_new, self.plan.ndof, grant)?;
                (self.plan.ndof, self.plan.compact_of_new.capacity(), receipt)
            }
            AssemblyConstructionStage::BuildPartitions if self.partitions.len() == self.partition_count && self.partition_reserve_cursor < self.partitions.len() => {
                let target = self.partition_triplet_counts[self.partition_reserve_cursor];
                let partition = &mut self.partitions[self.partition_reserve_cursor];
                let owner = if self.partition_reserve_lane == 0 { &mut partition.full } else { &mut partition.free };
                let receipt = reserve(owner, target, grant)?;
                (target, owner.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveMergedFull => {
                let receipt = reserve(&mut self.merged_full, self.maximum_triplets, grant)?;
                (self.maximum_triplets, self.merged_full.capacity(), receipt)
            }
            AssemblyConstructionStage::ReserveMergedFree => {
                let receipt = reserve(&mut self.merged_free, self.maximum_triplets, grant)?;
                (self.maximum_triplets, self.merged_free.capacity(), receipt)
            }
            _ => return self.step_one().map(|complete| (complete, RetainedCloneProgress::default())),
        };
        if receipt != RetainedCloneProgress::default() || capacity < target { return Ok((false, receipt)); }
        self.step_one().map(|complete| (complete, receipt))
    }

    pub fn step_one(&mut self) -> Result<bool, FemError> {
        match self.stage {
            AssemblyConstructionStage::ReservePartitionTripletCounts => {
                if self.partition_count == 0 {
                    return Err(FemError::EmptyModel);
                }
                if !reserve_exact_owner_page(&mut self.partition_triplet_counts, self.partition_count) {
                    return Err(FemError::ResourceAdmission);
                }
                self.stage = AssemblyConstructionStage::InitializePartitionTripletCounts;
            }
            AssemblyConstructionStage::InitializePartitionTripletCounts => {
                if self.partition_triplet_counts.len() < self.partition_count {
                    self.partition_triplet_counts.push(0);
                } else {
                    self.stage = AssemblyConstructionStage::ReserveDofs;
                }
            }
            AssemblyConstructionStage::ReserveDofs => {
                if self.model.as_ref().is_none_or(|model| model.nodes_len() == 0) {
                    return Err(FemError::EmptyModel);
                }
                let maximum_dofs = self.model.as_ref().ok_or(FemError::EmptyModel)?.nodes_len().checked_mul(6).ok_or(FemError::Singular)?;
                self.plan.dof_map.order.reserve_capacity_one(maximum_dofs, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.plan.dof_map.order.capacity() < maximum_dofs { return Ok(false); }
                self.stage = AssemblyConstructionStage::ReserveDofIndex;
            }
            AssemblyConstructionStage::ReserveDofIndex => {
                let maximum_dofs = self.model.as_ref().ok_or(FemError::EmptyModel)?.nodes_len().checked_mul(6).ok_or(FemError::Singular)?;
                let slot_count = DofMap::slot_count(maximum_dofs).ok_or(FemError::Singular)?;
                self.plan.dof_map.slots.reserve_capacity_one(slot_count, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.plan.dof_map.slots.capacity() < slot_count { return Ok(false); }
                self.stage = AssemblyConstructionStage::InitializeDofIndex;
            }
            AssemblyConstructionStage::InitializeDofIndex => {
                let maximum_dofs = self.model.as_ref().ok_or(FemError::EmptyModel)?.nodes_len().checked_mul(6).ok_or(FemError::Singular)?;
                if self.plan.dof_map.slots.len() < DofMap::slot_count(maximum_dofs).ok_or(FemError::Singular)? {
                    self.plan.dof_map.slots.push_reserved(None).map_err(|_| FemError::Singular)?;
                    return Ok(false);
                }
                self.stage = AssemblyConstructionStage::ValidateNodePairs;
            }
            AssemblyConstructionStage::ValidateNodePairs => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.node_outer >= model.nodes_len() {
                    self.stage = AssemblyConstructionStage::ValidateElementReferences;
                } else if self.node_inner >= model.nodes_len() {
                    self.node_outer += 1;
                    self.node_inner = self.node_outer + 1;
                } else {
                    let outer = model.node(self.node_outer).ok_or(FemError::EmptyModel)?;
                    let inner = model.node(self.node_inner).ok_or(FemError::EmptyModel)?;
                    if outer.id == inner.id {
                        return Err(FemError::DuplicateNodeId(outer.id.clone()));
                    }
                    self.node_inner += 1;
                }
            }
            AssemblyConstructionStage::ValidateElementReferences => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.element_cursor >= model.elements_len() {
                    self.stage = AssemblyConstructionStage::ValidateSupportReferences;
                } else if self.reference_cursor >= model.element(self.element_cursor).and_then(Elements::mounted_node_id_count).ok_or(FemError::Singular)? {
                    let element = model.element(self.element_cursor).ok_or(FemError::Singular)?;
                    let side = element.mounted_node_id_count().and_then(|nodes| nodes.checked_mul(element.dofs_per_node().len())).ok_or(FemError::Singular)?;
                    let triplets = side.checked_mul(side).ok_or(FemError::Singular)?;
                    self.maximum_triplets = self.maximum_triplets.checked_add(triplets).ok_or(FemError::Singular)?;
                    let partition = self.element_cursor % self.partition_count;
                    self.partition_triplet_counts[partition] = self.partition_triplet_counts[partition].checked_add(triplets).ok_or(FemError::Singular)?;
                    self.element_cursor += 1;
                    self.reference_cursor = 0;
                    self.reference_node_cursor = 0;
                } else if self.reference_node_cursor >= model.nodes_len() {
                    return Err(FemError::DanglingNodeRef(model.element(self.element_cursor).and_then(|element| element.mounted_node_id(self.reference_cursor)).ok_or(FemError::Singular)?.to_owned()));
                } else if model.node(self.reference_node_cursor).ok_or(FemError::EmptyModel)?.id == model.element(self.element_cursor).and_then(|element| element.mounted_node_id(self.reference_cursor)).ok_or(FemError::Singular)? {
                    self.reference_cursor += 1;
                    self.reference_node_cursor = 0;
                } else {
                    self.reference_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::ValidateSupportReferences => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.support_cursor >= model.supports_len() {
                    self.element_cursor = 0;
                    self.stage = AssemblyConstructionStage::DiscoverDofs;
                } else if self.support_node_cursor >= model.nodes_len() {
                    return Err(FemError::DanglingNodeRef(model.support_node_id(self.support_cursor).ok_or(FemError::Singular)?.to_owned()));
                } else if model.node(self.support_node_cursor).ok_or(FemError::EmptyModel)?.id == model.support_node_id(self.support_cursor).ok_or(FemError::Singular)? {
                    self.support_cursor += 1;
                    self.support_node_cursor = 0;
                } else {
                    self.support_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::DiscoverDofs => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.dof_node_cursor >= model.nodes_len() {
                    self.stage = AssemblyConstructionStage::ReservePermutation;
                } else if self.dof_element_cursor >= model.elements_len() {
                    self.dof_emit_cursor = 0;
                    self.stage = AssemblyConstructionStage::EmitDofs;
                } else if self.dof_reference_cursor >= model.element(self.dof_element_cursor).and_then(Elements::mounted_node_id_count).ok_or(FemError::Singular)? {
                    self.dof_element_cursor += 1;
                    self.dof_reference_cursor = 0;
                } else {
                    let element = model.element(self.dof_element_cursor).ok_or(FemError::Singular)?;
                    if element.mounted_node_id(self.dof_reference_cursor).ok_or(FemError::Singular)? == model.node(self.dof_node_cursor).ok_or(FemError::EmptyModel)?.id {
                        for dof in element.dofs_per_node() {
                            self.active_dofs[dof.index()] = true;
                        }
                    }
                    self.dof_reference_cursor += 1;
                }
            }
            AssemblyConstructionStage::EmitDofs => {
                if self.dof_emit_cursor >= self.active_dofs.len() {
                    self.dof_node_cursor += 1;
                    self.dof_element_cursor = 0;
                    self.active_dofs = [false; 6];
                    self.stage = AssemblyConstructionStage::DiscoverDofs;
                } else {
                    if self.active_dofs[self.dof_emit_cursor] {
                        let dof = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz][self.dof_emit_cursor];
                        let node_id = self.model.as_ref().and_then(|model| model.node(self.dof_node_cursor)).ok_or(FemError::EmptyModel)?.id.clone();
                        self.pending_dof_owner = Some((node_id, dof));
                        self.stage = AssemblyConstructionStage::CommitDofOwner;
                        return Ok(false);
                    }
                    self.dof_emit_cursor += 1;
                }
            }
            AssemblyConstructionStage::CommitDofOwner => {
                let owner = self.pending_dof_owner.as_ref().ok_or(FemError::Singular)?;
                if owner.0.capacity() > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                self.plan.dof_map.push(self.pending_dof_owner.take().ok_or(FemError::Singular)?);
                self.dof_emit_cursor += 1;
                self.stage = AssemblyConstructionStage::EmitDofs;
            }
            AssemblyConstructionStage::ReservePermutation => {
                self.plan.ndof = self.plan.dof_map.len();
                self.plan.inv_perm.reserve_capacity_one(self.plan.ndof, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.plan.inv_perm.capacity() < self.plan.ndof { return Ok(false); }
                self.stage = AssemblyConstructionStage::BuildPermutation;
            }
            AssemblyConstructionStage::BuildPermutation => {
                if self.plan.inv_perm.len() < self.plan.ndof {
                    self.plan.inv_perm.push(self.plan.inv_perm.len());
                } else {
                    self.stage = AssemblyConstructionStage::ReserveConstraints;
                }
            }
            AssemblyConstructionStage::ReserveConstraints => {
                self.constrained_old.reserve_capacity_one(self.plan.ndof, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.constrained_old.capacity() < self.plan.ndof { return Ok(false); }
                self.stage = AssemblyConstructionStage::InitializeConstraints;
            }
            AssemblyConstructionStage::InitializeConstraints => {
                if self.constrained_old.len() < self.plan.ndof {
                    self.constrained_old.push(false);
                } else {
                    self.support_cursor = 0;
                    self.stage = AssemblyConstructionStage::MarkConstraints;
                }
            }
            AssemblyConstructionStage::MarkConstraints => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.constraint_support_cursor >= model.supports_len() {
                    self.stage = AssemblyConstructionStage::ReserveFree;
                } else if self.constraint_dof_cursor >= model.support_fixed_len(self.constraint_support_cursor).ok_or(FemError::Singular)? {
                    self.constraint_support_cursor += 1;
                    self.constraint_dof_cursor = 0;
                    self.constraint_order_cursor = 0;
                } else if self.constraint_order_cursor >= self.plan.dof_map.order.len() {
                    self.constraint_dof_cursor += 1;
                    self.constraint_order_cursor = 0;
                } else {
                    let (node_id, dof) = &self.plan.dof_map.order[self.constraint_order_cursor];
                    if node_id == model.support_node_id(self.constraint_support_cursor).ok_or(FemError::Singular)? && *dof == model.support_fixed(self.constraint_support_cursor, self.constraint_dof_cursor).ok_or(FemError::Singular)? {
                        self.constrained_old[self.constraint_order_cursor] = true;
                        self.constraint_dof_cursor += 1;
                        self.constraint_order_cursor = 0;
                    } else {
                        self.constraint_order_cursor += 1;
                    }
                }
            }
            AssemblyConstructionStage::ReserveFree => {
                self.plan.free_new.reserve_capacity_one(self.plan.ndof, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.plan.free_new.capacity() < self.plan.ndof { return Ok(false); }
                self.scalar_cursor = 0;
                self.stage = AssemblyConstructionStage::BuildFree;
            }
            AssemblyConstructionStage::BuildFree => {
                if self.scalar_cursor < self.plan.ndof {
                    if !self.constrained_old[self.scalar_cursor] {
                        self.plan.free_new.push(self.scalar_cursor);
                    }
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReserveCompact;
                }
            }
            AssemblyConstructionStage::ReserveCompact => {
                self.plan.compact_of_new.reserve_capacity_one(self.plan.ndof, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.plan.compact_of_new.capacity() < self.plan.ndof { return Ok(false); }
                self.stage = AssemblyConstructionStage::InitializeCompact;
            }
            AssemblyConstructionStage::InitializeCompact => {
                if self.plan.compact_of_new.len() < self.plan.ndof {
                    self.plan.compact_of_new.push(None);
                } else {
                    self.scalar_cursor = 0;
                    self.stage = AssemblyConstructionStage::BuildCompact;
                }
            }
            AssemblyConstructionStage::BuildCompact => {
                if let Some(new_index) = self.plan.free_new.get(self.scalar_cursor).copied() {
                    self.plan.compact_of_new[new_index] = Some(self.scalar_cursor);
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReservePartitions;
                }
            }
            AssemblyConstructionStage::ReservePartitions => {
                if !reserve_exact_owner_page(&mut self.partitions, self.partition_count) {
                    return Err(FemError::ResourceAdmission);
                }
                self.stage = AssemblyConstructionStage::BuildPartitions;
            }
            AssemblyConstructionStage::BuildPartitions => {
                if self.partitions.len() < self.partition_count {
                    self.partitions.push(AssemblyPartitionBuffer::default());
                } else if self.partition_reserve_cursor < self.partitions.len() {
                    let partition = &mut self.partitions[self.partition_reserve_cursor];
                    let per_partition = self.partition_triplet_counts[self.partition_reserve_cursor];
                    if self.partition_reserve_lane == 0 {
                        partition.full.reserve_capacity_one(per_partition, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                        if partition.full.capacity() < per_partition {
                            return Ok(false);
                        }
                        self.partition_reserve_lane = 1;
                    } else {
                        partition.free.reserve_capacity_one(per_partition, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                        if partition.free.capacity() < per_partition {
                            return Ok(false);
                        }
                        self.partition_reserve_lane = 0;
                        self.partition_reserve_cursor += 1;
                    }
                } else {
                    self.stage = AssemblyConstructionStage::ReserveMergedFull;
                }
            }
            AssemblyConstructionStage::ReserveMergedFull => {
                self.merged_full.reserve_capacity_one(self.maximum_triplets, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.merged_full.capacity() >= self.maximum_triplets {
                    self.stage = AssemblyConstructionStage::ReserveMergedFree;
                }
            }
            AssemblyConstructionStage::ReserveMergedFree => {
                self.merged_free.reserve_capacity_one(self.maximum_triplets, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::ResourceAdmission)?;
                if self.merged_free.capacity() >= self.maximum_triplets {
                    self.scalar_cursor = 0;
                    self.stage = AssemblyConstructionStage::ReserveFullMergeCursors;
                }
            }
            AssemblyConstructionStage::ReserveFullMergeCursors => {
                if self.full_merge_cursors.is_empty() && !self.partition_triplet_counts.is_empty() {
                    self.full_merge_cursors = std::mem::take(&mut self.partition_triplet_counts);
                }
                if self.scalar_cursor < self.full_merge_cursors.len() {
                    self.full_merge_cursors[self.scalar_cursor] = 0;
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReserveFreeMergeCursors;
                }
            }
            AssemblyConstructionStage::ReserveFreeMergeCursors => {
                if !reserve_exact_owner_page(&mut self.free_merge_cursors, self.partition_count) {
                    return Err(FemError::ResourceAdmission);
                }
                self.stage = AssemblyConstructionStage::BuildMergeCursors;
            }
            AssemblyConstructionStage::BuildMergeCursors => {
                if self.full_merge_cursors.len() < self.partition_count {
                    self.full_merge_cursors.push(0);
                } else if self.free_merge_cursors.len() < self.partition_count {
                    self.free_merge_cursors.push(0);
                } else {
                    let model = self.model.take().ok_or(FemError::EmptyModel)?;
                    let total_elements = model.elements_len();
                    let model_signature = self.operation.operation.0 ^ self.operation.base_revision.0.rotate_left(17) ^ self.operation.generation.0.rotate_left(33);
                    let plan = std::mem::replace(&mut self.plan, AssemblyPlan { dof_map: DofMap::from_order(Vec::new()), inv_perm: AssemblyOrder::default(), ndof: 0, free_new: AssemblyOrder::default(), compact_of_new: AssemblyOrder::default(), node_slot: HashMap::new() });
                    self.job = Some(AssemblyJob {
                        state: AssemblyCheckpoint {
                            stage: AssemblyJobStage::ElementTriplets,
                            total_elements,
                            element_cursor: 0,
                            pending_build: None,
                            pending: None,
                            partitions: std::mem::take(&mut self.partitions),
                            full_merge_cursors: std::mem::take(&mut self.full_merge_cursors),
                            free_merge_cursors: std::mem::take(&mut self.free_merge_cursors),
                            merged_full: std::mem::take(&mut self.merged_full),
                            merged_free: std::mem::take(&mut self.merged_free),
                            checkpoint_due: false,
                            preview_due: false,
                            preview_cursor: 0,
                            resume_target: 0,
                            merge_scan_partition: 0,
                            merge_candidate: None,
                        },
                        model: match model {
                            AssemblyConstructionModel::Dynamic(model) => AnalysisModelOwner::Owned(model),
                            AssemblyConstructionModel::Mounted(model) => AnalysisModelOwner::Mounted(model),
                        },
                        operation: self.operation,
                        model_signature,
                        plan,
                        close_lane: 0,
                        model_close: AnalysisModelCloseCursor::default(),
                        desk: NumericalOutcomeDesk::default(),
                    });
                    self.stage = AssemblyConstructionStage::Complete;
                }
            }
            AssemblyConstructionStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    pub fn take_complete(&mut self) -> Option<AssemblyJob<'static>> {
        (self.stage == AssemblyConstructionStage::Complete).then(|| self.job.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(owner) = self.pending_dof_owner.as_mut() {
            if owner.0.capacity() != 0 {
                let bytes = owner.0.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                owner.0 = String::new();
                return (false, 1, bytes);
            }
            self.pending_dof_owner = None;
            return (false, 1, 0);
        }
        if let Some(job) = self.job.as_mut() {
            let (terminal, items, bytes) = job.close_retained_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.job = None;
            return (false, 1, 0);
        }
        match close_paged_owner_step(&mut self.plan.dof_map.slots, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        if let Some(key) = self.plan.dof_map.order.last_mut() {
            if key.0.capacity() != 0 {
                let bytes = key.0.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                key.0 = String::new();
                return (false, 1, bytes);
            }
            self.plan.dof_map.order.pop();
            return (false, 1, 0);
        }
        match close_paged_owner_step(&mut self.plan.dof_map.order, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.plan.inv_perm, &mut self.plan.free_new] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_paged_owner_step(&mut self.plan.compact_of_new, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        match close_paged_owner_step(&mut self.constrained_old, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        match close_vec_owner_step(&mut self.partition_triplet_counts, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.merged_full, &mut self.merged_free] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.full_merge_cursors, &mut self.free_merge_cursors] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        if let Some(partition) = self.partitions.last_mut() {
            match close_paged_owner_step(&mut partition.full, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            match close_paged_owner_step(&mut partition.free, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            self.partitions.pop();
            return (false, 1, 0);
        }
        match close_vec_owner_step(&mut self.partitions, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        if let Some(model) = self.model.as_mut() {
            let (terminal, items, bytes) = match model {
                AssemblyConstructionModel::Dynamic(model) => close_analysis_model_step(model, &mut self.model_close, maximum_bytes),
                AssemblyConstructionModel::Mounted(model) => model.close_step(maximum_bytes),
            };
            if !terminal {
                return (false, items, bytes);
            }
            self.model = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}
// #endregion 🔖️Assembly
