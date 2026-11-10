//! 🪞️ A transparent newtype forwards every turn to its inner field's cursor and only moves the finished inner value into its wrapper.
use crate::receiving::owned::*;
use crate::FieldValue;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkPoll,RetirementDemand,retirement::RetireOwned};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
/// 🧩️ Validation borrows the finished inner value and wrapping is one pure constructor, so a refusal leaves the inner value in its paid retirement slot.
pub trait TransparentField:RetireOwned+Sized+Send+'static{type Inner:DslOwnedReceivingField;fn check(_:&Self::Inner)->Result<(),ValueError>{Ok(())}fn wrap(inner:Self::Inner)->Self;}
pub struct TransparentReceiving<W:TransparentField>{inner:OwnedSlot<W::Inner>,output:OwnedSlot<W>,child:OwnedReceivingSlot<W::Inner>,phase:u8,closing:bool,failed:bool}
impl<W:TransparentField>Default for TransparentReceiving<W>{fn default()->Self{Self{inner:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),phase:0,closing:false,failed:false}}}
impl<W:TransparentField>TransparentReceiving<W>{
 fn selected(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.failed||self.closing{return done(effect,Err(invariant().into()))}if self.phase==4{return idle(effect)}
  let d=match self.demands(source,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let g=effect.grant();
  match self.phase{
   0 if !self.child.is_ready()=>{let poll=self.child.advance_local(source,body,&mut effect.nested(child_grant(g)));return settle(effect,poll)},
   1 if self.inner.value.is_none()=>{let poll=self.child.take_into(&mut self.inner.value,&mut effect.nested(child_grant(g)));return settle(effect,normal(poll))},
   2 if !self.child.terminal_is_empty()=>{let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,receiving(poll))},
   _=>{}
  }
  local(effect,d,||{match self.phase{
   0=>self.phase=1,
   1=>self.phase=2,
   2=>self.phase=3,
   3=>{W::check(self.inner.value.as_ref().ok_or_else(invariant)?)?;let inner=self.inner.value.take().ok_or_else(invariant)?;*self.output.value=Some(W::wrap(inner));self.phase=4;},
   _=>return Err(invariant())}Ok(())})
 }
}
impl<W:TransparentField>OwnedFieldReceiving<W>for TransparentReceiving<W>{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{0 if !self.child.is_ready()=>self.child.demands(source,body).and_then(nested),1 if self.inner.value.is_none()=>nested(nested(metadata::<()>())?),2 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),4=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,source:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.failed||self.closing{return Err(invariant())}let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});match self.phase{0 if !self.child.is_ready()=>self.child.select_advance(source,body,maximum_work_units),1 if self.inner.value.is_none()=>local(self.child.take_work_demand()?),2 if !self.child.terminal_is_empty()=>local(self.child.close_work_demand(body)?),4=>local(0),_=>local(1)}}
 fn advance_nested(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{if self.failed||self.closing||self.phase!=0||self.child.is_ready(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}let poll=self.child.advance_nested(source,native,witness,body,maximum_work_units,child_grant(g));poison(&mut self.failed,&poll);poll}
 fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(source,body,effect);poison(&mut self.failed,&poll);poll}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==4&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<W>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested)}if !self.inner.terminal_is_empty(){return self.inner.close_demands(body)}self.output.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.closing{return Ok(1)}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}self.output.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{if self.terminal_is_empty(){return idle(effect)}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())})}if !self.child.terminal_is_empty(){let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,poll)}if !self.inner.terminal_is_empty(){return self.inner.close_step(effect)}self.output.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.inner.terminal_is_empty()&&self.output.terminal_is_empty()}
}
/// 🧬️ One declaration gives a transparent newtype over a cursor-received field both receiving families.
#[macro_export]
macro_rules! dsl_transparent_receiving{

 ($name:ty,$inner:ty,$check:expr,$wrap:expr)=>{
  impl $crate::transparent::TransparentField for $name{type Inner=$inner;fn check(inner:&$inner)->Result<(),$crate::receiving::owned::ValueError>{let check:fn(&$inner)->Result<(),$crate::receiving::owned::ValueError>=$check;check(inner)}fn wrap(inner:$inner)->Self{let wrap:fn($inner)->Self=$wrap;wrap(inner)}}
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::transparent::TransparentReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,output:&mut Option<Self>,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<(),$crate::receiving::NativeSnapshotFailure>{$crate::receiving::receive_wrapped::<$inner,Self>(value,output,owner,<$name as $crate::transparent::TransparentField>::check,<$name as $crate::transparent::TransparentField>::wrap)}}
 };
 ($name:ty,$inner:ty)=>{
  impl $crate::transparent::TransparentField for $name{type Inner=$inner;fn wrap(inner:$inner)->Self{Self(inner)}}
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::transparent::TransparentReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,output:&mut Option<Self>,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<(),$crate::receiving::NativeSnapshotFailure>{$crate::receiving::receive_wrapped::<$inner,Self>(value,output,owner,<$name as $crate::transparent::TransparentField>::check,<$name as $crate::transparent::TransparentField>::wrap)}}
 };
}
