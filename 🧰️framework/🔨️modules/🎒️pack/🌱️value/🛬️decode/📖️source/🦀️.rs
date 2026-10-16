//! 📖️ Original paged record bytes feed one paid native cursor without source materialization.
use super::{RetainedRecordBodyCursor,RetainedRecordBodyToken,RetainedValueToken};
use semio_framework_replication_codec::PackLimits; use semio_framework_pack_format_retained::RetainedPackCloseStep;
use semio_framework_replication_wire_core::{RetainedWireByteSource,OriginalWireSourceIdentity};
use semio_framework_pack_error::PackRefusal;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueRefusalKind};
use std::mem::ManuallyDrop;
pub struct RetainedRecordBodyReadLimits{pub pack:PackLimits,pub maximum_symbol_utf8_bytes:usize,pub maximum_symbol_scalars:usize,pub maximum_allocation_bytes:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedRecordBodySourceStep{Blocked,Progress,Token,Complete,Fault,Closed}
pub struct RetainedRecordBodySourceDecode{
 source:OriginalWireSourceIdentity,range:(usize,usize),position:usize,sealed:bool,complete:bool,closing:bool,closed:bool,
 cursor:ManuallyDrop<Option<RetainedRecordBodyCursor>>,token:Option<RetainedRecordBodyToken>,fault:ManuallyDrop<Option<PackRefusal>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
}
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
fn failure(what:&'static str)->PackRefusal{PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"original-record-source",offset:0,detail:what}}
impl RetainedRecordBodySourceDecode{
 pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:RetainedRecordBodyReadLimits,grant:RetainedCloneGrant)->Result<Option<Self>,PackRefusal>{
  let identity=source.original_source_identity();if range.0>range.1||range.1>identity.extent{return Err(failure("original source range is invalid"))}
  if(range.1-range.0)as u128>u128::from(limits.pack.max_file_len){return Err(failure("original record body exceeds source ceiling"))}
  if grant.maximum_items==0||grant.maximum_depth<2{return Ok(None)}
  let symbols=limits.pack.max_symbols as usize;let cursor=RetainedRecordBodyCursor::try_new(limits.pack,symbols,limits.maximum_symbol_utf8_bytes,limits.maximum_symbol_scalars,limits.maximum_allocation_bytes)?;
  Ok(Some(Self{source:identity,range,position:0,sealed:false,complete:false,closing:false,closed:false,cursor:ManuallyDrop::new(Some(cursor)),token:None,fault:ManuallyDrop::new(None),receipt:Some((grant,metadata()))}))
 }
 fn validate<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<(),PackRefusal>{if self.source!=source.original_source_identity(){return Err(failure("original sealed source identity changed"))}Ok(())}
 fn publish(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),PackRefusal>{self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(failure("original source receipt exceeds supplied full grant"))}Ok(())}
 pub fn source_identity(&self)->OriginalWireSourceIdentity{self.source}
 pub fn original_range(&self)->(usize,usize){self.range}
 pub fn allocated_bytes(&self)->usize{self.cursor.as_ref().map_or(0,RetainedRecordBodyCursor::allocated_bytes)}
 pub fn consumed_bytes(&self)->usize{self.position}
 pub fn token(&self)->Option<&RetainedRecordBodyToken>{self.token.as_ref()}
 pub fn fault(&self)->Option<&PackRefusal>{self.fault.as_ref()}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn is_complete(&self)->bool{self.complete&&self.token.is_none()&&self.receipt.is_none()&&self.fault.is_none()}
 pub fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>{self.cursor.as_ref().ok_or_else(||failure("original symbol cursor is absent"))?.symbol_chars(symbol)}
 pub fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>{self.cursor.as_ref().ok_or_else(||failure("original symbol cursor is absent"))?.symbol_char(symbol,index)}
 pub fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>{self.cursor.as_ref().ok_or_else(||failure("original symbol cursor is absent"))?.symbol_utf8_bytes(symbol)}
 pub fn demands<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<RetirementDemand,PackRefusal>{
  self.validate(source)?;if self.receipt.is_some()||self.closed||self.closing||self.token.is_some()||self.complete||self.fault.is_some(){return Ok(Default::default())}
  let cursor=self.cursor.as_ref().ok_or_else(||failure("original cursor is absent"))?;
  let mut demand=if cursor.ingress_ready(){RetirementDemand{depth:1,..Default::default()}}else{cursor.source_step_demands()?};demand.depth=demand.depth.checked_add(1).ok_or_else(||failure("original source depth overflow"))?;Ok(demand)
 }
 pub fn step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,grant:RetainedCloneGrant)->Result<RetainedRecordBodySourceStep,PackRefusal>{
  self.validate(source)?;if self.receipt.is_some()||self.closed||self.closing||self.token.is_some()||self.complete{return Ok(RetainedRecordBodySourceStep::Blocked)}if self.fault.is_some(){return Ok(RetainedRecordBodySourceStep::Fault)}
  if grant.maximum_items==0||grant.maximum_depth<2{return Ok(RetainedRecordBodySourceStep::Blocked)}
  let demand=match self.demands(source){Ok(demand)=>demand,Err(error)=>{*self.fault=Some(error);self.publish(grant,metadata())?;return Ok(RetainedRecordBodySourceStep::Fault)}};if !permits(grant,demand){return Ok(RetainedRecordBodySourceStep::Blocked)}
  let cursor=self.cursor.as_mut().unwrap();let mut progress=metadata();let result=(||{if cursor.ingress_ready(){
   if self.position<self.range.1-self.range.0{let byte=source.byte_at(self.range.0+self.position).ok_or_else(||failure("original borrowed source byte is absent"))?;cursor.admit_byte(self.position as u64,byte).map_err(|_|failure("original record ingress refused its quoted byte"))?;self.position+=1;Ok(RetainedRecordBodySourceStep::Progress)}
   else{cursor.seal(self.position as u64)?;self.sealed=true;Ok(RetainedRecordBodySourceStep::Progress)}
  }else if demand.capacity_bytes>0{
   match cursor.reserve_allocation(grant.maximum_capacity_bytes){Ok(step)=>{progress.retained_capacity_bytes=step.allocated_bytes;Ok(RetainedRecordBodySourceStep::Progress)},Err(error)=>{progress.retained_capacity_bytes=error.allocated_bytes;Err(error.fault)}}
  }else{
   match cursor.grant(){Ok(token)=>{progress.copied_bytes=demand.copy_bytes;if let Some(token)=token{if matches!(token,RetainedRecordBodyToken::Value(RetainedValueToken::Complete{..})){self.complete=true}self.token=Some(token);Ok(if self.complete{RetainedRecordBodySourceStep::Complete}else{RetainedRecordBodySourceStep::Token})}else{Ok(RetainedRecordBodySourceStep::Progress)}},Err(error)=>Err(error)}
  }})();
  let status=match result{Ok(status)=>status,Err(error)=>{*self.fault=Some(error);RetainedRecordBodySourceStep::Fault}};self.publish(grant,progress)?;Ok(status)
 }
 pub fn take_token_into(&mut self,recipient:&mut Option<RetainedRecordBodyToken>,grant:RetainedCloneGrant)->Result<bool,PackRefusal>{if self.receipt.is_some()||recipient.is_some()||self.token.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}*recipient=self.token.take();self.publish(grant,metadata())?;Ok(true)}
 pub fn take_fault_into(&mut self,recipient:&mut Option<PackRefusal>,grant:RetainedCloneGrant)->Result<bool,PackRefusal>{if self.receipt.is_some()||recipient.is_some()||self.fault.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}*recipient=self.fault.take();self.closing=true;self.publish(grant,metadata())?;Ok(true)}
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_demands(&self)->Result<RetirementDemand,PackRefusal>{
  if self.closed||self.receipt.is_some()||!self.closing||self.token.is_some()||self.fault.is_some(){return Ok(Default::default())}
  let mut demand=match self.cursor.as_ref(){Some(cursor)if !cursor.terminal_is_empty()=>cursor.source_close_demands()?,_=>RetirementDemand{depth:1,..Default::default()}};demand.depth=demand.depth.checked_add(1).ok_or_else(||failure("original source close depth overflow"))?;Ok(demand)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedRecordBodySourceStep,PackRefusal>{
  if self.closed||self.receipt.is_some()||!self.closing||self.token.is_some()||self.fault.is_some(){return Ok(RetainedRecordBodySourceStep::Blocked)}let demand=self.close_demands()?;if !permits(grant,demand){return Ok(RetainedRecordBodySourceStep::Blocked)}
  let mut progress=metadata();if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){drop(self.cursor.take())}else{match cursor.close_step(1,grant.maximum_release_bytes)?{RetainedPackCloseStep::Pending{released_bytes,..}=>progress.released_bytes=released_bytes,RetainedPackCloseStep::Complete=>{}}}self.publish(grant,progress)?;return Ok(RetainedRecordBodySourceStep::Progress)}
  self.closed=true;self.publish(grant,progress)?;Ok(RetainedRecordBodySourceStep::Closed)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.cursor.is_none()&&self.token.is_none()&&self.fault.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedRecordBodySourceDecode{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original record source abandoned native cursor or receipt");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.fault)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
