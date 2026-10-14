//! ⚠️ Bounded typed Fault binding donates original strings and domain cells without cloning the parsed donor.
use super::*;
use semio_framework::{Fault,FaultCode,FaultScope,FaultOrigin,FaultCause,FaultParams,FaultSource,Severity};
pub(super) struct FaultCursor{output:ManuallyDrop<Option<Fault>>,closing:ManuallyDrop<Option<ControlledRetirement<Fault>>>,field:usize,nested:usize,kind:u8,cause:usize,cause_field:usize,seen:u8,progress_seen:u8,cause_seen:bool,reserved:bool,closed:bool}
impl FaultCursor{
 pub(super) const fn empty()->Self{Self{output:ManuallyDrop::new(None),closing:ManuallyDrop::new(None),field:0,nested:0,kind:0,cause:0,cause_field:0,seen:0,progress_seen:0,cause_seen:false,reserved:false,closed:false}}
 pub(super) fn step(&mut self,source:&mut DslValue,cx:&mut StepContext<'_>)->Result<Option<Fault>,OriginalTurnFailure>{
  if self.output.is_none(){let bytes=std::mem::size_of::<FaultScope>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}cx.consume_fuel(1);let scope=Box::default();*self.output=Some(Fault{retained_progress:Default::default(),origin:FaultOrigin::Framework,code:FaultCode(String::new()),severity:Severity::Error,message:String::new(),scope,span:None,source:None,domain:None,causes:Vec::new(),params:None,retryable:false});cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(None)}
  let DslValue::Object(fields)=source else{return Err(invalid())};
  if self.field==fields.len(){if self.seen!=31{return Err(invalid())}if !metadata(cx)?{return Ok(None)}return Ok(self.output.take())}
  let(name,value)=&mut fields[self.field];let output=self.output.as_mut().unwrap();
  if self.kind==0{
   if !metadata(cx)?{return Ok(None)}
   match name.as_str(){
    "origin"=>{let DslValue::String(text)=value else{return Err(invalid())};output.origin=match text.as_str(){"edge"=>FaultOrigin::Edge,"renderer"=>FaultOrigin::Renderer,"os"=>FaultOrigin::Os,"module"=>FaultOrigin::Module,"plugin"=>FaultOrigin::Plugin,"app"=>FaultOrigin::App,"extension"=>FaultOrigin::Extension,"framework"=>FaultOrigin::Framework,_=>return Err(invalid())};self.seen|=2},
    "code"=>{output.code=FaultCode(take_text(value)?);self.seen|=4},
    "severity"=>{let DslValue::String(text)=value else{return Err(invalid())};output.severity=match text.as_str(){"info"=>Severity::Info,"warning"=>Severity::Warning,"error"=>Severity::Error,"fatal"=>Severity::Fatal,_=>return Err(invalid())};self.seen|=8},
    "message"=>{output.message=take_text(value)?;self.seen|=16},
    "retainedProgress"=>{object(value)?;self.kind=1;self.progress_seen=0;return Ok(None)},
    "scope"=>if !matches!(value,DslValue::Null){object(value)?;self.kind=2;return Ok(None)},
    "source"=>if !matches!(value,DslValue::Null){object(value)?;output.source=Some(Default::default());self.kind=3;return Ok(None)},
    "span"=>if !matches!(value,DslValue::Null){object(value)?;output.span=Some(Default::default());self.kind=4;return Ok(None)},
    "domain"=>{object(value)?;output.domain=Some(std::mem::replace(value,DslValue::Null))},
    "retryable"=>match value{DslValue::Bool(flag)=>output.retryable=*flag,DslValue::Null=>{},_=>return Err(invalid())},
    "causes"=>if !matches!(value,DslValue::Null){if !matches!(value,DslValue::Array(_)){return Err(invalid())}self.kind=5;self.reserved=false;return Ok(None)},
    "params"=>if !matches!(value,DslValue::Null){object(value)?;self.kind=6;self.reserved=false;return Ok(None)},
    _=>return Err(invalid()),
   }self.field+=1;return Ok(None)
  }
  if self.kind==5{return self.causes_step(value,cx)}
  if self.kind==6{return self.params_step(value,cx)}
  let entries=object(value)?;
  if self.nested==entries.len(){if self.kind==1{if self.progress_seen!=15{return Err(invalid())}self.seen|=1}if !metadata(cx)?{return Ok(None)}self.kind=0;self.nested=0;self.field+=1;return Ok(None)}
  if !metadata(cx)?{return Ok(None)}let(name,value)=&mut entries[self.nested];
  match self.kind{
   1=>progress(&mut output.retained_progress,&mut self.progress_seen,name,value)?,
   2=>{let slot=match name.as_str(){"pluginId"=>&mut output.scope.plugin_id,"appId"=>&mut output.scope.app_id,"instanceId"=>&mut output.scope.instance_id,"module"=>&mut output.scope.module,"bodyKey"=>&mut output.scope.body_key,_=>return Err(invalid())};if !matches!(value,DslValue::Null){*slot=Some(take_text(value)?)}},
   3=>{let scalar=unsigned(value)?;let target=output.source.as_mut().unwrap();match name.as_str(){"offset"=>target.offset=Some(scalar),"depth"=>target.depth=Some(scalar),"byte"=>target.byte=Some(u8::try_from(scalar).map_err(|_|invalid())?),_=>return Err(invalid())}},
   4=>{let scalar=u32::try_from(unsigned(value)?).map_err(|_|invalid())?;let target=output.span.as_mut().unwrap();match name.as_str(){"line"=>target.line=scalar,"column"=>target.column=scalar,"length"=>target.length=scalar,_=>return Err(invalid())}},
   _=>return Err(invalid()),
  }self.nested+=1;Ok(None)
 }
 fn causes_step(&mut self,value:&mut DslValue,cx:&mut StepContext<'_>)->Result<Option<Fault>,OriginalTurnFailure>{
  let DslValue::Array(causes)=value else{return Err(invalid())};let output=self.output.as_mut().unwrap();
  if !self.reserved{if !CompletedInput::reserve(&mut output.causes,causes.len(),cx)?{return Ok(None)}self.reserved=true;return Ok(None)}
  if self.cause==causes.len(){if !metadata(cx)?{return Ok(None)}self.kind=0;self.field+=1;self.cause=0;self.cause_field=0;self.reserved=false;return Ok(None)}
  if output.causes.len()==self.cause{if !metadata(cx)?{return Ok(None)}output.causes.push(FaultCause{retained_progress:None,message:String::new(),code:None});self.cause_seen=false;return Ok(None)}
  let fields=object(&mut causes[self.cause])?;let target=&mut output.causes[self.cause];
  if self.cause_field==fields.len(){if !self.cause_seen{return Err(invalid())}if !metadata(cx)?{return Ok(None)}self.cause+=1;self.cause_field=0;return Ok(None)}
  let(name,value)=&mut fields[self.cause_field];
  if name=="retainedProgress"&&!matches!(value,DslValue::Null){let fields=object(value)?;if self.nested==0&&target.retained_progress.is_none(){if !metadata(cx)?{return Ok(None)}target.retained_progress=Some(Default::default());self.progress_seen=0;return Ok(None)}if self.nested==fields.len(){if self.progress_seen!=15{return Err(invalid())}if !metadata(cx)?{return Ok(None)}self.nested=0;self.cause_field+=1;return Ok(None)}if !metadata(cx)?{return Ok(None)}let(name,value)=&fields[self.nested];progress(target.retained_progress.as_mut().unwrap(),&mut self.progress_seen,name,value)?;self.nested+=1;return Ok(None)}
  if !metadata(cx)?{return Ok(None)}match name.as_str(){"message"=>{target.message=take_text(value)?;self.cause_seen=true},"code"=>if !matches!(value,DslValue::Null){target.code=Some(FaultCode(take_text(value)?))},"retainedProgress"=>{},_=>return Err(invalid())}self.cause_field+=1;Ok(None)
 }
 fn params_step(&mut self,value:&mut DslValue,cx:&mut StepContext<'_>)->Result<Option<Fault>,OriginalTurnFailure>{
  let fields=object(value)?;let output=self.output.as_mut().unwrap();if fields.is_empty(){if !metadata(cx)?{return Ok(None)}self.kind=0;self.field+=1;return Ok(None)}
  if output.params.is_none(){let bytes=std::mem::size_of::<FaultParams>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}cx.consume_fuel(1);output.params=Some(Box::default());cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(None)}
  let params=&mut output.params.as_mut().unwrap().0;
  if !self.reserved{if !CompletedInput::reserve(params,fields.len(),cx)?{return Ok(None)}self.reserved=true;return Ok(None)}
  if self.nested==fields.len(){if !metadata(cx)?{return Ok(None)}self.kind=0;self.field+=1;self.nested=0;self.reserved=false;return Ok(None)}
  let(name,value)=&mut fields[self.nested];if self.cause_field<name.len(){if !metadata(cx)?{return Ok(None)}let byte=name.as_bytes()[self.cause_field];if !(if self.cause_field==0{byte.is_ascii_lowercase()}else{byte.is_ascii_alphanumeric()}){return Err(invalid())}self.cause_field+=1;return Ok(None)}if name.is_empty(){return Err(invalid())}if !metadata(cx)?{return Ok(None)}let text=take_text(value)?;params.push((std::mem::take(name),text));self.nested+=1;self.cause_field=0;Ok(None)
 }
 pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(owner)=self.closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if !metadata(cx)?{return Ok(false)}self.closing.take();return Ok(false)}
  if self.output.is_some(){if !metadata(cx)?{return Ok(false)}let output=self.output.take().unwrap();match ControlledRetirement::new(output){Ok(owner)=>*self.closing=Some(owner),Err((error,output))=>{*self.output=Some(output);return Err(error.into())}}return Ok(false)}
  if !metadata(cx)?{return Ok(false)}self.closed=true;Ok(true)
 }
}
fn invalid()->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvalidValue,"original Fault field violates its closed typed schema").into()}
fn object(value:&mut DslValue)->Result<&mut Vec<(String,DslValue)>,OriginalTurnFailure>{match value{DslValue::Object(fields)=>Ok(fields),_=>Err(invalid())}}
fn take_text(value:&mut DslValue)->Result<String,OriginalTurnFailure>{if !matches!(value,DslValue::String(_)){return Err(invalid())}let DslValue::String(text)=std::mem::replace(value,DslValue::Null)else{unreachable!()};Ok(text)}
fn unsigned(value:&DslValue)->Result<u64,OriginalTurnFailure>{value.as_u64().ok_or_else(invalid)}
fn progress(target:&mut RetainedCloneProgress,seen:&mut u8,name:&str,value:&DslValue)->Result<(),OriginalTurnFailure>{let(index,slot)=match name{"copiedItems"=>(0,&mut target.copied_items),"copiedBytes"=>(1,&mut target.copied_bytes),"retainedCapacityBytes"=>(2,&mut target.retained_capacity_bytes),"releasedBytes"=>(3,&mut target.released_bytes),_=>return Err(invalid())};*slot=usize::try_from(unsigned(value)?).map_err(|_|invalid())?;*seen|=1<<index;Ok(())}
impl Drop for FaultCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.closed,"original Fault binder requires complete typed source retirement")}}
