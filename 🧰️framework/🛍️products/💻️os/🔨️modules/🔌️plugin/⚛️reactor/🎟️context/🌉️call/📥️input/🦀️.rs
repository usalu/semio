//! 📥️ A guest input decoder retains its wire identity, typed owner and complete native receipts.
use semio_framework_job::{OriginalCallerReturnPlace,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_job_native_decode::{OriginalNativeDecodeOutcome,OriginalNativeDecodePolicy};
use semio_framework_pack_json::JsonReadLimits;
pub use semio_framework_pack_json_output_receiving::OriginalJsonOutputFailure;
use semio_framework_pack_json_output_receiving::{NativeReceivingFailure,OriginalJsonOutputClose,OriginalJsonOutputOutcome,OriginalJsonOutputSource};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use semio_plugin_abi::owned_abi::ActorInput;
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum InputAdmission { Pending, Ready, Failed, Closed }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalActorInputFailureIdentity{owner:usize,outcome:usize,settlement:usize,binding:OriginalSourceBinding}
pub struct OriginalActorInputFailureView<'a>{pub identity:OriginalActorInputFailureIdentity,pub outcome:Option<&'a OriginalJsonOutputOutcome>,pub receiving:Option<&'a NativeReceivingFailure>,pub settlement:Option<&'a OriginalNativeDecodeOutcome<RetainedCloneStep>>,pub outcome_refusal:Option<&'a OriginalTurnFailure>,pub settlement_refusal:Option<&'a OriginalTurnFailure>}

fn projection_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"input failure projection changed its retained branch")}
impl OriginalActorInputFailureView<'_>{
 fn field(&self,index:usize)->Result<&'static str,ValueError>{[self.outcome.and_then(|v|v.value.as_ref().err()).map(|_|"outcomeValue"),self.outcome_refusal.map(|_|"outcomeReceiving"),self.settlement.and_then(|v|v.value.as_ref().err()).map(|_|"settlementValue"),self.settlement_refusal.map(|_|"settlementReceiving")].into_iter().flatten().nth(index).ok_or_else(projection_invalid)}
}
impl semio_framework_job::OriginalDomainProjection for OriginalActorInputFailureView<'_>{
 fn projection_identity(&self)->usize{self.identity.owner}
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{
  let Some((&field,rest))=path.split_first()else{return Ok(semio_framework_job::OriginalDomainView::IntrinsicObject((0..4).filter(|i|self.field(*i).is_ok()).count()))};
  match self.field(field)?{
   "outcomeValue"=>match self.outcome.and_then(|v|v.value.as_ref().err()).ok_or_else(projection_invalid)?{OriginalJsonOutputFailure::Refused(error)=>semio_framework_job::project_original_value_error(error,rest),OriginalJsonOutputFailure::Stored(_)=>semio_framework_job::project_original_receiving_primary(self.receiving.ok_or_else(projection_invalid)?.primary(),rest)},
   "outcomeReceiving"=>semio_framework_job::project_original_turn_failure(self.outcome_refusal.ok_or_else(projection_invalid)?,rest),
   "settlementValue"=>semio_framework_job::project_original_value_error(self.settlement.and_then(|v|v.value.as_ref().err()).ok_or_else(projection_invalid)?,rest),
   _=>semio_framework_job::project_original_turn_failure(self.settlement_refusal.ok_or_else(projection_invalid)?,rest)
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  let Some((&field,rest))=path.split_first()else{return self.field(index)};
  match self.field(field)?{
   "outcomeValue"=>match self.outcome.and_then(|v|v.value.as_ref().err()).ok_or_else(projection_invalid)?{OriginalJsonOutputFailure::Refused(error)=>semio_framework_job::key_original_value_error(error,rest,index),OriginalJsonOutputFailure::Stored(_)=>semio_framework_job::key_original_receiving_primary(self.receiving.ok_or_else(projection_invalid)?.primary(),rest,index)},
   "outcomeReceiving"=>semio_framework_job::key_original_turn_failure(self.outcome_refusal.ok_or_else(projection_invalid)?,rest,index),
   "settlementValue"=>semio_framework_job::key_original_value_error(self.settlement.and_then(|v|v.value.as_ref().err()).ok_or_else(projection_invalid)?,rest,index),
   _=>semio_framework_job::key_original_turn_failure(self.settlement_refusal.ok_or_else(projection_invalid)?,rest,index)
  }
 }
}

