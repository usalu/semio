//! ⚠️ Accepted completion faults retain their original diagnostic and build a bounded borrowed report without allocation.
use crate::app::ArtifactBoundedToolFault;
use semio_framework_diagnostic::{Fault,FaultOrigin,Severity};
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactCompletionFault{pub bounded:ArtifactBoundedToolFault,original:Fault}
impl std::fmt::Debug for ArtifactCompletionFault{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("ArtifactCompletionFault").field("original",&self.original).finish_non_exhaustive()}}
struct Frame{bytes:[u8;480],len:usize}
impl Frame{
 fn write(&mut self,text:&[u8]){self.bytes[self.len..self.len+text.len()].copy_from_slice(text);self.len+=text.len();}
 fn string(&mut self,text:&str,maximum:usize)->bool{
  self.write(b"\"");let start=self.len;let mut complete=true;
  for scalar in text.chars(){let mut escaped=[0u8;6];let length=match scalar{'"'|'\\'=>{escaped[0]=b'\\';escaped[1]=scalar as u8;2},'\n'=>{escaped[..2].copy_from_slice(b"\\n");2},'\r'=>{escaped[..2].copy_from_slice(b"\\r");2},'\t'=>{escaped[..2].copy_from_slice(b"\\t");2},'\u{08}'=>{escaped[..2].copy_from_slice(b"\\b");2},'\u{0c}'=>{escaped[..2].copy_from_slice(b"\\f");2},scalar if scalar<'\u{20}'=>{escaped[..4].copy_from_slice(b"\\u00");let byte=scalar as u8;escaped[4]=b"0123456789abcdef"[(byte>>4)as usize];escaped[5]=b"0123456789abcdef"[(byte&15)as usize];6},scalar=>scalar.encode_utf8(&mut escaped).len()};if self.len-start+length>maximum{complete=false;break;}self.write(&escaped[..length]);}
  self.write(b"\"");complete
 }
}
impl ArtifactCompletionFault{
 /// 📥️ Moves the actual original fault while retaining an inline report for host publication.
 pub fn new(original:Fault)->Self{
  Self{bounded:borrowed_report(original.origin,&original.code.0,original.severity,&original.message,original.retryable),original}
 }
 /// 📤️ Transfers the exact original diagnostic rather than reconstructing it from a report.
 pub fn into_fault(self)->Fault{self.original}
 pub(crate) fn framed_page_bytes(&self,buffer:&mut[u8;480])->usize{self.bounded.framed_page_bytes(buffer)}
}
/// 🧾️ Copies a bounded borrowed diagnostic into its actual fixed inline carrier without constructing an owned Fault.
pub(crate) fn borrowed_report(origin:FaultOrigin,code:&str,severity:Severity,message:&str,retryable:bool)->ArtifactBoundedToolFault{
 let origin=match origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"};
 let severity=match severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"};
 let mut frame=Frame{bytes:[0;480],len:0};frame.write(b"{\"origin\":");frame.string(origin,16);frame.write(b",\"code\":");let code_start=frame.len;if !frame.string(code,128){frame.len=code_start;frame.string("interactive-job.fault-capacity",128);}frame.write(b",\"severity\":");frame.string(severity,16);frame.write(b",\"message\":");frame.string(message,224);frame.write(b",\"scope\":{},\"retryable\":");frame.write(if retryable{b"true"}else{b"false"});frame.write(b"}");ArtifactBoundedToolFault::from_inline_report(frame.bytes,frame.len)
}
/// 🧮️ Prices transfer of every byte in the concrete bounded report carrier, including its inline frame and length.
pub(crate) const fn borrowed_report_copy_bytes()->usize{std::mem::size_of::<ArtifactBoundedToolFault>()}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn completion_fault_moves_original_without_heap_birth_and_matches_shared_json(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let mut original=Fault::new(FaultOrigin::App,semio_framework_diagnostic::FaultCode::new(row["code"].as_str().unwrap()),row["message"].as_str().unwrap());original.retryable=row["retryable"].as_bool().unwrap();original.params=Some(Box::new(semio_framework_diagnostic::FaultParams(vec![("path".into(),"drawing/clipboard".into()),("detail".into(),"original diagnostic parameter".into())])));original.scope.module=Some("draw".into());let message=original.message.as_ptr();let scope=(&*original.scope)as*const _;
   let(carrier,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactCompletionFault::new(original));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
   let mut bytes=[0;480];let len=carrier.framed_page_bytes(&mut bytes);let wire:serde_json::Value=serde_json::from_slice(&bytes[..len]).unwrap();assert!(len<=fixture["frameBytes"].as_u64().unwrap()as usize);let expected_code=if serde_json::to_string(row["code"].as_str().unwrap()).unwrap().len()-2<=fixture["codeEscapeBytes"].as_u64().unwrap()as usize{row["code"].as_str().unwrap()}else{"interactive-job.fault-capacity"};assert_eq!(wire["code"],expected_code);assert_eq!(wire["retryable"],row["retryable"]);let mut expected=String::new();for scalar in row["message"].as_str().unwrap().chars(){let mut candidate=expected.clone();candidate.push(scalar);if serde_json::to_string(&candidate).unwrap().len()-2>fixture["messageEscapeBytes"].as_u64().unwrap()as usize{break;}expected=candidate;}assert_eq!(wire["message"],expected);
   let(original,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||carrier.into_fault());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(original.message.as_ptr(),message);assert_eq!((&*original.scope)as*const _,scope);assert_eq!(original.message,row["message"].as_str().unwrap());
   let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original diagnostic declares exact retirement"));for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(event.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(event.released_bytes,step.progress().released_bytes);}assert!(owner.terminal_is_empty());
  }
  eprintln!("[DEBUG] Completion original fault has zero-heap carrier birth, bounded serde JSON report and exact original diagnostic retirement");
 }
}
