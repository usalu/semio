//! 🕹️ The `MeshJob` run loop, interactive step and retirement.

use super::*;

// #region 🧵️IncrementalMeshJob
impl MeshJob {
    /// ⏭️ Runs one bounded unit; a finished publication payload is staged in the job, never lent from here.
    fn run(&mut self, context: &mut StepContext<'_>) -> MeshRun {
        if context.is_cancelled() {
            return MeshRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fail(b"stale-mesh-operation".to_vec());
        }
        context.set_stage(match self.stage {
            MeshJobStage::Validate => "validate-references",
            MeshJobStage::ReservePreparation => "reserve-preparation-points",
            MeshJobStage::ReservePreparationIndex => "reserve-preparation-index",
            MeshJobStage::ReservePreparationConstraints => "reserve-preparation-constraints",
            MeshJobStage::PreparationFailed => "preparation-failed",
            MeshJobStage::PrepareInput => "prepare-input",
            MeshJobStage::CountInput => "count-input",
            MeshJobStage::Initialize => "initialize-triangulation",
            MeshJobStage::ReserveEdgeAuthorities => "reserve-edge-authorities",
            MeshJobStage::InsertBoundary => "insert-boundary",
            MeshJobStage::IndexEdges => "index-edges",
            MeshJobStage::ConstrainBoundary => match self.constraint_stage {
                ConstraintRecoveryStage::ReserveConstraintWorkspace => "reserve-constraint-workspace",
                ConstraintRecoveryStage::IndexTriangleEdge => "index-triangle-edge",
                ConstraintRecoveryStage::SearchConstraintEdge => "search-constraint-edge",
                ConstraintRecoveryStage::ClassifyIntersection => "classify-intersection",
                ConstraintRecoveryStage::SelectDeterministicFlip => "select-deterministic-flip",
                ConstraintRecoveryStage::ValidateFlip => "validate-flip",
                ConstraintRecoveryStage::ApplyFlip => "apply-flip",
                ConstraintRecoveryStage::RetireFormerEdge => "retire-former-edge",
                ConstraintRecoveryStage::PublishConstraintProgress => "publish-constraint-progress",
                ConstraintRecoveryStage::ConstraintComplete => "constraint-complete",
            },
            MeshJobStage::ReservePointIndex => "reserve-point-index",
            MeshJobStage::ReserveMeshPoints => "reserve-mesh-points",
            MeshJobStage::ReserveMeshTriangles => "reserve-mesh-triangles",
            MeshJobStage::Classify => "classify-elements",
            MeshJobStage::PublishPreview => "publish-preview-entry",
            MeshJobStage::Finalize => "finalize-mesh",
            MeshJobStage::PublishCheckpoint => "publish-checkpoint-entry",
            MeshJobStage::Complete => "publish-complete-entry",
            MeshJobStage::Published => "complete",
        });
        if context.should_yield() {
            return MeshRun::Yield;
        }
        context.consume_fuel(1);
        match self.stage {
            MeshJobStage::Validate => {
                if self.domain.outer().len() < 3 {
                    return Self::fail(MeshError::DegenerateDomain.to_string().into_bytes());
                }
                if let Some(hole) = self.domain.hole(self.validation_hole_cursor) {
                    if hole.len() < 3 {
                        return Self::fail(MeshError::DegenerateDomain.to_string().into_bytes());
                    }
                    self.validation_hole_cursor += 1;
                    return MeshRun::Yield;
                }
                let maximum_constraints = if self.maximum_triangles == usize::MAX {
                    usize::MAX
                } else {
                    let Some(maximum) = self.maximum_triangles.checked_mul(3) else {
                        self.stage = MeshJobStage::PreparationFailed;
                        return Self::fail(b"mesh-fixed-constraint-capacity".as_slice());
                    };
                    maximum
                };
                self.preparation = Some(MeshInputPreparation::new(self.maximum_points, maximum_constraints));
                self.stage = MeshJobStage::ReservePreparation;
                MeshRun::Yield
            }
            MeshJobStage::ReservePreparation | MeshJobStage::ReservePreparationIndex | MeshJobStage::ReservePreparationConstraints => {
                let preparation = self.preparation.as_mut().expect("input preparation initialized");
                let (admitted, next) = match self.stage {
                    MeshJobStage::ReservePreparation => (self.maximum_points == usize::MAX || MeshInputPreparation::reserve_owner(&mut preparation.points, self.maximum_points).is_ok(), MeshJobStage::ReservePreparationIndex),
                    MeshJobStage::ReservePreparationIndex => (self.maximum_points == usize::MAX || MeshInputPreparation::reserve_owner(&mut preparation.point_indices, self.maximum_points).is_ok(), MeshJobStage::ReservePreparationConstraints),
                    _ => (self.maximum_triangles == usize::MAX || MeshInputPreparation::reserve_owner(&mut preparation.constraints, preparation.maximum_constraints).is_ok(), MeshJobStage::PrepareInput),
                };
                if !admitted {
                    self.stage = MeshJobStage::PreparationFailed;
                    return Self::fail(b"mesh-fixed-preparation-backing".as_slice());
                }
                self.stage = next;
                MeshRun::Yield
            }
            MeshJobStage::PreparationFailed => Self::fail(b"mesh-fixed-preparation-backing".as_slice()),
            MeshJobStage::PrepareInput => {
                let complete = match self.preparation.as_mut().expect("input preparation initialized").advance(&self.domain, &self.options) {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.stage = MeshJobStage::PreparationFailed;
                        return Self::fail(error.as_bytes());
                    }
                };
                if complete {
                    let preparation = self.preparation.as_mut().expect("input preparation complete");
                    self.prepared_input = Some((std::mem::take(&mut preparation.points), std::mem::take(&mut preparation.constraints)));
                    self.input_count = self.domain.outer().len();
                    self.input_count_hole = 0;
                    self.stage = MeshJobStage::CountInput;
                }
                MeshRun::Yield
            }
            MeshJobStage::CountInput => {
                if let Some(hole) = self.domain.hole(self.input_count_hole) {
                    self.input_count = match self.input_count.checked_add(hole.len()) {
                        Some(count) => count,
                        None => return Self::fail(b"mesh-input-count-overflow".to_vec()),
                    };
                    self.input_count_hole += 1;
                } else {
                    self.stage = MeshJobStage::Initialize;
                }
                MeshRun::Yield
            }
            MeshJobStage::Initialize => {
                if self.triangulation.is_none() {
                    let (prepared_points, input_constraints) = self.prepared_input.take().expect("prepared input");
                    if prepared_points.len() > self.maximum_points {
                        self.prepared_input = Some((prepared_points, input_constraints));
                        return Self::fail(b"mesh-fixed-point-capacity".to_vec());
                    }
                    if self.maximum_points == usize::MAX {
                        let triangulation = match OwnedTriangulation::begin(prepared_points.clone()) {
                            Ok(triangulation) => triangulation,
                            Err(error) => return Self::fail(error.to_string().into_bytes()),
                        };
                        self.constraints = remap_constraints(&prepared_points, &triangulation.points[..triangulation.input_len], input_constraints);
                        self.refinement_steps = triangulation.input_len.saturating_sub(self.input_count);
                        self.triangulation = Some(triangulation);
                        self.stage = MeshJobStage::InsertBoundary;
                        return MeshRun::Yield;
                    }
                    let triangulation = match OwnedTriangulation::begin_mounted(prepared_points, self.maximum_triangles) {
                        Ok(triangulation) => triangulation,
                        Err(error) => return Self::fail(error.to_string().into_bytes()),
                    };
                    self.constraints = input_constraints;
                    self.refinement_steps = triangulation.input_len.saturating_sub(self.input_count);
                    self.triangulation = Some(triangulation);
                    return MeshRun::Yield;
                }
                let initialized = match self.triangulation.as_mut().expect("mounted triangulation shell retained").advance_mounted_initialization() {
                    Ok(initialized) => initialized,
                    Err(error) => return Self::fail(error.to_string().into_bytes()),
                };
                if initialized {
                    self.stage = MeshJobStage::InsertBoundary;
                }
                MeshRun::Yield
            }
            MeshJobStage::ReserveEdgeAuthorities => {
                let Some(edge_capacity) = self.triangulation.as_ref().and_then(|triangulation| triangulation.triangles.len().checked_mul(3)) else {
                    return Self::fail(b"mesh-edge-authority-face-capacity".to_vec());
                };
                if self.maximum_triangles != usize::MAX && edge_capacity.checked_mul(size_of::<IndexedConstraintEdge>()).is_none_or(|bytes| bytes > 16_384)
                    || self.indexed_constraint_edges.try_reserve_exact(edge_capacity).is_err()
                    || self.maximum_triangles != usize::MAX && self.indexed_constraint_edges.capacity().checked_mul(size_of::<IndexedConstraintEdge>()).is_none_or(|bytes| bytes > 16_384)
                {
                    return Self::fail(b"mesh-fixed-edge-authority-backing".to_vec());
                }
                self.stage = MeshJobStage::IndexEdges;
                MeshRun::Yield
            }
            MeshJobStage::InsertBoundary => {
                let triangulation = self.triangulation.as_mut().expect("validated triangulation");
                if triangulation.insert_cursor < triangulation.input_len {
                    triangulation.insert_next();
                } else if triangulation.advance_finish_insertion() {
                    self.stage = MeshJobStage::ReserveEdgeAuthorities;
                }
                if triangulation.allocation_fault {
                    return Self::fail(b"mesh-fixed-triangulation-workspace-backing".to_vec());
                }
                if triangulation.triangles.len() > self.maximum_triangles.saturating_mul(4).saturating_add(1) {
                    return Self::fail(b"mesh-fixed-triangulation-capacity".to_vec());
                }
                MeshRun::Yield
            }
            MeshJobStage::IndexEdges => {
                let face_count = self.triangulation.as_ref().map_or(0, |triangulation| triangulation.triangles.len());
                if self.edge_index_candidate.is_none() {
                    if self.edge_index_cursor < face_count && !self.begin_edge_index_candidate(self.edge_index_cursor, self.edge_index_local_cursor) {
                        return Self::fail(b"mesh-edge-index-triangle".to_vec());
                    }
                } else {
                    match self.advance_edge_index_candidate() {
                        Ok(true) => {
                            self.edge_index_local_cursor += 1;
                            if self.edge_index_local_cursor == 3 {
                                self.edge_index_local_cursor = 0;
                                self.edge_index_cursor += 1;
                            }
                        }
                        Ok(false) => {}
                        Err(()) => return Self::fail(b"mesh-edge-index-capacity".to_vec()),
                    }
                }
                if self.edge_index_cursor == face_count {
                    self.edge_index_valid = true;
                    self.constraint_stage = ConstraintRecoveryStage::ReserveConstraintWorkspace;
                    self.stage = MeshJobStage::ConstrainBoundary;
                }
                MeshRun::Yield
            }
            MeshJobStage::ConstrainBoundary => {
                if self.constraint_cursor < self.constraints.len() {
                    match self.advance_constraint_recovery() {
                        Ok(true) => {
                            self.constraint_cursor += 1;
                            self.constraint_stage = ConstraintRecoveryStage::ReserveConstraintWorkspace;
                        }
                        Ok(false) => {}
                        Err(error) => return Self::fail(error.to_string().into_bytes()),
                    }
                }
                if self.constraint_cursor == self.constraints.len() {
                    if self.maximum_points == usize::MAX {
                        self.begin_classification(if self.refinement_steps == 0 { MeshQualityTier::Coarse } else { MeshQualityTier::Final });
                    } else {
                        self.stage = MeshJobStage::ReservePointIndex;
                    }
                }
                MeshRun::Yield
            }
            MeshJobStage::ReservePointIndex => {
                if self.point_index.try_reserve_exact(self.maximum_points).is_err() || self.point_index.capacity().checked_mul(size_of::<((u64, u64), u32)>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-point-index-backing".to_vec());
                }
                self.stage = MeshJobStage::ReserveMeshPoints;
                MeshRun::Yield
            }
            MeshJobStage::ReserveMeshPoints => {
                if self.mesh.points.try_reserve_exact(self.maximum_points).is_err() || self.mesh.points.capacity().checked_mul(size_of::<[f64; 2]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-point-backing".to_vec());
                }
                self.stage = MeshJobStage::ReserveMeshTriangles;
                MeshRun::Yield
            }
            MeshJobStage::ReserveMeshTriangles => {
                if self.mesh.tris.try_reserve_exact(self.maximum_triangles).is_err() || self.mesh.tris.capacity().checked_mul(size_of::<[u32; 3]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-triangle-backing".to_vec());
                }
                self.begin_classification(if self.refinement_steps == 0 { MeshQualityTier::Coarse } else { MeshQualityTier::Final });
                MeshRun::Yield
            }
            MeshJobStage::Classify => {
                let face_count = self.triangulation.as_ref().expect("constrained triangulation").triangles.len();
                if self.face_cursor >= face_count {
                    self.stage = MeshJobStage::PublishPreview;
                    return MeshRun::Yield;
                }
                if self.advance_face_classification().is_err() {
                    return Self::fail(b"mesh-fixed-output-capacity".to_vec());
                }
                MeshRun::Yield
            }
            MeshJobStage::PublishPreview => match self.advance_publication(context, MeshPublicationKind::Preview) {
                Ok(true) => {
                    self.stage = MeshJobStage::Finalize;
                    MeshRun::Preview
                }
                Ok(false) => MeshRun::Yield,
                Err(()) => Self::fail(b"mesh-preview-publication".to_vec()),
            },
            MeshJobStage::Finalize => {
                self.preview_tier = MeshQualityTier::Final;
                self.stage = MeshJobStage::PublishCheckpoint;
                MeshRun::Yield
            }
            MeshJobStage::PublishCheckpoint => match self.advance_publication(context, MeshPublicationKind::Checkpoint) {
                Ok(true) => {
                    self.stage = MeshJobStage::Complete;
                    MeshRun::Checkpoint(self.mesh.tris.len() as u64)
                }
                Ok(false) => MeshRun::Yield,
                Err(()) => Self::fail(b"mesh-checkpoint-publication".to_vec()),
            },
            MeshJobStage::Complete => match self.advance_publication(context, MeshPublicationKind::Complete) {
                Ok(true) => {
                    self.stage = MeshJobStage::Published;
                    MeshRun::Complete
                }
                Ok(false) => MeshRun::Yield,
                Err(()) => Self::fail(b"mesh-output-publication".to_vec()),
            },
            MeshJobStage::Published => MeshRun::Published,
        }
    }
}

