//! 🔀️ The native enum receiving machine is compiled once per owner shape instead of once per derived enum; an enum contributes only its children table and its variant constructors.
use super::*;
use super::tree::{fixed,invalid,receipt,child_grant,NativeValueSource,NativeFieldPlanSlot,variant::{NativeVariantSelector,NativeVariantSpec}};
use crate::native_decoding::{NativeReceivingFailure,NativeSeedTemplate};
use crate::retained_clone::{RetainedCloneClose,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,retained_clone_grant_admits};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,ValueRefusalKind};
use super::machine::{RecordSlot,RecordSlotView,Source,Step,Close,progress};
use std::mem::{size_of,ManuallyDrop,MaybeUninit};

/// 🧩️ The resident state of one enum receiver that does not depend on its variant types.
pub struct NativeEnumCore<I:NativeValueSource+?Sized,N>{pub selector:NativeVariantSelector<I>,pub projection:RetainedOwnedProjectionAdmission<dyn NativeValueSource>,pub source:Source,pub output:ManuallyDrop<Option<N>>,pub retirement:RetainedCloneClose,pub direct:bool,pub phase:u8,pub closing:bool}

/// 👶️ The one live child of an enum receiver: its slot, its pending value and its variant constructor.
pub struct EnumChild<P,T,N>{pub slot:P,pub pending:ManuallyDrop<Option<T>>,pub construct:fn(T)->N}
impl<P:Default,T,N> EnumChild<P,T,N>{
 /// 🌱️ Births an empty child around its variant constructor.
 pub fn born(construct:fn(T)->N)->Self{Self{slot:P::default(),pending:ManuallyDrop::new(None),construct}}
}

