//! 🧬️ Original assigned fields stage under real grants and install only with the central mutation capability.
use super::*;
use protocol::{RetainedDiff, RetainedDiffApply, RetainedDiffPreflightAdvance, RetainedFieldPreparation, RetainedRowPatchApply};
use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;
use std::mem::size_of;

impl RetainedDiff<RemodelingSnapshot> for RemodelingDiff {
    type Apply = RemodelingApply;
}

pub struct RemodelingApply {
    schema: RetainedFieldPreparation<String>,
    id: RetainedFieldPreparation<String>,
    result_sparse: RetainedFieldPreparation<Option<SparseCloud>>,
    result_dense: RetainedFieldPreparation<Option<DenseCloud>>,
    result_mesh: RetainedFieldPreparation<RemodelingMesh>,
    result_trajectory: RetainedFieldPreparation<Option<CameraTrajectory>>,
    result_tracks: RetainedFieldPreparation<Vec<MotionTrackSummary>>,
    result_geo: RetainedFieldPreparation<Option<GeoProducts>>,
    result_qc: RetainedFieldPreparation<Option<QcReportSnapshot>>,
    params: RetainedRowPatchApply<ReconstructionParams, RemodelingParamsDiff>,
    diff: Option<RetainedCloneBinding>,
    original: Option<RetainedCloneBinding>,
    post_identity: Option<(u64, usize, usize, usize)>,
    phase: usize,
    commit: usize,
    committing: bool,
    closing: bool,
}
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "remodeling retained assigned-field phase or source changed")
}
fn scalar(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}
impl RemodelingApply {
    fn verify(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>) -> Result<(), ValueError> {
        d.check_binding(&self.diff)?;
        original.check_binding(&self.original)?;
        Ok(())
    }
    fn preflight_sources(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>, post: RetainedCloneRef<'_, RemodelingSnapshot>) -> Result<(), ValueError> {
        if self.closing || self.committing {
            return Err(invalid());
        }
        self.verify(d, original)?;
        if self.post_identity.is_some_and(|identity| identity != post.reference_identity()) {
            return Err(invalid());
        }
        if d.get().streams.is_some() || d.get().assets.is_some() || d.get().durable_artifacts.is_some() || d.get().calibration.is_some() || d.get().gcps.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "remodeling keyed collection preparation is not registered"));
        }
        Ok(())
    }
    fn commit_sources(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>) -> Result<(), ValueError> {
        if self.closing || self.phase != 10 {
            return Err(invalid());
        }
        self.verify(d, original)
    }
    fn closing_binding(&self) -> &Option<RetainedCloneBinding> {
        if self.original.is_some() {
            &self.original
        } else {
            &self.diff
        }
    }
}
impl RetainedDiffApply<RemodelingSnapshot, RemodelingDiff> for RemodelingApply {
    fn new() -> Self {
        Self {
            schema: Default::default(),
            id: Default::default(),
            result_sparse: Default::default(),
            result_dense: Default::default(),
            result_mesh: Default::default(),
            result_trajectory: Default::default(),
            result_tracks: Default::default(),
            result_geo: Default::default(),
            result_qc: Default::default(),
            params: RetainedRowPatchApply::new(),
            diff: None,
            original: None,
            post_identity: None,
            phase: 0,
            commit: 0,
            committing: false,
            closing: false,
        }
    }
    fn preflight_demand(
        &self,
        d: RetainedCloneRef<'_, RemodelingDiff>,
        original: RetainedCloneRef<'_, RemodelingSnapshot>,
        post: RetainedCloneRef<'_, RemodelingSnapshot>,
        body: usize,
        maximum_work_units: u64,
    ) -> Result<RetirementDemand, ValueError> {
        self.preflight_sources(d, original, post)?;
        if maximum_work_units == 0 {
            return Ok(Default::default());
        }
        if self.diff.is_none() {
            return Ok(scalar(d.binding_copy_bytes()));
        }
        if self.original.is_none() {
            return Ok(scalar(original.binding_copy_bytes()));
        }
        if self.post_identity.is_none() {
            return Ok(scalar(size_of::<Option<(u64, usize, usize, usize)>>()));
        }
        match self.phase {
            0 => {
                if d.get().schema.is_some() && self.schema.value().is_none() {
                    self.schema.demand(d.project(0, |d| d.schema.as_ref().expect("declared schema field")), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            1 => {
                if d.get().id.is_some() && self.id.value().is_none() {
                    self.id.demand(d.project(1, |d| d.id.as_ref().expect("declared id field")), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            3 => {
                if d.get().results.as_ref().is_some_and(|r| r.sparse.is_some()) && self.result_sparse.value().is_none() {
                    self.result_sparse.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(0, |r| &r.sparse.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            4 => {
                if d.get().results.as_ref().is_some_and(|r| r.dense.is_some()) && self.result_dense.value().is_none() {
                    self.result_dense.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(1, |r| &r.dense.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            5 => {
                if d.get().results.as_ref().is_some_and(|r| r.mesh.is_some()) && self.result_mesh.value().is_none() {
                    self.result_mesh.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(2, |r| r.mesh.as_ref().expect("declared result slot")), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            6 => {
                if d.get().results.as_ref().is_some_and(|r| r.trajectory.is_some()) && self.result_trajectory.value().is_none() {
                    self.result_trajectory.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(3, |r| &r.trajectory.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            7 => {
                if d.get().results.as_ref().is_some_and(|r| r.tracks.is_some()) && self.result_tracks.value().is_none() {
                    self.result_tracks.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(4, |r| r.tracks.as_ref().expect("declared result slot")), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            8 => {
                if d.get().results.as_ref().is_some_and(|r| r.geo.is_some()) && self.result_geo.value().is_none() {
                    self.result_geo.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(5, |r| &r.geo.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            9 => {
                if d.get().results.as_ref().is_some_and(|r| r.qc.is_some()) && self.result_qc.value().is_none() {
                    self.result_qc.demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(6, |r| &r.qc.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            2 => {
                if d.get().params.is_some() && !self.params.staged() {
                    self.params.stage_demand(d.project(6, |d| d.params.as_ref().expect("declared params field")), original.project(6, |snapshot| &snapshot.params), body, maximum_work_units)
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            10 => Ok(Default::default()),
            _ => Err(invalid()),
        }
    }
    fn preflight_work_demand(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>, post: RetainedCloneRef<'_, RemodelingSnapshot>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.preflight_demand(d, original, post, body, maximum_work_units)?;
        if maximum_work_units == 0 {
            return Ok(0);
        }
        if self.diff.is_none() || self.original.is_none() || self.post_identity.is_none() {
            return Ok(1);
        }
        match self.phase {
            0 => {
                if d.get().schema.is_some() && self.schema.value().is_none() {
                    self.schema.advance_work_demand(d.project(0, |d| d.schema.as_ref().expect("declared schema field")), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            1 => {
                if d.get().id.is_some() && self.id.value().is_none() {
                    self.id.advance_work_demand(d.project(1, |d| d.id.as_ref().expect("declared id field")), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            3 => {
                if d.get().results.as_ref().is_some_and(|r| r.sparse.is_some()) && self.result_sparse.value().is_none() {
                    self.result_sparse.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(0, |r| &r.sparse.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            4 => {
                if d.get().results.as_ref().is_some_and(|r| r.dense.is_some()) && self.result_dense.value().is_none() {
                    self.result_dense.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(1, |r| &r.dense.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            5 => {
                if d.get().results.as_ref().is_some_and(|r| r.mesh.is_some()) && self.result_mesh.value().is_none() {
                    self.result_mesh.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(2, |r| r.mesh.as_ref().expect("declared result slot")), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            6 => {
                if d.get().results.as_ref().is_some_and(|r| r.trajectory.is_some()) && self.result_trajectory.value().is_none() {
                    self.result_trajectory.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(3, |r| &r.trajectory.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            7 => {
                if d.get().results.as_ref().is_some_and(|r| r.tracks.is_some()) && self.result_tracks.value().is_none() {
                    self.result_tracks.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(4, |r| r.tracks.as_ref().expect("declared result slot")), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            8 => {
                if d.get().results.as_ref().is_some_and(|r| r.geo.is_some()) && self.result_geo.value().is_none() {
                    self.result_geo.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(5, |r| &r.geo.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            9 => {
                if d.get().results.as_ref().is_some_and(|r| r.qc.is_some()) && self.result_qc.value().is_none() {
                    self.result_qc.advance_work_demand(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(6, |r| &r.qc.as_ref().expect("declared result slot").value), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            2 => {
                if d.get().params.is_some() && !self.params.staged() {
                    self.params.stage_work_demand(d.project(6, |d| d.params.as_ref().expect("declared params field")), original.project(6, |snapshot| &snapshot.params), body, maximum_work_units)
                } else {
                    Ok(1)
                }
            }
            10 => Ok(0),
            _ => Err(invalid()),
        }
    }
    fn preflight(
        &mut self,
        d: RetainedCloneRef<'_, RemodelingDiff>,
        original: RetainedCloneRef<'_, RemodelingSnapshot>,
        post: RetainedCloneRef<'_, RemodelingSnapshot>,
        body: usize,
        maximum_work_units: u64,
        grant: RetainedCloneGrant,
        _: &mut semio_framework_value::native_decoding::NativeDecodeControl<'_>,
    ) -> RetainedWorkOutcome<RetainedDiffPreflightAdvance, OriginalRetirementFailure> {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.preflight_work_demand(d, original, post, body, maximum_work_units)?;
            if self.phase == 10 {
                return Ok(RetainedDiffPreflightAdvance::Ready(Default::default(), true));
            }
            if selected == 0 || selected > maximum_work_units {
                return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));
            }
            let demand = self.preflight_demand(d, original, post, body, maximum_work_units)?;
            if !retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));
            }
            if self.diff.is_none() {
                let turn = d.bind(&mut self.diff, grant);
                work = turn.work;
                return Ok(RetainedDiffPreflightAdvance::Progress(turn.result?.ok_or_else(invalid)?));
            }
            if self.original.is_none() {
                let turn = original.bind(&mut self.original, grant);
                work = turn.work;
                return Ok(RetainedDiffPreflightAdvance::Progress(turn.result?.ok_or_else(invalid)?));
            }
            if self.post_identity.is_none() {
                self.post_identity = Some(post.reference_identity());
                work = 1;
                return Ok(RetainedDiffPreflightAdvance::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            macro_rules! relay {
                ($turn:expr) => {{
                    let turn = $turn;
                    work = turn.work;
                    return turn.result.map(|step| RetainedDiffPreflightAdvance::Progress(step.progress()));
                }};
            }
            match self.phase {
                0 => {
                    if d.get().schema.is_some() && self.schema.value().is_none() {
                        relay!(self.schema.advance(d.project(0, |d| d.schema.as_ref().expect("declared schema field")), body, maximum_work_units, grant));
                    }
                }
                1 => {
                    if d.get().id.is_some() && self.id.value().is_none() {
                        relay!(self.id.advance(d.project(1, |d| d.id.as_ref().expect("declared id field")), body, maximum_work_units, grant));
                    }
                }
                3 => {
                    if d.get().results.as_ref().is_some_and(|r| r.sparse.is_some()) && self.result_sparse.value().is_none() {
                        relay!(self.result_sparse.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(0, |r| &r.sparse.as_ref().expect("declared result slot").value), body, maximum_work_units, grant));
                    }
                }
                4 => {
                    if d.get().results.as_ref().is_some_and(|r| r.dense.is_some()) && self.result_dense.value().is_none() {
                        relay!(self.result_dense.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(1, |r| &r.dense.as_ref().expect("declared result slot").value), body, maximum_work_units, grant));
                    }
                }
                5 => {
                    if d.get().results.as_ref().is_some_and(|r| r.mesh.is_some()) && self.result_mesh.value().is_none() {
                        relay!(self.result_mesh.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(2, |r| r.mesh.as_ref().expect("declared result slot")), body, maximum_work_units, grant));
                    }
                }
                6 => {
                    if d.get().results.as_ref().is_some_and(|r| r.trajectory.is_some()) && self.result_trajectory.value().is_none() {
                        relay!(self.result_trajectory.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(3, |r| &r.trajectory.as_ref().expect("declared result slot").value), body, maximum_work_units, grant));
                    }
                }
                7 => {
                    if d.get().results.as_ref().is_some_and(|r| r.tracks.is_some()) && self.result_tracks.value().is_none() {
                        relay!(self.result_tracks.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(4, |r| r.tracks.as_ref().expect("declared result slot")), body, maximum_work_units, grant));
                    }
                }
                8 => {
                    if d.get().results.as_ref().is_some_and(|r| r.geo.is_some()) && self.result_geo.value().is_none() {
                        relay!(self.result_geo.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(5, |r| &r.geo.as_ref().expect("declared result slot").value), body, maximum_work_units, grant));
                    }
                }
                9 => {
                    if d.get().results.as_ref().is_some_and(|r| r.qc.is_some()) && self.result_qc.value().is_none() {
                        relay!(self.result_qc.advance(d.project(8, |d| d.results.as_ref().expect("declared results field")).project(6, |r| &r.qc.as_ref().expect("declared result slot").value), body, maximum_work_units, grant));
                    }
                }
                2 => {
                    if d.get().params.is_some() && !self.params.staged() {
                        relay!(self.params.stage(d.project(6, |d| d.params.as_ref().expect("declared params field")), original.project(6, |snapshot| &snapshot.params), body, maximum_work_units, grant));
                    }
                }
                _ => {}
            }
            self.phase += 1;
            work = 1;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() };
            Ok(if self.phase == 10 { RetainedDiffPreflightAdvance::Ready(progress, true) } else { RetainedDiffPreflightAdvance::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }
    fn take_messages(&mut self) -> Vec<protocol::MutationMessage> {
        Vec::new()
    }
    fn commit_demand(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>, _: &RemodelingSnapshot, _: usize) -> Result<RetirementDemand, ValueError> {
        self.commit_sources(d, original)?;
        if !self.committing {
            return Ok(scalar(size_of::<bool>()));
        }
        match self.commit {
            0 => {
                if d.get().schema.is_some() && self.schema.value().is_some() {
                    self.schema.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            1 => {
                if d.get().id.is_some() && self.id.value().is_some() {
                    self.id.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            3 => {
                if d.get().results.as_ref().is_some_and(|r| r.sparse.is_some()) && self.result_sparse.value().is_some() {
                    self.result_sparse.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            4 => {
                if d.get().results.as_ref().is_some_and(|r| r.dense.is_some()) && self.result_dense.value().is_some() {
                    self.result_dense.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            5 => {
                if d.get().results.as_ref().is_some_and(|r| r.mesh.is_some()) && self.result_mesh.value().is_some() {
                    self.result_mesh.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            6 => {
                if d.get().results.as_ref().is_some_and(|r| r.trajectory.is_some()) && self.result_trajectory.value().is_some() {
                    self.result_trajectory.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            7 => {
                if d.get().results.as_ref().is_some_and(|r| r.tracks.is_some()) && self.result_tracks.value().is_some() {
                    self.result_tracks.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            8 => {
                if d.get().results.as_ref().is_some_and(|r| r.geo.is_some()) && self.result_geo.value().is_some() {
                    self.result_geo.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            9 => {
                if d.get().results.as_ref().is_some_and(|r| r.qc.is_some()) && self.result_qc.value().is_some() {
                    self.result_qc.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            2 => {
                if d.get().params.is_some() && !self.params.complete() {
                    self.params.exchange_demand()
                } else {
                    Ok(scalar(size_of::<usize>()))
                }
            }
            10 => Ok(Default::default()),
            _ => Err(invalid()),
        }
    }
    fn commit_work_demand(&self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>, post: &RemodelingSnapshot, body: usize) -> Result<u64, ValueError> {
        self.commit_demand(d, original, post, body)?;
        if !self.committing {
            return Ok(1);
        }
        if self.commit == 10 {
            return Ok(0);
        }
        if self.commit == 2 && d.get().params.is_some() && !self.params.complete() {
            self.params.exchange_work_demand()
        } else {
            Ok(1)
        }
    }
    fn commit(&mut self, d: RetainedCloneRef<'_, RemodelingDiff>, original: RetainedCloneRef<'_, RemodelingSnapshot>, post: &mut RemodelingSnapshot, grant: RetainedCloneGrant, capability: protocol::ApplyCapability) -> OriginalRetirementOutcome {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let demand = self.commit_demand(d, original, post, grant.maximum_copy_bytes)?;
            if self.commit == 10 {
                return Ok(RetainedCloneStep::Complete(Default::default()));
            }
            if !retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if !self.committing {
                self.committing = true;
                work = 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() }));
            }
            match self.commit {
                0 => {
                    if d.get().schema.is_some() && self.schema.value().is_some() {
                        work = 1;
                        return self.schema.install(&mut post.schema, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                1 => {
                    if d.get().id.is_some() && self.id.value().is_some() {
                        work = 1;
                        return self.id.install(&mut post.id, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                3 => {
                    if d.get().results.as_ref().is_some_and(|r| r.sparse.is_some()) && self.result_sparse.value().is_some() {
                        work = 1;
                        return self.result_sparse.install(&mut post.results.sparse, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                4 => {
                    if d.get().results.as_ref().is_some_and(|r| r.dense.is_some()) && self.result_dense.value().is_some() {
                        work = 1;
                        return self.result_dense.install(&mut post.results.dense, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                5 => {
                    if d.get().results.as_ref().is_some_and(|r| r.mesh.is_some()) && self.result_mesh.value().is_some() {
                        work = 1;
                        return self.result_mesh.install(&mut post.results.mesh, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                6 => {
                    if d.get().results.as_ref().is_some_and(|r| r.trajectory.is_some()) && self.result_trajectory.value().is_some() {
                        work = 1;
                        return self.result_trajectory.install(&mut post.results.trajectory, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                7 => {
                    if d.get().results.as_ref().is_some_and(|r| r.tracks.is_some()) && self.result_tracks.value().is_some() {
                        work = 1;
                        return self.result_tracks.install(&mut post.results.tracks, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                8 => {
                    if d.get().results.as_ref().is_some_and(|r| r.geo.is_some()) && self.result_geo.value().is_some() {
                        work = 1;
                        return self.result_geo.install(&mut post.results.geo, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                9 => {
                    if d.get().results.as_ref().is_some_and(|r| r.qc.is_some()) && self.result_qc.value().is_some() {
                        work = 1;
                        return self.result_qc.install(&mut post.results.qc, capability, grant).map(RetainedCloneStep::Progress).map_err(Into::into);
                    }
                }
                2 => {
                    if d.get().params.is_some() && !self.params.complete() {
                        let turn = self.params.exchange(&mut post.params, capability, grant);
                        work = turn.work;
                        return turn.result;
                    }
                }
                _ => {}
            }
            self.commit += 1;
            work = 1;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() };
            Ok(if self.commit == 10 { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }
    fn begin_close(&mut self) {
        self.closing = true;
        self.schema.begin_close();
        self.id.begin_close();
        self.result_sparse.begin_close();
        self.result_dense.begin_close();
        self.result_mesh.begin_close();
        self.result_trajectory.begin_close();
        self.result_tracks.begin_close();
        self.result_geo.begin_close();
        self.result_qc.begin_close();
        self.params.begin_close();
    }
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing {
            return Err(invalid());
        }
        if !self.schema.terminal_is_empty() {
            return self.schema.close_demand(body);
        }
        if !self.id.terminal_is_empty() {
            return self.id.close_demand(body);
        }
        if !self.result_sparse.terminal_is_empty() {
            return self.result_sparse.close_demand(body);
        }
        if !self.result_dense.terminal_is_empty() {
            return self.result_dense.close_demand(body);
        }
        if !self.result_mesh.terminal_is_empty() {
            return self.result_mesh.close_demand(body);
        }
        if !self.result_trajectory.terminal_is_empty() {
            return self.result_trajectory.close_demand(body);
        }
        if !self.result_tracks.terminal_is_empty() {
            return self.result_tracks.close_demand(body);
        }
        if !self.result_geo.terminal_is_empty() {
            return self.result_geo.close_demand(body);
        }
        if !self.result_qc.terminal_is_empty() {
            return self.result_qc.close_demand(body);
        }
        if !self.params.terminal_is_empty() {
            return self.params.close_demand(body);
        }
        RetainedCloneBinding::close_demand(self.closing_binding(), body)
    }
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        if !self.schema.terminal_is_empty() {
            return self.schema.close_work_demand(body);
        }
        if !self.id.terminal_is_empty() {
            return self.id.close_work_demand(body);
        }
        if !self.result_sparse.terminal_is_empty() {
            return self.result_sparse.close_work_demand(body);
        }
        if !self.result_dense.terminal_is_empty() {
            return self.result_dense.close_work_demand(body);
        }
        if !self.result_mesh.terminal_is_empty() {
            return self.result_mesh.close_work_demand(body);
        }
        if !self.result_trajectory.terminal_is_empty() {
            return self.result_trajectory.close_work_demand(body);
        }
        if !self.result_tracks.terminal_is_empty() {
            return self.result_tracks.close_work_demand(body);
        }
        if !self.result_geo.terminal_is_empty() {
            return self.result_geo.close_work_demand(body);
        }
        if !self.result_qc.terminal_is_empty() {
            return self.result_qc.close_work_demand(body);
        }
        if !self.params.terminal_is_empty() {
            return self.params.close_work_demand(body);
        }
        RetainedCloneBinding::close_work_demand(self.closing_binding(), body)
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error.into()), work: 0 },
        };
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 };
        }
        if !self.schema.terminal_is_empty() {
            return self.schema.close_step(grant);
        }
        if !self.id.terminal_is_empty() {
            return self.id.close_step(grant);
        }
        if !self.result_sparse.terminal_is_empty() {
            return self.result_sparse.close_step(grant);
        }
        if !self.result_dense.terminal_is_empty() {
            return self.result_dense.close_step(grant);
        }
        if !self.result_mesh.terminal_is_empty() {
            return self.result_mesh.close_step(grant);
        }
        if !self.result_trajectory.terminal_is_empty() {
            return self.result_trajectory.close_step(grant);
        }
        if !self.result_tracks.terminal_is_empty() {
            return self.result_tracks.close_step(grant);
        }
        if !self.result_geo.terminal_is_empty() {
            return self.result_geo.close_step(grant);
        }
        if !self.result_qc.terminal_is_empty() {
            return self.result_qc.close_step(grant);
        }
        if !self.params.terminal_is_empty() {
            return self.params.close_step(grant);
        }
        RetainedCloneBinding::close_one(if self.original.is_some() { &mut self.original } else { &mut self.diff }, grant)
    }
    fn terminal_is_empty(&self) -> bool {
        self.schema.terminal_is_empty()
            && self.id.terminal_is_empty()
            && self.result_sparse.terminal_is_empty()
            && self.result_dense.terminal_is_empty()
            && self.result_mesh.terminal_is_empty()
            && self.result_trajectory.terminal_is_empty()
            && self.result_tracks.terminal_is_empty()
            && self.result_geo.terminal_is_empty()
            && self.result_qc.terminal_is_empty()
            && self.params.terminal_is_empty()
            && self.diff.is_none()
            && self.original.is_none()
    }
}
impl Drop for RemodelingApply {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "remodeling retained diff requires complete owned field and source closure");
    }
}
