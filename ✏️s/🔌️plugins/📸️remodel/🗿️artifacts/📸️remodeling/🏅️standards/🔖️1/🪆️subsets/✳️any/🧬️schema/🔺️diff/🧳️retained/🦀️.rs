//! 🧬️ Original assigned fields stage under real grants and install only with the central mutation capability.
use super::*;
use protocol::{retained_native_child as native_child, retained_routed, RetainedDiff, RetainedDiffApply, RetainedDiffPreflightAdvance, RetainedDone as Done, RetainedFieldPreparation, RetainedFlight as Flight, RetainedReply as Reply, RetainedRowPatchApply, retained_conclude as conclude, retained_settle as settle};
use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedClone, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{NativeDecodeControl, OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;
use std::marker::PhantomData;
use std::mem::{size_of, ManuallyDrop};

impl RetainedDiff<RemodelingSnapshot> for RemodelingDiff {
    type Apply = RemodelingApply;
}

const SLOTS: usize = 10;
type Diff<'a> = RetainedCloneRef<'a, RemodelingDiff>;
type Base<'a> = RetainedCloneRef<'a, RemodelingSnapshot>;

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "remodeling retained assigned-field phase or source changed")
}
fn scalar(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}
fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.copy_bytes = demand.copy_bytes.checked_add(1).ok_or_else(invalid)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(invalid)?;
    Ok(demand)
}
fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(1), maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}
fn tagged(error: OriginalRetirementFailure, paid: bool) -> OriginalRetirementFailure {
    if !paid {
        return error;
    }
    let mut progress = error.retained_progress();
    progress.copied_bytes = progress.copied_bytes.saturating_add(1);
    error.with_retained_progress(progress)
}

trait Spec: 'static {
    type Ty: RetainedClone + RetireOwned + Send + Sync + 'static;
    fn present(diff: &RemodelingDiff) -> bool;
    fn source(diff: Diff<'_>) -> RetainedCloneRef<'_, Self::Ty>;
    fn target(snapshot: &mut RemodelingSnapshot) -> &mut Self::Ty;
}

