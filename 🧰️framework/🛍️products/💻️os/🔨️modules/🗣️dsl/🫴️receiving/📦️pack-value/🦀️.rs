//! 📦️ Plain pack-value operations keep their document parser, decoded value and typed output across original native turns.
use super::*;
use semio_framework_value::{DslValue,FromValue};
use semio_os_pack::pack_rt::{renormalize_whole_number_floats,take_received_wire_value,wire_value_spec};
/// 🧳️ Receives one stored `pack_rt` document in bounded steps, then binds its decoded value to the typed operation under an honest quote.
pub struct OriginalPackValueReceiving<M:FromValue+RetireOwned>{
 source:(usize,usize),range:(usize,usize),record:Option<RetainedRecordReceiving>,value:OwnedSlot<DslValue>,output:OwnedSlot<M>,phase:u8,closing:u8,closed:bool,failed:bool,
}
/// ⚖️ Retained bytes of one decoded value tree: every node header plus its string, byte and key payloads.
pub fn decoded_value_retained_bytes(value:&DslValue)->usize{
 let node=std::mem::size_of::<DslValue>();
 node.saturating_add(match value{
  DslValue::String(text)=>text.len(),
  DslValue::Bytes(bytes)=>bytes.len(),
  DslValue::Array(items)=>items.iter().fold(0usize,|total,item|total.saturating_add(decoded_value_retained_bytes(item))),
  DslValue::Object(fields)=>fields.iter().fold(0usize,|total,(key,item)|total.saturating_add(std::mem::size_of::<String>()).saturating_add(key.len()).saturating_add(decoded_value_retained_bytes(item))),
  _=>0,
 })
}
/// ⚖️ The conversion quote: the decoded value's retained bytes plus the typed output header.
pub fn pack_value_conversion_quote<M>(value:&DslValue)->RetirementDemand{RetirementDemand{copy_bytes:decoded_value_retained_bytes(value).saturating_add(std::mem::size_of::<Option<M>>()),depth:1,..Default::default()}}
impl<M:FromValue+RetireOwned> OriginalPackValueReceiving<M>{
 /// 🎟️ Preserves the full original source and the exact document range without parsing or allocating.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let mut work=0;
  let result=(||{
   if range.0>range.1||range.1>source.len(){return Err(invalid("original pack value escaped its full source"))}
   if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
   work=1;
   Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,record:None,value:Default::default(),output:Default::default(),phase:0,closing:0,closed:false,failed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
  })();
  RetainedWorkOutcome{result,work}
 }
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original pack value source identity changed"))}Ok(())}
 fn demands_inner(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  if self.closing!=0{return self.close_demand(body)}
  let mut demand=match self.phase{
   0=>RetainedRecordReceiving::document_birth_demand(),
   1=>{let record=self.record.as_ref().ok_or_else(||ValueError::invariant("original pack value record is absent"))?;if record.is_ready(){metadata::<DslValue>()}else{record.demands(source)?}},
   2=>metadata::<DslValue>(),
   3=>pack_value_conversion_quote::<M>(self.value.value.as_ref().ok_or_else(||ValueError::invariant("original pack value is absent"))?),
   _=>RetirementDemand{depth:1,..Default::default()},
  };
  demand.depth+=1;
  Ok(demand)
 }
 fn step_action(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  let grant=effect.grant();let demand=self.demands_inner(source,body)?;
  if self.failed||self.phase>=4||self.closing!=0||!funded(grant,demand){return Ok(Default::default())}
  let child=grant_child(grant);
  match self.phase{
   0=>{let turn=RetainedRecordReceiving::admit_document(source,(self.range.0,self.range.1),wire_value_spec(),semio_os_pack::PackLimits::default(),child);perform(effect,turn.work)?;Ok(match turn.result?{Some((owner,progress))=>{self.record=Some(owner);self.phase=1;progress},None=>Default::default()})},
   1=>{
    let record=self.record.as_mut().ok_or_else(||ValueError::invariant("original pack value record is absent"))?;
    if record.is_ready(){self.phase=2;perform(effect,1)?;return Ok(receipt(demand))}
    record.step(source,native,&mut effect.nested(child)).result
   },
   2=>{let record=self.record.as_mut().ok_or_else(||ValueError::invariant("original pack value record is absent"))?;let turn=take_received_wire_value(record,&mut self.value.value,child);perform(effect,turn.work)?;let progress=turn.result?;if self.value.value.is_some(){self.phase=3}Ok(progress)},
   _=>{
    perform(effect,1)?;
    let value=self.value.value.take().ok_or_else(||ValueError::invariant("original pack value is absent"))?;
    let converted=M::from_value(renormalize_whole_number_floats(value)).map_err(|error|error.with_retained_progress(receipt(demand)))?;
    *self.output.value=Some(converted);self.phase=4;
    Ok(receipt(demand))
   },
  }
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}
  if self.closing==0{return Ok(metadata::<()>())}
  if !self.output.terminal_is_empty(){return self.output.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if !self.value.terminal_is_empty(){return self.value.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(record)=&self.record{if !record.terminal_is_empty(){return record.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let child=grant_child(grant);
  let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  let demand=match self.close_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return settle(effect,Err(error))};
  if !funded(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  if self.closing==0{if let Err(error)=perform(effect,1){return settle(effect,Err(error))}self.closing=1;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if !self.output.terminal_is_empty(){return stepped(self.output.close_step(&mut effect.nested(child)))}
  if !self.value.terminal_is_empty(){return stepped(self.value.close_step(&mut effect.nested(child)))}
  if let Some(record)=&mut self.record{
   if !record.terminal_is_empty(){return match record.close_step(grant.maximum_copy_bytes,&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
   if let Err(error)=perform(effect,1){return settle(effect,Err(error))}
   self.record=None;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))
  }
  if let Err(error)=perform(effect,1){return settle(effect,Err(error))}
  self.closed=true;settle(effect,Ok(RetainedCloneStep::Complete(receipt(demand))))
 }
}
impl<M:FromValue+RetireOwned+OpBinaryReceiving<DomainFailure=ValueError,Receiving=OriginalPackValueReceiving<M>>> OpBinaryReceivingCursor<M> for OriginalPackValueReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands_inner(source,body)}
 fn step_in(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let result=self.step_action(source,native,body,effect);if result.is_err(){self.failed=true}RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn domain_failure(&self)->Option<&ValueError>{None}
 fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn domain_failure_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
 fn take_domain_failure_into(&mut self,recipient:&mut Option<ValueError>,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{RetainedWorkOutcome{result:if recipient.is_some(){Err(invalid("pack value domain failure requires an empty original recipient"))}else{Ok(Default::default())},work:0}}
 fn is_ready(&self)->bool{self.phase==4&&self.closing==0&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  let demand=metadata::<M>();
  if self.failed||self.phase!=4||self.closing!=0||recipient.is_some()||self.output.value.is_none()||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  *recipient=self.output.value.take();RetainedWorkOutcome{result:Ok(receipt(demand)),work:1}
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close(effect)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.output.terminal_is_empty()&&self.value.terminal_is_empty()&&self.record.is_none()}
}
/// ♻️ Retires the pack-value receiving's remaining owners through its paid close.
#[doc(hidden)]
pub struct PackValueRetirement<M:FromValue+RetireOwned>(ManuallyDrop<OriginalPackValueReceiving<M>>);
impl<M:FromValue+RetireOwned> RetireOwned for OriginalPackValueReceiving<M>{
 type RetirementCursor=PackValueRetirement<M>;
 fn retirement_seed(self)->Self::RetirementCursor{PackValueRetirement(ManuallyDrop::new(self))}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:FromValue+RetireOwned> PackValueRetirement<M>{
 fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{erased_cursor_turn(self.0.close(effect))}
}
impl<M:FromValue+RetireOwned> RetirementCursor for PackValueRetirement<M>{
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:FromValue+RetireOwned> Drop for PackValueRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original pack value retirement lost its retained fields");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}
/// 📦️ Implements a plain `ToValue`/`FromValue` pack-document operation through its actual bounded pack-value receiving capability.
#[macro_export]
macro_rules! original_pack_value_operation_receiving{
 ($operation:ty)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type DomainFailure=$crate::original_receiving::ValueError;
  type Receiving=$crate::original_receiving::OriginalPackValueReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{$crate::original_receiving::OriginalPackValueReceiving::admit(source,range,grant)}
 }};
}
