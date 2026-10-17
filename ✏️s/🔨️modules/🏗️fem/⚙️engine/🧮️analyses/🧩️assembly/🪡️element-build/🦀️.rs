//! 🪡️ The checkpointed assembly job: element build, partition merge and retained close.

use super::*;

impl AssemblyJob<'static> {
    /// 🧭 Resolves one old node/DOF identity to its retained full and free equation indices.
    pub fn visual_equation_indices(&self, node_id: &str, dof: Dof) -> Option<(usize, Option<usize>)> {
        let old = self.plan.dof_map.get(node_id, dof)?;
        let new = *self.plan.inv_perm.get(old)?;
        Some((new, self.plan.compact_of_new.get(new).copied().flatten()))
    }

    pub fn visual_free_order(&self) -> usize {
        self.plan.free_new.len()
    }
}

pub(super) struct UnfactoredSystem {
    pub(super) plan: AssemblyPlan,
    pub(super) k_full_coo: Coo,
    pub(super) k_ff_coo: Coo,
}

/// 🧮️ Persistent per-element assembly with worker-local triplets and a stable k-way merge.
pub(super) enum AnalysisModelOwner<'model> {
    Borrowed(&'model AnalysisModel),
    Owned(Arc<AnalysisModel>),
    Mounted(MountedAnalysisModel),
}

impl AnalysisModelOwner<'_> {
    fn dynamic(&self) -> Option<&AnalysisModel> {
        match self {
            Self::Borrowed(model) => Some(model),
            Self::Owned(model) => Some(model.as_ref()),
            Self::Mounted(_) => None,
        }
    }

    pub(super) fn elements_len(&self) -> usize {
        match self {
            Self::Borrowed(model) => model.elements.len(),
            Self::Owned(model) => model.elements.len(),
            Self::Mounted(model) => model.elements_len(),
        }
    }

    pub(super) fn node(&self, index: usize) -> Option<&Node> {
        match self {
            Self::Borrowed(model) => model.nodes.get(index),
            Self::Owned(model) => model.nodes.get(index),
            Self::Mounted(model) => model.node(index),
        }
    }

    pub(super) fn element(&self, index: usize) -> Option<&Elements> {
        match self {
            Self::Borrowed(model) => model.elements.get(index),
            Self::Owned(model) => model.elements.get(index),
            Self::Mounted(model) => model.element(index),
        }
    }
}

pub struct AssemblyJob<'model> {
    pub(super) model: AnalysisModelOwner<'model>,
    pub(super) operation: Operation,
    pub(super) model_signature: u64,
    pub(super) plan: AssemblyPlan,
    pub(super) state: AssemblyCheckpoint,
    pub(super) close_lane: u8,
    pub(super) model_close: AnalysisModelCloseCursor,
    pub(super) desk: NumericalOutcomeDesk,
}

impl<'model> AssemblyJob<'model> {
    pub fn new(model: &'model AnalysisModel, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        Self::from_owner(AnalysisModelOwner::Borrowed(model), operation, partition_count)
    }

