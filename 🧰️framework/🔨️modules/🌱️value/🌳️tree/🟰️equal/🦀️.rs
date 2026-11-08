//! 🟰️ Typed equality reads original fields directly and retains every funded traversal frontier.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,RetainedEqualityElement,retirement::RetireOwned};
use semio_framework_value_clone_source::{RetainedCloneRef,RetainedCloneBinding};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use std::mem::size_of;
use semio_framework_value_retirement::retirement_contract::RetirementPollExit;

/// 🌳️ Original sources stay stable; cursor construction allocates nothing, inspects no source and finishes within its declared fixed work.
pub trait RetainedEquality:Sized+RetireOwned+Send+Sync+'static{
 type Equality:RetainedEqualityCursor<Self>;
 const CURSOR_BIRTH_WORK:u64;
 const INLINE_PAIR_BYTES:Option<usize>;
 const INLINE_PAIR_WORK:Option<u64>;
 fn equality_cursor()->Self::Equality;
 fn inline_equal(left:&Self,right:&Self)->Option<bool>;
}
/// 🧾️ A ready result follows actual funded inspection; cancellation retains child-first original metadata custody.
pub struct RetainedEqualityStep{pub equal:Option<bool>,pub progress:RetainedCloneProgress}
/// 🪡️ Typed child cursors retain no borrowed pointers and close all owned frontier allocations explicitly.
pub trait RetainedEqualityCursor<T:?Sized>:Send+'static{
 fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 /// 📏️ Quotes the same demand once the caller has already verified a nonzero work quote, so nested frontiers are not re-gated at every depth.
 fn demand_admitted(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.demand(body,maximum_work_units)}
 fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn complete<T,E>(poll:crate::RetainedWorkPoll<T,E>)->Result<crate::RetainedWorkOutcome<T,E>,ValueError>{match poll{crate::RetainedWorkPoll::Complete(turn)=>Ok(turn),crate::RetainedWorkPoll::Pending{..}=>Err(invalid("typed equality child close stayed pending"))}}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}

/// 🔬️ Inline native equality never traverses a hidden referenced payload.
pub struct RetainedInlineEquality<T>{result:Option<bool>,marker:std::marker::PhantomData<fn()->T>}
impl<T> RetainedInlineEquality<T>{pub fn new()->Self{Self{result:None,marker:Default::default()}}}
impl<T:RetainedEqualityElement+Send+Sync+'static> RetainedEqualityCursor<T> for RetainedInlineEquality<T>{
 fn demand(&self,_:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.result.is_some(){return Err(invalid("typed inline comparison is settled"));}if T::WORK>maximum_work_units{return Ok(Default::default())}Ok(scalar(size_of::<T>()*2))}
 fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demand(body,maximum_work_units)?;Ok(if T::WORK<=maximum_work_units{T::WORK}else{0})}
 fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if effect.actual().is_some(){return Err(invalid("equality advance needs an empty allocation marker").into())}if self.result.is_some(){return Ok(RetainedEqualityStep{equal:self.result,progress:Default::default()})}if self.work_demand(body,maximum_work_units)?==0{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}let d=self.demand(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()});}self.result=Some(T::inline_eq(left,right));work=u128::from(T::WORK);Ok(RetainedEqualityStep{equal:self.result,progress:RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}})})();settle_work(effect,crate::RetainedWorkOutcome{result,work})}
 fn begin_close(&mut self){}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let _=effect.grant();crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
 fn terminal_is_empty(&self)->bool{true}
}
macro_rules! inline{($($ty:ty),*)=>{$(impl RetainedEquality for $ty{
 type Equality=RetainedInlineEquality<Self>;
 const CURSOR_BIRTH_WORK:u64=2;
 const INLINE_PAIR_BYTES:Option<usize>=Some(size_of::<Self>()*2);
 const INLINE_PAIR_WORK:Option<u64>=Some(<Self as RetainedEqualityElement>::WORK);
 fn equality_cursor()->Self::Equality{RetainedInlineEquality::new()}
 fn inline_equal(left:&Self,right:&Self)->Option<bool>{Some(<Self as RetainedEqualityElement>::inline_eq(left,right))}
 })*};}
inline!((),bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64);

