//! 📮️ One routed plugin call: a registration admits its own finite operation under the caller's driver and the paid result returns to the caller. See [`super`].

use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin_host_post_turn_relay::{GuestRelayAdmissionFailure, GuestRelayBorrowedBytes, GuestRelayFault, GuestRelayNativeOrigin, GuestRelayOperationAdmission, GuestRelayOperationStatus, GuestRelayResult, GuestRelayWakeDriver, RuntimeActorId};

pub use semio_plugin_guest_contract::job_kinds::{JOB_KIND_COMPOSE, JOB_KIND_INFER, JOB_KIND_IO_RUN, JOB_KIND_IO_SNIFF, JOB_KIND_MUTATION_PLAN};

/// 🏭️ Every call asks for its own driver, so no operation Source is shared between concurrent callers.
pub type PluginCallDrivers<'a> = &'a (dyn Fn() -> Result<GuestRelayWakeDriver, ValueError> + Send + Sync);

/// 🪪️ The immutable identity a router keeps for one loaded plugin; its instance is born inside each call's own Source.
#[derive(Clone, Copy)]
pub struct PluginRegistration {
    pub actor: RuntimeActorId,
    pub origin: GuestRelayNativeOrigin,
}

impl PartialEq for PluginRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.actor == other.actor && self.origin.cache_identity == other.origin.cache_identity && self.origin.package_hash == other.origin.package_hash
    }
}

struct Receiving {
    bytes: Vec<u8>,
    length: Option<usize>,
    cursor: usize,
}

impl Receiving {
    fn step(&mut self, source: &[u8], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        let word = std::mem::size_of::<usize>();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(false);
        }
        let Some(length) = self.length else {
            if grant.maximum_capacity_bytes < source.len() || grant.maximum_copy_bytes < word {
                return Ok(false);
            }
            self.bytes.try_reserve_exact(source.len()).map_err(|_| refused("plugin call result allocation failed"))?;
            self.length = Some(source.len());
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: word, retained_capacity_bytes: self.bytes.capacity(), ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(source.is_empty());
        };
        if length != source.len() {
            return Err(refused("plugin call result changed its extent").into());
        }
        let count = (length - self.cursor).min(grant.maximum_copy_bytes.saturating_sub(word)).min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
        if count == 0 {
            return Ok(self.cursor == length);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count + word, ..Default::default() })?;
        cx.consume_fuel(u64::try_from(count).unwrap_or(u64::MAX) + 1);
        self.bytes.extend_from_slice(&source[self.cursor..self.cursor + count]);
        self.cursor += count;
        Ok(self.cursor == length)
    }
}

fn refused(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}

fn failure_error(failure: GuestRelayAdmissionFailure) -> PluginHostError {
    match failure.primary {
        Some(primary) => PluginHostError::OriginalTurn(primary),
        None => PluginHostError::Plugin("plugin call failed without a primary cause".into()),
    }
}

fn read_cause(out: &mut Vec<u8>, failure: &GuestRelayAdmissionFailure, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let message = failure.primary.as_ref().map_or("plugin call failed without a primary cause", |primary| primary.primary.message());
    let grant = cx.retained_grant();
    let word = std::mem::size_of::<usize>();
    let count = message.len().min(grant.maximum_copy_bytes.saturating_sub(word)).min(grant.maximum_capacity_bytes).min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
    out.try_reserve_exact(count).map_err(|_| refused("plugin call cause allocation failed"))?;
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count + word, retained_capacity_bytes: out.capacity(), ..Default::default() })?;
    cx.consume_fuel(u64::try_from(count).unwrap_or(u64::MAX) + 1);
    out.extend_from_slice(&message.as_bytes()[..count]);
    Ok(())
}

fn refusal_error(retired: Result<(), GuestRelayAdmissionFailure>, cause: Vec<u8>) -> PluginHostError {
    match retired {
        Ok(()) => PluginHostError::Plugin(String::from_utf8_lossy(&cause).into_owned()),
        Err(error) => failure_error(error),
    }
}

async fn receive_result(driver: &GuestRelayWakeDriver, result: &mut Option<GuestRelayResult>) -> Result<Result<Vec<u8>, Vec<u8>>, PluginHostError> {
    let mut receiving = Receiving { bytes: Vec::new(), length: None, cursor: 0 };
    let mut done = false;
    let mut succeeded = false;
    while !done {
        driver
            .dispatch_operation_step(false, false, |cx| {
                match result.as_mut().ok_or_else(|| refused("plugin call result slot is empty"))? {
                    GuestRelayResult::Success(output) => {
                        succeeded = true;
                        output.with_bytes(cx, |bytes, cx| {
                            done = receiving.step(bytes, cx)?;
                            Ok(())
                        })?;
                    }
                    GuestRelayResult::Failure(fault @ GuestRelayFault::Native { .. }) => {
                        fault.with_fault(cx, |failure, cx| {
                            let message = failure.primary.as_ref().map_or("native plugin job failure", |primary| primary.primary.message());
                            done = receiving.step(message.as_bytes(), cx)?;
                            Ok(())
                        })?;
                    }
                    GuestRelayResult::Failure(fault) => {
                        fault.with_guest_fault_bytes(cx, |bytes, cx| {
                            done = receiving.step(bytes, cx)?;
                            Ok(())
                        })?;
                    }
                }
                Ok(())
            })
            .map_err(failure_error)?;
        if !done {
            semio_framework_async::yield_once().await;
        }
    }
    Ok(if succeeded { Ok(receiving.bytes) } else { Err(receiving.bytes) })
}

/// 📮️ Admits one operation for `registration` on `driver`, runs the declared cold job over `input` and returns the paid result bytes. Pending polls keep every owner; cancellation travels through the driver.
pub async fn call_plugin(registration: &PluginRegistration, kind: &'static str, input: &[u8], driver: GuestRelayWakeDriver) -> Result<Vec<u8>, PluginHostError> {
    let mut admission = GuestRelayOperationAdmission::new(driver, registration.actor, registration.origin);
    let mut slot = None;
    if admission.admit(&mut slot).await != GuestRelayOperationStatus::Complete {
        let mut cause = Vec::new();
        let retired = admission.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        admission.close().await;
        return Err(refusal_error(retired, cause));
    }
    let Some(mut operation) = slot else {
        return Err(PluginHostError::Plugin(format!("{kind} admission produced no operation")));
    };
    let outcome = if operation.run_bytes_from(kind, &mut GuestRelayBorrowedBytes(input)).await == GuestRelayOperationStatus::Complete {
        let (driver, result) = operation.result_parts();
        receive_result(driver, result).await
    } else {
        let mut cause = Vec::new();
        let retired = operation.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        Err(refusal_error(retired, cause))
    };
    if operation.close().await != GuestRelayOperationStatus::Complete {
        let mut cause = Vec::new();
        let retired = operation.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        let cleanup = refusal_error(retired, cause);
        operation.close().await;
        return Err(outcome.err().unwrap_or(cleanup));
    }
    match outcome? {
        Ok(bytes) => Ok(bytes),
        Err(fault) => Err(PluginHostError::Plugin(format!("{kind} job failed: {}", String::from_utf8_lossy(&fault)))),
    }
}
