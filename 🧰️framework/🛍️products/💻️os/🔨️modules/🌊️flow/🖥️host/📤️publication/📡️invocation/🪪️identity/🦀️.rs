//! 🪪️ Original operation identity becomes cancellation text through separately paid source events.
use super::*;
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowInvocationIdentity{source:u64,remaining:u64,digits:[u8;20],length:usize,buffer:Vec<u8>,text:Option<String>,phase:u8}
impl FlowInvocationIdentity{
 pub fn new(operation:u64)->Self{Self{source:operation,remaining:operation,digits:[0;20],length:0,buffer:Vec::new(),text:None,phase:0}}
 pub fn source(&self)->u64{self.source}
 pub fn text(&self)->Option<&str>{self.text.as_deref()}
 pub fn take_text(&mut self)->Option<String>{self.text.take()}
 pub fn complete(&self)->bool{self.phase==255}
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:usize::from(self.phase==2&&self.buffer.len()<self.length),capacity_bytes:if self.phase==1{self.length}else{0},depth:1,..Default::default()}}
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.demands();if grant.maximum_items==0{return Ok(Default::default())}if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation identity requires depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes{return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};match self.phase{
  0=>{self.digits[self.length]=b'0'+(self.remaining%10)as u8;self.remaining/=10;self.length+=1;if self.remaining==0{self.phase=1;}},
  1=>{self.buffer.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation identity allocation refused"))?;p.retained_capacity_bytes=self.buffer.capacity();self.phase=2;},
  2=>{if self.buffer.len()==self.length{self.phase=3;}else{let count=(self.length-self.buffer.len()).min(grant.maximum_copy_bytes);for _ in 0..count{self.buffer.push(self.digits[self.length-self.buffer.len()-1]);}p.copied_bytes=count;}},
  3=>{self.text=Some(unsafe{String::from_utf8_unchecked(std::mem::take(&mut self.buffer))});self.phase=255;},
  255=>return Ok(Default::default()),_=>unreachable!("original invocation identity phase"),
 }Ok(p)}
}
