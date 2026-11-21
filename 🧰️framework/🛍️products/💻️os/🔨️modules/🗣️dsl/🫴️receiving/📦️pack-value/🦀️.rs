//! 📦️ Pack-value operations keep their parser, decoded value and typed output across original native turns.
use super::*;
use crate::{protocol_record, tagged_value_binary};
use semio_framework_value::{DslValue,FromValue};
use semio_os_pack::format::{RetainedVarintCursor, RetainedVarintStep};
use semio_os_pack::pack_rt::{renormalize_whole_number_floats,take_received_wire_value,wire_value_spec};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
#[path = "🏷️tagged/🦀️.rs"]
mod tagged;
pub use tagged::OriginalTaggedPackValueReceiving;

enum PackOperationTag {
    Format,
    Varint(RetainedVarintCursor),
    Select { tag: u64, line: usize },
    Selected(&'static str),
}

struct TaggedPackOperation {
    protocol: &'static str,
    tag: PackOperationTag,
}

impl TaggedPackOperation {
    fn selected(&self) -> Option<&'static str> {
        match self.tag {
            PackOperationTag::Selected(kind) => Some(kind),
            _ => None,
        }
    }

    fn step(&mut self, source: &[u8], range: &mut (usize, usize)) -> Result<(), ValueError> {
        match &mut self.tag {
            PackOperationTag::Format => {
                if range.0 == range.1 || source[range.0] != tagged_value_binary::OP_BINARY_FORMAT {
                    return Err(invalid("original tagged pack operation format is invalid"));
                }
                range.0 += 1;
                self.tag = PackOperationTag::Varint(RetainedVarintCursor::default());
            }
            PackOperationTag::Varint(cursor) => {
                if range.0 == range.1 {
                    return Err(invalid("original tagged pack operation tag is truncated"));
                }
                let step = cursor.admit(source[range.0], range.0 as u64)
                    .map_err(|_| invalid("original tagged pack operation tag is noncanonical"))?;
                range.0 += 1;
                if let RetainedVarintStep::Complete(tag) = step {
                    self.tag = PackOperationTag::Select { tag, line: 0 };
                }
            }
            PackOperationTag::Select { tag, line } => {
                let record = protocol_record::next_record_line(self.protocol, line)
                    .ok_or_else(|| invalid("original tagged pack operation tag is not declared"))?;
                if let Some((kind, record_tag)) = record {
                    if record_tag == *tag {
                        self.tag = PackOperationTag::Selected(kind);
                    }
                }
            }
            PackOperationTag::Selected(_) => return Err(ValueError::invariant("original tagged pack operation was already selected")),
        }
        Ok(())
    }
}

/// 🧳️ Receives one complete pack document or tagged wire value across original turns before binding its typed operation.
pub struct OriginalPackValueReceiving<M:FromValue+RetireOwned>{
 source:(usize,usize),range:(usize,usize),tagged:Option<TaggedPackOperation>,record:Option<RetainedRecordReceiving>,value:OwnedSlot<DslValue>,output:OwnedSlot<M>,phase:u8,closing:u8,closed:bool,failed:bool,
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
   Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,tagged:None,record:None,value:Default::default(),output:Default::default(),phase:0,closing:0,closed:false,failed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
  })();
  RetainedWorkOutcome{result,work}
 }
 /// 🏷️ Preserves the full source and borrows the specification for an externally tagged wire value.
 fn admit_tagged_key(source:&[u8],range:(usize,usize),protocol:&'static str,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let turn=Self::admit(source,range,grant);
  RetainedWorkOutcome{result:turn.result.map(|admission|admission.map(|(mut receiving,progress)|{
   receiving.tagged=Some(TaggedPackOperation{protocol,tag:PackOperationTag::Format});
   (receiving,progress)
  })),work:turn.work}
 }
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original pack value source identity changed"))}Ok(())}
 fn demands_inner(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  if self.closing!=0{return self.close_demand(body)}
  let mut demand=match self.phase{
   0=>match &self.tagged{
    Some(tagged) if tagged.selected().is_none()=>metadata::<()>(),
    Some(_)=>RetainedRecordReceiving::birth_demand(),
    None=>RetainedRecordReceiving::document_birth_demand(),
   },
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
   0=>{
    let limits=semio_os_pack::PackLimits::default();
    let turn=if let Some(tagged)=self.tagged.as_mut(){
     if tagged.selected().is_none(){
      native.checkpoint()?;
      perform(effect,1)?;
      tagged.step(source,&mut self.range)?;
      return Ok(receipt(demand));
     }
     RetainedRecordReceiving::admit(source,self.range,wire_value_spec(),limits,child)
    }else{
     RetainedRecordReceiving::admit_document(source,self.range,wire_value_spec(),limits,child)
    };
    perform(effect,turn.work)?;
    Ok(match turn.result?{Some((owner,progress))=>{self.record=Some(owner);self.phase=1;progress},None=>Default::default()})
   },
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
 fn close_work(&self, body: usize) -> Result<u64, ValueError> {
  if self.closed { return Ok(0); }
  if self.closing == 0 { return Ok(1); }
  if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }
  if !self.value.terminal_is_empty() { return self.value.close_work_demand(body); }
  if let Some(record) = &self.record {
   return if record.terminal_is_empty() { Ok(1) } else { record.close_work_demand(body) };
  }
  Ok(1)
 }
 fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let child=grant_child(grant);
  let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  let demand=match self.close_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return settle(effect,Err(error))};
  if !funded(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  match self.close_work(grant.maximum_copy_bytes).and_then(|work| effect.remaining_work_units().map(|remaining| remaining >= work)) {
   Ok(true) => {}
   Ok(false) => return settle(effect, Ok(RetainedCloneStep::Progress(Default::default()))),
   Err(error) => return settle(effect, Err(error)),
  }
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
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_work(body)}
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
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.close_work(body)}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:FromValue+RetireOwned> Drop for PackValueRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original pack value retirement lost its retained fields");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}
/// 📦️ Implements an operation encoded as a pack document or an externally tagged pack wire value.
#[macro_export]
macro_rules! original_pack_value_operation_receiving{
 ($operation:ty $(, tagged_key = $protocol:expr)?)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type DomainFailure=$crate::original_receiving::ValueError;
  type Receiving=$crate::original_pack_value_operation_receiving!(@receiver Self $(, $protocol)?);
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{
   $crate::original_pack_value_operation_receiving!(@admit Self::Receiving, source, range, grant $(, $protocol)?)
  }
 }};
 (@receiver $operation:ty)=>{$crate::original_receiving::OriginalPackValueReceiving<$operation>};
 (@receiver $operation:ty, $protocol:expr)=>{$crate::original_receiving::OriginalTaggedPackValueReceiving<$operation>};
 (@admit $receiving:ty, $source:expr, $range:expr, $grant:expr)=>{<$receiving>::admit($source,$range,$grant)};
 (@admit $receiving:ty, $source:expr, $range:expr, $grant:expr, $protocol:expr)=>{<$receiving>::admit($source,$range,$protocol,$grant)};
}
