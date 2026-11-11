//! 🧮️ Sparse indexed edits retain paid coordinate directories and stream original headers without shifts.
use crate::{HeapSortCursor,HeapSortWork,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_control_contract::{NativeAllocationIdentity,OriginalEffectCallbackMode,OriginalEffectInvocationIdentity,OriginalEffectReply,OriginalEffectReplyComplete,OriginalWorkBindingIdentity};
use std::mem::{ManuallyDrop,size_of};

/// 📐️ Base coordinates address removals/modifications; addition coordinates address the resulting collection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IndexedEditShape{pub base_len:usize,pub removed:usize,pub modified:usize,pub added:usize}
/// 🎚️ One original authored coordinate directory.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditKind{Removed,Modified,Added}
impl IndexedEditKind{fn index(self)->usize{match self{Self::Removed=>0,Self::Modified=>1,Self::Added=>2}}fn next(self)->Option<Self>{match self{Self::Removed=>Some(Self::Modified),Self::Modified=>Some(Self::Added),Self::Added=>None}}}
/// 🪪️ Original ordinal remains the authored payload address after sorting coordinates.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IndexedEditEntry{pub index:usize,pub ordinal:usize}
crate::artifact_retire_leaf!(IndexedEditEntry);
/// 🚫️ Invalid sparse coordinates refuse before any candidate header changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditFault{Remove,Modify,Add}
/// 🚦️ One funded directory action, settled admission or settled semantic refusal.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditStep{Progress(RetainedCloneProgress),Admitted(RetainedCloneProgress),Refused(IndexedEditFault,RetainedCloneProgress)}
impl IndexedEditStep{pub fn progress(self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Admitted(p)|Self::Refused(_,p)=>p}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Reserve,Fill,Sort,Validate,Settled}

/// 🧳️ The caller retains the original immutable coordinate sources and admits every action before advancing.
pub struct IndexedEditPlan{shape:IndexedEditShape,orders:ManuallyDrop<[Option<Vec<IndexedEditEntry>>;3]>,kind:IndexedEditKind,phase:Phase,cursor:usize,removal:usize,sort:HeapSortCursor,fault:Option<IndexedEditFault>,output_len:Option<usize>,closing:bool,retirement:RetainedCloneClose,close_route:Option<CloseKey>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed edit plan lacks its live admitted phase")}
/// 🪪️ The actual original loan one close invocation borrowed.
pub(crate) type LoanKey=(NativeAllocationIdentity,Option<u64>,OriginalWorkBindingIdentity);
/// 🔑️ Retains the unfinished close invocation, its selected route, body and original loan until the actual reply is deposited.
pub(crate) type CloseKey=(OriginalEffectInvocationIdentity,u8,usize,Option<LoanKey>);
/// 🪪️ Authenticates the actual original invocation and its receive or recover mode.
pub(crate) fn invocation_identity(effect:&crate::RetainedAllocationEffect<'_>,recover:bool)->Result<OriginalEffectInvocationIdentity,ValueError>{let invocation=effect.invocation().ok_or_else(||ValueError::invariant("indexed edit close lacks its original invocation"))?;if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}{return Err(ValueError::invariant("indexed edit close changed its invocation mode"))}Ok(invocation.identity())}
/// 🪢️ Binds the actual original loan to the same Work Source as the effect.
pub(crate) fn loan_key(original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&crate::RetainedAllocationEffect<'_>)->Result<LoanKey,ValueError>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("indexed edit close changed its Work Source"))}Ok((original.ledger_identity(),original.retirement_recipient_identity(),work))}
pub type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
pub type Done<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
/// 📬️ One selected child close, dispatched over fresh or recovered invocation and ordinary or original-loan control.
pub trait CloseReply{fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;}
impl CloseReply for RetainedCloneClose{fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{match(original,recover){(Some(loan),false)=>self.step_original_into(loan,body,effect,reply),(Some(loan),true)=>self.recover_step_original_into(loan,body,effect,reply),(None,false)=>self.step_into(body,effect,reply),(None,true)=>self.recover_step_into(body,effect,reply)}}}
/// 🧾️ Deposits a settled child close with this parent's own header receipt and clears the retained invocation key.
pub(crate) fn deposit<'r>(slot:&mut Option<Result<RetainedCloneStep,crate::OriginalRetirementFailure>>,route:&mut Option<CloseKey>,key:CloseKey,bytes:usize,grant:RetainedCloneGrant,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
 let Some(answer)=slot.as_ref()else{return Ok(reply.pending())};
 let p=match answer{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let mut p=p.checked_add(RetainedCloneProgress{copied_bytes:bytes,..Default::default()})?;p.copied_items=p.copied_items.max(1);
 if !p.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed edit child receipt exceeds its original grant"))}
 let mut scope=reply.scope(route,key)?;let parent=scope.borrow()?;let answer=slot.take().expect("indexed edit child reply remains resident");
 parent.complete(match answer{Ok(_)=>Ok(RetainedCloneStep::Progress(p)),Err(error)=>Err(error.with_retained_progress(p))});Ok(scope.forwarded())
}
/// 🧾️ Settles a child turn whether it already settled or may still be pending, so a pending close inside one action is an invariant failure.
pub(crate) trait Settle<T,E>{fn settled(self)->crate::RetainedWorkOutcome<T,E>;}
impl<T,E> Settle<T,E> for crate::RetainedWorkOutcome<T,E>{fn settled(self)->crate::RetainedWorkOutcome<T,E>{self}}
impl<T,E:From<ValueError>> Settle<T,E> for crate::retained_clone::RetainedWorkPoll<T,E>{fn settled(self)->crate::RetainedWorkOutcome<T,E>{match self{crate::retained_clone::RetainedWorkPoll::Complete(turn)=>turn,crate::retained_clone::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original child close left its producer reply pending inside one action").into()),work}}}}
/// 🔁️ Lifts a settled or pending child turn into a poll so a pending close keeps propagating to the original driver.
pub(crate) trait Polled<T,E>{fn polled(self)->crate::retained_clone::RetainedWorkPoll<T,E>;}
impl<T,E> Polled<T,E> for crate::RetainedWorkOutcome<T,E>{fn polled(self)->crate::retained_clone::RetainedWorkPoll<T,E>{crate::retained_clone::RetainedWorkPoll::Complete(self)}}
impl<T,E> Polled<T,E> for crate::retained_clone::RetainedWorkPoll<T,E>{fn polled(self)->crate::retained_clone::RetainedWorkPoll<T,E>{self}}
/// 🚪️ Exits a close action early when a child stays pending and otherwise yields its settled turn.
pub(crate) trait Exit{fn exit(self)->Result<crate::OriginalRetirementOutcome,semio_framework_value_retirement::RetirementPollExit>;}
impl Exit for crate::OriginalRetirementOutcome{fn exit(self)->Result<crate::OriginalRetirementOutcome,semio_framework_value_retirement::RetirementPollExit>{Ok(self)}}
impl Exit for crate::retained_clone::RetainedWorkPoll<crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>{fn exit(self)->Result<crate::OriginalRetirementOutcome,semio_framework_value_retirement::RetirementPollExit>{semio_framework_value_retirement::RetirementPollExit::settled(self)}}
/// 💳️ Records on the original effect exactly the work this action owes beyond what its children already recorded.
pub(crate) fn record_owed(effect:&mut crate::RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError>{let done=effect.performed_work();if done>work{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed edit recorded more work than it reports"))}let owed=u64::try_from(work-done).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"indexed edit work does not fit one original unit"))?;if owed==0||effect.performed(owed)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"indexed edit work exceeded its original authority"))}}
/// 🧾️ Settles one action so the reported work equals the work its original effect recorded.
pub(crate) fn finish<T,E:From<ValueError>>(effect:&mut crate::RetainedAllocationEffect<'_>,work:u128,result:Result<T,E>)->crate::RetainedWorkOutcome<T,E>{let recorded=record_owed(effect,work);crate::RetainedWorkOutcome{result:match(result,recorded){(Ok(value),Ok(()))=>Ok(value),(Err(error),_)=>Err(error),(Ok(_),Err(error))=>Err(error.into())},work}}
/// 📦️ Allocates one empty vector through the original allocation marker, recording the real bytes even on refusal.
pub(crate) fn allocate_empty_vec<T>(count:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<Vec<T>,ValueError>{let actual=effect.actual();if actual.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed edit allocation attempt is occupied"))}if count==0||size_of::<T>()==0{return Ok(Vec::new())}let layout=std::alloc::Layout::array::<T>(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed edit vector layout overflow"))?;let pointer=unsafe{std::alloc::alloc(layout)}.cast::<T>();*actual=Some(if pointer.is_null(){0}else{layout.size()});if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"indexed edit vector backing allocation refused"))}Ok(unsafe{Vec::from_raw_parts(pointer,0,count)})}
impl IndexedEditPlan{
 pub fn new(shape:IndexedEditShape)->Self{Self{shape,orders:ManuallyDrop::new([None,None,None]),kind:IndexedEditKind::Removed,phase:Phase::Reserve,cursor:0,removal:0,sort:HeapSortCursor::new(0),fault:None,output_len:None,closing:false,retirement:Default::default(),close_route:None}}
 pub fn shape(&self)->IndexedEditShape{self.shape}
 pub fn settled(&self)->bool{self.phase==Phase::Settled}
 pub fn ready(&self)->bool{self.settled()&&self.fault.is_none()&&!self.closing}
 pub fn fault(&self)->Option<IndexedEditFault>{self.fault}
 pub fn output_len(&self)->Result<usize,ValueError>{if !self.ready(){return Err(invalid());}self.output_len.ok_or_else(invalid)}
 pub fn entries(&self,kind:IndexedEditKind)->Result<&[IndexedEditEntry],ValueError>{if !self.ready(){return Err(invalid());}Ok(self.orders[kind.index()].as_deref().unwrap_or(&[]))}
 fn count(&self)->usize{match self.kind{IndexedEditKind::Removed=>self.shape.removed,IndexedEditKind::Modified=>self.shape.modified,IndexedEditKind::Added=>self.shape.added}}
 fn fixed()->usize{size_of::<Self>()*2+size_of::<IndexedEditEntry>()*4}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.settled(){return Err(invalid());}
  let capacity_bytes=if self.phase==Phase::Reserve&&!(self.kind==IndexedEditKind::Removed&&self.shape.removed>self.shape.base_len){self.count().checked_mul(size_of::<IndexedEditEntry>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed coordinate directory extent overflow"))?}else{0};
  Ok(RetirementDemand{copy_bytes:Self::fixed(),capacity_bytes,depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the Work of the next action: one unit, plus one coordinate read while filling a directory.
 pub fn work_demand(&self)->u64{1+u64::from(self.phase==Phase::Fill&&self.cursor<self.count())}
 fn refuse(&mut self,p:RetainedCloneProgress)->IndexedEditStep{let fault=match self.kind{IndexedEditKind::Removed=>IndexedEditFault::Remove,IndexedEditKind::Modified=>IndexedEditFault::Modify,IndexedEditKind::Added=>IndexedEditFault::Add};self.fault=Some(fault);self.phase=Phase::Settled;IndexedEditStep::Refused(fault,p)}
 pub fn advance(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,coordinate:impl FnOnce(IndexedEditKind,usize)->crate::RetainedWorkOutcome<usize>)->crate::RetainedWorkOutcome<IndexedEditStep>{let grant=effect.grant();let mut work=0;let result=(||{
  let demand=self.demand()?;if !retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<self.work_demand(){return Ok(IndexedEditStep::Progress(Default::default()));}
  work=1;record_owed(effect,work)?;let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let kind=self.kind.index();
  match self.phase{
   Phase::Reserve=>{
    if self.kind==IndexedEditKind::Removed&&self.shape.removed>self.shape.base_len{return Ok(self.refuse(progress));}
    let order=crate::allocate_empty_vec::<IndexedEditEntry>(self.count(),effect)?;progress.retained_capacity_bytes=order.capacity()*size_of::<IndexedEditEntry>();self.orders[kind]=Some(order);self.phase=Phase::Fill;
   },
   Phase::Fill=>{
    if self.cursor<self.count(){let turn=coordinate(self.kind,self.cursor);work+=turn.work;record_owed(effect,work)?;let entry=IndexedEditEntry{index:turn.result?,ordinal:self.cursor};self.orders[kind].as_mut().ok_or_else(invalid)?.push(entry);self.cursor+=1;}
    else{self.sort=HeapSortCursor::new(self.count());self.phase=Phase::Sort;}
   },
   Phase::Sort=>{
    let order=self.orders[kind].as_mut().ok_or_else(invalid)?;match self.sort.next(){
     HeapSortWork::Complete=>{self.cursor=0;self.phase=Phase::Validate;},
     HeapSortWork::Compare{left,right}=>{let a=order[left];let b=order[right];self.sort.advance(Some((a.index,a.ordinal).cmp(&(b.index,b.ordinal)))).map_err(|_|invalid())?;},
     HeapSortWork::Swap{left,right}=>{order.swap(left,right);self.sort.advance(None).map_err(|_|invalid())?;},
     HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,
    }
   },
   Phase::Validate=>{
    if self.cursor==self.count(){
     if let Some(next)=self.kind.next(){self.kind=next;self.cursor=0;self.phase=Phase::Reserve;}
     else{self.output_len=Some(self.shape.base_len.checked_sub(self.shape.removed).and_then(|n|n.checked_add(self.shape.added)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed final cardinality overflow"))?);self.phase=Phase::Settled;return Ok(IndexedEditStep::Admitted(progress));}
    }else{
     let order=self.orders[kind].as_ref().ok_or_else(invalid)?;let row=order[self.cursor];let removed=self.orders[0].as_ref().ok_or_else(invalid)?;
     if self.kind==IndexedEditKind::Modified&&removed.get(self.removal).is_some_and(|r|r.index<row.index){self.removal+=1;return Ok(IndexedEditStep::Progress(progress));}
     let duplicate=self.cursor>0&&order[self.cursor-1].index==row.index;
     let bad=duplicate||match self.kind{IndexedEditKind::Removed=>row.index>=self.shape.base_len,IndexedEditKind::Modified=>row.index>=self.shape.base_len||removed.get(self.removal).is_some_and(|r|r.index==row.index),IndexedEditKind::Added=>row.index>self.shape.base_len-self.shape.removed+self.cursor};
     if bad{return Ok(self.refuse(progress));}self.cursor+=1;
    }
   },
   Phase::Settled=>return Err(invalid()),
  }
  Ok(IndexedEditStep::Progress(progress))
 })();crate::finish(effect,work,result)}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}self.retirement.next_owner_demands::<Vec<IndexedEditEntry>>(self.orders.iter().any(Option::is_some),&None,body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}self.retirement.next_owner_work::<Vec<IndexedEditEntry>>(self.orders.iter().any(Option::is_some),&None,body)}
 /// 📬️ Records one ordinary close directly into the caller's reply.
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply_plan(None,body,effect,reply,false)}
 /// 🔁️ Recovers the same ordinary close invocation.
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply_plan(None,body,effect,reply,true)}
 /// 🪢️ Lends the actual original loan to the same close invocation.
 pub fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_birth(){return Err(invalid())}if !self.requires_original_control(){return self.close_into(body,effect,reply)}self.close_reply_plan(Some(original),body,effect,reply,false)}
 /// 🔁️ Recovers the original-loan close invocation under its unchanged loan.
 pub fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.close_route.is_some_and(|(_,_,_,loan)|loan.is_none()){return self.close_reply_plan(None,body,effect,reply,true)}self.close_reply_plan(Some(original),body,effect,reply,true)}
 fn close_reply_plan<'r>(&mut self,mut original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if !self.closing{return Err(invalid())}
  let identity=invocation_identity(effect,recover)?;let body=if recover{self.close_route.map_or(body,|key|key.2)}else{body};let loan=match original.as_deref_mut(){Some(loan)=>Some(loan_key(loan,effect)?),None=>None};
  if recover{
   let key=self.close_route.ok_or_else(invalid)?;if key.0!=identity||key.3!=loan{return Err(ValueError::invariant("indexed edit close recovery changed its invocation, body or loan"))}
   let mut scope=reply.progress().scope(&mut self.close_route,key)?;let inner=scope.borrow()?;
   let done=self.retirement.close_reply(original,body,effect,inner,true)?;drop(done);return Ok(scope.forwarded());
  }
  if self.close_route.is_some(){return Err(ValueError::invariant("indexed edit close requires recovery"))}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();let demand=self.close_demand(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.retirement.is_empty(){
   let mut scope=reply.progress().scope(&mut self.close_route,(identity,1,body,loan))?;let inner=scope.borrow()?;
   let done=self.retirement.close_reply(original,body,effect,inner,false)?;drop(done);return Ok(scope.forwarded());
  }
  for order in self.orders.iter_mut(){match self.retirement.begin_granted(order,effect).result{Ok(Some(step))=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(step.progress())))),Ok(None)=>{},Err(error)=>return Ok(reply.complete(Err(error.into())))}}
  Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))
 }

 pub fn requires_original_birth(&self)->bool{self.retirement.requires_original_birth()}
 pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{crate::Settle::settled(self.retirement.step_original_birth(original,grant))}
 pub fn requires_original_control(&self)->bool{self.retirement.requires_original_control()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.orders.iter().all(Option::is_none)&&self.retirement.is_empty()}
}
#[cfg(feature="testkit")]
impl IndexedEditPlan{
 /// 🔬️ Admits a foreign owner into the plan's close custody so integration targets can drive a genuinely pending child close.
 pub fn adopt<T:semio_framework_value_retirement::RetireOwned>(&mut self,value:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{self.retirement.begin_granted(value,effect).result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}
 /// 🔬️ Observes the whole resumable admission state without exposing its fields.
 pub fn probe(&self)->String{format!("{:?}",(self.kind,self.phase,self.cursor,self.removal,self.sort))}
 /// 🔬️ Observes the exact custody of every paid coordinate directory.
 pub fn custody(&self)->Vec<Option<(usize,usize,usize)>>{self.orders.iter().map(|order|order.as_ref().map(|value|(value.as_ptr()as usize,value.len(),value.capacity()))).collect()}
}
impl CloseReply for IndexedEditPlan{fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply_plan(original,body,effect,reply,recover)}}
impl Drop for IndexedEditPlan{fn drop(&mut self){let empty=self.orders.iter().all(Option::is_none)&&self.retirement.is_empty()&&self.close_route.is_none();assert!(std::thread::panicking()||empty,"indexed edit directories require funded original closure");if empty{unsafe{ManuallyDrop::drop(&mut self.orders);}}}}

/// 🚚️ One original survivor/removal or authored addition is transferred per admitted output action.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedOutputWork{Original{index:usize},Removed{index:usize},Added{ordinal:usize},Complete}
/// 🧭️ Fixed output traversal never inserts, removes or shifts an existing collection.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct IndexedOutputCursor{base:usize,output:usize,removed:usize,added:usize}
impl IndexedOutputCursor{
 pub fn next(&self,plan:&IndexedEditPlan)->Result<IndexedOutputWork,ValueError>{
  let removed=plan.entries(IndexedEditKind::Removed)?;let added=plan.entries(IndexedEditKind::Added)?;
  if removed.get(self.removed).is_some_and(|entry|entry.index==self.base){return Ok(IndexedOutputWork::Removed{index:self.base});}
  if let Some(entry)=added.get(self.added).filter(|entry|entry.index==self.output){return Ok(IndexedOutputWork::Added{ordinal:entry.ordinal});}
  if self.base<plan.shape.base_len{return Ok(IndexedOutputWork::Original{index:self.base});}
  if self.output!=plan.output_len()?||self.added!=added.len(){return Err(invalid());}Ok(IndexedOutputWork::Complete)
 }
 pub fn advance(&mut self,plan:&IndexedEditPlan)->Result<(),ValueError>{match self.next(plan)?{IndexedOutputWork::Original{..}=>{self.base+=1;self.output+=1;},IndexedOutputWork::Removed{..}=>{self.base+=1;self.removed+=1;},IndexedOutputWork::Added{..}=>{self.output+=1;self.added+=1;},IndexedOutputWork::Complete=>return Err(invalid())}Ok(())}
}


