//! 🎟️ The session receives the same Host text source through funded bytes and a genuine shared header.
use super::*;
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct SessionLivePublication{pointer:usize,length:usize,buffer:Vec<u8>,text:Option<String>,shared:Option<FlowPublicationShared<String>>,phase:u8}
impl SessionLivePublication{
 pub(super) fn new(source:&str)->Self{Self{pointer:source.as_ptr()as usize,length:source.len(),buffer:Vec::new(),text:None,shared:None,phase:0}}
 pub(super) fn matches(&self,source:&str)->bool{self.pointer==source.as_ptr()as usize&&self.length==source.len()}
 pub(super) fn demands(&self,source:&str)->Result<RetirementDemand,ValueError>{
  if !self.matches(source){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original session publication source changed"))}
  Ok(RetirementDemand{copy_bytes:usize::from(self.phase==1&&self.buffer.len()<self.length),capacity_bytes:match self.phase{0=>self.length,3=>semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>(),_=>0},depth:1,..Default::default()})
 }
 pub(super) fn step(&mut self,source:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let d=self.demands(source)?;if grant.maximum_items==0{return Ok(Default::default())}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original session publication requires source depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes{return Ok(Default::default())}
  let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};match self.phase{0=>{self.buffer.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original session text birth refused"))?;p.retained_capacity_bytes=self.buffer.capacity();self.phase=1;},1=>{let begin=self.buffer.len();let end=(begin+grant.maximum_copy_bytes.min(self.length-begin)).min(self.length);self.buffer.extend_from_slice(&source.as_bytes()[begin..end]);p.copied_bytes=end-begin;if end==self.length{self.phase=2;}},2=>{self.text=Some(unsafe{String::from_utf8_unchecked(std::mem::take(&mut self.buffer))});self.phase=3;},3=>{self.shared=Some(FlowPublicationShared(Arc::new(self.text.take().unwrap())));p.retained_capacity_bytes=d.capacity_bytes;self.phase=4;},_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original session publication text was already admitted"))}Ok(p)
 }
 pub(super) fn ready(&self)->bool{self.phase==4}
 pub(super) fn published(&self)->bool{self.phase==5}
 pub(super) fn mark_published(&mut self){assert!(self.shared.is_none());self.phase=5;}
 pub(super) fn take_shared(&mut self)->Option<Arc<String>>{self.shared.take().map(|value|value.0)}
 pub(super) fn restore_shared(&mut self,value:Arc<String>){assert!(self.shared.is_none());self.shared=Some(FlowPublicationShared(value));}
}
