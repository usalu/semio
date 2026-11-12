//! 🛬️ Original Energy actions receive their actual leaf arguments before command encoding and ownership handoff.
use semio_framework_job::{StepContext,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalSourceBinding,OriginalTurnFailure,OriginalTurnPrimary,InteractiveJobCloseStep};
use semio_framework_value::{DslValue,NativeDecodeControl,OriginalReceivingPrimary,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,native_decoding::construction::NativeConstructionScope,retirement::{RetireOwned,RetirementStep},retained_clone::{NativeAllocationIdentity,OriginalEffectReply,RetainedCloneClose,RetainedCloneSource,StagedSourceAdmission,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::NativeReceivingFailure;
use semio_framework_diagnostic::Fault;
use semio_framework_action_bus::RetainedToolWireInput;
use semio_plugin_decl_app::tool_contract::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory};
use semio_plugin_command::command_admission::original_typed_command::{ArtifactOriginalCommandWire as Wire,original_wire::ArtifactPreparedOperationSource};
use semio_s_artifact_energy_model_native_mutations_singletons::action::{NativeActionFactory,NativeActionReceiving};
use std::{mem::{size_of,ManuallyDrop},marker::PhantomData};
/// 🧬️ Authored static descriptors and borrowed command wire select the exact public Energy vocabulary.
pub trait EnergyOriginalCommand<M>:RetireOwned+Send+Sync+'static{
 fn descriptor(index:usize)->Option<(&'static str,&'static str,NativeActionFactory<M>)>;
 fn from_original_mutation(kind:String,mutation:M)->Self;
 fn original_wire(&self)->ArtifactPreparedOperationSource<'_>;
 const MAXIMUM_WIRE_BYTES:usize;
 fn from_action(action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>)->Result<Self,Fault> where Self:Sized;
}
/// 🏭️ The declaration installs one stateless factory over its authored command and mutation vocabulary.
pub struct EnergyActionFactory<C,M>(PhantomData<fn()->(C,M)>);
impl<C,M> EnergyActionFactory<C,M>{pub const fn new()->Self{Self(PhantomData)}}