impl MeshJob {
    /// 🤝️ Pays back the delivered payload turn by turn from the next call's own wallet before the search resumes.
    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let lent = self.lent.as_mut().expect("a delivered mesh publication is staged");
        let demand = lent.payload.retirement_demands()?;
        let grant = cx.retained_grant();
        if mesh_covers(grant, demand) {
            let progress = match lent.payload.close_step(grant)? {
                RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress,
            };
            cx.consume_retained(progress)?;
        }
        if lent.payload.terminal_is_empty() {
            self.lent = None;
            self.lent_delivered = false;
        }
        Ok(None)
    }
}

impl InteractiveJob for MeshJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.lent_delivered && !cx.is_cancelled() && cx.operation() == self.operation.operation && cx.generation() == self.operation.generation {
            return Ok(self.retire_delivered(cx)?);
        }
        let run = match self.pending_run.take() {
            Some(run) => run,
            None => self.run(cx),
        };
        let admitted = match run {
            MeshRun::Yield => return Ok(None),
            MeshRun::Cancelled => JobOutcomeBorrow::admit_cancelled(cx)?,
            MeshRun::Fault => JobOutcomeBorrow::admit_fault(cx, &self.markers.fault)?,
            MeshRun::Preview => JobOutcomeBorrow::admit_preview(cx, &self.lent.as_ref().expect("a staged mesh preview").payload)?,
            MeshRun::Checkpoint(applied_progress) => JobOutcomeBorrow::admit_checkpoint(cx, &self.lent.as_ref().expect("a staged mesh checkpoint").payload, applied_progress)?,
            MeshRun::Complete => JobOutcomeBorrow::admit_complete(cx, Some(&self.markers.state), self.lent.as_ref().map(|lent| &lent.payload))?,
            MeshRun::Published => JobOutcomeBorrow::admit_complete(cx, Some(&self.markers.state), Some(&self.markers.output))?,
        };
        if admitted.is_none() {
            self.pending_run = Some(run);
        } else if matches!(run, MeshRun::Preview | MeshRun::Checkpoint(_) | MeshRun::Complete) {
            self.lent_delivered = true;
        }
        Ok(admitted)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        let lent = || self.lent.as_ref().map(|lent| &lent.payload).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "mesh outcome has no staged payload"));
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault => descriptor.fault(&self.markers.fault),
            JobOutcomeKind::PreviewReady => descriptor.preview(lent()?),
            JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(lent()?),
            JobOutcomeKind::Complete => descriptor.complete(Some(&self.markers.state), Some(self.lent.as_ref().map_or(&self.markers.output, |lent| &lent.payload))),
        }
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.publication_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.retirement_demand()?;
        let step = close_original_retained_step(cx, demand, |grant| {
            Ok(match self.close_granted(grant)? {
                InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
                InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
                InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(RetainedCloneProgress::default()),
            })
        })?;
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.retirement_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.publication_writer.is_none() && self.lent.is_none() && self.close_lane > 9
            && self.preparation.is_none() && self.prepared_input.is_none() && self.triangulation.is_none()
            && self.constraints.capacity() == 0 && self.mesh.points.capacity() == 0 && self.mesh.tris.capacity() == 0
            && self.point_index.capacity() == 0 && self.indexed_constraint_edges.capacity() == 0
            && match &self.domain {
                MeshDomainOwner::Dynamic(domain) => domain.outer.capacity() == 0 && domain.holes.capacity() == 0,
                MeshDomainOwner::Mounted(domain) => domain.outer.len == 0 && domain.outer.admitted == 0
                    && domain.hole_count == 0 && domain.admitted_holes == 0
                    && domain.holes.iter().all(|hole| hole.len == 0 && hole.admitted == 0),
            }
    }
}
// #endregion 🧵️IncrementalMeshJob