    pub(super) fn from_owner(model: AnalysisModelOwner<'model>, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        assert!(partition_count > 0, "assembly requires at least one worker-local partition");
        let dynamic = model.dynamic().ok_or(FemError::EmptyModel)?;
        let plan = AssemblyPlan::prepare(dynamic)?;
        let model_signature = assembly_model_signature(dynamic);
        let total_elements = model.elements_len();
        let mut partition_triplet_counts = vec![0usize; partition_count];
        let maximum_triplets = dynamic.elements.iter().enumerate().try_fold(0usize, |total, (index, element)| {
            let side = element.node_ids().len().checked_mul(element.dofs_per_node().len()).ok_or(FemError::Singular)?;
            let triplets = side.checked_mul(side).ok_or(FemError::Singular)?;
            let partition = index % partition_count;
            partition_triplet_counts[partition] = partition_triplet_counts[partition].checked_add(triplets).ok_or(FemError::Singular)?;
            total.checked_add(triplets).ok_or(FemError::Singular)
        })?;
        let mut partitions = Vec::with_capacity(partition_count);
        for capacity in partition_triplet_counts {
            partitions.push(AssemblyPartitionBuffer { full: cold_paged_owner(capacity)?, free: cold_paged_owner(capacity)? });
        }
        Ok(Self {
            model,
            operation,
            model_signature,
            plan,
            state: AssemblyCheckpoint {
                stage: AssemblyJobStage::ElementTriplets,
                total_elements,
                element_cursor: 0,
                pending_build: None,
                pending: None,
                partitions,
                full_merge_cursors: vec![0; partition_count],
                free_merge_cursors: vec![0; partition_count],
                merged_full: cold_paged_owner(maximum_triplets)?,
                merged_free: cold_paged_owner(maximum_triplets)?,
                checkpoint_due: false,
                preview_due: false,
                preview_cursor: 0,
                resume_target: 0,
                merge_scan_partition: 0,
                merge_candidate: None,
            },
            close_lane: 0,
            model_close: AnalysisModelCloseCursor::default(),
            desk: NumericalOutcomeDesk::default(),
        })
    }

    pub fn from_checkpoint(model: &'model AnalysisModel, operation: Operation, bytes: &[u8]) -> Result<Self, String> {
        let checkpoint: AssemblyResumeCheckpoint = decode_value(bytes)?;
        if checkpoint.version != 1 || checkpoint.partition_count == 0 || checkpoint.total_elements != model.elements.len() || checkpoint.completed_elements > checkpoint.total_elements || checkpoint.model_signature != assembly_model_signature(model) {
            return Err("assembly checkpoint does not match the supplied model".to_string());
        }
        let mut job = Self::new(model, operation, checkpoint.partition_count).map_err(|error| error.to_string())?;
        job.state.resume_target = checkpoint.completed_elements;
        job.state.preview_cursor = checkpoint.completed_elements;
        Ok(job)
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        let checkpoint = AssemblyResumeCheckpoint {
            version: 1,
            model_signature: self.model_signature,
            total_elements: self.state.total_elements,
            completed_elements: self.state.element_cursor.max(self.state.resume_target),
            partition_count: self.state.partitions.len(),
        };
        encode_value(&checkpoint)
    }

    pub fn preview(&self) -> AssemblyPreview {
        AssemblyPreview {
            stage: self.state.stage,
            completed_elements: self.state.element_cursor,
            total_elements: self.state.total_elements,
            full_triplets: self.state.partitions.iter().map(|partition| partition.full.len()).sum(),
            free_triplets: self.state.partitions.iter().map(|partition| partition.free.len()).sum(),
            assembled_element_ids: (self.state.preview_cursor..self.state.element_cursor).filter_map(|index| self.model.element(index)).map(|element| element.id().to_string()).collect(),
        }
    }

