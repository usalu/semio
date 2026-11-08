//! 🔐️ Shared receiving keeps its source, output and callback frame in one movable owner.
//! 🔐️ One movable shared constructor retains its actual producer and original receipts.
use semio_framework_value_native_decoding::{NativeDecodeControl,NativeSharedAdmission,OriginalReceivingPrimary,NativeReceivingEffect,NativeReceivingEffectState,NativeReceivingEffectOutcome,NativeEffectCapturePlan};
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value_retained_clone::retained_clone::RetainedCloneClose;
use semio_framework_value_retirement::retirement::RetireOwned;
use semio_framework_value_shared_issuance::SealedShared;
use semio_framework_value_allocation_return::original::SharedOriginalStorage;
use std::{marker::PhantomData,mem::ManuallyDrop};
#[path="🏭️producer/🦀️.rs"]mod producer;
use producer::{Producer,Context,Operation,OperationKind,Kind};
type Step=RetainedCloneStep;
type Failure=OriginalRetirementFailure;
type Reply<'a>=OriginalEffectReply<'a,Step,Failure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,Step,Failure>;
fn invalid()->ValueError{ValueError::invariant("shared slots require their original pending operation and empty destination")}
#[derive(Clone,Copy,PartialEq,Eq)]
struct SharedCloseLoan{ledger:NativeAllocationIdentity,recipient:Option<u64>,work:OriginalWorkBindingIdentity}
#[derive(Clone,Copy,PartialEq,Eq)]
struct SharedCloseSelection{invocation:OriginalEffectInvocationIdentity,body:usize,grant:RetainedCloneGrant,loan:Option<SharedCloseLoan>,demand:RetirementDemand,work:u64}
/// 🧳️ Owns the actual input, native birth, output and original action across relocation.
pub struct NativeSharedSlots<T:RetireOwned+Sync,R:RetireOwned+Sync>{authority:Option<OriginalEffectAuthority<Producer<T>>>,retired:ManuallyDrop<Option<Producer<T>>>,bootstrap:Option<RetainedCloneProgress>,effect:NativeReceivingEffect<Failure>,local_action:Option<OriginalEffectAction<Step,Failure>>,local_reply:ManuallyDrop<Option<Result<Step,Failure>>>,local_transfer:Option<RetainedCloneProgress>,operation:Option<OperationKind>,birth_pending:bool,result:PhantomData<R>}
impl<T:RetireOwned+Sync,R:RetireOwned+Sync> Default for NativeSharedSlots<T,R>{fn default()->Self{Self::vacant()}}
impl<T:RetireOwned+Sync,R:RetireOwned+Sync> NativeSharedSlots<T,R>{
 /// 🪹️ Supplies only a const empty template, without issuing authority or taking input.
 pub const fn vacant()->Self{Self{authority:None,retired:ManuallyDrop::new(None),bootstrap:None,effect:NativeReceivingEffect::empty(),local_action:None,local_reply:ManuallyDrop::new(None),local_transfer:None,operation:None,birth_pending:false,result:PhantomData}}

