//! ⏯️ Operation begin, resume and cancel: starting an owned operation, the deadlines, the resume variants, the bound resume loop, the fault text and the cancellation. See [`super`].

use super::*;

//#region 🧠️OwnedRuntime
pub(super) fn begin_owned_operation(state: &mut OwnedGuestInstance, operation: OwnedOperation, input: Option<Vec<u8>>) -> Result<(), TurnFault> {
    if state.original_core.is_some(){return Err(identity_turn_fault(operation_authority::refusal(4)))}
    if state.poisoned {
        return Err(TurnFault::Trapped("owned actor instance is poisoned by an earlier guest trap and must be re-instantiated".to_string()));
    }
    if let Some(pending) = &state.pending {
        return if pending.operation == operation { Ok(()) } else { Err(TurnFault::Trapped(format!("owned operation {:?} is still active", pending.operation))) };
    }
    let stage = match input {
        Some(input) => {
            state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.begin(OwnedSemioExport::Allocate, vec![Value::I32(i32::try_from(input.len()).map_err(|_| TurnFault::Trapped("owned input exceeds wasm32 address space".to_string()))?)])?;
            OwnedStage::Allocate { input }
        }
        None => {
            state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.begin(operation.export(), Vec::new())?;
            OwnedStage::Call
        }
    };
    state.pending = Some(OwnedPending { retained_input:None, operation, stage, fuel_used: 0 });
    Ok(())
}

/// ⏱️ Original operations retain their supplied absolute clock boundary; duration APIs carry explicit wall durations.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum OwnedDeadline {
    TotalWall,
    Original,
}

pub(super) fn resume_owned_operation(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_us: u64) -> Result<OwnedInvocation, TurnFault> {
    resume_owned_operation_observed(state, operation, fuel, deadline_us, OwnedDeadline::TotalWall, |_, _| {}, None)
}

