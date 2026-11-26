//! 📮️ One routed plugin call: a registration admits its own finite operation under the caller's driver and the paid result returns to the caller. See [`super`].

use super::*;
use std::fmt::Write;
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
    match failure {
        GuestRelayAdmissionFailure { closure: None, primary: Some(primary), settlement: None } => PluginHostError::OriginalTurn(Box::new(primary)),
        failure => PluginHostError::Plugin(format!("{failure:?}")),
    }
}

#[derive(Default)]
struct RefusalCause {
    kind: Option<ValueRefusalKind>,
    message: String,
    seen: bool,
}

impl RefusalCause {
    fn into_error(self) -> PluginHostError {
        if let Some(kind) = self.kind {
            PluginHostError::ProducerRefusal { kind, message: self.message }
        } else if self.seen {
            PluginHostError::Plugin(self.message)
        } else {
            PluginHostError::Plugin("plugin call failed without a primary cause".into())
        }
    }
}

struct CauseText<'a> {
    output: Option<&'a mut String>,
    remaining: usize,
}

impl Write for CauseText<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let count = value.floor_char_boundary(value.len().min(self.remaining));
        if let Some(output) = &mut self.output {
            output.push_str(&value[..count]);
        }
        self.remaining -= count;
        if count == value.len() { Ok(()) } else { Err(std::fmt::Error) }
    }
}

fn standalone_failure(failure: &GuestRelayAdmissionFailure) -> Option<&OriginalTurnFailure> {
    let primary = failure.primary.as_ref()?;
    (failure.closure.is_none() && failure.settlement.is_none() &&
        primary.receipt_refusal.is_none() && primary.context_refusal.is_none() && primary.work_refusal.is_none()).then_some(primary)
}

fn write_cause(writer: &mut impl Write, failure: &GuestRelayAdmissionFailure, repeated: bool) -> std::fmt::Result {
    if repeated { writer.write_str("; subsequent failure: ")?; }
    match standalone_failure(failure) {
        Some(primary) => writer.write_str(primary.primary.message()),
        None => write!(writer, "{failure:?}"),
    }
}

fn read_cause(out: &mut RefusalCause, failure: &GuestRelayAdmissionFailure, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let grant = cx.retained_grant();
    let metadata = std::mem::size_of::<Option<ValueRefusalKind>>() + std::mem::size_of::<bool>() + std::mem::size_of::<usize>();
    let limit = grant.maximum_copy_bytes.saturating_sub(metadata)
        .min(grant.maximum_capacity_bytes.saturating_add(out.message.capacity() - out.message.len()))
        .min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
    let mut measure = CauseText { output: None, remaining: limit };
    let _ = write_cause(&mut measure, failure, out.seen);
    let count = limit - measure.remaining;
    let previous_capacity = out.message.capacity();
    out.message.try_reserve_exact(count).map_err(|_| refused("plugin call cause allocation failed"))?;
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count + metadata, retained_capacity_bytes: out.message.capacity() - previous_capacity, ..Default::default() })?;
    cx.consume_fuel(u64::try_from(count).unwrap_or(u64::MAX) + 1);
    let _ = write_cause(&mut CauseText { output: Some(&mut out.message), remaining: count }, failure, out.seen);
    out.kind = if out.seen { None } else { standalone_failure(failure).and_then(OriginalTurnFailure::primary_value).map(|value| value.kind) };
    out.seen = true;
    Ok(())
}

fn compound_failure(primary: PluginHostError, secondary: PluginHostError) -> PluginHostError {
    PluginHostError::WithCleanupFailure { primary: Box::new(primary), cleanup: Box::new(secondary) }
}

fn refusal_error(retired: Result<(), GuestRelayAdmissionFailure>, cause: RefusalCause) -> PluginHostError {
    match retired {
        Ok(()) => cause.into_error(),
        Err(error) if cause.seen => compound_failure(cause.into_error(), failure_error(error)),
        Err(error) => failure_error(error),
    }
}

async fn receive_result(driver: &GuestRelayWakeDriver, result: &mut Option<GuestRelayResult>, kind: &str) -> Result<Vec<u8>, PluginHostError> {
    let mut receiving = Receiving { bytes: Vec::new(), length: None, cursor: 0 };
    let mut done = false;
    let mut succeeded = false;
    let mut cause = RefusalCause::default();
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
                            read_cause(&mut cause, failure, cx)?;
                            done = true;
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
    if succeeded { Ok(receiving.bytes) }
    else if cause.seen { Err(cause.into_error()) }
    else { Err(PluginHostError::Plugin(format!("{kind} job failed: {}", String::from_utf8_lossy(&receiving.bytes)))) }
}

