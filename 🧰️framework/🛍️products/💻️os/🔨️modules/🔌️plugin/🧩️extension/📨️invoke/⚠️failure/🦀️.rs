//! 🧯️ Original extension causes project directly to the existing Fault wire under caller authority.
use semio_framework_diagnostic::{Fault, FaultCause, FaultOrigin, FaultParams, FaultScope, FaultSource, Severity, TextSpan};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V};
use semio_framework_job::{StepContext, OriginalTurnFailure};
use semio_framework_os_kernel::record::{BorrowedProjectedPackCursor, BorrowedProjectedPackFailure};
use semio_framework_value::{Number, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;

#[doc(hidden)]
pub enum ExtensionInvocationCause{Fault(Fault),Value(ValueError),Turn(OriginalTurnFailure),Inference(crate::artifact_inference::ArtifactInferenceGatewayRefusal)}
impl RetireOwned for ExtensionInvocationCause{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Fault(value)=>value.retirement(),Self::Value(value)=>value.retirement(),Self::Turn(value)=>value.retirement(),Self::Inference(value)=>value.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Fault(value)=>value.retirement_birth_bytes(),Self::Value(value)=>value.retirement_birth_bytes(),Self::Turn(value)=>value.retirement_birth_bytes(),Self::Inference(value)=>value.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}

/// 🫴️ An immutable view preserves the defining full cause projection without moving its original owner.
#[derive(Clone,Copy)]
pub(crate) enum OriginalCauseLoan<'a>{Fault(&'a Fault),Value(&'a ValueError),Turn(&'a OriginalTurnFailure),Inference(&'a crate::artifact_inference::ArtifactInferenceGatewayRefusal)}
#[doc(hidden)]
pub enum Node<'a>{Root(OriginalCauseLoan<'a>),Scope(&'a FaultScope),Progress(RetainedCloneProgress),Span(&'a TextSpan),Source(FaultSource),Causes(&'a [FaultCause],[Option<&'a ValueError>;2]),Cause(&'a FaultCause),ReceiptCauses([Option<&'a ValueError>;2]),ReceiptCause(&'a ValueError),Params(&'a FaultParams),Text(&'a str),Bool(bool),UInt(u64),Empty}
#[doc(hidden)]
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation diagnostic ordinal changed")}
fn fault_loan<'a>(cause:OriginalCauseLoan<'a>)->Option<(&'a Fault,[Option<&'a ValueError>;2])>{match cause{OriginalCauseLoan::Fault(fault)=>Some((fault,[None,None])),OriginalCauseLoan::Turn(error)=>error.primary_fault().map(|fault|(fault,[error.receipt_refusal.as_ref(),error.context_refusal.as_ref()])),_=>None}}
#[derive(Clone,Copy)]
enum DomainLoan<'a>{Source(&'a dyn FieldProjectionSource),Receiving(&'a OriginalTurnFailure)}
const RECEIVING_PREFIX:[usize;2]=[0,1];
impl<'a> DomainLoan<'a>{
 fn projection_view(self,path:&[usize])->Result<V<'a>,ValueError>{match self{Self::Source(source)=>source.projection_view(path),Self::Receiving(error)=>error.projection_view(&[&RECEIVING_PREFIX[..],path].concat())}}
 fn projection_key(self,path:&[usize],index:usize)->Result<&'a str,ValueError>{match self{Self::Source(source)=>source.projection_key(path,index),Self::Receiving(error)=>error.projection_key(&[&RECEIVING_PREFIX[..],path].concat(),index)}}
}
fn domain_loan<'a>(cause:OriginalCauseLoan<'a>)->Option<DomainLoan<'a>>{
 if let Some((fault,_))=fault_loan(cause){return fault.domain.as_ref().map(|domain|DomainLoan::Source(domain as &dyn FieldProjectionSource))}
 match cause{OriginalCauseLoan::Turn(error)=>if error.primary_receiving().is_some(){Some(DomainLoan::Receiving(error))}else{error.primary_domain().map(|domain|DomainLoan::Source(domain.projection()))},OriginalCauseLoan::Inference(error)=>match &error.primary{crate::artifact_inference::ArtifactInferenceGatewayFailure::Execution(original)=>Some(DomainLoan::Source(original)),_=>None},_=>None}
}
#[doc(hidden)]
pub fn extras(cause:OriginalCauseLoan<'_>)->[(bool,&'static str);5]{
 if let Some((fault,secondary))=fault_loan(cause){return [(fault.source.is_some(),"source"),(fault.span.is_some(),"span"),(!fault.causes.is_empty()||secondary.iter().any(Option::is_some),"causes"),(fault.params.as_deref().is_some_and(|params|!params.0.is_empty()),"params"),(fault.domain.is_some(),"domain")]}
 match cause{OriginalCauseLoan::Turn(error)=>[(error.primary_receiving().is_some()||error.primary_domain().is_some(),"domain"),(error.receipt_refusal.is_some()||error.context_refusal.is_some(),"causes"),(false,""),(false,""),(false,"")],OriginalCauseLoan::Inference(error)=>[(error.primary.fault_source().is_some(),"source"),(error.receipt_refusal.is_some(),"causes"),(domain_loan(cause).is_some(),"domain"),(false,""),(false,"")],_=>[(false,"");5]}
}
fn root_fields(cause:OriginalCauseLoan<'_>)->usize{7+extras(cause).into_iter().filter(|(present,_)|*present).count()}
#[doc(hidden)]
pub fn root_key(cause:OriginalCauseLoan<'_>,index:usize)->Result<&'static str,ValueError>{if index<6{return Ok(["origin","code","severity","message","scope","retainedProgress"][index])}let extras=extras(cause);let count=extras.iter().filter(|(present,_)|*present).count();if index==6+count{return Ok("retryable")}extras.into_iter().filter(|(present,_)|*present).nth(index-6).map(|(_,key)|key).ok_or_else(invalid)}
fn domain_path<'a>(cause:OriginalCauseLoan<'a>,path:&[usize])->Option<DomainLoan<'a>>{let first=path.first()?;(root_key(cause,*first).ok()==Some("domain")).then(||domain_loan(cause)).flatten()}
impl<'a> Node<'a>{
 fn fault_child(fault:&'a Fault,secondary:[Option<&'a ValueError>;2],key:&str)->Result<Self,ValueError>{Ok(match key{"origin"=>Self::Text(match fault.origin{FaultOrigin::Edge=>"edge",FaultOrigin::Renderer=>"renderer",FaultOrigin::Os=>"os",FaultOrigin::Module=>"module",FaultOrigin::Plugin=>"plugin",FaultOrigin::App=>"app",FaultOrigin::Extension=>"extension",FaultOrigin::Framework=>"framework"}),"code"=>Self::Text(&fault.code.0),"severity"=>Self::Text(match fault.severity{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"}),"message"=>Self::Text(&fault.message),"scope"=>Self::Scope(&fault.scope),"retainedProgress"=>Self::Progress(fault.retained_progress),"source"=>Self::Source(fault.source.ok_or_else(invalid)?),"span"=>Self::Span(fault.span.as_ref().ok_or_else(invalid)?),"causes"=>Self::Causes(&fault.causes,secondary),"params"=>Self::Params(fault.params.as_deref().ok_or_else(invalid)?),"retryable"=>Self::Bool(fault.retryable),_=>return Err(invalid())})}

 pub fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Root(cause)=>if let Some((fault,secondary))=fault_loan(cause){if root_key(cause,index)?=="retainedProgress"{Self::Progress(match cause{OriginalCauseLoan::Turn(error)=>error.retained_progress(),_=>fault.retained_progress()})}else{Self::fault_child(fault,secondary,root_key(cause,index)?)?}}else{match cause{
   OriginalCauseLoan::Inference(error)=>{let(code,message,progress)=error.primary.fault_wire_fields();match root_key(cause,index)?{"origin"=>Self::Text("plugin"),"code"=>Self::Text(code),"severity"=>Self::Text("error"),"message"=>Self::Text(message),"scope"=>Self::Empty,"retainedProgress"=>Self::Progress(progress),"source"=>Self::Source(error.primary.fault_source().ok_or_else(invalid)?),"causes"=>Self::ReceiptCauses([error.receipt_refusal.as_ref(),None]),"retryable"=>Self::Bool(false),_=>return Err(invalid())}},
   OriginalCauseLoan::Turn(error)=>{let(code,message)=if let Some(primary)=error.primary_value(){(primary.kind.as_str(),primary.message.as_ref())}else if let Some(primary)=error.primary_domain(){(primary.code(),primary.message())}else if let Some(receiving)=error.primary_receiving(){("original.receiving",receiving.primary.as_ref().or(receiving.quote_refusal.as_ref()).or(receiving.allocation_refusal.as_ref()).map_or("Original receiving receipt refused",|cause|cause.message.as_ref()))}else{return Err(invalid())};match root_key(cause,index)?{"origin"=>Self::Text("framework"),"code"=>Self::Text(code),"severity"=>Self::Text("error"),"message"=>Self::Text(message),"scope"=>Self::Empty,"retainedProgress"=>Self::Progress(error.retained_progress()),"causes"=>Self::ReceiptCauses([error.receipt_refusal.as_ref(),error.context_refusal.as_ref()]),"retryable"=>Self::Bool(false),_=>return Err(invalid())}},
   OriginalCauseLoan::Value(error)=>match root_key(cause,index)?{"origin"=>Self::Text("framework"),"code"=>Self::Text(error.kind.as_str()),"severity"=>Self::Text("error"),"message"=>Self::Text(&error.message),"scope"=>Self::Empty,"retainedProgress"=>Self::Progress(error.retained_progress()),"retryable"=>Self::Bool(false),_=>return Err(invalid())},
   OriginalCauseLoan::Fault(_)=>return Err(invalid())}},
  Self::Scope(scope)=>{let mut present=0;let mut selected=None;for value in [&scope.plugin_id,&scope.app_id,&scope.instance_id,&scope.module,&scope.body_key]{if let Some(value)=value{if present==index{selected=Some(value.as_str());break}present+=1}}Self::Text(selected.ok_or_else(invalid)?)},
  Self::Progress(progress)=>Self::UInt(*[progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes].get(index).ok_or_else(invalid)? as u64),
  Self::Source(source)=>Self::UInt([source.offset,source.byte.map(u64::from),source.depth].into_iter().flatten().nth(index).ok_or_else(invalid)?),
  Self::Span(span)=>Self::UInt(u64::from(*[span.line,span.column,span.length].get(index).ok_or_else(invalid)?)),
  Self::Causes(causes,secondary)=>if let Some(cause)=causes.get(index){Self::Cause(cause)}else{Self::ReceiptCause(secondary.into_iter().flatten().nth(index-causes.len()).ok_or_else(invalid)?)},
  Self::Cause(cause)=>{if index==0{Self::Text(&cause.message)}else if cause.code.is_some()&&index==1{Self::Text(&cause.code.as_ref().unwrap().0)}else if index==1+usize::from(cause.code.is_some()){Self::Progress(cause.retained_progress.ok_or_else(invalid)?)}else{return Err(invalid())}},
  Self::ReceiptCauses(errors)=>Self::ReceiptCause(errors.into_iter().flatten().nth(index).ok_or_else(invalid)?),Self::ReceiptCause(error)=>match index{0=>Self::Text(&error.message),1=>Self::Text(error.kind.as_str()),2=>Self::Progress(error.retained_progress()),_=>return Err(invalid())},
  Self::Params(params)=>Self::Text(&params.0.get(index).ok_or_else(invalid)?.1),_=>return Err(invalid())
 })}
 pub fn view(self)->V<'a>{match self{Self::Root(cause)=>V::IntrinsicObject(root_fields(cause)),Self::Scope(scope)=>V::IntrinsicObject([&scope.plugin_id,&scope.app_id,&scope.instance_id,&scope.module,&scope.body_key].into_iter().filter(|value|value.is_some()).count()),Self::Progress(_)=>V::IntrinsicObject(4),Self::Span(_)=>V::IntrinsicObject(3),Self::Source(source)=>V::IntrinsicObject(usize::from(source.offset.is_some())+usize::from(source.byte.is_some())+usize::from(source.depth.is_some())),Self::Causes(causes,secondary)=>V::IntrinsicArray(causes.len()+secondary.into_iter().flatten().count()),Self::Cause(cause)=>V::IntrinsicObject(1+usize::from(cause.code.is_some())+usize::from(cause.retained_progress.is_some())),Self::ReceiptCauses(errors)=>V::IntrinsicArray(errors.into_iter().flatten().count()),Self::ReceiptCause(_)=>V::IntrinsicObject(3),Self::Params(params)=>V::IntrinsicObject(params.0.len()),Self::Text(text)=>V::IntrinsicText(text),Self::Bool(value)=>V::IntrinsicBool(value),Self::UInt(value)=>V::IntrinsicNumber(Number::UInt(value)),Self::Empty=>V::IntrinsicObject(0)}}
 pub fn key(self,index:usize)->Result<&'a str,ValueError>{match self{Self::Root(cause)=>root_key(cause,index),Self::Scope(scope)=>{let mut present=0;for(value,key)in[(&scope.plugin_id,"pluginId"),(&scope.app_id,"appId"),(&scope.instance_id,"instanceId"),(&scope.module,"module"),(&scope.body_key,"bodyKey")]{if value.is_some(){if present==index{return Ok(key)}present+=1}}Err(invalid())},Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),Self::Source(source)=>[("offset",source.offset.is_some()),("byte",source.byte.is_some()),("depth",source.depth.is_some())].into_iter().filter(|(_,present)|*present).nth(index).map(|(key,_)|key).ok_or_else(invalid),Self::Span(_)=>["line","column","length"].get(index).copied().ok_or_else(invalid),Self::Cause(cause)=>match index{0=>Ok("message"),1 if cause.code.is_some()=>Ok("code"),index if index==1+usize::from(cause.code.is_some())&&cause.retained_progress.is_some()=>Ok("retainedProgress"),_=>Err(invalid())},Self::ReceiptCause(_)=>["message","code","retainedProgress"].get(index).copied().ok_or_else(invalid),Self::Params(params)=>params.0.get(index).map(|entry|entry.0.as_str()).ok_or_else(invalid),_=>Err(invalid())}}
}
impl<'a> OriginalCauseLoan<'a>{
 pub fn node(self,path:&[usize])->Result<Node<'a>,ValueError>{let mut node=Node::Root(self);for &index in path{node=node.child(index)?}Ok(node)}
 fn view_at(self,path:&[usize])->Result<V<'a>,ValueError>{if let Some(domain)=domain_path(self,path){return domain.projection_view(&path[1..])}Ok(self.node(path)?.view())}
 pub fn key_at(self,path:&[usize],index:usize)->Result<&'a str,ValueError>{if let Some(domain)=domain_path(self,path){return domain.projection_key(&path[1..],index)}self.node(path)?.key(index)}
}
impl FieldProjectionSource for OriginalCauseLoan<'_>{
 fn projection_identity(&self)->usize{match self{Self::Fault(fault)=>fault.scope.as_ref()as *const FaultScope as usize,Self::Value(error)=>error.message.as_ptr()as usize,Self::Turn(error)=>*error as *const OriginalTurnFailure as usize,Self::Inference(error)=>error.primary.source_identity()}}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{self.view_at(path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.key_at(path,index)}
}
impl ExtensionInvocationCause{
 pub fn loan(&self)->OriginalCauseLoan<'_>{match self{Self::Fault(original)=>OriginalCauseLoan::Fault(original),Self::Value(original)=>OriginalCauseLoan::Value(original),Self::Turn(original)=>OriginalCauseLoan::Turn(original),Self::Inference(original)=>OriginalCauseLoan::Inference(original)}}
}
impl FieldProjectionSource for ExtensionInvocationCause{
 fn projection_identity(&self)->usize{match self{Self::Turn(original)=>original.primary_domain().map_or_else(||original.primary.message().as_ptr()as usize,|domain|domain.projection().projection_identity()),_=>self.loan().projection_identity()}}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{self.loan().view_at(path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.loan().key_at(path,index)}
}

/// 🎒️ Measurement, output, symbol release and complete typed cause closure use one original wallet.
#[doc(hidden)]
pub struct RetainedExtensionFaultReply{terminal_cancellation:bool,pub cause:Option<ExtensionInvocationCause>,pub closing:Option<ControlledRetirement<ExtensionInvocationCause>>,pub output_close:Option<ControlledRetirement<Vec<u8>>>,pub cancelled:bool,pub cursor:BorrowedProjectedPackCursor,pub bytes:Vec<u8>,pub scratch:[u8;64],pending_length:usize,pub pending_offset:usize,pub measured:usize,pub phase:u8}
impl RetainedExtensionFaultReply{
 pub fn new(cause:ExtensionInvocationCause)->Self{Self{terminal_cancellation:false,cause:Some(cause),closing:None,output_close:None,cancelled:false,cursor:Default::default(),bytes:Vec::new(),scratch:[0;64],pending_length:0,pending_offset:0,measured:0,phase:0}}
 /// 🛑️ The registered terminal cancellation reply keeps its already canceled original caller.
 pub fn new_terminal_cancellation(cause:ExtensionInvocationCause)->Self{let mut reply=Self::new(cause);reply.terminal_cancellation=true;reply}
 pub fn debit(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(())}
 fn pack_failure(error:BorrowedProjectedPackFailure,cx:&mut StepContext<'_>)->OriginalTurnFailure{cx.consume_fuel(1);OriginalTurnFailure::debit_original(error.reason,cx)}
 /// 📏️ Quotes only the next actual symbol, output allocation or typed original cause close.
 pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{Ok(match self.phase{9=>self.cursor.retirement_demands()?,13=>if let Some(closing)=&self.output_close{closing.selected_demand(copy)?}else{RetirementDemand{depth:1,..Default::default()}},0|3=>RetirementDemand{copy_bytes:if self.phase==3&&self.pending_offset<self.pending_length{1}else{self.cursor.next_minimum_copy_bytes()},capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:self.cursor.next_advance_depth_demand()?,..Default::default()},1|4=>self.cursor.retirement_demands()?,2=>RetirementDemand{capacity_bytes:self.measured,depth:1,..Default::default()},6=>if let Some(closing)=&self.closing{closing.selected_demand(copy)?}else{RetirementDemand{depth:1,..Default::default()}},8=>Default::default(),_=>RetirementDemand{depth:1,..Default::default()}})}
 /// 🛑️ Keeps the original diagnostic, symbols and output backing until their paid cancellation close.
 pub fn begin_close(&mut self){if self.phase==8{return}self.cancelled=true;self.phase=if self.closing.is_some(){6}else{9};}
 /// 📨️ Publishes only exact initialized original fault bytes after every cause and symbol owner closed.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<Option<Vec<u8>>,OriginalTurnFailure>{
  let remaining=cx.retained_grant();if cx.should_yield()||(cx.is_cancelled()&&!self.cancelled&&!self.terminal_cancellation)||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(None)}let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,..remaining};
  match self.phase{
   0|3=>{
    if self.phase==3&&self.pending_offset<self.pending_length{let count=(self.pending_length-self.pending_offset).min(grant.maximum_copy_bytes);if count==0{return Ok(None)}if self.bytes.capacity()-self.bytes.len()<count{return Err(invalid().into())}self.bytes.extend_from_slice(&self.scratch[self.pending_offset..self.pending_offset+count]);self.pending_offset+=count;Self::debit(cx,RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;return Ok(None)}
    if self.cursor.is_complete(){Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=if self.phase==0{1}else{4};return Ok(None)}
    let source=self.cause.as_ref().ok_or_else(invalid)?;let step=self.cursor.advance_intrinsic(source,1,&mut self.scratch,grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;
    if self.phase==0{self.measured=self.measured.checked_add(step.written_bytes).ok_or_else(invalid)?}else{self.pending_length=step.written_bytes;self.pending_offset=0}Ok(None)
   },
   1|4=>{let step=self.cursor.close(grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;if step.complete{self.phase=if self.phase==1{2}else{5}}Ok(None)},
   2=>{if grant.maximum_capacity_bytes<self.measured{return Ok(None)}let before=self.bytes.capacity();self.bytes.try_reserve_exact(self.measured).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original invocation diagnostic output allocation refused"))?;let capacity=self.bytes.capacity()-before;self.cursor=Default::default();self.phase=3;Self::debit(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;Ok(None)},
   5=>{let source=self.cause.take().ok_or_else(invalid)?;self.closing=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("original invocation cause declares typed retirement")));Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=6;Ok(None)},
   6=>{let closing=self.closing.as_mut().ok_or_else(invalid)?;if closing.terminal_is_empty(){self.closing=None;Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=if self.cancelled{12}else{7};return Ok(None)}let step=closing.step(grant).map_err(|error|{cx.consume_fuel(1);OriginalTurnFailure::debit_original(error,cx)})?;Self::debit(cx,step.progress())?;Ok(None)},
   9=>{if self.cursor.terminal_is_empty(){Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=if self.cause.is_some(){5}else{12};return Ok(None)}let step=self.cursor.close(grant).map_err(|error|Self::pack_failure(error,cx))?;Self::debit(cx,step.progress)?;Ok(None)},
   12=>{let bytes=std::mem::take(&mut self.bytes);self.output_close=Some(ControlledRetirement::new(bytes).unwrap_or_else(|_|unreachable!("original diagnostic bytes declare typed retirement")));Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=13;Ok(None)},
   13=>{let closing=self.output_close.as_mut().ok_or_else(invalid)?;if closing.terminal_is_empty(){self.output_close=None;Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.pending_length=0;self.pending_offset=0;self.phase=8;return Ok(None)}let step=closing.step(grant).map_err(|error|{cx.consume_fuel(1);OriginalTurnFailure::debit_original(error,cx)})?;Self::debit(cx,step.progress())?;Ok(None)},
   7=>{if self.bytes.len()!=self.measured{return Err(invalid().into())}Self::debit(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=8;Ok(Some(std::mem::take(&mut self.bytes)))},_=>Ok(None)
  }
 }
 pub fn terminal_is_empty(&self)->bool{self.phase==8&&self.cause.is_none()&&self.closing.is_none()&&self.output_close.is_none()&&self.cursor.terminal_is_empty()&&self.bytes.capacity()==0}
}