fn vector_retirement_demand<T>(owner: &Vec<T>, depth: usize) -> Result<Option<RetirementDemand>, ValueError> {
    if !owner.is_empty() {
        return Ok(Some(RetirementDemand { copy_bytes: size_of::<T>(), depth, ..RetirementDemand::default() }));
    }
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "mesh vector backing extent overflow"))?;
    Ok((bytes != 0).then_some(RetirementDemand { release_bytes: bytes, depth, ..RetirementDemand::default() }))
}

fn logical_retirement_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..RetirementDemand::default() }
}

impl MeshJob {
    pub(super) fn retirement_demand(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(lent) = self.lent.as_ref() {
            return lent.payload.retirement_demands();
        }
        if let Some(writer) = self.publication_writer.as_ref() {
            return writer.retirement_demands();
        }
        for lane in self.close_lane..=9 {
            match lane {
                0 => match &self.domain {
                    MeshDomainOwner::Dynamic(domain) => if let Some(demand) = vector_retirement_demand(&domain.outer, 1)? { return Ok(demand); },
                    MeshDomainOwner::Mounted(domain) => {
                        if domain.close_hole < domain.hole_count {
                            let polygon = &domain.holes[domain.close_hole];
                            return Ok(RetirementDemand { copy_bytes: if polygon.len != 0 { size_of::<[f64; 2]>() } else { 0 }, depth: 2, ..RetirementDemand::default() });
                        }
                        if domain.hole_count != 0 || domain.admitted_holes != 0 || domain.outer.len != 0 || domain.outer.admitted != 0 {
                            return Ok(RetirementDemand { copy_bytes: if domain.hole_count == 0 && domain.admitted_holes == 0 && domain.outer.len != 0 { size_of::<[f64; 2]>() } else { 0 }, depth: 1, ..RetirementDemand::default() });
                        }
                    }
                },
                1 => if let MeshDomainOwner::Dynamic(domain) = &self.domain {
                    if let Some(hole) = domain.holes.last() {
                        return Ok(vector_retirement_demand(hole, 2)?.unwrap_or(RetirementDemand { copy_bytes: size_of::<Vec<[f64; 2]>>(), depth: 1, ..RetirementDemand::default() }));
                    }
                    if let Some(demand) = vector_retirement_demand(&domain.holes, 1)? { return Ok(demand); }
                },
                2 => if let Some(owner) = self.preparation.as_ref() {
                    if let Some(demand) = vector_retirement_demand(&owner.points, 1)? { return Ok(demand); }
                    if let Some(demand) = vector_retirement_demand(&owner.constraints, 1)? { return Ok(demand); }
                    if let Some(demand) = vector_retirement_demand(&owner.point_indices, 1)? { return Ok(demand); }
                    return Ok(logical_retirement_demand());
                },
                3 => if let Some((points, edges)) = self.prepared_input.as_ref() {
                    if let Some(demand) = vector_retirement_demand(points, 1)? { return Ok(demand); }
                    if let Some(demand) = vector_retirement_demand(edges, 1)? { return Ok(demand); }
                    return Ok(logical_retirement_demand());
                },
                4 => if let Some(owner) = self.triangulation.as_ref() {
                    if let Some(insertion) = owner.insertion.as_ref() {
                        if let Some(demand) = vector_retirement_demand(&insertion.bad, 1)? { return Ok(demand); }
                        if let Some(demand) = vector_retirement_demand(&insertion.boundary, 1)? { return Ok(demand); }
                        if let Some(demand) = vector_retirement_demand(&insertion.retained, 1)? { return Ok(demand); }
                        return Ok(logical_retirement_demand());
                    }
                    if let Some(demand) = vector_retirement_demand(&owner.points, 1)? { return Ok(demand); }
                    if let Some(demand) = vector_retirement_demand(&owner.triangles, 1)? { return Ok(demand); }
                    if let Some(demand) = vector_retirement_demand(&owner.insertion_order, 1)? { return Ok(demand); }
                    return Ok(logical_retirement_demand());
                },
                5 => if let Some(demand) = vector_retirement_demand(&self.constraints, 1)? { return Ok(demand); },
                6 => if let Some(demand) = vector_retirement_demand(&self.mesh.points, 1)? { return Ok(demand); },
                7 => if let Some(demand) = vector_retirement_demand(&self.mesh.tris, 1)? { return Ok(demand); },
                8 => if let Some(demand) = vector_retirement_demand(&self.point_index, 1)? { return Ok(demand); },
                9 => if let Some(demand) = vector_retirement_demand(&self.indexed_constraint_edges, 1)? { return Ok(demand); },
                _ => {}
            }
        }
        Ok(RetirementDemand::default())
    }

