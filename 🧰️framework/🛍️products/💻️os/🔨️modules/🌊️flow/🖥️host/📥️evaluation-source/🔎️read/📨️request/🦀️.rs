//! 📨️ Tessellation wire borrows the admitted original output and carries no reusable authority.
use super::*;
use semio_framework_pack_json::{JsonBorrowedWriteCursor,JsonWriteNode,JsonWriteSource};
#[derive(Clone,Copy,PartialEq,Eq)]
struct RequestBinding{text:[usize;6],tolerance:u64,node_hash:u64,output:[usize;8],revision:[u8;32]}
const SOURCE_CHECK_BYTES:usize=size_of::<RequestBinding>()+size_of::<(u64,u64)>();
/// 📥️ A request view lends exact original channel text to the existing registered tessellation capability.
pub struct FlowPreviewTessellationRequest<'a>{handle:&'a str,output_binding:&'a[usize;8],output_revision:&'a[u8;32],output_origin:&'a(u64,u64),window_id:&'a str,window_kind_id:&'a str,tolerance:f64,node_hash:u64}
impl<'a> FlowPreviewTessellationRequest<'a>{
 pub fn new(output:&'a FlowCurrentOutput<'a>,window_id:&'a str,window_kind_id:&'a str,tolerance:f64,node_hash:u64)->Result<Self,ValueError>{if output.preview_handle().is_none()||window_id.is_empty()||window_kind_id.is_empty()||!tolerance.is_finite()||tolerance<=0.0{return Err(refusal("original preview tessellation source is invalid"))}Ok(Self{handle:output.preview_handle().unwrap(),output_binding:&output.binding,output_revision:&output.revision,output_origin:&output.origin,window_id,window_kind_id,tolerance,node_hash})}
 pub fn from_source(source:&'a FlowPreviewSource,window_id:&'a str,window_kind_id:&'a str,tolerance:f64,node_hash:u64)->Result<Self,ValueError>{if window_id.is_empty()||window_kind_id.is_empty()||!tolerance.is_finite()||tolerance<=0.0{return Err(refusal("original preview tessellation source is invalid"))}Ok(Self{handle:source.original_handle(),output_binding:source.read.binding.as_ref().unwrap(),output_revision:&source.read.revision,output_origin:source.read.origin.as_ref().unwrap(),window_id,window_kind_id,tolerance,node_hash})}
 fn binding(&self)->RequestBinding{let handle=self.handle;RequestBinding{text:[handle.as_ptr()as usize,handle.len(),self.window_id.as_ptr()as usize,self.window_id.len(),self.window_kind_id.as_ptr()as usize,self.window_kind_id.len()],tolerance:self.tolerance.to_bits(),node_hash:self.node_hash,output:*self.output_binding,revision:*self.output_revision}}
}
impl JsonWriteSource for FlowPreviewTessellationRequest<'_>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{if path.is_empty(){return Ok(JsonWriteNode::Object(5))}if path.len()!=1{return Err(refusal("original tessellation request field is absent"))}Ok(match path[0]{0=>JsonWriteNode::String(self.handle),1=>JsonWriteNode::Number(semio_framework_value::Number::Float(self.tolerance)),2=>JsonWriteNode::Number(semio_framework_value::Number::UInt(self.node_hash)),3=>JsonWriteNode::String(self.window_id),4=>JsonWriteNode::String(self.window_kind_id),_=>return Err(refusal("original tessellation request field is absent"))})}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if !path.is_empty(){return Err(refusal("original tessellation request object is absent"))}["handle","tolerance","nodeHash","windowId","windowKindId"].get(index).copied().ok_or_else(||refusal("original tessellation request key is absent"))}
}
#[derive(semio_framework_value::RetireOwned)]
struct WriterSources{writer:JsonBorrowedWriteCursor,control:Option<semio_framework_value::NativeEncodeContinuation>}
/// 🧵️ The registered caller retains every accepted request byte until funded transfer or full close.
pub struct FlowPreviewTessellationWriter{sources:ManuallyDrop<Option<WriterSources>>,retirement:Option<ControlledRetirement<WriterSources>>,binding:Option<RequestBinding>,origin:Option<(u64,u64)>,completed:bool,closing:bool,progress:RetainedCloneProgress}
impl FlowPreviewTessellationWriter{
 pub fn new()->Self{Self{sources:ManuallyDrop::new(Some(WriterSources{writer:JsonBorrowedWriteCursor::new(),control:None})),retirement:None,binding:None,origin:None,completed:false,closing:false,progress:Default::default()}}
 pub fn next_demands(&self,source:&FlowPreviewTessellationRequest<'_>)->Result<RetirementDemand,ValueError>{if self.closing{return Err(refusal("original tessellation request is closing"))}let mut d=self.sources.as_ref().unwrap().writer.normal_step_demands(source)?;d.copy_bytes=d.copy_bytes.checked_add(SOURCE_CHECK_BYTES).ok_or_else(||refusal("original tessellation request body overflows"))?;Ok(d)}
 pub fn step(&mut self,source:&FlowPreviewTessellationRequest<'_>,cx:&mut StepContext<'_>)->Result<Option<String>,ValueError>{
  self.progress=Default::default();if self.closing{return Err(refusal("original tessellation request is closing"))}if self.completed||cx.is_cancelled()||cx.should_yield(){return Ok(None)}let grant=cx.retained_grant();if !admitted(self.next_demands(source)?,grant){return Ok(None)}let origin=(cx.operation().0,cx.generation().0);let binding=source.binding();if self.binding.is_some_and(|prior|prior!=binding)||self.origin.is_some_and(|prior|prior!=origin)||*source.output_origin!=origin{self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:SOURCE_CHECK_BYTES,..Default::default()};cx.consume_retained(self.progress)?;cx.consume_fuel(1);return Err(refusal("original tessellation request source or context changed").with_retained_progress(self.progress))}self.binding=Some(binding);self.origin=Some(origin);
  let fields=self.sources.as_mut().unwrap();let mut receive=|_|true;let mut control=match fields.control.take(){Some(saved)=>semio_framework_value::NativeEncodeControl::resume(saved,&mut receive)?,None=>semio_framework_value::NativeEncodeControl::new_retained(&mut receive)};let result=fields.writer.step(source,1,&mut control,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-SOURCE_CHECK_BYTES,..grant});self.progress=fields.writer.normal_step_progress();fields.control=Some(control.pause()?);self.progress.copied_items=self.progress.copied_items.max(1);self.progress.copied_bytes=self.progress.copied_bytes.checked_add(SOURCE_CHECK_BYTES).ok_or_else(||refusal("original tessellation request receipt overflows"))?;cx.consume_retained(self.progress)?;cx.consume_fuel(1);let result=result.map_err(|error|error.with_retained_progress(self.progress))?;if result.is_some(){self.completed=true;}Ok(result)
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(self.sources.is_some()),..Default::default()}),|owner|FlowEvaluationSource::demand(owner,copy))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();if !self.closing{return Err(refusal("original tessellation request close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !admitted(self.next_close_demands(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}self.retirement=Some(ControlledRetirement::new(self.sources.take().unwrap()).unwrap_or_else(|_|unreachable!()));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.sources.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowPreviewTessellationWriter{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tessellation writer requires complete explicit custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.sources)}}}}

struct WriterRetirement(FlowPreviewTessellationWriter);
impl RetirementCursor for WriterRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Ok(step)if step.progress()!=Default::default()=>RetirementStep::Progress(step.progress()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.retirement.as_ref().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
 fn next_selected_work_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.0.next_close_demands(copy)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for FlowPreviewTessellationWriter{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(WriterRetirement(self))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<WriterRetirement>())}fn controlled_retirement_supported()->bool{true}}
