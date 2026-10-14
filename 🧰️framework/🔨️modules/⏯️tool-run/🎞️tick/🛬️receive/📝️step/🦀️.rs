//! 📝️ Typed step scalars and fixed9-byte argument groups consume the original native token recipient.
use crate::{ToolRunStep,ToolRunStepKind,ToolRunStepArg,TOOL_RUN_STEP_ARGS_MAX};
use pack::record::{RetainedRecordBodySourceDecode,RetainedRecordBodyToken,RetainedValueToken,RetainedValueContainer,RetainedValueRole};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Begin,Field,Tag,Scalar,Length,Bytes,Complete,Closed}
pub struct ToolRunStepReceiving{
 source:(usize,usize,Option<(u64,u64)>,(usize,usize)),phase:Phase,field:u8,seen:u8,fields:u8,argument_bytes:usize,argument_at:usize,argument:[u8;9],
 step:ManuallyDrop<Option<ToolRunStep>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,
}
fn witness(source:&RetainedRecordBodySourceDecode)->(usize,usize,Option<(u64,u64)>,(usize,usize)){let original=source.source_identity();(original.address,original.extent,original.authority,source.original_range())}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original typed step token or source is invalid")}
fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_depth>=d.depth&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes}
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl ToolRunStepReceiving{
 pub fn admit(source:&RetainedRecordBodySourceDecode,g:RetainedCloneGrant)->Option<(Self,RetainedCloneProgress)>{if !funds(g,RetirementDemand{depth:2,..Default::default()}){return None}let p=metadata();Some((Self{source:witness(source),phase:Phase::Begin,field:0,seen:0,fields:0,argument_bytes:0,argument_at:0,argument:[0;9],step:ManuallyDrop::new(Some(ToolRunStep::new(0,ToolRunStepKind::Info,0,0))),receipt:Some((g,p)),closing:false},p))}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn allocated_bytes(&self)->usize{self.step.as_ref().map_or(0,|step|step.args.capacity()*std::mem::size_of::<ToolRunStepArg>())}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn demands(&self,pending:&Option<RetainedRecordBodyToken>)->Result<RetirementDemand,ValueError>{
  if self.receipt.is_some()||self.closing||matches!(self.phase,Phase::Complete|Phase::Closed){return Ok(Default::default())}
  let mut d=RetirementDemand{depth:2,..Default::default()};if self.phase==Phase::Bytes{d.copy_bytes=1}
  if self.phase==Phase::Length{if let Some(RetainedRecordBodyToken::Value(RetainedValueToken::Unsigned{role:RetainedValueRole::BytesLength,value}))=pending{let n=usize::try_from(*value).map_err(|_|fault())?;if n%9!=0||n/9>TOOL_RUN_STEP_ARGS_MAX{return Err(fault())}d.capacity_bytes=(n/9).saturating_sub(self.step.as_ref().ok_or_else(fault)?.args.capacity())*std::mem::size_of::<ToolRunStepArg>()}}
  Ok(d)
 }
 pub fn receive(&mut self,source:&RetainedRecordBodySourceDecode,pending:&mut Option<RetainedRecordBodyToken>,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if witness(source)!=self.source{return Err(fault())}if self.receipt.is_some()||source.receipt().is_some()||self.closing||matches!(self.phase,Phase::Complete|Phase::Closed){return Ok(None)}let d=self.demands(pending)?;if !funds(g,d){return Ok(None)}let Some(RetainedRecordBodyToken::Value(token))=pending.as_ref().copied()else{return Ok(None)};
  let mut next=self.phase;let mut field=self.field;let mut seen=self.seen;let mut fields=self.fields;let mut argument_bytes=self.argument_bytes;let mut argument_at=self.argument_at;let mut p=metadata();let step=self.step.as_mut().ok_or_else(fault)?;
  match(self.phase,token){
   (Phase::Begin,RetainedValueToken::Begin{kind:RetainedValueContainer::Record,count})if(5..=7).contains(&count)=>{fields=count as u8;next=Phase::Field},
   (Phase::Field,RetainedValueToken::Unsigned{role:RetainedValueRole::FieldId,value})if(1..=7).contains(&value)&&self.seen&(1<<(value-1))==0=>{field=value as u8;next=Phase::Tag},
   (Phase::Tag,RetainedValueToken::Tag{value:0,..})if matches!(self.field,5|7)=>{seen|=1<<(self.field-1);next=Phase::Field},
   (Phase::Tag,RetainedValueToken::Tag{value,..})if value==if self.field==7{8}else{4}=>next=if self.field==7{Phase::Length}else{Phase::Scalar},
   (Phase::Scalar,RetainedValueToken::Unsigned{role:RetainedValueRole::Unsigned,value})=>{match self.field{1=>step.sequence=value,2=>step.kind=ToolRunStepKind::from_ordinal(value).ok_or_else(fault)?,3=>step.stage=u16::try_from(value).map_err(|_|fault())?,4=>step.reason=u16::try_from(value).map_err(|_|fault())?,5=>step.subject=Some(value),6 if value>0=>step.repeat=u32::try_from(value).map_err(|_|fault())?,_=>return Err(fault())}seen|=1<<(self.field-1);next=Phase::Field},
   (Phase::Length,RetainedValueToken::Unsigned{role:RetainedValueRole::BytesLength,value})=>{argument_bytes=usize::try_from(value).map_err(|_|fault())?;if argument_bytes%9!=0||argument_bytes/9>TOOL_RUN_STEP_ARGS_MAX{return Err(fault())}let before=step.args.capacity();step.args.try_reserve_exact(argument_bytes/9).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original typed step argument backing allocation refused"))?;p.retained_capacity_bytes=(step.args.capacity()-before)*std::mem::size_of::<ToolRunStepArg>();if !p.fits(g){self.receipt=Some((g,p));return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original typed step actual argument birth exceeds supplied grant").with_retained_progress(p))}if argument_bytes==0{seen|=64;next=Phase::Field}else{next=Phase::Bytes}},
   (Phase::Bytes,RetainedValueToken::Byte(byte))if self.argument_bytes>0=>{if self.argument_at==0&&byte>1{return Err(fault())}self.argument[self.argument_at]=byte;argument_at+=1;argument_bytes-=1;p.copied_bytes=1;if argument_at==9{let value=u64::from_le_bytes(self.argument[1..].try_into().unwrap());let arg=match self.argument[0]{0=>ToolRunStepArg::Unsigned(value),1=>ToolRunStepArg::Float(f64::from_bits(value)),_=>return Err(fault())};step.args.push(arg);argument_at=0;}if argument_bytes==0{seen|=64;next=Phase::Field}},
   (Phase::Field,RetainedValueToken::End(RetainedValueContainer::Record))if self.seen&47==47&&self.seen.count_ones()==u32::from(self.fields)=>next=Phase::Complete,
   _=>return Err(fault()),
  }
  self.phase=next;self.field=field;self.seen=seen;self.fields=fields;self.argument_bytes=argument_bytes;self.argument_at=argument_at;drop(pending.take());self.receipt=Some((g,p));Ok(Some(p))
 }
 pub fn step(&self)->Option<&ToolRunStep>{if self.phase==Phase::Complete{self.step.as_ref()}else{None}}
 pub fn take_step_into(&mut self,recipient:&mut Option<ToolRunStep>,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{if self.receipt.is_some()||self.closing||self.phase!=Phase::Complete||recipient.is_some()||self.step.is_none()||!funds(g,RetirementDemand{depth:2,..Default::default()}){return None}*recipient=self.step.take();let p=metadata();self.receipt=Some((g,p));Some(p)}
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.closing||self.receipt.is_some()||self.phase==Phase::Closed{return Ok(Default::default())}Ok(RetirementDemand{release_bytes:self.step.as_ref().filter(|step|step.args.is_empty()).map_or(0,|step|step.args.capacity()*std::mem::size_of::<ToolRunStepArg>()),depth:2,..Default::default()})}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if !self.closing||self.receipt.is_some()||self.phase==Phase::Closed{return Ok(None)}let d=self.close_demands()?;if !funds(g,d){return Ok(None)}let mut p=metadata();if let Some(step)=self.step.as_mut(){if !step.args.is_empty(){step.args.pop();}else if step.args.capacity()>0{drop(std::mem::take(&mut step.args));p.released_bytes=d.release_bytes;}else{drop(self.step.take());self.phase=Phase::Closed;}}else{self.phase=Phase::Closed;}self.receipt=Some((g,p));Ok(Some(p))}
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.step.is_none()&&self.receipt.is_none()}
}
impl Drop for ToolRunStepReceiving{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original typed step abandoned native arguments or receipt");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.step)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
