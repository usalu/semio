//! 📤️ Canonical Fault wire advances through original borrowed fields without whole-value materialization.
use crate::{Fault,FaultOrigin,Severity};
use std::mem::MaybeUninit;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:usize,target:usize,extent:usize}
#[derive(Clone,Copy,Default)]
struct TokenCursor{offset:usize,escape:usize,opened:bool,closed:bool}
enum Part<'a>{Raw(&'a[u8]),Text(&'a str),Number(u64),Skip}

/// 🧳️ Enclosing admission owns the inline cursor and target; callers retain the same immutable Fault.
pub struct FaultWireCursor{binding:Option<Binding>,phase:u8,index:usize,part:u8,token:TokenCursor,written:usize,drained:usize}
impl FaultWireCursor{
 pub fn new()->Self{Self{binding:None,phase:0,index:0,part:0,token:Default::default(),written:0,drained:0}}
 pub fn written(&self)->usize{self.written}
 pub fn is_complete(&self)->bool{self.phase==25}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()}
 fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical Fault wire requires its same original source and target extent")}
 fn original(fault:&Fault,target:&[MaybeUninit<u8>])->Binding{Binding{source:fault as*const Fault as usize,target:target.as_ptr()as usize,extent:target.len()}}
 fn scope(fault:&Fault)->[(&'static[u8],Option<&str>);5]{[(b"\"pluginId\":",fault.scope.plugin_id.as_deref()),(b"\"appId\":",fault.scope.app_id.as_deref()),(b"\"instanceId\":",fault.scope.instance_id.as_deref()),(b"\"module\":",fault.scope.module.as_deref()),(b"\"bodyKey\":",fault.scope.body_key.as_deref())]}
 fn params(fault:&Fault)->Option<&[(String,String)]>{fault.params.as_deref().map(|params|params.0.as_slice()).filter(|params|!params.is_empty())}
 fn current<'a>(&self,fault:&'a Fault)->Part<'a>{
  match self.phase{
   0=>Part::Raw(b"{\"origin\":"),1=>Part::Text(match fault.origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"}),
   2=>Part::Raw(b",\"code\":"),3=>Part::Text(&fault.code.0),4=>Part::Raw(b",\"severity\":"),5=>Part::Text(match fault.severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"}),
   6=>Part::Raw(b",\"message\":"),7=>Part::Text(&fault.message),8=>Part::Raw(b",\"scope\":{"),
   9=>{let fields=Self::scope(fault);let Some((key,Some(value)))=fields.get(self.index)else{return Part::Skip};match self.part{0=>Part::Raw(if fields[..self.index].iter().any(|(_,value)|value.is_some()){b","}else{b""}),1=>Part::Raw(key),_=>Part::Text(value)}},
   10=>Part::Raw(b"},\"retainedProgress\":{"),
   11=>match self.index{0=>Part::Raw(b"\"copiedItems\":"),1=>Part::Number(fault.retained_progress.copied_items as u64),2=>Part::Raw(b",\"copiedBytes\":"),3=>Part::Number(fault.retained_progress.copied_bytes as u64),4=>Part::Raw(b",\"retainedCapacityBytes\":"),5=>Part::Number(fault.retained_progress.retained_capacity_bytes as u64),6=>Part::Raw(b",\"releasedBytes\":"),7=>Part::Number(fault.retained_progress.released_bytes as u64),_=>Part::Skip},
   12=>Part::Raw(b"}"),13=>if fault.span.is_some(){Part::Raw(b",\"span\":{")}else{Part::Skip},
   14=>{let Some(span)=fault.span else{return Part::Skip};match self.index{0=>Part::Raw(b"\"line\":"),1=>Part::Number(u64::from(span.line)),2=>Part::Raw(b",\"column\":"),3=>Part::Number(u64::from(span.column)),4=>Part::Raw(b",\"length\":"),5=>Part::Number(u64::from(span.length)),_=>Part::Skip}},
   15=>if fault.span.is_some(){Part::Raw(b"}")}else{Part::Skip},16=>if fault.causes.is_empty(){Part::Skip}else{Part::Raw(b",\"causes\":[")},
   17=>{let Some(cause)=fault.causes.get(self.index)else{return Part::Skip};match self.part{0=>Part::Raw(if self.index==0{b"{\"message\":"}else{b",{\"message\":"}),1=>Part::Text(&cause.message),2=>if cause.code.is_some(){Part::Raw(b",\"code\":")}else{Part::Skip},3=>cause.code.as_ref().map_or(Part::Skip,|code|Part::Text(&code.0)),_=>Part::Raw(b"}")}},
   18=>if fault.causes.is_empty(){Part::Skip}else{Part::Raw(b"]")},19=>if Self::params(fault).is_some(){Part::Raw(b",\"params\":{")}else{Part::Skip},
   20=>{let Some((name,value))=Self::params(fault).and_then(|params|params.get(self.index))else{return Part::Skip};match self.part{0=>Part::Raw(if self.index==0{b""}else{b","}),1=>Part::Text(name),2=>Part::Raw(b":"),_=>Part::Text(value)}},
   21=>if Self::params(fault).is_some(){Part::Raw(b"}")}else{Part::Skip},22=>Part::Raw(b",\"retryable\":"),23=>Part::Raw(if fault.retryable{b"true"}else{b"false"}),24=>Part::Raw(b"}"),_=>Part::Skip
  }
 }
 fn escape(byte:u8,index:usize)->(u8,usize){
  let short=match byte{b'"'=>Some(b'"'),b'\\'=>Some(b'\\'),8=>Some(b'b'),9=>Some(b't'),10=>Some(b'n'),12=>Some(b'f'),13=>Some(b'r'),_=>None};
  if let Some(short)=short{return(if index==0{b'\\'}else{short},2)}
  if byte<32{return(match index{0=>b'\\',1=>b'u',2|3=>b'0',4=>b"0123456789abcdef"[(byte>>4)as usize],_=>b"0123456789abcdef"[(byte&15)as usize]},6)}
  (byte,1)
 }
 fn next_byte(&self,part:Part<'_>)->Option<(u8,TokenCursor)>{
  let mut next=self.token;
  let byte=match part{
   Part::Skip=>return None,
   Part::Raw(bytes)=>{let byte=*bytes.get(next.offset)?;next.offset+=1;byte},
   Part::Number(mut value)=>{let original=value;let mut digits=1;while value>=10{value/=10;digits+=1;}if next.offset>=digits{return None}let divisor=10u64.pow((digits-next.offset-1)as u32);next.offset+=1;b'0'+((original/divisor)%10)as u8},
   Part::Text(text)=>{if !next.opened{next.opened=true;b'"'}else if let Some(byte)=text.as_bytes().get(next.offset){let(encoded,length)=Self::escape(*byte,next.escape);next.escape+=1;if next.escape==length{next.escape=0;next.offset+=1}encoded}else if !next.closed{next.closed=true;b'"'}else{return None}}
  };Some((byte,next))
 }
 fn next_part(&mut self,fault:&Fault){
  match self.phase{
   9 if self.index<5=>{if Self::scope(fault)[self.index].1.is_none()||self.part==2{self.index+=1;self.part=0}else{self.part+=1}},
   11 if self.index<8=>self.index+=1,
   14 if fault.span.is_some()&&self.index<6=>self.index+=1,
   17 if self.index<fault.causes.len()=>{if self.part==4{self.index+=1;self.part=0}else{self.part+=1}},
   20 if Self::params(fault).is_some_and(|params|self.index<params.len())=>{if self.part==3{self.index+=1;self.part=0}else{self.part+=1}},
   _=>{self.phase+=1;self.index=0;self.part=0}
  }self.token=Default::default();
 }
 /// 📏️ Quotes one actual byte or metadata transition without reading a whole string or allocating.
 pub fn advance_demands(&self,fault:&Fault,target:&[MaybeUninit<u8>])->Result<RetirementDemand,ValueError>{
  if self.binding.is_some_and(|binding|binding!=Self::original(fault,target)){return Err(Self::refused())}
  if self.is_complete(){return Ok(Default::default())}
  Ok(RetirementDemand{copy_bytes:usize::from(self.binding.is_some()&&self.next_byte(self.current(fault)).is_some()),depth:1,..Default::default()})
 }
 /// ✍️ Writes one paid canonical byte into the same caller-owned target; denials retain both cursors.
 pub fn advance_one(&mut self,fault:&Fault,target:&mut[MaybeUninit<u8>],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.advance_demands(fault,target)?;if self.is_complete(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.binding.is_none(){self.binding=Some(Self::original(fault,target));return Ok(RetainedCloneStep::Progress(progress))}
  if let Some((byte,next))=self.next_byte(self.current(fault)){let slot=target.get_mut(self.written-self.drained).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"canonical Fault wire exhausted its original target extent"))?;slot.write(byte);self.token=next;self.written+=1;progress.copied_bytes=1;}else{self.next_part(fault)}
  Ok(if self.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 /// 🔎️ Borrows only the initialized prefix of the same original streaming window.
 pub fn buffered_bytes<'a>(&self,target:&'a[MaybeUninit<u8>])->Result<&'a[u8],ValueError>{
  if self.binding.is_some_and(|binding|binding.target!=target.as_ptr()as usize||binding.extent!=target.len()){return Err(Self::refused())}
  let length=self.written-self.drained;if length>target.len(){return Err(Self::refused())}
  Ok(unsafe{std::slice::from_raw_parts(target.as_ptr().cast::<u8>(),length)})
 }
 /// 🌊️ Ends one original window loan on a separate metadata turn before reusing its storage.
 pub fn drain_buffer(&mut self,target:&[MaybeUninit<u8>],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let empty=self.buffered_bytes(target)?.is_empty();if empty{return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.drained=self.written;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 pub fn retirement_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(self.binding.is_some()),..Default::default()}}
 /// ♻️ Releases only cursor metadata after the caller ends the original byte loan.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_depth<1{return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.binding=None;self.phase=0;self.index=0;self.part=0;self.token=Default::default();self.written=0;self.drained=0;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl Default for FaultWireCursor{fn default()->Self{Self::new()}}
impl Drop for FaultWireCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Fault wire cursor reached Drop before its original metadata retirement")}}