macro_rules! spec {
    ($name:ident, $ty:ty, |$d:ident| $present:expr, |$r:ident| $source:expr, |$p:ident| $target:expr) => {
        struct $name;
        impl Spec for $name {
            type Ty = $ty;
            fn present($d: &RemodelingDiff) -> bool {
                $present
            }
            fn source($r: Diff<'_>) -> RetainedCloneRef<'_, $ty> {
                $source
            }
            fn target($p: &mut RemodelingSnapshot) -> &mut $ty {
                $target
            }
        }
    };
}

spec!(SchemaSpec, String, |d| d.schema.is_some(), |d| d.project(0, |d| d.schema.as_ref().expect("declared schema field")), |p| &mut p.schema);
spec!(IdSpec, String, |d| d.id.is_some(), |d| d.project(1, |d| d.id.as_ref().expect("declared id field")), |p| &mut p.id);
spec!(SparseSpec, Option<SparseCloud>, |d| d.results.as_ref().is_some_and(|r| r.sparse.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(0, |r| &r.sparse.as_ref().expect("declared result slot").value), |p| &mut p.results.sparse);
spec!(DenseSpec, Option<DenseCloud>, |d| d.results.as_ref().is_some_and(|r| r.dense.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(1, |r| &r.dense.as_ref().expect("declared result slot").value), |p| &mut p.results.dense);
spec!(MeshSpec, RemodelingMesh, |d| d.results.as_ref().is_some_and(|r| r.mesh.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(2, |r| r.mesh.as_ref().expect("declared result slot")), |p| &mut p.results.mesh);
spec!(TrajectorySpec, Option<CameraTrajectory>, |d| d.results.as_ref().is_some_and(|r| r.trajectory.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(3, |r| &r.trajectory.as_ref().expect("declared result slot").value), |p| &mut p.results.trajectory);
spec!(TracksSpec, Vec<MotionTrackSummary>, |d| d.results.as_ref().is_some_and(|r| r.tracks.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(4, |r| r.tracks.as_ref().expect("declared result slot")), |p| &mut p.results.tracks);
spec!(GeoSpec, Option<GeoProducts>, |d| d.results.as_ref().is_some_and(|r| r.geo.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(5, |r| &r.geo.as_ref().expect("declared result slot").value), |p| &mut p.results.geo);
spec!(QcSpec, Option<QcReportSnapshot>, |d| d.results.as_ref().is_some_and(|r| r.qc.is_some()), |d| d.project(8, |d| d.results.as_ref().expect("declared results field")).project(6, |r| &r.qc.as_ref().expect("declared result slot").value), |p| &mut p.results.qc);

/// 🧩️ One assigned snapshot slot: a field preparation, the staged owned value and the displaced original it exchanges for.
trait Stage: Send {
    fn present(&self, diff: &RemodelingDiff) -> bool;
    fn staged(&self) -> bool;
    fn stage_demand(&self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError>;
    fn stage_work_demand(&self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64) -> Result<u64, ValueError>;
    fn stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError>;
    fn recover_stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError>;
    fn exchange_demand(&self) -> Result<RetirementDemand, ValueError>;
    fn exchange_work_demand(&self) -> Result<u64, ValueError>;
    fn exchange(&mut self, snapshot: &mut RemodelingSnapshot, capability: protocol::ApplyCapability, grant: RetainedCloneGrant) -> OriginalRetirementOutcome;
    fn begin_close(&mut self);
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError>;
    fn requires_original_birth(&self) -> bool;
    fn requires_original_control(&self) -> bool;
    fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll;
    fn close_reply<'r>(&mut self, original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Done<'r>, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

struct FieldSlot<S: Spec> {
    prep: RetainedFieldPreparation<S::Ty>,
    staged: ManuallyDrop<Option<S::Ty>>,
    retirement: RetainedCloneClose,
    flight: Flight,
    spec: PhantomData<fn() -> S>,
}

impl<S: Spec> Default for FieldSlot<S> {
    fn default() -> Self {
        Self { prep: Default::default(), staged: ManuallyDrop::new(None), retirement: Default::default(), flight: Default::default(), spec: PhantomData }
    }
}

impl<S: Spec> Drop for FieldSlot<S> {
    fn drop(&mut self) {
        let empty = Stage::terminal_is_empty(self);
        assert!(std::thread::panicking() || empty, "remodeling retained field requires complete owned closure");
        if empty {
            unsafe { ManuallyDrop::drop(&mut self.staged) };
        }
    }
}

impl<S: Spec> FieldSlot<S> {
    fn stage_reply<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Done<'r>, ValueError> {
        let grant = effect.grant();
        let base_work = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<RetainedCloneStep, RetirementPollExit> = (|| {
            if !recover {
                if !self.flight.idle() {
                    return Err(invalid().into());
                }
                if self.stage_work_demand(diff, base, body, maximum)? == 0 {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if !retained_clone_grant_admits(grant, self.stage_demand(diff, base, body, maximum)?) {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if self.prep.value().is_some() {
                    let slot = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(size_of::<Option<S::Ty>>()), ..grant };
                    let step = native_child(effect, &mut work, |e, r| self.prep.take_into(&mut self.staged, &mut e.nested(slot), r))??;
                    let mut progress = step.progress();
                    progress.copied_bytes += size_of::<Option<S::Ty>>();
                    return Ok(if self.staged.is_some() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) });
                }
            }
            let source = S::source(diff);
            let step = retained_routed!(
                self.flight,
                effect,
                recover,
                1,
                body,
                maximum,
                None,
                work,
                |r| self.prep.advance_into(source, body, maximum, &mut effect.nested(grant), r),
                self.prep.recover_advance_into(source, body, maximum, &mut effect.nested(grant), r)
            );
            Ok(RetainedCloneStep::Progress(step?.progress()))
        })();
        conclude(effect, base_work, result, work, |step| step.progress(), reply)
    }
}

impl<S: Spec> Stage for FieldSlot<S> {
    fn present(&self, diff: &RemodelingDiff) -> bool {
        S::present(diff)
    }
    fn staged(&self) -> bool {
        self.staged.is_some()
    }
    fn stage_demand(&self, diff: Diff<'_>, _: Base<'_>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        if self.prep.value().is_some() {
            let mut demand = self.prep.exchange_demand()?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<S::Ty>>()).ok_or_else(invalid)?;
            return Ok(demand);
        }
        self.prep.demand(S::source(diff), body, maximum)
    }
    fn stage_work_demand(&self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        self.stage_demand(diff, base, body, maximum)?;
        if self.prep.value().is_some() {
            return Ok(1);
        }
        self.prep.advance_work_demand(S::source(diff), body, maximum)
    }
    fn stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError> {
        self.stage_reply(diff, base, body, maximum, effect, reply, false)
    }
    fn recover_stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError> {
        self.stage_reply(diff, base, body, maximum, effect, reply, true)
    }
    fn exchange_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(scalar(2 * size_of::<S::Ty>()))
    }
    fn exchange_work_demand(&self) -> Result<u64, ValueError> {
        Ok(1)
    }
    fn exchange(&mut self, snapshot: &mut RemodelingSnapshot, _: protocol::ApplyCapability, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let demand = match self.exchange_demand() {
            Ok(demand) => demand,
            Err(error) => return semio_framework_value::RetainedWorkOutcome { result: Err(error.into()), work: 0 },
        };
        if !retained_clone_grant_admits(grant, demand) {
            return semio_framework_value::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 };
        }
        let Some(value) = self.staged.as_mut() else {
            return semio_framework_value::RetainedWorkOutcome { result: Err(invalid().into()), work: 0 };
        };
        std::mem::swap(S::target(snapshot), value);
        semio_framework_value::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })), work: 1 }
    }
    fn begin_close(&mut self) {}
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.prep.terminal_is_empty() {
            return if self.prep.close_started() { self.prep.close_demand(body) } else { Ok(self.prep.begin_close_demand()) };
        }
        self.retirement.next_owner_demands::<S::Ty>(self.staged.is_some() && self.retirement.is_empty(), &None, body)
    }
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        if !self.prep.terminal_is_empty() {
            return if self.prep.close_started() { self.prep.close_work_demand(body) } else { Ok(self.prep.begin_close_work_demand()) };
        }
        self.retirement.next_owner_work::<S::Ty>(self.staged.is_some() && self.retirement.is_empty(), &None, body)
    }
    fn requires_original_birth(&self) -> bool {
        if !self.flight.idle() {
            return false;
        }
        if !self.prep.terminal_is_empty() {
            return self.prep.close_started() && self.prep.requires_original_birth();
        }
        !self.retirement.is_empty() && self.retirement.requires_original_birth()
    }
    fn requires_original_control(&self) -> bool {
        if let Some(loaned) = self.flight.loaned() {
            return loaned;
        }
        if !self.prep.terminal_is_empty() {
            return self.prep.close_started() && self.prep.requires_original_control();
        }
        !self.retirement.is_empty() && self.retirement.requires_original_control()
    }
    fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        if !self.prep.terminal_is_empty() {
            return self.prep.close_original_birth_step(original, grant);
        }
        self.retirement.step_original_birth(original, grant)
    }
    fn close_reply<'r>(&mut self, mut original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Done<'r>, ValueError> {
        let grant = effect.grant();
        let base_work = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<RetainedCloneStep, RetirementPollExit> = (|| {
            let route: u8;
            let loaned: bool;
            if recover {
                route = self.flight.route().ok_or_else(invalid)?;
                loaned = self.flight.loaned().unwrap_or(false);
            } else {
                if !self.flight.idle() {
                    return Err(invalid().into());
                }
                if !protocol::retained_mutation_demand_fits(self.close_demand(body)?, grant) {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if self.requires_original_birth() {
                    return Err(invalid().into());
                }
                loaned = self.requires_original_control();
                if loaned && original.is_none() {
                    return Err(invalid().into());
                }
                if !self.prep.terminal_is_empty() {
                    if !self.prep.close_started() {
                        let turn = self.prep.begin_close_paid(effect);
                        work += turn.work;
                        return Ok(RetainedCloneStep::Progress(turn.result?));
                    }
                    route = 1;
                } else if !self.retirement.is_empty() {
                    route = 2;
                } else if self.staged.is_some() {
                    let turn = self.retirement.begin_granted(&mut self.staged, effect);
                    work += turn.work;
                    return Ok(RetainedCloneStep::Progress(turn.result?.map(|step| step.progress()).unwrap_or_default()));
                } else {
                    return Ok(RetainedCloneStep::Complete(Default::default()));
                }
            }
            let loan = if loaned { original.as_deref_mut() } else { None };
            let missing = || ValueError::invariant("remodeling field close lost its original controller");
            let step = retained_routed!(
                self.flight,
                effect,
                recover,
                route,
                body,
                0,
                loan,
                work,
                |r| match (route, loaned) {
                    (1, false) => self.prep.close_into(body, effect, r),
                    (1, true) => self.prep.close_original_into(original.as_deref_mut().ok_or_else(missing)?, body, effect, r),
                    (2, false) => self.retirement.step_into(body, effect, r),
                    (2, true) => self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?, body, effect, r),
                    _ => Err(invalid()),
                },
                match (route, loaned) {
                    (1, false) => self.prep.recover_close_into(body, effect, r),
                    (1, true) => self.prep.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?, body, effect, r),
                    (2, false) => self.retirement.recover_step_into(body, effect, r),
                    (2, true) => self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?, body, effect, r),
                    _ => Err(invalid()),
                }
            );
            Ok(RetainedCloneStep::Progress(step?.progress()))
        })();
        conclude(effect, base_work, result, work, |step| step.progress(), reply)
    }
    fn terminal_is_empty(&self) -> bool {
        self.prep.terminal_is_empty() && self.staged.is_none() && self.retirement.is_empty() && self.flight.idle()
    }
}

