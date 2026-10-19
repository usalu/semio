//! 👥️ Heterogeneous sequences retain one original typed child and separately settle every authored field.
use super::*;
macro_rules! native_tuple{
 ($cursor:ident,$count:literal,$($index:literal:$kind:ident:$slot:ident:$field:ident),+)=>{
  pub struct $cursor<I:NativeValueSource,$($kind:NativeReceive<I>),+>{binding:Option<RetainedCloneBinding>,$($slot:ManuallyDrop<Option<Box<NativeReceiveSlot<I,$kind>>>>,$field:ManuallyDrop<Option<$kind>>,)+retirement:RetainedCloneClose,index:usize,stage:u8,phase:u8,closing:bool}
  impl<I:NativeValueSource,$($kind:NativeReceive<I>),+> NativeReceive<I> for ($($kind,)+){
   type Receiver=$cursor<I,$($kind),+>;
   fn receive_birth_demand()->RetirementDemand{fixed(2*size_of::<Self::Receiver>())}
   fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok(($cursor{binding:None,$($slot:ManuallyDrop::new(None),$field:ManuallyDrop::new(None),)+retirement:Default::default(),index:0,stage:0,phase:0,closing:false},receipt(d)))}
  }
  impl<I:NativeValueSource,$($kind:NativeReceive<I>),+> NativeReceiver<I,($($kind,)+)> for $cursor<I,$($kind),+>{
   fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
    if self.closing||self.phase>=3{return Err(invalid());}input.check_binding(&self.binding)?;let overhead=size_of::<usize>()+size_of::<u8>();
    if self.phase==0{return Ok(fixed(input.binding_copy_bytes()+size_of::<u8>()));}if self.phase==1{return Ok(fixed(size_of::<usize>()+size_of::<u8>()));}if self.index==$count{return Ok(fixed(overhead));}let rank=self.index;input.get().sequence_child(rank)?;let source=input.project(rank,move|v|v.sequence_child(rank).unwrap());
    match self.index{$($index=>match self.stage{
     0=>Ok(RetirementDemand{copy_bytes:2*size_of::<NativeReceiveSlot<I,$kind>>()+2*size_of::<Option<Box<NativeReceiveSlot<I,$kind>>>>()+overhead,capacity_bytes:size_of::<NativeReceiveSlot<I,$kind>>(),depth:1,..Default::default()}),
     1=>nested(self.$slot.as_ref().ok_or_else(invalid)?.demands(source,body)?,overhead),
     2=>Ok(fixed(2*size_of::<$kind>()+size_of::<Option<$kind>>()+overhead)),
     3=>nested(self.$slot.as_ref().ok_or_else(invalid)?.close_demands(body.saturating_sub(overhead))?,overhead),
     4=>Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Box<NativeReceiveSlot<I,$kind>>>>()+overhead,release_bytes:size_of::<NativeReceiveSlot<I,$kind>>(),depth:1,..Default::default()}),_=>Err(invalid())},)+_=>Err(invalid())}
   }
   fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;
    if self.phase==0{let p=input.bind(&mut self.binding,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<u8>(),..g})?.ok_or_else(invalid)?;self.phase=1;return Ok(RetainedCloneStep::Progress(add_receipt(p,size_of::<u8>())?));}if self.phase==1{if input.get().sequence_len()? != $count{return Err(invalid());}self.phase=2;return Ok(RetainedCloneStep::Progress(receipt(d)));}if self.index==$count{self.phase=3;return Ok(RetainedCloneStep::Complete(receipt(d)));}
    let rank=self.index;let source=input.project(rank,move|v|v.sequence_child(rank).unwrap());let overhead=size_of::<usize>()+size_of::<u8>();let child=child_grant(g,overhead);
    match self.index{$($index=>match self.stage{
     0=>{control.charge(d.capacity_bytes)?;*self.$slot=Some(Box::new(NativeReceiveSlot::default()));self.stage=1;},
     1=>{let slot=self.$slot.as_mut().ok_or_else(invalid)?;let p=slot.advance(source,control,body,child).map_err(|e|{let p=e.retained_progress();e.with_retained_progress(add_receipt(p,overhead).unwrap_or(p))})?.progress();if slot.ready(){self.stage=2;}return Ok(RetainedCloneStep::Progress(add_receipt(p,overhead)?));},
     2=>{let slot=self.$slot.as_mut().ok_or_else(invalid)?;*self.$field=Some(slot.take().ok_or_else(invalid)?);slot.begin_close();self.stage=if slot.terminal_is_empty(){4}else{3};},
     3=>{control.charge(d.capacity_bytes)?;let slot=self.$slot.as_mut().ok_or_else(invalid)?;let p=slot.close_step(child)?.progress();if slot.terminal_is_empty(){self.stage=4;}return Ok(RetainedCloneStep::Progress(add_receipt(p,overhead)?));},
     4=>{self.$slot.take();self.index+=1;self.stage=0;},_=>return Err(invalid())},)+_=>return Err(invalid())}Ok(RetainedCloneStep::Progress(receipt(d)))
   }
   fn output_ready(&self)->bool{!self.closing&&self.phase==3$(&&self.$field.is_some())+}
   fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(2*size_of::<($($kind,)+)>()$(+size_of::<Option<$kind>>())++size_of::<u8>()))}
   fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(($($kind,)+),RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}let result=($(self.$field.take().ok_or_else(invalid)?,)+);self.phase=4;Ok(Some((result,receipt(d))))}
   fn begin_close(&mut self){self.closing=true;$(if let Some(slot)=self.$slot.as_mut(){slot.begin_close();})+}
   fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
    if !self.closing{return Err(invalid());}$(if let Some(slot)=self.$slot.as_ref(){return if slot.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Box<NativeReceiveSlot<I,$kind>>>>(),release_bytes:size_of::<NativeReceiveSlot<I,$kind>>(),depth:1,..Default::default()})}else{slot.close_demands(body)};})+
    if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<($($kind,)+)>(false,&None,body);}$(if self.$field.is_some(){return self.retirement.next_owner_demands::<$kind>(true,&None,body);})+RetainedCloneBinding::demands(&self.binding,body)
   }
   fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}$(if let Some(slot)=self.$slot.as_mut(){if !slot.terminal_is_empty(){return slot.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}self.$slot.take();return Ok(RetainedCloneStep::Progress(receipt(d)));})+if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}$(if let Some(step)=self.retirement.begin_granted(&mut self.$field,g)?{return Ok(RetainedCloneStep::Progress(step.progress()));})+RetainedCloneBinding::close_one(&mut self.binding,g)}
   fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.retirement.is_empty()$(&&self.$slot.is_none()&&self.$field.is_none())+}
  }
  impl<I:NativeValueSource,$($kind:NativeReceive<I>),+> Drop for $cursor<I,$($kind),+>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native tuple receiving requires original funded closure");}}
 };
}
native_tuple!(NativeTuple2Receiver,2,0:A:slot_a:field_a,1:B:slot_b:field_b);
native_tuple!(NativeTuple3Receiver,3,0:A:slot_a:field_a,1:B:slot_b:field_b,2:C:slot_c:field_c);
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;