/// 🔤️ Original string bytes compare through the existing bounded scalar buffer cursor.
pub struct RetainedStringEquality{cursor:crate::slice_equality::RetainedStagedSliceEquality}
impl RetainedEquality for String{
 type Equality=RetainedStringEquality;
 const CURSOR_BIRTH_WORK:u64=7;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedStringEquality{cursor:crate::slice_equality::RetainedStagedSliceEquality::new()}}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl RetainedEqualityCursor<String> for RetainedStringEquality{
 fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.cursor.demand::<u8>(body,maximum_work_units.saturating_sub(2))}
 fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{let child=self.cursor.work_demand::<u8>(body,maximum_work_units.saturating_sub(2))?;if child==0{return Ok(0)}child.checked_add(2).ok_or_else(||invalid("string equality projection work overflow"))}
 fn advance(&mut self,left:&String,right:&String,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();
  if effect.actual().is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}
  let selected=match self.work_demand(body,maximum_work_units){Ok(v)=>v,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}
  let d=match self.demand(body,maximum_work_units){Ok(v)=>v,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}
  let turn=self.cursor.advance(left.as_bytes(),right.as_bytes(),body,maximum_work_units-2,g);let equal=self.cursor.equal();settle_work(effect,crate::RetainedWorkOutcome{work:turn.work+2,result:turn.result.map(|step|RetainedEqualityStep{equal,progress:step.progress()}).map_err(Into::into)})
 }
 fn begin_close(&mut self){self.cursor.begin_close()}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demand()}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{self.cursor.close_work_demand()}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let turn=self.cursor.close_step(effect.maximum_work_units(),effect.grant());crate::RetainedWorkPoll::Complete(settle_work(effect,crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}))}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}

