//! 🟰️ Borrowed scalar buffers compare only funded pages and retain no original payload.
use crate::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,RETAINED_CLONE_BULK_PAGE_BYTES};
use std::mem::size_of;

/// 🔬️ The implementing first-party inline representation declares fixed work and cannot traverse, allocate or panic.
pub trait RetainedEqualityElement:Copy{const WORK:u64;fn inline_eq(left:&Self,right:&Self)->bool;}
macro_rules! scalar{($($ty:ty),*)=>{$(impl RetainedEqualityElement for $ty{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}})*}}
scalar!(bool,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,char,());

impl RetainedEqualityElement for f32{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}}
impl RetainedEqualityElement for f64{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}}

/// 📖️ The caller retains immutable original slices and their revision; the cursor owns only scalar metadata.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedSliceEquality{offset:usize,result:Option<bool>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"slice equality lost its original immutable bounds or live phase")}
impl RetainedSliceEquality{
 pub fn new()->Self{Self::default()}
 pub fn ready(&self)->bool{self.result.is_some()}
 pub fn equal(&self)->Option<bool>{self.result}
 pub fn fixed_copy_bytes()->usize{0}
 fn count<T:RetainedEqualityElement>(&self,left:usize,right:usize,body:usize,maximum_work_units:u64)->Result<usize,ValueError>{
  if self.ready(){return Err(invalid());}if left!=right{return Ok(0);}let remaining=left.checked_sub(self.offset).ok_or_else(invalid)?;
  if T::WORK==0{return Err(invalid())}let work=usize::try_from(maximum_work_units.saturating_sub(1)/T::WORK).unwrap_or(usize::MAX);
  if size_of::<T>()==0{return Ok(remaining.min(1).min(work));}
  let width=size_of::<T>().checked_mul(2).ok_or_else(invalid)?;
  Ok(remaining.min((RETAINED_CLONE_BULK_PAGE_BYTES/width).max(1)).min(body.saturating_sub(Self::fixed_copy_bytes())/width).min(work))
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
 pub fn advance<T:RetainedEqualityElement>(&mut self,left:&[T],right:&[T],body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work:u128=0;let result=(||{
  if self.result.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if self.work_demand(left,right,body,maximum_work_units)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let demand=self.demand(left,right,body,maximum_work_units)?;
  if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if left.len()!=right.len(){self.result=Some(false);work=1;return Ok(RetainedCloneStep::Complete(progress))}
  let end=self.offset+self.count::<T>(left.len(),right.len(),body,maximum_work_units)?;
  work=1;for index in self.offset..end{work=work.checked_add(u128::from(T::WORK)).ok_or_else(invalid)?;progress.copied_bytes+=size_of::<T>()*2;if !T::inline_eq(&left[index],&right[index]){self.result=Some(false);return Ok(RetainedCloneStep::Complete(progress))}}
  self.offset=end;if end==left.len(){self.result=Some(true);Ok(RetainedCloneStep::Complete(progress))}else{Ok(RetainedCloneStep::Progress(progress))}
 })();crate::RetainedWorkOutcome{result,work}}

}
/// 🔎️ Paid slice header custody requires the caller to preserve its genuine original source bindings.
#[derive(Debug)]
struct SliceHeader{left:(usize,usize),right:(usize,usize)}
/// 🧳️ The staged inspection retains no payload and closes its scalar header under the original caller grant.
pub(crate) struct RetainedSliceInspection{header:Option<SliceHeader>,closing:bool}
impl RetainedSliceInspection{
 pub fn new()->Self{Self{header:None,closing:false}}
 pub fn ready(&self)->bool{self.header.is_some()&&!self.closing}
 pub(crate) fn is_empty(&self)->bool{self.header.is_none()}
 pub(crate) fn lengths(&self)->Result<(usize,usize),ValueError>{if !self.ready(){return Err(invalid())}let header=self.header.as_ref().ok_or_else(invalid)?;Ok((header.left.1,header.right.1))}
 pub fn inspect_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.header.is_some(){return Err(invalid())}Ok(RetirementDemand{copy_bytes:size_of::<Option<SliceHeader>>()*2,depth:1,..Default::default()})}
 pub fn inspect<T>(&mut self,left:&[T],right:&[T],maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  let d=match self.inspect_demand(){Ok(d)=>d,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e),work:0}};
  if maximum_work_units<3||!retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.header=Some(SliceHeader{left:(left.as_ptr()as usize,left.len()),right:(right.as_ptr()as usize,right.len())});
  crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:3}
 }
 pub fn verify<T>(&self,left:&[T],right:&[T],maximum_work_units:u64)->crate::RetainedWorkOutcome<Option<bool>>{
  let header=match self.header.as_ref(){Some(header)if !self.closing=>header,_=>return crate::RetainedWorkOutcome{result:Err(invalid()),work:0}};
  if maximum_work_units<3{return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
  let left_same=header.left==(left.as_ptr()as usize,left.len());let right_same=header.right==(right.as_ptr()as usize,right.len());
  crate::RetainedWorkOutcome{result:Ok(Some(left_same&&right_same)),work:3}
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.header.is_none()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}Ok(if self.header.is_some(){RetirementDemand{copy_bytes:size_of::<Option<SliceHeader>>()*2,depth:1,..Default::default()}}else{Default::default()})}
 pub fn close_step(&mut self,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  let d=match self.close_demand(){Ok(d)=>d,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e),work:0}};
  if self.terminal_is_empty(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if maximum_work_units==0||!retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.header=None;crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
}
impl Drop for RetainedSliceInspection{fn drop(&mut self){assert!(std::thread::panicking()||self.header.is_none(),"staged slice header requires admitted closure")}}
/// 🪜️ Staged scalar comparison holds only the paid header; its caller owns the genuine immutable source bindings.
pub(crate) struct RetainedStagedSliceEquality{cursor:RetainedSliceEquality,inspection:RetainedSliceInspection}
impl RetainedStagedSliceEquality{
 pub fn new()->Self{Self{cursor:RetainedSliceEquality::new(),inspection:RetainedSliceInspection::new()}}
 pub fn ready(&self)->bool{self.cursor.ready()}
 pub fn equal(&self)->Option<bool>{self.cursor.equal()}
 fn selection<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<(RetirementDemand,u64,usize),ValueError>{
  let header=self.inspection.header.as_ref().ok_or_else(invalid)?;if self.cursor.ready(){return Err(invalid())}if work<5{return Ok((Default::default(),0,0))}
  let fixed=size_of::<RetainedSliceEquality>()*2;let left=header.left.1;let right=header.right.1;
  if left!=right||self.cursor.offset==left{return Ok((RetirementDemand{copy_bytes:fixed,depth:1,..Default::default()},5,0))}
  let n=self.cursor.count::<T>(left,right,body,work-4)?;if n==0{return Ok((Default::default(),0,0))}
  let bytes=n.checked_mul(size_of::<T>()).and_then(|x|x.checked_mul(2)).and_then(|x|x.checked_add(fixed)).ok_or_else(invalid)?;let selected=(n as u64).checked_mul(T::WORK).and_then(|x|x.checked_add(5)).ok_or_else(invalid)?;
  Ok((RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()},selected,n))
 }
 pub fn demand<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if !self.inspection.ready(){return self.inspection.inspect_demand()}Ok(self.selection::<T>(body,work)?.0)}
 pub fn work_demand<T:RetainedEqualityElement>(&self,body:usize,work:u64)->Result<u64,ValueError>{if !self.inspection.ready(){self.inspection.inspect_demand()?;return Ok(if work>=3{3}else{0})}Ok(self.selection::<T>(body,work)?.1)}
 pub fn advance<T:RetainedEqualityElement>(&mut self,left:&[T],right:&[T],body:usize,work:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  if !self.inspection.ready(){return self.inspection.inspect(left,right,work,g)}
  let(d,selected,n)=match self.selection::<T>(body,work){Ok(x)=>x,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e),work:0}};if selected==0||!retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let check=self.inspection.verify(left,right,work);match check.result{Ok(Some(true))=>{},Ok(_)=>return crate::RetainedWorkOutcome{result:Err(invalid()),work:check.work},Err(e)=>return crate::RetainedWorkOutcome{result:Err(e),work:check.work}}
  let mut actual:u128=5;let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RetainedSliceEquality>()*2,..Default::default()};let header=self.inspection.header.as_ref().unwrap();
  if header.left.1!=header.right.1{self.cursor.result=Some(false);return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(p)),work:actual}}
  let end=self.cursor.offset+n;for i in self.cursor.offset..end{actual+=u128::from(T::WORK);p.copied_bytes+=size_of::<T>()*2;if !T::inline_eq(&left[i],&right[i]){self.cursor.result=Some(false);return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(p)),work:actual}}}
  self.cursor.offset=end;if end==header.left.1{self.cursor.result=Some(true)}crate::RetainedWorkOutcome{result:Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:actual}
 }
 pub fn begin_close(&mut self){self.inspection.begin_close()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{let d=self.inspection.close_demand()?;if !self.inspection.terminal_is_empty(){return Ok(d)}if self.cursor==RetainedSliceEquality::new(){return Ok(Default::default())}Ok(RetirementDemand{copy_bytes:size_of::<RetainedSliceEquality>()*2,depth:1,..Default::default()})}
 pub fn close_work_demand(&self)->Result<u64,ValueError>{self.close_demand()?;Ok(if self.terminal_is_empty(){0}else{1})}
 pub fn close_step(&mut self,work:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{if !self.inspection.terminal_is_empty(){let turn=self.inspection.close_step(work,g);let result=turn.result.map(|step|if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});return crate::RetainedWorkOutcome{result,work:turn.work}}let d=match self.close_demand(){Ok(d)=>d,Err(e)=>return crate::RetainedWorkOutcome{result:Err(e),work:0}};if self.terminal_is_empty(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if work==0||!retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.cursor=RetainedSliceEquality::new();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.inspection.header.is_none()&&self.cursor==RetainedSliceEquality::new()}
}
impl Drop for RetainedStagedSliceEquality{fn drop(&mut self){assert!(std::thread::panicking()||(self.inspection.header.is_none()&&self.cursor==RetainedSliceEquality::new()),"staged slice metadata requires admitted closure")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
