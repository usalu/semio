//! 🔢️ Original field7 bytes enter the existing final native column under one captured receiving policy.
use crate::ToolRunNativeTickPreparation;
use pack::record::{RetainedRecordBodySourceDecode,RetainedRecordBodyToken,RetainedValueRole,RetainedValueToken};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
type Witness=(usize,usize,Option<(u64,u64)>,(usize,usize));
type Receipt=Option<(RetainedCloneGrant,RetainedCloneProgress)>;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Field,Tag,Length,Bytes,Commit,Complete,Closed}
pub struct ToolRunEntityColumnReceiving{source:Witness,frame:usize,phase:Phase,remaining:usize,scalar:[u8;8],at:usize,receipt:Receipt,closing:bool}
fn witness(source:&RetainedRecordBodySourceDecode)->Witness{let s=source.source_identity();(s.address,s.extent,s.authority,source.original_range())}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original entity column source or token is invalid")}
fn limit()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original entity column exceeds captured receiving allocation or item ceiling")}
fn metadata()->RetirementDemand{RetirementDemand{depth:2,..Default::default()}}
fn funded(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_depth>=d.depth&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes}
fn progress()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl ToolRunEntityColumnReceiving{
 pub fn admit(source:&RetainedRecordBodySourceDecode,frame:&mut ToolRunNativeTickPreparation,g:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  if source.receipt().is_some()||!funded(g,metadata()){return Ok(None)}let address=frame.native_address().ok_or_else(fault)?;if !frame.begin_entity_issuer(source)?{return Ok(None)}let p=progress();let receiver=Self{source:witness(source),frame:address,phase:Phase::Field,remaining:0,scalar:[0;8],at:0,receipt:Some((g,p)),closing:false};Ok(Some((receiver,p)))
 }
 fn validate(&self,source:&RetainedRecordBodySourceDecode,frame:&ToolRunNativeTickPreparation)->Result<(),ValueError>{if witness(source)!=self.source{return Err(fault())}frame.entity_column(source,self.frame)?;Ok(())}
 fn allocation(&self,source:&RetainedRecordBodySourceDecode,frame:&ToolRunNativeTickPreparation,birth:usize)->Result<(),ValueError>{
  let column=frame.entity_column(source,self.frame)?;let total=source.allocated_bytes().checked_add(ToolRunNativeTickPreparation::birth_bytes()).and_then(|bytes|bytes.checked_add(column.allocated_bytes())).and_then(|bytes|bytes.checked_add(birth)).ok_or_else(limit)?;if total>frame.entity_limits().tick.maximum_total_allocation_bytes as usize{return Err(limit())}Ok(())
 }
 pub fn receipt(&self)->Receipt{self.receipt}
 pub fn take_receipt(&mut self,source:&RetainedRecordBodySourceDecode,frame:&mut ToolRunNativeTickPreparation)->Result<Receipt,ValueError>{if self.receipt.is_none(){return Ok(None)}self.validate(source,frame)?;if source.receipt().is_some(){return Ok(None)}if self.phase==Phase::Closed{frame.end_entity_issuer(source,self.frame)?}Ok(self.receipt.take())}
 pub fn is_complete(&self)->bool{self.phase==Phase::Complete&&self.receipt.is_none()}
 pub fn demands(&self,source:&RetainedRecordBodySourceDecode,frame:&ToolRunNativeTickPreparation,pending:&Option<RetainedRecordBodyToken>)->Result<RetirementDemand,ValueError>{
  self.validate(source,frame)?;if self.receipt.is_some()||source.receipt().is_some()||self.closing||matches!(self.phase,Phase::Complete|Phase::Closed){return Ok(Default::default())}self.allocation(source,frame,0)?;let column=frame.entity_column(source,self.frame)?;let mut d=metadata();
  if self.phase==Phase::Length{if let Some(RetainedRecordBodyToken::Value(RetainedValueToken::Unsigned{role:RetainedValueRole::BytesLength,value}))=pending{let bytes=usize::try_from(*value).map_err(|_|limit())?;if bytes%8!=0||bytes/8>frame.entity_limits().tick.maximum_items as usize||bytes/8>262144{return Err(limit())}}}
  if self.phase==Phase::Bytes{if !column.has_reserved_slot(){return Ok(Default::default())}d.copy_bytes=1}
  if self.phase==Phase::Commit{d.depth=column.next_push_depth_demand().map_err(|_|fault())?+1}Ok(d)
 }
 pub fn reserve_demands(&self,source:&RetainedRecordBodySourceDecode,frame:&ToolRunNativeTickPreparation)->Result<RetirementDemand,ValueError>{
  self.validate(source,frame)?;if self.receipt.is_some()||source.receipt().is_some()||self.closing||self.phase!=Phase::Bytes||self.at!=0{return Ok(Default::default())}let column=frame.entity_column(source,self.frame)?;if column.has_reserved_slot(){return Ok(Default::default())}let d=RetirementDemand{copy_bytes:column.next_reserve_copy_byte_demand().map_err(|_|limit())?,capacity_bytes:column.next_allocation_bytes().map_err(|_|limit())?,depth:column.next_reserve_depth_demand().map_err(|_|limit())?+1,..Default::default()};self.allocation(source,frame,d.capacity_bytes)?;Ok(d)
 }
 pub fn reserve_step(&mut self,source:&RetainedRecordBodySourceDecode,frame:&mut ToolRunNativeTickPreparation,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  let d=self.reserve_demands(source,frame)?;if d.depth==0||!funded(g,d){return Ok(None)}let result=frame.entity_column_mut(source,self.frame)?.reserve_one_funded(g);let p=match result{Ok(p)=>p,Err(error)=>{let p=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes>0),retained_capacity_bytes:error.allocated_bytes,..Default::default()};self.receipt=Some((g,p));return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(p))}};self.receipt=Some((g,p));if !p.fits(g){return Err(limit().with_retained_progress(p))}Ok(Some(p))
 }
 pub fn receive(&mut self,source:&RetainedRecordBodySourceDecode,frame:&ToolRunNativeTickPreparation,pending:&mut Option<RetainedRecordBodyToken>,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  let d=self.demands(source,frame,pending)?;if d.depth==0||self.phase==Phase::Commit||!funded(g,d){return Ok(None)}let Some(RetainedRecordBodyToken::Value(token))=pending.as_ref().copied()else{return Ok(None)};let mut p=progress();
  match(self.phase,token){
   (Phase::Field,RetainedValueToken::Unsigned{role:RetainedValueRole::FieldId,value:7})=>self.phase=Phase::Tag,
   (Phase::Tag,RetainedValueToken::Tag{value:8,..})=>self.phase=Phase::Length,
   (Phase::Length,RetainedValueToken::Unsigned{role:RetainedValueRole::BytesLength,value})=>{self.remaining=usize::try_from(value).map_err(|_|limit())?;self.phase=if self.remaining==0{Phase::Complete}else{Phase::Bytes}},
   (Phase::Bytes,RetainedValueToken::Byte(byte))if self.remaining>0=>{self.scalar[self.at]=byte;self.at+=1;self.remaining-=1;p.copied_bytes=1;if self.at==8{self.phase=Phase::Commit}},
   _=>return Err(fault()),
  }
  pending.take();self.receipt=Some((g,p));Ok(Some(p))
 }
 pub fn commit_step(&mut self,source:&RetainedRecordBodySourceDecode,frame:&mut ToolRunNativeTickPreparation,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  let d=self.demands(source,frame,&None)?;if self.phase!=Phase::Commit||d.depth==0||!funded(g,d){return Ok(None)}let mut original=Some(u64::from_le_bytes(self.scalar));let adopted=frame.entity_column_mut(source,self.frame)?.adopt_reserved(&mut original).map_err(|_|fault())?;if !adopted.progressed||original.is_some(){return Err(fault())}self.at=0;self.phase=if self.remaining==0{Phase::Complete}else{Phase::Bytes};let p=progress();self.receipt=Some((g,p));Ok(Some(p))
 }
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_step(&mut self,source:&RetainedRecordBodySourceDecode,frame:&mut ToolRunNativeTickPreparation,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.receipt.is_some()||source.receipt().is_some()||self.phase==Phase::Closed||(!self.closing&&self.phase!=Phase::Complete)||!funded(g,metadata()){return Ok(None)}self.validate(source,frame)?;self.at=0;self.remaining=0;self.phase=Phase::Closed;let p=progress();self.receipt=Some((g,p));Ok(Some(p))
 }
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.at==0&&self.remaining==0&&self.receipt.is_none()}
}
impl Drop for ToolRunEntityColumnReceiving{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original entity issuer abandoned its final native column or receipt")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