 /// 🧮️ Counts actual private fields, nesting the defining receiving-frame plan.
 pub const fn vacant_field_count()->usize{NativeReceivingEffect::<Failure>::vacant_field_count()+9}
 /// 📏️ Quotes a field in the actual exclusive uninitialized Slots backing.
 /// # Safety
 /// Target must be valid aligned exclusive uninitialized storage for Self throughout this seed plan.
 pub unsafe fn vacant_field_size(target:*mut std::mem::MaybeUninit<Self>,index:usize)->Option<usize>{fn field_size<T>(_:*mut T)->usize{std::mem::size_of::<T>()}let target=target.cast::<Self>();let nested=NativeReceivingEffect::<Failure>::vacant_field_count();unsafe{if index<nested{return NativeReceivingEffect::<Failure>::vacant_field_size(std::ptr::addr_of_mut!((*target).effect).cast(),index)}match index-nested{0=>Some(field_size(std::ptr::addr_of_mut!((*target).authority))),1=>Some(field_size(std::ptr::addr_of_mut!((*target).retired))),2=>Some(field_size(std::ptr::addr_of_mut!((*target).bootstrap))),3=>Some(field_size(std::ptr::addr_of_mut!((*target).local_action))),4=>Some(field_size(std::ptr::addr_of_mut!((*target).local_reply))),5=>Some(field_size(std::ptr::addr_of_mut!((*target).operation))),6=>Some(field_size(std::ptr::addr_of_mut!((*target).birth_pending))),7=>Some(field_size(std::ptr::addr_of_mut!((*target).result))),8=>Some(field_size(std::ptr::addr_of_mut!((*target).local_transfer))),_=>None}}}
 /// 🌾️ Initializes one quoted vacant field without constructing or borrowing the complete Slots owner.
 /// # Safety
 /// Target must satisfy vacant_field_size and each field must be initialized once before typed publication; no live owner may be overwritten.
 pub unsafe fn write_vacant_field(target:*mut std::mem::MaybeUninit<Self>,index:usize)->bool{let target=target.cast::<Self>();let nested=NativeReceivingEffect::<Failure>::vacant_field_count();unsafe{if index<nested{return NativeReceivingEffect::<Failure>::write_vacant_field(std::ptr::addr_of_mut!((*target).effect).cast(),index)}match index-nested{0=>std::ptr::addr_of_mut!((*target).authority).write(None),1=>std::ptr::addr_of_mut!((*target).retired).write(ManuallyDrop::new(None)),2=>std::ptr::addr_of_mut!((*target).bootstrap).write(None),3=>std::ptr::addr_of_mut!((*target).local_action).write(None),4=>std::ptr::addr_of_mut!((*target).local_reply).write(ManuallyDrop::new(None)),5=>std::ptr::addr_of_mut!((*target).operation).write(None),6=>std::ptr::addr_of_mut!((*target).birth_pending).write(false),7=>std::ptr::addr_of_mut!((*target).result).write(PhantomData),8=>std::ptr::addr_of_mut!((*target).local_transfer).write(None),_=>return false}}true}
 pub fn capture_demand()->Result<RetirementDemand,ValueError>{let mut demand=Producer::<T>::capture_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(Self::bootstrap_bytes()?).ok_or_else(invalid)?;Ok(demand)}
 fn bootstrap_bytes()->Result<usize,ValueError>{std::mem::size_of::<Producer<T>>().checked_mul(2).and_then(|n|n.checked_add(std::mem::size_of::<Option<OriginalEffectAuthority<Producer<T>>>>())).ok_or_else(invalid)}
 fn data(&self)->Option<&Producer<T>>{self.authority.as_ref().map(OriginalEffectAuthority::producer)}
 fn begin_capture(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{if self.authority.is_some(){return Ok(true)}let demand=Self::capture_demand()?;if self.retired.is_some()||!retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<2{return Ok(false)}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared authority birth exceeded its original Work"))}self.authority=Some(OriginalEffectAuthority::new(Producer::new()));self.bootstrap=Some(RetainedCloneProgress{copied_bytes:Self::bootstrap_bytes()?,..Default::default()});Ok(true)}
 pub fn capture_work_demand(&self,input:&Option<T>)->Result<u64,ValueError>{if self.has_pending_recovery()||self.retired.is_some()||self.data().is_some_and(|data|!data.empty())||input.is_none(){return Err(invalid())}Ok(if self.authority.is_none(){2}else{1})}
 pub fn input(&self)->Option<&T>{self.data().and_then(|data|data.input.as_ref())}
 pub fn output_ready(&self)->bool{!self.has_pending_recovery()&&self.data().is_some_and(Producer::output_ready)}
 pub fn advance_pending(&self)->bool{(self.effect.has_pending_recovery()&&!self.effect.transfer_ready())||self.birth_pending}
 pub fn has_pending_recovery(&self)->bool{self.effect.has_pending_recovery()||self.local_action.is_some()||self.local_reply.is_some()||self.local_transfer.is_some()||self.operation.is_some()||self.authority.as_ref().is_some_and(OriginalEffectAuthority::has_pending_action)}
 pub fn capture_plan(&self)->Result<Option<NativeEffectCapturePlan>,ValueError>{Ok(Some(self.effect.capture_plan()))}
 fn local<'reply,'turn,'source>(&mut self,kind:OperationKind,mut context:Context<'turn,'source,T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>where 'source:'turn{
  if self.effect.has_pending_recovery()||self.birth_pending||self.operation.is_some_and(|pending|pending!=kind){if self.local_action.is_none(){return Ok(reply.complete(Err(invalid().into())))}return Err(invalid())}
  if let Err(error)=self.authority.as_ref().ok_or_else(invalid)?.producer().validate_context(&context){if self.local_action.is_none(){return Ok(reply.complete(Err(error.into())))}return Err(error)}
  self.operation=Some(kind);if !self.authority.as_mut().ok_or_else(invalid)?.drive(&mut context,effect,&mut self.local_action)?{return Ok(reply.pending())}
  if !self.local_action.as_mut().ok_or_else(invalid)?.transfer_reply(&mut self.local_reply)?{return Ok(reply.pending())}
  self.authority.as_mut().ok_or_else(invalid)?.finish(&mut self.local_action)?;self.operation=None;let mut result=self.local_reply.take().ok_or_else(invalid)?;if let Some(bootstrap)=self.bootstrap.take(){result=match result{Ok(step)=>{let progress=bootstrap.checked_add(step.progress()).expect("capture quote bounds its native header");Ok(match step{Step::Complete(_)=>Step::Complete(progress),Step::Progress(_)=>Step::Progress(progress)})},Err(error)=>{let progress=bootstrap.checked_add(error.retained_progress()).expect("capture refusal retains its actual native header");Err(error.with_retained_progress(progress))}}}Ok(reply.complete(result))
 }
 fn advance_kind(&mut self,control:&mut NativeDecodeControl<'_>,kind:Kind)->Result<NativeReceivingEffectState,ValueError>{
  if self.local_action.is_some()||self.operation.is_some()||self.birth_pending||self.data().and_then(|data|data.kind)!=Some(kind){return Err(invalid())}
  let(demand,work)=match self.effect.pending_request(){Some(request)=>request,None=>(self.data().ok_or_else(invalid)?.demand()?,self.data().ok_or_else(invalid)?.work_demand()?)};
  let source=control.original_work_source_turn()?;let work_binding=control.original_work_borrow().binding_identity()?;let mut context=Context{source,work:work_binding,operation:Operation::Advance};self.authority.as_ref().ok_or_else(invalid)?.producer().validate_context(&context)?;
  let result=control.receive_original_effect_retaining_work_with(demand,work,&mut self.effect,self.authority.as_mut().ok_or_else(invalid)?,|_,effect,authority,action,_|authority.drive(&mut context,effect,action))?;
  Ok(result)
 }
 pub fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->Result<NativeReceivingEffectState,ValueError>{if !self.effect.has_pending_recovery()||self.effect.transfer_ready(){return Err(invalid())}self.advance_kind(control,self.data().and_then(|data|data.kind).ok_or_else(invalid)?)}
 
 /// 📥️ Holds the complete native step and every Source refusal until its funded local return.
 pub fn advance_result_ready(&self)->bool{self.effect.transfer_ready()}
 /// 🧾️ Settles the original child Work proof before paying a refused physical receipt.
 pub fn finish_refused_receipt_work(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.effect.finish_child_work_for_payout(control)}
 /// 📦️ Moves the exact refused receipt into its caller-resident completion under the original payout.
 pub fn stage_refused_receipt(&mut self,control:&mut NativeDecodeControl<'_>,payout:&OriginalConstructionAction,output:&mut Option<OriginalConstructionCompletion>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{self.effect.stage_receipt_completion(control,payout,output,effect)}
 /// 🪢️ Stages the same refused receipt while the original Work scope is guarded.
 pub fn stage_refused_receipt_with_loan(&mut self,recipient:&OriginalConstructionLoan<'_>,payout:&OriginalConstructionAction,output:&mut Option<OriginalConstructionCompletion>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{self.effect.stage_receipt_completion_with_loan(recipient,payout,output,effect)}
 /// 🪢️ Stages the published completion while the same original Work scope is guarded.
 pub fn stage_published_receipt_with_loan(&mut self,recipient:&OriginalConstructionLoan<'_>,completion:&mut Option<OriginalConstructionCompletion>,payout:&OriginalConstructionAction,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{self.effect.stage_published_receipt_with_loan(recipient,completion,payout,effect)}
 /// 🔁️ Retries the actual published receipt with its separately retained payout refusal.
 pub fn retry_published_receipt_with_loan(&mut self,recipient:&OriginalConstructionLoan<'_>,refusal:&OriginalConstructionCompletion,payout:&OriginalConstructionAction,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{self.effect.retry_published_receipt_with_loan(recipient,refusal,payout,effect)}
 /// 📥️ Moves the published original completion into the same resident receiving owner.
 pub fn stage_published_receipt(&mut self,control:&mut NativeDecodeControl<'_>,completion:&mut Option<OriginalConstructionCompletion>,payout:&OriginalConstructionAction,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{self.effect.stage_published_receipt_completion(control,completion,payout,effect)}
 /// 🔓️ Publishes the same receipt only after its original physical and Work settlement.
 pub fn publish_receipt(&mut self,control:&mut NativeDecodeControl<'_>,payout:&OriginalConstructionAction,proof:&OriginalWorkActionProof)->Result<bool,ValueError>{self.effect.publish_receipt_completion(control,payout,proof)}
 /// 📏️ Quotes the actual retained native frame and caller outcome destination.
 pub fn advance_result_demand(&self)->Result<RetirementDemand,ValueError>{self.effect.transfer_demand()}
 /// ⛽️ Returns the genuine native result handoff cost.
 pub fn advance_result_work_demand(&self)->u64{self.effect.transfer_work_demand()}
 /// 📤️ Returns the whole original native outcome without exposing an owned intermediate result.
 pub fn take_advance_result<'reply>(&mut self,output:&mut Option<NativeReceivingEffectOutcome<Failure>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{
  if self.local_action.is_some()||self.operation.is_some()||self.birth_pending{return Ok(reply.complete(Err(invalid().into())))}self.effect.transfer_into(output,effect,reply)
 }
 /// 🔁️ A recorded enclosing action settles its reply without replaying this native transfer.
 pub fn recover_take_advance_result<'reply>(&mut self,output:&mut Option<NativeReceivingEffectOutcome<Failure>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.take_advance_result(output,effect,reply)}


 /// 🪪️ Borrows only the identity of the actual settled semantic frame.
 pub fn advance_success_identity(&self)->Result<semio_framework_value_native_decoding::NativeReceivingSuccessIdentity,ValueError>{self.effect.resident_success_identity()}
 /// 🔍️ Authenticates the resident semantic frame or its paid terminal marker.
 pub fn matches_advance_success_identity(&self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity)->bool{self.effect.matches_success_identity(identity)}
 /// 📐️ Quotes the actual sparse semantic return and same-frame marker.
 pub fn advance_success_demand(&self)->Result<RetirementDemand,ValueError>{self.effect.resident_success_driver_demand()}
 /// 📥️ A native leaf keeps every refusal resident and returns only an actual successful step.
 pub fn acknowledge_advance_success(&mut self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity,output:&mut Option<Step>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{if self.local_action.is_some()||self.operation.is_some()||self.birth_pending{return Err(invalid())}self.effect.retire_resident_success_for_driver(identity,output,effect)}
 /// 🫴️ Records the sparse leaf receipt directly into its caller's original reply.
 pub fn acknowledge_advance_success_into<'r>(&mut self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity,output:&mut Option<Step>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{let result=self.acknowledge_advance_success(identity,output,effect);Ok(reply.complete(result.map(|p|Step::Progress(p.unwrap_or_default())).map_err(Into::into)))}
 /// 📏️ Quotes retirement of the exact resident semantic marker.
 pub fn advance_success_close_demand(&self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity)->Result<RetirementDemand,ValueError>{self.effect.success_identity_close_demand(identity)}
 /// 🏁️ Clears the exact marker under its original Work after caller-owned settlement.
 pub fn close_advance_success(&mut self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{if self.local_action.is_some()||self.operation.is_some()||self.birth_pending{return Err(invalid())}self.effect.close_success_identity(identity,effect)}
 /// 🧾️ Deposits the paid native marker retirement receipt directly into the actual reply.
 pub fn close_advance_success_into<'r>(&mut self,identity:semio_framework_value_native_decoding::NativeReceivingSuccessIdentity,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{let result=self.close_advance_success(identity,effect);Ok(reply.complete(result.map(|p|p.map_or_else(||Step::Progress(Default::default()),Step::Complete)).map_err(Into::into)))}
 pub fn reclaim_input<'reply>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.local(OperationKind::Reclaim,Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::Reclaim(output)},effect,reply)}
 pub fn recover_reclaim_input<'reply>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.reclaim_input(output,effect,reply)}
 pub fn reclaim_input_demand(&self)->Result<RetirementDemand,ValueError>{Producer::<T>::take_demand::<T>()}
 fn close_prefix()->Result<usize,ValueError>{let transfer=OriginalEffectAuthority::<Producer<T>>::transfer_reply_demand()?.copy_bytes;transfer.checked_add(2*std::mem::size_of::<Option<OperationKind>>()).and_then(|n|n.checked_add(2*std::mem::size_of::<Option<RetainedCloneProgress>>())).and_then(|n|n.checked_add(std::mem::size_of::<Option<Result<Step,Failure>>>())).and_then(|n|n.checked_add(std::mem::size_of::<Result<Step,Failure>>())).ok_or_else(invalid)}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.effect.has_pending_recovery(){return Err(invalid())}if let Some(OperationKind::Close(selected))=self.operation{if selected.body!=body{return Err(ValueError::invariant("shared close body changed"))}return Ok(selected.demand)}if self.retired.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Producer<T>>>()+std::mem::size_of::<Producer<T>>(),depth:1,..Default::default()})}match self.authority.as_ref(){Some(owner)if owner.producer().empty()=>OriginalEffectAuthority::<Producer<T>>::producer_transfer_demand(),Some(owner)=>{let prefix=Self::close_prefix()?;let mut demand=owner.producer().close_demand(body.saturating_sub(prefix))?;demand.copy_bytes=demand.copy_bytes.checked_add(prefix).ok_or_else(invalid)?;Ok(demand)},None=>Ok(Default::default())}}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.effect.has_pending_recovery(){return Err(invalid())}if let Some(OperationKind::Close(selected))=self.operation{if selected.body!=body{return Err(ValueError::invariant("shared close body changed"))}return Ok(selected.work)}if self.retired.is_some()||self.data().is_some_and(Producer::empty){return Ok(1)}self.data().map_or(Ok(0),|data|data.close_work(body.saturating_sub(Self::close_prefix()?))?.checked_add(3).ok_or_else(invalid))}
 fn close_header<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{if self.has_pending_recovery(){return Err(invalid())}if let Some(retired)=self.retired.as_ref(){if let Err(error)=retired.validate_context(&Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::Close(body,None)}){return Ok(reply.complete(Err(error.into())))}let demand=self.close_demand(body)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared producer clear exceeded original Work"))}drop(self.retired.take());return Ok(reply.complete(Ok(Step::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}if self.authority.is_none(){return Ok(reply.complete(Ok(Step::Complete(Default::default()))))}let context=Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::Close(body,None)};let turn=OriginalEffectAuthority::transfer_producer(&mut self.authority,&context,&mut self.retired,effect);Ok(reply.complete(turn.result.map(|progress|Step::Progress(progress.unwrap_or_default())).map_err(Into::into)))}
 fn close_local<'reply>(&mut self,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>,recover:bool)->Result<Complete<'reply>,ValueError>{
  if self.effect.has_pending_recovery()||self.birth_pending{return Err(invalid())}let invocation=effect.invocation().ok_or_else(invalid)?.identity();let grant=effect.grant();let loan=match original.as_deref_mut(){None=>None,Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(invalid())}Some(SharedCloseLoan{ledger:original.ledger_identity(),recipient:original.retirement_recipient_identity(),work})}};let selected=match self.operation{Some(OperationKind::Close(selected))=>{if !recover||selected.invocation!=invocation||selected.body!=body||selected.grant!=grant||selected.loan!=loan{return Err(invalid())}selected},Some(_)=>return Err(invalid()),None=>{if recover||self.local_action.is_some()||self.local_reply.is_some()||self.local_transfer.is_some(){return Err(invalid())}let demand=self.close_demand(body)?;let work=self.close_work_demand(body)?;if !retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared close route exceeded original Work").into())))}let selected=SharedCloseSelection{invocation,body,grant,loan,demand,work};self.operation=Some(OperationKind::Close(selected));selected}};
  let prefix=Self::close_prefix()?;let child_body=selected.body.saturating_sub(prefix);let mut context=Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::Close(child_body,original.as_deref_mut())};if self.local_reply.is_none(){let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.checked_sub(prefix).ok_or_else(invalid)?,..grant};let maximum=effect.remaining_work_units()?.checked_sub(2).ok_or_else(invalid)?;let ready={let mut child=effect.reborrow(child_grant,maximum)?;self.authority.as_mut().ok_or_else(invalid)?.drive(&mut context,&mut child,&mut self.local_action)?};if !ready{return Ok(reply.pending())}let turn=self.authority.as_mut().ok_or_else(invalid)?.transfer_reply_into(&context,&mut self.local_action,&mut self.local_reply,effect);match turn.result{Ok(Some(progress))=>self.local_transfer=Some(progress),Ok(None)=>return Ok(reply.pending()),Err(error)=>return Err(error)}}
  let Some(transfer)=self.local_transfer else{return Err(invalid())};if effect.remaining_work_units()?<1{return Ok(reply.pending())}if !effect.performed(1)?{return Ok(reply.pending())}let mut result=self.local_reply.take().expect("shared close reply remains resident");self.local_transfer=None;self.operation=None;let header=prefix.checked_sub(OriginalEffectAuthority::<Producer<T>>::transfer_reply_demand()?.copy_bytes).ok_or_else(invalid)?;let mut progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};progress.copied_bytes=progress.copied_bytes.checked_add(transfer.copied_bytes).and_then(|n|n.checked_add(header)).ok_or_else(invalid)?;progress.copied_items=progress.copied_items.max(1);if !progress.fits(grant){return Err(invalid())}result=match result{Ok(_)=>Ok(Step::Progress(progress)),Err(error)=>Err(error.with_retained_progress(progress))};Ok(reply.complete(result))
 }
 pub fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{if self.retired.is_some()||self.authority.is_none()||self.data().is_some_and(Producer::empty){return self.close_header(body,effect,reply)}self.close_local(body,None,effect,reply,false)}
 pub fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.close_local(body,None,effect,reply,true)}
 pub fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{if self.retired.is_some()||self.authority.is_none()||self.data().is_some_and(Producer::empty){return self.close_header(body,effect,reply)}self.close_local(body,Some(original),effect,reply,false)}
 pub fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.close_local(body,Some(original),effect,reply,true)}
 pub fn requires_original_birth(&self)->bool{self.operation.is_none()&&!self.effect.has_pending_recovery()&&self.data().is_some_and(OriginalEffectBirthProducer::requires_original_birth)}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,Failure>{
  let work=match original.original_work().binding_identity(){Ok(binding)=>binding,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};let mut context=Context{source:work.source_turn(),work,operation:Operation::Birth};
  match self.authority.as_mut().expect("selected original shared producer").drive_original_birth(&mut context,original,grant){Ok(poll)=>{self.birth_pending=matches!(poll,RetainedWorkPoll::Pending{..});poll},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 pub fn requires_original_control(&self)->bool{if let Some(OperationKind::Close(selected))=self.operation{return selected.loan.is_some()}self.data().is_some_and(Producer::requires_control)}
 pub fn terminal_is_empty(&self)->bool{!self.has_pending_recovery()&&!self.birth_pending&&self.local_reply.is_none()&&self.authority.is_none()&&self.retired.is_none()&&self.bootstrap.is_none()}
}
macro_rules! mode{
 ($result:ty,$kind:ident,$take:ident)=>{
 impl<T:RetireOwned+Sync> NativeSharedSlots<T,$result>{
  pub fn capture<'reply>(&mut self,input:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{if effect.work_source_turn().is_none(){return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared capture requires its original Source turn").into())))}if !self.begin_capture(effect)?{return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}self.local(OperationKind::Capture,Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::Capture(input,Kind::$kind)},effect,reply)}
  pub fn recover_capture<'reply>(&mut self,input:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.capture(input,effect,reply)}
  pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Producer::<T>::take_demand::<$result>()}
  pub fn take_work_demand(&self,output:&Option<$result>)->Result<u64,ValueError>{self.take_demand()?;if output.is_some(){return Err(invalid())}Ok(1)}
  pub fn take<'reply>(&mut self,output:&mut Option<$result>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.local(OperationKind::Take,Context{source:effect.work_source_turn(),work:OriginalWorkBindingIdentity::from_effect(effect),operation:Operation::$take(output)},effect,reply)}
  pub fn recover_take<'reply>(&mut self,output:&mut Option<$result>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>{self.take(output,effect,reply)}
  pub fn demand(&self)->Result<RetirementDemand,ValueError>{if self.data().and_then(|data|data.kind)!=Some(Kind::$kind){return Err(invalid())}self.effect.pending_request().map_or_else(||self.data().ok_or_else(invalid)?.demand(),|request|Ok(request.0))}
  pub fn advance_work_demand(&self)->Result<u64,ValueError>{if self.data().and_then(|data|data.kind)!=Some(Kind::$kind){return Err(invalid())}self.effect.pending_request().map_or_else(||self.data().ok_or_else(invalid)?.work_demand(),|request|Ok(request.1))}
  pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>)->Result<NativeReceivingEffectState,ValueError>{self.advance_kind(control,Kind::$kind)}
 }
 }}
mode!(SealedShared<T>,Sealed,TakeSealed);
mode!(SharedOriginalStorage<T>,Issued,TakeIssued);
impl<T:RetireOwned+Sync,R:RetireOwned+Sync> Drop for NativeSharedSlots<T,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared slots require original funded closure")}}


