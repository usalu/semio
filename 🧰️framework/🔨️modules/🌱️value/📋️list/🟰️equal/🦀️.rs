//! 🟰️ Borrowed scalar buffers compare only funded pages and retain no original payload.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,RETAINED_CLONE_BULK_PAGE_BYTES}};
use std::mem::size_of;

/// 🔬️ Equality reads only the fixed inline representation and performs no allocation or borrowed-payload traversal.
pub trait RetainedEqualityElement:Copy{fn inline_eq(left:&Self,right:&Self)->bool;}
macro_rules! scalar{($($ty:ty),*)=>{$(impl RetainedEqualityElement for $ty{fn inline_eq(left:&Self,right:&Self)->bool{left==right}})*}}
scalar!(bool,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64,char,());

/// 📖️ The caller retains immutable original slices and their revision; the cursor owns only scalar metadata.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedSliceEquality{offset:usize,result:Option<bool>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"slice equality lost its original immutable bounds or live phase")}
impl RetainedSliceEquality{
 pub fn new()->Self{Self::default()}
 pub fn ready(&self)->bool{self.result.is_some()}
 pub fn equal(&self)->Option<bool>{self.result}
 pub fn fixed_copy_bytes()->usize{size_of::<Self>()*2+size_of::<usize>()*8}
 fn count<T:RetainedEqualityElement>(&self,left:usize,right:usize,body:usize)->Result<usize,ValueError>{
  if self.ready(){return Err(invalid());}if left!=right{return Ok(0);}let remaining=left.checked_sub(self.offset).ok_or_else(invalid)?;
  if size_of::<T>()==0{return Ok(remaining.min(1));}
  let width=size_of::<T>().checked_mul(2).ok_or_else(invalid)?;
  Ok(remaining.min((RETAINED_CLONE_BULK_PAGE_BYTES/width).max(1)).min((body.saturating_sub(Self::fixed_copy_bytes())/width).max(1)))
 }
 pub fn demand<T:RetainedEqualityElement>(&self,left:&[T],right:&[T],body:usize)->Result<RetirementDemand,ValueError>{
  let count=self.count::<T>(left.len(),right.len(),body)?;
  let bytes=count.checked_mul(size_of::<T>()).and_then(|bytes|bytes.checked_mul(2)).and_then(|bytes|bytes.checked_add(Self::fixed_copy_bytes())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"slice equality page extent overflow"))?;
  Ok(RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})
 }
 pub fn advance<T:RetainedEqualityElement>(&mut self,left:&[T],right:&[T],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.demand(left,right,grant.maximum_copy_bytes)?;
  if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if left.len()!=right.len(){self.result=Some(false);return Ok(RetainedCloneStep::Complete(progress));}
  let end=self.offset+self.count::<T>(left.len(),right.len(),grant.maximum_copy_bytes)?;
  for index in self.offset..end{if !T::inline_eq(&left[index],&right[index]){self.result=Some(false);return Ok(RetainedCloneStep::Complete(progress));}}
  self.offset=end;if end==left.len(){self.result=Some(true);Ok(RetainedCloneStep::Complete(progress))}else{Ok(RetainedCloneStep::Progress(progress))}
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