/// 📮️ Admits one operation for `registration` on `driver`, runs the declared cold job over `input` and returns the paid result bytes. Pending polls keep every owner; cancellation travels through the driver.
pub async fn call_plugin(registration: &PluginRegistration, kind: &'static str, input: &[u8], driver: GuestRelayWakeDriver) -> Result<Vec<u8>, PluginHostError> {
    let mut admission = GuestRelayOperationAdmission::new(driver, registration.actor, registration.origin);
    let mut slot = None;
    if admission.admit(&mut slot).await != GuestRelayOperationStatus::Complete {
        let mut cause = RefusalCause::default();
        let retired = admission.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        let primary = refusal_error(retired, cause);
        if admission.close().await != GuestRelayOperationStatus::Complete {
            let mut cause = RefusalCause::default();
            let retired = admission.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
            let cleanup = refusal_error(retired, cause);
            admission.close().await;
            return Err(compound_failure(primary, cleanup));
        }
        return Err(primary);
    }
    let Some(mut operation) = slot else {
        return Err(PluginHostError::Plugin(format!("{kind} admission produced no operation")));
    };
    let outcome = if operation.run_bytes_from(kind, &mut GuestRelayBorrowedBytes(input)).await == GuestRelayOperationStatus::Complete {
        let (driver, result) = operation.result_parts();
        receive_result(driver, result, kind).await
    } else {
        let mut cause = RefusalCause::default();
        let retired = operation.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        Err(refusal_error(retired, cause))
    };
    if std::env::var_os("SEMIO_DEBUG_GUEST_RELAY_REFUSAL").is_some_and(|enabled| enabled == "1") {
        if let Err(error) = &outcome {
            let message = error.to_string();
            eprintln!("[DEBUG] guest relay phase=receive primary={:?}", &message[..message.floor_char_boundary(256)]);
        }
    }
    if operation.close().await != GuestRelayOperationStatus::Complete {
        let mut cause = RefusalCause::default();
        let retired = operation.retire_refusal(&mut |failure, cx| read_cause(&mut cause, failure, cx)).await;
        let cleanup = refusal_error(retired, cause);
        operation.close().await;
        return Err(match outcome { Ok(_) => cleanup, Err(primary) => compound_failure(primary, cleanup) });
    }
    outcome
}

#[cfg(test)]
#[test]
fn producer_refusal_paid_receiving_keeps_the_original_kind_and_compound_receipts() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_framework_value::RetainedCloneGrant;

    let grant = RetainedCloneGrant { maximum_items: 1024, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(63), Generation(1), grant, || Some(0), 1_000_000, 20_000);
    let mut compound = GuestRelayAdmissionFailure::from(ValueError::literal(ValueRefusalKind::WorkLimit, "producer work refused"));
    let mut projection = RefusalCause::default();
    caller.bare_turn(grant, |cx| read_cause(&mut projection, &compound, cx)).unwrap();
    let error = refusal_error(Ok(()), projection);
    assert_eq!(error.value_refusal_kind(), Some(ValueRefusalKind::WorkLimit));
    assert_eq!(error.to_string(), "producer work refused");
    let pair = compound_failure(error, PluginHostError::NativeIo(ValueError::literal(ValueRefusalKind::Canceled, "cleanup cancelled")));
    assert_eq!(pair.value_refusal_kind(), None);
    match pair {
        PluginHostError::WithCleanupFailure { primary, cleanup } => {
            assert_eq!(primary.value_refusal_kind(), Some(ValueRefusalKind::WorkLimit));
            assert_eq!(cleanup.value_refusal_kind(), Some(ValueRefusalKind::Canceled));
        }
        other => panic!("failure lost its causes: {other:?}"),
    }

    compound.primary.as_mut().unwrap().work_refusal = Some(ValueError::literal(ValueRefusalKind::InvariantViolated, "work settlement refused"));
    compound.settlement = Some(ValueError::literal(ValueRefusalKind::AllocationFailed, "additional settlement refused").into());
    let mut projection = RefusalCause::default();
    caller.bare_turn(grant, |cx| read_cause(&mut projection, &compound, cx)).unwrap();
    let error = refusal_error(Ok(()), projection);
    assert_eq!(error.value_refusal_kind(), None);
    let message = error.to_string();
    assert!(message.contains("producer work refused") && message.contains("work settlement refused") && message.contains("additional settlement refused"));
    caller.retire(grant);
}
