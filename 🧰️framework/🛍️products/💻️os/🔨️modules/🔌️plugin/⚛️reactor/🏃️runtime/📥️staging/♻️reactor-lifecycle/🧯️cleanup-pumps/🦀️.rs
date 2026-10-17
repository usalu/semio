//! 🧯️ Live and close cleanup pumps and jobs of the runtime, and the instance close admission.

use semio_framework_plugin_decl_app::ArtifactApp;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::cell::Cell;
use semio_framework_plugin_job::reserved_job_outcomes::CleanupTurn;
use semio_framework::{Fault, FaultFrom};
use semio_framework_value::{FromValue, RetirementDemand, ToValue};
use semio_framework_plugin_reactor_app::app_runtime_helpers::{LAST_MAINTENANCE_STAGE, debug_runtime_line, plugin_internal_fault};
use semio_framework_plugin_mounted_owner::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerTurnV1};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_plugin_program::PluginProgram;
use crate::life_instance_registry::{PluginRuntime, RuntimeActorAuthority, RuntimeCloseEntry, RuntimeCloseWorkerState};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use crate::life_app_cell::{RuntimeAppCell, RuntimeInstanceRefusal};
use crate::life_cleanup_vocabulary::{RuntimeCloseStatus, RuntimeMaintenanceStatus};
use std::mem::size_of_val;
/// 🧳️ Keeps the exact opening inputs and partial app through every original caller admission turn.
#[doc(hidden)]
pub struct PendingNativeCreation<PA:PluginApp>{pub request:semio_framework::kernel::ActorInstanceOpenRequest,pub source:std::mem::ManuallyDrop<Option<(String,String)>>,pub app:std::mem::ManuallyDrop<Option<PA>>,pub loan:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,pub identity:(semio_framework_job::OperationId,semio_framework_job::Generation),pub parent:usize,pub returning_loan:bool,pub authority:Option<RuntimeActorAuthority>,pub issuer_ready:bool,pub closing:bool,pub failure:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,pub metadata_source:std::mem::ManuallyDrop<Option<String>>,pub metadata_closing:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>>,pub source_closing:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,String)>>>}
impl<PA:PluginApp> PendingNativeCreation<PA>{
 pub fn validate(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if self.identity!=(cx.operation(),cx.generation())||self.parent!=cx.original_source_parent()?.source_identity()?{return Err(plugin_internal_fault("original opening continuation requires its same actor creation source").into())}if !self.returning_loan{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
}
#[doc(hidden)]
pub struct RuntimeLiveCleanupPump<PA: PluginApp> {
    pub session: Option<semio_framework_job_worker::BatchJobSession<RuntimeLiveCleanupJob<PA>>>,
    pub rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<RuntimeLiveCleanupJob<PA>>>,
    pub outcome_pending: bool,
    pub terminal: bool,
    pub pending_status: RuntimeMaintenanceStatus,
    pub closing: bool,
    pub faulted: bool,
    pub resume_pending: bool,
}
impl<PA: PluginApp> RuntimeLiveCleanupPump<PA> {
    pub fn new() -> Self {
        Self { session: None, rejected: None, outcome_pending: false, terminal: false, pending_status: RuntimeMaintenanceStatus::Ready, closing: false, faulted: false, resume_pending: false }
    }
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,semio_framework_value::ValueError>{
        let metadata=|bytes|RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};
        let quote=if let Some(rejected)=self.rejected.as_ref(){if rejected.terminal_is_empty(){return Ok(metadata(size_of_val(rejected)));}rejected.retirement_demands(body)?}
        else if let Some(session)=self.session.as_ref(){if session.terminal_is_empty(){return Ok(metadata(size_of_val(session)));}session.retirement_demands(body).map_err(|error|match error{semio_framework_job_worker::WorkerJobDemandError::Refused(error)=>error,semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"runtime cleanup original session remains checked out")})?}
        else{return Ok(Default::default());};
        Ok(RetirementDemand{depth:quote.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"runtime cleanup session depth overflow"))?,..quote})
    }
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Step};
use semio_framework_job_worker_authority::{WorkerJobCloseStep as Worker};
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(Step::Pending{progress:Default::default()})}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Step::Pending{progress:Default::default()});}
        self.closing=true;
        let metadata=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if let Some(rejected)=self.rejected.as_mut(){
            if rejected.terminal_is_empty(){cx.consume_retained(metadata)?;drop(self.rejected.take());cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata});}
            let step=rejected.close_step(cx)?.admit(grant,rejected.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            return Ok(match step{Step::Complete{progress}=>Step::Pending{progress},step=>step});
        }
        if let Some(session)=self.session.as_mut(){
            if session.terminal_is_empty(){cx.consume_retained(metadata)?;drop(self.session.take());self.outcome_pending=false;self.resume_pending=false;cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata});}
            session.begin_close();
            let step=match session.close_step(cx)?{Worker::Pending{progress}|Worker::Complete{progress}=>Step::Pending{progress},Worker::Blocked=>Step::Blocked};
            return step.admit(grant,false).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Ok(Step::Complete{progress:Default::default()})
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.rejected.is_none() && !self.outcome_pending
    }
}
#[doc(hidden)]
pub struct RuntimeCloseCleanupPump<PA: PluginApp> {
    pub session: Option<semio_framework_job_worker::BatchJobSession<RuntimeCloseCleanupJob<PA>>>,
    pub rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<RuntimeCloseCleanupJob<PA>>>,
    pub outcome_pending: bool,
    pub terminal: bool,
    pub complete: bool,
    pub blocked: bool,
    pub pending_status: RuntimeCloseStatus,
    pub faulted: bool,
    pub resume_pending: bool,
}
impl<PA: PluginApp> RuntimeCloseCleanupPump<PA> {
    pub fn new() -> Self {
        Self { session: None, rejected: None, outcome_pending: false, terminal: false, complete: false, blocked: false, pending_status: RuntimeCloseStatus::Ready, faulted: false, resume_pending: false }
    }
}
#[doc(hidden)]
pub struct RuntimeLiveCleanupJob<PA: PluginApp> {
    pub instance_id: u32,
    pub generation: semio_framework_job::Generation,
    pub cell: Option<std::sync::Weak<RuntimeAppCell<PA>>>,
    pub progress: Option<PluginLifecycleStep>,
    pub contended: bool,
    pub publication: semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication,
    pub closing: bool,
}
impl<PA: PluginApp> semio_framework_plugin_job::reserved_job_outcomes::CleanupJob for RuntimeLiveCleanupJob<PA> {
    fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> CleanupTurn {
        RuntimeLiveCleanupJob::advance(self, cx)
    }
    fn instance_id(&self) -> u32 {
        self.instance_id
    }
    fn publication_mut(&mut self) -> &mut semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication {
        &mut self.publication
    }
}
impl<PA: PluginApp> RuntimeLiveCleanupJob<PA> {
    pub fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        let Self{instance_id,cell,progress,contended,..}=self;let Some(original)=cell.as_ref()else{return CleanupTurn::Cancelled};
        match cx.with_original_native_weak_source(original,|cell,cx|Ok(Self::advance_source(cell,*instance_id,progress,contended,cx))){Ok(turn)=>turn,Err(original)=>CleanupTurn::OriginalFailure(original)}
    }
    pub fn advance_source(cell:&RuntimeAppCell<PA>,instance_id:u32,outcome_progress:&mut Option<PluginLifecycleStep>,contended:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        cx.set_stage("plugin-runtime-live-cleanup");
        *outcome_progress = None;
        *contended = false;
        if cx.is_cancelled() {
            return CleanupTurn::Cancelled;
        }
        if cx.deadline_exceeded() || cx.fuel_exhausted() { return CleanupTurn::Yield; }
        let mut instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => {
                *contended = true;
                return CleanupTurn::Yield;
            }
            Err(RuntimeInstanceRefusal::Poisoned) => {
                return CleanupTurn::Fault(b"plugin live cleanup authority is poisoned");
            }
        };
        #[cfg(any(test, target_arch = "wasm32"))]
        cell.maintenance_probe_entries.fetch_add(1, Ordering::Relaxed);
        let traced = semio_framework_trace::runtime_diagnostics_enabled();
        let maintenance_started_us = traced.then(semio_framework_job::default_now_us).flatten();
        let maintenance = (|| -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let phase = instance.app.mounted_owner_maintenance_phase()?;
            let identity = MountedOwnerIdentityV1 { instance_id: instance_id, operation: cx.operation(), generation: cx.generation() };
            let Some(mut turn) = MountedOwnerTurnV1::admit(instance_id, identity, phase, cell.mounted_policy, cx)? else { return Ok(PluginLifecycleStep::AwaitingInput { reason: "mounted maintenance retains its original scheduling authority" }); };
            let step = instance.app.mounted_owner_maintenance_step(&mut turn)?;
            if step.progress().unwrap_or_default() != turn.progress() || matches!(step, PluginLifecycleStep::Complete(_)) && !instance.app.maintenance_terminal_is_empty() { return Err(Fault::from("mounted maintenance receipt or terminal witness disagrees").into()); }
            Ok(step)
        })();
        cell.maintenance_pressure.store(instance.app.maintenance_under_pressure(), Ordering::Relaxed);
        if let (Some(started_us), Some(finished_us)) = (maintenance_started_us, maintenance_started_us.and_then(|_| semio_framework_job::default_now_us())) {
            if finished_us.saturating_sub(started_us) >= 2_000 {
                eprintln!("[TRACE] maintenance stage={} elapsed_us={} outcome={:?}", LAST_MAINTENANCE_STAGE.load(Ordering::Relaxed), finished_us - started_us, maintenance.as_ref().map(|_| ()).map_err(|failure| &failure.primary));
            }
        }
        match maintenance {
            Ok(progress @ PluginLifecycleStep::Progress(receipt)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Checkpoint(receipt.copied_items as u64)
            }
            Ok(progress @ PluginLifecycleStep::Blocked { .. }) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::AwaitingInput { .. }) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::Complete(_)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Complete
            }
            Err(error) => CleanupTurn::OriginalFailure(error.into()),
        }
    }
}
impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeLiveCleanupJob<PA> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        semio_framework_plugin_job::reserved_job_outcomes::drive_turn(self, cx)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing = true;
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(cx);
        }
        if self.cell.is_some(){return Ok(match cx.return_original_native_weak(&mut self.cell)?{Some(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},None=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()}})}
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let demand = self.publication.close_demands()?;
        Ok(if demand == RetirementDemand::default() { usize::from(self.cell.is_some()) } else { demand.depth })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.cell.is_none() && self.publication.terminal_is_empty()
    }
}
#[doc(hidden)]
pub struct RuntimeCloseCleanupJob<PA: PluginApp> {
    pub state: Option<std::sync::Weak<RuntimeCloseWorkerState<PA>>>,
    pub instance_id: u32,
    pub progress: Option<PluginLifecycleStep>,
    pub contended: bool,
    pub publication: semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication,
    pub closing: bool,
}
impl<PA: PluginApp> semio_framework_plugin_job::reserved_job_outcomes::CleanupJob for RuntimeCloseCleanupJob<PA> {
    fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> CleanupTurn {
        RuntimeCloseCleanupJob::advance(self, cx)
    }
    fn instance_id(&self) -> u32 {
        self.instance_id
    }
    fn publication_mut(&mut self) -> &mut semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication {
        &mut self.publication
    }
}
impl<PA: PluginApp> RuntimeCloseCleanupJob<PA> {
    pub fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        let Self{state,progress,contended,..}=self;let Some(original)=state.as_ref()else{return CleanupTurn::Cancelled};
        match cx.with_original_native_weak_source(original,|state,cx|Ok(Self::advance_source(state,progress,contended,cx))){Ok(turn)=>turn,Err(original)=>CleanupTurn::OriginalFailure(original)}
    }
    pub fn advance_source(state:&RuntimeCloseWorkerState<PA>,outcome_progress:&mut Option<PluginLifecycleStep>,contended:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        runtime_close_phase(&state, 8);
        cx.set_stage("plugin-runtime-close-cleanup");
        runtime_close_phase(&state, 9);
        *outcome_progress = None;
        *contended = false;
        if cx.is_cancelled() {
            return CleanupTurn::Cancelled;
        }
        if cx.deadline_exceeded() || cx.fuel_exhausted() { return CleanupTurn::Yield; }
        let cell_owner = match state.cell.try_lock() {
                Ok(cell) => cell,
                Err(std::sync::TryLockError::WouldBlock) => {
                    *contended = true;
                    return CleanupTurn::Yield;
                }
                Err(std::sync::TryLockError::Poisoned(_)) => {
                    return CleanupTurn::Fault(b"plugin close cell authority is poisoned");
                }
            };
        let Some(cell) = cell_owner.as_ref() else {
                *outcome_progress = Some(PluginLifecycleStep::Complete(Default::default()));
                return CleanupTurn::Complete;
            };
        {

            let mut maintenance = match cell.maintenance_pump.try_lock() {
                Ok(maintenance) => maintenance,
                Err(std::sync::TryLockError::WouldBlock) => {
                    *contended = true;
                    return CleanupTurn::Yield;
                }
                Err(std::sync::TryLockError::Poisoned(_)) => {
                    return CleanupTurn::Fault(b"plugin maintenance session authority is poisoned");
                }
            };
            runtime_close_phase(&state, 10);
            let grant=state.mounted_policy.close;
            let progress=match cx.with_original_native_weak_parent(cell,|cx|maintenance.close_step(cx)){Ok(step)=>match step.admit(grant,maintenance.terminal_is_empty()){Ok(step)=>step,Err(error)=>return CleanupTurn::OriginalFailure(semio_framework_job::OriginalTurnFailure::from(error))},Err(failure)=>return CleanupTurn::OriginalFailure(failure)};
            runtime_close_phase(&state, 11);
            match progress {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress }|semio_framework_job::InteractiveJobCloseStep::Complete { progress } => {
                    *outcome_progress = Some(PluginLifecycleStep::Progress(progress));
                    if progress!=RetainedCloneProgress::default(){return CleanupTurn::Yield;}
                }
                semio_framework_job::InteractiveJobCloseStep::Blocked => *contended = true,
            }
            if !maintenance.terminal_is_empty(){return CleanupTurn::Yield;}
        }
        let mut instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => {
                *contended = true;
                return CleanupTurn::Yield;
            }
            Err(RuntimeInstanceRefusal::Poisoned) => {
                return CleanupTurn::Fault(b"plugin app close authority is poisoned");
            }
        };
        runtime_close_phase(&state, 12);
        #[cfg(test)]
        state.physical_close_calls.fetch_add(1, Ordering::SeqCst);
        let progress = (|| -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let identity = MountedOwnerIdentityV1 { instance_id: state.instance_id, operation: cx.operation(), generation: cx.generation() };
            let Some(mut turn) = MountedOwnerTurnV1::admit(state.instance_id, identity, MountedOwnerPhaseV1::Close, state.mounted_policy, cx)? else { return Ok(PluginLifecycleStep::AwaitingInput { reason: "mounted close retains its original scheduling authority" }); };
            let step = instance.app.mounted_owner_close_step(&mut turn)?;
            if step.progress().unwrap_or_default() != turn.progress() || matches!(step, PluginLifecycleStep::Complete(_)) && !instance.app.close_terminal_is_empty() { return Err(Fault::from("mounted close receipt or terminal witness disagrees").into()); }
            Ok(step)
        })();
        runtime_close_phase(&state, 13);
        let outcome = match progress {
            Ok(progress @ PluginLifecycleStep::Progress(receipt)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Checkpoint(receipt.copied_items as u64)
            }
            Ok(progress @ PluginLifecycleStep::Blocked { reason }) => {
                runtime_close_pending_authority(reason);
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::AwaitingInput { reason }) => {
                runtime_close_pending_authority(reason);
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::Complete(_)) => {
                if !instance.app.close_terminal_is_empty() {
                    return CleanupTurn::Fault(b"plugin close reported Complete without a terminal-empty app witness");
                }
                *outcome_progress = Some(progress);
                CleanupTurn::Complete
            }
            Err(error) => CleanupTurn::OriginalFailure(error.into()),
        };
        runtime_close_phase(&state, 14);
        outcome
    }
}
impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        semio_framework_plugin_job::reserved_job_outcomes::drive_turn(self, cx)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing = true;
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(cx);
        }
        if self.state.is_some(){return Ok(match cx.return_original_native_weak(&mut self.state)?{Some(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},None=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()}})}
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let demand = self.publication.close_demands()?;
        Ok(if demand == RetirementDemand::default() { usize::from(self.state.is_some()) } else { demand.depth })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.state.is_none() && self.publication.terminal_is_empty()
    }
}
/// ⏳️ A close authority that is not yet releasable is a WAIT, never a fault: the ladder keeps
/// stepping until its external owner lets go (mirrors the live cleanup job, which has always
/// yielded on `Blocked`). Reported on a doubling cadence so a genuinely stuck owner is visible
/// without flooding a close that clears after a few turns.
fn runtime_close_pending_authority(reason: &'static str) {
    let seen = RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS.fetch_add(1, Ordering::Relaxed).saturating_add(1);
    if seen.is_power_of_two() {
        debug_runtime_line(format_args!("[TRACE] runtime close pending authority turn={seen} reason={reason}"));
    }
}
static RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS: AtomicU64 = AtomicU64::new(0);
#[doc(hidden)]
pub fn runtime_close_phase<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, phase: usize) {
    #[cfg(test)]
    if let Some(now) = semio_framework_job::default_now_us() {
        state.callback_phase_us[phase].store(now.saturating_sub(state.callback_phase_started_us.load(Ordering::SeqCst)), Ordering::SeqCst);
    }
    #[cfg(not(test))]
    let _ = (state, phase);
}

