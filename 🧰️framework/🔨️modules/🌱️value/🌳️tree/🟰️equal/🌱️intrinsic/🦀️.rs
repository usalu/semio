//! 🟰️ Intrinsic equality composes the existing typed record and sequence cursors over original immutable fields.
use semio_framework_value_intrinsic::{DslValue,Number};
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use crate::{RetainedEquality,RetainedEqualityCursor,RetainedEqualityFields,RetainedRecordEquality,RetainedEqualityElement};
use crate::retained_equality::{RetainedInlineEquality,equality_box};
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original intrinsic equality changed its source or field")}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl RetainedEqualityElement for Number{fn inline_eq(left:&Self,right:&Self)->bool{left==right}}
impl RetainedEquality for Number{
 type Equality=RetainedInlineEquality<Self>;
 const INLINE_PAIR_BYTES:Option<usize>=Some(size_of::<Self>()*2);
 fn equality_cursor()->Self::Equality{RetainedInlineEquality::new()}
 fn inline_equal(left:&Self,right:&Self)->Option<bool>{Some(left==right)}
}
struct ValueField<T:RetainedEquality>{cursor:T::Equality,project:fn(&DslValue)->Option<&T>}
impl<T:RetainedEquality> RetainedEqualityCursor<DslValue> for ValueField<T>{
 fn demand(&self,left:&DslValue,right:&DslValue,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.demand((self.project)(left).ok_or_else(invalid)?,(self.project)(right).ok_or_else(invalid)?,body)}
 fn advance(&mut self,left:&DslValue,right:&DslValue,g:RetainedCloneGrant)->RetainedWorkOutcome<crate::RetainedEqualityStep>{match((self.project)(left),(self.project)(right)){(Some(left),Some(right))=>self.cursor.advance(left,right,g),_=>RetainedWorkOutcome{result:Err(invalid()),work:0}}}
 fn begin_close(&mut self){self.cursor.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.cursor.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}
fn field<T:RetainedEquality>(project:fn(&DslValue)->Option<&T>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<DslValue>>,RetainedCloneProgress)>>{
 let turn=equality_box(ValueField{cursor:T::equality_cursor(),project},g);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<DslValue>>,p)))}
}
fn text(value:&DslValue)->Option<&String>{match value{DslValue::String(value)=>Some(value),_=>None}}
fn bytes(value:&DslValue)->Option<&Vec<u8>>{match value{DslValue::Bytes(value)=>Some(value),_=>None}}
fn array(value:&DslValue)->Option<&Vec<DslValue>>{match value{DslValue::Array(value)=>Some(value),_=>None}}
fn object(value:&DslValue)->Option<&Vec<(String,DslValue)>>{match value{DslValue::Object(value)=>Some(value),_=>None}}
impl RetainedEquality for DslValue{
 type Equality=RetainedRecordEquality<Self>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedRecordEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl RetainedEqualityFields for DslValue{
 fn equality_shape(left:&Self,right:&Self)->Option<usize>{match(left,right){(Self::Null,Self::Null)=>Some(0),(Self::Bool(_),Self::Bool(_))|(Self::Number(_),Self::Number(_))|(Self::String(_),Self::String(_))|(Self::Bytes(_),Self::Bytes(_))|(Self::Array(_),Self::Array(_))|(Self::Object(_),Self::Object(_))=>Some(1),_=>None}}
 fn equality_field_inline_bytes(left:&Self,index:usize)->Option<usize>{if index!=0{return None}match left{Self::Bool(_)=>Some(size_of::<bool>()*2),Self::Number(_)=>Some(size_of::<Number>()*2),_=>None}}
 fn equality_field_inline(left:&Self,right:&Self,index:usize)->Option<bool>{if index!=0{return None}match(left,right){(Self::Bool(a),Self::Bool(b))=>Some(a==b),(Self::Number(a),Self::Number(b))=>Some(a==b),_=>None}}
 fn equality_field_bytes(left:&Self,index:usize)->usize{if index!=0{return 0}match left{Self::String(_)=>size_of::<ValueField<String>>(),Self::Bytes(_)=>size_of::<ValueField<Vec<u8>>>(),Self::Array(_)=>size_of::<ValueField<Vec<DslValue>>>(),Self::Object(_)=>size_of::<ValueField<Vec<(String,DslValue)>>>(),_=>0}}
 fn equality_field(left:&Self,index:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<Self>>,RetainedCloneProgress)>>{if index==0{match left{Self::String(_)=>return field(text,g),Self::Bytes(_)=>return field(bytes,g),Self::Array(_)=>return field(array,g),Self::Object(_)=>return field(object,g),_=>{}}}RetainedWorkOutcome{result:Err(invalid()),work:0}}
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
 pub fn next_demands(&self,left:&DslValue,right:&DslValue,body:usize)->Result<RetirementDemand,ValueError>{self.validate(left,right)?;if self.binding.is_none(){Ok(RetirementDemand{copy_bytes:size_of::<(usize,usize)>(),depth:1,..Default::default()})}else{self.cursor.demand(left,right,body)}}
 pub fn advance(&mut self,left:&DslValue,right:&DslValue,g:RetainedCloneGrant)->RetainedWorkOutcome<ValueEqualityStep>{
  let result=match self.next_demands(left,right,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if !permits(g,result){return RetainedWorkOutcome{result:Ok(ValueEqualityStep{equal:None,progress:Default::default()}),work:0}}
  if self.binding.is_none(){self.binding=Some((left as*const DslValue as usize,right as*const DslValue as usize));return RetainedWorkOutcome{result:Ok(ValueEqualityStep{equal:None,progress:RetainedCloneProgress{copied_items:1,copied_bytes:result.copy_bytes,..Default::default()}}),work:1}}
  let turn=self.cursor.advance(left,right,g);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|{self.result=step.equal;ValueEqualityStep{equal:step.equal,progress:step.progress}})}
 }
 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close()}
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.cursor.terminal_is_empty(){return self.cursor.close_demand(body)}Ok(if self.binding.is_some(){RetirementDemand{copy_bytes:size_of::<(usize,usize)>(),depth:1,..Default::default()}}else{Default::default()})}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let d=match self.next_close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !permits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_step(g)}self.binding=None;self.result=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.cursor.terminal_is_empty()}
}
impl Drop for OriginalValueEquality{fn drop(&mut self){assert!(self.terminal_is_empty(),"original intrinsic equality requires complete funded closure")}}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

struct EqualityRetirement(OriginalValueEquality);
impl crate::retirement::RetireOwned for OriginalValueEquality{
 fn retirement(mut self)->Box<dyn crate::retirement::RetirementCursor>{self.begin_close();Box::new(EqualityRetirement(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EqualityRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetirementCursor for EqualityRetirement{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.next_close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::retirement::RetirementStep{use crate::retirement::RetirementStep;let turn=self.0.close_step(g);match turn.result{Err(error)=>RetirementStep::Failure(error,turn.work),Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),Ok(step)if step.progress()!=RetainedCloneProgress::default()||turn.work!=0=>RetirementStep::Progress(step.progress(),turn.work),Ok(_)=>RetirementStep::BudgetExhausted}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.next_close_demands(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.0.terminal_is_empty().then_some(size_of::<Self>())}
}