    /// 🎟️ Releases the lent payload first, then the half-built publication writer, one page or ledger per granted turn.
    fn close_publication(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let step = if let Some(lent) = self.lent.as_mut() {
            let step = lent.payload.close_step(grant);
            if lent.payload.terminal_is_empty() {
                self.lent = None;
                self.lent_delivered = false;
            }
            step
        } else if let Some(writer) = self.publication_writer.as_mut() {
            writer.begin_close();
            let step = writer.close_step(grant);
            if writer.terminal_is_empty() {
                self.publication_writer = None;
                self.publication_kind = None;
            }
            step
        } else {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        };
        let (RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) = step?;
        Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } })
    }

    /// 🎟️ One close turn under an explicit grant, admitted against the job's own quote; the original-context
    /// `close_step` runs it under the caller's wallet.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let demand = self.retirement_demand()?;
        if grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "mesh close grant is shallower than its quoted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        if self.lent.is_some() || self.publication_writer.is_some() {
            return self.close_publication(grant);
        }
        let (complete, items, bytes) = self.retire_owner_turn(grant.maximum_release_bytes);
        let progress = RetainedCloneProgress { copied_items: items, copied_bytes: if items != 0 { demand.copy_bytes } else { 0 }, released_bytes: bytes, ..RetainedCloneProgress::default() };
        let step = if complete { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } };
        step.admit(grant, self.terminal_is_empty())
    }
}