#[derive(Default)]
struct ParamsSlot(RetainedRowPatchApply<ReconstructionParams, RemodelingParamsDiff>);

fn params_of<'a>(diff: Diff<'a>) -> RetainedCloneRef<'a, RemodelingParamsDiff> {
    diff.project(6, |d| d.params.as_ref().expect("declared params field"))
}
fn params_base<'a>(base: Base<'a>) -> RetainedCloneRef<'a, ReconstructionParams> {
    base.project(6, |snapshot| &snapshot.params)
}

impl Stage for ParamsSlot {
    fn present(&self, diff: &RemodelingDiff) -> bool {
        diff.params.is_some()
    }
    fn staged(&self) -> bool {
        self.0.staged()
    }
    fn stage_demand(&self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        self.0.stage_demand(params_of(diff), params_base(base), body, maximum)
    }
    fn stage_work_demand(&self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        self.0.stage_work_demand(params_of(diff), params_base(base), body, maximum)
    }
    fn stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError> {
        self.0.stage_into(params_of(diff), params_base(base), body, maximum, effect, reply)
    }
    fn recover_stage_into<'r>(&mut self, diff: Diff<'_>, base: Base<'_>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Done<'r>, ValueError> {
        self.0.recover_stage_into(params_of(diff), params_base(base), body, maximum, effect, reply)
    }
    fn exchange_demand(&self) -> Result<RetirementDemand, ValueError> {
        self.0.exchange_demand()
    }
    fn exchange_work_demand(&self) -> Result<u64, ValueError> {
        self.0.exchange_work_demand()
    }
    fn exchange(&mut self, snapshot: &mut RemodelingSnapshot, capability: protocol::ApplyCapability, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        self.0.exchange(&mut snapshot.params, capability, grant)
    }
    fn begin_close(&mut self) {
        self.0.begin_close();
    }
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.0.close_demand(body)
    }
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.0.close_work_demand(body)
    }
    fn requires_original_birth(&self) -> bool {
        self.0.requires_original_birth()
    }
    fn requires_original_control(&self) -> bool {
        self.0.requires_original_control()
    }
    fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        self.0.close_original_birth_step(original, grant)
    }
    fn close_reply<'r>(&mut self, original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Done<'r>, ValueError> {
        self.0.close_reply(original, body, effect, reply, recover)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.terminal_is_empty()
    }
}

