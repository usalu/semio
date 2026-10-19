//! 🟰️ Typed equality reads original fields directly and retains every funded traversal frontier.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,RetainedEqualityElement,RetainedSliceEquality,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::size_of;

/// 🌳️ The original immutable caller keeps sources stable; constructors inspect no source and allocate no payload/frontier.
pub trait RetainedEquality:Sized+RetireOwned+Send+Sync+'static{
 type Equality:RetainedEqualityCursor<Self>;
 const INLINE_PAIR_BYTES:Option<usize>;
 fn equality_cursor()->Self::Equality;
 fn inline_equal(left:&Self,right:&Self)->Option<bool>;
}
/// 🧾️ A ready result follows actual funded inspection; cancellation retains child-first original metadata custody.
pub struct RetainedEqualityStep{pub equal:Option<bool>,pub progress:RetainedCloneProgress}
/// 🪡️ Typed child cursors retain no borrowed pointers and close all owned frontier allocations explicitly.
pub trait RetainedEqualityCursor<T:?Sized>:Send+'static{
 fn demand(&self,left:&T,right:&T,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,left:&T,right:&T,grant:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}

/// 🔬️ Inline native equality never traverses a hidden referenced payload.
pub struct RetainedInlineEquality<T>{result:Option<bool>,marker:std::marker::PhantomData<fn()->T>}
impl<T> RetainedInlineEquality<T>{pub fn new()->Self{Self{result:None,marker:Default::default()}}}
impl<T:RetainedEqualityElement+Send+Sync+'static> RetainedEqualityCursor<T> for RetainedInlineEquality<T>{
 fn demand(&self,_:&T,_:&T,_:usize)->Result<RetirementDemand,ValueError>{if self.result.is_some(){return Err(invalid("typed inline comparison is settled"));}Ok(scalar(size_of::<T>()*2+size_of::<Self>()*2+size_of::<bool>()))}
 fn advance(&mut self,left:&T,right:&T,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{let d=self.demand(left,right,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()});}self.result=Some(T::inline_eq(left,right));Ok(RetainedEqualityStep{equal:self.result,progress:RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}})}
 fn begin_close(&mut self){}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{true}
}
macro_rules! inline{($($ty:ty),*)=>{$(impl RetainedEquality for $ty{
 type Equality=RetainedInlineEquality<Self>;
 const INLINE_PAIR_BYTES:Option<usize>=Some(size_of::<Self>()*2);
 fn equality_cursor()->Self::Equality{RetainedInlineEquality::new()}
 fn inline_equal(left:&Self,right:&Self)->Option<bool>{Some(<Self as RetainedEqualityElement>::inline_eq(left,right))}
 })*};}
inline!((),bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64);

/// 🔤️ Original string bytes compare through the existing bounded scalar buffer cursor.
pub struct RetainedStringEquality{cursor:RetainedSliceEquality}
impl RetainedEquality for String{
 type Equality=RetainedStringEquality;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedStringEquality{cursor:Default::default()}}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl RetainedEqualityCursor<String> for RetainedStringEquality{
 fn demand(&self,left:&String,right:&String,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.demand(left.as_bytes(),right.as_bytes(),body)}
 fn advance(&mut self,left:&String,right:&String,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{let step=self.cursor.advance(left.as_bytes(),right.as_bytes(),g)?;Ok(RetainedEqualityStep{equal:self.cursor.equal(),progress:step.progress()})}
 fn begin_close(&mut self){}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{true}
}

/// 🧳️ Genuine original source bindings outlive the whole typed comparison and every child cancellation.
pub struct RetainedEqualityOwner<T:RetainedEquality>{cursor:T::Equality,left:Option<RetainedCloneBinding>,right:Option<RetainedCloneBinding>,result:Option<bool>,closing:bool}
impl<T:RetainedEquality> RetainedEqualityOwner<T>{
 pub fn new()->Self{Self{cursor:T::equality_cursor(),left:None,right:None,result:None,closing:false}}
 pub fn result(&self)->Option<bool>{self.result}
 /// 🧭️ Validates the actual source pair after decision without changing original custody.
 pub fn verify(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>)->Result<(),ValueError>{left.check_binding(&self.left)?;right.check_binding(&self.right)?;Ok(())}
 /// 🪡️ Inverse preparation must retain the exact original right-side source.
 pub fn verify_original(&self,right:RetainedCloneRef<'_,T>)->Result<(),ValueError>{right.check_binding(&self.right)?;Ok(())}
 fn fixed()->usize{size_of::<Option<RetainedCloneBinding>>()*2+size_of::<Option<bool>>()+size_of::<usize>()*2}
 pub fn demand(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.result.is_some(){return Err(invalid("original typed comparison is closing or settled"));}left.check_binding(&self.left)?;right.check_binding(&self.right)?;if self.left.is_none(){return Ok(scalar(left.binding_copy_bytes()));}if self.right.is_none(){return Ok(scalar(right.binding_copy_bytes()));}let mut d=self.cursor.demand(left.get(),right.get(),body.saturating_sub(Self::fixed()))?;d.copy_bytes+=Self::fixed();d.depth+=1;Ok(d)}
 pub fn advance(&mut self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{let d=self.demand(left,right,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()});}if let Some(p)=left.bind(&mut self.left,g)?{return Ok(RetainedEqualityStep{equal:None,progress:p});}if let Some(p)=right.bind(&mut self.right,g)?{return Ok(RetainedEqualityStep{equal:None,progress:p});}let mut step=self.cursor.advance(left.get(),right.get(),RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g})?;step.progress.copied_bytes+=Self::fixed();self.result=step.equal;Ok(step)}
 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("original typed comparison close has not begun"));}if !self.cursor.terminal_is_empty(){return self.cursor.close_demand(body);}RetainedCloneBinding::demands(if self.right.is_some(){&self.right}else{&self.left},body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.cursor.terminal_is_empty(){return self.cursor.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress()));}let step=RetainedCloneBinding::close_one(if self.right.is_some(){&mut self.right}else{&mut self.left},g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()&&self.left.is_none()&&self.right.is_none()}
}
impl<T:RetainedEquality> Drop for RetainedEqualityOwner<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.cursor.terminal_is_empty()&&self.left.is_none()&&self.right.is_none(),"original typed equality retains source/frontier custody");}}

#[path="📋️sequence/🦀️.rs"] pub mod sequence;
#[path="🎚️record/🦀️.rs"] pub mod record;
pub use record::{RetainedEqualityFields,RetainedRecordEquality};
/// 🏗️ Allocates precisely the admitted typed cursor body and reports allocation refusal before moving it.
pub fn equality_box<T>(value:T)->Result<Box<T>,ValueError>{
 let layout=std::alloc::Layout::new::<T>();
 if layout.size()==0{return Ok(Box::new(value));}
 let pointer=unsafe{std::alloc::alloc(layout)} as *mut T;
 if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"typed equality cursor allocation refused"));}
 unsafe{pointer.write(value);Ok(Box::from_raw(pointer))}
}

#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;

#[path="🤝️tuple/🦀️.rs"] mod tuple;
