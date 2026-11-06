//! 🔁️ Source-owned Poll transport over the real input, scalar and unique-output frontiers.
use semio_framework_actor::{OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobBudget};
use semio_framework_job::{OriginalCallerReturnPlace, OriginalNativeRetirement, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_job_native_decode::{OriginalNativeDecodeOutcome, OriginalNativeDecodePolicy};
use semio_framework_pack_json::{JsonReadLimits, JsonWriteSource};
use semio_framework_pack_json_output_receiving::{OriginalJsonOutputClose, OriginalJsonOutputOutcome, OriginalJsonOutputSource};
use semio_framework_value::{DslValue, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_native_owned::NativeReceive;
use semio_plugin_guest_contract::{operation_authority::owned_result::{self, OwnedReturn}, original_actor_context::{OriginalActorFixedCall, OriginalActorFixedKind, OriginalActorFixedReceipt}};
use semio_plugin_host_guest_runtime::{CompiledHandle, GuestInstance, GuestRuntimes, OriginalCoreValue, OwnedOriginalCallProgress, OwnedOriginalCallResult, OwnedOriginalCallStatus, OwnedSemioExport};
use semio_plugin_host_actor_failure::{ActorFailureRecipient,ActorFailureFetch};
use semio_plugin_host_post_turn_relay::{GuestRelayUnitFlow, GuestRelayUnitStatus, NativeInstanceRetirement};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Boot, ReleaseBoot, ReleaseBootstrap, CloseContext, ReleaseContext, Encode, Input, Poll, ReleaseHeader, Decode, ParserClose, ReleaseOutput, CloseOutput, CloseReceipt, Deliver, RetireCapture, CloseDecoder, Complete, Fault }

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
pub enum GuestPollFrozenKind { ActorFailure, Boot, CloseContext, Input, Poll, CloseOutput }

/// 🧳 The actual input, guest result, decoder and captured value remain in one Source owner.
pub struct GuestPoll<T: NativeReceive<DslValue> + RetireOwned,R:GuestPollFailureReceiving> {
    receiver:R,
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
    scalar_received:Option<OwnedOriginalCallResult>,
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
    core_quiescent:bool,
    origin_binding:Option<OriginalSourceBinding>,origin_address:Option<usize>,decode_policy:OriginalNativeDecodePolicy,decode_limits:JsonReadLimits,
    actor_failure:Option<semio_plugin_host_guest_runtime::GuestActorFailureCallReference>,
    failure_fetch:ActorFailureFetch,
    failure_plan:Option<semio_plugin_host_actor_failure::ActorFailurePlan>,
    failure_budget:Option<OriginalJobBudget>,
    failure_received:Option<OwnedOriginalCallResult>,
    failure_active:bool,
    failure_begin_place:OriginalCallerReturnPlace,
    failure_born:Option<bool>,
    failure_releasing:bool,
    failure_pointer:u32,
    failure_length:u32,
    failure_recipient_accepted:Option<semio_plugin_host_actor_failure::ActorFailureRecipientAccepted>,
    failure_ack_frame:Option<semio_plugin_host_guest_runtime::OwnedActorFailureAckFrame>,
    failure_ack_lower:Option<semio_plugin_host_guest_runtime::OwnedActorFailureAcknowledged>,
    failure_ack_host:Option<semio_plugin_host_guest_runtime::GuestActorFailureAcknowledged>,
    failure_acknowledged:Option<semio_plugin_host_actor_failure::ActorFailureAcknowledged>,
    bootstrap_identity:Option<semio_plugin_host_guest_runtime::GuestBootstrapHeaderIdentity>,
    bootstrap_returned:Option<semio_plugin_host_guest_runtime::GuestBootstrapHeaderDeallocateReturned>,
    bootstrap_status:Option<semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapStatus>,
    bootstrap_resume:Option<Phase>,
    journal_identity:Option<semio_plugin_host_guest_runtime::GuestFrozenEscrowIdentity>,
    header_returned:Option<semio_plugin_host_guest_runtime::GuestHeaderDeallocateReturned>,
}

fn scalar_failed(progress:&OwnedOriginalCallProgress)->bool{progress.result.is_err()||matches!(&progress.result,Ok(OwnedOriginalCallStatus::Cancelled|OwnedOriginalCallStatus::Deadline))}
fn invalid(message: &'static str) -> OriginalTurnFailure { ValueError::literal(ValueRefusalKind::InvariantViolated, message).into() }
fn require_core_wire_reserve(budget:OriginalJobBudget,header:usize)->Result<(),OriginalTurnFailure>{for(items,demand,work)in std::iter::once(semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve(header)?).chain((header==200).then(semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapReceipt::first_bootstrap_response_reserve)){for grant in[budget.input.grant,budget.retained_policy]{if grant.maximum_items<items||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Poll Core crossing lacks its mandatory escrow reserve").into())}}if budget.input.work.fuel<work||budget.fuel<work{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Poll Core crossing lacks its mandatory escrow work").into())}}Ok(())}
fn charge_metadata(cx:&mut StepContext<'_>,retiring:bool,bytes:usize)->Result<bool,OriginalTurnFailure>{
    let grant = cx.retained_grant();
    if (if retiring { cx.retirement_should_yield() } else { cx.should_yield() || cx.is_cancelled() }) || grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}