/// 🪡️ Every assigned field stages before the first central write; each owned slot and displaced original closes independently.
#[derive(Default)]
pub struct RemodelingApply {
    schema: FieldSlot<SchemaSpec>,
    id: FieldSlot<IdSpec>,
    sparse: FieldSlot<SparseSpec>,
    dense: FieldSlot<DenseSpec>,
    mesh: FieldSlot<MeshSpec>,
    trajectory: FieldSlot<TrajectorySpec>,
    tracks: FieldSlot<TracksSpec>,
    geo: FieldSlot<GeoSpec>,
    qc: FieldSlot<QcSpec>,
    params: ParamsSlot,
    flight: Flight,
    phase: usize,
    commit: usize,
}

impl RemodelingApply {
    fn slot(&self, index: usize) -> &dyn Stage {
        match index {
            0 => &self.schema,
            1 => &self.id,
            2 => &self.sparse,
            3 => &self.dense,
            4 => &self.mesh,
            5 => &self.trajectory,
            6 => &self.tracks,
            7 => &self.geo,
            8 => &self.qc,
            _ => &self.params,
        }
    }
    fn slot_mut(&mut self, index: usize) -> &mut dyn Stage {
        match index {
            0 => &mut self.schema,
            1 => &mut self.id,
            2 => &mut self.sparse,
            3 => &mut self.dense,
            4 => &mut self.mesh,
            5 => &mut self.trajectory,
            6 => &mut self.tracks,
            7 => &mut self.geo,
            8 => &mut self.qc,
            _ => &mut self.params,
        }
    }
    fn open(&self) -> Option<usize> {
        (0..SLOTS).find(|index| !self.slot(*index).terminal_is_empty())
    }
}