pub(super) fn resume_owned_operation_observed(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_us: u64, deadline: OwnedDeadline, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedInvocation, TurnFault> {
    resume_owned_operation_bound(state, operation, fuel, deadline_us, deadline, progress, cancellation, None)
}

pub(super) fn resume_owned_operation_authored(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_us: u64, deadline: OwnedDeadline, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, receiver: &mut dyn OriginalOperationReceiving) -> Result<OwnedInvocation, TurnFault> {
    let result=resume_owned_operation_bound(state, operation, fuel, deadline_us, deadline, progress, cancellation, Some(receiver));
    result.and_then(|output|{receiver.ensure_finished().map_err(identity_turn_fault)?;Ok(output)})
}

pub(super) fn original_deadline_at(now:Option<u64>,deadline_us:u64)->Result<bool,TurnFault>{Ok(now.ok_or_else(||identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original interpreter deadline requires its caller clock")))?>=deadline_us)}

pub(super) fn resume_owned_operation_bound(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_us: u64, deadline: OwnedDeadline, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, mut receiver: Option<&mut dyn OriginalOperationReceiving>) -> Result<OwnedInvocation, TurnFault> {
    let started = std::time::Instant::now();
    let mut remaining = fuel;
    let mut next_progress_fuel = 25_000_000;
    let mut next_progress_elapsed = std::time::Duration::from_secs(5);
    loop {
        let elapsed = started.elapsed();
        if cancellation.is_some_and(GuestCallCancellation::is_cancelled) {
            progress(fuel.saturating_sub(remaining), elapsed);
            return Err(TurnFault::Cancelled);
        }
        let expired = match deadline {
            OwnedDeadline::TotalWall => elapsed >= std::time::Duration::from_micros(deadline_us),
            OwnedDeadline::Original => original_deadline_at(state.original_job_clock.and_then(|(_,clock)|clock()),deadline_us)?,
        };
        if expired {
            progress(fuel.saturating_sub(remaining), elapsed);
            return Err(TurnFault::DeadlineExceeded);
        }
        if remaining == 0 {
            progress(fuel, elapsed);
            return Err(TurnFault::FuelExhausted);
        }
        let mut pending = state.pending.take().ok_or_else(|| TurnFault::Trapped("owned operation has no resumable state".to_string()))?;
        if pending.operation != operation {
            state.pending = Some(pending);
            return Err(TurnFault::Trapped("owned operation resume type mismatch".to_string()));
        }
        let grant = remaining.min(OWNED_STEP_FUEL);
        let completed_fuel;
        match state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.step(grant, StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel = pending.fuel_used;
                state.pending = Some(pending);
                if fuel_used == 0 {
                    return Err(TurnFault::Trapped("owned interpreter yielded without consuming fuel".to_string()));
                }
            }
            CoreStepOutcome::HostCall { fuel_used, call } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel = pending.fuel_used;
                let result=reply_owned_host(state,&call,receiver.as_deref_mut()).map_err(|error|TurnFault::Trapped(error.to_string())).and_then(|results|state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.resume_host(call.id,Ok(results)).map_err(TurnFault::from));
                state.pending=Some(pending);
                result?;
            }
            CoreStepOutcome::Complete { fuel_used, values } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel=pending.fuel_used;
                let result=(||->Result<Option<OwnedInvocation>,TurnFault>{match &mut pending.stage {
                    OwnedStage::Allocate { input } => {
                        let [Value::I32(pointer)] = values.as_slice() else { return Err(TurnFault::Trapped("owned allocator returned an invalid pointer".to_string())) };
                        write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?,*pointer,input)?;
                        let length=input.len()as i32;
                        if matches!(operation,OwnedOperation::SqliteExport|OwnedOperation::SqliteImport|OwnedOperation::SqliteRetirement|OwnedOperation::SqliteClose|OwnedOperation::SqliteTakeFile|OwnedOperation::SqliteTakePayload){pending.retained_input=Some(std::mem::take(input));}
                        state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.begin(operation.export(),vec![Value::I32(*pointer),Value::I32(length)])?;
                        pending.stage=OwnedStage::Call;
                        Ok(None)
                    }
                    OwnedStage::Call => {
                        let [Value::I64(pair)] = values.as_slice() else { return Err(TurnFault::Trapped("owned call returned an invalid pointer/length pair".to_string())) };
                        let pair=*pair as u64;
                        match operation_authority::owned_result::decode(pair).map_err(identity_turn_fault)?{operation_authority::owned_result::OwnedReturn::Bytes{..}=>{},operation_authority::owned_result::OwnedReturn::Refusal(kind)=>return Err(identity_turn_fault(semio_framework_value::ValueError::literal(kind,"original guest operation returned scalar refusal"))),}
                        if let Some(receiver)=receiver.as_deref_mut(){receiver.receive_output((pair>>32)as u32 as usize).map_err(identity_turn_fault)?;}
                        let output=state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.read_bytes_result(&values,OWNED_RESULT_LIMIT)?;
                        pending.stage=OwnedStage::Deallocate{output};
                        state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.begin(OwnedSemioExport::Deallocate,vec![Value::I32(pair as u32 as i32),Value::I32((pair>>32)as u32 as i32)])?;
                        Ok(None)
                    }
                    OwnedStage::Deallocate { output } => Ok(Some(OwnedInvocation{retained_input:pending.retained_input.take(),output:std::mem::take(output),fuel_used:pending.fuel_used})),
                }})();
                match result{Ok(Some(invocation))=>{progress(invocation.fuel_used,started.elapsed());return Ok(invocation)},Ok(None)=>state.pending=Some(pending),Err(error)=>{state.pending=Some(pending);return Err(error)}}
            }
            CoreStepOutcome::Cancelled { fuel_used } => {
                pending.fuel_used=pending.fuel_used.saturating_add(fuel_used);
                progress(pending.fuel_used,started.elapsed());
                state.pending=Some(pending);
                return Err(TurnFault::Trapped(format!("owned operation cancelled after {fuel_used} instructions")));
            }
            CoreStepOutcome::Fault { fuel_used, error } => {
                pending.fuel_used=pending.fuel_used.saturating_add(fuel_used);
                progress(pending.fuel_used,started.elapsed());
                state.pending=Some(pending);
                state.poisoned=true;
                return Err(TurnFault::Trapped(owned_fault_text(state,&error.to_string())));
            }
        }
        let elapsed = started.elapsed();
        if completed_fuel >= next_progress_fuel || elapsed >= next_progress_elapsed {
            progress(completed_fuel, elapsed);
            next_progress_fuel = completed_fuel.saturating_add(25_000_000);
            next_progress_elapsed = elapsed.saturating_add(std::time::Duration::from_secs(5));
        }
    }
}

/// 🩺️ A guest fault, spoken in the guest's own words. `unreachable executed` from a
/// `panic = "abort"` component IS a Rust panic, and the panic message it printed a moment earlier is
/// the only thing that names the failing frame — so the retained tail of guest stderr travels with
/// the trap instead of being thrown away with the instance.
fn owned_fault_text(state: &OwnedGuestInstance, error: &str) -> String {
    let printed = String::from_utf8_lossy(&state.diagnostics);
    let printed = printed.trim();
    if printed.is_empty() {
        return error.to_string();
    }
    let tail = printed.char_indices().rev().nth(OWNED_FAULT_DIAGNOSTICS_TAIL_CHARS).map_or(printed, |(index, _)| &printed[index..]);
    format!("{error} — the guest printed: {tail}")
}

/// 🩺️ How much of the guest's own output a trap carries. A panic line plus its location is well
/// under this; the cap keeps a guest that logged a megabyte from burying the fault it ends with.
const OWNED_FAULT_DIAGNOSTICS_TAIL_CHARS: usize = 4_000;

pub(super) fn cancel_owned_operation(state: &mut OwnedGuestInstance) -> Result<(), TurnFault> {
    match state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.step(1, StepControl { cancelled: true }) {
        CoreStepOutcome::HostCall { call, .. } => {
            state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.resume_host(call.id, Err("owned operation cancelled".to_string()))?;
            if !matches!(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.step(1, StepControl { cancelled: true }), CoreStepOutcome::Cancelled { .. }) {
                return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string()));
            }
        }
        CoreStepOutcome::Cancelled { .. } => {}
        _ => return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string())),
    }
    state.pending = None;
    state.original_receipt=None;
    Ok(())
}
//#endregion 🧠️OwnedRuntime
