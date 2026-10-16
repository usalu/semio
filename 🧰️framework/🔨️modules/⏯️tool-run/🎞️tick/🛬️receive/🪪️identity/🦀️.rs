//! 🪪️ One original record token enters the fixed typed identity only after its unchanged grant admits it.
use crate::{ToolRunId,ToolRunIdentity};
use pack::record::{RetainedRecordBodySourceDecode,RetainedRecordBodyToken,RetainedValueToken,RetainedValueContainer,RetainedValueRole};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Begin,Field,Tag,Scalar,Length,Bytes,Complete,Closed}
pub struct ToolRunIdentityReceiving{
 source:(usize,usize,Option<(u64,u64)>,(usize,usize)),phase:Phase,field:u8,seen:u8,revision_at:usize,
 identity:Option<ToolRunIdentity>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,
}
fn witness(source:&RetainedRecordBodySourceDecode)->(usize,usize,Option<(u64,u64)>,(usize,usize)){let original=source.source_identity();(original.address,original.extent,original.authority,source.original_range())}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original typed identity token or source is invalid")}
fn funds(grant:RetainedCloneGrant,copy:usize)->bool{grant.maximum_items>0&&grant.maximum_depth>=2&&grant.maximum_copy_bytes>=copy}
impl ToolRunIdentityReceiving{
 pub fn admit(source:&RetainedRecordBodySourceDecode,grant:RetainedCloneGrant)->Option<(Self,RetainedCloneProgress)>{if !funds(grant,0){return None}let progress=RetainedCloneProgress{copied_items:1,..Default::default()};Some((Self{source:witness(source),phase:Phase::Begin,field:0,seen:0,revision_at:0,identity:Some(ToolRunIdentity{id:ToolRunId{app_instance_id:0,run:0},generation:0,base_revision:[0;32]}),receipt:Some((grant,progress)),closing:false},progress))}
 pub fn source_identity(&self)->(usize,usize,Option<(u64,u64)>,(usize,usize)){self.source}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn demands(&self)->RetirementDemand{if self.receipt.is_some()||self.closing||matches!(self.phase,Phase::Complete|Phase::Closed){return Default::default()}RetirementDemand{copy_bytes:usize::from(self.phase==Phase::Bytes),depth:2,..Default::default()}}
 pub fn receive(&mut self,source:&RetainedRecordBodySourceDecode,pending:&mut Option<RetainedRecordBodyToken>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if witness(source)!=self.source{return Err(fault())}if self.receipt.is_some()||source.receipt().is_some()||self.closing||matches!(self.phase,Phase::Complete|Phase::Closed)||!funds(grant,self.demands().copy_bytes){return Ok(None)}
  let Some(RetainedRecordBodyToken::Value(token))=pending.as_ref().copied()else{return Ok(None)};
  let mut next=self.phase;let mut field=self.field;let mut seen=self.seen;let mut revision_at=self.revision_at;let identity=self.identity.as_mut().ok_or_else(fault)?;
  match(self.phase,token){
   (Phase::Begin,RetainedValueToken::Begin{kind:RetainedValueContainer::Record,count:4})=>next=Phase::Field,
   (Phase::Field,RetainedValueToken::Unsigned{role:RetainedValueRole::FieldId,value})if(1..=4).contains(&value)&&self.seen&(1<<(value-1))==0=>{field=value as u8;next=Phase::Tag},
   (Phase::Tag,RetainedValueToken::Tag{value,..})if value==if self.field==4{8}else{4}=>next=if self.field==4{Phase::Length}else{Phase::Scalar},
   (Phase::Scalar,RetainedValueToken::Unsigned{role:RetainedValueRole::Unsigned,value})=>{match self.field{1=>identity.id.app_instance_id=u32::try_from(value).map_err(|_|fault())?,2=>identity.id.run=value,3=>identity.generation=u32::try_from(value).map_err(|_|fault())?,_=>return Err(fault())}seen|=1<<(self.field-1);next=Phase::Field},
   (Phase::Length,RetainedValueToken::Unsigned{role:RetainedValueRole::BytesLength,value:32})=>next=Phase::Bytes,
   (Phase::Bytes,RetainedValueToken::Byte(byte))if self.revision_at<32=>{identity.base_revision[self.revision_at]=byte;revision_at+=1;if revision_at==32{seen|=8;next=Phase::Field}},
   (Phase::Field,RetainedValueToken::End(RetainedValueContainer::Record))if self.seen==15=>next=Phase::Complete,
   _=>return Err(fault()),
  }
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:usize::from(self.phase==Phase::Bytes),..Default::default()};self.phase=next;self.field=field;self.seen=seen;self.revision_at=revision_at;drop(pending.take());self.receipt=Some((grant,progress));Ok(Some(progress))
 }
 pub fn identity(&self)->Option<&ToolRunIdentity>{if self.phase==Phase::Complete{self.identity.as_ref()}else{None}}
 pub fn take_identity_into(&mut self,recipient:&mut Option<ToolRunIdentity>,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{if self.receipt.is_some()||self.closing||self.phase!=Phase::Complete||recipient.is_some()||self.identity.is_none()||!funds(grant,0){return None}*recipient=self.identity.take();let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.receipt=Some((grant,progress));Some(progress)}
 pub fn retire_identity_step(&mut self,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{if self.receipt.is_some()||self.closing||self.phase!=Phase::Complete||self.identity.is_none()||!funds(grant,0){return None}self.identity=None;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.receipt=Some((grant,progress));Some(progress)}
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{if !self.closing||self.receipt.is_some()||self.phase==Phase::Closed||!funds(grant,0){return None}self.identity=None;self.phase=Phase::Closed;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.receipt=Some((grant,progress));Some(progress)}
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.identity.is_none()&&self.receipt.is_none()}
}
impl Drop for ToolRunIdentityReceiving{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original typed identity abandoned its receipt or semantic recipient")}}
