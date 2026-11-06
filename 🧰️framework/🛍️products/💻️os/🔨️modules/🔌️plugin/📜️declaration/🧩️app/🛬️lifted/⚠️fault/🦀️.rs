//! ⚠️ Accepted completion faults retain their original diagnostic and build a bounded borrowed report without allocation (lifted from the WP1 completion tree; WP2 wires it into decl-app).
use crate::tool_contract::ArtifactBoundedToolFault;
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
 pub fn framed_page_bytes(&self,buffer:&mut[u8;480])->usize{self.bounded.framed_page_bytes(buffer)}
}
/// 🧾️ Copies a bounded borrowed diagnostic into its actual fixed inline carrier without constructing an owned Fault.
pub fn borrowed_report(origin:FaultOrigin,code:&str,severity:Severity,message:&str,retryable:bool)->ArtifactBoundedToolFault{
 let origin=match origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"};
 let severity=match severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"};
 let mut frame=Frame{bytes:[0;480],len:0};frame.write(b"{\"origin\":");frame.string(origin,16);frame.write(b",\"code\":");let code_start=frame.len;if !frame.string(code,128){frame.len=code_start;frame.string("interactive-job.fault-capacity",128);}frame.write(b",\"severity\":");frame.string(severity,16);frame.write(b",\"message\":");frame.string(message,224);frame.write(b",\"scope\":{},\"retryable\":");frame.write(if retryable{b"true"}else{b"false"});frame.write(b"}");ArtifactBoundedToolFault::from_inline_report(frame.bytes,frame.len)
}
/// 🧮️ Prices transfer of every byte in the concrete bounded report carrier, including its inline frame and length.
pub const fn borrowed_report_copy_bytes()->usize{size_of::<ArtifactBoundedToolFault>()}
