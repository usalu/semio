//! 📦️ Paid type erasure retains the complete original domain owner in the existing controlled frontier.
use super::{OriginalTurnFailure,OriginalTurnPrimary,OriginalFailureContext,ValueError};
use semio_framework_value_retained_clone::{RetirementDemand,RetainedWorkOutcome,ValueRefusalKind,retained_clone::{RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement}};
pub use semio_framework_dsl_record::native_encoding::{FieldProjectionSource as OriginalDomainProjection,FieldProjectionView as OriginalDomainView};

/// 🧬️ Domain implementations lend their complete original fields and actual failed effects.
pub trait OriginalDomainFailure:RetireOwned+Sync+OriginalDomainProjection+std::fmt::Debug{
 fn domain_code(&self)->&str;
 fn domain_message(&self)->&str;
 fn domain_retained_progress(&self)->RetainedCloneProgress;
}
trait DomainOwner:Send+Sync{
 fn original(&self)->&dyn OriginalDomainProjection;
 fn code(&self)->&str;
 fn message(&self)->&str;
 fn progress(&self)->RetainedCloneProgress;
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>;
}
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
struct DomainPayload<T:OriginalDomainFailure>{original:Option<T>,callback_refusal:Option<OriginalTurnFailure>,publication_refusal:Option<OriginalTurnFailure>}
impl<T:OriginalDomainFailure> DomainPayload<T>{
 fn expanded(&self)->bool{self.callback_refusal.is_some()||self.publication_refusal.is_some()}
 fn count(&self)->usize{usize::from(self.original.is_some())+usize::from(self.callback_refusal.is_some())+usize::from(self.publication_refusal.is_some())}
 fn field(&self,index:usize)->Result<&str,ValueError>{[self.original.as_ref().map(|_|"original"),self.callback_refusal.as_ref().map(|_|"callbackRefusal"),self.publication_refusal.as_ref().map(|_|"publicationRefusal")].into_iter().flatten().nth(index).ok_or_else(projection_refused)}
 fn pending(&self)->bool{self.count()!=0}
}
fn projection_refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original domain reservation projection ordinal changed")}
impl<T:OriginalDomainFailure> OriginalDomainProjection for DomainPayload<T>{
 fn projection_identity(&self)->usize{if !self.expanded(){self.original.as_ref().map_or(self as*const Self as usize,OriginalDomainProjection::projection_identity)}else{self as*const Self as usize}}
 fn projection_view(&self,path:&[usize])->Result<OriginalDomainView<'_>,ValueError>{
  if !self.expanded(){return self.original.as_ref().ok_or_else(projection_refused)?.projection_view(path)}
  let Some((&field,rest))=path.split_first()else{return Ok(OriginalDomainView::IntrinsicObject(self.count()))};
  match self.field(field)?{"original"=>self.original.as_ref().ok_or_else(projection_refused)?.projection_view(rest),"callbackRefusal"=>super::project_original_turn_failure(self.callback_refusal.as_ref().ok_or_else(projection_refused)?,rest),"publicationRefusal"=>super::project_original_turn_failure(self.publication_refusal.as_ref().ok_or_else(projection_refused)?,rest),_=>Err(projection_refused())}
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if !self.expanded(){return self.original.as_ref().ok_or_else(projection_refused)?.projection_key(path,index)}
  let Some((&field,rest))=path.split_first()else{return self.field(index)};
  match self.field(field)?{"original"=>self.original.as_ref().ok_or_else(projection_refused)?.projection_key(rest,index),"callbackRefusal"=>super::key_original_turn_failure(self.callback_refusal.as_ref().ok_or_else(projection_refused)?,rest,index),"publicationRefusal"=>super::key_original_turn_failure(self.publication_refusal.as_ref().ok_or_else(projection_refused)?,rest,index),_=>Err(projection_refused())}
 }
}
struct DomainFrame<T:OriginalDomainFailure>{owner:ControlledRetirement<DomainPayload<T>>}
/// 🔒️ Shared access reads only immutable originals; retirement cursor mutation requires exclusive access.
unsafe impl<T:OriginalDomainFailure> Sync for DomainFrame<T>{}
impl<T:OriginalDomainFailure> DomainOwner for DomainFrame<T>{
 fn original(&self)->&dyn OriginalDomainProjection{self.owner.original().expect("original domain projection precedes retirement")}
 fn code(&self)->&str{let payload=self.owner.original().expect("original domain code precedes retirement");payload.original.as_ref().map_or("original.callback",OriginalDomainFailure::domain_code)}
 fn message(&self)->&str{let payload=self.owner.original().expect("original domain message precedes retirement");if let Some(original)=&payload.original{original.domain_message()}else{payload.callback_refusal.as_ref().or(payload.publication_refusal.as_ref()).expect("original domain retains a cause").primary.message()}}
 fn progress(&self)->RetainedCloneProgress{self.owner.original().and_then(|payload|payload.original.as_ref()).map_or(Default::default(),OriginalDomainFailure::domain_retained_progress)}
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>{self}
}
impl<T:OriginalDomainFailure> RetirementCursor for DomainFrame<T>{
 fn close_step(&mut self,grant:semio_framework_value_retained_clone::RetainedCloneGrant)->RetirementStep{let result=self.owner.step(grant);let work=self.owner.last_turn_work();match result{Ok(RetainedCloneStep::Complete(progress)|RetainedCloneStep::Progress(progress))=>RetirementStep::Progress(progress,work),Err(error)=>RetirementStep::Failure(error,work)}}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.owner.selected_demand(body)?.copy_bytes)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🫴️ The erased header moves without cloning the admitted typed source or its nested payloads.