    /// 🧹️ Retires exactly one nested assembly owner per call. The model root is deliberately
    /// retained until the caller observes `true`; mounted sessions keep a separate exact model root
    /// while dropping the resulting shallow assembly shell.
    pub fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let released = match self.close_lane {
                0 => {
                    if let Some(pending) = self.state.pending_build.as_mut() {
                        match close_vec_owner_step(&mut pending.stiffness, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.positions, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.indices_new, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.pending_build = None;
                        return (false, 1, 0);
                    }
                    if let Some(pending) = self.state.pending.as_mut() {
                        match close_vec_owner_step(&mut pending.stiffness, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.indices_new, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.positions, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.pending = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                1 => {
                    if let Some(partition) = self.state.partitions.last_mut() {
                        match close_paged_owner_step(&mut partition.full, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_paged_owner_step(&mut partition.free, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.partitions.pop();
                        (1, 0)
                    } else {
                        let bytes = self.state.partitions.capacity() * size_of::<AssemblyPartitionBuffer>();
                        if bytes != 0 {
                            if bytes > maximum_bytes {
                                return (false, 0, 0);
                            }
                            self.state.partitions = Vec::new();
                            return (false, 1, bytes);
                        }
                        self.close_lane += 1;
                        continue;
                    }
                }
                2 => match close_vec_owner_step(&mut self.state.full_merge_cursors, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                3 => match close_vec_owner_step(&mut self.state.free_merge_cursors, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                4 => match close_paged_owner_step(&mut self.state.merged_full, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                5 => match close_paged_owner_step(&mut self.state.merged_free, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                6 => {
                    self.close_lane += 1;
                    continue;
                }
                7 => match self.plan.dof_map.order.last_mut() {
                    Some((id, _)) if id.capacity() != 0 => {
                        let bytes = id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *id = String::new();
                        (1, bytes)
                    }
                    Some(_) => {
                        self.plan.dof_map.order.pop();
                        (1, 0)
                    }
                    None => match close_vec_owner_step(&mut self.plan.dof_map.order, maximum_bytes) {
                        Ok(Some(step)) => step,
                        Err(()) => return (false, 0, 0),
                        Ok(None) => {
                            self.close_lane += 1;
                            continue;
                        }
                    },
                },
                8 => match close_vec_owner_step(&mut self.plan.inv_perm, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                9 => match close_vec_owner_step(&mut self.plan.free_new, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                10 => match close_vec_owner_step(&mut self.plan.compact_of_new, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                11 => match &mut self.model {
                    AnalysisModelOwner::Borrowed(_) => {
                        self.close_lane += 1;
                        continue;
                    }
                    AnalysisModelOwner::Owned(model) => {
                        let (terminal, items, bytes) = close_analysis_model_step(model, &mut self.model_close, maximum_bytes);
                        if terminal {
                            self.close_lane += 1;
                            continue;
                        }
                        return (false, items, bytes);
                    }
                    AnalysisModelOwner::Mounted(model) => {
                        let (terminal, items, bytes) = model.close_step(maximum_bytes);
                        if terminal {
                            self.close_lane += 1;
                            continue;
                        }
                        return (false, items, bytes);
                    }
                },
                _ => return (true, 0, 0),
            };
            return (false, released.0, released.1);
        }
    }

    pub(super) fn advance_element_build(&mut self) -> Result<bool, FemError> {
        if self.state.pending_build.is_none() {
            let element_index = self.state.element_cursor;
            let element = self.model.element(element_index).ok_or(FemError::Singular)?;
            let node_count = element.mounted_node_id_count().ok_or(FemError::Singular)?;
            let dof_count = element.dofs_per_node().len();
            node_count.checked_mul(dof_count).ok_or(FemError::Singular)?;
            self.state.pending_build = Some(PendingElementBuild {
                element_index,
                node_count,
                dof_count,
                scalar_cursor: 0,
                lookup_cursor: 0,
                lookup_match: None,
                stage: PendingElementBuildStage::ReserveIndices,
                indices_new: Vec::new(),
                positions: Vec::new(),
                stiffness: Vec::new(),
                stiffness_dimensions: [0; 2],
                stiffness_observed_bytes: 0,
                stiffness_credit_reserved: false,
                stiffness_admitted: false,
            });
            return Ok(false);
        }
        let build = self.state.pending_build.as_mut().ok_or(FemError::Singular)?;
        let element = self.model.element(build.element_index).ok_or(FemError::Singular)?;
        match build.stage {
            PendingElementBuildStage::ReserveIndices => {
                let side = build.node_count.checked_mul(build.dof_count).ok_or(FemError::Singular)?;
                if !reserve_exact_owner_page(&mut build.indices_new, side) {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::Indices;
            }
            PendingElementBuildStage::Indices => {
                let side = build.node_count.checked_mul(build.dof_count).ok_or(FemError::Singular)?;
                if build.scalar_cursor < side {
                    let node_index = build.scalar_cursor / build.dof_count;
                    let dof_index = build.scalar_cursor % build.dof_count;
                    let node_id = element.mounted_node_id(node_index).ok_or(FemError::Singular)?;
                    let Some((current, current_dof)) = self.plan.dof_map.order.get(build.lookup_cursor) else { return Err(FemError::Singular) };
                    if current == node_id && *current_dof == element.dofs_per_node()[dof_index] {
                        build.lookup_match = Some(build.lookup_cursor);
                        build.stage = PendingElementBuildStage::PublishIndex;
                    } else {
                        build.lookup_cursor += 1;
                    }
                } else {
                    build.scalar_cursor = 0;
                    build.lookup_cursor = 0;
                    build.stage = PendingElementBuildStage::ReservePositions;
                }
            }
            PendingElementBuildStage::PublishIndex => {
                let old = build.lookup_match.take().ok_or(FemError::Singular)?;
                build.indices_new.push(self.plan.inv_perm[old]);
                build.scalar_cursor += 1;
                build.lookup_cursor = 0;
                build.stage = PendingElementBuildStage::Indices;
            }
            PendingElementBuildStage::ReservePositions => {
                if !reserve_exact_owner_page(&mut build.positions, build.node_count) {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::Positions;
            }
            PendingElementBuildStage::Positions => {
                if build.scalar_cursor < build.node_count {
                    let node_id = element.mounted_node_id(build.scalar_cursor).ok_or(FemError::Singular)?;
                    let Some(node) = self.model.node(build.lookup_cursor) else { return Err(FemError::Singular) };
                    if node.id == node_id {
                        build.lookup_match = Some(build.lookup_cursor);
                        build.stage = PendingElementBuildStage::PublishPosition;
                    } else {
                        build.lookup_cursor += 1;
                    }
                } else {
                    build.stage = PendingElementBuildStage::ReserveStiffnessCredit;
                }
            }
            PendingElementBuildStage::PublishPosition => {
                let node = build.lookup_match.take().and_then(|index| self.model.node(index)).ok_or(FemError::Singular)?;
                build.positions.push(node.pos);
                build.scalar_cursor += 1;
                build.lookup_cursor = 0;
                build.stage = PendingElementBuildStage::Positions;
            }
            PendingElementBuildStage::ReserveStiffnessCredit => {
                let side = build.indices_new.len();
                let requested_bytes = side.checked_mul(side).and_then(|cells| cells.checked_mul(size_of::<f64>())).ok_or(FemError::Singular)?;
                if requested_bytes > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                build.stiffness_credit_reserved = true;
                build.stage = PendingElementBuildStage::AllocateStiffness;
            }
            PendingElementBuildStage::AllocateStiffness => {
                let side = build.indices_new.len();
                if !reserve_exact_owner_page(&mut build.stiffness, side.saturating_mul(side)) {
                    return Err(FemError::Singular);
                }
                build.stiffness_dimensions = [side, side];
                build.scalar_cursor = 0;
                build.stage = PendingElementBuildStage::ReferenceQuadraturePoint;
            }
            PendingElementBuildStage::ReferenceQuadraturePoint => {
                build.stage = PendingElementBuildStage::ShapeFunctionDerivativeScalar;
            }
            PendingElementBuildStage::ShapeFunctionDerivativeScalar => {
                build.stage = PendingElementBuildStage::JacobianCell;
            }
            PendingElementBuildStage::JacobianCell => {
                build.stage = PendingElementBuildStage::DeterminantInverseCell;
            }
            PendingElementBuildStage::DeterminantInverseCell => {
                build.stage = PendingElementBuildStage::StrainDisplacementCell;
            }
            PendingElementBuildStage::StrainDisplacementCell => {
                build.stage = PendingElementBuildStage::ConstitutiveCell;
            }
            PendingElementBuildStage::ConstitutiveCell => {
                build.stage = PendingElementBuildStage::LocalStiffnessMultiplyCell;
            }
            PendingElementBuildStage::LocalStiffnessMultiplyCell => {
                let side = build.indices_new.len();
                if build.scalar_cursor < side.saturating_mul(side) {
                    let context = ElementContext { positions: std::mem::take(&mut build.positions) };
                    let row = build.scalar_cursor / side;
                    let column = build.scalar_cursor % side;
                    let value = element.mounted_stiffness_cell(&context, row, column).ok_or(FemError::Singular)?;
                    build.positions = context.positions;
                    build.stiffness.push(value);
                    build.scalar_cursor += 1;
                } else {
                    build.stage = PendingElementBuildStage::BodyTractionLoadCell;
                }
            }
            PendingElementBuildStage::BodyTractionLoadCell => {
                build.stage = PendingElementBuildStage::LocalToGlobalTripletCell;
            }
            PendingElementBuildStage::LocalToGlobalTripletCell => {
                build.stage = PendingElementBuildStage::ObserveStiffnessBacking;
            }
            PendingElementBuildStage::ObserveStiffnessBacking => {
                let side = build.indices_new.len();
                let Some(observed_bytes) = build.stiffness.capacity().checked_mul(size_of::<f64>()) else { return Err(FemError::Singular) };
                build.stiffness_observed_bytes = observed_bytes;
                if !build.stiffness_credit_reserved || build.stiffness_dimensions != [side, side] || observed_bytes > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::AdmitStiffnessBacking;
            }
            PendingElementBuildStage::AdmitStiffnessBacking => {
                if build.stiffness_observed_bytes == 0 && !build.stiffness.is_empty() {
                    return Err(FemError::Singular);
                }
                build.stiffness_admitted = true;
                build.stage = PendingElementBuildStage::Complete;
            }
            PendingElementBuildStage::Complete => {
                let build = self.state.pending_build.take().ok_or(FemError::Singular)?;
                if !build.stiffness_admitted {
                    self.state.pending_build = Some(build);
                    return Err(FemError::Singular);
                }
                let side = build.indices_new.len();
                self.state.pending = Some(PendingElementAssembly { element_index: build.element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new: build.indices_new, positions: build.positions, stiffness: build.stiffness });
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn begin_borrowed_element(&mut self) -> Result<(), FemError> {
        let element_index = self.state.element_cursor;
        let dynamic = self.model.dynamic().ok_or(FemError::Singular)?;
        let element = dynamic.elements.get(element_index).ok_or(FemError::Singular)?;
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices_old = element_global_indices(&self.plan.dof_map, &node_ids, dofs).ok_or(FemError::Singular)?;
        let indices_new = indices_old.iter().map(|&old| self.plan.inv_perm[old]).collect::<Vec<_>>();
        let context = ElementContext { positions: positions_of(&self.plan.node_slot, &dynamic.nodes, &node_ids) };
        let stiffness = element.stiffness_global(&context);
        let side = indices_new.len();
        self.state.pending = Some(PendingElementAssembly { element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new, positions: context.positions, stiffness: stiffness.data });
        Ok(())
    }

    pub(super) fn assemble_cell(&mut self) -> Result<(), FemError> {
        let pending = self.state.pending.as_mut().expect("pending element exists");
        if pending.side == 0 {
            pending.complete = true;
            self.state.element_cursor += 1;
            return Ok(());
        }
        let local_row = pending.cell_cursor / pending.side;
        let local_col = pending.cell_cursor % pending.side;
        let value = pending.stiffness[pending.cell_cursor];
        if value != 0.0 {
            let new_row = pending.indices_new[local_row];
            let new_col = pending.indices_new[local_col];
            let sequence = ((pending.element_index as u64) << 32) | pending.cell_cursor as u64;
            let partition = pending.element_index % self.state.partitions.len();
            let full = AssemblyTriplet { sequence, row: new_row as u32, col: new_col as u32, value };
            let free =
                if let (Some(compact_row), Some(compact_col)) = (self.plan.compact_of_new[new_row], self.plan.compact_of_new[new_col]) { Some(AssemblyTriplet { sequence, row: compact_row as u32, col: compact_col as u32, value }) } else { None };
            let owner = &mut self.state.partitions[partition];
            if !owner.full.has_reserved_slot() || free.is_some() && !owner.free.has_reserved_slot() {
                return Err(FemError::Singular);
            }
            owner.full.push_reserved(full).map_err(|_| FemError::Singular)?;
            if let Some(free) = free {
                owner.free.push_reserved(free).map_err(|_| FemError::Singular)?;
            }
        }
        pending.cell_cursor += 1;
        if pending.cell_cursor == pending.side * pending.side {
            self.state.element_cursor += 1;
            pending.complete = true;
        }
        Ok(())
    }

    pub(super) fn reclaim_element_owner(&mut self) -> bool {
        let Some(pending) = self.state.pending.as_mut().filter(|pending| pending.complete) else { return false };
        match pending.reclaim_lane {
            0 => {
                pending.stiffness = Vec::new();
                pending.reclaim_lane = 1;
            }
            1 => {
                pending.indices_new = Vec::new();
                pending.reclaim_lane = 2;
            }
            2 => {
                pending.positions = Vec::new();
                pending.reclaim_lane = 3;
            }
            _ => {
                self.state.pending = None;
                if self.state.resume_target > self.state.element_cursor {
                    return true;
                }
                self.state.resume_target = 0;
                self.state.preview_due = true;
                self.state.checkpoint_due = self.state.element_cursor.is_multiple_of(16) || self.state.element_cursor == self.state.total_elements;
            }
        }
        true
    }

    pub(super) fn advance_partition_merge(&mut self, full: bool) -> Result<Option<bool>, FemError> {
        if self.state.merge_scan_partition < self.state.partitions.len() {
            let partition_index = self.state.merge_scan_partition;
            let partition = &self.state.partitions[partition_index];
            let cursor = if full { self.state.full_merge_cursors[partition_index] } else { self.state.free_merge_cursors[partition_index] };
            let entry = if full { partition.full.get(cursor) } else { partition.free.get(cursor) }.copied();
            if let Some(entry) = entry {
                if self.state.merge_candidate.is_none_or(|(_, candidate)| entry.sequence < candidate.sequence) {
                    self.state.merge_candidate = Some((partition_index, entry));
                }
            }
            self.state.merge_scan_partition += 1;
            return Ok(Some(false));
        }
        let Some((partition_index, entry)) = self.state.merge_candidate else {
            return Ok(None);
        };
        let placement = if full { self.state.merged_full.push_reserved(entry) } else { self.state.merged_free.push_reserved(entry) };
        if let Err(entry) = placement {
            self.state.merge_candidate = Some((partition_index, entry));
            return Err(FemError::Singular);
        }
        if full {
            self.state.full_merge_cursors[partition_index] += 1;
        } else {
            self.state.free_merge_cursors[partition_index] += 1;
        }
        self.state.merge_candidate = None;
        self.state.merge_scan_partition = 0;
        Ok(Some(true))
    }

    pub(super) fn finish(self) -> Option<UnfactoredSystem> {
        if self.state.stage != AssemblyJobStage::Complete {
            return None;
        }
        let mut k_full_coo = Coo::new(self.plan.ndof);
        for entry in self.state.merged_full.iter() {
            k_full_coo.add(entry.row as usize, entry.col as usize, entry.value);
        }
        let mut k_ff_coo = Coo::new(self.plan.free_new.len());
        for entry in self.state.merged_free.iter() {
            k_ff_coo.add(entry.row as usize, entry.col as usize, entry.value);
        }
        Some(UnfactoredSystem { plan: self.plan, k_full_coo, k_ff_coo })
    }

    /// 🧵️ Transfers the completed full stiffness matrix into a retained iterative child.
    /// `None` is the exact false-terminal witness; no partial matrix escapes before assembly completes.
    pub fn into_full_matrix(self) -> Option<Csr> {
        self.finish().map(|system| system.k_full_coo.to_csr())
    }
}
