//! 🔁️ Source-owned Poll transport over the real input, scalar and unique-output frontiers.
use semio_framework_actor::{OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobBudget};
use semio_framework_job::{OriginalCallerReturnPlace, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_job_native_decode::{OriginalNativeDecodeOutcome, OriginalNativeDecodePolicy};
use semio_framework_job_return_escrow_publication::OriginalEscrowPublisher;
use semio_framework_pack_json::{JsonReadLimits, JsonWriteSource};
use semio_framework_pack_json_output_receiving::{OriginalJsonOutputClose, OriginalJsonOutputOutcome, OriginalJsonOutputSource};
use semio_framework_value::{DslValue, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_native_owned::NativeReceive;
use semio_plugin_guest_contract::{operation_authority::owned_result::{self, OwnedReturn}, original_actor_context::{OriginalActorFixedCall, OriginalActorFixedKind, OriginalActorFixedReceipt}};
use semio_plugin_host_guest_runtime::{CompiledHandle, GuestInstance, GuestRuntimes, OriginalCoreValue, OwnedOriginalCallProgress, OwnedOriginalCallResult, OwnedOriginalCallStatus, OwnedSemioExport};
use semio_plugin_host_post_turn_relay::{GuestRelayUnitFlow, GuestRelayUnitStatus, NativeInstanceRetirement};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Encode, Input, Poll, ReleaseHeader, Decode, ParserClose, ReleaseOutput, CloseOutput, CloseReceipt, Deliver, RetireCapture, CloseDecoder, Complete, Fault }

/// 🚦 The caller drives one bounded slice and observes every execution receipt before the next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestPollStatus { Pending, AwaitBudget, Frozen, Output, Complete, Refused }

/// 🪙️ The issuer holds one purpose-scoped loan until its actual guest receipt is received.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestPollGuestPurpose { Infrastructure, Semantic }
/// 🧾️ The issuer reserves for the exact fixed header of this Core crossing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuestPollGuestLoan { pub purpose: GuestPollGuestPurpose, pub header_bytes: usize }
/// 🧊️ The actual receipt frontier remains frozen until Source-ledger settlement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestPollFrozenKind { Input, Poll, CloseOutput }

/// 🧳 The actual input, guest result, decoder and captured value remain in one Source owner.
pub struct GuestPoll<T: NativeReceive<DslValue> + RetireOwned> {
    binding: Option<OriginalSourceBinding>,
    home_address: Option<usize>,
    unit: GuestRelayUnitFlow,
    decoder: OriginalJsonOutputSource<T>,
    captured: Option<T>,
    capture_close: Option<ControlledRetirement<T>>,
    outcome: Option<ControlledRetirement<OriginalJsonOutputOutcome>>,
    settlement: Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>,
    return_place: OriginalCallerReturnPlace,
    scalar_progress: Option<OwnedOriginalCallProgress>,
    scalar_result: Option<OwnedOriginalCallResult>,
    transport_failure: Option<OriginalTurnFailure>,
    fixed: Option<OriginalActorFixedCall>,
    close_kind: Option<OriginalActorFixedKind>,
    budget: Option<OriginalJobBudget>,
    input_budget: Option<OriginalJobBudget>,
    raw_word: Option<u64>,
    input_progress_pending: bool,
    awaiting_input_budget: bool,
    prepared: Option<u64>,
    pointer: u32,
    length: u32,
    phase: Phase,
    scalar_started: bool,
    failed: bool,
    resident: bool,
    output_captured: bool,
    capsule: Option<(u32,u32)>,
    frozen: bool,
}

fn invalid(message: &'static str) -> OriginalTurnFailure { ValueError::literal(ValueRefusalKind::InvariantViolated, message).into() }
fn require_core_wire_reserve(budget:OriginalJobBudget,header:usize)->Result<(),OriginalTurnFailure>{let demand=OriginalEscrowPublisher::core_wire_reserve(header)?;for grant in [budget.input.grant,budget.retained_policy]{if grant.maximum_items<demand.items||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Poll Core crossing lacks its mandatory escrow reserve").into())}}if budget.input.work.fuel<demand.work||budget.fuel<demand.work{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Poll Core crossing lacks its mandatory escrow work").into())}Ok(())}
fn charge(cx: &mut StepContext<'_>, retiring: bool) -> Result<bool, OriginalTurnFailure> {
    let grant = cx.retained_grant();
    let bytes = std::mem::size_of::<Phase>() + std::mem::size_of::<Option<OriginalSourceBinding>>() + std::mem::size_of::<Option<u64>>() + std::mem::size_of::<Option<usize>>();
    if (if retiring { cx.retirement_should_yield() } else { cx.should_yield() || cx.is_cancelled() }) || grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}