/// 🔭️ The child operations that do not depend on a variant type, so they run through one erased view.
pub trait EnumChildOps<I:NativeValueSource+?Sized,N>{
 fn plan(&self,c:&NativeEnumCore<I,N>,child_bytes:usize,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>;
 fn apply(&mut self,c:&mut NativeEnumCore<I,N>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<Option<Step>>,OriginalReceivingPrimary>;
 fn close_plan(&self,c:&NativeEnumCore<I,N>,child_bytes:usize,body:usize)->Result<(RetirementDemand,u64),ValueError>;
 fn close(&mut self,c:&mut NativeEnumCore<I,N>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<Close>;
 fn retire(&mut self,begin:bool,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn work(&self,c:&NativeEnumCore<I,N>,body:usize)->Result<u64,ValueError>;
 fn nested(&mut self,c:&NativeEnumCore<I,N>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>;
 fn view(&self)->&dyn RecordSlotView;
 fn view_mut(&mut self)->&mut dyn RecordSlotView;
}

impl<I:NativeValueSource+?Sized,N,T:RetireOwned,P:RecordSlot<T>+NativeFieldPlanSlot> EnumChildOps<I,N> for EnumChild<P,T,N>{
 fn plan(&self,c:&NativeEnumCore<I,N>,_:usize,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>{
  let slot=&self.slot;let placed=size_of::<Option<T>>()+size_of::<Option<N>>()+size_of::<u8>();
  match c.phase{
   2=>if slot.plan_ready(){if c.selector.settled(){Ok((fixed(size_of::<u8>()),1,4))}else{Ok((c.selector.settlement_demand(body)?,c.selector.settlement_work_demand(body)?,8))}}else{Ok((slot.plan_demands(&c.source,body,maximum)?,1,3))},
   3=>{let mut d=slot.plan_take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(placed).ok_or_else(invalid)?;Ok((d,tree::work_sum(slot.plan_take_work_demand()?,2)?,4))},
   4=>Ok((fixed(placed),3,4)),
   5=>if slot.plan_terminal_is_empty(){Ok((fixed(size_of::<u8>()),1,4))}else if !slot.plan_is_closing(){Ok((slot.plan_begin_close_demand()?,slot.plan_begin_close_work_demand()?,9))}else{Ok((slot.plan_close_demands(body)?,slot.plan_close_work_demand(body)?,5))},
   _=>Err(invalid())}
 }
 fn apply(&mut self,c:&mut NativeEnumCore<I,N>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<Option<Step>>,OriginalReceivingPrimary>{
  match c.phase{
   2=>{c.phase=3;*work=1},
   3=>match self.slot.slot_take(&mut *self.pending,effect){RetainedWorkPoll::Pending{..}=>return Ok(Some(None)),RetainedWorkPoll::Complete(turn)=>{*work=turn.work;turn.result?;if self.pending.is_none(){return Ok(Some(Some(Step::Progress(Default::default()))))}c.phase=4;*work+=2}},
   4=>{let value=self.pending.take().ok_or_else(invalid)?;*c.output=Some((self.construct)(value));c.phase=5;*work=3},
   5=>{c.phase=6;*work=1},_=>return Err(invalid().into())}
  Ok(None)
 }
 fn close_plan(&self,c:&NativeEnumCore<I,N>,child_bytes:usize,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  if let Some(plan)=self.slot.slot_close_plan(body){return plan}
  if self.pending.is_some(){return Ok((c.retirement.next_owner_demands::<T>(true,&None,body)?,c.retirement.next_owner_work::<T>(true,&None,body)?))}
  Ok((fixed(child_bytes),1))
 }
 fn close(&mut self,c:&mut NativeEnumCore<I,N>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<Close>{
  if let Some(poll)=self.slot.slot_close_step(None,body,effect){return Some(poll)}
  if self.pending.is_some(){let turn=c.retirement.begin_granted(&mut *self.pending,effect);return Some(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(Step::Progress(Default::default()))).map_err(Into::into),work:turn.work}))}
  None
 }
 fn retire(&mut self,begin:bool,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{if begin{self.slot.slot_begin(effect)}else{self.slot.slot_retire(body,effect)}}
 fn work(&self,c:&NativeEnumCore<I,N>,body:usize)->Result<u64,ValueError>{self.slot.slot_work(&c.source,body)}
 fn nested(&mut self,c:&NativeEnumCore<I,N>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{self.slot.slot_nested(&c.source,control,witness,body,maximum,grant)}
 fn view(&self)->&dyn RecordSlotView{&self.slot}
 fn view_mut(&mut self)->&mut dyn RecordSlotView{&mut self.slot}
}

/// 🧬️ The children table of one derived enum: its variant spec and the born child of every selectable variant.
pub trait NativeEnumChildren<I:NativeValueSource+?Sized,N>:Send+'static+Sized{
 fn spec()->NativeVariantSpec;
 fn variants()->usize;
 fn born(index:usize)->Option<Self>;
 fn child(&self)->&dyn EnumChildOps<I,N>;
 fn child_mut(&mut self)->&mut dyn EnumChildOps<I,N>;
}

/// 🏗️ One native enum receiver: the shared core plus the live child of its derived enum.
pub struct NativeEnumReceiver<I:NativeValueSource+?Sized,N:RetireOwned,S:NativeEnumChildren<I,N>>{core:NativeEnumCore<I,N>,child:Option<S>}

fn finish(effect:&mut RetainedAllocationEffect<'_>,work:u128,result:Result<Step,OriginalRetirementFailure>)->Close{let recorded=effect.performed_work();let result=if work>recorded{match u64::try_from(work-recorded){Ok(owed)=>match effect.performed(owed){Ok(true)=>result,Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native enum close exceeded its original work").into()),Err(error)=>Err(error.into())},Err(_)=>Err(ValueError::invariant("native enum close work overflow").into())}}else{result};RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})}

fn failure<T,E:From<ValueError>>()->RetainedWorkPoll<T,E>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}

impl<I:NativeValueSource+?Sized,N:RetireOwned,S:NativeEnumChildren<I,N>> NativeEnumReceiver<I,N,S>{
 /// 🌱️ Births the receiver of one derived enum.
 pub fn birth()->Self{Self{core:NativeEnumCore{selector:NativeVariantSelector::new(S::spec()),projection:Default::default(),source:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),direct:false,phase:0,closing:false},child:None}}
 fn plan(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>{
  let c=&self.core;
  if c.closing||c.phase>=7{return Err(invalid())}
  c.selector.validate_input(input)?;
  match c.phase{
   0=>if c.selector.result().is_some(){Ok((fixed(size_of::<Option<S>>()+size_of::<bool>()+size_of::<u8>()),3,0))}else{Ok((c.selector.demands(input,body,maximum)?,match c.selector.select_advance(input,body,maximum)?{NativeAdvanceSelection::Nested=>1,NativeAdvanceSelection::Local{work}=>work},1))},
   1=>if !c.projection.terminal_is_empty(){Ok((c.projection.take_demand()?,c.projection.take_work_demand()?,24))}else{let child=input.try_project::<dyn NativeValueSource,ValueError,_>(if c.direct{usize::MAX-1}else{usize::MAX},|value|if c.direct{value.variant_payload()}else{Ok(value.as_source())})?;Ok((c.projection.demand(child)?,c.projection.work_demand(child)?,23))},
   2|3|4|5=>self.child.as_ref().ok_or_else(invalid)?.child().plan(c,size_of::<Option<S>>(),body,maximum),
   6=>if let Some(source)=c.source.as_ref(){if source.terminal_is_empty(){Ok((fixed(size_of::<Option<RetainedOwnedProjection<dyn NativeValueSource>>>()),1,6))}else{Ok((source.close_demand(body)?,source.close_work_demand(body)?,7))}}else{Ok((fixed(size_of::<u8>()),1,6))},
   _=>Err(invalid())}
 }
 fn apply(&mut self,action:u8,effect:&mut RetainedAllocationEffect<'_>,d:RetirementDemand)->Result<Option<Step>,OriginalReceivingPrimary>{
  let mut work:u128=0;let core=&mut self.core;let child=&mut self.child;
  let step=(||->Result<Option<Step>,OriginalReceivingPrimary>{match action{
   0=>{let selected=core.selector.result().ok_or_else(invalid)?;core.direct=selected.direct;let index=selected.index+if selected.direct{S::variants()}else{0};*child=Some(S::born(index).ok_or_else(invalid)?);core.phase=1;work=3},
   4=>{if let Some(early)=child.as_mut().ok_or_else(invalid)?.child_mut().apply(core,effect,&mut work)?{return Ok(early)}},
   6=>{if core.source.is_some(){core.source.take();}else{core.phase=7;}work=1},_=>return Err(invalid().into())}Ok(Some(Step::Progress(receipt(d))))})()?;
  let Some(step)=step else{return Ok(None)};
  let recorded=effect.performed_work();
  if work>recorded{let owed=u64::try_from(work-recorded).map_err(|_|ValueError::invariant("native enum action work overflow"))?;if !effect.performed(owed)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native enum action exceeded its original work").into())}}else if work<recorded{return Err(ValueError::invariant("native enum action recorded more work than it quoted").into())}
  Ok(Some(step))
 }
 fn retire(&mut self,action:u8,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{
  if action==7{return self.core.source.as_mut().expect("retained enum payload projection").close_step(body,effect).map(progress)}
  match self.child.as_mut(){Some(child)=>child.child_mut().retire(action==9,body,effect).map(progress),None=>failure()}
 }
 fn close_plan(&self,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  let c=&self.core;
  if !c.closing{return Err(invalid())}
  if !c.selector.terminal_is_empty(){return Ok((c.selector.close_demands(body)?,c.selector.close_work_demand(body)?))}
  if !c.projection.terminal_is_empty(){return Ok((c.projection.close_demand(body)?,c.projection.close_work_demand(body)?))}
  if let Some(source)=c.source.as_ref(){return if source.terminal_is_empty(){Ok((fixed(size_of::<Option<RetainedOwnedProjection<dyn NativeValueSource>>>()),1))}else{Ok((source.close_demand(body)?,source.close_work_demand(body)?))}}
  if !c.retirement.is_empty(){return Ok((c.retirement.demands(body)?,c.retirement.next_owner_work::<N>(false,&None,body)?))}
  if let Some(child)=self.child.as_ref(){return child.child().close_plan(c,size_of::<Option<S>>(),body)}
  Ok((c.retirement.next_owner_demands::<N>(c.output.is_some(),&None,body)?,c.retirement.next_owner_work::<N>(c.output.is_some(),&None,body)?))
 }
 fn close_poll(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{
  use RetainedWorkPoll::Complete;
  let g=effect.grant();
  if self.empty(){return Complete(RetainedWorkOutcome{result:Ok(Step::Complete(Default::default())),work:0})}
  let d=match self.close_plan(g.maximum_copy_bytes){Ok(plan)=>plan.0,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0})}
  let c=&mut self.core;
  if !c.selector.terminal_is_empty(){return native_close_settle(effect,|effect,reply|c.selector.close_into(body,effect,reply)).map(progress)}
  if !c.projection.terminal_is_empty(){return native_close_settle(effect,|effect,reply|c.projection.close_into(body,effect,reply)).map(progress)}
  if let Some(source)=c.source.as_mut(){if source.terminal_is_empty(){c.source.take();return finish(effect,1,Ok(Step::Progress(receipt(d))))}return source.close_step(body,effect).map(progress)}
  if !c.retirement.is_empty(){return c.retirement.step_granted(body,effect).map(progress)}
  if let Some(child)=self.child.as_mut(){return match child.child_mut().close(c,body,effect){Some(poll)=>poll,None=>{self.child=None;finish(effect,1,Ok(Step::Progress(receipt(d))))}}}
  let turn=c.retirement.begin_granted(&mut c.output,effect);
  Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(Step::Progress(Default::default()))).map_err(Into::into),work:turn.work})
 }
 fn empty(&self)->bool{let c=&self.core;c.selector.terminal_is_empty()&&c.projection.terminal_is_empty()&&c.source.is_none()&&self.child.is_none()&&c.output.is_none()&&c.retirement.is_empty()}
 const SEEDS:usize=9;
 unsafe fn seed_size(target:*mut MaybeUninit<Self>,index:usize)->Option<usize>{
  fn size<T>(_:*mut T)->usize{size_of::<T>()}
  let target=target.cast::<Self>();
  Some(unsafe{match index{
   0=>size(std::ptr::addr_of_mut!((*target).core.selector)),1=>size(std::ptr::addr_of_mut!((*target).core.projection)),2=>size(std::ptr::addr_of_mut!((*target).core.source)),
   3=>size(std::ptr::addr_of_mut!((*target).child)),4=>size(std::ptr::addr_of_mut!((*target).core.output)),5=>size(std::ptr::addr_of_mut!((*target).core.retirement)),
   6=>size(std::ptr::addr_of_mut!((*target).core.direct)),7=>size(std::ptr::addr_of_mut!((*target).core.phase)),8=>size(std::ptr::addr_of_mut!((*target).core.closing)),
   _=>return None}})
 }
 unsafe fn seed_write(target:*mut MaybeUninit<Self>,index:usize)->bool{
  let target=target.cast::<Self>();
  unsafe{match index{
   0=>std::ptr::addr_of_mut!((*target).core.selector).write(NativeVariantSelector::new(S::spec())),1=>std::ptr::addr_of_mut!((*target).core.projection).write(Default::default()),
   2=>std::ptr::addr_of_mut!((*target).core.source).write(ManuallyDrop::new(None)),3=>std::ptr::addr_of_mut!((*target).child).write(None),
   4=>std::ptr::addr_of_mut!((*target).core.output).write(ManuallyDrop::new(None)),5=>std::ptr::addr_of_mut!((*target).core.retirement).write(Default::default()),
   6=>std::ptr::addr_of_mut!((*target).core.direct).write(false),7=>std::ptr::addr_of_mut!((*target).core.phase).write(0),8=>std::ptr::addr_of_mut!((*target).core.closing).write(false),
   _=>return false}}
  true
 }
 /// 🧬️ Returns the factory seed template of this enum shape; an enum has no subset receiver.
 pub fn template(subset:bool)->Option<NativeSeedTemplate<Self>>{if subset{return None}Some(unsafe{NativeSeedTemplate::fields(Self::SEEDS,Self::seed_size,Self::seed_write)})}
}

impl<I:NativeValueSource+?Sized,N:RetireOwned,S:NativeEnumChildren<I,N>> NativeReceiver<I,N> for NativeEnumReceiver<I,N,S>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{let(d,q,_)=self.plan(input,body,maximum)?;Ok(if q<=maximum{d}else{Default::default()})}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<NativeAdvanceSelection,ValueError>{let(_,q,action)=self.plan(input,body,maximum)?;Ok(match action{1=>self.core.selector.select_advance(input,body,maximum)?,3=>NativeAdvanceSelection::Nested,_=>NativeAdvanceSelection::Local{work:q}})}
 fn advance_work_demand(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<u64,ValueError>{
  let(_,work,action)=self.plan(input,body,u64::MAX)?;
  match action{1=>self.core.selector.advance_work_demand(input,body),3=>self.child.as_ref().ok_or_else(invalid)?.child().work(&self.core,body),_=>Ok(work)}
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,g:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{
  use RetainedWorkPoll::Complete;
  let(d,q,action)=match self.plan(input,body,maximum){Ok(plan)=>plan,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if q==0||q>maximum||!retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0})}
  if action==1{return self.core.selector.advance_nested(input,control,witness,body,maximum,g)}
  if action==3{return match self.child.as_mut(){Some(child)=>child.child_mut().nested(&self.core,control,witness,body,maximum,g),None=>failure()}}
  failure()
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalReceivingPrimary>,ValueError>{
  let maximum=effect.maximum_work_units();
  if let Ok((d,q,action@(23|24)))=self.plan(input,body,maximum){
   if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}
   let core=&mut self.core;
   let done=if action==23{native_relay(reply.progress(),|reply|{let child=input.try_project::<dyn NativeValueSource,ValueError,_>(if core.direct{usize::MAX-1}else{usize::MAX},|value|if core.direct{value.variant_payload()}else{Ok(value.as_source())})?;core.projection.admit(child,effect,reply)})}else{native_relay(reply.progress(),|reply|core.projection.take(&mut *core.source,effect,reply))};
   if action==24&&core.source.is_some(){core.phase=2}
   return done}
  if let Ok((d,q,action))=self.plan(input,body,maximum){if matches!(action,1|8){if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}return if action==1{self.core.selector.advance_local(input,body,effect,reply.progress())}else{self.core.selector.settle_into(effect,reply.progress())}}}
  let result=(||->Result<Option<Step>,OriginalReceivingPrimary>{
   let(d,q,action)=self.plan(input,body,maximum)?;
   if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(Some(Step::Progress(Default::default())))}
   if matches!(action,5|7|9){return match self.retire(action,body,effect){RetainedWorkPoll::Pending{..}=>Ok(None),RetainedWorkPoll::Complete(turn)=>turn.result.map(Some).map_err(Into::into)}}
   let step=self.apply(action,effect,d)?;
   Ok(step.map(|step|if self.output_ready(){Step::Complete(step.progress())}else{step}))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.core.closing&&self.core.phase==7&&self.core.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.output_ready(){Ok(fixed(0))}else{Err(invalid())}}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 fn take<'reply>(&mut self,output:&mut Option<N>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalReceivingPrimary>,ValueError>{
  let turn=(||->RetainedWorkOutcome<Step,OriginalReceivingPrimary>{
   let d=match self.take_demand(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
   if output.is_some()||self.core.output.is_none(){return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
   if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0}}
   match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:1}}
   *output=self.core.output.take();self.core.phase=8;RetainedWorkOutcome{result:Ok(Step::Complete(receipt(d))),work:1}})();
  Ok(reply.complete(turn.result))
 }
 fn advance_pending(&self)->bool{self.core.selector.advance_pending()||self.child.as_ref().is_some_and(|child|child.child().view().view_pending())}
 fn is_closing(&self)->bool{self.core.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(if self.core.closing{Default::default()}else{fixed(size_of::<bool>())})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.core.closing))}
 fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{
  if self.core.closing{return Ok(reply.complete(Ok(Step::Complete(Default::default()))))}
  let d=fixed(size_of::<bool>());
  if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native enum close start exceeded its work authority").into())))}
  self.core.selector.begin_close();self.core.closing=true;Ok(reply.complete(Ok(Step::Complete(receipt(d)))))
 }
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{self.begin_close_into(effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_plan(body).map(|p|p.0)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_plan(body).map(|p|p.1)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(match self.close_poll(body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})}
 fn recover_close_step<'reply>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn close_original_step<'reply>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{self.close_step(body,effect,reply)}
 fn recover_close_original_step<'reply>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|owner:&Self|owner.terminal_is_empty())})}
}

impl<I:NativeValueSource+?Sized,N:RetireOwned,S:NativeEnumChildren<I,N>> Drop for NativeEnumReceiver<I,N,S>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"native enum receiving requires funded original custody closure")}}