pub struct OriginalDomainPrimary{owner:Box<dyn DomainOwner>}
impl std::fmt::Debug for OriginalDomainPrimary{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalDomainPrimary").field("code",&self.code()).finish_non_exhaustive()}}
impl OriginalDomainPrimary{
 pub fn projection(&self)->&dyn OriginalDomainProjection{self.owner.original()}
 pub fn code(&self)->&str{self.owner.code()}
 pub fn message(&self)->&str{self.owner.message()}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.owner.progress()}
}
impl RetireOwned for OriginalDomainPrimary{
 fn retirement(self)->Box<dyn RetirementCursor>{self.owner.into_retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(0)}
 fn controlled_retirement_supported()->bool{true}
}

/// 📏️ Quotes the complete physical frame before any producer body can create a failure.
pub const fn original_domain_failure_demand<T:OriginalDomainFailure>()->RetirementDemand{RetirementDemand{copy_bytes:0,capacity_bytes:size_of::<DomainFrame<T>>(),release_bytes:0,depth:1}}

/// 🎟️ Prepaid typed custody keeps body and callback causes under their initiating Original source.
pub struct OriginalDomainReservation<T:OriginalDomainFailure>{frame:Option<Box<DomainFrame<T>>>,source:Option<(usize,Option<u64>)>,closing:bool}
impl<T:OriginalDomainFailure> Default for OriginalDomainReservation<T>{fn default()->Self{Self::new()}}
impl<T:OriginalDomainFailure> OriginalDomainReservation<T>{
 pub fn new()->Self{Self{frame:None,source:None,closing:false}}
 fn identity(cx:&impl OriginalFailureContext)->Result<(usize,Option<u64>),OriginalTurnFailure>{cx.original_failure_identity().map_err(Into::into)}
 fn matches(&self,cx:&impl OriginalFailureContext)->Result<bool,OriginalTurnFailure>{Ok(self.source==Some(Self::identity(cx)?))}
 /// 🚦️ Observes whether paid retirement has selected this resident frame.
 pub fn is_closing(&self)->bool{self.closing}
 pub fn ready(&self)->bool{!self.closing&&self.frame.as_ref().is_some_and(|frame|frame.owner.original().is_some())}
 /// 🆔️ Observes the actual paid frame backing without issuing ownership or authority.
 pub fn allocation_identity(&self)->Option<usize>{self.frame.as_ref().map(|frame|&**frame as*const DomainFrame<T>as usize)}
 /// 🧬️ Borrows the complete resident typed failure without moving its publication owner.
 pub fn pending_original(&self)->Option<&T>{self.frame.as_ref()?.owner.original()?.original.as_ref()}
 /// 🔎️ Borrows the complete still-resident cause tree before publication.
 pub fn pending_projection(&self)->Option<&dyn OriginalDomainProjection>{if !self.pending(){return None}self.frame.as_ref()?.owner.original().map(|payload|payload as&dyn OriginalDomainProjection)}
 pub fn pending(&self)->bool{self.frame.as_ref().and_then(|frame|frame.owner.original()).is_some_and(DomainPayload::pending)}
 pub fn terminal_is_empty(&self)->bool{self.frame.is_none()}
 pub fn prepare(&mut self,cx:&mut impl OriginalFailureContext)->Result<bool,OriginalTurnFailure>{
  let identity=Self::identity(cx)?;if self.closing||cx.is_cancelled(){return Ok(false)}if self.frame.is_some(){return Ok(self.source==Some(identity))}
  if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original domain requires its controlled producer").into())}
  let d=original_domain_failure_demand::<T>();if cx.should_yield()||!cx.retained_work_can_enter(d){return Ok(false)}
  let layout=std::alloc::Layout::new::<DomainFrame<T>>();let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<DomainFrame<T>>());cx.consume_fuel(1);
  let Some(pointer)=pointer else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original domain frame allocation refused").into())};
  let frame=DomainFrame{owner:ControlledRetirement::retain(DomainPayload{original:None,callback_refusal:None,publication_refusal:None})};
  self.frame=Some(unsafe{pointer.as_ptr().write(frame);Box::from_raw(pointer.as_ptr())});self.source=Some(identity);
  cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})?;Ok(true)
 }
 pub fn with_body<R:Copy,C:OriginalFailureContext>(&mut self,cx:&mut C,run:impl FnOnce(&mut C,&mut Option<T>)->Result<R,OriginalTurnFailure>)->Result<Option<R>,OriginalTurnFailure>{
  if !self.ready()||self.pending()||!self.matches(cx)?||cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}
  let total=cx.original_total_retained_grant();let fuel=cx.original_total_fuel_remaining();if total.maximum_items<=1||fuel<=1{return Ok(None)}
  let mut grant=cx.retained_grant();grant.maximum_items=grant.maximum_items.min(total.maximum_items-1);
  let issued=cx.fuel_remaining().min(fuel-1);let deadline=cx.original_absolute_deadline_us();let payload=self.frame.as_mut().ok_or_else(||ValueError::invariant("ready domain frame"))?.owner.original_mut().ok_or_else(||ValueError::invariant("unretired domain payload"))?;
  let result=cx.with_retained_loan(grant,|same|same.with_original_work_loan(issued,deadline,|body|run(body,&mut payload.original)));
  match result{Ok(value)=>Ok(Some(value)),Err(failure)=>{payload.callback_refusal=Some(failure);Ok(None)}}
 }
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let Some(frame)=&self.frame else{return Ok(Default::default())};if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}if frame.owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<DomainFrame<T>>(),depth:1,..Default::default()})}else{frame.owner.next_demand(body)}}
 pub fn close_step(&mut self,grant:semio_framework_value_retained_clone::RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(frame)=self.frame.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if !self.closing{if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.closing=true;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  if !frame.owner.terminal_is_empty(){let result=frame.owner.step(grant);return RetainedWorkOutcome{result,work:frame.owner.last_turn_work()}}
  let d=RetirementDemand{release_bytes:size_of::<DomainFrame<T>>(),depth:1,..Default::default()};if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  drop(self.frame.take());self.source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}
 }
}

