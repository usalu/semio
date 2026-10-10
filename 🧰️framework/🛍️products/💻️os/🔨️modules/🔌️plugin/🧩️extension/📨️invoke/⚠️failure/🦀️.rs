//! 🧯️ Original extension causes project directly to the existing Fault wire under caller authority.
use semio_framework_diagnostic::{Fault,FaultCause,FaultOrigin,FaultParams,FaultScope,Severity,TextSpan};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
use semio_framework_job::StepContext;
use semio_framework_os_kernel::record::{BorrowedProjectedPackCursor,BorrowedProjectedPackFailure};
use semio_framework_value::{Number,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement}};

pub(crate) enum ExtensionInvocationCause{Fault(Fault),Value(ValueError),Inference(crate::ArtifactInferenceGatewayFailure)}
impl RetireOwned for ExtensionInvocationCause{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Fault(value)=>value.retirement(),Self::Value(value)=>value.retirement(),Self::Inference(value)=>value.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Fault(value)=>value.retirement_birth_bytes(),Self::Value(value)=>value.retirement_birth_bytes(),Self::Inference(value)=>value.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}

enum Node<'a>{Root(&'a ExtensionInvocationCause),Scope(&'a FaultScope),Progress(RetainedCloneProgress),Span(&'a TextSpan),Causes(&'a [FaultCause]),Cause(&'a FaultCause),Params(&'a FaultParams),Text(&'a str),Bool(bool),UInt(u64),Empty}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation diagnostic ordinal changed")}
fn root_fields(cause:&ExtensionInvocationCause)->usize{match cause{ExtensionInvocationCause::Fault(fault)=>7+usize::from(fault.span.is_some())+usize::from(!fault.causes.is_empty())+usize::from(fault.params.as_deref().is_some_and(|params|!params.0.is_empty())),ExtensionInvocationCause::Value(_)|ExtensionInvocationCause::Inference(_)=>7}}
fn root_key(cause:&ExtensionInvocationCause,index:usize)->Result<&'static str,ValueError>{
 if index<6{return Ok(["origin","code","severity","message","scope","retainedProgress"][index])}
 let mut next=6;if let ExtensionInvocationCause::Fault(fault)=cause{for(present,key)in [(fault.span.is_some(),"span"),(!fault.causes.is_empty(),"causes"),(fault.params.as_deref().is_some_and(|params|!params.0.is_empty()),"params")]{if present{if index==next{return Ok(key)}next+=1}}}if index==next{Ok("retryable")}else{Err(invalid())}
}
impl<'a> Node<'a>{
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Root(cause)=>match cause{
   ExtensionInvocationCause::Inference(error)=>{let(code,message,progress)=error.fault_wire_fields();match root_key(cause,index)?{"origin"=>Self::Text("plugin"),"code"=>Self::Text(code),"severity"=>Self::Text("error"),"message"=>Self::Text(message),"scope"=>Self::Empty,"retainedProgress"=>Self::Progress(progress),"retryable"=>Self::Bool(false),_=>return Err(invalid())}},
   ExtensionInvocationCause::Value(error)=>match root_key(cause,index)?{"origin"=>Self::Text("framework"),"code"=>Self::Text(error.kind.as_str()),"severity"=>Self::Text("error"),"message"=>Self::Text(&error.message),"scope"=>Self::Empty,"retainedProgress"=>Self::Progress(error.retained_progress()),"retryable"=>Self::Bool(false),_=>return Err(invalid())},
   ExtensionInvocationCause::Fault(fault)=>match root_key(cause,index)?{"origin"=>Self::Text(match fault.origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"}),"code"=>Self::Text(&fault.code.0),"severity"=>Self::Text(match fault.severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"}),"message"=>Self::Text(&fault.message),"scope"=>Self::Scope(&fault.scope),"retainedProgress"=>Self::Progress(fault.retained_progress),"span"=>Self::Span(fault.span.as_ref().ok_or_else(invalid)?),"causes"=>Self::Causes(&fault.causes),"params"=>Self::Params(fault.params.as_deref().ok_or_else(invalid)?),"retryable"=>Self::Bool(fault.retryable),_=>return Err(invalid())}},
  Self::Scope(scope)=>{let mut present=0;let mut selected=None;for value in [&scope.plugin_id,&scope.app_id,&scope.instance_id,&scope.module,&scope.body_key]{if let Some(value)=value{if present==index{selected=Some(value.as_str());break}present+=1}}Self::Text(selected.ok_or_else(invalid)?)},
  Self::Progress(progress)=>Self::UInt(*[progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes].get(index).ok_or_else(invalid)? as u64),
  Self::Span(span)=>Self::UInt(u64::from(*[span.line,span.column,span.length].get(index).ok_or_else(invalid)?)),
  Self::Causes(causes)=>Self::Cause(causes.get(index).ok_or_else(invalid)?),
  Self::Cause(cause)=>match index{0=>Self::Text(&cause.message),1=>Self::Text(&cause.code.as_ref().ok_or_else(invalid)?.0),_=>return Err(invalid())},
  Self::Params(params)=>Self::Text(&params.0.get(index).ok_or_else(invalid)?.1),_=>return Err(invalid())
 })}
 fn view(self)->V<'a>{match self{Self::Root(cause)=>V::IntrinsicObject(root_fields(cause)),Self::Scope(scope)=>V::IntrinsicObject([&scope.plugin_id,&scope.app_id,&scope.instance_id,&scope.module,&scope.body_key].into_iter().filter(|value|value.is_some()).count()),Self::Progress(_)=>V::IntrinsicObject(4),Self::Span(_)=>V::IntrinsicObject(3),Self::Causes(causes)=>V::IntrinsicArray(causes.len()),Self::Cause(cause)=>V::IntrinsicObject(1+usize::from(cause.code.is_some())),Self::Params(params)=>V::IntrinsicObject(params.0.len()),Self::Text(text)=>V::IntrinsicText(text),Self::Bool(value)=>V::IntrinsicBool(value),Self::UInt(value)=>V::IntrinsicNumber(Number::UInt(value)),Self::Empty=>V::IntrinsicObject(0)}}
 fn key(self,index:usize)->Result<&'a str,ValueError>{match self{Self::Root(cause)=>root_key(cause,index),Self::Scope(scope)=>{let mut present=0;for(value,key)in[(&scope.plugin_id,"pluginId"),(&scope.app_id,"appId"),(&scope.instance_id,"instanceId"),(&scope.module,"module"),(&scope.body_key,"bodyKey")]{if value.is_some(){if present==index{return Ok(key)}present+=1}}Err(invalid())},Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),Self::Span(_)=>["line","column","length"].get(index).copied().ok_or_else(invalid),Self::Cause(cause)=>match index{0=>Ok("message"),1 if cause.code.is_some()=>Ok("code"),_=>Err(invalid())},Self::Params(params)=>params.0.get(index).map(|entry|entry.0.as_str()).ok_or_else(invalid),_=>Err(invalid())}}
}
impl ExtensionInvocationCause{
 fn node(&self,path:&[usize])->Result<Node<'_>,ValueError>{let Some((&0,rest))=path.split_first()else{return Err(invalid())};let mut node=Node::Root(self);for &index in rest{node=node.child(index)?}Ok(node)}
}
impl FieldProjectionSource for ExtensionInvocationCause{
 fn projection_identity(&self)->usize{match self{Self::Fault(fault)=>fault.scope.as_ref()as *const FaultScope as usize,Self::Value(error)=>error.message.as_ptr()as usize,Self::Inference(error)=>error.source_identity()}}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{if path.is_empty(){Ok(V::Record(&[1]))}else{Ok(self.node(path)?.view())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.node(path)?.key(index)}
}

/// 🎒️ Measurement, output, symbol release and complete typed cause closure use one original wallet.
pub(crate) struct RetainedExtensionFaultReply{cause:Option<ExtensionInvocationCause>,closing:Option<ControlledRetirement<ExtensionInvocationCause>>,output_close:Option<ControlledRetirement<Vec<u8>>>,cancelled:bool,cursor:BorrowedProjectedPackCursor,bytes:Vec<u8>,scratch:[u8;64],pending_length:usize,pending_offset:usize,measured:usize,phase:u8}
impl RetainedExtensionFaultReply{
 pub(crate) fn new(cause:ExtensionInvocationCause)->Self{Self{cause:Some(cause),closing:None,output_close:None,cancelled:false,cursor:Default::default(),bytes:Vec::new(),scratch:[0;64],pending_length:0,pending_offset:0,measured:0,phase:0}}
 fn debit(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),ValueError>{cx.consume_retained(progress)}
 fn pack_failure(error:BorrowedProjectedPackFailure,cx:&mut StepContext<'_>)->ValueError{let progress=error.reason.retained_progress();if let Err(refusal)=cx.consume_retained(progress){return refusal}error.reason}
 /// 📏️ Quotes only the next actual symbol, output allocation or typed original cause close.
 pub(crate) fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{Ok(match self.phase{9=>self.cursor.retirement_demands()?,13=>if let Some(closing)=&self.output_close{RetirementDemand{copy_bytes:closing.next_copy_byte_demand()?,capacity_bytes:closing.next_capacity_byte_demand(copy)?,release_bytes:closing.next_release_byte_demand()?,depth:closing.next_depth_demand()?,..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}},0|3=>RetirementDemand{copy_bytes:if self.phase==3&&self.pending_offset<self.pending_length{1}else{self.cursor.next_minimum_copy_bytes()},capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:self.cursor.next_advance_depth_demand()?,..Default::default()},1|4=>self.cursor.retirement_demands()?,2=>RetirementDemand{capacity_bytes:self.measured,depth:1,..Default::default()},6=>if let Some(closing)=&self.closing{RetirementDemand{copy_bytes:closing.next_copy_byte_demand()?,capacity_bytes:closing.next_capacity_byte_demand(copy)?,release_bytes:closing.next_release_byte_demand()?,depth:closing.next_depth_demand()?,..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}},8=>Default::default(),_=>RetirementDemand{depth:1,..Default::default()}})}
 /// 🛑️ Keeps the original diagnostic, symbols and output backing until their paid cancellation close.
 pub(crate) fn begin_close(&mut self){if self.phase==8{return}self.cancelled=true;self.phase=if self.closing.is_some(){6}else{9};}
 /// 📨️ Publishes only exact initialized original fault bytes after every cause and symbol owner closed.
 pub(crate) fn advance(&mut self,cx:&mut StepContext<'_>)->Result<Option<Vec<u8>>,ValueError>{
  let grant=cx.retained_grant();if (cx.is_cancelled()&&!self.cancelled)||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  match self.phase{
   0|3=>{
    if self.phase==3&&self.pending_offset<self.pending_length{let count=(self.pending_length-self.pending_offset).min(grant.maximum_copy_bytes);if count==0{return Ok(None)}if self.bytes.capacity()-self.bytes.len()<count{return Err(invalid())}self.bytes.extend_from_slice(&self.scratch[self.pending_offset..self.pending_offset+count]);self.pending_offset+=count;Self::debit(cx,RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;return Ok(None)}
    if self.cursor.is_complete(){self.phase=if self.phase==0{1}else{4};return Ok(None)}
    let source=self.cause.as_ref().ok_or_else(invalid)?;let step=self.cursor.advance_intrinsic(source,1,&mut self.scratch,grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;
    if self.phase==0{self.measured=self.measured.checked_add(step.written_bytes).ok_or_else(invalid)?}else{self.pending_length=step.written_bytes;self.pending_offset=0}Ok(None)
   },
   1|4=>{let step=self.cursor.close(grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;if step.complete{self.phase=if self.phase==1{2}else{5}}Ok(None)},
   2=>{if grant.maximum_capacity_bytes<self.measured{return Ok(None)}let before=self.bytes.capacity();self.bytes.try_reserve_exact(self.measured).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original invocation diagnostic output allocation refused"))?;let capacity=self.bytes.capacity()-before;self.cursor=Default::default();self.phase=3;Self::debit(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;Ok(None)},
   5=>{let source=self.cause.take().ok_or_else(invalid)?;self.closing=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("original invocation cause declares typed retirement")));Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=6;Ok(None)},
   6=>{let closing=self.closing.as_mut().ok_or_else(invalid)?;if closing.terminal_is_empty(){self.closing=None;Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=if self.cancelled{12}else{7};return Ok(None)}let step=closing.step(grant).map_err(|error|{let progress=error.retained_progress();match cx.consume_retained(progress){Ok(())=>error,Err(refusal)=>refusal}})?;Self::debit(cx,step.progress())?;Ok(None)},
   9=>{if self.cursor.terminal_is_empty(){self.phase=if self.cause.is_some(){5}else{12};return Ok(None)}let step=self.cursor.close(grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;Ok(None)},
   12=>{let bytes=std::mem::take(&mut self.bytes);self.output_close=Some(ControlledRetirement::new(bytes).unwrap_or_else(|_|unreachable!("original diagnostic bytes declare typed retirement")));Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=13;Ok(None)},
   13=>{let closing=self.output_close.as_mut().ok_or_else(invalid)?;if closing.terminal_is_empty(){self.output_close=None;Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.pending_length=0;self.pending_offset=0;self.phase=8;return Ok(None)}let step=closing.step(grant).map_err(|error|{let progress=error.retained_progress();match cx.consume_retained(progress){Ok(())=>error,Err(refusal)=>refusal}})?;Self::debit(cx,step.progress())?;Ok(None)},
   7=>{if self.bytes.len()!=self.measured{return Err(invalid())}Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=8;Ok(Some(std::mem::take(&mut self.bytes)))},_=>Ok(None)
  }
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.phase==8&&self.cause.is_none()&&self.closing.is_none()&&self.output_close.is_none()&&self.cursor.terminal_is_empty()&&self.bytes.capacity()==0}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
