//! 🟰️ Intrinsic equality composes the existing typed record and sequence cursors over original immutable fields.
use semio_framework_value_intrinsic::{DslValue,Number};
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use crate::{RetainedEquality,RetainedEqualityCursor,RetainedEqualityFields,RetainedRecordEquality,RetainedEqualityElement};
use crate::retained_equality::{RetainedInlineEquality,RetainedEqualityBirth,equality_box,equality_box_work};
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original intrinsic equality changed its source or field")}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl RetainedEqualityElement for Number{const WORK:u64=1;fn inline_eq(left:&Self,right:&Self)->bool{left==right}}
impl RetainedEquality for Number{
 type Equality=RetainedInlineEquality<Self>;
 const CURSOR_BIRTH_WORK:u64=2;
 const INLINE_PAIR_BYTES:Option<usize>=Some(size_of::<Self>()*2);
 const INLINE_PAIR_WORK:Option<u64>=Some(<Self as RetainedEqualityElement>::WORK);
 fn equality_cursor()->Self::Equality{RetainedInlineEquality::new()}
 fn inline_equal(left:&Self,right:&Self)->Option<bool>{Some(left==right)}
}
struct ValueField<T:RetainedEquality>{cursor:T::Equality,project:fn(&DslValue)->Option<&T>}
impl<T:RetainedEquality> RetainedEqualityCursor<DslValue> for ValueField<T>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}self.cursor.demand(body,work-2)}
 fn demand_admitted(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}self.cursor.demand_admitted(body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let child=self.cursor.work_demand(body,work-2)?;if child==0{return Ok(0)}child.checked_add(2).ok_or_else(invalid)}
 fn advance(&mut self,left:&DslValue,right:&DslValue,body:usize,work:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<crate::RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}let selected=match self.work_demand(body,work){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return RetainedWorkOutcome{result:Ok(crate::RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match self.demand_admitted(body,work){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if !permits(g,d){return RetainedWorkOutcome{result:Ok(crate::RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let left=match(self.project)(left){Some(value)=>value,None=>return RetainedWorkOutcome{result:Err(invalid().into()),work:1}};let right=match(self.project)(right){Some(value)=>value,None=>return RetainedWorkOutcome{result:Err(invalid().into()),work:2}};let turn=self.cursor.advance(left,right,body,work-2,effect);RetainedWorkOutcome{result:turn.result,work:turn.work+2}}
 fn begin_close(&mut self){self.cursor.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.cursor.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.cursor.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}
struct TextBirth;struct BytesBirth;struct ArrayBirth;struct ObjectBirth;
macro_rules! value_birth{($name:ident,$ty:ty,$project:ident)=>{impl RetainedEqualityBirth for $name{type Output=ValueField<$ty>;fn work()->Result<u64,ValueError>{<$ty as RetainedEquality>::CURSOR_BIRTH_WORK.checked_add(2).ok_or_else(invalid)}fn birth()->Self::Output{ValueField{cursor:<$ty as RetainedEquality>::equality_cursor(),project:$project}}}};}
value_birth!(TextBirth,String,text);value_birth!(BytesBirth,Vec<u8>,bytes);value_birth!(ArrayBirth,Vec<DslValue>,array);value_birth!(ObjectBirth,Vec<(String,DslValue)>,object);
fn field<B>(work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<DslValue>>,RetainedCloneProgress)>> where B:RetainedEqualityBirth,B::Output:RetainedEqualityCursor<DslValue>+'static{let turn=equality_box::<B>(work,g,actual);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<DslValue>>,p)))}}
fn text(value:&DslValue)->Option<&String>{match value{DslValue::String(value)=>Some(value),_=>None}}
fn bytes(value:&DslValue)->Option<&Vec<u8>>{match value{DslValue::Bytes(value)=>Some(value),_=>None}}
fn array(value:&DslValue)->Option<&Vec<DslValue>>{match value{DslValue::Array(value)=>Some(value),_=>None}}
fn object(value:&DslValue)->Option<&Vec<(String,DslValue)>>{match value{DslValue::Object(value)=>Some(value),_=>None}}
impl RetainedEquality for DslValue{
 type Equality=RetainedRecordEquality<Self>;
 const CURSOR_BIRTH_WORK:u64=7;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedRecordEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl RetainedEqualityFields for DslValue{
 const SHAPE_WORK:u64=1;const FIELD_METADATA_WORK:u64=1;const FIELD_PROJECTION_WORK:u64=1;
 fn equality_field_birth_work(left:&Self,index:usize)->Result<Option<u64>,ValueError>{if index!=0{return Ok(None)}match left{Self::String(_)=>equality_box_work::<TextBirth>().map(Some),Self::Bytes(_)=>equality_box_work::<BytesBirth>().map(Some),Self::Array(_)=>equality_box_work::<ArrayBirth>().map(Some),Self::Object(_)=>equality_box_work::<ObjectBirth>().map(Some),_=>Ok(None)}}
 fn equality_shape(left:&Self,right:&Self)->Option<usize>{match(left,right){(Self::Null,Self::Null)=>Some(0),(Self::Bool(_),Self::Bool(_))|(Self::Number(_),Self::Number(_))|(Self::String(_),Self::String(_))|(Self::Bytes(_),Self::Bytes(_))|(Self::Array(_),Self::Array(_))|(Self::Object(_),Self::Object(_))=>Some(1),_=>None}}
 fn equality_field_inline_bytes(left:&Self,index:usize)->Option<usize>{if index!=0{return None}match left{Self::Bool(_)=>Some(size_of::<bool>()*2),Self::Number(_)=>Some(size_of::<Number>()*2),_=>None}}
 fn equality_field_inline_work(left:&Self,index:usize)->Option<u64>{if index!=0{return None}match left{Self::Bool(_)=>Some(<bool as RetainedEquality>::INLINE_PAIR_WORK?),Self::Number(_)=>Some(<Number as RetainedEquality>::INLINE_PAIR_WORK?),_=>None}}
 fn equality_field_inline(left:&Self,right:&Self,index:usize)->Option<bool>{if index!=0{return None}match(left,right){(Self::Bool(a),Self::Bool(b))=>Some(a==b),(Self::Number(a),Self::Number(b))=>Some(a==b),_=>None}}
 fn equality_field_bytes(left:&Self,index:usize)->usize{if index!=0{return 0}match left{Self::String(_)=>size_of::<ValueField<String>>(),Self::Bytes(_)=>size_of::<ValueField<Vec<u8>>>(),Self::Array(_)=>size_of::<ValueField<Vec<DslValue>>>(),Self::Object(_)=>size_of::<ValueField<Vec<(String,DslValue)>>>(),_=>0}}
 fn equality_field(left:&Self,index:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<Self>>,RetainedCloneProgress)>>{if index==0{match left{Self::String(_)=>return field::<TextBirth>(work,g,actual),Self::Bytes(_)=>return field::<BytesBirth>(work,g,actual),Self::Array(_)=>return field::<ArrayBirth>(work,g,actual),Self::Object(_)=>return field::<ObjectBirth>(work,g,actual),_=>{}}}RetainedWorkOutcome{result:Err(invalid()),work:0}}
}
/// 🪞️ Every ready result follows funded original-field reads and retains the real comparison frontier until explicit close.
#[derive(Clone,Copy)]
pub struct ValueEqualityStep{pub equal:Option<bool>,pub progress:RetainedCloneProgress}
/// 🧳️ The caller keeps these exact immutable source fields stable; this owner stores no borrowed pointer or alternate traversal.
pub struct OriginalValueEquality{cursor:RetainedRecordEquality<DslValue>,binding:Option<(usize,usize)>,result:Option<bool>,closing:bool}
impl OriginalValueEquality{
 pub fn new()->Self{Self{cursor:RetainedRecordEquality::new(),binding:None,result:None,closing:false}}
 fn validate(&self,left:&DslValue,right:&DslValue)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|pair|pair!=(left as*const DslValue as usize,right as*const DslValue as usize)){Err(invalid())}else{Ok(())}}
 pub fn result(&self)->Option<bool>{self.result}
 pub fn next_demands(&self,left:&DslValue,right:&DslValue,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.validate(left,right)?;if maximum_work_units==0{return Ok(Default::default())}if self.binding.is_none(){Ok(RetirementDemand{copy_bytes:size_of::<(usize,usize)>(),depth:1,..Default::default()})}else{self.cursor.demand(body,maximum_work_units.saturating_sub(1))}}
 pub fn work_demand(&self,left:&DslValue,right:&DslValue,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.next_demands(left,right,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.binding.is_none(){return Ok(1)}let work=self.cursor.work_demand(body,maximum_work_units.saturating_sub(1))?;if work==0{return Ok(0)}let work=work.checked_add(1).ok_or_else(invalid)?;Ok(if work<=maximum_work_units{work}else{0})}
 pub fn advance(&mut self,left:&DslValue,right:&DslValue,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<ValueEqualityStep,crate::OriginalRetirementFailure>{let turn=self.advance_phase(left,right,body,maximum_work_units,effect);crate::retained_equality::settle_work(effect,turn)}
 fn advance_phase(&mut self,left:&DslValue,right:&DslValue,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<ValueEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();
  if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
  let selected=match self.work_demand(left,right,body,maximum_work_units){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return RetainedWorkOutcome{result:Ok(ValueEqualityStep{equal:None,progress:Default::default()}),work:0}}
  let result=match self.next_demands(left,right,body,maximum_work_units){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if !permits(g,result){return RetainedWorkOutcome{result:Ok(ValueEqualityStep{equal:None,progress:Default::default()}),work:0}}
  if self.binding.is_none(){self.binding=Some((left as*const DslValue as usize,right as*const DslValue as usize));return RetainedWorkOutcome{result:Ok(ValueEqualityStep{equal:None,progress:RetainedCloneProgress{copied_items:1,copied_bytes:result.copy_bytes,..Default::default()}}),work:1}}
  let turn=self.cursor.advance(left,right,body,maximum_work_units-1,effect);RetainedWorkOutcome{work:1+turn.work,result:turn.result.map(|step|{self.result=step.equal;ValueEqualityStep{equal:step.equal,progress:step.progress}})}
 }
 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close()}
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.cursor.terminal_is_empty(){return self.cursor.close_demand(body)}Ok(if self.binding.is_some(){RetirementDemand{copy_bytes:size_of::<(usize,usize)>(),depth:1,..Default::default()}}else{Default::default()})}
 pub fn next_close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.cursor.terminal_is_empty(){return self.cursor.close_work_demand(body)}Ok(u64::from(self.binding.is_some()))}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let poll=self.close_phase(effect);crate::retained_equality::settle_poll(effect,poll)}
 fn close_phase(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let done=|result:Result<RetainedCloneStep,ValueError>,work:u128|crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work});
  if !self.closing{return done(Err(invalid()),0)}if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  let d=match self.next_close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error),0)};if !permits(g,d){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_step(effect)}self.binding=None;self.result=None;done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),1)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.cursor.terminal_is_empty()}
}
impl Drop for OriginalValueEquality{fn drop(&mut self){assert!(self.terminal_is_empty(),"original intrinsic equality requires complete funded closure")}}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

/// 🧳️ Inline retirement keeps the already closing original equality owner.
#[doc(hidden)]
pub struct EqualityRetirement(OriginalValueEquality);
impl crate::retirement::RetireOwned for OriginalValueEquality{
 type RetirementCursor=EqualityRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{EqualityRetirement(self)}
 fn retirement_ready(&self)->bool{self.closing}
 fn retirement_birth_bytes(&self)->Option<usize>{self.closing.then_some(size_of::<EqualityRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetirementCursor for EqualityRetirement{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.next_close_demands(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{use crate::retirement::RetirementStep;let turn=match self.0.close_step(effect){crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{work}=>return RetirementStep::Pending(work)};let work=turn.work;match turn.result{Err(error)=>RetirementStep::Failure(error.into(),work),Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete(work),Ok(step)if step.progress()!=RetainedCloneProgress::default()||work!=0=>RetirementStep::Progress(step.progress(),work),Ok(_)=>RetirementStep::BudgetExhausted}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.next_close_demands(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.0.terminal_is_empty().then_some(size_of::<Self>())}
}
