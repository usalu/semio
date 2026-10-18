//! 🔁️ Live and close cleanup turns, stall verdicts, destroy and the live maintenance steps.

use semio_framework_plugin_decl_app::ArtifactApp;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::{PLUGIN_RUNTIME_INSTANCE_SLOTS, PluginRuntime, RuntimeCloseWorkerState};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::{RUNTIME_CLEANUP_UNMEASURED_US, RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT, RuntimeCleanupFault, RuntimeCloseStatus, RuntimeMaintenanceStatus, runtime_cleanup_fault};
use crate::inst_opening::{RUNTIME_CLOSE_INNER_GRANT_US, RUNTIME_CLOSE_STALL_CREDIT_US, RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_reactor_lifecycle::life_app_cell::{RuntimeAppCell, RuntimeInstanceRefusal};
use semio_framework_plugin_reactor_lifecycle::life_cleanup_pumps::{RuntimeCloseCleanupJob, RuntimeCloseCleanupPump, RuntimeLiveCleanupJob, RuntimeLiveCleanupPump, plugin_begin_instance_close, runtime_close_phase};
use semio_framework_plugin_reactor_app::app_runtime_helpers::{debug_runtime_line, plugin_internal_fault};
use std::mem::size_of_val;
#[doc(hidden)]
pub fn runtime_live_cleanup_nonterminal_status(contended: bool, progress: Option<PluginLifecycleStep>, stalled_steps: &AtomicU32) -> RuntimeMaintenanceStatus {
    if contended {
        return RuntimeMaintenanceStatus::Ready;
    }
    if matches!(progress, Some(PluginLifecycleStep::AwaitingInput { .. })) {
        stalled_steps.store(0, Ordering::SeqCst);
        return RuntimeMaintenanceStatus::Ready;
    }
    let zero_progress = matches!(progress,Some(PluginLifecycleStep::Progress(receipt)) if receipt==RetainedCloneProgress::default())||matches!(progress,Some(PluginLifecycleStep::Blocked{..}));
    if !zero_progress {
        stalled_steps.store(0, Ordering::SeqCst);
        return RuntimeMaintenanceStatus::Ready;
    }
    let stalled = stalled_steps.fetch_add(1, Ordering::SeqCst).saturating_add(1);
    if stalled >= RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
        RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress)
    } else {
        RuntimeMaintenanceStatus::Ready
    }
}
/// ⏱️ Publishes one maintenance turn: the measured wall cost is recorded next to the status so
/// the decode site can name the overrun in microseconds, and a clock failure or a ceiling
/// overrun outranks whatever the pump itself concluded.
fn runtime_live_cleanup_publish_turn<PA: PluginApp>(cell: &RuntimeAppCell<PA>, status: RuntimeMaintenanceStatus, elapsed: Result<u64, RuntimeCleanupFault>) {
    let verdict = match elapsed {
        Err(cause) => RuntimeMaintenanceStatus::Fault(cause),
        Ok(elapsed_us) if semio_framework_trace::interactive_step_contract_violated(elapsed_us) => {
            if semio_framework_trace::runtime_diagnostics_enabled() {
                eprintln!("[TRACE] cooperative maintenance callback overran the interactive ceiling for instance {} (elapsed {elapsed_us}us) — recorded, not fatal: the callback clock measures wall time including descheduling and the pumped foreign job step; the job-level step contract quarantines slow steps", cell.id);
            }
            status
        }
        Ok(_) => status,
    };
    cell.maintenance_fault_us.store(elapsed.unwrap_or(RUNTIME_CLEANUP_UNMEASURED_US), Ordering::SeqCst);
    cell.maintenance_status.store(verdict.repr(), Ordering::SeqCst);
}
fn run_runtime_live_cleanup_turn<PA:PluginApp+'static>(cell:&std::sync::Arc<RuntimeAppCell<PA>>,generation:semio_framework_job::Generation,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(())}
    let mut clock_source=cx.clock_source();let clock=runtime_callback_clock_begin(&mut clock_source);
    let status=cx.with_original_native_weak_parent(cell,|cx|match cell.maintenance_pump.try_lock(){Ok(mut pump)=>runtime_live_cleanup_pump_one(cell,generation,&mut pump,cx),Err(_)=>Ok(RuntimeMaintenanceStatus::Ready)})?;
    let elapsed=clock.and_then(|clock|runtime_callback_clock_elapsed(&mut clock_source,clock));
    runtime_live_cleanup_publish_turn(cell,status,elapsed);Ok(())
}
fn runtime_live_cleanup_pump_one<PA: PluginApp + 'static>(cell: &std::sync::Arc<RuntimeAppCell<PA>>, generation: semio_framework_job::Generation, pump: &mut RuntimeLiveCleanupPump<PA>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RuntimeMaintenanceStatus,semio_framework_job::OriginalTurnFailure> {
    let grant=cx.retained_grant();
    if pump.closing {let _=pump.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::AlreadyClosing));}
    if let Some(rejected)=pump.rejected.as_mut().filter(|pending|!pending.admission_pending()||cx.is_cancelled()){
        if rejected.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(rejected){return Ok(RuntimeMaintenanceStatus::Ready);}drop(pump.rejected.take());return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::AdmissionRejected));}
        rejected.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Ready)
    }
    if pump.outcome_pending{
        let Some(session)=pump.session.as_mut()else{return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::MissingSession))};
        let acknowledgement=session.acknowledge_outcome(grant);cx.consume_retained(acknowledgement.progress())?;if !matches!(acknowledgement,RetainedCloneStep::Complete(_)){return Ok(RuntimeMaintenanceStatus::Ready);}
        pump.outcome_pending=false;
        let Some(job) = session.checked_out_job_mut() else { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::CheckedOutJob)) };
        pump.pending_status = runtime_live_cleanup_nonterminal_status(job.contended, job.progress, &cell.maintenance_stalled_steps);
        #[cfg(test)]
        if matches!(job.progress, Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { .. })) {
            cell.maintenance_probe_input_waits.fetch_add(1, Ordering::SeqCst);
        }
        pump.resume_pending=true;
        return Ok(RuntimeMaintenanceStatus::Ready);
    }
    if pump.resume_pending{
        pump.resume_pending=false;
        if pump.terminal{if let Some(session)=pump.session.as_mut(){session.begin_close();}return Ok(RuntimeMaintenanceStatus::Ready);}
        return Ok(if pump.session.as_mut().is_some_and(|session|session.resume().is_err()){RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::Resume)}else{pump.pending_status});
    }
    if pump.terminal{
        let Some(session)=pump.session.as_mut()else{pump.terminal=false;return Ok(pump.pending_status);};
        if session.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(session){return Ok(RuntimeMaintenanceStatus::Ready);}drop(pump.session.take());pump.terminal=false;return Ok(if pump.faulted{RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::PriorOutcome)}else{pump.pending_status});}
        session.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Ready)
    }
    if pump.session.is_none()&&pump.rejected.is_none(){let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RuntimeMaintenanceStatus::Ready)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);pump.rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(RuntimeLiveCleanupJob { instance_id: cell.id, generation, cell: Some(std::sync::Arc::downgrade(cell)), progress: None, contended: false, publication: semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication::new(), closing: false },(cx.operation(),cx.generation())));}
    if let Some(pending)=pump.rejected.as_mut(){
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_runtime_live_cleanup",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:RUNTIME_CLOSE_INNER_GRANT_US};
        match pending.admit_pending_batch(config,cx)?{Some(session)=>{pump.session=Some(session);drop(pump.rejected.take());},None=>return Ok(RuntimeMaintenanceStatus::Ready)}
        return Ok(RuntimeMaintenanceStatus::Ready);
    }
    let Some(session) = pump.session.as_mut() else { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::MissingSession)) };
    match session.step_on_original_caller(cx) {
        Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) => {}
        Ok(_) => return Ok(RuntimeMaintenanceStatus::Ready),
        Err(failure) => return Err(failure),
    }
    if !session.checkout_outcome() {
        return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::Checkout));
    }
    if !session.take_checked_out_retained_step_receipt().is_some_and(|(issued, progress)| progress.fits(issued)) { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::TakeOutcome)); }
    let (terminal, faulted) = match session.checked_out_outcome() {
        Ok(Some(view)) => (view.is_terminal(), matches!(view, semio_framework_job::JobOutcomeView::Fault { .. } | semio_framework_job::JobOutcomeView::Cancelled { .. })),
        Ok(None) => (false, false),
        Err(_) => return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::TakeOutcome)),
    };
    pump.terminal = terminal;
    pump.faulted = faulted;
    pump.outcome_pending = true;
    Ok(RuntimeMaintenanceStatus::Ready)
}
/// 🩺️ The structural accountant's verdict on ONE close-ladder step, priced in BOTH currencies it
/// claims to spend: consecutive fruitless steps AND the real microseconds they burned.
///
/// `progress` is what the ladder READ this step. `None` means the step recorded no reading at all —
/// it was cancelled, or it never reached the ladder — which is an absence of evidence, not evidence
/// of a stall, so it buys no stall debt; a cancelled or faulted outcome is terminal through
/// `RuntimeCleanupFault::PriorOutcome` instead. Only an observed `Pending { 0, 0 }` — the ladder
/// answered, and answered that it released nothing — is a stalled step.
///
/// The fault needs both: `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` consecutive stalled steps, and
/// `RUNTIME_CLOSE_STALL_CREDIT_US` of real elapsed time since the ladder last moved. `stall_since_us`
/// carries that instant (`0` = not stalling); the returned reading is the credit actually spent, so
/// the emitted message quotes a measured stall rather than the unrelated cost of its last turn.
#[doc(hidden)]
pub fn runtime_close_stall_verdict(contended: bool, progress: Option<PluginLifecycleStep>, stalled_steps: &AtomicU8, stall_since_us: &AtomicU64, now_us: Option<u64>) -> (RuntimeCloseStatus, u64) {
    let clear = || {
        stalled_steps.swap(0, Ordering::SeqCst);
        stall_since_us.store(0, Ordering::SeqCst);
    };
    if contended {
        return (RuntimeCloseStatus::Ready, 0);
    }
    if matches!(progress, Some(PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. })) {
        clear();
        return (RuntimeCloseStatus::ExternalWait, 0);
    }
    if !matches!(progress,Some(PluginLifecycleStep::Progress(receipt)) if receipt==RetainedCloneProgress::default()) {
        clear();
        return (RuntimeCloseStatus::Ready, 0);
    }
    let stalled = stalled_steps.try_update(Ordering::SeqCst, Ordering::SeqCst, |seen| Some(seen.saturating_add(1))).unwrap_or(u8::MAX).saturating_add(1);
    let Some(now_us) = now_us else { return (RuntimeCloseStatus::Ready, 0) };
    let since_us = match stall_since_us.compare_exchange(0, now_us.max(1), Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => now_us.max(1),
        Err(previous) => previous,
    };
    let credit_us = now_us.saturating_sub(since_us);
    if stalled >= RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT && credit_us >= RUNTIME_CLOSE_STALL_CREDIT_US {
        (RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress), credit_us)
    } else {
        (RuntimeCloseStatus::Ready, credit_us)
    }
}
#[doc(hidden)]
pub fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
    if state.status.compare_exchange(RuntimeCloseStatus::Queued.repr(), RuntimeCloseStatus::Running.repr(), Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Ok(None);
    }
    let retained = state.deadline_resume.load(Ordering::SeqCst);
    if retained != u8::MAX {
        return Ok(Some(RuntimeCloseStatus::from_repr(retained)));
    }
    #[cfg(test)]
    {
        state.callback_phase_started_us.store(semio_framework_job::default_now_us().unwrap_or(0), Ordering::SeqCst);
        for elapsed in &state.callback_phase_us {
            elapsed.store(0, Ordering::SeqCst);
        }
    }
    let status = match state.pump.try_lock() {
        Ok(mut pump) => {
            let status = cx.with_original_native_weak_parent(state,|cx|runtime_close_cleanup_pump_one(state,&mut pump,cx))?;
            runtime_close_phase(state, 20);
            drop(pump);
            runtime_close_phase(state, 21);
            status
        }
        Err(_) => RuntimeCloseStatus::Ready,
    };
    runtime_close_phase(state, 6);
    let candidate = if status == RuntimeCloseStatus::Complete { runtime_close_retire_cell(state) } else { status };
    runtime_close_phase(state, 7);
    Ok(Some(candidate))
}
#[doc(hidden)]
pub fn runtime_close_retire_cell<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>) -> RuntimeCloseStatus {
    let mut owner = match state.cell.try_lock() {
        Ok(owner) => owner,
        Err(std::sync::TryLockError::WouldBlock) => return RuntimeCloseStatus::Ready,
        Err(std::sync::TryLockError::Poisoned(_)) => return runtime_close_fault(state, RuntimeCleanupFault::OwnerPoisoned),
    };
    let Some(cell) = owner.as_ref() else { return RuntimeCloseStatus::Complete };
    {
        let instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => return RuntimeCloseStatus::Ready,
            Err(RuntimeInstanceRefusal::Poisoned) => return runtime_close_fault(state, RuntimeCleanupFault::InstancePoisoned),
        };
        if !instance.app.close_terminal_is_empty() {
            return runtime_close_fault(state, RuntimeCleanupFault::InstanceNotDrained);
        }
        let maintenance = match cell.maintenance_pump.try_lock() {
            Ok(maintenance) => maintenance,
            Err(std::sync::TryLockError::WouldBlock) => return RuntimeCloseStatus::Ready,
            Err(std::sync::TryLockError::Poisoned(_)) => return runtime_close_fault(state, RuntimeCleanupFault::MaintenancePoisoned),
        };
        if !maintenance.terminal_is_empty() {
            return runtime_close_fault(state, RuntimeCleanupFault::MaintenanceNotDrained);
        }
    }
    let detached = owner.take().expect("captured owner remains under the same guard");
    match std::sync::Arc::try_unwrap(detached) {
        Ok(cell) => {
            drop(cell);
            RuntimeCloseStatus::Complete
        }
        Err(detached) => {
            **owner = Some(detached);
            RuntimeCloseStatus::ExternalWait
        }
    }
}
#[doc(hidden)]
pub fn runtime_close_fault<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, cause: RuntimeCleanupFault) -> RuntimeCloseStatus {
    #[cfg(test)]
    state.last_fault_origin.store(cause as u8, Ordering::SeqCst);
    #[cfg(not(test))]
    let _ = state;
    RuntimeCloseStatus::Fault(cause)
}
#[doc(hidden)]
pub fn runtime_close_cleanup_pump_one<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>, pump: &mut RuntimeCloseCleanupPump<PA>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RuntimeCloseStatus,semio_framework_job::OriginalTurnFailure> {
    runtime_close_phase(state, 0);
    let grant=cx.retained_grant();
    if let Some(rejected)=pump.rejected.as_mut().filter(|pending|!pending.admission_pending()||cx.is_cancelled()){
        if rejected.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(rejected){return Ok(RuntimeCloseStatus::Ready);}drop(pump.rejected.take());return Ok(runtime_close_fault(state,RuntimeCleanupFault::AdmissionRejected));}
        rejected.close_step(cx)?;return Ok(RuntimeCloseStatus::Ready);
    }
    if pump.outcome_pending{
        let Some(session)=pump.session.as_mut()else{return Ok(runtime_close_fault(state,RuntimeCleanupFault::MissingSession));};
        let acknowledgement=session.acknowledge_outcome(grant);cx.consume_retained(acknowledgement.progress())?;if !matches!(acknowledgement,RetainedCloneStep::Complete(_)){return Ok(RuntimeCloseStatus::Ready);}
        pump.outcome_pending=false;
        let Some(job) = session.checked_out_job_mut() else { return Ok(runtime_close_fault(state, RuntimeCleanupFault::CheckedOutJob)) };
        pump.complete = matches!(job.progress,Some(PluginLifecycleStep::Complete(_)));
        pump.blocked = matches!(job.progress, Some(PluginLifecycleStep::Blocked { .. }));
        let (status, credit_us) = runtime_close_stall_verdict(job.contended, job.progress, &state.stalled_steps, &state.stall_since_us, semio_framework_job::default_now_us());
        pump.pending_status = status;
        if let Some(cause) = pump.pending_status.cause() {
            state.stall_credit_spent_us.store(credit_us, Ordering::SeqCst);
            runtime_close_fault(state, cause);
        }
        pump.resume_pending=true;
        return Ok(RuntimeCloseStatus::Ready);
    }
    if pump.resume_pending{
        pump.resume_pending=false;
        if pump.terminal{if let Some(session)=pump.session.as_mut(){session.begin_close();}return Ok(RuntimeCloseStatus::Ready);}
        return Ok(if pump.session.as_mut().is_some_and(|session|session.resume().is_err()){runtime_close_fault(state,RuntimeCleanupFault::Resume)}else{pump.pending_status});
    }
    if pump.terminal{
        let Some(session)=pump.session.as_mut()else{return Ok(runtime_close_fault(state,RuntimeCleanupFault::MissingSession));};
        if !session.terminal_is_empty(){session.close_step(cx)?;return Ok(RuntimeCloseStatus::Ready);}
        if grant.maximum_copy_bytes<size_of_val(session){return Ok(RuntimeCloseStatus::Ready);}
        drop(pump.session.take());pump.terminal=false;
        return Ok(if pump.faulted{runtime_close_fault(state,RuntimeCleanupFault::PriorOutcome)}else if pump.blocked{RuntimeCloseStatus::ExternalWait}else if pump.complete{RuntimeCloseStatus::Complete}else{pump.pending_status});
    }
    if pump.session.is_none()&&pump.rejected.is_none(){let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RuntimeCloseStatus::Ready)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);pump.rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(RuntimeCloseCleanupJob { state: Some(std::sync::Arc::downgrade(state)), instance_id: state.instance_id, progress: None, contended: false, publication: semio_framework_plugin_job::reserved_job_outcomes::CleanupPublication::new(), closing: false },(cx.operation(),cx.generation())));}
    if let Some(pending)=pump.rejected.as_mut(){
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_runtime_close_cleanup",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:RUNTIME_CLOSE_INNER_GRANT_US};
        match pending.admit_pending_batch(config,cx)?{Some(session)=>{pump.session=Some(session);drop(pump.rejected.take());},None=>return Ok(RuntimeCloseStatus::Ready)}
        return Ok(RuntimeCloseStatus::Ready);
    }
    let Some(session) = pump.session.as_mut() else { return Ok(runtime_close_fault(state, RuntimeCleanupFault::MissingSession)) };
    runtime_close_phase(state, 22);
    let stepped = session.step_on_original_caller(cx);
    runtime_close_phase(state, 23);
    match stepped {
        Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) => {}
        Ok(_) => return Ok(RuntimeCloseStatus::Ready),
        Err(failure) => return Err(failure),
    }
    runtime_close_phase(state, 3);
    if !session.checkout_outcome() {
        return Ok(runtime_close_fault(state, RuntimeCleanupFault::Checkout));
    }
    if !session.take_checked_out_retained_step_receipt().is_some_and(|(issued, progress)| progress.fits(issued)) { return Ok(runtime_close_fault(state, RuntimeCleanupFault::TakeOutcome)); }
    let (terminal, faulted) = match session.checked_out_outcome() {
        Ok(Some(view)) => {
            #[cfg(test)]
            if let semio_framework_job::JobOutcomeView::Fault { detail, .. } = &view {
                if let Some(bytes) = detail.single_page() {
                    let mut last_fault = state.last_fault.lock().unwrap();
                    let count = bytes.len().min(last_fault.len());
                    last_fault[..count].copy_from_slice(&bytes[..count]);
                }
            }
            (view.is_terminal(), matches!(view, semio_framework_job::JobOutcomeView::Fault { .. } | semio_framework_job::JobOutcomeView::Cancelled { .. }))
        }
        Ok(None) => (false, false),
        Err(_) => return Ok(runtime_close_fault(state, RuntimeCleanupFault::TakeOutcome)),
    };
    runtime_close_phase(state, 4);
    pump.terminal = terminal;
    pump.faulted = faulted;
    pump.outcome_pending = true;
    Ok(RuntimeCloseStatus::Ready)
}
fn run_runtime_close_turn<PA:PluginApp+'static>(state:&std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(())}
    state.status.store(RuntimeCloseStatus::Queued.repr(),Ordering::SeqCst);
    let mut clock=cx.clock_source();let measured=runtime_callback_clock_begin(&mut clock);
    let Some(status)=run_runtime_close_turn_inner(state,cx)?else{return Ok(())};
    match measured.and_then(|start|runtime_callback_clock_elapsed(&mut clock,start)){Ok(elapsed)=>runtime_close_publish_turn(state,status,elapsed),Err(cause)=>{state.last_callback_elapsed_us.store(u64::MAX,Ordering::SeqCst);state.status.store(runtime_close_fault(state,cause).repr(),Ordering::SeqCst);}}Ok(())
}
#[doc(hidden)]
pub fn runtime_callback_clock_begin(now_us: &mut impl FnMut() -> Option<u64>) -> Result<(u64, u64), RuntimeCleanupFault> {
    let start = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    let preflight = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    if preflight < start {
        return Err(RuntimeCleanupFault::ClockRegression);
    }
    Ok((start, preflight))
}
#[doc(hidden)]
pub fn runtime_callback_clock_elapsed(now_us: &mut impl FnMut() -> Option<u64>, (start, preflight): (u64, u64)) -> Result<u64, RuntimeCleanupFault> {
    let finished = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    if finished < preflight {
        return Err(RuntimeCleanupFault::ClockRegression);
    }
    Ok(finished - start)
}