const STEP_WORK:u64=4096;
const SELECT:u8=0;const ADMIT:u8=1;const BIRTH:u8=2;const CAPTURE:u8=3;const BEGIN:u8=4;const RECEIVE:u8=5;const TAKE:u8=6;const CLOSE_RECEIVER:u8=7;const COMMAND:u8=8;const WIRE_BIRTH:u8=9;const WIRE_ADVANCE:u8=10;const WIRE_TAKE:u8=11;const WIRE_CLOSE:u8=12;const READY:u8=13;const TAKEN:u8=14;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy action command source or ownership phase changed")}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn wire_source<C:EnergyOriginalCommand<M>,M>(command:&C)->ArtifactPreparedOperationSource<'_>{command.original_wire()}
fn actual<O>(result:&Result<(O,RetainedCloneProgress),ValueError>,reported:&mut Option<usize>){if let Ok((_,progress))=result{if progress.retained_capacity_bytes!=0&&reported.is_none(){*reported=Some(progress.retained_capacity_bytes)}}}
/// 🚨️ A constructor's own refusal is the failure the caller sees; its complete causes stay in the resident admission until closure retires them.
fn built<O>(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{let mut refused=None;let settled=semio_framework_job::admit_original_construction_settled(cx,admission,output,demand,1,|_,reported|{let result=construct();actual(&result,reported);if let Err(error)=&result{refused=Some(error.clone())}RetainedWorkOutcome{result,work:1}});match refused{Some(error)=>Err(error.into()),None=>settled}}
fn built_with<O>(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{semio_framework_job::admit_original_construction_settled(cx,admission,output,demand,1,|control,reported|{let result=construct(control);actual(&result,reported);RetainedWorkOutcome{result,work:1}})}
/// 🪜️ One bounded state change settles through the resident admission and its resident marker, so a pending turn never repeats the change.
fn stepped(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,marker:&mut Option<()>,demand:RetirementDemand,apply:impl FnOnce()->Result<RetainedCloneProgress,ValueError>)->Result<bool,OriginalTurnFailure>{let settled=built(cx,admission,marker,original(demand),||apply().map(|progress|((),progress)))?;if settled{marker.take();}Ok(settled)}
fn unbuilt(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission)->Result<bool,OriginalTurnFailure>{if admission.terminal_is_empty(){return Ok(true)}admission.close_step(cx)?;Ok(admission.terminal_is_empty())}
fn retired(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{semio_framework_job::close_original_effect_step(cx,demand,advance)}
fn settled(effect:&mut RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work:effect.performed_work()})}
fn unit(effect:&mut RetainedAllocationEffect<'_>,run:impl FnOnce()->RetainedCloneStep)->OriginalRetirementPoll{let paid=effect.performed(1).and_then(|admitted|if admitted{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Energy action producer exceeded its original work"))});match paid{Ok(())=>settled(effect,Ok(run())),Err(error)=>settled(effect,Err(error))}}
fn begun<T:RetireOwned>(retirement:&mut RetainedCloneClose,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let turn=retirement.begin_granted(value,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; `false` means it was not funded and nothing ran.
fn effected(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.should_yield()||!retained_clone_grant_admits(grant,d){return Ok(false)}
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_work(performed);result?;Ok(true)
}
/// 📤️ Runs one receiving poll inside one original effect and posts its settled work; `false` means it was not funded.
fn funded(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>)->Result<bool,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.should_yield()||!retained_clone_grant_admits(grant,d){return Ok(false)}
 let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
 let overrun=performed>turn.work;semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map_err(OriginalTurnPrimary::Receiving),work:turn.work})?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy receiver performed more work than it settled").into())}Ok(true)
}
fn primary(failure:NativeReceivingFailure)->OriginalTurnPrimary{OriginalTurnPrimary::Receiving(match failure{NativeReceivingFailure::Primary(primary)=>primary,NativeReceivingFailure::ValueEffect(error)=>error.receiving,NativeReceivingFailure::RetirementEffect(error)=>error.receiving,NativeReceivingFailure::ReceivingEffect(error)=>error.receiving})}
/// 🛬️ Advances one receiver inside the decoder's receiving scope against the caller's Source witness.
fn receiving(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,u64,RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>)->Result<(),OriginalTurnFailure>{
 if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(())}
 let maximum=cx.fuel_remaining().min(STEP_WORK);let mut out=None;let mut polled=None;
 cx.with_native_receiving(&mut out,|native,grant|{polled=Some(run(native,maximum,grant));Ok(())})?;
 match polled{Some(RetainedWorkPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{result:turn.result.map_err(primary),work:turn.work})?;Ok(())},Some(RetainedWorkPoll::Pending{work})=>{cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));Ok(())},None=>Ok(())}
}
/// ♻️ Begins or continues the close of one leaf receiver inside the calling turn's retirement effect.
fn receiver_step<M>(receiver:&mut dyn NativeActionReceiving<M>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let mut begun=None;let mut closed=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut begun).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop))}else{OriginalEffectReply::borrow(&mut closed).and_then(|reply|receiver.close_into(body,effect,reply).map(drop))};
 let work=effect.performed_work();
 match(ran,begun.take(),closed.take()){
  (Err(error),..)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(()),Some(result),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),
  (Ok(()),None,Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(invalid().into())}),work}),
  (Ok(()),None,None)=>RetainedWorkPoll::Pending{work},
 }
}
struct Producer<C:EnergyOriginalCommand<M>,M:RetireOwned+Send+Sync+'static>{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,index:usize,kind:Option<&'static str>,factory:Option<NativeActionFactory<M>>,args:ManuallyDrop<Option<DslValue>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,admit:ManuallyDrop<Option<StagedSourceAdmission<DslValue,DslValue>>>,receiver:ManuallyDrop<Option<Box<dyn NativeActionReceiving<M>>>>,mutation:ManuallyDrop<Option<M>>,command:ManuallyDrop<Option<Box<C>>>,wire:ManuallyDrop<Option<Box<Wire<C>>>>,input:ManuallyDrop<Option<RetainedToolWireInput>>,failure:ManuallyDrop<Option<OriginalConstructionFailure>>,retirement:RetainedCloneClose,construction:OriginalConstructionAdmission,unit:Option<()>,phase:u8,closing:bool,fallback:bool}
impl<C:EnergyOriginalCommand<M>,M:RetireOwned+Send+Sync+'static> ArtifactOriginalCommandProducerFactory<C> for EnergyActionFactory<C,M>{
 fn birth_demands(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{capacity_bytes:size_of::<Producer<C,M>>(),..fixed::<Producer<C,M>>()})}
 fn admit(&self,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<Box<dyn ArtifactOriginalCommandProducer<C>>>)->Result<bool,OriginalTurnFailure>{let d=self.birth_demands()?;let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;built_with(cx,admission,output,original(d),|control|Ok((Box::new(Producer::<C,M>{binding,decoder:(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?),body,index:0,kind:None,factory:None,args:ManuallyDrop::new(None),source:ManuallyDrop::new(None),admit:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),mutation:ManuallyDrop::new(None),command:ManuallyDrop::new(None),wire:ManuallyDrop::new(None),input:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),retirement:Default::default(),construction:Default::default(),unit:None,phase:SELECT,closing:false,fallback:false})as Box<dyn ArtifactOriginalCommandProducer<C>>,receipt(d))))}
}
impl<C:EnergyOriginalCommand<M>,M:RetireOwned+Send+Sync+'static> Producer<C,M>{
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
 fn step_demands(&self,action:&str)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_quote()}Ok(match self.phase{
  SELECT=>match C::descriptor(self.index){Some((candidate,_,_))=>RetirementDemand{copy_bytes:if action.len()==candidate.len(){candidate.len()*2}else{0}+size_of::<(usize,u8,Option<&str>,Option<NativeActionFactory<M>>,Option<DslValue>,Option<DslValue>)>(),depth:1,..Default::default()},None=>fixed::<(usize,bool,Option<&str>)>()},
  ADMIT=>{let d=match self.admit.as_ref(){Some(a)=>a.next_demand()?,None=>fixed::<StagedSourceAdmission<DslValue,DslValue>>()};RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Option<StagedSourceAdmission<DslValue,DslValue>>>(),..d}},
  BIRTH=>(self.factory.ok_or_else(invalid)?.birth)(),CAPTURE=>self.receiver.as_ref().ok_or_else(invalid)?.capture_demands()?,BEGIN=>self.receiver.as_ref().ok_or_else(invalid)?.receiver_birth_demands()?,RECEIVE=>self.receiver.as_ref().ok_or_else(invalid)?.demands(self.body,STEP_WORK)?,TAKE=>self.receiver.as_ref().ok_or_else(invalid)?.take_demands()?,
  CLOSE_RECEIVER=>self.receiver_close_quote()?,COMMAND=>RetirementDemand{copy_bytes:self.kind.ok_or_else(invalid)?.len()+size_of::<(M,C,Option<Box<C>>,u8)>(),capacity_bytes:self.kind.ok_or_else(invalid)?.len()+size_of::<C>(),depth:1,..Default::default()},WIRE_BIRTH=>Wire::<C>::birth_demands(),WIRE_ADVANCE=>self.wire.as_ref().ok_or_else(invalid)?.demands()?,WIRE_TAKE=>self.wire.as_ref().ok_or_else(invalid)?.take_demands()?,WIRE_CLOSE=>self.wire_close_quote()?,READY=>self.take_demands()?,_=>return Err(invalid()),})}
 fn receiver_close_quote(&self)->Result<RetirementDemand,ValueError>{let Some(receiver)=self.receiver.as_ref()else{return Ok(fixed::<u8>())};if receiver.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn NativeActionReceiving<M>>>>(),..receiver.release_demands()})}if !receiver.is_closing(){return receiver.begin_close_demand()}receiver.close_demands(self.body)}
 fn close_receiver(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(receiver)=self.receiver.as_ref()else{return Ok(true)};let terminal=receiver.terminal_is_empty();let d=self.receiver_close_quote()?;let body=self.body;
  retired(cx,d,|effect|if terminal{unit(effect,||{self.receiver.take();RetainedCloneStep::Complete(receipt(d))})}else{match self.receiver.as_mut(){Some(receiver)=>receiver_step(&mut **receiver,body,effect),None=>settled(effect,Err(invalid()))}})?;Ok(false)}
 fn wire_close_quote(&self)->Result<RetirementDemand,ValueError>{let Some(wire)=self.wire.as_ref()else{return Ok(fixed::<u8>())};if wire.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<Wire<C>>>>(),..Wire::<C>::release_demands()})}wire.close_demands()}
 fn close_wire(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(wire)=self.wire.as_mut()else{return Ok(true)};if !wire.terminal_is_empty(){wire.close_step(cx)?;return Ok(false)}let d=self.wire_close_quote()?;retired(cx,d,|effect|unit(effect,||{self.wire.take();RetainedCloneStep::Complete(receipt(d))}))?;Ok(false)}
 fn next_phase(&mut self,phase:u8,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=fixed::<u8>();stepped(cx,&mut self.construction,&mut self.unit,d,||{self.phase=phase;Ok(receipt(d))})}
 fn owned_is_empty(&self)->bool{self.args.is_none()&&self.admit.is_none()&&self.source.is_none()&&self.receiver.is_none()&&self.mutation.is_none()&&self.command.is_none()&&self.wire.is_none()&&self.input.is_none()&&self.failure.is_none()&&self.retirement.is_empty()}
 fn close_quote(&self)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(fixed::<bool>())}if self.receiver.is_some(){return self.receiver_close_quote()}if self.wire.is_some(){return self.wire_close_quote()}if !self.retirement.is_empty(){return self.retirement.demands(self.body)}if self.failure.is_none()&&self.construction.has_failure(){return Ok(self.construction.take_failure_demand().physical())}if self.failure.is_some(){return self.retirement.next_owner_demands::<OriginalConstructionFailure>(true,&None,self.body)}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed::<Option<RetainedCloneSource<DslValue>>>())}else{source.next_close_demand(self.body)}}if self.admit.is_some(){return self.retirement.next_owner_demands::<StagedSourceAdmission<DslValue,DslValue>>(true,&None,self.body)}if self.args.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,self.body)}if self.mutation.is_some(){return self.retirement.next_owner_demands::<M>(true,&None,self.body)}if self.command.is_some(){return self.retirement.next_owner_demands::<Box<C>>(true,&None,self.body)}if self.input.is_some(){return self.retirement.next_owner_demands::<RetainedToolWireInput>(true,&None,self.body)}Ok(Default::default())}
}
impl<C:EnergyOriginalCommand<M>,M:RetireOwned+Send+Sync+'static> ArtifactOriginalCommandProducer<C> for Producer<C,M>{
 fn demands(&self,action:&str,_args:&Option<DslValue>,_fault:&Option<Fault>,_body:usize)->Result<RetirementDemand,ValueError>{self.step_demands(action)}
 fn advance(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||fault.is_some(){return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original Energy action was cancelled").into())}if self.ready(){return Ok(true)}let d=self.step_demands(action)?;let g=cx.retained_grant();match self.phase{
  SELECT=>{let descriptor=C::descriptor(self.index);stepped(cx,&mut self.construction,&mut self.unit,d,||{match descriptor{Some((candidate,kind,factory))=>{if action==candidate{self.kind=Some(kind);self.factory=Some(factory);*self.args=Some(args.take().unwrap_or_else(||DslValue::Object(Vec::new())));self.phase=ADMIT}else{self.index+=1}}None=>{self.kind=Some("");self.fallback=true;self.phase=COMMAND}}Ok(receipt(d))})?;},
  ADMIT=>{if self.admit.is_none(){stepped(cx,&mut self.construction,&mut self.unit,d,||{let args=self.args.take().ok_or_else(invalid)?;*self.admit=Some(StagedSourceAdmission::owned(args));Ok(receipt(d))})?;}else{let admission=self.admit.as_mut().ok_or_else(invalid)?;effected(cx,d,|effect|admission.advance(&mut self.source,effect).result)?;if self.admit.as_ref().is_some_and(StagedSourceAdmission::terminal_is_empty){self.admit.take();}}if self.source.is_some(){self.phase=BIRTH}},
  BIRTH=>{let factory=self.factory.ok_or_else(invalid)?;built(cx,&mut self.construction,&mut self.receiver,original(d),||(factory.admit)(g))?;if self.receiver.is_some(){self.phase=CAPTURE}},
  CAPTURE=>{stepped(cx,&mut self.construction,&mut self.unit,d,||{let progress=self.receiver.as_mut().ok_or_else(invalid)?.capture(&mut self.source,g)?;if self.source.is_none(){self.phase=BEGIN}Ok(progress)})?;},
  BEGIN=>{stepped(cx,&mut self.construction,&mut self.unit,d,||{let progress=self.receiver.as_mut().ok_or_else(invalid)?.begin_receiver(g)?;self.phase=RECEIVE;Ok(progress)})?;},
  RECEIVE=>{let body=self.body;receiving(cx,d,|native,maximum,grant|match(self.receiver.as_mut(),native.source_witness()){(Some(receiver),Ok(witness))=>receiver.advance(native,witness,body,maximum,grant),(None,_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0}),(_,Err(error))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})})?;if self.receiver.as_ref().is_some_and(|receiver|receiver.ready()){self.phase=TAKE}},
  TAKE=>{let receiver=self.receiver.as_mut().ok_or_else(invalid)?;funded(cx,d,|effect|receiver.take(&mut self.mutation,effect))?;if self.mutation.is_some(){self.phase=CLOSE_RECEIVER}},
  CLOSE_RECEIVER=>{if self.close_receiver(cx)?{self.next_phase(COMMAND,cx)?;}},
  COMMAND=>{built(cx,&mut self.construction,&mut self.command,original(d),||{let command=if self.fallback{C::from_action(action,args,fault).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"Energy action has no declared original command source or its arguments were refused"))?}else{let kind=self.kind.ok_or_else(invalid)?.to_owned();let mutation=self.mutation.take().ok_or_else(invalid)?;C::from_original_mutation(kind,mutation)};Ok((Box::new(command),receipt(d)))})?;if self.command.is_some(){self.phase=WIRE_BIRTH}},
  WIRE_BIRTH=>{if Wire::admit(&mut self.command,wire_source::<C,M>,C::MAXIMUM_WIRE_BYTES,cx,&mut self.construction,&mut self.wire)?&&self.wire.is_some(){self.phase=WIRE_ADVANCE}},
  WIRE_ADVANCE=>{if self.wire.as_mut().ok_or_else(invalid)?.advance(cx)?{self.phase=WIRE_TAKE}},
  WIRE_TAKE=>{if self.wire.as_mut().ok_or_else(invalid)?.take_into(&mut self.command,&mut self.input,cx)?{self.phase=WIRE_CLOSE}},
  WIRE_CLOSE=>{if self.close_wire(cx)?{self.next_phase(READY,cx)?;}},
  _=>return Err(invalid().into()),}Ok(self.ready())}
 fn ready(&self)->bool{!self.closing&&self.phase==READY&&self.command.is_some()&&self.input.is_some()}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed::<(Option<Box<C>>,Option<Box<C>>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>,u8)>())}
 fn take_into(&mut self,command:&mut Option<Box<C>>,wire:&mut Option<RetainedToolWireInput>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if command.is_some()||wire.is_some(){return Err(invalid().into())}let d=self.take_demands()?;stepped(cx,&mut self.construction,&mut self.unit,d,||{*command=self.command.take();*wire=self.input.take();self.phase=TAKEN;Ok(receipt(d))})?;Ok(self.phase==TAKEN)}
 fn close_demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{self.close_quote()}
 fn close_items(&self)->usize{1}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.terminal_is_empty(){return Ok(true)}
  if !self.closing{let d=fixed::<bool>();retired(cx,d,|effect|unit(effect,||{self.closing=true;RetainedCloneStep::Progress(receipt(d))}))?;return Ok(false)}
  if self.failure.is_none()&&self.construction.has_failure(){self.construction.take_failure_into_step(&mut self.failure,cx)?;return Ok(false)}
  if self.receiver.is_some(){self.close_receiver(cx)?;return Ok(false)}
  if self.wire.is_some(){self.close_wire(cx)?;return Ok(false)}
  if self.owned_is_empty(){return unbuilt(cx,&mut self.construction)}
  let d=self.close_quote()?;let body=self.body;
  retired(cx,d,|effect|{
   if !self.retirement.is_empty(){return self.retirement.step_granted(body,effect)}
   if self.failure.is_some(){return begun(&mut self.retirement,&mut self.failure,effect)}
   if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(body,effect)}return unit(effect,||{self.source.take();RetainedCloneStep::Complete(receipt(d))})}
   if self.admit.is_some(){return begun(&mut self.retirement,&mut self.admit,effect)}
   if self.args.is_some(){return begun(&mut self.retirement,&mut self.args,effect)}
   if self.mutation.is_some(){return begun(&mut self.retirement,&mut self.mutation,effect)}
   if self.command.is_some(){return begun(&mut self.retirement,&mut self.command,effect)}
   if self.input.is_some(){return begun(&mut self.retirement,&mut self.input,effect)}
   settled(effect,Ok(RetainedCloneStep::Complete(Default::default())))
  })?;Ok(false)
 }
 fn terminal_is_empty(&self)->bool{self.owned_is_empty()&&self.construction.terminal_is_empty()}
}
impl<C:EnergyOriginalCommand<M>,M:RetireOwned+Send+Sync+'static> Drop for Producer<C,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Energy action producer requires funded closure")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
