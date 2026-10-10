//! 🧾️ One original History operation retains its actual native fields through nested granted decoding and paid typed receiving custody.
use super::OpPayload;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::ManuallyDrop;
use ::protocol::codec::RetainedWireFieldDecode;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedHistoryOpDecodeStep{Blocked,Progress,Ready,Closed}
pub struct RetainedHistoryOpDecode{source_identity:(usize,usize),original_range:(usize,usize),position:usize,flags:u8,phase:u8,partial:ManuallyDrop<Option<OpPayload>>,field:ManuallyDrop<Option<RetainedWireFieldDecode>>,field_range:(usize,usize),receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,closed:bool}
impl RetainedHistoryOpDecode{
 /// 🎟️ Admits the original borrowed payload and real typed partial recipient without native allocation.
 pub fn admit(source:&[u8],original_range:(usize,usize),grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{
  source.get(original_range.0..original_range.1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation escaped its source"))?;
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  Ok(Some(Self{source_identity:(source.as_ptr()as usize,source.len()),original_range,position:original_range.0,flags:0,phase:0,partial:ManuallyDrop::new(Some(OpPayload{text:None,binary:None})),field:ManuallyDrop::new(None),field_range:(0,0),receipt:Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()})),closing:false,closed:false}))
 }
 fn validate_source(&self,source:&[u8])->Result<(),ValueError>{if (source.as_ptr()as usize,source.len())!=self.source_identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original History operation source changed"))}Ok(())}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn is_ready(&self)->bool{!self.closing&&!self.closed&&self.phase==5&&self.receipt.is_none()}
 fn field_input<'a>(&self,source:&'a[u8])->&'a[u8]{&source[self.field_range.0..self.field_range.1]}
 fn record(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original History operation receipt exceeded its original grant").with_retained_progress(progress))}Ok(())}
 /// 📐️ Quotes the next genuine field frontier plus its actual typed parent depth before mutation.
 pub fn demands(&self,source:&[u8],maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  self.validate_source(source)?;if self.receipt.is_some()||self.closed{return Ok(Default::default())}
  if let Some(field)=self.field.as_ref(){let mut demand=if self.closing{field.close_demands()}else{field.demands(self.field_input(source),maximum_copy_bytes)?};demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original History operation depth overflow"))?;return Ok(demand)}
  Ok(RetirementDemand{depth:if !self.closing&&(self.phase==1||(self.phase==3&&self.flags&2!=0)){2}else{1},..Default::default()})
 }
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
 fn length_range(&self,source:&[u8])->Result<(usize,usize),ValueError>{
  let mut position=self.position;let mut length=0u64;let mut complete=false;for shift in(0..70).step_by(7){let byte=*source.get(position).filter(|_|position<self.original_range.1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation length is truncated"))?;position+=1;if shift==63&&byte>1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation length overflow"))}length|=u64::from(byte&127)<<shift;if byte&128==0{complete=true;break}}if !complete{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation length overflow"))}
  let length=usize::try_from(length).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original History operation extent overflow"))?;let end=position.checked_add(length).filter(|end|*end<=self.original_range.1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation field is truncated"))?;Ok((position,end))
 }
 /// 🧵️ Advances exactly one funded original header, physical field piece or typed receiving transfer.
 pub fn step(&mut self,source:&[u8],grant:RetainedCloneGrant)->Result<RetainedHistoryOpDecodeStep,ValueError>{
  self.validate_source(source)?;if self.receipt.is_some()||self.closing||self.closed{return Ok(RetainedHistoryOpDecodeStep::Blocked)}if self.is_ready(){return Ok(RetainedHistoryOpDecodeStep::Ready)}
  let demand=self.demands(source,grant.maximum_copy_bytes)?;if !Self::permits(grant,demand){return Ok(RetainedHistoryOpDecodeStep::Blocked)}
  if self.field.is_some(){
   if self.field.as_ref().unwrap().terminal_is_empty(){if self.phase==4&&self.position!=self.original_range.1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation has trailing payload bytes"))}drop(self.field.take());self.phase+=1;self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(RetainedHistoryOpDecodeStep::Progress)}
   let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let range=self.field_range;
   let result=if self.field.as_ref().unwrap().is_ready(){if self.phase==2{self.field.as_mut().unwrap().take_text_into(&mut self.partial.as_mut().unwrap().text,child_grant)}else{self.field.as_mut().unwrap().take_into(&mut self.partial.as_mut().unwrap().binary,child_grant)}}else{self.field.as_mut().unwrap().advance(&source[range.0..range.1],child_grant)};
   if let Some((_,progress))=self.field.as_mut().unwrap().take_receipt(){self.record(grant,progress)?}result?;return Ok(if self.receipt.is_some(){RetainedHistoryOpDecodeStep::Progress}else{RetainedHistoryOpDecodeStep::Blocked})
  }
  match self.phase{
   0=>{let flags=*source.get(self.position).filter(|_|self.position<self.original_range.1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation tag is absent"))?;if flags&3==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation has no payload"))}self.flags=flags;self.position+=1;self.phase=if flags&1!=0{1}else{3};},
   1|3=>{if self.phase==3&&self.flags&2==0{self.phase=5;}else{let range=self.length_range(source)?;*self.field=Some(RetainedWireFieldDecode::new(&source[range.0..range.1],self.phase==1));self.field_range=range;self.position=range.1;self.phase+=1;}},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original History operation phase has no field")),
  }
  if self.phase==5&&self.position!=self.original_range.1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original History operation has trailing payload bytes"))}
  self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryOpDecodeStep::Progress)
 }
 /// 📤️ Moves the exact ready original payload into the caller's genuine empty typed slot.
 pub fn take_ready_into(&mut self,recipient:&mut Option<OpPayload>,grant:RetainedCloneGrant)->Result<RetainedHistoryOpDecodeStep,ValueError>{
  if !self.is_ready()||recipient.is_some()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedHistoryOpDecodeStep::Blocked)}*recipient=self.partial.take();self.closed=true;self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryOpDecodeStep::Closed)
 }
 /// 🧳️ Returns the original partially decoded native fields after the child cursor has its paid close.
 pub fn take_partial_into(&mut self,recipient:&mut Option<OpPayload>,grant:RetainedCloneGrant)->Result<RetainedHistoryOpDecodeStep,ValueError>{
  if !self.closing||self.closed||self.field.is_some()||self.receipt.is_some()||recipient.is_some()||self.partial.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedHistoryOpDecodeStep::Blocked)}*recipient=self.partial.take();self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryOpDecodeStep::Progress)
 }
 pub fn cancel(&mut self){self.closing=true;if let Some(field)=self.field.as_mut(){field.cancel()}}
 /// ♻️ Closes the exact child under original nested authority, then waits for its real partial receiver.
 pub fn close_step(&mut self,source:&[u8],grant:RetainedCloneGrant)->Result<RetainedHistoryOpDecodeStep,ValueError>{
  self.validate_source(source)?;if self.receipt.is_some()||!self.closing{return Ok(RetainedHistoryOpDecodeStep::Blocked)}if self.closed{return Ok(RetainedHistoryOpDecodeStep::Closed)}let demand=self.demands(source,grant.maximum_copy_bytes)?;if !Self::permits(grant,demand){return Ok(RetainedHistoryOpDecodeStep::Blocked)}
  if self.field.is_some(){if self.field.as_ref().unwrap().terminal_is_empty(){drop(self.field.take());self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(RetainedHistoryOpDecodeStep::Progress)}let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let result=self.field.as_mut().unwrap().close_step(child_grant);if let Some((_,progress))=self.field.as_mut().unwrap().take_receipt(){self.record(grant,progress)?}result?;return Ok(if self.receipt.is_some(){RetainedHistoryOpDecodeStep::Progress}else{RetainedHistoryOpDecodeStep::Blocked})}
  if self.partial.is_some(){return Ok(RetainedHistoryOpDecodeStep::Blocked)}self.closed=true;self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryOpDecodeStep::Closed)
 }
 pub fn partial_is_receivable(&self)->bool{self.closing&&!self.closed&&self.field.is_none()&&self.receipt.is_none()&&self.partial.is_some()}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.receipt.is_none()&&self.field.is_none()&&self.partial.is_none()}
}
impl Drop for RetainedHistoryOpDecode{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original History operation abandoned partial native field custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.field);ManuallyDrop::drop(&mut self.partial)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