/// ⏱️ Records the work a turn reports but did not yet put on the original receipt, so every reported unit is performed exactly once.
pub(crate) fn settle_work<T,E:From<ValueError>>(effect:&mut crate::RetainedAllocationEffect<'_>,mut turn:crate::RetainedWorkOutcome<T,E>)->crate::RetainedWorkOutcome<T,E>{
 let owed=turn.work.saturating_sub(effect.performed_work());
 if owed>0{let refusal=match u64::try_from(owed).map_err(|_|ValueError::invariant("equality turn work overflow")).and_then(|owed|effect.performed(owed)){Ok(true)=>None,Ok(false)=>Some(ValueError::literal(ValueRefusalKind::WorkLimit,"equality turn exceeded its original work")),Err(error)=>Some(error)};if let Some(error)=refusal{if turn.result.is_ok(){turn.result=Err(E::from(error))}}}
 turn.work=effect.performed_work();turn
}
/// ⏱️ Settles a retirement poll against the original receipt; a pending poll reports only performed work.
pub(crate) fn settle_poll(effect:&mut crate::RetainedAllocationEffect<'_>,poll:crate::OriginalRetirementPoll)->crate::OriginalRetirementPoll{
 match poll{crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(settle_work(effect,turn)),crate::RetainedWorkPoll::Pending{work}=>{let turn=settle_work(effect,crate::RetainedWorkOutcome{result:Ok::<(),crate::OriginalRetirementFailure>(()),work});crate::RetainedWorkPoll::Pending{work:turn.work}}}
}
/// 🧳️ Genuine original source bindings outlive the whole typed comparison and every child cancellation.
pub struct RetainedEqualityOwner<T:RetainedEquality>{cursor:T::Equality,left:Option<RetainedCloneBinding>,right:Option<RetainedCloneBinding>,result:Option<bool>,closing:bool}
impl<T:RetainedEquality> RetainedEqualityOwner<T>{
 pub fn new()->Self{Self{cursor:T::equality_cursor(),left:None,right:None,result:None,closing:false}}
 /// ⛽️ Quotes the typed cursor and five direct owner fields.
 pub fn birth_work()->Result<u64,ValueError>{if T::CURSOR_BIRTH_WORK==0{return Err(invalid("typed equality cursor birth work absent"))}T::CURSOR_BIRTH_WORK.checked_add(5).ok_or_else(||invalid("typed equality owner birth work overflow"))}
 pub fn result(&self)->Option<bool>{self.result}
 /// 🧭️ Validates the actual source pair after decision without changing original custody.
 pub fn verify(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>)->Result<(),ValueError>{left.check_binding(&self.left)?;right.check_binding(&self.right)?;Ok(())}
 /// ✅️ Inverse preparation must retain the exact original right-side source.
 pub fn verify_original(&self,right:RetainedCloneRef<'_,T>)->Result<(),ValueError>{right.check_binding(&self.right)?;Ok(())}
 /// ⛽️ Quotes the source-owned original-binding check without inspecting the source.
 pub fn verify_original_work(&self)->u64{RetainedCloneRef::<T>::check_binding_work(&self.right)}
 /// 🪪️ Verifies the original inverse source under the actual caller grant.
 pub fn verify_original_paid(&self,right:RetainedCloneRef<'_,T>,work:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<bool>>{right.check_binding_paid(&self.right,work,g)}
 fn fixed()->usize{0}
 pub fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.result.is_some(){return Err(invalid("original typed comparison is closing or settled"))}if self.work_demand(body,maximum_work_units)?==0{return Ok(Default::default())}if self.left.is_none()||self.right.is_none(){return Ok(scalar(RetainedCloneBinding::alias_copy_bytes()))}let checks=RetainedCloneRef::<T>::check_binding_work(&self.left).checked_add(RetainedCloneRef::<T>::check_binding_work(&self.right)).and_then(|v|v.checked_add(1)).ok_or_else(||invalid("typed equality work overflow"))?;let mut d=self.cursor.demand(body.saturating_sub(Self::fixed()),maximum_work_units-checks)?;d.copy_bytes+=size_of::<Option<bool>>()*2+Self::fixed();d.depth+=1;Ok(d)}
 pub fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.closing||self.result.is_some(){return Err(invalid("original typed comparison is closing or settled"))}let check=RetainedCloneRef::<T>::check_binding_work(&self.left);let required=if self.left.is_none(){RetainedCloneBinding::alias_work_demand()}else if self.right.is_none(){check.checked_add(RetainedCloneBinding::alias_work_demand()).ok_or_else(||invalid("typed equality work overflow"))?}else{let checks=check.checked_add(RetainedCloneRef::<T>::check_binding_work(&self.right)).and_then(|v|v.checked_add(1)).ok_or_else(||invalid("typed equality work overflow"))?;let child=self.cursor.work_demand(body.saturating_sub(Self::fixed()),maximum_work_units.saturating_sub(checks))?;if child==0{return Ok(0)}child.checked_add(checks).ok_or_else(||invalid("typed equality work overflow"))?};Ok(if required<=maximum_work_units{required}else{0})}
 pub fn advance(&mut self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{
  if effect.actual().is_some(){return Err(invalid("equality advance needs an empty allocation marker").into())}if self.closing{return Err(invalid("original typed comparison is closing").into())}if self.result.is_some(){return Ok(RetainedEqualityStep{equal:self.result,progress:Default::default()})}if self.work_demand(body,maximum_work_units)?==0{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}let d=self.demand(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}
  if self.left.is_none(){let turn=left.bind(&mut self.left,effect);work=turn.work;return turn.result.map(|p|RetainedEqualityStep{equal:None,progress:p.unwrap_or_default()}).map_err(Into::into)}
  let checked=left.check_binding_paid(&self.left,maximum_work_units,g);work=checked.work;if checked.result?!=Some(true){return Err(invalid("typed equality left binding changed").into())}if self.right.is_none(){let turn=right.bind(&mut self.right,effect);work+=turn.work;return turn.result.map(|p|RetainedEqualityStep{equal:None,progress:p.unwrap_or_default()}).map_err(Into::into)}
  let checked=right.check_binding_paid(&self.right,maximum_work_units.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX)),g);work+=checked.work;if checked.result?!=Some(true){return Err(invalid("typed equality right binding changed").into())}let checks=RetainedCloneRef::<T>::check_binding_work(&self.left).checked_add(RetainedCloneRef::<T>::check_binding_work(&self.right)).and_then(|v|v.checked_add(1)).ok_or_else(||invalid("typed equality work overflow"))?;let extra=size_of::<Option<bool>>()*2+Self::fixed();let child_work=maximum_work_units.checked_sub(checks).ok_or_else(||invalid("typed equality child work underflows"))?;let turn=self.cursor.advance(left.get(),right.get(),body.saturating_sub(Self::fixed()),child_work,&mut effect.nested_within(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.checked_sub(extra).ok_or_else(||invalid("typed equality child copy budget underflows"))?,maximum_depth:g.maximum_depth.checked_sub(1).ok_or_else(||invalid("typed equality child depth underflows"))?,..g},child_work));work+=turn.work;let mut step=turn.result?;if let Some(equal)=step.equal{self.result=Some(equal);step.progress.copied_items=1;step.progress.copied_bytes+=extra;work+=1}Ok(step)
 })();settle_work(effect,crate::RetainedWorkOutcome{result,work})}

 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("original typed comparison close has not begun"));}if !self.cursor.terminal_is_empty(){return self.cursor.close_demand(body);}RetainedCloneBinding::close_demand(if self.right.is_some(){&self.right}else{&self.left},body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid("original typed comparison close has not begun"));}if !self.cursor.terminal_is_empty(){return self.cursor.close_work_demand(body);}RetainedCloneBinding::close_work_demand(if self.right.is_some(){&self.right}else{&self.left},body)}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let mut work:u128=0;let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.cursor.terminal_is_empty(){let turn=RetirementPollExit::settled(self.cursor.close_step(&mut effect.nested(g)))?;work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}
  let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(if self.right.is_some(){&mut self.right}else{&mut self.left},effect))?;work=turn.work;let step=turn.result?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 })();settle_poll(effect,RetirementPollExit::finish(result,work))}

 pub fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()&&self.left.is_none()&&self.right.is_none()}
}
impl<T:RetainedEquality> Drop for RetainedEqualityOwner<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.cursor.terminal_is_empty()&&self.left.is_none()&&self.right.is_none(),"original typed equality retains source/frontier custody");}}

