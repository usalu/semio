//! 🟰️ Borrowed scalar buffers compare only funded pages and retain no original payload.
use crate::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,RETAINED_CLONE_BULK_PAGE_BYTES};
use std::mem::size_of;

/// 🔬️ The inline representation declares bounded native comparison work.
///
/// # Safety
/// Implementations must perform at most `WORK` units without allocation, callbacks, traversal or unwinding, and `WORK` must be nonzero.
pub unsafe trait RetainedEqualityElement:Copy{const WORK:u64;fn inline_eq(left:&Self,right:&Self)->bool;}
macro_rules! scalar{($($ty:ty),*)=>{$(unsafe impl RetainedEqualityElement for $ty{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}})*}}
scalar!(bool,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,char,());

unsafe impl RetainedEqualityElement for f32{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}}
unsafe impl RetainedEqualityElement for f64{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}}

/// 📖️ The caller retains immutable original slices and their revision; the cursor owns only scalar metadata.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedSliceEquality{offset:usize,result:Option<bool>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"slice equality lost its original immutable bounds or live phase")}
fn with_progress(error:ValueError,progress:RetainedCloneProgress)->ValueError{let original=error.retained_progress();error.with_retained_progress(original.saturating_add(progress))}
impl RetainedSliceEquality{
 pub fn new()->Self{Self::default()}
 pub fn ready(&self)->bool{self.result.is_some()}
 pub fn equal(&self)->Option<bool>{self.result}
 pub fn fixed_copy_bytes()->usize{size_of::<usize>()+size_of::<Option<bool>>()}
 fn count<T:RetainedEqualityElement>(&self,left:usize,right:usize,body:usize,maximum_work_units:u64)->Result<usize,ValueError>{
  if self.ready(){return Err(invalid());}if left!=right{return Ok(0);}let remaining=left.checked_sub(self.offset).ok_or_else(invalid)?;
  if T::WORK==0{return Err(invalid())}let work=usize::try_from(maximum_work_units.saturating_sub(1)/T::WORK).unwrap_or(usize::MAX);
  if size_of::<T>()==0{return Ok(remaining.min(1).min(work));}
  let width=size_of::<T>().checked_mul(2).ok_or_else(invalid)?;
  Ok(remaining.min((RETAINED_CLONE_BULK_PAGE_BYTES/width).max(1)).min(body/width).min(work))
 }
 pub fn demand<T:RetainedEqualityElement>(&self,left:&[T],right:&[T],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  let count=self.count::<T>(left.len(),right.len(),body,maximum_work_units)?;
  if maximum_work_units==0||(left.len()==right.len()&&self.offset<left.len()&&count==0){return Ok(Default::default())}
  let bytes=count.checked_mul(size_of::<T>()).and_then(|bytes|bytes.checked_mul(2)).and_then(|bytes|bytes.checked_add(Self::fixed_copy_bytes())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"slice equality page extent overflow"))?;
  Ok(RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the length check and each selected element's declared fixed work.
 pub fn work_demand<T:RetainedEqualityElement>(&self,left:&[T],right:&[T],body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demand(left,right,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}
  if left.len()!=right.len()||self.offset==left.len(){return Ok(1)}
  let count=self.count::<T>(left.len(),right.len(),body,maximum_work_units)?;if count==0{return Ok(0)}(count as u64).checked_mul(T::WORK).and_then(|work|work.checked_add(1)).ok_or_else(invalid)
 }
 /// 📥️ Compares only the admitted native page and records its original reply.
 pub fn advance_into<'r,T:RetainedEqualityElement>(&mut self,left:&[T],right:&[T],body:usize,maximum_work_units:u64,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{
  if self.result.is_some(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let maximum=maximum_work_units;let work=match self.work_demand(left,right,body,maximum){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};let demand=match self.demand(left,right,body,maximum){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};if work==0||remaining<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let count=match self.count::<T>(left.len(),right.len(),body,maximum){Ok(count)=>count,Err(error)=>return Ok(reply.complete(Err(error)))};
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice equality metadata exceeded original Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};if left.len()!=right.len(){self.result=Some(false);progress.copied_bytes+=size_of::<Option<bool>>();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))}
  let end=self.offset+count;for index in self.offset..end{match effect.performed(T::WORK){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(with_progress(ValueError::literal(ValueRefusalKind::WorkLimit,"slice equality element exceeded original Work"),progress)))),Err(error)=>return Ok(reply.complete(Err(with_progress(error,progress))))}progress.copied_bytes+=size_of::<T>()*2;if !T::inline_eq(&left[index],&right[index]){self.result=Some(false);progress.copied_bytes+=size_of::<Option<bool>>();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))}}
  self.offset=end;progress.copied_bytes+=size_of::<usize>();let step=if end==left.len(){self.result=Some(true);progress.copied_bytes+=size_of::<Option<bool>>();RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)};Ok(reply.complete(Ok(step)))
 }

}
/// 🔎️ Paid slice header custody requires the caller to preserve its genuine original source bindings.
#[derive(Debug)]
struct SliceHeader{left:(usize,usize),right:(usize,usize)}
/// 🧳️ The staged inspection retains no payload and closes its scalar header under the original caller grant.
pub struct RetainedSliceInspection{header:Option<SliceHeader>,closing:bool}
impl RetainedSliceInspection{
 pub fn new()->Self{Self{header:None,closing:false}}
 pub fn ready(&self)->bool{self.header.is_some()&&!self.closing}
 /// 👁️ Observes the original paid close-start state.
 pub fn close_started(&self)->bool{self.closing}
 pub fn is_empty(&self)->bool{self.header.is_none()}
 pub fn lengths(&self)->Result<(usize,usize),ValueError>{if !self.ready(){return Err(invalid())}let header=self.header.as_ref().ok_or_else(invalid)?;Ok((header.left.1,header.right.1))}
 pub fn inspect_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.header.is_some(){return Err(invalid())}Ok(RetirementDemand{copy_bytes:size_of::<Option<SliceHeader>>(),depth:1,..Default::default()})}
 pub fn inspect<T>(&mut self,left:&[T],right:&[T],effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_work();let result=(||{let d=self.inspect_demand()?;if effect.remaining_work_units()?<3||!retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(3)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice inspection exceeded original Work"))}self.header=Some(SliceHeader{left:(left.as_ptr()as usize,left.len()),right:(right.as_ptr()as usize,right.len())});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}
 }
 pub fn verify<T>(&self,left:&[T],right:&[T],effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<bool>>{
  let before=effect.performed_work();let result=(||{let header=match self.header.as_ref(){Some(header)if !self.closing=>header,_=>return Err(invalid())};if effect.remaining_work_units()?<3{return Ok(None)}if !effect.performed(3)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice verification exceeded original Work"))}Ok(Some(header.left==(left.as_ptr()as usize,left.len())&&header.right==(right.as_ptr()as usize,right.len())))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}
 }
 pub fn begin_close(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let before=effect.performed_work();let result=(||{if self.closing{return Ok(RetainedCloneStep::Complete(Default::default()))}let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};if effect.remaining_work_units()?==0||!retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice close start exceeded original Work"))}self.closing=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.header.is_none()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}Ok(if self.header.is_some(){RetirementDemand{copy_bytes:size_of::<Option<SliceHeader>>(),depth:1,..Default::default()}}else{Default::default()})}
 pub fn close_step(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let before=effect.performed_work();let result=(||{let d=self.close_demand()?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if effect.remaining_work_units()?==0||!retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice header close exceeded original Work"))}self.header=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}}

}
impl Drop for RetainedSliceInspection{fn drop(&mut self){assert!(std::thread::panicking()||self.header.is_none(),"staged slice header requires admitted closure")}}
/// 🪜️ Staged scalar comparison holds only the paid header; its caller owns the genuine immutable source bindings.
pub struct RetainedStagedSliceEquality{cursor:RetainedSliceEquality,inspection:RetainedSliceInspection}
impl RetainedStagedSliceEquality{
 pub fn new()->Self{Self{cursor:RetainedSliceEquality::new(),inspection:RetainedSliceInspection::new()}}
 pub fn ready(&self)->bool{self.cursor.ready()}
 pub fn equal(&self)->Option<bool>{self.cursor.equal()}
 fn selection<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<(RetirementDemand,u64,usize),ValueError>{
  let header=self.inspection.header.as_ref().ok_or_else(invalid)?;if self.cursor.ready(){return Err(invalid())}if work<5{return Ok((Default::default(),0,0))}
  let fixed=RetainedSliceEquality::fixed_copy_bytes();let left=header.left.1;let right=header.right.1;
  if left!=right||self.cursor.offset==left{return Ok((RetirementDemand{copy_bytes:fixed,depth:1,..Default::default()},5,0))}
  let n=self.cursor.count::<T>(left,right,body,work-4)?;if n==0{return Ok((Default::default(),0,0))}
  let bytes=n.checked_mul(size_of::<T>()).and_then(|x|x.checked_mul(2)).and_then(|x|x.checked_add(fixed)).ok_or_else(invalid)?;let selected=(n as u64).checked_mul(T::WORK).and_then(|x|x.checked_add(5)).ok_or_else(invalid)?;
  Ok((RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()},selected,n))
 }
 pub fn demand<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if !self.inspection.ready(){return self.inspection.inspect_demand()}Ok(self.selection::<T>(body,work)?.0)}
 pub fn work_demand<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<u64,ValueError>{if !self.inspection.ready(){self.inspection.inspect_demand()?;return Ok(if work>=3{3}else{0})}Ok(self.selection::<T>(body,work)?.1)}
 pub fn advance<T:RetainedEqualityElement>(&mut self,left:&[T],right:&[T],body:usize,maximum_work_units:u64,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  if !self.inspection.ready(){return self.inspection.inspect(left,right,effect)}let before=effect.performed_work();let result=(||{let(d,selected,n)=self.selection::<T>(body,maximum_work_units)?;if selected==0||effect.remaining_work_units()?<selected||!retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}match self.inspection.verify(left,right,effect).result?{Some(true)=>{},_=>return Err(invalid())}if !effect.performed(2)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice equality metadata exceeded original Work"))}let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};let header=self.inspection.header.as_ref().ok_or_else(invalid)?;if header.left.1!=header.right.1{self.cursor.result=Some(false);p.copied_bytes+=size_of::<Option<bool>>();return Ok(RetainedCloneStep::Complete(p))}let end=self.cursor.offset+n;for i in self.cursor.offset..end{match effect.performed(T::WORK){Ok(true)=>{},Ok(false)=>return Err(with_progress(ValueError::literal(ValueRefusalKind::WorkLimit,"slice equality element exceeded original Work"),p)),Err(error)=>return Err(with_progress(error,p))}p.copied_bytes+=size_of::<T>()*2;if !T::inline_eq(&left[i],&right[i]){self.cursor.result=Some(false);p.copied_bytes+=size_of::<Option<bool>>();return Ok(RetainedCloneStep::Complete(p))}}self.cursor.offset=end;p.copied_bytes+=size_of::<usize>();if end==header.left.1{self.cursor.result=Some(true);p.copied_bytes+=size_of::<Option<bool>>()}Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}
 }
 pub fn close_started(&self)->bool{self.inspection.closing}
 pub fn begin_close_demand(&self)->RetirementDemand{if self.inspection.closing{Default::default()}else{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.inspection.closing)}
 pub fn begin_close(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{self.inspection.begin_close(effect)}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{let d=self.inspection.close_demand()?;if !self.inspection.terminal_is_empty(){return Ok(d)}if self.cursor==RetainedSliceEquality::new(){return Ok(Default::default())}Ok(RetirementDemand{copy_bytes:size_of::<RetainedSliceEquality>(),depth:1,..Default::default()})}
 pub fn close_work_demand(&self)->Result<u64,ValueError>{self.close_demand()?;Ok(if self.terminal_is_empty(){0}else{1})}
 pub fn close_step(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{if !self.inspection.terminal_is_empty(){let turn=self.inspection.close_step(effect);let result=turn.result.map(|step|if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});return crate::RetainedWorkOutcome{result,work:turn.work}}let before=effect.performed_work();let result=(||{let d=self.close_demand()?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if effect.remaining_work_units()?==0||!retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"slice equality close exceeded original Work"))}self.cursor=RetainedSliceEquality::new();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}}
 pub fn terminal_is_empty(&self)->bool{self.inspection.header.is_none()&&self.cursor==RetainedSliceEquality::new()}
}
impl Drop for RetainedStagedSliceEquality{fn drop(&mut self){assert!(std::thread::panicking()||(self.inspection.header.is_none()&&self.cursor==RetainedSliceEquality::new()),"staged slice metadata requires admitted closure")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