pub(super) fn checked_runtime_close_generation(current: u64) -> Result<u64, Fault> {
    current.checked_add(1).ok_or_else(|| plugin_internal_fault("runtime close generation exhausted"))
}
#[cfg(any(test, feature = "artifact-app-testing"))]
std::thread_local! {
    #[doc(hidden)]
    pub static RUNTIME_CLOSE_CONSTRUCTION_LIVE: Cell<Option<bool>> = const { Cell::new(None) };
    #[doc(hidden)]
    pub static RUNTIME_CLOSE_CONSTRUCTION_FAIL: Cell<bool> = const { Cell::new(false) };
}
#[doc(hidden)]
pub fn plugin_begin_instance_close<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>, instance_id: u32, expected: Option<&std::sync::Weak<RuntimeAppCell<PA>>>) -> Result<std::sync::Arc<RuntimeCloseWorkerState<PA>>, Fault> {
    let mut instances = runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let mut quarantine = runtime.close_quarantine.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime close quarantine is busy"))?;
    let mut actors = runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?;
    let mut document_backbones = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    if let Some(expected) = expected {
        let current = instances.get(instance_id).ok_or_else(|| plugin_internal_fault("captured app lifetime is no longer live"))?;
        if !std::sync::Weak::ptr_eq(expected, &std::sync::Arc::downgrade(current)) {
            return Err(plugin_internal_fault("captured app lifetime does not match the current instance"));
        }
    }
    if !quarantine.can_insert(instance_id) {
        return Err(plugin_internal_fault(format!("runtime close quarantine is saturated or collided: {instance_id}")));
    }
    let generation = checked_runtime_close_generation(runtime.close_generation.get())?;
    #[cfg(test)]
    let original_allocation = instances.get(instance_id).map(std::sync::Arc::as_ptr);
    let cell = instances.get(instance_id).cloned().ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))?;
    #[cfg(test)]
    RUNTIME_CLOSE_CONSTRUCTION_FAIL.with(|fail| {
        if fail.replace(false) {
            panic!("injected before close worker allocation");
        }
    });
    let state = std::sync::Arc::new(RuntimeCloseWorkerState {
        instance_id,
        generation: semio_framework_job::Generation(generation),
        mounted_policy: cell.mounted_policy,
        cell: std::sync::Mutex::new(std::mem::ManuallyDrop::new(Some(cell))),
        pump: std::sync::Mutex::new(RuntimeCloseCleanupPump::new()),
        status: AtomicU8::new(RuntimeCloseStatus::Ready.repr()),
        deadline_resume: AtomicU8::new(u8::MAX),
        deadline_elapsed_us: AtomicU64::new(0),
        stalled_steps: AtomicU8::new(0),
        stall_since_us: AtomicU64::new(0),
        stall_credit_spent_us: AtomicU64::new(0),
        last_callback_elapsed_us: AtomicU64::new(0),
        #[cfg(test)]
        last_fault: std::sync::Mutex::new([0; 256]),
        #[cfg(test)]
        last_fault_origin: AtomicU8::new(0),
        #[cfg(test)]
        physical_close_calls: AtomicU64::new(0),
        #[cfg(test)]
        callback_phase_started_us: AtomicU64::new(0),
        #[cfg(test)]
        callback_phase_us: std::array::from_fn(|_| AtomicU64::new(0)),
    });
    #[cfg(test)]
    RUNTIME_CLOSE_CONSTRUCTION_LIVE.with(|probe| probe.set(Some(original_allocation.is_some() && instances.get(instance_id).map(std::sync::Arc::as_ptr) == original_allocation)));
    quarantine.insert_admitted(instance_id, RuntimeCloseEntry { state: state.clone() });
    drop(instances.take(instance_id));
    runtime.close_generation.set(generation);
    let _ = actors.take(instance_id);
    if let Some(binding) = document_backbones.take(instance_id) {
        if let Some(owner) = binding.channel {
            owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
        }
    }
    drop(document_backbones);
    drop(actors);
    drop(quarantine);
    drop(instances);
    Ok(state)
}