/// 🧳️ The caller owns this frame before the decoder borrows an unchanged wire allocation.
pub struct OriginalActorInputReceiving {
    binding:Option<OriginalSourceBinding>, pointer:usize, length:usize, capture:OriginalCallerReturnPlace,
    decoder:ManuallyDrop<OriginalJsonOutputSource<ActorInput>>, input:ManuallyDrop<Option<ActorInput>>,
    outcome:ManuallyDrop<Option<ControlledRetirement<OriginalJsonOutputOutcome>>>,
    settlement:ManuallyDrop<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>,
    retirement:ManuallyDrop<Option<ControlledRetirement<ActorInput>>>,
    event:ManuallyDrop<Option<semio_framework::kernel::Event>>,
    job:ManuallyDrop<Option<semio_framework_actor::OriginalJobSource>>,
    job_retirement:ManuallyDrop<Option<ControlledRetirement<semio_framework_actor::OriginalJobSource>>>, job_id:Option<u64>,
    failed:bool, outcome_failed:bool, settlement_failed:bool, closing:bool, returning:bool, returned:bool,
}

/// ♻️ Drives one paid retirement step of a retained owner through the caller's own original effect.
pub fn retire_step<U:semio_framework_value::retirement::RetireOwned>(owner:&mut ControlledRetirement<U>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
    let grant=cx.retained_grant();let body=grant.maximum_copy_bytes;
    let Some((poll,_))=cx.with_original_effect(grant,|_,effect|owner.step(body,effect))?else{return Ok(())};
    let turn=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
    semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())
}
fn invalid()->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,"actor input changed its original Source or borrowed wire").into()}
fn charge(cx:&mut StepContext<'_>,retiring:bool,bytes:usize)->Result<bool,OriginalTurnFailure>{
    let grant=cx.retained_grant();
    if (if retiring{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
impl OriginalActorInputReceiving {
    /// 🌱️ Construction declares limits but admits no parser, source loan or byte owner.
    pub const fn new(policy:OriginalNativeDecodePolicy,limits:JsonReadLimits)->Self{Self{binding:None,pointer:0,length:0,capture:OriginalCallerReturnPlace::empty(),decoder:ManuallyDrop::new(OriginalJsonOutputSource::new(policy,limits)),input:ManuallyDrop::new(None),outcome:ManuallyDrop::new(None),settlement:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),event:ManuallyDrop::new(None),job:ManuallyDrop::new(None),job_retirement:ManuallyDrop::new(None),job_id:None,failed:false,outcome_failed:false,settlement_failed:false,closing:false,returning:false,returned:false}}
    /// 🎟️ The containing Source pays the frame before binding its exact wire and caller.
    pub(crate) fn new_bound(policy:OriginalNativeDecodePolicy,limits:JsonReadLimits,wire:&[u8],cx:&StepContext<'_>)->Self{let mut owner=Self::new(policy,limits);owner.binding=Some(OriginalSourceBinding::borrow(cx));owner.pointer=wire.as_ptr() as usize;owner.length=wire.len();owner}
    fn validate(&self,wire:Option<&[u8]>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invalid())}
        if self.returned{return if wire.is_none(){Ok(())}else{Err(invalid())}}
        let wire=wire.ok_or_else(invalid)?;
        if self.binding.is_some()&&(self.pointer!=wire.as_ptr() as usize||self.length!=wire.len()){return Err(invalid())}Ok(())
    }
    /// 👁️ The retained original failure is available for a funded publication before retirement.
    pub fn outcome(&self)->Option<&OriginalJsonOutputOutcome>{self.outcome.as_ref().and_then(ControlledRetirement::original)}
    /// 🔎️ Borrows every actual retained failure branch without moving an outcome or inventing a cancellation cause.
    pub fn with_failure_view_step(&self,expected:Option<&OriginalActorInputFailureIdentity>,cx:&mut StepContext<'_>,receive:impl for<'view> FnOnce(OriginalActorInputFailureView<'view>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let outcome=self.outcome().filter(|raw|raw.value.is_err()||raw.receiving_refusal.is_some());let settlement=self.settlement.as_ref().and_then(ControlledRetirement::original).filter(|raw|raw.value.is_err()||raw.receiving_refusal.is_some());
        if outcome.is_none()&&settlement.is_none(){return Ok(false)}let binding=self.binding.filter(|binding|binding.matches(cx)).ok_or_else(invalid)?;
        let identity=OriginalActorInputFailureIdentity{owner:self as*const Self as usize,outcome:outcome.map_or(0,|value|value as*const OriginalJsonOutputOutcome as usize),settlement:settlement.map_or(0,|value|value as*const OriginalNativeDecodeOutcome<RetainedCloneStep>as usize),binding};
        if expected.is_some_and(|expected|*expected!=identity){return Err(invalid())}if !charge(cx,true,std::mem::size_of::<OriginalActorInputFailureView<'_>>())?{return Ok(false)}receive(OriginalActorInputFailureView{identity,outcome,receiving:self.decoder.failure(),settlement,outcome_refusal:outcome.and_then(|raw|self.refusal(raw)),settlement_refusal:settlement.and_then(|raw|self.refusal(raw))},cx)?;Ok(true)
    }
    /// 🎫️ Resolves the outcome's receiving ticket through the decoder Source that issued it.
    fn refusal<T,E>(&self,raw:&OriginalNativeDecodeOutcome<T,E>)->Option<&OriginalTurnFailure>{raw.receiving_refusal.as_ref().and_then(|ticket|self.decoder.receiving_refusal(ticket))}
    /// 🧯️ A fixed refusal names only a kind present in the retained original decoder outcome.
    pub fn failure_kind(&self)->Option<ValueRefusalKind>{
        let settlement=self.settlement.as_ref().and_then(ControlledRetirement::original).and_then(|raw|self.refusal(raw).and_then(|failure|failure.primary_value().map(|error|error.kind)).or_else(||raw.value.as_ref().err().map(|error|error.kind)));
        settlement.or_else(||{let outcome=self.outcome()?;self.refusal(outcome).and_then(|failure|failure.primary_value().map(|error|error.kind).or_else(||failure.primary_receiving().and_then(|cause|cause.primary.as_ref().or(cause.quote_refusal.as_ref()).or(cause.allocation_refusal.as_ref()).map(|error|error.kind))))
            .or_else(||outcome.value.as_ref().err().and_then(|cause|match cause{OriginalJsonOutputFailure::Refused(error)=>Some(error.kind),OriginalJsonOutputFailure::Stored(_)=>self.decoder.failure().map(NativeReceivingFailure::primary).and_then(|cause|cause.primary.as_ref().or(cause.quote_refusal.as_ref()).or(cause.allocation_refusal.as_ref()).map(|error|error.kind))}))})
    }
    pub fn input(&self)->Option<&ActorInput>{if self.input_returned()&&!self.closing{self.input.as_ref()}else{None}}
    pub fn with_input_step<T>(&mut self,cx:&mut StepContext<'_>,use_input:impl FnOnce(&mut ActorInput,&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<Option<T>,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}if !charge(cx,false,std::mem::size_of::<&mut ActorInput>())?{return Ok(None)}
        Ok(Some(use_input(self.input.as_mut().ok_or_else(invalid)?,cx)?))
    }
    /// 📨️ Keeps the unconsumed event in the same Source owner across a pending turn.
    pub fn prepare_poll_event(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}
        let Some(ActorInput::Poll(poll))=self.input.as_ref()else{return Err(invalid())};
        if poll.events.len()>1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"poll input exceeds its retained single-event ingress").into())}
        if self.event.is_some()||poll.events.is_empty(){return Ok(true)}
        let bytes=std::mem::size_of::<Option<semio_framework::kernel::Event>>()+std::mem::size_of::<usize>();let progress=RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()};if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()<2||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return Ok(false)}cx.retained_progress().checked_add(progress)?;
        let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};*self.event=poll.events.pop();let posted=cx.consume_retained(progress);cx.consume_fuel(2);posted?;Ok(true)
    }
    pub fn with_poll_step<T>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_plugin_abi::owned_abi::PollInput,&mut Option<semio_framework::kernel::Event>,&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}
        let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};run(poll,&mut *self.event,cx)
    }
    pub fn poll_event_pending(&self)->bool{self.event.is_some()||matches!(self.input.as_ref(),Some(ActorInput::Poll(poll))if !poll.events.is_empty()||poll.command_page.is_some())}
    pub fn input_returned(&self)->bool{self.returned&&self.capture.terminal_is_empty()}
    /// 🪪️ A replacement wire cannot displace an earlier still-borrowed allocation.
    pub fn owns_wire(&self,wire:&[u8])->bool{self.binding.is_some()&&self.pointer==wire.as_ptr() as usize&&self.length==wire.len()}
    /// 🧹️ A frame can leave its original parent only after every real child has closed.
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.capture.terminal_is_empty()&&self.decoder.terminal_is_empty()&&self.input.is_none()&&self.outcome.is_none()&&self.settlement.is_none()&&self.retirement.is_none()&&self.event.is_none()&&self.job.is_none()&&self.job_retirement.is_none()}
    /// 💼️ The exact decoded job becomes one retryable slot only under its original Source.
    pub fn prepare_job(&mut self,job:u64,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}
        if let Some(existing)=self.job_id{return if existing==job{Ok(true)}else{Err(invalid())}}
        if !matches!(self.input.as_ref(),Some(ActorInput::Job(source)) if source.job==job){return Err(invalid())}
        let grant=cx.retained_grant();let bytes=std::mem::size_of::<Option<ActorInput>>()+std::mem::size_of::<Option<semio_framework_actor::OriginalJobSource>>();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
        let Some(ActorInput::Job(source))=self.input.take()else{unreachable!()};*self.job=Some(source);self.job_id=Some(job);Ok(true)
    }
    /// 🔒️ Only the already prepared matching job can lend its original source slot to Entry.
    pub fn job_slot(&mut self,job:u64)->Option<&mut Option<semio_framework_actor::OriginalJobSource>>{if !self.returned||self.closing||self.job_id!=Some(job){return None}Some(&mut *self.job)}
    pub fn job_prepared(&self,job:u64)->bool{self.returned&&!self.closing&&self.job_id==Some(job)}
    pub fn holds_job(&self,job:u64)->bool{matches!(self.input.as_ref(),Some(ActorInput::Job(source)) if source.job==job)||self.job_id==Some(job)&&self.job.is_some()}
    pub fn holds_job_in(&self,job:u64,entry_has_source:bool,cx:&StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(None,cx)?;Ok(self.holds_job(job)||self.job_prepared(job)&&entry_has_source)}
    pub fn matches_job_in(&self,job:u64,cx:&StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(None,cx)?;Ok(self.holds_job(job)||self.job_prepared(job))}
    /// 🛑️ Cancellation retains every typed child for separately funded retirement.
    pub fn begin_close(&mut self,wire:Option<&[u8]>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(wire,cx)?;
        if self.closing{return Ok(true)}let bytes=std::mem::size_of::<(bool,bool)>()+if self.binding.is_none(){std::mem::size_of::<(Option<OriginalSourceBinding>,usize,usize)>()}else{0};if !charge(cx,true,bytes)?{return Ok(false)}
        if self.binding.is_none(){let wire=wire.ok_or_else(invalid)?;self.binding=Some(OriginalSourceBinding::borrow(cx));self.pointer=wire.as_ptr() as usize;self.length=wire.len()}
        self.closing=true;self.returning=true;Ok(true)
    }
    /// 🧾️ The exact original decoder failure can move only into an explicitly supplied receiver.
    pub fn transfer_failure_step(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut Option<ControlledRetirement<OriginalJsonOutputOutcome>>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
        if self.binding.is_none_or(|binding|!binding.matches(cx)){return Err(invalid())}
        if !self.outcome_failed||self.outcome.is_none(){return Ok(false)}
        if !charge(cx,true,std::mem::size_of::<&mut Option<ControlledRetirement<OriginalJsonOutputOutcome>>>())?{return Ok(false)}receive(&mut *self.outcome,cx)?;if self.outcome.is_none(){self.outcome_failed=false;Ok(true)}else{Ok(false)}
    }
    /// 🧾️ A failed close receipt remains original until its caller explicitly receives it.
    pub fn transfer_settlement_failure_step(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
        if self.binding.is_none_or(|binding|!binding.matches(cx)){return Err(invalid())}if !self.settlement_failed||self.settlement.is_none(){return Ok(false)}
        if !charge(cx,true,std::mem::size_of::<&mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>())?{return Ok(false)}receive(&mut *self.settlement,cx)?;if self.settlement.is_none(){self.settlement_failed=false;Ok(true)}else{Ok(false)}
    }
    fn settlement_destination(&mut self)->(usize,usize){((&mut *self.settlement as *mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>) as usize,std::mem::size_of::<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>())}
    fn outcome_destination(&mut self)->(usize,usize){((&mut *self.outcome as *mut Option<ControlledRetirement<OriginalJsonOutputOutcome>>) as usize,std::mem::size_of::<Option<ControlledRetirement<OriginalJsonOutputOutcome>>>())}
    fn retain_settlement(&mut self,raw:OriginalNativeDecodeOutcome<RetainedCloneStep>)->Result<(),OriginalTurnFailure>{self.settlement_failed=raw.value.is_err()||raw.receiving_refusal.is_some();self.failed|=self.settlement_failed;*self.settlement=Some(ControlledRetirement::retain(raw));let(address,bytes)=self.settlement_destination();self.capture.record_sized_return(address,bytes)?;Ok(())}
    fn retire_outcome_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if let Some(settlement)=self.settlement.as_mut(){
            if self.settlement_failed{return Ok(false)}
            if !settlement.terminal_is_empty(){retire_step(settlement,cx)?;return Ok(false)}
            if !charge(cx,true,std::mem::size_of::<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>())?{return Ok(false)}self.settlement.take();return Ok(false)
        }
        if let Some(outcome)=self.outcome.as_mut(){
            if self.outcome_failed{return Ok(false)}
            if !outcome.terminal_is_empty(){let address=(&mut *self.settlement as *mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>) as usize;let bytes=std::mem::size_of::<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>();let admitted=if self.closing{cx.with_original_retirement(|cx|self.capture.admit_sized(cx,address,bytes))?}else{self.capture.admit_sized(cx,address,bytes)?};if !admitted{return Ok(false)}let receipt=if self.closing{cx.with_original_retirement(|cx|self.decoder.close_owned_step(outcome,cx))?}else{self.decoder.close_owned_step(outcome,cx)?};if let Some(receipt)=receipt{self.retain_settlement(receipt)?}return Ok(false)}
            if !charge(cx,true,std::mem::size_of::<Option<ControlledRetirement<OriginalJsonOutputOutcome>>>())?{return Ok(false)}self.outcome.take();
        }Ok(true)
    }
    /// 🪜️ Readiness follows actual parser, typed receiver and decoder Source closure.
    pub fn advance(&mut self,wire:Option<&[u8]>,cx:&mut StepContext<'_>)->Result<InputAdmission,OriginalTurnFailure>{
        self.validate(wire,cx)?;
        if self.capture.is_admitted(){if !self.capture.settle_return(cx)?{return Ok(InputAdmission::Pending)}if self.returned{return Ok(if self.closing{if self.terminal_is_empty(){InputAdmission::Closed}else{InputAdmission::Pending}}else if self.failed{InputAdmission::Failed}else{InputAdmission::Ready})}return Ok(InputAdmission::Pending)}
        if self.returned&&!self.closing{return Ok(if self.failed{InputAdmission::Failed}else{InputAdmission::Ready})}
        if self.binding.is_none(){if !charge(cx,false,std::mem::size_of::<(Option<OriginalSourceBinding>,usize,usize)>())?{return Ok(InputAdmission::Pending)}let wire=wire.ok_or_else(invalid)?;self.binding=Some(OriginalSourceBinding::borrow(cx));self.pointer=wire.as_ptr() as usize;self.length=wire.len();return Ok(InputAdmission::Pending)}
        if !self.retire_outcome_step(cx)?{return Ok(if self.failed{InputAdmission::Failed}else{InputAdmission::Pending})}
        if self.closing{
            if self.event.is_some(){let progress=RetainedCloneProgress{copied_items:2,copied_bytes:std::mem::size_of::<Option<semio_framework::kernel::Event>>()+std::mem::size_of::<usize>(),..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()<2||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return Ok(InputAdmission::Pending)}cx.retained_progress().checked_add(progress)?;let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};if poll.events.len()==poll.events.capacity(){return Err(invalid())}poll.events.push(self.event.take().ok_or_else(invalid)?);let posted=cx.consume_retained(progress);cx.consume_fuel(2);posted?;return Ok(InputAdmission::Pending)}
            if self.job_retirement.is_none()&&self.job.is_some(){if !charge(cx,true,std::mem::size_of::<Option<semio_framework_actor::OriginalJobSource>>()+std::mem::size_of::<Option<ControlledRetirement<semio_framework_actor::OriginalJobSource>>>())?{return Ok(InputAdmission::Pending)}*self.job_retirement=Some(ControlledRetirement::retain(self.job.take().ok_or_else(invalid)?));return Ok(InputAdmission::Pending)}
            if let Some(owner)=self.job_retirement.as_mut(){
                if !owner.terminal_is_empty(){
                    if self.decoder.terminal_is_empty(){retire_step(owner,cx)?;}
                    else {let address=(&mut *self.settlement as *mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>) as usize;let bytes=std::mem::size_of::<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>();if !cx.with_original_retirement(|cx|self.capture.admit_sized(cx,address,bytes))?{return Ok(InputAdmission::Pending)}if let Some(raw)=cx.with_original_retirement(|cx|self.decoder.close_owned_step(owner,cx))?{self.retain_settlement(raw)?}}
                    return Ok(InputAdmission::Pending)
                }
                if !charge(cx,true,std::mem::size_of::<Option<ControlledRetirement<semio_framework_actor::OriginalJobSource>>>())?{return Ok(InputAdmission::Pending)}self.job_retirement.take();return Ok(InputAdmission::Pending)
            }
            if self.retirement.is_none()&&self.input.is_some(){if !charge(cx,true,std::mem::size_of::<Option<ActorInput>>()+std::mem::size_of::<Option<ControlledRetirement<ActorInput>>>())?{return Ok(InputAdmission::Pending)}*self.retirement=Some(ControlledRetirement::retain(self.input.take().ok_or_else(invalid)?));return Ok(InputAdmission::Pending)}
            if let Some(owner)=self.retirement.as_mut(){
                if !owner.terminal_is_empty(){
                    if self.decoder.terminal_is_empty(){retire_step(owner,cx)?;}
                    else {let address=(&mut *self.settlement as *mut Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>) as usize;let bytes=std::mem::size_of::<Option<ControlledRetirement<OriginalNativeDecodeOutcome<RetainedCloneStep>>>>();if !cx.with_original_retirement(|cx|self.capture.admit_sized(cx,address,bytes))?{return Ok(InputAdmission::Pending)}if let Some(raw)=cx.with_original_retirement(|cx|self.decoder.close_owned_step(owner,cx))?{self.retain_settlement(raw)?}}
                    return Ok(InputAdmission::Pending)
                }
                if !charge(cx,true,std::mem::size_of::<Option<ControlledRetirement<ActorInput>>>())?{return Ok(InputAdmission::Pending)}self.retirement.take();return Ok(InputAdmission::Pending)
            }
            if self.returned{return Ok(if self.terminal_is_empty(){InputAdmission::Closed}else{InputAdmission::Pending})}
        }
        if self.failed||self.input.is_some(){self.returning=true}
        if self.returning{
            let(address,bytes)=self.settlement_destination();let admitted=if self.closing{cx.with_original_retirement(|cx|self.capture.admit_sized(cx,address,bytes))?}else{self.capture.admit_sized(cx,address,bytes)?};if !admitted{return Ok(InputAdmission::Pending)}
            let close=if self.closing{cx.with_original_retirement(|cx|self.decoder.close_step(cx))?}else{self.decoder.close_step(cx)?};
            match close{
                OriginalJsonOutputClose::Producer(Some(raw))=>self.retain_settlement(raw)?,
                OriginalJsonOutputClose::Producer(None)=>{},
                OriginalJsonOutputClose::Source(_) if self.decoder.terminal_is_empty()=>{if charge(cx,true,std::mem::size_of::<bool>())?{self.returned=true}},
                OriginalJsonOutputClose::Source(_)=>{},
            }
        }else{
            let(address,bytes)=self.outcome_destination();if !self.capture.admit_sized(cx,address,bytes)?{return Ok(InputAdmission::Pending)}
            let raw=if self.decoder.ready(){self.decoder.take_step(&mut *self.input,cx)?}else{self.decoder.receive_step(wire.ok_or_else(invalid)?,cx)?};
            if let Some(raw)=raw{self.outcome_failed=raw.value.is_err()||raw.receiving_refusal.is_some();self.failed|=self.outcome_failed;*self.outcome=Some(ControlledRetirement::retain(raw));self.capture.record_sized_return(address,bytes)?;}
        }
        Ok(if self.returned&&self.capture.terminal_is_empty(){if self.closing{InputAdmission::Closed}else if self.failed{InputAdmission::Failed}else{InputAdmission::Ready}}else if self.failed{InputAdmission::Failed}else{InputAdmission::Pending})
    }
}
impl Drop for OriginalActorInputReceiving{fn drop(&mut self){assert!(std::thread::panicking()||self.capture.terminal_is_empty()&&self.decoder.terminal_is_empty()&&self.input.is_none()&&self.outcome.is_none()&&self.settlement.is_none()&&self.retirement.is_none()&&self.job.is_none()&&self.job_retirement.is_none(),"actor input retains original decoder or payload")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