#[doc(hidden)]
pub fn runtime_close_publish_turn<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, status: RuntimeCloseStatus, elapsed_us: u64) {
    state.last_callback_elapsed_us.store(elapsed_us, Ordering::SeqCst);
    let verdict = if !matches!(status, RuntimeCloseStatus::Fault(_)) && semio_framework_trace::interactive_step_contract_violated(elapsed_us) {
        #[cfg(test)]
        debug_runtime_line(format_args!("[TRACE] native close deadline instance={} generation={} candidate={status:?} elapsed_us={elapsed_us}", state.instance_id, state.generation.0));
        state.deadline_resume.store(status.repr(), Ordering::SeqCst);
        let _ = state.deadline_elapsed_us.compare_exchange(0, elapsed_us, Ordering::SeqCst, Ordering::SeqCst);
        RuntimeCloseStatus::DeadlineYield
    } else {
        state.deadline_resume.store(u8::MAX, Ordering::SeqCst);
        status
    };
    state.status.store(verdict.repr(), Ordering::SeqCst);
}
pub async fn plugin_destroy_app<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<(), Fault> {
    plugin_begin_instance_close(runtime, instance_id, None).map(|_| ())
}
/// 🚪️ A faulted close is TERMINAL for this instance: the entry leaves quarantine with the
/// verdict, so the same dead lifetime cannot re-raise it on every later cursor pass (which,
/// now that the reactor turn survives a close fault, would be an unbounded fault storm).
/// `ZeroProgress` quotes the stall credit it actually spent; every other cause quotes the
/// turn cost it was measured against.
pub fn plugin_step_close_cleanup<PA: PluginApp + 'static>(runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let quarantine=runtime.close_quarantine.try_borrow().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?;
    let Some((index,instance_id,entry))=quarantine.next_entry_from(runtime.close_cleanup_cursor.get())else{return Ok(false)};
    let state=&entry.state;runtime.close_cleanup_cursor.set((index+1)%PLUGIN_RUNTIME_INSTANCE_SLOTS);
    match RuntimeCloseStatus::from_repr(state.status.load(Ordering::SeqCst)){
        RuntimeCloseStatus::Complete=>{drop(quarantine);let removed=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?.take(instance_id);drop(removed);Ok(true)}
        RuntimeCloseStatus::Fault(cause)=>{let elapsed_us=if cause==RuntimeCleanupFault::ZeroProgress{state.stall_credit_spent_us.load(Ordering::SeqCst)}else{state.last_callback_elapsed_us.load(Ordering::SeqCst)};drop(quarantine);let removed=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?.take(instance_id);drop(removed);Err(runtime_cleanup_fault("close",cause,instance_id,elapsed_us).into())}
        _=>{if cx.should_yield(){return Ok(false)}run_runtime_close_turn(state,cx)?;Ok(true)}
    }
}

