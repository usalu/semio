//! 🧩️ Component advance and retirement: the credit admission, fault handling, the existing and fresh component advance, the record assembly, the pending patch, the native resource close and the retirement step. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
impl SurfaceReconcileCursor {
    /// 🎟️ Grows this reconcile's resident credit to cover what it has ACTUALLY used, in page quanta, before
    /// the next step allocates into it — the reservation is priced by the tree being reconciled instead of by
    /// the per-surface ceiling.
    ///
    /// 🐛️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B58: `SurfaceReconcileReservation::try_new` took
    /// [`SURFACE_RECONCILE_SURFACE_BYTES`] (8 MiB) of a 32 MiB aggregate for EVERY surface, so three
    /// reconciles ran process-wide while one puzzle3d session mounts thirteen and every fourth surface was
    /// refused `registry-resident-credit-exhausted`, deferred and re-dirtied for ever (wave B54 §8.2, wave
    /// B56 §1.1). The credit now starts at [`SURFACE_RECONCILE_FLOOR_BYTES`] — one assembly root plus four
    /// work pages, more than any single step can charge — and climbs with `usage`, so the per-surface
    /// ceiling is reached only by a body that genuinely fills it. An aggregate that cannot take the increase
    /// is a NAMED terminal fault, never a silent defer: a busy ledger is retried on the next step (the floor
    /// headroom covers it), a full one faults.
    pub(super) fn admit_credit(&mut self, required: SurfaceReconcileUsage) -> Result<(), SurfaceReconcileFault> {
        if !required.fits(self.limits) {
            return Err(SurfaceReconcileFault::Credits { usage: required, limits: self.limits });
        }
        let needed = ui_contract::UiResidentLimits {
            items: required.items.saturating_add(SURFACE_RECONCILE_FLOOR_ITEMS).min(self.limits.max_items),
            bytes: required.bytes.saturating_add(SURFACE_RECONCILE_FLOOR_BYTES).min(self.limits.max_bytes),
        };
        let refusal = || SurfaceReconcileFault::ResidentCredit { required: needed.bytes, aggregate: ui_contract::UiResidentPermit::snapshot().unwrap_or_default().bytes };
        if let Some(credit) = self.assembly_credit.as_mut() {
            let held = credit.limits();
            if needed.items <= held.items && needed.bytes <= held.bytes {
                return Ok(());
            }
            return match credit.try_reprice(ui_contract::UiResidentLimits { items: needed.items.max(held.items), bytes: needed.bytes.max(held.bytes) }) {
                Ok(_) | Err(ui_contract::UiResidentFault::Contended) => Ok(()),
                Err(_) => Err(refusal()),
            };
        }
        if !self.assembly_open {
            return Ok(());
        }
        let held = match self.assembly.resident_limits() {
            Ok(limits) => limits,
            Err(_) => return Ok(()),
        };
        if needed.items <= held.items && needed.bytes <= held.bytes {
            return Ok(());
        }
        match self.assembly.reprice_resident(ui_contract::UiResidentLimits { items: needed.items.max(held.items), bytes: needed.bytes.max(held.bytes) }, 1, SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(_) => Ok(()),
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => Ok(()),
            Err(_) => Err(refusal()),
        }
    }

    pub fn clear_fault(&mut self) {
        self.fault = None;
    }

    pub(super) fn fail(&mut self, fault: SurfaceReconcileFault) -> SurfaceReconcileStep {
        self.fault = Some(fault.clone());
        SurfaceReconcileStep::Fault(fault)
    }

    pub(super) fn advance_existing_component(&mut self, current: &SurfaceShadow) -> SurfaceReconcileStep {
        self.advance_existing_component_with_grant(current, SURFACE_COMPONENT_COPY_WORK_BYTES)
    }