struct DomainReservationRetirement<T:OriginalDomainFailure>{reservation:OriginalDomainReservation<T>}
impl<T:OriginalDomainFailure> RetirementCursor for DomainReservationRetirement<T>{
 fn close_step(&mut self,grant:semio_framework_value_retained_clone::RetainedCloneGrant)->RetirementStep{let turn=self.reservation.close_step(grant);match turn.result{Ok(step)=>if step.progress()==Default::default()&&self.terminal_is_empty(){RetirementStep::Complete(turn.work)}else{RetirementStep::Progress(step.progress(),turn.work)},Err(error)=>RetirementStep::Failure(error,turn.work)}}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.reservation.retirement_demands(body)}
 fn terminal_is_empty(&self)->bool{self.reservation.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.reservation.retirement_demands(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.reservation.retirement_demands(body)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.reservation.retirement_demands(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.reservation.retirement_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.reservation.retirement_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:OriginalDomainFailure> RetireOwned for OriginalDomainReservation<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(DomainReservationRetirement{reservation:self})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<DomainReservationRetirement<T>>())}
 fn controlled_retirement_supported()->bool{true}
}

/// 📤️ Moves one prepaid frame without allocating, cloning or reposting its already received body effects.
pub fn admit_original_domain_failure<T:OriginalDomainFailure>(reservation:&mut OriginalDomainReservation<T>,cx:&mut impl OriginalFailureContext)->Result<Option<OriginalTurnFailure>,OriginalTurnFailure>{
 if !reservation.ready()||!reservation.pending()||cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(RetirementDemand{depth:1,..Default::default()}){return Ok(None)}
 if !reservation.matches(cx)?{
  let payload=reservation.frame.as_mut().ok_or_else(||ValueError::invariant("pending domain frame"))?.owner.original_mut().ok_or_else(||ValueError::invariant("pending domain payload"))?;if payload.publication_refusal.is_none(){
   payload.publication_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"original domain reservation requires its initiating source").into());let progress=RetainedCloneProgress{copied_items:1,..Default::default()};let receipt=cx.consume_retained(progress).err();cx.consume_fuel(1);let context=cx.original_failure_identity().err();let failure=payload.publication_refusal.as_mut().ok_or_else(||ValueError::invariant("recorded original source refusal"))?;failure.effects=progress;failure.receipt_refusal=receipt;failure.context_refusal=context;
  }return Ok(None)
 }
 let owner=reservation.frame.take().ok_or_else(||ValueError::invariant("pending prepaid domain frame"))?;reservation.source=None;cx.consume_fuel(1);
 let mut failure=OriginalTurnFailure::from(OriginalTurnPrimary::Domain(OriginalDomainPrimary{owner}));
 failure.receipt_refusal=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).err();Ok(Some(failure))
}
#[path="🤝️closure/🦀️.rs"]
mod closure;
pub use closure::OriginalClosureFailure;