/// 🌡️ Whether the instance the last [`plugin_step_live_cleanup`] stepped still reports
/// maintenance pressure — the reactor turn keeps stepping while this holds, bounded by
/// [`LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN`] and its retirement deadline.
pub fn plugin_live_cleanup_under_pressure<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>) -> bool {
    runtime.live_cleanup_pressure.get()
}
/// 🌡️ Maintenance steps one reactor turn may spend on pressured instances. A live commit cadence
/// displaces three or more owners per commit and at most one commit lands per turn, so this
/// burst drains faster than any cadence fills while the fair single step stays the idle cost.
pub const LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN: usize = 32;
pub fn plugin_step_live_cleanup<PA: PluginApp + 'static>(runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let instances=runtime.instances.try_borrow().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;
    let Some((index,_,cell))=instances.next_entry_from(runtime.live_cleanup_cursor.get())else{runtime.live_cleanup_pressure.set(false);return Ok(false)};
    runtime.live_cleanup_cursor.set((index+1)%PLUGIN_RUNTIME_INSTANCE_SLOTS);let stepped=plugin_step_live_cleanup_cell(cell,cx);runtime.live_cleanup_pressure.set(cell.maintenance_pressure.load(Ordering::Relaxed));stepped
}

fn plugin_step_live_cleanup_cell<PA:PluginApp+'static>(cell:&std::sync::Arc<RuntimeAppCell<PA>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(false)}
    if let RuntimeMaintenanceStatus::Fault(cause)=RuntimeMaintenanceStatus::from_repr(cell.maintenance_status.load(Ordering::SeqCst)){return Err(runtime_cleanup_fault("live",cause,cell.id,cell.maintenance_fault_us.load(Ordering::SeqCst)).into())}
    let generation=semio_framework_job::Generation(cell.maintenance_generation.load(Ordering::SeqCst));
    run_runtime_live_cleanup_turn(cell,generation,cx)?;Ok(true)
}