    pub(super) fn advance_existing_component_with_grant(&mut self, current: &SurfaceShadow, work: usize) -> SurfaceReconcileStep {
        if work == 0 { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
        let required = size_of::<ExistingComponentComparison>();
        if self.record_diff.as_ref().unwrap().owned_copy.is_none() {
            let mut projected = self.usage;
            if !projected.include(0, 1, required) || !projected.fits(self.limits) {
                return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
            }
            if required > work { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            self.record_diff.as_mut().unwrap().owned_copy = Some(RecordOwnedCopy::Comparison(ExistingComponentComparison::default()));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: required };
        }
        let diff = self.record_diff.as_mut().unwrap();
        if let Some(RecordOwnedCopy::Comparison(comparison)) = diff.owned_copy.as_mut() {
            if comparison.changed.is_none() {
                if let Some(equal) = comparison.cursor.result() {
                    comparison.cursor.release_reads();
                    comparison.changed = Some(!equal);
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<bool>() };
                }
                if comparison.lease.is_none() {
                    return match current.capture_document(&mut comparison.lease, work) {
                        Ok(true) => SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiDocumentLease>() },
                        Ok(false) | Err(ui_contract::UiDocumentLeaseError::Contended) => SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                        Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                    };
                }
                let Some(ordinal) = current.ordinals.get(&diff.id).copied() else { return self.fail(SurfaceReconcileFault::AliasCapacity); };
                let progress = (|| {
                    let read = comparison.lease.as_ref().unwrap().try_read()?;
                    let old = read.exact_node(ordinal, diff.id)?;
                    comparison.cursor.advance(&old.component, &diff.record.component, work).map_err(|_| ui_contract::UiDocumentLeaseError::NodeIdentity)
                })();
                return match progress {
                    Ok(progress) => SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.compared_bytes },
                    Err(ui_contract::UiDocumentLeaseError::Contended) => SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                    Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                };
            }
            if let Some(lease) = comparison.lease.as_mut() {
                if lease.terminal_is_empty() {
                    if work < size_of::<ui_contract::UiDocumentLease>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                    comparison.lease = None;
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiDocumentLease>() };
                }
                return match lease.close_read_step_with_grant(1, work) {
                    Ok(progress) => SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.released_bytes },
                    Err(_) => self.fail(SurfaceReconcileFault::AliasCapacity),
                };
            }
            if work < required { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let changed = comparison.changed.unwrap();
            diff.owned_copy = Some(RecordOwnedCopy::Changed(changed));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: required };
        }
        if let Some(RecordOwnedCopy::Changed(changed)) = diff.owned_copy.as_ref() {
            if !changed { diff.owned_copy = None; diff.field = 1; return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<bool>() }; }
            let initialized = size_of::<ui_contract::UiComponentCopy>() + size_of::<ui_contract::Component>();
            if initialized > SURFACE_RECONCILE_PAGE_BYTES || work < size_of::<ui_contract::Component>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let source = std::mem::replace(&mut diff.record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
            diff.owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: initialized };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::component_mut).expect("changed component retains its copy cursor");
        if copy.terminal_is_empty() {
            diff.owned_copy = None;
            diff.field = 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() };
        }
        if copy.candidate().is_some() {
            if work < size_of::<ui_contract::Component>() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            if let Some(source) = copy.take_completed_source_with_grant(work) {
                diff.record.component = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::Component>() };
            }
            let target = self.pending_op.source_mut().expect("active pending operation is writable");
            if target.is_some() || size_of::<ui_contract::UiPatchOp>() > SURFACE_RECONCILE_PAGE_BYTES { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            let candidate = copy.take_completed_candidate_with_grant(work).expect("completed candidate remains in its exact owner");
            *target = Some(ui_contract::UiPatchOp::SetComponent { id: diff.id, component: candidate });
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
        }
        let requested = match copy.next_allocation_bytes() { Ok(bytes) => bytes, Err(_) => return self.fail(SurfaceReconcileFault::CounterOverflow) };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
        }
        let child = if requested == 0 { copy.advance(1, 0, work) } else { copy.reserve_next(requested) };
        let progress = match child {
            Ok(progress) => progress,
            Err(error) => { self.usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes); return self.fail(SurfaceReconcileFault::AliasCapacity); }
        };
        if !self.usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !self.usage.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits });
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes }
    }

    pub(super) fn advance_record_assembly(&mut self) -> SurfaceReconcileStep {
        if !self.assembly_open {
            if self.assembly_credit.is_none() { return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits }); }
            let root = self.ids.first().copied().or_else(|| self.record_diff.as_ref().map(|record| record.id));
            match self.assembly.open_with_permit(&mut self.assembly_credit, &mut self.assembly_surface, ui_contract::UiDocumentAssemblyIdentity { generation: self.assembly_generation, revision: self.base_revision, root, layout_epoch: 0 }, 1, SURFACE_RECONCILE_PAGE_BYTES) {
                Ok(progress) => { self.assembly_open = progress.progressed; return SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.initialized_bytes + progress.moved_bytes }; }
                Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
            }
        }
        let id = self.record_diff.as_ref().unwrap().id;
        let requested = match self.assembly.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
            Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
        };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: projected, limits: self.limits });
        }
        let progress = match self.assembly.place_one(&mut self.record_diff.as_mut().unwrap().record.0, 1, SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(progress) => progress,
            Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
            Err(error) => { self.usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes); return self.fail(SurfaceReconcileFault::AliasCapacity); }
        };
        if !self.usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !self.usage.fits(self.limits) {
            return self.fail(SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits });
        }
        if self.record_diff.as_ref().unwrap().record.0.is_none() {
            if self.new_ordinals.try_insert(id, self.diff_index).is_err() { return self.fail(SurfaceReconcileFault::CounterOverflow); }
            self.record_diff = None;
            self.diff_index += 1;
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.initialized_bytes + progress.compared_bytes + progress.moved_bytes }
    }

    pub(super) fn advance_fresh_component(diff: &mut RecordDiffCursor, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits, fault_owner: &mut Option<SurfaceReconcileFault>) -> SurfaceReconcileStep {
        if diff.owned_copy.is_none() {
            let source = std::mem::replace(&mut diff.record.component, ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
            diff.owned_copy = Some(RecordOwnedCopy::Component(ui_contract::UiComponentCopy::new(source)));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() + size_of::<ui_contract::Component>() };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::component_mut).expect("component copy retained in its cursor");
        if copy.candidate().is_some() {
            if let Some(source) = copy.take_completed_source_with_grant(SURFACE_COMPONENT_COPY_WORK_BYTES) {
                diff.record.component = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::Component>() };
            }
            let target = &mut diff.fresh.as_mut().expect("fresh component field retained").component;
            if target.is_some() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
            *target = copy.take_completed_candidate_with_grant(SURFACE_COMPONENT_COPY_WORK_BYTES);
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: if target.is_some() { size_of::<ui_contract::Component>() } else { 0 } };
        }
        if copy.terminal_is_empty() {
            diff.owned_copy = None;
            diff.field += 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiComponentCopy>() };
        }
        let requested = match copy.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => { *fault_owner = Some(SurfaceReconcileFault::CounterOverflow); return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow); }
        };
        let mut projected = *usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let progress = match if requested == 0 { copy.advance(1, 0, SURFACE_COMPONENT_COPY_WORK_BYTES) } else { copy.reserve_next(requested) } {
            Ok(progress) => progress,
            Err(error) => {
                let counted = usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes);
                let fault = if !counted { SurfaceReconcileFault::CounterOverflow } else if error.allocated_bytes != 0 { SurfaceReconcileFault::Credits { usage: *usage, limits } } else { SurfaceReconcileFault::AliasCapacity };
                *fault_owner = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        };
        if !usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !usage.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: *usage, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes }
    }

    pub(super) fn advance_fresh_bindings(diff: &mut RecordDiffCursor, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits, fault_owner: &mut Option<SurfaceReconcileFault>) -> SurfaceReconcileStep {
        let fresh = diff.fresh.as_mut().expect("fresh binding copy owns the field");
        if diff.owned_copy.is_none() {
            diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut diff.record.bindings))));
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiBindingsCopy>() };
        }
        let copy = diff.owned_copy.as_mut().and_then(RecordOwnedCopy::bindings_mut).expect("binding copy retained above");
        if copy.is_complete() {
            if let Some(source) = copy.take_completed_source_with_grant(SURFACE_RECONCILE_PAGE_BYTES) {
                diff.record.bindings = source;
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeBindings>() };
            }
            if copy.candidate().is_some() {
                if fresh.bindings.is_some() { return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                fresh.bindings = copy.take_completed_candidate_with_grant(SURFACE_RECONCILE_PAGE_BYTES);
                return SurfaceReconcileStep::Yield { nodes: 0, bytes: if fresh.bindings.is_some() { size_of::<ui_contract::UiNodeBindings>() } else { 0 } };
            }
            diff.owned_copy = None;
            diff.field += 1;
            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiBindingsCopy>() };
        }
        let requested = match copy.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => { *fault_owner = Some(SurfaceReconcileFault::CounterOverflow); return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow); }
        };
        let mut projected = *usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, usize::from(requested != 0), requested) || !projected.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let progress = match copy.advance(1, requested, if requested == 0 { SURFACE_RECONCILE_PAGE_BYTES } else { 0 }) {
            Ok(progress) => progress,
            Err(error) => {
                let counted = usage.include(0, usize::from(error.allocated_bytes != 0), error.allocated_bytes);
                let fault = if !counted { SurfaceReconcileFault::CounterOverflow } else if error.allocated_bytes != 0 { SurfaceReconcileFault::Credits { usage: *usage, limits } } else { SurfaceReconcileFault::AliasCapacity };
                *fault_owner = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        };
        if !usage.include(0, usize::from(progress.allocated_bytes != 0), progress.allocated_bytes) || !usage.fits(limits) {
            let fault = SurfaceReconcileFault::Credits { usage: *usage, limits };
            *fault_owner = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: progress.allocated_bytes + progress.copied_bytes + progress.placed_bytes }
    }

    pub(super) fn advance_pending_patch(&mut self) -> SurfaceReconcileStep {
        if self.ops.has_reserved_slot() {
            return match self.pending_op.place_into(&mut self.ops, SURFACE_RECONCILE_PAGE_BYTES) {
                Ok(bytes) => SurfaceReconcileStep::Yield { nodes: 0, bytes },
                Err(_) => {
                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                    SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow)
                }
            };
        }
        let requested = match self.ops.next_allocation_bytes() {
            Ok(bytes) => bytes,
            Err(_) => {
                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
            }
        };
        let mut projected = self.usage;
        if requested > SURFACE_RECONCILE_PAGE_BYTES || !projected.include(0, 1, requested) || !projected.fits(self.limits) {
            let fault = SurfaceReconcileFault::Credits { usage: projected, limits: self.limits };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let (allocated, failed) = match self.ops.try_reserve_one(requested) {
            Ok(bytes) => (bytes, false),
            Err(error) => (error.allocated_bytes, true),
        };
        if !self.usage.include(0, usize::from(allocated != 0), allocated) || !self.usage.fits(self.limits) || failed {
            let fault = SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        SurfaceReconcileStep::Yield { nodes: 0, bytes: allocated }
    }

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.native_resources.as_ref() }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        let Some(resources) = self.native_resources.as_mut() else { return Ok(Some(true)); };
        if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
        self.native_resources = None;
        Ok(Some(false))
    }

    pub fn retire_one(&mut self) -> bool {
        if self.native_resources.is_some() { return false; }
        if !self.retire_tree.step() {
            return false;
        }
        if self.retire_tree.try_begin_held(&mut self.held_node) {
            self.semantic_census = None;
            return false;
        }
        if self.retire_tree.try_begin_node(&mut self.pending_root) {
            return false;
        }
        if let Some(mut frame) = self.overflow_frame.take() {
            if let Some(child) = frame.children.next() {
                self.overflow_frame = Some(frame);
                self.held_node = Some((None, child));
                self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
            }
            return false;
        }
        if !self.pending_op.terminal_is_empty() {
            let _ = self.pending_op.close_step(1, SURFACE_RECONCILE_PAGE_BYTES);
            return false;
        }
        if let Some(diff) = self.record_diff.as_mut() {
            if diff.record.0.is_none() { self.record_diff = None; return false; }
            if let Some(fresh) = diff.fresh.as_mut() {
                if retire_fresh_record_one(fresh, &mut self.retire_fresh_field, &mut diff.owned_copy) {
                    diff.fresh = None;
                    self.retire_fresh_field = 0;
                }
                return false;
            }
            if retire_record_one(&mut diff.record, &mut self.retire_record_field, &mut diff.owned_copy) {
                self.record_diff = None;
                self.retire_record_field = 0;
            }
            return false;
        }
        if let Some(frame) = self.traversal.last_mut() {
            if let Some(child) = frame.children.next() {
                self.held_node = Some((None, child));
                self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                return false;
            }
            self.traversal.pop();
            return false;
        }
        if !self.ops.terminal_is_empty() {
            let _ = self.ops.close_step(1, SURFACE_RECONCILE_PAGE_BYTES);
            return false;
        }
        if self.flat.pop().is_some() || self.postorder.pop().is_some() || self.ids.pop().is_some() || self.removal.pop().is_some() {
            return false;
        }
        if self.seen.pop().is_some() {
            return false;
        }
        if !self.assembly.terminal_is_empty() {
            self.assembly.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("retained assembly close preserves exact fault");
            return false;
        }
        if self.new_ordinals.take_first().is_some() {
            return false;
        }
        if self.assembly_credit.is_some() {
            release_surface_reconcile_one(&mut self.assembly_credit).expect("unattached candidate permit remains exact");
            return false;
        }
        if self.new_key_index.take_first().is_some() {
            return false;
        }
        self.remove_next = None;
        self.fault = None;
        true
    }
}
//#endregion ⏭️ResumableReconcile
