//! 🏭️ Patch apply producer: the apply step and validate frame and the resumable `UiPatchApplyProducer`. See [`super`].

use super::*;

//#region 🔖️Apply
#[derive(Debug)]
pub struct UiPatchApplyProducer {
    pub(super) retirement_handle: Option<UiPatchApplyHandle>,
    pub(super) generation: u64,
    pub(super) limits: UiDocumentLimits,
    pub(super) original: Option<crate::UiSnapshotState>,
    pub(super) draft: Option<crate::UiSnapshotState>,
    pub(super) patch: Option<crate::UiPatch>,
    phase: UiPatchApplyPhase,
    patch_bytes: usize,
    next_node: usize,
    next_op: usize,
    remove_stack: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    pub(super) remove_record: Option<crate::UiNodeRecord>,
    remove_child: usize,
    validation_started: bool,
    validation_stack: crate::UiFixedList<UiPatchValidateFrame, { crate::UI_DOCUMENT_NODES }>,
    validation_visited: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    validation_path: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    pub(super) validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    validation_dangling: usize,
    pub(super) rejection: Option<PatchRejection>,
}

impl UiPatchApplyProducer {
    #[expect(clippy::result_large_err, reason = "Refused patch admission returns the original state and patch with their explicit retirement authority.")]
    pub fn try_new(state: crate::UiSnapshotState, patch: crate::UiPatch, limits: UiDocumentLimits, generation: u64) -> Result<Self, UiPatchApplyRejected> {
        let retirement_handle = with_ui_patch_apply_arena(|arena| arena.reserve(generation));
        if retirement_handle.is_none() {
            return Err(UiPatchApplyRejected::new(None, generation, state, patch, PatchRejection::AliasCapacity));
        }
        if generation == 0 {
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, PatchRejection::StaleGeneration { expected: 1, actual: 0 }));
        }
        if patch.base_revision != state.revision {
            let rejection = PatchRejection::RevisionMismatch { expected: state.revision, actual: patch.base_revision };
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, rejection));
        }
        if patch.ops.len() > limits.max_patch_ops {
            let rejection = PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: patch.ops.len(), max: limits.max_patch_ops };
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, rejection));
        }
        let draft = crate::UiSnapshotState::new(state.surface.clone());
        Ok(Self {
            retirement_handle,
            generation,
            limits,
            original: Some(state),
            draft: Some(draft),
            patch: Some(patch),
            phase: UiPatchApplyPhase::CensusPatch,
            patch_bytes: 0,
            next_node: 0,
            next_op: 0,
            remove_stack: crate::UiFixedList::default(),
            remove_record: None,
            remove_child: 0,
            validation_started: false,
            validation_stack: crate::UiFixedList::default(),
            validation_visited: crate::UiFixedList::default(),
            validation_path: crate::UiFixedList::default(),
            validation_seen: std::array::from_fn(|_| crate::UiFixedList::default()),
            validation_dangling: 0,
            rejection: None,
        })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn rejection(&self) -> Option<&PatchRejection> {
        self.rejection.as_ref()
    }

    pub fn drive_one(&mut self, generation: u64, cancelled: bool, deadline_expired: bool) -> UiPatchApplyStep {
        if self.phase == UiPatchApplyPhase::Ready {
            return UiPatchApplyStep::Ready;
        }
        if self.phase == UiPatchApplyPhase::Rejected {
            return UiPatchApplyStep::Rejected;
        }
        if generation != self.generation {
            self.reject(PatchRejection::StaleGeneration { expected: self.generation, actual: generation });
            return UiPatchApplyStep::Rejected;
        }
        if cancelled {
            self.reject(PatchRejection::Cancelled);
            return UiPatchApplyStep::Rejected;
        }
        if deadline_expired {
            return UiPatchApplyStep::MoreWork;
        }
        match self.phase {
            UiPatchApplyPhase::CensusPatch => self.census_patch_one(),
            UiPatchApplyPhase::CloneState => self.clone_state_one(),
            UiPatchApplyPhase::ApplyOps => self.apply_op_one(),
            UiPatchApplyPhase::RemoveSubtree => self.remove_subtree_one(),
            UiPatchApplyPhase::Validate => self.validate_candidate_one(),
            UiPatchApplyPhase::Ready => UiPatchApplyStep::Ready,
            UiPatchApplyPhase::Rejected => UiPatchApplyStep::Rejected,
        }
    }

    fn census_patch_one(&mut self) -> UiPatchApplyStep {
        let Some(patch) = self.patch.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(op) = patch.ops.get(self.next_op) else {
            self.phase = UiPatchApplyPhase::CloneState;
            return UiPatchApplyStep::MoreWork;
        };
        const OP_OVERHEAD_BYTES: usize = 16;
        let Some(bytes) = self.patch_bytes.checked_add(OP_OVERHEAD_BYTES).and_then(|bytes| bytes.checked_add(op_text_bytes(op))) else {
            self.reject(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: usize::MAX, max: self.limits.max_patch_bytes });
            return UiPatchApplyStep::Rejected;
        };
        if bytes > self.limits.max_patch_bytes {
            self.reject(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: bytes, max: self.limits.max_patch_bytes });
            return UiPatchApplyStep::Rejected;
        }
        self.patch_bytes = bytes;
        let Some(next_op) = self.next_op.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_op = next_op;
        UiPatchApplyStep::MoreWork
    }

    fn remove_subtree_one(&mut self) -> UiPatchApplyStep {
        if let Some(record) = self.remove_record.as_ref() {
            if let Some(child) = record.children.get(self.remove_child).copied() {
                if let Err(child) = self.remove_stack.try_push(child) {
                    self.reject(PatchRejection::UnknownNode { id: child });
                    return UiPatchApplyStep::Rejected;
                }
                let Some(remove_child) = self.remove_child.checked_add(1) else {
                    self.reject(PatchRejection::AliasCapacity);
                    return UiPatchApplyStep::Rejected;
                };
                self.remove_child = remove_child;
                return UiPatchApplyStep::MoreWork;
            }
            drop(self.remove_record.take());
            self.remove_child = 0;
            return UiPatchApplyStep::MoreWork;
        }
        let Some(id) = self.remove_stack.pop() else {
            let Some(next_op) = self.next_op.checked_add(1) else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            self.next_op = next_op;
            self.phase = UiPatchApplyPhase::ApplyOps;
            return UiPatchApplyStep::MoreWork;
        };
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.remove_record = draft.nodes.remove(&id);
        UiPatchApplyStep::MoreWork
    }

    fn clone_state_one(&mut self) -> UiPatchApplyStep {
        let Some(original) = self.original.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(record) = original.nodes.get_index(self.next_node) else {
            let Some(draft) = self.draft.as_mut() else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            draft.root = original.root;
            draft.revision = original.revision;
            self.next_op = 0;
            self.phase = UiPatchApplyPhase::ApplyOps;
            return UiPatchApplyStep::MoreWork;
        };
        let Some(record) = record.credited_clone() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if let Err(record) = draft.nodes.try_insert(record) {
            drop(record);
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        }
        let Some(next_node) = self.next_node.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_node = next_node;
        UiPatchApplyStep::MoreWork
    }

    fn apply_op_one(&mut self) -> UiPatchApplyStep {
        let Some(patch) = self.patch.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(op) = patch.ops.get(self.next_op) else {
            let Some(draft) = self.draft.as_mut() else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            draft.revision = patch.revision;
            self.phase = UiPatchApplyPhase::Validate;
            return UiPatchApplyStep::MoreWork;
        };
        if let crate::UiPatchOp::Remove { id } = op {
            if let Err(id) = self.remove_stack.try_push(*id) {
                self.reject(PatchRejection::UnknownNode { id });
                return UiPatchApplyStep::Rejected;
            }
            self.phase = UiPatchApplyPhase::RemoveSubtree;
            return UiPatchApplyStep::MoreWork;
        }
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if let Err(rejection) = apply_op(draft, op, &self.limits) {
            self.reject(rejection);
            return UiPatchApplyStep::Rejected;
        }
        let Some(next_op) = self.next_op.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_op = next_op;
        UiPatchApplyStep::MoreWork
    }

    fn validate_candidate_one(&mut self) -> UiPatchApplyStep {
        let Some(draft) = self.draft.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if !self.validation_started {
            self.validation_started = true;
            if draft.nodes.len() > self.limits.max_nodes {
                return self.reject_violation(UiContractViolation::NodeQuota { count: draft.nodes.len(), max: self.limits.max_nodes });
            }
            if let Some(root) = draft.root.filter(|root| draft.nodes.get(root).is_some()) {
                let frame = UiPatchValidateFrame { id: root, depth: 0, parent_in_section: false, in_section: false, child: 0, stage: 0 };
                if self.validation_stack.try_push(frame).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
            }
            return UiPatchApplyStep::MoreWork;
        }
        if let Some(frame) = self.validation_stack.len().checked_sub(1).and_then(|index| self.validation_stack.get(index)).copied() {
            if frame.stage == 0 {
                if self.validation_path.iter().any(|id| *id == frame.id) {
                    return self.reject_violation(UiContractViolation::Cycle { node: frame.id });
                }
                if self.validation_visited.iter().any(|id| *id == frame.id) {
                    let _ = self.validation_stack.pop();
                    return UiPatchApplyStep::MoreWork;
                }
                if self.validation_visited.try_push(frame.id).is_err() || self.validation_path.try_push(frame.id).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                let Some(record) = draft.nodes.get(&frame.id) else {
                    return self.reject_violation(UiContractViolation::DanglingRoot { node: frame.id });
                };
                if frame.parent_in_section && is_section(&record.component) {
                    return self.reject_violation(UiContractViolation::SectionNested { node: frame.id });
                }
                if !component_is_finite(&record.component) {
                    return self.reject_violation(UiContractViolation::NonFiniteNumber { node: frame.id });
                }
                if !component_snaps_are_valid(&record.component) {
                    return self.reject_violation(UiContractViolation::InvalidSnaps { node: frame.id });
                }
                if !component_number_range_is_valid(&record.component) {
                    return self.reject_violation(UiContractViolation::InvalidNumberRange { node: frame.id });
                }
                if let crate::Component::TreeItem(props) = &record.component {
                    if let Some(toolbar) = props.inline_toolbar.filter(|_| !tree_inline_toolbar_is_valid(record, |id| draft.nodes.get(&id))) {
                        return self.reject_violation(UiContractViolation::InvalidTreeInlineToolbar { node: frame.id, toolbar });
                    }
                    if let Some(detail) = props.detail.filter(|_| !tree_detail_is_valid(record, |id| draft.nodes.get(&id))) {
                        return self.reject_violation(UiContractViolation::InvalidTreeDetail { node: frame.id, detail });
                    }
                }
                if let crate::Component::TreeSection(props) = &record.component {
                    if let Some(toolbar) = props.header_toolbar.filter(|_| !tree_section_header_toolbar_is_valid(record, |id| draft.nodes.get(&id))) {
                        return self.reject_violation(UiContractViolation::InvalidTreeSectionHeaderToolbar { node: frame.id, toolbar });
                    }
                }
                if !row_target_is_valid(record) {
                    return self.reject_violation(UiContractViolation::InvalidRowTarget { node: frame.id });
                }
                if frame.depth > self.limits.max_depth {
                    return self.reject_violation(UiContractViolation::DepthQuota { node: frame.id, depth: frame.depth, max: self.limits.max_depth });
                }
                let Some(depth_index) = self.validation_stack.len().checked_sub(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                self.validation_seen[depth_index] = crate::UiFixedList::default();
                let Some(active) = self.validation_stack.get_mut(depth_index) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                active.in_section = frame.parent_in_section || is_section(&record.component);
                active.stage = 1;
                return UiPatchApplyStep::MoreWork;
            }
            if frame.stage == 1 {
                let Some(record) = draft.nodes.get(&frame.id) else {
                    return self.reject_violation(UiContractViolation::DanglingRoot { node: frame.id });
                };
                let Some(child_id) = record.children.get(frame.child).copied() else {
                    let Some(active_index) = self.validation_stack.len().checked_sub(1) else {
                        return self.reject_violation(UiContractViolation::CensusCapacity);
                    };
                    let Some(active) = self.validation_stack.get_mut(active_index) else {
                        return self.reject_violation(UiContractViolation::CensusCapacity);
                    };
                    active.stage = 2;
                    return UiPatchApplyStep::MoreWork;
                };
                let Some(child) = draft.nodes.get(&child_id) else {
                    return self.reject_violation(UiContractViolation::OrphanChild { parent: frame.id, child: child_id });
                };
                let Some(depth_index) = self.validation_stack.len().checked_sub(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                if self.validation_seen[depth_index].iter().any(|key| key.as_str() == child.key.as_str()) {
                    return self.reject_violation(UiContractViolation::DuplicateSiblingKey { parent: frame.id, key: child.key.clone() });
                }
                if self.validation_seen[depth_index].try_push(child.key.clone()).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                let Some(depth) = frame.depth.checked_add(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                let Some(active) = self.validation_stack.get_mut(depth_index) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                let Some(next_child) = active.child.checked_add(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                active.child = next_child;
                let child_frame = UiPatchValidateFrame { id: child_id, depth, parent_in_section: frame.in_section, in_section: false, child: 0, stage: 0 };
                if self.validation_stack.try_push(child_frame).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                return UiPatchApplyStep::MoreWork;
            }
            let _ = self.validation_stack.pop();
            if self.validation_path.iter().next_back().is_some_and(|id| *id == frame.id) {
                let _ = self.validation_path.pop();
            }
            return UiPatchApplyStep::MoreWork;
        }
        if let Some(record) = draft.nodes.get_index(self.validation_dangling) {
            if !self.validation_visited.iter().any(|id| *id == record.id) {
                return self.reject_violation(UiContractViolation::DanglingRoot { node: record.id });
            }
            let Some(next) = self.validation_dangling.checked_add(1) else {
                return self.reject_violation(UiContractViolation::CensusCapacity);
            };
            self.validation_dangling = next;
            return UiPatchApplyStep::MoreWork;
        }
        self.phase = UiPatchApplyPhase::Ready;
        UiPatchApplyStep::Ready
    }

    fn reject_violation(&mut self, violation: UiContractViolation) -> UiPatchApplyStep {
        let mut violations = UiContractViolations::default();
        if violations.try_push(violation).is_err() {
            self.reject(PatchRejection::AliasCapacity);
        } else {
            self.reject(PatchRejection::InvariantViolated { violations });
        }
        UiPatchApplyStep::Rejected
    }

    pub(super) fn reject(&mut self, rejection: PatchRejection) {
        self.rejection = Some(rejection);
        self.phase = UiPatchApplyPhase::Rejected;
    }

    #[expect(clippy::result_large_err, reason = "A not-yet-ready producer must retain its exact state, patch, and retirement handle for another step.")]
    pub fn take_ready(mut self) -> Result<UiPatchApplyOutcome, Self> {
        if self.phase != UiPatchApplyPhase::Ready || self.remove_record.is_some() {
            return Err(self);
        }
        let (Some(state), Some(previous), Some(patch)) = (self.draft.take(), self.original.take(), self.patch.take()) else { return Err(self) };
        let Some(retirement_handle) = self.retirement_handle.take() else { return Err(self) };
        let validation_seen = std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default()));
        Ok(UiPatchApplyOutcome { retirement_handle: Some(retirement_handle), generation: self.generation, state: Some(state), previous: Some(previous), patch: Some(patch), validation_seen, validation_seen_cursor: 0 })
    }

    #[expect(clippy::result_large_err, reason = "A producer that has not reached rejection returns its full retained authority without a new allocation.")]
    pub fn take_rejected(mut self) -> Result<UiPatchApplyRejected, Self> {
        if self.phase != UiPatchApplyPhase::Rejected {
            return Err(self);
        }
        let (Some(state), Some(patch), Some(rejection)) = (self.original.take(), self.patch.take(), self.rejection.take()) else { return Err(self) };
        let Some(retirement_handle) = self.retirement_handle.take() else { return Err(self) };
        let validation_seen = std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default()));
        Ok(UiPatchApplyRejected {
            retirement_handle: Some(retirement_handle),
            generation: self.generation,
            state: Some(state),
            patch: Some(patch),
            draft: self.draft.take(),
            remove_record: self.remove_record.take(),
            rejection,
            retire_scalar: 0,
            validation_seen,
            validation_seen_cursor: 0,
        })
    }
}
//#endregion 🔖️Apply