impl RetainedDiffApply<RemodelingSnapshot, RemodelingDiff> for RemodelingApply {
    fn new() -> Self {
        Self::default()
    }
    fn preflight_demand(&self, d: Diff<'_>, original: Base<'_>, _: Base<'_>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.phase > SLOTS {
            return Err(invalid());
        }
        if maximum_work_units == 0 || self.phase == SLOTS {
            return Ok(Default::default());
        }
        let slot = self.slot(self.phase);
        if slot.present(d.get()) {
            nested(slot.stage_demand(d, original, body.saturating_sub(1), maximum_work_units)?)
        } else {
            Ok(scalar(size_of::<usize>()))
        }
    }
    fn preflight_work_demand(&self, d: Diff<'_>, original: Base<'_>, post: Base<'_>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.preflight_demand(d, original, post, body, maximum_work_units)?;
        if maximum_work_units == 0 || self.phase == SLOTS {
            return Ok(0);
        }
        let slot = self.slot(self.phase);
        if slot.present(d.get()) {
            slot.stage_work_demand(d, original, body.saturating_sub(1), maximum_work_units)
        } else {
            Ok(1)
        }
    }
    fn preflight_reply<'r>(
        &mut self,
        d: Diff<'_>,
        original: Base<'_>,
        post: Base<'_>,
        body: usize,
        maximum_work_units: u64,
        effect: &mut RetainedAllocationEffect<'_>,
        _: &mut NativeDecodeControl<'_>,
        reply: Reply<'r, RetainedDiffPreflightAdvance>,
        recover: bool,
    ) -> Result<Done<'r, RetainedDiffPreflightAdvance>, ValueError> {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<RetainedDiffPreflightAdvance, RetirementPollExit> = (|| {
            let route: u8 = if recover {
                self.flight.route().ok_or_else(invalid)?
            } else {
                if !self.flight.idle() {
                    return Err(invalid().into());
                }
                if self.phase == SLOTS {
                    return Ok(RetainedDiffPreflightAdvance::Ready(Default::default(), true));
                }
                if self.preflight_work_demand(d, original, post, body, maximum_work_units)? == 0 {
                    return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));
                }
                if !retained_clone_grant_admits(grant, self.preflight_demand(d, original, post, body, maximum_work_units)?) {
                    return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));
                }
                if !self.slot(self.phase).present(d.get()) {
                    work += 1;
                    self.phase += 1;
                    let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() };
                    return Ok(if self.phase == SLOTS { RetainedDiffPreflightAdvance::Ready(progress, true) } else { RetainedDiffPreflightAdvance::Progress(progress) });
                }
                (self.phase + 1) as u8
            };
            let index = usize::from(route) - 1;
            let nested_body = body.saturating_sub(1);
            let before = work;
            let step = retained_routed!(
                self.flight,
                effect,
                recover,
                route,
                body,
                maximum_work_units,
                None,
                work,
                |r| self.slot_mut(index).stage_into(d, original, nested_body, maximum_work_units, &mut effect.nested(child(grant)), r),
                self.slot_mut(index).recover_stage_into(d, original, nested_body, maximum_work_units, &mut effect.nested(child(grant)), r)
            );
            let step = step.map_err(|error| tagged(error, work != before))?;
            let mut progress = step.progress();
            progress.copied_bytes += 1;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.phase += 1;
            }
            Ok(if self.phase == SLOTS { RetainedDiffPreflightAdvance::Ready(progress, true) } else { RetainedDiffPreflightAdvance::Progress(progress) })
        })();
        conclude(effect, base, result, work, RetainedDiffPreflightAdvance::progress, reply)
    }
    fn take_messages(&mut self) -> Vec<protocol::MutationMessage> {
        Vec::new()
    }
    fn commit_demand(&self, d: Diff<'_>, _: Base<'_>, _: &RemodelingSnapshot, _: usize) -> Result<RetirementDemand, ValueError> {
        if self.phase != SLOTS || self.commit > SLOTS {
            return Err(invalid());
        }
        if self.commit == SLOTS {
            return Ok(Default::default());
        }
        let slot = self.slot(self.commit);
        if slot.present(d.get()) && slot.staged() {
            nested(slot.exchange_demand()?)
        } else {
            Ok(scalar(size_of::<usize>()))
        }
    }
    fn commit_work_demand(&self, d: Diff<'_>, original: Base<'_>, post: &RemodelingSnapshot, body: usize) -> Result<u64, ValueError> {
        self.commit_demand(d, original, post, body)?;
        if self.commit == SLOTS {
            return Ok(0);
        }
        let slot = self.slot(self.commit);
        if slot.present(d.get()) && slot.staged() {
            slot.exchange_work_demand()
        } else {
            Ok(1)
        }
    }
    fn commit(&mut self, d: Diff<'_>, original: Base<'_>, post: &mut RemodelingSnapshot, effect: &mut RetainedAllocationEffect<'_>, capability: protocol::ApplyCapability) -> OriginalRetirementOutcome {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<RetainedCloneStep, OriginalRetirementFailure> = (|| {
            let demand = self.commit_demand(d, original, post, grant.maximum_copy_bytes)?;
            if self.commit == SLOTS {
                return Ok(RetainedCloneStep::Complete(Default::default()));
            }
            if !retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            let index = self.commit;
            let slot = self.slot_mut(index);
            if slot.present(d.get()) && slot.staged() {
                let turn = slot.exchange(post, capability, child(grant));
                work = turn.work;
                let step = turn.result.map_err(|error| tagged(error, turn.work != 0))?;
                let mut progress = step.progress();
                progress.copied_bytes += 1;
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.commit += 1;
                }
                return Ok(if self.commit == SLOTS { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) });
            }
            work = 1;
            self.commit += 1;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() };
            Ok(if self.commit == SLOTS { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        settle(effect, base, result, work)
    }
    fn begin_close(&mut self) {
        (0..SLOTS).for_each(|index| self.slot_mut(index).begin_close());
    }
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.open() {
            Some(index) => nested(self.slot(index).close_demand(body.saturating_sub(1))?),
            None => Ok(Default::default()),
        }
    }
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        match self.open() {
            Some(index) => self.slot(index).close_work_demand(body.saturating_sub(1)),
            None => Ok(0),
        }
    }
    fn requires_original_birth(&self) -> bool {
        self.flight.idle() && self.open().is_some_and(|index| self.slot(index).requires_original_birth())
    }
    fn requires_original_control(&self) -> bool {
        if let Some(loaned) = self.flight.loaned() {
            return loaned;
        }
        self.open().is_some_and(|index| self.slot(index).requires_original_control())
    }
    fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        match self.open() {
            Some(index) => self.slot_mut(index).close_original_birth_step(original, grant),
            None => semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: Err(invalid().into()), work: 0 }),
        }
    }
    fn close_reply<'r>(&mut self, mut original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Done<'r>, ValueError> {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<RetainedCloneStep, RetirementPollExit> = (|| {
            let route: u8;
            let loaned: bool;
            if recover {
                route = self.flight.route().ok_or_else(invalid)?;
                loaned = self.flight.loaned().unwrap_or(false);
            } else {
                if !self.flight.idle() {
                    return Err(invalid().into());
                }
                let Some(index) = self.open() else {
                    return Ok(RetainedCloneStep::Complete(Default::default()));
                };
                if !protocol::retained_mutation_demand_fits(self.close_demand(body)?, grant) {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if self.requires_original_birth() {
                    return Err(invalid().into());
                }
                loaned = self.requires_original_control();
                if loaned && original.is_none() {
                    return Err(invalid().into());
                }
                route = (index + 1) as u8;
            }
            let index = usize::from(route) - 1;
            let nested_body = body.saturating_sub(1);
            let before = work;
            let loan = if loaned { original.as_deref_mut() } else { None };
            let step = retained_routed!(
                self.flight,
                effect,
                recover,
                route,
                body,
                0,
                loan,
                work,
                |r| self.slot_mut(index).close_reply(if loaned { original.as_deref_mut() } else { None }, nested_body, &mut effect.nested(child(grant)), r, false),
                self.slot_mut(index).close_reply(if loaned { original.as_deref_mut() } else { None }, nested_body, &mut effect.nested(child(grant)), r, true)
            );
            let step = step.map_err(|error| tagged(error, work != before))?;
            let mut progress = step.progress();
            progress.copied_bytes += 1;
            Ok(RetainedCloneStep::Progress(progress))
        })();
        conclude(effect, base, result, work, |step| step.progress(), reply)
    }
    fn terminal_is_empty(&self) -> bool {
        self.open().is_none() && self.flight.idle()
    }
}