fn control_bytes()->usize{std::mem::size_of::<Phase>()+std::mem::size_of::<Option<OriginalSourceBinding>>()+std::mem::size_of::<Option<u64>>()+std::mem::size_of::<Option<usize>>()}
fn charge(cx:&mut StepContext<'_>,retiring:bool)->Result<bool,OriginalTurnFailure>{charge_metadata(cx,retiring,control_bytes())}
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

impl<T: NativeReceive<DslValue> + RetireOwned,R:GuestPollFailureReceiving> GuestPoll<T,R> {
    /// 🌱️ No guest allocation or decoder Source is admitted by construction.
    pub fn new(policy: OriginalNativeDecodePolicy, limits: JsonReadLimits,receiver:R) -> Self {
        Self { origin_binding:None,origin_address:None,decode_policy:policy,decode_limits:limits,receiver,binding: None, home_address: None, unit: GuestRelayUnitFlow::empty(), decoder: OriginalJsonOutputSource::new(policy, limits), captured: None, capture_close: None, outcome: None, settlement: None, return_place: OriginalCallerReturnPlace::empty(), scalar_progress: None, scalar_result: None,scalar_received:None, transport_failure: None, fixed: None, close_kind: None, budget: None, input_budget: None, raw_word: None, input_progress_pending: false, awaiting_input_budget: false, prepared: None, pointer: 0, length: 0, phase: Phase::Boot, scalar_started: false, failed: false, resident: false, output_captured: false, capsule: None, frozen: false,core_quiescent:false,actor_failure:None,failure_fetch:ActorFailureFetch::empty(),failure_plan:None,failure_budget:None,failure_received:None,failure_active:false,failure_begin_place:OriginalCallerReturnPlace::empty(),failure_born:None,failure_releasing:false,failure_pointer:0,failure_length:0,failure_recipient_accepted:None,failure_ack_frame:None,failure_ack_lower:None,failure_ack_host:None,failure_acknowledged:None, bootstrap_identity:None,bootstrap_returned:None,bootstrap_status:None,bootstrap_resume:None,journal_identity:None,header_returned:None }
    }
    fn validate(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.origin_binding.is_some_and(|binding|!binding.matches(cx))||self.origin_address.is_some_and(|address|address!=self as*const Self as usize){return Err(invalid("Poll changed its original native producer across commands"))}
        if self.binding.is_some_and(|binding| !binding.matches(cx)) { return Err(invalid("Poll changed its original Source")); }
        if self.home_address.is_some_and(|address| address != self as *const Self as usize) { return Err(invalid("Poll moved after binding its Source-owned frame")); }
        Ok(())
    }
    /// 🧳️ Borrows the actual retained consumer only through this Source-validated producer.
    pub fn receiver_state_mut(&mut self,cx:&StepContext<'_>)->Result<&mut R,OriginalTurnFailure>{self.validate(cx)?;Ok(&mut self.receiver)}
    pub fn requires_execution(&self, runtime: &GuestRuntimes, instance: &GuestInstance) -> Result<bool, OriginalTurnFailure> {
        if self.scalar_received.is_some(){return Ok(false)}
        if self.failure_active{return runtime.original_scalar_requires_execution(instance)}match self.phase { Phase::Input => self.unit.requires_execution(runtime, instance), Phase::Boot | Phase::ReleaseBoot | Phase::ReleaseBootstrap | Phase::CloseContext | Phase::ReleaseContext | Phase::Poll | Phase::ReleaseHeader | Phase::ReleaseOutput | Phase::CloseOutput | Phase::CloseReceipt if self.scalar_started => runtime.original_scalar_requires_execution(instance), _ => Ok(false) }
    }
    /// 🧭️ Describes the purpose of the in-flight guest crossing; it never issues or renews a loan.
    pub fn guest_purpose(&self) -> Option<GuestPollGuestPurpose> {
        if self.failure_active{return Some(GuestPollGuestPurpose::Infrastructure)}
        match self.phase { Phase::Boot | Phase::CloseContext | Phase::Input | Phase::CloseOutput | Phase::CloseReceipt => Some(GuestPollGuestPurpose::Infrastructure), Phase::Poll => Some(if self.output_captured { GuestPollGuestPurpose::Infrastructure } else { GuestPollGuestPurpose::Semantic }), _ => None }
    }
    /// 🧭️ Reveals only the current actual frozen frontier under its Source grant.
    pub fn frozen_kind_step(&self,cx:&mut StepContext<'_>)->Result<Option<Option<GuestPollFrozenKind>>,OriginalTurnFailure>{self.validate(cx)?;let retirement=cx.original_retirement_active();if !charge(cx,retirement)?{return Ok(None)}if self.failure_active{return Ok(Some((self.frozen&&self.scalar_progress.is_none()).then_some(GuestPollFrozenKind::ActorFailure)))}Ok(Some(if self.frozen&&!self.input_progress_pending&&self.scalar_progress.is_none(){Some(match self.phase{Phase::CloseContext=>GuestPollFrozenKind::CloseContext,Phase::Boot=>GuestPollFrozenKind::Boot,Phase::Input=>GuestPollFrozenKind::Input,Phase::Poll=>GuestPollFrozenKind::Poll,Phase::CloseOutput=>GuestPollFrozenKind::CloseOutput,_=>return Err(invalid("Poll froze outside a receipt frontier"))})}else{None}))}
    /// 🌱️ Only these phase boundaries admit a new real guest subloan.
    pub fn next_guest_subloan_step(&self, cx: &mut StepContext<'_>) -> Result<Option<Option<GuestPollGuestLoan>>, OriginalTurnFailure> {
        self.validate(cx)?;
        let retirement = cx.original_retirement_active();
        if !charge(cx, retirement)? { return Ok(None); }
        if self.failure_active{self.failure_fetch.status(cx)?;return Ok(Some(if !self.frozen&&self.scalar_progress.is_none()&&self.failure_received.is_none()&&self.failure_budget.is_none()&&!self.scalar_started&&self.failure_born.is_none()&&self.failure_begin_place.terminal_is_empty()&&self.failure_plan.as_ref().is_some_and(|plan|plan.scoped){Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:184})}else{None}))}
        if self.journal_identity.is_some() || self.frozen || self.return_place.is_admitted() || self.outcome.is_some() || self.settlement.is_some() || self.fixed.is_some() || self.raw_word.is_some() || self.scalar_progress.is_some() || self.scalar_result.is_some() || self.transport_failure.is_some() || self.input_progress_pending { return Ok(Some(None)); }
        Ok(Some(match self.phase { Phase::CloseContext if !self.scalar_started=>Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:200}),Phase::Boot if !self.scalar_started=>Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:200}),Phase::Input if self.unit.needs_admission_budget() && self.input_budget.is_none() || self.awaiting_input_budget => Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:144}), Phase::Poll if !self.scalar_started => Some(GuestPollGuestLoan{purpose:if self.output_captured { GuestPollGuestPurpose::Infrastructure } else { GuestPollGuestPurpose::Semantic },header_bytes:152}), Phase::CloseOutput if !self.scalar_started => Some(GuestPollGuestLoan{purpose:GuestPollGuestPurpose::Infrastructure,header_bytes:144}), _ => None }))
    }
    /// 🎟️ Settles controller and cleanup metadata before the caller issues the same turn's guest loan and VM credit.
    pub fn prepare_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.failure_active{return self.prepare_failure_step(runtime,instance,cx)}if self.phase == Phase::Complete && self.binding.is_none() { return Ok(true); }
        let epoch = cx.original_source_epoch().ok_or_else(|| invalid("Poll preparation requires its actual epoch"))?;
        if self.prepared == Some(epoch) { return Ok(true); }
        let retirement = cx.original_retirement_active();
        let bytes=control_bytes()+if self.origin_binding.is_none(){std::mem::size_of_val(&self.origin_binding)+std::mem::size_of_val(&self.origin_address)}else{0};if !charge_metadata(cx,retirement,bytes)?{return Ok(false)}
        self.origin_binding.get_or_insert(OriginalSourceBinding::borrow(cx));let address=self as*const Self as usize;self.origin_address.get_or_insert(address);
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
        if self.phase == Phase::Input&&!self.failure_active { self.unit.execution_receipt() } else { self.scalar_progress.as_ref().and_then(|progress| progress.execution) }
    }
    pub fn scalar_progress(&self) -> Option<&OwnedOriginalCallProgress> { self.scalar_progress.as_ref() }
    pub fn scalar_result(&self) -> Option<&OwnedOriginalCallResult> { self.scalar_result.as_ref() }
    /// ⚠️ Borrows only the actual failed export reference captured under its native producer.
    pub fn actor_failure_reference(&self,cx:&StepContext<'_>)->Result<Option<&semio_plugin_host_guest_runtime::GuestActorFailureCallReference>,OriginalTurnFailure>{self.validate(cx)?;Ok(self.actor_failure.as_ref())}
    fn needs_actor_failure_reference(&self)->bool{self.actor_failure.is_none()&&self.fixed.as_ref().is_some_and(|call|matches!(call.kind,OriginalActorFixedKind::Failure{..})||call.kind.input_disposition().is_some_and(|v|v.2))}
    pub fn transport_failure(&self) -> Option<&OriginalTurnFailure> { self.transport_failure.as_ref() }
    /// 📤️ The original recipient takes the actual transport failure before instance retirement.
    pub fn take_transport_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OriginalTurnFailure>,&mut R, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        if self.transport_failure.is_none() { return Ok(false); }
        if !charge(cx, true)? { return Ok(false); }
        receive(&mut self.transport_failure,&mut self.receiver, cx)?;
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
        if scalar_failed(progress) { return Err(invalid("Poll retains its original scalar failure")); }
        if self.failure_active&&self.return_place.is_admitted()&&!self.return_place.settle_return(cx)?{return Ok(false)}
        if !charge(cx, true)? { return Ok(false); }
        self.scalar_progress.take();
        Ok(true)
    }
    /// 📤️ Transfers an actual scalar failure only to an explicit original recipient.
    pub fn take_scalar_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OwnedOriginalCallProgress>,&mut R, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.scalar_progress.as_ref().is_none_or(|progress|!scalar_failed(progress)) { return Ok(false); }
        if self.failure_active&&self.return_place.is_admitted(){self.return_place.settle_return(cx)?;return Ok(false)}
        if !charge(cx, true)? { return Ok(false); }
        receive(&mut self.scalar_progress,&mut self.receiver, cx)?;
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
    pub fn take_input_failure_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&mut Option<OriginalTurnFailure>,&mut R, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> { self.validate(cx)?;let receiver=&mut self.receiver;self.unit.take_failure_step(cx,|failure,cx|receive(failure,receiver,cx)) }
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
    pub fn deliver_step(&mut self, cx: &mut StepContext<'_>, receive: impl FnOnce(&T,&mut R, &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.phase != Phase::Deliver { return Err(invalid("Poll output acknowledgement is out of phase")); }
        if self.transport_failure.is_some(){return Err(invalid("Poll retains its unreceived receiver failure"))}
        if cx.is_cancelled()&&!cx.original_retirement_active() { return Ok(false); }
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(false); }
        let address=(&mut self.transport_failure as *mut _) as usize;let bytes=std::mem::size_of_val(&self.transport_failure);
        if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(false); }
        if !charge_capture::<T>(cx)? { return Ok(false); }
        let result = receive(self.captured.as_ref().ok_or_else(|| invalid("Poll lost its captured output"))?,&mut self.receiver, cx);
        if matches!(result,Ok(false)){return Ok(false)}
        if let Err(error) = result { self.transport_failure = Some(error); self.return_place.record_sized_return(address,bytes)?;return Ok(false); }
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

    /// 🧹️ Closing uses only already-owned work and explicit Infrastructure authority.
    pub fn advance_retirement_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,budget:Option<OriginalJobBudget>,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<GuestPollStatus,OriginalTurnFailure>{if !cx.original_retirement_active(){return Err(invalid("Poll retirement requires its original cleanup context"))}self.advance_shared(runtime,instance,None,budget,execution,cx)}
    pub fn advance_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,input:&dyn JsonWriteSource,budget:Option<OriginalJobBudget>,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<GuestPollStatus,OriginalTurnFailure>{self.advance_shared(runtime,instance,Some(input),budget,execution,cx)}
    fn request_core_retirement_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.core_quiescent{return Ok(true)}if !self.terminal_is_empty(){return Ok(false)}if !charge(cx,true)?{return Ok(false)}self.binding=Some(OriginalSourceBinding::borrow(cx));self.home_address=Some(self as*const Self as usize);self.phase=Phase::CloseContext;Ok(true)}
    /// ▶️ `input` is the actual borrowed tagged Poll JSON source, never a projected owned DTO.
    fn advance_shared(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, input: Option<&dyn JsonWriteSource>, budget: Option<OriginalJobBudget>, execution: Option<OriginalGuestExecutionBudget>, cx: &mut StepContext<'_>) -> Result<GuestPollStatus, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.return_place.is_admitted() { self.return_place.settle_return(cx)?; return Ok(GuestPollStatus::Pending); }
        if self.input_progress_pending || self.scalar_progress.is_some() || self.scalar_result.is_some() || self.fixed.is_some() || self.raw_word.is_some() || self.transport_failure.is_some() { return Ok(GuestPollStatus::Pending); }
        if self.frozen { return Ok(GuestPollStatus::Frozen); }
        if self.awaiting_input_budget { return Ok(GuestPollStatus::AwaitBudget); }
        if self.outcome.is_some() || self.settlement.is_some() { return Ok(GuestPollStatus::Pending); }
        if self.phase == Phase::Complete { if self.binding.is_some() { if !charge(cx,true)? { return Ok(GuestPollStatus::Pending); } self.binding=None;self.home_address=None;self.prepared=None; } return Ok(GuestPollStatus::Complete); }
        if self.phase == Phase::Fault { if input.is_none(){if self.scalar_started{if runtime.close_original_scalar_step(instance,cx)?{self.scalar_started=false;}}else if !self.unit.terminal_is_empty(){self.unit.close_step(runtime,instance,cx)?;}else{self.close_payload_step(cx)?;}return Ok(GuestPollStatus::Pending)}return Ok(GuestPollStatus::Refused); }
        if self.phase == Phase::Deliver { return Ok(GuestPollStatus::Output); }
        if self.prepared.is_none() || self.prepared != cx.original_source_epoch() { return Err(invalid("Poll controller metadata must settle before guest authorization")); }
        if input.is_none()&&((matches!(self.phase,Phase::Encode|Phase::Boot|Phase::Poll)&&!self.scalar_started)||(self.phase==Phase::Input&&self.unit.needs_admission_budget())){self.prepared=None;if !self.unit.terminal_is_empty(){self.unit.close_step(runtime,instance,cx)?;}else{self.close_payload_step(cx)?;}return Ok(GuestPollStatus::Pending)}
        if self.phase == Phase::Input && self.unit.needs_admission_budget() { require_core_wire_reserve(budget.ok_or_else(|| invalid("Poll input requires its issued infrastructure loan"))?,144)?; }
        if matches!(self.phase,Phase::Boot|Phase::CloseContext|Phase::Poll|Phase::CloseOutput) && !self.scalar_started { require_core_wire_reserve(budget.ok_or_else(|| invalid("Poll export requires its issued guest subloan"))?,if self.phase==Phase::Poll{152}else if matches!(self.phase,Phase::Boot|Phase::CloseContext){200}else{144})?; }
        self.prepared = None;

        if self.phase == Phase::Decode && cx.is_cancelled() && cx.original_retirement_active() { self.failed = true; self.phase = Phase::ParserClose; }
        match self.phase {
            Phase::Encode => { if execution.is_some() { return Err(invalid("Poll encoding cannot consume VM execution")); } if self.unit.encode_step(input.ok_or_else(||invalid("Poll encoding requires its actual immutable input"))?, cx)? { self.phase = Phase::Input; } }
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
            Phase::Boot | Phase::ReleaseBoot | Phase::ReleaseBootstrap | Phase::CloseContext | Phase::ReleaseContext | Phase::Poll | Phase::ReleaseHeader | Phase::ReleaseOutput | Phase::CloseOutput | Phase::CloseReceipt => self.scalar_step(runtime, instance, budget, execution, cx)?,
            Phase::Decode => {
                if execution.is_some() { return Err(invalid("Poll receiving cannot consume VM execution")); }
                let address = (&mut self.outcome as *mut _) as usize;
                let bytes = std::mem::size_of_val(&self.outcome);
                if !self.return_place.admit_sized(cx,address,bytes)? { return Ok(GuestPollStatus::Pending); }
                let mut outcome = None;
                if self.decoder.ready() { outcome = self.decoder.take_step(&mut self.captured, cx)?; }
                else { runtime.with_original_scalar_bytes(instance, self.pointer, self.length as usize, cx, |bytes, cx| { outcome = self.decoder.receive_step(bytes, cx)?; Ok(()) })?; }
                if let Some(raw) = outcome { if raw.value.is_err() || raw.receiving_refusal.is_some() { self.failed = true; self.phase = Phase::ParserClose; } self.outcome = Some(ControlledRetirement::retain(raw)); self.return_place.record_sized_return(address,bytes)?; }
                if self.captured.is_some() { self.phase = Phase::ReleaseOutput; }
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

    /// 🧾️ Only the actual committed History view authorizes this exact frozen receipt continuation.
    pub fn accept_journaled_step(&mut self,view:&semio_plugin_host_wake::GuestJournaledReceipt<'_>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if !self.frozen{return Err(invalid("Poll receipt is not frozen"))}let call=view.actor_call()?;let expected=if self.phase==Phase::Input{self.input_budget}else{self.budget}.ok_or_else(||invalid("Poll receipt lost its issued guest scope"))?;if call.input!=expected.input{return Err(invalid("Poll receipt changed its issued guest input"))}if let Some(identity)=self.journal_identity{if identity!=view.identity()||self.fixed!=Some(call){return Err(invalid("Poll journaled receipt changed while held"))}return Ok(true)}
        let valid=matches!((self.phase,call.kind),(Phase::Boot,OriginalActorFixedKind::Admission{..})|(Phase::CloseContext,OriginalActorFixedKind::Close{..})|(Phase::CloseContext,OriginalActorFixedKind::QuiescentForCoreRetirement)|(Phase::Input,OriginalActorFixedKind::InputAdmission{..})|(Phase::Input,OriginalActorFixedKind::InputFailure{..})|(Phase::Poll,OriginalActorFixedKind::TurnOutput{..})|(Phase::CloseOutput,OriginalActorFixedKind::Close{..})|(_,OriginalActorFixedKind::Failure{..}));if !valid{return Err(invalid("Poll fixed receipt has another actual frontier"))}
        let bytes=std::mem::size_of_val(&self.fixed)+std::mem::size_of_val(&self.journal_identity)+std::mem::size_of_val(&self.close_kind)+std::mem::size_of_val(&self.capsule)+2*std::mem::size_of::<bool>();if !charge_frame(cx,bytes)?{return Ok(false)}
        match call.kind{OriginalActorFixedKind::TurnOutput{ready,captured,pointer,length,..}=>{if self.output_captured&&!captured{return Err(invalid("Poll captured flag regressed before CloseOutput"))}self.output_captured=captured;if ready{self.capsule=Some((pointer,length));self.resident=true;}},OriginalActorFixedKind::QuiescentForCoreRetirement|OriginalActorFixedKind::Admission{..}|OriginalActorFixedKind::Close{..}=>self.close_kind=Some(call.kind),_=>{}}self.fixed=Some(call);self.journal_identity=Some(view.identity());Ok(true)
    }
    /// ▶️ The frozen bytes stay immutable until the caller has observed their complete committed receipt.
    pub fn resume_journaled_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if !self.frozen||self.journal_identity.is_none(){return Err(invalid("Poll continuation lacks its actual journaled header"))}if self.fixed.is_some()||self.input_progress_pending||self.scalar_progress.is_some(){return Ok(false)}
        if self.phase==Phase::Input{if self.unit.receive_output_step(runtime,instance,cx,|bytes,_|{let(call,_)=OriginalActorFixedReceipt::read(bytes)?;if call.kind.input_disposition().is_none(){return Err(invalid("Poll input receipt changed before acknowledgment"))}Ok(true)})?{self.frozen=false;return Ok(true)}return Ok(false)}
        if !matches!(self.phase,Phase::Boot|Phase::CloseContext|Phase::Poll|Phase::CloseOutput){return Err(invalid("Poll journal continuation has no scalar frontier"))}if self.scalar_received.is_none(){runtime.take_original_scalar_into_step(instance,&mut self.scalar_received,cx)?;return Ok(false)}
        let result=*self.scalar_received.as_ref().unwrap();let word=match result.value{Some(OriginalCoreValue::I64(word))=>word as u64,_=>return Err(invalid("Poll fixed receipt returned another scalar type"))};let(pointer,length)=match owned_result::decode(word)?{OwnedReturn::Bytes{pointer,length}=>(pointer,length),OwnedReturn::Refusal(_)=>return Err(invalid("journaled Poll receipt changed into an unframed refusal"))};let bytes=std::mem::size_of_val(&self.scalar_received)+std::mem::size_of_val(&self.scalar_result)+std::mem::size_of_val(&self.raw_word)+2*std::mem::size_of::<u32>()+std::mem::size_of::<Phase>()+2;if !charge_frame(cx,bytes)?{return Ok(false)}self.scalar_result=self.scalar_received.take();self.raw_word=Some(word);self.pointer=pointer;self.length=length;self.phase=match self.phase{Phase::CloseContext=>Phase::ReleaseContext,Phase::Boot=>Phase::ReleaseBoot,Phase::Poll=>Phase::ReleaseHeader,_=>Phase::CloseReceipt};self.scalar_started=false;self.frozen=false;Ok(true)
    }
    fn accept_bootstrap_step(&mut self,call:&semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapCall,identity:semio_plugin_host_guest_runtime::GuestBootstrapHeaderIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if let Some(existing)=self.bootstrap_identity{if existing!=identity||self.bootstrap_status!=Some(call.status){return Err(invalid("Poll bootstrap changed its settled actual target"))}return Ok(true)}if !self.frozen||!matches!(self.phase,Phase::Boot|Phase::CloseContext)||self.budget!=Some(call.budget)||self.bootstrap_status.is_some()||self.bootstrap_returned.is_some(){return Err(invalid("Poll bootstrap lacks its exact frozen original admission"))}let bytes=std::mem::size_of_val(&self.bootstrap_identity)+std::mem::size_of_val(&self.bootstrap_status)+std::mem::size_of_val(&self.bootstrap_resume);if !charge_frame(cx,bytes)?{return Ok(false)}self.bootstrap_identity=Some(identity);self.bootstrap_status=Some(call.status);self.bootstrap_resume=Some(self.phase);Ok(true)
    }
    fn resume_bootstrap_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if !self.frozen||self.bootstrap_identity.is_none()||self.bootstrap_status.is_none()||!matches!(self.phase,Phase::Boot|Phase::CloseContext){return Err(invalid("Poll bootstrap continuation lacks its actual settled header"))}if self.scalar_progress.is_some(){return Ok(false)}if self.scalar_received.is_none(){runtime.take_original_scalar_into_step(instance,&mut self.scalar_received,cx)?;return Ok(false)}let result=*self.scalar_received.as_ref().unwrap();let word=match result.value{Some(OriginalCoreValue::I64(word))=>word as u64,_=>return Err(invalid("Poll bootstrap returned another scalar type"))};let(pointer,length)=match owned_result::decode(word)?{OwnedReturn::Bytes{pointer,length}if length==304=>(pointer,length),_=>return Err(invalid("Poll bootstrap changed its original SAB1 extent"))};let bytes=std::mem::size_of_val(&self.scalar_received)+std::mem::size_of_val(&self.scalar_result)+std::mem::size_of_val(&self.raw_word)+2*std::mem::size_of::<u32>()+std::mem::size_of::<Phase>()+2;if !charge_frame(cx,bytes)?{return Ok(false)}self.scalar_result=self.scalar_received.take();self.raw_word=Some(word);self.pointer=pointer;self.length=length;self.phase=Phase::ReleaseBootstrap;self.scalar_started=false;self.frozen=false;Ok(true)
    }
    fn finish_bootstrap_header_return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.bootstrap_returned.is_some()||self.bootstrap_identity.is_none(){return Err(invalid("Poll bootstrap handback has no actual settled return"))}if !charge_frame(cx,std::mem::size_of_val(&self.bootstrap_identity))?{return Ok(false)}self.bootstrap_identity=None;Ok(true)}
    pub fn header_identity(&self)->Option<semio_plugin_host_guest_runtime::GuestFrozenEscrowIdentity>{self.journal_identity}
    fn finish_header_return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.header_returned.is_some(){return Ok(false)}if self.journal_identity.is_none(){return Ok(true)}if !charge_frame(cx,std::mem::size_of_val(&self.journal_identity))?{return Ok(false)}self.journal_identity=None;Ok(true)}
    /// 🔄️ Renews the exact retained input borrow only after its pending admission receipt.
    pub fn repeat_input_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, budget: OriginalJobBudget, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> { self.validate(cx)?; if self.input_progress_pending { return Ok(false); } if !self.awaiting_input_budget { return Err(invalid("Poll input is not awaiting a renewed budget")); } require_core_wire_reserve(budget,144)?; if self.unit.repeat_step(runtime, instance, budget, cx)? { self.input_budget = Some(budget); self.awaiting_input_budget = false; return Ok(true); } Ok(false) }
    fn scalar_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,budget:Option<OriginalJobBudget>,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if self.scalar_received.is_none(){
            if !self.scalar_started{
                if execution.is_some(){return Err(invalid("Poll scalar admission cannot consume VM execution"))}let mut args=[OriginalCoreValue::I64(0);OriginalJobBudget::SCALAR_ARGUMENT_COUNT];
                let export=match self.phase{
                    Phase::Boot|Phase::CloseContext|Phase::Poll|Phase::CloseOutput=>{let issued=budget.ok_or_else(||invalid("Poll export requires its issued guest subloan"))?;require_core_wire_reserve(issued,if self.phase==Phase::Poll{152}else if matches!(self.phase,Phase::Boot|Phase::CloseContext){200}else{144})?;for(slot,value)in args.iter_mut().zip(issued.scalar_arguments()){*slot=OriginalCoreValue::I64(value as i64)}if self.phase==Phase::CloseContext{OwnedSemioExport::CloseActorContext}else if self.phase==Phase::Boot{OwnedSemioExport::AdmitActorContext}else if self.phase==Phase::Poll{OwnedSemioExport::Poll}else{OwnedSemioExport::CloseActorOutput}},
                    Phase::ReleaseBoot|Phase::ReleaseBootstrap|Phase::ReleaseContext|Phase::ReleaseHeader|Phase::ReleaseOutput|Phase::CloseReceipt=>{args[0]=OriginalCoreValue::I32(self.pointer as i32);args[1]=OriginalCoreValue::I32(self.length as i32);OwnedSemioExport::Deallocate},
                    _=>return Err(invalid("Poll scalar export is out of phase"))
                };let count=if export==OwnedSemioExport::Deallocate{2}else{OriginalJobBudget::SCALAR_ARGUMENT_COUNT};if runtime.begin_original_scalar_step(instance,export,&args[..count],cx)?{self.scalar_started=true;if export!=OwnedSemioExport::Deallocate{self.budget=budget}}return Ok(())
            }
            let progress=runtime.advance_original_scalar_step(instance,execution,cx)?;let ready=matches!(&progress.result,Ok(OwnedOriginalCallStatus::Ready));let refused=progress.result.is_err()||matches!(&progress.result,Ok(OwnedOriginalCallStatus::Cancelled|OwnedOriginalCallStatus::Deadline));self.scalar_progress=Some(progress);if refused{self.failed=true;self.phase=Phase::Fault;return Ok(())}if !ready{return Ok(())}if matches!(self.phase,Phase::Boot|Phase::CloseContext|Phase::Poll|Phase::CloseOutput){self.frozen=true;return Ok(())}if self.journal_identity.is_some()||self.bootstrap_identity.is_some(){return Ok(())}runtime.take_original_scalar_into_step(instance,&mut self.scalar_received,cx)?;return Ok(())
        }
        let result=*self.scalar_received.as_ref().unwrap();if result.value.is_some(){return Err(invalid("Poll deallocator returned a value"))}
        let mut bytes=std::mem::size_of_val(&self.scalar_received)+std::mem::size_of_val(&self.scalar_result)+std::mem::size_of::<bool>()+std::mem::size_of::<Phase>();
        bytes+=match self.phase{
            Phase::ReleaseBootstrap=>std::mem::size_of_val(&self.bootstrap_resume)+if matches!(self.bootstrap_status,Some(semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapStatus::Failed(_))){std::mem::size_of::<bool>()}else{std::mem::size_of_val(&self.bootstrap_status)+std::mem::size_of_val(&self.budget)},
            Phase::ReleaseContext=>std::mem::size_of_val(&self.close_kind)+match self.close_kind{Some(OriginalActorFixedKind::QuiescentForCoreRetirement)=>std::mem::size_of::<bool>()+std::mem::size_of_val(&self.budget),Some(OriginalActorFixedKind::Close{complete:false,refusal:None,..})=>0,_=>std::mem::size_of::<bool>()},
            Phase::ReleaseBoot=>std::mem::size_of_val(&self.close_kind)+if matches!(self.close_kind,Some(OriginalActorFixedKind::Admission{refusal:None,..})){0}else{std::mem::size_of::<bool>()},
            Phase::ReleaseHeader=>std::mem::size_of_val(&self.capsule)+if !self.failed&&self.capsule.is_some(){2*std::mem::size_of::<u32>()}else{0},
            Phase::ReleaseOutput=>0,
            Phase::CloseReceipt=>std::mem::size_of_val(&self.close_kind)+if matches!(self.close_kind,Some(OriginalActorFixedKind::Close{complete:true,blocked:false,refusal:None}))&&self.failed&&self.decoder.terminal_is_empty(){std::mem::size_of_val(&self.budget)}else if self.close_kind.is_some_and(|kind|!matches!(kind,OriginalActorFixedKind::Close{refusal:None,..})){std::mem::size_of::<bool>()}else{0},
            _=>return Err(invalid("Poll retained scalar result is out of phase"))
        };if !charge_frame(cx,bytes)?{return Ok(())}self.scalar_result=self.scalar_received.take();self.scalar_started=false;
        match self.phase {
            Phase::Boot | Phase::CloseContext | Phase::Poll | Phase::CloseOutput => return Err(invalid("frozen receipt requires Source escrow settlement")),
            Phase::ReleaseBootstrap=>{if result.value.is_some(){return Err(invalid("Poll bootstrap deallocator returned a value"))}let resume=self.bootstrap_resume.take().ok_or_else(||invalid("bootstrap deallocator lost its original frontier"))?;match self.bootstrap_status{Some(semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapStatus::Failed(_))=>{self.failed=true;self.phase=Phase::Fault;},Some(_)=>{self.bootstrap_status=None;self.phase=resume;self.budget=None;},None=>return Err(invalid("bootstrap deallocator lost its settled status"))}}
            Phase::ReleaseContext=>{if result.value.is_some(){return Err(invalid("Poll context deallocator returned a value"))}self.phase=match self.close_kind.take(){Some(OriginalActorFixedKind::QuiescentForCoreRetirement)=>{self.core_quiescent=true;self.budget=None;Phase::Complete},Some(OriginalActorFixedKind::Close{complete:false,refusal:None,..})=>Phase::CloseContext,_=>{self.failed=true;Phase::Fault}};}
            Phase::ReleaseBoot=>{if result.value.is_some(){return Err(invalid("Poll boot deallocator returned a value"))}self.phase=match self.close_kind.take(){Some(OriginalActorFixedKind::Admission{ready:true,refusal:None})=>Phase::Encode,Some(OriginalActorFixedKind::Admission{ready:false,refusal:None})=>Phase::Boot,_=>{self.failed=true;Phase::Fault}};}
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
    /// 🔁️ Begins another fully settled command on the same native producer and resident guest context.
    pub fn begin_next_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if !self.terminal_is_empty(){return Ok(false)}if self.origin_binding.is_none()||self.origin_address.is_none()||self.failed||self.core_quiescent{return Err(invalid("next Poll command requires its original successful settled round"))}if cx.original_retirement_active()||cx.is_cancelled()||cx.should_yield(){return Ok(false)}
        let bytes=std::mem::size_of_val(&self.unit)+std::mem::size_of_val(&self.decoder)+std::mem::size_of_val(&self.prepared)+std::mem::size_of_val(&self.phase)+3*std::mem::size_of::<bool>()+2*std::mem::size_of::<u32>();if !charge_frame(cx,bytes)?{return Ok(false)}
        self.unit=GuestRelayUnitFlow::empty();self.decoder=OriginalJsonOutputSource::new(self.decode_policy,self.decode_limits);self.prepared=None;self.phase=Phase::Encode;self.resident=false;self.output_captured=false;self.awaiting_input_budget=false;self.pointer=0;self.length=0;Ok(true)
    }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Complete && self.binding.is_none() && self.home_address.is_none() && self.return_place.terminal_is_empty() && self.budget.is_none() && self.input_budget.is_none() && self.unit.terminal_is_empty() && self.decoder.terminal_is_empty() && self.captured.is_none() && self.capture_close.is_none() && self.outcome.is_none() && self.settlement.is_none() && self.scalar_progress.is_none() && self.scalar_result.is_none() && self.scalar_received.is_none() && self.transport_failure.is_none() && self.fixed.is_none() && self.raw_word.is_none() && self.prepared.is_none() && self.capsule.is_none() && !self.input_progress_pending && !self.scalar_started && !self.frozen && self.journal_identity.is_none()&&self.header_returned.is_none()&&self.failure_terminal_is_empty()&&self.bootstrap_identity.is_none()&&self.bootstrap_returned.is_none()&&self.bootstrap_status.is_none()&&self.bootstrap_resume.is_none() }
    fn close_payload_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if matches!(self.bootstrap_status,Some(semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapStatus::Failed(_))){return Err(invalid("Poll retains the actual complete pre-Source bootstrap cause"))}
        if !self.failure_terminal_is_empty(){return Err(invalid("Poll retains an unreceived actual actor cause"))}
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
impl<T: NativeReceive<DslValue> + RetireOwned+Send+'static,R:GuestPollFailureReceiving> NativeInstanceRetirement for GuestPoll<T,R> {
    fn close_step(&mut self, runtime: &GuestRuntimes, instance: &mut GuestInstance, _: &CompiledHandle, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.terminal_is_empty() { return self.receiver.close_on_source(cx); }
        if self.frozen { return Err(invalid("Poll must settle its frozen receipt before retirement")); }
        if self.input_progress_pending || self.unit.failure().is_some() { return Err(invalid("Poll retains its unreceived input result")); }
        if self.scalar_started { if !runtime.close_original_scalar_step(instance, cx)? { return Ok(false); } self.scalar_started = false; return Ok(false); }
        if !self.unit.terminal_is_empty() { if !self.unit.close_step(runtime, instance, cx)? { return Ok(false); } return Ok(false); }
        if !self.close_payload_step(cx)?{return Ok(false)}self.receiver.close_on_source(cx)
    }
    fn close_unopened(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.terminal_is_empty(){return self.receiver.close_on_source(cx)}
        if self.frozen { return Err(invalid("Poll must settle its frozen receipt before retirement")); }
        if self.input_progress_pending || self.unit.failure().is_some() { return Err(invalid("Poll retains its unreceived input result")); }
        if !self.unit.terminal_is_empty() { if !self.unit.close_unopened(cx)? { return Ok(false); } return Ok(false); }
        if !self.close_payload_step(cx)?{return Ok(false)}self.receiver.close_on_source(cx)
    }
    fn terminal_is_empty(&self) -> bool { GuestPoll::terminal_is_empty(self)&&self.receiver.terminal_is_empty() }
}
impl<T: NativeReceive<DslValue> + RetireOwned,R:GuestPollFailureReceiving> Drop for GuestPoll<T,R> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty()&&self.receiver.terminal_is_empty(), "Poll requires complete original Source retirement"); }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path="📥️events/🦀️.rs"]
mod event_json;
pub use event_json::{EventJsonSource,PollJsonSource,PollRequestLimits};

#[path="🎛️session/🦀️.rs"]
mod original_session;
pub use original_session::{GuestPollSession,GuestPollSessionStatus,GuestPollSessionInput};

/// 🪶️ A genuine empty consumer for probes that retain no semantic output descendants.
pub trait GuestPollFailureReceiving:OriginalNativeRetirement{fn actor_failure_recipient(&mut self)->&mut ActorFailureRecipient;}
pub struct GuestPollNoReceiverState{failure:ActorFailureRecipient}
impl GuestPollNoReceiverState{pub const fn new()->Self{Self{failure:ActorFailureRecipient::empty()}}}
impl GuestPollFailureReceiving for GuestPollNoReceiverState{fn actor_failure_recipient(&mut self)->&mut ActorFailureRecipient{&mut self.failure}}
impl OriginalNativeRetirement for GuestPollNoReceiverState{fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.failure.close_on_source(cx)}fn terminal_is_empty(&self)->bool{self.failure.terminal_is_empty()}}

#[path="⚠️failure/🦀️.rs"]
mod actor_failure_flow;