fn charge_capture<T: RetireOwned>(cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    let bytes=std::mem::size_of::<Option<T>>() + std::mem::size_of::<Option<ControlledRetirement<T>>>();
    let grant=cx.retained_grant();
    if cx.retirement_should_yield() || grant.maximum_items==0 || grant.maximum_copy_bytes<bytes || grant.maximum_depth==0 { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
fn charge_frame(cx: &mut StepContext<'_>, bytes: usize) -> Result<bool, OriginalTurnFailure> {
    let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}

impl<T: NativeReceive<DslValue> + RetireOwned> GuestPoll<T> {
    /// 🌱️ No guest allocation or decoder Source is admitted by construction.
    pub fn new(policy: OriginalNativeDecodePolicy, limits: JsonReadLimits) -> Self {
        Self { binding: None, home_address: None, unit: GuestRelayUnitFlow::empty(), decoder: OriginalJsonOutputSource::new(policy, limits), captured: None, capture_close: None, outcome: None, settlement: None, return_place: OriginalCallerReturnPlace::empty(), scalar_progress: None, scalar_result: None, transport_failure: None, fixed: None, close_kind: None, budget: None, input_budget: None, raw_word: None, input_progress_pending: false, awaiting_input_budget: false, prepared: None, pointer: 0, length: 0, phase: Phase::Encode, scalar_started: false, failed: false, resident: false, output_captured: false, capsule: None, frozen: false }
    }
    fn validate(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.binding.is_some_and(|binding| !binding.matches(cx)) { return Err(invalid("Poll changed its original Source")); }
        if self.home_address.is_some_and(|address| address != self as *const Self as usize) { return Err(invalid("Poll moved after binding its Source-owned frame")); }
        Ok(())
    }
    pub fn requires_execution(&self, runtime: &GuestRuntimes, instance: &GuestInstance) -> Result<bool, OriginalTurnFailure> {
        match self.phase { Phase::Input => self.unit.requires_execution(runtime, instance), Phase::Poll | Phase::ReleaseHeader | Phase::ReleaseOutput | Phase::CloseOutput | Phase::CloseReceipt if self.scalar_started => runtime.original_scalar_requires_execution(instance), _ => Ok(false) }
    }
    /// 🧭️ Describes the purpose of the in-flight guest crossing; it never issues or renews a loan.
    pub fn guest_purpose(&self) -> Option<GuestPollGuestPurpose> {
        match self.phase { Phase::Input | Phase::CloseOutput | Phase::CloseReceipt => Some(GuestPollGuestPurpose::Infrastructure), Phase::Poll => Some(if self.output_captured { GuestPollGuestPurpose::Infrastructure } else { GuestPollGuestPurpose::Semantic }), _ => None }
    }
    /// 🧭️ Reveals only the current actual frozen frontier under its Source grant.
    pub fn frozen_kind_step(&self,cx:&mut StepContext<'_>)->Result<Option<Option<GuestPollFrozenKind>>,OriginalTurnFailure>{self.validate(cx)?;let retirement=cx.original_retirement_active();if !charge(cx,retirement)?{return Ok(None)}Ok(Some(if self.frozen&&!self.input_progress_pending&&self.scalar_progress.is_none(){Some(match self.phase{Phase::Input=>GuestPollFrozenKind::Input,Phase::Poll=>GuestPollFrozenKind::Poll,Phase::CloseOutput=>GuestPollFrozenKind::CloseOutput,_=>return Err(invalid("Poll froze outside a receipt frontier"))})}else{None}))}
    /// 🌱️ Only these phase boundaries admit a new real guest subloan.
    pub fn next_guest_subloan_step(&self, cx: &mut StepContext<'_>) -> Result<Option<Option<GuestPollGuestLoan>>, OriginalTurnFailure> {
        self.validate(cx)?;
        let retirement = cx.original_retirement_active();
        if !charge(cx, retirement)? { return Ok(None); }
        if self.frozen || self.return_place.is_admitted() || self.outcome.is_some() || self.settlement.is_some() || self.fixed.is_some() || self.raw_word.is_some() || self.scalar_progress.is_some() || self.scalar_result.is_some() || self.transport_failure.is_some() || self.input_progress_pending { return Ok(Some(None)); }
        Ok(Some(match self.phase { Phase::Input if self.unit.needs_admission_budget() && self.input_budget.is_none() || self.awaiting_input_budget => Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:144}), Phase::Poll if !self.scalar_started => Some(GuestPollGuestLoan{purpose:if self.output_captured { GuestPollGuestPurpose::Infrastructure } else { GuestPollGuestPurpose::Semantic },header_bytes:152}), Phase::CloseOutput if !self.scalar_started => Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:144}), _ => None }))
    }
    /// 🎟️ Settles controller and cleanup metadata before the caller issues the same turn's guest loan and VM credit.
    pub fn prepare_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.phase == Phase::Complete && self.binding.is_none() { return Ok(true); }
        let epoch = cx.original_source_epoch().ok_or_else(|| invalid("Poll preparation requires its actual epoch"))?;
        if self.prepared == Some(epoch) { return Ok(true); }
        let retirement = cx.original_retirement_active();
        if !charge(cx, retirement)? { return Ok(false); }
        self.binding = Some(OriginalSourceBinding::borrow(cx));
        self.home_address = Some(self as *const Self as usize);
        if self.requires_execution(runtime, instance)? {
            if retirement && !runtime.prepare_original_retirement_step(instance, cx)? { return Ok(false); }
            if self.phase == Phase::Input && !self.unit.prepare_execution_step(cx)? { return Ok(false); }
        }
        self.prepared = Some(epoch);
        Ok(true)
    }
    pub fn execution_receipt(&self) -> Option<OriginalGuestExecutionReceipt> {
        if self.phase == Phase::Input { self.unit.execution_receipt() } else { self.scalar_progress.as_ref().and_then(|progress| progress.execution) }
    }
    pub fn scalar_progress(&self) -> Option<&OwnedOriginalCallProgress> { self.scalar_progress.as_ref() }
    pub fn scalar_result(&self) -> Option<&OwnedOriginalCallResult> { self.scalar_result.as_ref() }
    pub fn transport_failure(&self) -> Option<&OriginalTurnFailure> { self.transport_failure.as_ref() }
    /// 📤️ The original recipient takes the actual transport failure before instance retirement.
    pub fn take_transport_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OriginalTurnFailure>, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        if self.transport_failure.is_none() { return Ok(false); }
        if !charge(cx, true)? { return Ok(false); }
        receive(&mut self.transport_failure, cx)?;
        Ok(self.transport_failure.is_none())
    }
    /// 🎟️ The caller observes the whole scalar result and its original input before release.
    pub fn acknowledge_scalar_result_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.scalar_result.is_none() { return Ok(true); }
        if !charge(cx, true)? { return Ok(false); }
        self.scalar_result.take();
        Ok(true)
    }
    /// 🧾️ The caller observes the whole successful VM receipt before its fixed owner is released.
    pub fn acknowledge_scalar_progress_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        let Some(progress) = self.scalar_progress.as_ref() else { return Ok(true); };
        if progress.result.is_err() { return Err(invalid("Poll retains its original scalar failure")); }
        if !charge(cx, true)? { return Ok(false); }
        self.scalar_progress.take();
        Ok(true)
    }
    /// 📤️ Transfers an actual scalar failure only to an explicit original recipient.
    pub fn take_scalar_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OwnedOriginalCallProgress>, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.scalar_progress.as_ref().is_none_or(|progress| progress.result.is_ok()) { return Ok(false); }
        if !charge(cx, true)? { return Ok(false); }
        receive(&mut self.scalar_progress, cx)?;
        Ok(self.scalar_progress.is_none())
    }
    pub fn input_failure(&self) -> Option<&OriginalTurnFailure> { self.unit.failure() }
    pub fn input_progress(&self) -> Option<&OwnedOriginalCallProgress> { self.unit.progress() }
    pub fn input_progress_pending(&self) -> bool { self.input_progress_pending }
    /// 🎟️ The next input slice cannot replace its preceding whole VM receipt before observation.
    pub fn acknowledge_input_progress_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if !self.input_progress_pending { return Ok(true); }
        if self.unit.failure().is_some() { return Err(invalid("Poll retains its original input failure")); }
        if !charge(cx, true)? { return Ok(false); }
        self.input_progress_pending = false;
        Ok(true)
    }
    /// 📤️ Transfers the real input failure to the original recipient without rebuilding it.
    pub fn take_input_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OriginalTurnFailure>, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> { self.validate(cx)?; self.unit.take_failure_step(cx, receive) }
    pub fn decoding_outcome(&self) -> Option<&OriginalJsonOutputOutcome> { self.outcome.as_ref().and_then(ControlledRetirement::original) }
    pub fn settlement_outcome(&self) -> Option<&OriginalNativeDecodeOutcome<RetainedCloneStep>> { self.settlement.as_ref().and_then(ControlledRetirement::original) }
    pub fn fixed_receipt(&self) -> Option<&OriginalActorFixedCall> { self.fixed.as_ref() }
    pub fn raw_word(&self) -> Option<u64> { self.raw_word }
    /// 🧾️ Releases an observed fixed receipt only after its original metadata is paid.
    pub fn acknowledge_fixed_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.fixed.is_none() { return Ok(true); }
        if !charge(cx, true)? { return Ok(false); }
        self.fixed.take();
        Ok(true)
    }
    /// 🪪️ Transfers the exact observed scalar word before another export can replace it.
    pub fn take_raw_word_step(&mut self, cx: &mut StepContext<'_>) -> Result<Option<u64>, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.raw_word.is_none() || !charge(cx, true)? { return Ok(None); }
        Ok(self.raw_word.take())
    }
    /// 👁️ Admits the callback before borrowing the captured output and starting its retirement exactly once.
    pub fn deliver_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&T, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.phase != Phase::Deliver { return Err(invalid("Poll output acknowledgement is out of phase")); }
        if cx.is_cancelled() { return Ok(false); }
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        let address=(&mut self.transport_failure as *mut _) as usize;let bytes=std::mem::size_of_val(&self.transport_failure);
        if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(false); }
        if !charge_capture::<T>(cx)? { return Ok(false); }
        let result = receive(self.captured.as_ref().ok_or_else(|| invalid("Poll lost its captured output"))?, cx);
        if let Err(error) = result { self.transport_failure = Some(error); self.return_place.record_sized_return(address,bytes)?; }
        self.capture_close = Some(ControlledRetirement::retain(self.captured.take().ok_or_else(|| invalid("Poll lost its captured output"))?));
        self.phase = Phase::RetireCapture;
        Ok(true)
    }
    /// 🛑️ A cancelled capture retires on the same live decoder without invoking the callback.
    pub fn discard_cancelled_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.phase != Phase::Deliver || !cx.is_cancelled() { return Err(invalid("Poll capture discard requires its actual cancellation")); }
        if !charge_capture::<T>(cx)? { return Ok(false); }
        self.capture_close = Some(ControlledRetirement::retain(self.captured.take().ok_or_else(|| invalid("Poll lost its captured output"))?));
        self.phase = Phase::RetireCapture;
        Ok(true)
    }
    /// ♻️ Retires the complete raw decoder result, including its primary and settlement, on the live decoder Source.
    pub fn retire_outcome_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        if let Some(settlement) = self.settlement.as_mut() {
            if !settlement.terminal_is_empty() {
                let result = settlement.step(cx.retained_grant());
                let work = settlement.last_turn_work();
                semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome { result, work })?;
                return Ok(false);
            }
            if !charge_frame(cx,std::mem::size_of_val(&self.settlement))? { return Ok(false); }
            self.settlement.take();
            return Ok(false);
        }
        if let Some(outcome) = self.outcome.as_mut() {
            if !outcome.terminal_is_empty() {
                let address = (&mut self.settlement as *mut _) as usize;
                let bytes = std::mem::size_of_val(&self.settlement);
                if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(false); }
                if let Some(receipt) = self.decoder.close_owned_step(outcome, cx)? { self.settlement = Some(ControlledRetirement::retain(receipt)); self.return_place.record_sized_return(address,bytes)?; }
                return Ok(false);
            }
            if !charge_frame(cx,std::mem::size_of_val(&self.outcome))? { return Ok(false); }
            self.outcome.take();
        }
        Ok(true)
    }
    /// ▶️ `input` is the actual borrowed tagged Poll JSON source, never a projected owned DTO.
    pub fn advance_step<S: JsonWriteSource + ?Sized>(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, input: &S, budget: Option<OriginalJobBudget>, execution: Option<OriginalGuestExecutionBudget>, cx: &mut StepContext<'_>) -> Result<GuestPollStatus, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(GuestPollStatus::Pending); }
        if self.input_progress_pending || self.scalar_progress.is_some() || self.scalar_result.is_some() || self.fixed.is_some() || self.raw_word.is_some() || self.transport_failure.is_some() { return Ok(GuestPollStatus::Pending); }
        if self.frozen { return Ok(GuestPollStatus::Frozen); }
        if self.awaiting_input_budget { return Ok(GuestPollStatus::AwaitBudget); }
        if self.outcome.is_some() || self.settlement.is_some() { return Ok(GuestPollStatus::Pending); }
        if self.phase == Phase::Complete { if self.binding.is_some() { if !charge(cx,true)? { return Ok(GuestPollStatus::Pending); } self.binding=None;self.home_address=None;self.prepared=None; } return Ok(GuestPollStatus::Complete); }
        if self.phase == Phase::Fault { return Ok(GuestPollStatus::Refused); }
        if self.phase == Phase::Deliver { return Ok(GuestPollStatus::Output); }
        if self.prepared.is_none() || self.prepared != cx.original_source_epoch() { return Err(invalid("Poll controller metadata must settle before guest authorization")); }
        if self.phase == Phase::Input && self.unit.needs_admission_budget() { require_core_wire_reserve(budget.ok_or_else(|| invalid("Poll input requires its issued infrastructure loan"))?,144)?; }
        if matches!(self.phase,Phase::Poll|Phase::CloseOutput) && !self.scalar_started { require_core_wire_reserve(budget.ok_or_else(|| invalid("Poll export requires its issued guest subloan"))?,if self.phase==Phase::Poll{152}else{144})?; }
        self.prepared = None;
        if self.phase == Phase::Decode && cx.is_cancelled() && cx.original_retirement_active() { self.failed = true; self.phase = Phase::ParserClose; }
        match self.phase {
            Phase::Encode => { if execution.is_some() { return Err(invalid("Poll encoding cannot consume VM execution")); } if self.unit.encode_step(input, cx)? { self.phase = Phase::Input; } }
            Phase::Input => {
                if self.unit.needs_admission_budget() { self.input_budget = Some(budget.ok_or_else(|| invalid("Poll input requires its issued infrastructure loan"))?); }
                let input_budget = self.input_budget.ok_or_else(|| invalid("Poll input lost its original infrastructure loan"))?;
                let status = self.unit.advance_step(runtime, instance, input_budget, execution, cx)?;
                if self.unit.progress().is_some() { self.input_progress_pending = true; }
                match status {
                    GuestRelayUnitStatus::Output => {
                        self.frozen = true;
                    }
                    GuestRelayUnitStatus::AwaitBudget => { self.awaiting_input_budget = true; return Ok(GuestPollStatus::AwaitBudget); }
                    GuestRelayUnitStatus::Complete => { self.input_budget = None; self.phase = if self.failed { Phase::Fault } else { Phase::Poll }; },
                    GuestRelayUnitStatus::Refused => { self.failed = true; self.phase = Phase::Fault; }
                    GuestRelayUnitStatus::Pending => {}
                }
            }
            Phase::Poll | Phase::ReleaseHeader | Phase::ReleaseOutput | Phase::CloseOutput | Phase::CloseReceipt => self.scalar_step(runtime, instance, budget, execution, cx)?,
            Phase::Decode => {
                if execution.is_some() { return Err(invalid("Poll receiving cannot consume VM execution")); }
                let address = (&mut self.outcome as *mut _) as usize;
                let bytes = std::mem::size_of_val(&self.outcome);
                if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(GuestPollStatus::Pending); }
                let mut outcome = None;
                if self.decoder.ready() { outcome = self.decoder.take_step(&mut self.captured, cx)?; }
                else { runtime.with_original_scalar_bytes(instance, self.pointer, self.length as usize, cx, |bytes, cx| { outcome = self.decoder.receive_step(bytes, cx)?; Ok(()) })?; }
                if let Some(raw) = outcome { if raw.value.is_err() || raw.receiving_refusal.is_some() { self.failed = true; self.phase = Phase::ParserClose; } self.outcome = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; }
                if self.captured.is_some() { self.phase = Phase::CloseOutput; }
            }
            Phase::ParserClose => { let address=(&mut self.settlement as *mut _) as usize;let bytes=std::mem::size_of_val(&self.settlement);if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(GuestPollStatus::Pending); } match self.decoder.close_step(cx)? {
                OriginalJsonOutputClose::Producer(Some(raw)) => { self.settlement = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; },
                OriginalJsonOutputClose::Producer(None) => {},
                OriginalJsonOutputClose::Source(_) if self.decoder.terminal_is_empty() => self.phase = Phase::CloseOutput,
                OriginalJsonOutputClose::Source(_) => {},
            } },
            Phase::RetireCapture => {
                let owner = self.capture_close.as_mut().ok_or_else(|| invalid("Poll lost captured retirement"))?;
                if owner.terminal_is_empty() { if charge_frame(cx,std::mem::size_of::<Option<ControlledRetirement<T>>>())? { self.capture_close.take(); self.phase = Phase::CloseDecoder; } }
                else { let address=(&mut self.settlement as *mut _) as usize;let bytes=std::mem::size_of_val(&self.settlement);if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(GuestPollStatus::Pending); } if let Some(raw) = self.decoder.close_owned_step(owner, cx)? { self.settlement = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; } }
            }
            Phase::CloseDecoder => { let address=(&mut self.settlement as *mut _) as usize;let bytes=std::mem::size_of_val(&self.settlement);if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(GuestPollStatus::Pending); } match self.decoder.close_step(cx)? {
                OriginalJsonOutputClose::Producer(Some(raw)) => { self.settlement = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; },
                OriginalJsonOutputClose::Producer(None) => {},
                OriginalJsonOutputClose::Source(_) if self.decoder.terminal_is_empty() => { self.budget = None; self.phase = Phase::Complete; },
                OriginalJsonOutputClose::Source(_) => {},
            } },
            _ => return Err(invalid("Poll phase cannot advance")),
        }
        Ok(if self.frozen && !self.input_progress_pending && self.scalar_progress.is_none() { GuestPollStatus::Frozen } else { match self.phase { Phase::Complete if self.terminal_is_empty() => GuestPollStatus::Complete, Phase::Deliver => GuestPollStatus::Output, Phase::Fault => GuestPollStatus::Refused, _ => GuestPollStatus::Pending } })
    }
    /// 🔄️ Renews the exact retained input borrow only after its pending admission receipt.
    pub fn repeat_input_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, budget: OriginalJobBudget, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> { self.validate(cx)?; if self.input_progress_pending { return Ok(false); } if !self.awaiting_input_budget { return Err(invalid("Poll input is not awaiting a renewed budget")); } require_core_wire_reserve(budget,144)?; if self.unit.repeat_step(runtime, instance, budget, cx)? { self.input_budget = Some(budget); self.awaiting_input_budget = false; return Ok(true); } Ok(false) }
    fn scalar_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, budget: Option<OriginalJobBudget>, execution: Option<OriginalGuestExecutionBudget>, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if !self.scalar_started {
            if execution.is_some() { return Err(invalid("Poll scalar admission cannot consume VM execution")); }
            let mut args = [OriginalCoreValue::I64(0); OriginalJobBudget::SCALAR_ARGUMENT_COUNT];
            let export = match self.phase {
                Phase::Poll | Phase::CloseOutput => { let issued = budget.ok_or_else(|| invalid("Poll export requires its issued guest subloan"))?; require_core_wire_reserve(issued,if self.phase==Phase::Poll{152}else{144})?; for (slot, value) in args.iter_mut().zip(issued.scalar_arguments()) { *slot = OriginalCoreValue::I64(value as i64); } if self.phase == Phase::Poll { OwnedSemioExport::Poll } else { OwnedSemioExport::CloseActorOutput } },
                Phase::ReleaseHeader | Phase::ReleaseOutput | Phase::CloseReceipt => { args[0] = OriginalCoreValue::I32(self.pointer as i32); args[1] = OriginalCoreValue::I32(self.length as i32); OwnedSemioExport::Deallocate },
                _ => return Err(invalid("Poll scalar export is out of phase")),
            };
            let count = if export == OwnedSemioExport::Deallocate { 2 } else { OriginalJobBudget::SCALAR_ARGUMENT_COUNT };
            if runtime.begin_original_scalar_step(instance, export, &args[..count], cx)? { self.scalar_started = true; if export != OwnedSemioExport::Deallocate { self.budget = budget; } }
            return Ok(());
        }
        let progress = runtime.advance_original_scalar_step(instance, execution, cx)?;
        let ready = matches!(&progress.result, Ok(OwnedOriginalCallStatus::Ready));
        let refused = progress.result.is_err() || matches!(&progress.result, Ok(OwnedOriginalCallStatus::Cancelled | OwnedOriginalCallStatus::Deadline));
        self.scalar_progress = Some(progress);
        if refused { self.failed = true; self.phase = Phase::Fault; return Ok(()); }
        if !ready { return Ok(()); }
        if matches!(self.phase,Phase::Poll|Phase::CloseOutput) { self.frozen=true;return Ok(()); }
        let mut result = None;
        if !runtime.take_original_scalar_step(instance, cx, |value, _| { result = Some(value); Ok(()) })? { return Ok(()); }
        let result = result.ok_or_else(|| invalid("Poll scalar returned no actual result"))?;
        self.scalar_result = Some(result);
        self.scalar_started = false;
        match self.phase {
            Phase::Poll | Phase::CloseOutput => return Err(invalid("frozen receipt requires Source escrow settlement")),
            Phase::ReleaseHeader => { if result.value.is_some() { return Err(invalid("Poll header deallocator returned a value")); } if self.failed { self.capsule = None; self.phase = Phase::CloseOutput; } else if let Some((pointer,length)) = self.capsule.take() { self.pointer = pointer; self.length = length; self.phase = Phase::Decode; } else { self.phase = Phase::Poll; } }
            Phase::ReleaseOutput => { if result.value.is_some() { return Err(invalid("Poll output deallocator returned a value")); } self.phase = if self.resident { Phase::CloseOutput } else { Phase::CloseDecoder }; }
            Phase::CloseReceipt => {
                if result.value.is_some() { return Err(invalid("Poll close receipt deallocator returned a value")); }
                let Some(kind) = self.close_kind.take() else { self.phase = Phase::Fault; return Ok(()); };
                match kind {
                    OriginalActorFixedKind::Close { complete: true, blocked: false, refusal: None } => { if self.failed && self.decoder.terminal_is_empty() { self.budget = None; self.phase = Phase::Complete; } else { self.phase = if self.failed { Phase::CloseDecoder } else { Phase::Deliver }; } },
                    OriginalActorFixedKind::Close { complete: false, refusal: None, .. } => self.phase = Phase::CloseOutput,
                    _ => { self.failed = true; self.phase = Phase::Fault; }
                }
            }
            _ => return Err(invalid("Poll scalar result is out of phase")),
        }
        Ok(())
    }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Complete && self.binding.is_none() && self.home_address.is_none() && self.return_place.terminal_is_empty() && self.budget.is_none() && self.input_budget.is_none() && self.unit.terminal_is_empty() && self.decoder.terminal_is_empty() && self.captured.is_none() && self.capture_close.is_none() && self.outcome.is_none() && self.settlement.is_none() && self.scalar_progress.is_none() && self.scalar_result.is_none() && self.transport_failure.is_none() && self.fixed.is_none() && self.raw_word.is_none() && self.prepared.is_none() && self.capsule.is_none() && !self.input_progress_pending && !self.scalar_started && !self.frozen }
    fn close_payload_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        if self.input_progress_pending || self.scalar_progress.is_some() || self.scalar_result.is_some() || self.transport_failure.is_some() || self.fixed.is_some() || self.raw_word.is_some() || self.outcome.is_some() || self.settlement.is_some() { return Err(invalid("Poll retains an unreceived actual result")); }
        if self.captured.is_some() { if !charge_capture::<T>(cx)? { return Ok(false); } self.capture_close = Some(ControlledRetirement::retain(self.captured.take().unwrap())); return Ok(false); }
        if let Some(owner) = self.capture_close.as_mut() {
            if !owner.terminal_is_empty() { let address=(&mut self.settlement as *mut _) as usize;let bytes=std::mem::size_of_val(&self.settlement);if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(false); } if let Some(raw) = self.decoder.close_owned_step(owner, cx)? { self.settlement = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; } return Ok(false); }
            if !charge_frame(cx,std::mem::size_of::<Option<ControlledRetirement<T>>>())? { return Ok(false); }
            self.capture_close.take();
        }
        if !self.decoder.terminal_is_empty() {
            let address=(&mut self.settlement as *mut _) as usize;let bytes=std::mem::size_of_val(&self.settlement);if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(false); } if let OriginalJsonOutputClose::Producer(Some(raw)) = self.decoder.close_step(cx)? { self.settlement = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; }
            return Ok(false);
        }
        if !charge(cx, true)? { return Ok(false); }
        self.awaiting_input_budget = false;
        self.budget = None;
        self.input_budget = None;
        self.capsule = None;
        self.prepared = None;
        self.binding = None;
        self.home_address = None;
        self.phase = Phase::Complete;
        Ok(self.terminal_is_empty())
    }
}
impl<T: NativeReceive<DslValue> + RetireOwned> NativeInstanceRetirement for GuestPoll<T> {
    fn close_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, _: &CompiledHandle, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.terminal_is_empty() { return Ok(true); }
        if self.frozen { return Err(invalid("Poll must settle its frozen receipt before retirement")); }
        if self.input_progress_pending || self.unit.failure().is_some() { return Err(invalid("Poll retains its unreceived input result")); }
        if self.scalar_started { if !runtime.close_original_scalar_step(instance, cx)? { return Ok(false); } self.scalar_started = false; return Ok(false); }
        if !self.unit.terminal_is_empty() { if !self.unit.close_step(runtime, instance, cx)? { return Ok(false); } return Ok(false); }
        self.close_payload_step(cx)
    }
    fn close_unopened(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.frozen { return Err(invalid("Poll must settle its frozen receipt before retirement")); }
        if self.input_progress_pending || self.unit.failure().is_some() { return Err(invalid("Poll retains its unreceived input result")); }
        if !self.unit.terminal_is_empty() { if !self.unit.close_unopened(cx)? { return Ok(false); } return Ok(false); }
        self.close_payload_step(cx)
    }
    fn terminal_is_empty(&self) -> bool { GuestPoll::terminal_is_empty(self) }
}
impl<T: NativeReceive<DslValue> + RetireOwned> Drop for GuestPoll<T> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "Poll requires complete original Source retirement"); }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path="📥️events/🦀️.rs"]
mod event_json;
pub use event_json::{EventJsonSource,PollJsonSource};
