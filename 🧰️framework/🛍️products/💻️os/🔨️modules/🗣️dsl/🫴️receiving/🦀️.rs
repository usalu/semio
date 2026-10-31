//! 🫴️ Ordinal operations retain generic parsing and owned typed binding across original native turns.
pub use semio_framework_dsl_record::BorrowedDslVariants;
use semio_framework_dsl_record::{FieldValue,receiving::{DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot}};
pub use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError};
pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
pub use protocol::{OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::{RetainedCloneStep, RetirementDemand, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_os_pack::record::RetainedRecordReceiving;
use std::mem::{ManuallyDrop,size_of};
/// 🎞️ Implements an authored ordinal operation only through its actual owned receiving capability.
#[macro_export]
macro_rules! original_ordinal_operation_receiving{
 ($operation:ty)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{$crate::original_receiving::OriginalOperationReceiving::admit(source,range,grant)}
 }};
}
/// 🏷️ Resolves authored protocol tags at compile time and retains one-tag runtime selection.
#[macro_export]
macro_rules! original_tagged_operation_receiving{
 ($operation:ty,$protocol:expr)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{
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
 record:Option<RetainedRecordReceiving>,generic:OwnedSlot<FieldValue>,binding:Option<M::Cursor>,output:OwnedSlot<M>,closing:u8,closed:bool,failed:bool,
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn metadata<T>()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn grant_child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> OriginalOperationReceiving<M>{
 /// 🔭️ Observes the receiving phase without advancing it.
 pub fn phase(&self)->u8{self.phase}
 /// 🔭️ Borrows the generic field value once it has been received.
 pub fn generic_value(&self)->Option<&FieldValue>{self.generic.value.as_ref()}
 /// 🎟️ Preserves the full original source and exact operation range without parsing or allocating.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let mut work=0;let result=(||{
  if range.0>range.1||range.1>source.len(){return Err(invalid("original operation escaped its full source"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  work=1;Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,position:range.0,phase:0,tag:0,shift:0,tags:None,ordinal:0,record:None,generic:Default::default(),binding:None,output:Default::default(),closing:0,closed:false,failed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
 })();RetainedWorkOutcome{result,work}}
 /// 🏷️ Borrows immutable authored tags without a runtime metadata owner.
 pub fn admit_tagged(source:&[u8],range:(usize,usize),tags:&'static[u64],grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{if tags.len()!=M::VARIANTS.len(){return RetainedWorkOutcome{result:Err(invalid("original operation tag declaration differs from variants")),work:0}}let turn=Self::admit(source,range,grant);RetainedWorkOutcome{result:turn.result.map(|admitted|admitted.map(|(mut owner,p)|{owner.tags=Some(tags);(owner,p)})),work:turn.work}}
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
 fn post(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,turn:RetainedWorkOutcome<RetainedCloneProgress>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=native.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work});RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work}}
 fn fixed_post(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,turn:RetainedWorkOutcome<RetainedCloneProgress>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{native.commit_work(turn.work).expect("one concrete operation action consumes only its held native work");Self::post(native,grant,turn)}
 fn step_inner(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let demand=match self.demands_inner(source,body){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};
  if self.failed||self.phase==6||self.closing!=0||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  if self.phase<=3||self.phase==4&&self.record.as_ref().unwrap().is_ready(){match native.prepare_work(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(Default::default()),work:0},Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}}}
  let turn=match self.phase{
   4 if !self.record.as_ref().unwrap().is_ready()=>self.record.as_mut().unwrap().step(source,native,grant_child(grant)),
   5 if !self.binding.as_ref().unwrap().is_ready()=>self.binding.as_mut().unwrap().step(&mut self.generic.value,native,body,grant_child(grant)),
   3=>{let result=self.spec();match result{Err(e)=>RetainedWorkOutcome{result:Err(e.into()),work:0},Ok((_,spec))=>{let turn=RetainedRecordReceiving::admit(source,(self.position,self.range.1),spec,semio_framework_os_pack::PackLimits::default(),grant_child(grant));let result=turn.result.map(|owner|match owner{Some((owner,p))=>{self.record=Some(owner);self.phase=4;p},None=>Default::default()});Self::fixed_post(native,grant,RetainedWorkOutcome{result,work:turn.work})}}},
   4=>{let mut original=None;let turn=self.record.as_mut().unwrap().take_into(&mut original,grant_child(grant));if let Some(value)=original{*self.generic.value=Some(FieldValue::Record(value));self.phase=5}Self::fixed_post(native,grant,turn)},
   5=>{let turn=self.binding.as_mut().unwrap().take_into(&mut self.output.value,grant_child(grant));if self.output.value.is_some(){self.phase=6}Self::post(native,grant,turn)},
   _=>{
    if let Err(e)=native.checkpoint(){native.commit_work(0).expect("canceled concrete operation action consumes no held work");return RetainedWorkOutcome{result:Err(e.into()),work:0}}
    let mut progress=RetainedCloneProgress::default();let result=(||{
     match self.phase{
      0=>{if source.get(self.position)!=Some(&1)||self.position>=self.range.1{return Err(invalid("original operation binary format is invalid"))}self.position+=1;self.phase=1;progress=receipt(demand)},
      1=>{let byte=source.get(self.position).copied().filter(|_|self.position<self.range.1).ok_or_else(||invalid("original operation ordinal is truncated"))?;if self.shift==63&&byte>1{return Err(invalid("original operation ordinal overflows u64"))}self.tag|=u64::from(byte&127)<<self.shift;self.position+=1;progress=receipt(demand);if byte&128==0{if self.shift>0&&byte==0{return Err(invalid("original operation ordinal is not minimal"))}self.phase=2}else{self.shift=self.shift.checked_add(7).filter(|value|*value<=63).ok_or_else(||invalid("original operation ordinal overflows u64"))?;}},
      2=>{if let Some(tags)=self.tags{let tag=tags.get(self.ordinal).ok_or_else(||invalid("original operation tag is not declared"))?;if *tag!=self.tag{self.ordinal+=1;progress=receipt(demand);return Ok(progress)}}else{self.ordinal=usize::try_from(self.tag).map_err(|_|invalid("original operation ordinal exceeds address space"))?;progress=receipt(demand)}let(keyword,_)=self.spec()?;self.binding=Some(M::receiving_cursor(keyword)?);self.phase=3;progress=receipt(demand)},
      _=>return Err(invalid("original operation phase is invalid")),
     }Ok(progress)
    })();let result=result.map_err(|e|e.with_retained_progress(progress));Self::fixed_post(native,grant,RetainedWorkOutcome{result,work:1})
   }
  };if turn.result.is_err(){self.failed=true}turn
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}if self.closing==0{return Ok(metadata::<()>())}if !self.output.terminal_is_empty(){return self.output.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(binding)=&self.binding{if !binding.terminal_is_empty(){return binding.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.generic.terminal_is_empty(){return self.generic.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(record)=&self.record{if !record.terminal_is_empty(){return record.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn close(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.closed{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let demand=match self.close_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funded(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.closing==0{self.closing=1;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:1}}
  if !self.output.terminal_is_empty(){let turn=self.output.close_step(grant_child(grant));return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
  if let Some(binding)=&mut self.binding{if !binding.terminal_is_empty(){let turn=binding.close_step(grant_child(grant));return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}std::mem::forget(self.binding.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:1}}
  if !self.generic.terminal_is_empty(){let turn=self.generic.close_step(grant_child(grant));return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
  if let Some(record)=&mut self.record{if !record.terminal_is_empty(){let turn=record.close_step(grant_child(grant));return RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())),work:turn.work}}self.record=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:1}}
  self.closed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(demand))),work:1}
 }
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> protocol::OpBinaryReceivingCursor<M> for OriginalOperationReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands_inner(source,body)}
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{self.step_inner(source,native,body,grant)}
 fn is_ready(&self)->bool{self.phase==6&&self.closing==0&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{let demand=metadata::<M>();if self.failed||self.phase!=6||self.closing!=0||recipient.is_some()||self.output.value.is_none()||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*recipient=self.output.value.take();RetainedWorkOutcome{result:Ok(receipt(demand)),work:1}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{self.close(grant)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.output.terminal_is_empty()&&self.binding.is_none()&&self.generic.terminal_is_empty()&&self.record.is_none()}
}
struct OperationRetirement<M:DslOwnedReceivingVariants+BorrowedDslVariants>(ManuallyDrop<OriginalOperationReceiving<M>>);
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetireOwned for OriginalOperationReceiving<M>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OperationRetirement(ManuallyDrop::new(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OperationRetirement<M>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetirementCursor for OperationRetirement<M>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{{let turn=self.0.close(grant);match turn.result{Ok(RetainedCloneStep::Progress(progress))=>RetirementStep::Progress(progress,turn.work),Ok(RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> Drop for OperationRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original operation retirement lost its retained fields");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}