#[path="📋️sequence/🦀️.rs"] pub mod sequence;
#[path="🎚️record/🦀️.rs"] pub mod record;
pub use record::{RetainedEqualityFields,RetainedRecordEquality};
/// 🏗️ A first-party birth descriptor constructs one fixed cursor without allocation, source access, traversal or panic within its stated work.
pub trait RetainedEqualityBirth{type Output;fn work()->Result<u64,ValueError>;fn birth()->Self::Output;}
/// ⛽️ Quotes descriptor work, fixed layout transfer and allocation before any constructor runs.
pub fn equality_box_work<B:RetainedEqualityBirth>()->Result<u64,ValueError>{let declared=B::work()?;if declared==0{return Err(invalid("equality birth work is invalid"))}let copied=size_of::<B::Output>().checked_mul(2).and_then(|v|u64::try_from(v).ok()).ok_or_else(||invalid("equality birth work overflow"))?;declared.checked_add(1).and_then(|v|v.checked_add(copied)).ok_or_else(||invalid("equality birth work overflow"))}
/// 🎟️ The actual allocation marker belongs to the original caller; no cursor is constructed before a successful admitted allocation.
pub fn equality_box<B:RetainedEqualityBirth>(maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<Option<(Box<B::Output>,RetainedCloneProgress)>>{
 if actual.is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality birth needs an empty allocation marker")),work:0}}
 let work=match equality_box_work::<B>(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};let layout=std::alloc::Layout::new::<B::Output>();let copied=match layout.size().checked_mul(2){Some(bytes)=>bytes,None=>return crate::RetainedWorkOutcome{result:Err(invalid("equality birth copy overflow")),work:0}};let d=RetirementDemand{copy_bytes:copied,capacity_bytes:layout.size(),depth:1,..Default::default()};
 if maximum_work_units<work||!retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
 if layout.size()==0{return crate::RetainedWorkOutcome{result:Ok(Some((Box::new(B::birth()),RetainedCloneProgress{copied_items:1,..Default::default()}))),work:u128::from(work)}}
 *actual=Some(0);let pointer=unsafe{std::alloc::alloc(layout)}as*mut B::Output;if pointer.is_null(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"typed equality cursor allocation refused")),work:1}}
 *actual=Some(layout.size());unsafe{pointer.write(B::birth());crate::RetainedWorkOutcome{result:Ok(Some((Box::from_raw(pointer),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))),work:u128::from(work)}}
}

#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;

#[path="🤝️tuple/🦀️.rs"] mod tuple;

#[cfg(test)]
#[path="🧪️tests/actual.rs"]
mod actual_work_tests;
