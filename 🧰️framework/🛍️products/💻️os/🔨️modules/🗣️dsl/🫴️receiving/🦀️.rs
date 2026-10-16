//! 🫴️ Ordinal operations retain generic parsing and owned typed binding across original native turns.
pub use semio_framework_dsl_record::BorrowedDslVariants;
use semio_framework_dsl_record::{FieldValue,receiving::{DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot}};
pub use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};
pub use protocol::OpBinaryReceiving;
use semio_framework_value::{NativeDecodeControl,RetainedCloneStep,RetirementDemand,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_os_pack::record::RetainedRecordReceiving;
use std::mem::{ManuallyDrop,size_of};
/// 🎞️ Implements an authored ordinal operation only through its actual owned receiving capability.
#[macro_export]
macro_rules! original_ordinal_operation_receiving{
 ($operation:ty)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->Result<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>,$crate::original_receiving::ValueError>{$crate::original_receiving::OriginalOperationReceiving::admit(source,range,grant)}
 }};
}
/// 🏷️ Resolves authored protocol tags at compile time and retains one-tag runtime selection.
#[macro_export]
macro_rules! original_tagged_operation_receiving{
 ($operation:ty,$protocol:expr)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->Result<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>,$crate::original_receiving::ValueError>{
   const TAGS:[u64;<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS.len()]={
    let variants=<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS;let mut tags=[0;<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS.len()];let mut index=0;
    while index<variants.len(){tags[index]=$crate::protocol_record::tag($protocol,variants[index].0);let mut previous=0;while previous<index{assert!(tags[previous]!=tags[index],"authored operation tags must be unique");previous+=1;}index+=1;}tags
   };
   $crate::original_receiving::OriginalOperationReceiving::admit_tagged(source,range,&TAGS,grant)
  }
 }};
}
pub struct OriginalOperationReceiving<M:DslOwnedReceivingVariants+BorrowedDslVariants>{
 source:(usize,usize),range:(usize,usize),position:usize,phase:u8,tag:u64,shift:u8,tags:Option<&'static[u64]>,ordinal:usize,
 record:Option<RetainedRecordReceiving>,generic:OwnedSlot<FieldValue>,binding:Option<M::Cursor>,output:OwnedSlot<M>,closing:u8,closed:bool,
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn metadata<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn grant_child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> OriginalOperationReceiving<M>{
 /// 🎟️ Preserves the full original source and exact operation range without parsing or allocating.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  if range.0>range.1||range.1>source.len(){return Err(invalid("original operation escaped its full source"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,position:range.0,phase:0,tag:0,shift:0,tags:None,ordinal:0,record:None,generic:Default::default(),binding:None,output:Default::default(),closing:0,closed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
 }
 /// 🏷️ Borrows immutable authored tags without a runtime metadata owner.
 pub fn admit_tagged(source:&[u8],range:(usize,usize),tags:&'static[u64],grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{if tags.len()!=M::VARIANTS.len(){return Err(invalid("original operation tag declaration differs from variants"))}Self::admit(source,range,grant).map(|admitted|admitted.map(|(mut owner,progress)|{owner.tags=Some(tags);(owner,progress)}))}
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original operation source identity changed"))}Ok(())}
 fn spec(&self)->Result<(&'static str,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{let(keyword,make)=M::VARIANTS.get(self.ordinal).ok_or_else(||invalid("original operation ordinal is not declared"))?;Ok((*keyword,make()))}
 fn demands_inner(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;let mut demand=match self.phase{
   0|1=>metadata::<u64>(),2=>metadata::<usize>(),3=>RetirementDemand{depth:2,..Default::default()},
   4=>{let record=self.record.as_ref().unwrap();if record.is_ready(){metadata::<FieldValue>()}else{return record.demands(source).map(|mut demand|{demand.depth+=1;demand})}},
   5=>{let binding=self.binding.as_ref().unwrap();if binding.is_ready(){metadata::<M>()}else{return binding.demands(&self.generic.value,body).map(|mut demand|{demand.depth+=1;demand})}},
   _=>RetirementDemand{depth:1,..Default::default()},
  };if self.phase==4||self.phase==5{demand.depth+=1}Ok(demand)
 }
 fn step_inner(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let demand=self.demands_inner(source,grant.maximum_copy_bytes)?;if self.phase==6||self.closing!=0||!funded(grant,demand){return Ok(Default::default())}
  match self.phase{
   4 if !self.record.as_ref().unwrap().is_ready()=>return self.record.as_mut().unwrap().step(source,native,grant_child(grant)),
   5 if !self.binding.as_ref().unwrap().is_ready()=>return self.binding.as_mut().unwrap().step(&mut self.generic.value,native,grant_child(grant)),
   _=>{}
  }
  native.checkpoint()?;
  match self.phase{
   0=>{if source.get(self.position)!=Some(&1)||self.position>=self.range.1{return Err(invalid("original operation binary format is invalid"))}self.position+=1;self.phase=1},
   1=>{let byte=source.get(self.position).copied().filter(|_|self.position<self.range.1).ok_or_else(||invalid("original operation ordinal is truncated"))?;if self.shift==63&&byte>1{return Err(invalid("original operation ordinal overflows u64"))}self.tag|=u64::from(byte&127)<<self.shift;self.position+=1;if byte&128==0{if self.shift>0&&byte==0{return Err(invalid("original operation ordinal is not minimal"))}self.phase=2}else{self.shift=self.shift.checked_add(7).filter(|value|*value<=63).ok_or_else(||invalid("original operation ordinal overflows u64"))?;}},
   2=>{if let Some(tags)=self.tags{let tag=tags.get(self.ordinal).ok_or_else(||invalid("original operation tag is not declared"))?;if *tag!=self.tag{self.ordinal+=1;return Ok(receipt(demand))}}else{self.ordinal=usize::try_from(self.tag).map_err(|_|invalid("original operation ordinal exceeds address space"))?;}let(keyword,_)=self.spec()?;self.binding=Some(M::receiving_cursor(keyword)?);self.phase=3},
   3=>{let(_,spec)=self.spec()?;let Some((record,progress))=RetainedRecordReceiving::admit(source,(self.position,self.range.1),spec,semio_framework_os_pack::PackLimits::default(),grant_child(grant))?else{return Ok(Default::default())};self.record=Some(record);self.phase=4;return Ok(progress)},
   4=>{let mut original=None;let progress=self.record.as_mut().unwrap().take_into(&mut original,grant_child(grant))?;if original.is_some(){*self.generic.value=Some(FieldValue::Record(original.take().unwrap()));self.phase=5;}return Ok(progress)},
   5=>{let progress=self.binding.as_mut().unwrap().take_into(&mut self.output.value,grant_child(grant))?;if self.output.value.is_some(){self.phase=6;}return Ok(progress)},
   _=>return Err(invalid("original operation phase is invalid")),
  }Ok(receipt(demand))
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}if !self.output.terminal_is_empty(){return self.output.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(binding)=&self.binding{if !binding.terminal_is_empty(){return binding.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.generic.terminal_is_empty(){return self.generic.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(record)=&self.record{if !record.terminal_is_empty(){return record.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=1;
  if !self.output.terminal_is_empty(){return self.output.close_step(grant_child(grant)).map(RetainedCloneStep::Progress)}
  if let Some(binding)=&mut self.binding{if !binding.terminal_is_empty(){return binding.close_step(grant_child(grant)).map(RetainedCloneStep::Progress)}self.binding=None;return Ok(RetainedCloneStep::Progress(receipt(demand)))}
  if !self.generic.terminal_is_empty(){return self.generic.close_step(grant_child(grant)).map(RetainedCloneStep::Progress)}
  if let Some(record)=&mut self.record{if !record.terminal_is_empty(){return record.close_step(grant_child(grant))}self.record=None;return Ok(RetainedCloneStep::Progress(receipt(demand)))}
  self.closed=true;Ok(RetainedCloneStep::Complete(receipt(demand)))
 }
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> protocol::OpBinaryReceivingCursor<M> for OriginalOperationReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands_inner(source,body)}
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.step_inner(source,native,grant)}
 fn is_ready(&self)->bool{self.phase==6&&self.closing==0&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let demand=metadata::<M>();if self.phase!=6||self.closing!=0||recipient.is_some()||self.output.value.is_none()||!funded(grant,demand){return Ok(Default::default())}*recipient=self.output.value.take();Ok(receipt(demand))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close(grant)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.output.terminal_is_empty()&&self.binding.is_none()&&self.generic.terminal_is_empty()&&self.record.is_none()}
}
struct OperationRetirement<M:DslOwnedReceivingVariants+BorrowedDslVariants>(ManuallyDrop<OriginalOperationReceiving<M>>);
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetireOwned for OriginalOperationReceiving<M>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OperationRetirement(ManuallyDrop::new(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OperationRetirement<M>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetirementCursor for OperationRetirement<M>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> Drop for OperationRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original operation retirement lost its retained fields");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}
