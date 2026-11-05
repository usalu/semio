//! 📥️ The genuine pending evaluation becomes one funded immutable invocation origin.
use super::*;
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowSourceIdentity};
use std::mem::ManuallyDrop;
const VERSION_PREFIX:&[u8]=b"registry:";
const VERSION_SUFFIX:&[u8]=b";geometry:1;policy:1";
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct OriginFields{pending:Option<neural::PendingExtensionEval>,dependency:Option<String>,window_id:Option<String>,window_kind:Option<String>,identity:FlowInvocationIdentity,digits:Option<String>,version:Option<String>,buffer:Vec<u8>,origin:Option<FlowInvocationOrigin>,lease:Option<FlowInvocationOriginLease>}
pub struct FlowInvocationOriginWork{fields:ManuallyDrop<Option<OriginFields>>,dependency:Option<FlowInferenceDependencyWork>,retirement:Option<ControlledRetirement<OriginFields>>,binding:Option<(FlowSourceIdentity,usize,usize,usize,usize)>,operation:Option<semio_framework_job::OriginalSourceBinding>,phase:u8,offset:usize,closing:bool,progress:RetainedCloneProgress,work:u64}
impl FlowInvocationOriginWork{
 pub fn new(pending:neural::PendingExtensionEval,registry_generation:u64)->Self{Self{fields:ManuallyDrop::new(Some(OriginFields{pending:Some(pending),dependency:None,window_id:None,window_kind:None,identity:FlowInvocationIdentity::new(registry_generation),digits:None,version:None,buffer:Vec::new(),origin:None,lease:None})),dependency:Some(FlowInferenceDependencyWork::new()),retirement:None,binding:None,operation:None,phase:0,offset:0,closing:false,progress:Default::default(),work:0}}
 fn check(&self,source:&dyn FlowSnapshotSource,window:&str,kind:&str)->Result<(),ValueError>{if self.binding.is_some_and(|binding|binding!=(source.source_identity(),window.as_ptr()as usize,window.len(),kind.as_ptr()as usize,kind.len())){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation origin scene or address changed"))}Ok(())}
 fn version_length(fields:&OriginFields)->usize{VERSION_PREFIX.len()+fields.digits.as_ref().unwrap().len()+VERSION_SUFFIX.len()}
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,window:&str,kind:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.check(source,window,kind)?;let fields=self.fields.as_ref().unwrap();
  if self.phase==0{let mut demand=self.dependency.as_ref().unwrap().next_demands(source,&fields.pending.as_ref().unwrap().neuron_id,copy)?;demand.depth+=1;return Ok(demand)}
  if self.phase==1{let mut demand=self.dependency.as_ref().unwrap().next_close_demands(copy)?;demand.depth+=1;return Ok(demand)}
  if self.phase==6&&!fields.identity.complete(){return Ok(fields.identity.demands())}
  let mut demand=RetirementDemand{depth:1,..Default::default()};
  match self.phase{2=>demand.capacity_bytes=window.len(),3=>demand.copy_bytes=usize::from(self.offset<window.len()),4=>demand.capacity_bytes=kind.len(),5=>demand.copy_bytes=usize::from(self.offset<kind.len()),7=>demand.capacity_bytes=Self::version_length(fields),8=>demand.copy_bytes=usize::from(self.offset<Self::version_length(fields)),10=>demand.capacity_bytes=FlowInvocationOriginLease::allocation_bytes(),_=>{}}Ok(demand)
 }
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,window:&str,kind:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FlowInvocationOriginLease>,semio_framework_job::OriginalTurnFailure>{
  let operation=semio_framework_job::OriginalSourceBinding::borrow(cx);if self.operation.is_some_and(|original|!original.matches(cx)){return Err(semio_framework_job::OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation origin operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let mut total=RetainedCloneProgress::default();let mut total_work=0;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){self.progress=total;self.work=total_work;return Ok(None)}
   let result=self.advance(source,window,kind,cx.retained_grant());let progress=self.progress;let work=self.work;total_work+=work;
   if work>0||progress!=Default::default(){self.operation=Some(operation)}
   cx.consume_fuel(work);
   let sum=match total.checked_add(progress){Ok(sum)=>sum,Err(error)=>{if let Ok(Some(output))=result{self.restore_output(output)}self.progress=total;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))}};
   match result{
    Ok(output)=>{if let Err(error)=cx.consume_retained(progress){if let Some(output)=output{self.restore_output(output)}self.progress=sum;self.work=total_work;return Err(error.into())}total=sum;if let Some(output)=output{self.progress=total;self.work=total_work;return Ok(Some(output))}if progress==Default::default(){self.progress=total;self.work=total_work;return Ok(None)}},
    Err(error)=>{self.progress=sum;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))},
   }
  }self.progress=total;self.work=total_work;Ok(None)
 }

 fn allocate(fields:&mut OriginFields,length:usize)->Result<RetainedCloneProgress,ValueError>{fields.buffer.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original invocation origin text allocation refused"))?;Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:fields.buffer.capacity(),..Default::default()})}
 pub fn advance(&mut self,source:&dyn FlowSnapshotSource,window:&str,kind:&str,grant:RetainedCloneGrant)->Result<Option<FlowInvocationOriginLease>,ValueError>{
  self.progress=Default::default();self.work=0;if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original invocation origin is closing"))}let demand=self.next_demands(source,window,kind,grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.binding=Some((source.source_identity(),window.as_ptr()as usize,window.len(),kind.as_ptr()as usize,kind.len()));self.work=1;self.progress.copied_items=1;let fields=self.fields.as_mut().unwrap();
  match self.phase{
   0=>{let child=self.dependency.as_mut().unwrap();let result=child.advance(source,&fields.pending.as_ref().unwrap().neuron_id,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();self.work=child.last_turn_work();if let Some(output)=result?{fields.dependency=Some(output);child.begin_close();self.phase=1}},
   1=>{let child=self.dependency.as_mut().unwrap();let result=child.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();self.work=child.last_turn_work();result?;if child.terminal_is_empty(){self.dependency=None;self.phase=2}},
   2|4=>{self.progress=Self::allocate(fields,if self.phase==2{window.len()}else{kind.len()})?;self.offset=0;self.phase+=1},
   3|5=>{let text=if self.phase==3{window}else{kind};if self.offset==text.len(){let value=unsafe{String::from_utf8_unchecked(std::mem::take(&mut fields.buffer))};if self.phase==3{fields.window_id=Some(value)}else{fields.window_kind=Some(value)}self.phase+=1;self.offset=0}else{let count=(text.len()-self.offset).min(grant.maximum_copy_bytes).min(64);fields.buffer.extend_from_slice(&text.as_bytes()[self.offset..self.offset+count]);self.offset+=count;self.progress.copied_bytes=count}},
   6=>{if fields.identity.complete(){fields.digits=fields.identity.take_text();self.phase=7}else{self.progress=fields.identity.step(grant)?}},
   7=>{let length=Self::version_length(fields);self.progress=Self::allocate(fields,length)?;self.offset=0;self.phase=8},
   8=>{if self.offset==Self::version_length(fields){fields.version=Some(unsafe{String::from_utf8_unchecked(std::mem::take(&mut fields.buffer))});self.phase=9}else{let digits=fields.digits.as_ref().unwrap().as_bytes();let count=(Self::version_length(fields)-self.offset).min(grant.maximum_copy_bytes).min(64);for index in self.offset..self.offset+count{let byte=if index<VERSION_PREFIX.len(){VERSION_PREFIX[index]}else if index<VERSION_PREFIX.len()+digits.len(){digits[index-VERSION_PREFIX.len()]}else{VERSION_SUFFIX[index-VERSION_PREFIX.len()-digits.len()]};fields.buffer.push(byte)}self.offset+=count;self.progress.copied_bytes=count}},
   9=>{let pending=fields.pending.take().unwrap();fields.origin=Some(FlowInvocationOrigin{window_id:fields.window_id.take().unwrap(),window_kind_id:fields.window_kind.take().unwrap(),neuron_id:pending.neuron_id,extension_id:pending.extension_id,operator_id:pending.operator_id,node_hash:pending.node_hash,input_json:pending.input_json,dependency_json:fields.dependency.take().unwrap(),operator_version:fields.version.take().unwrap(),outer_node_hash:None,preview_source:None});self.phase=10},
   10=>{let origin=fields.origin.take().unwrap();let turn=FlowInvocationOriginLease::admit(origin,grant);self.work=turn.work;match turn.result{Ok((lease,progress))=>{fields.lease=Some(lease);self.progress=progress;self.phase=11},Err((error,origin))=>{self.progress=error.retained_progress();fields.origin=Some(origin);return Err(error)}}},
   11=>{self.phase=12;return Ok(fields.lease.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation origin was transferred")),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:FlowInvocationOriginLease){assert_eq!(self.phase,12);self.fields.as_mut().unwrap().lease=Some(output);self.phase=11}
 #[cfg(feature="testkit")]
 #[doc(hidden)]
 pub fn test_diagnostic(&self)->u8{self.phase}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.dependency.as_mut(){child.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.dependency.as_ref(){let mut demand=child.next_close_demands(copy)?;demand.depth+=1;return Ok(demand)}if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.progress=Default::default();self.work=0;if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation origin close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(child)=self.dependency.as_mut(){if grant.maximum_depth<2{return Ok(RetainedCloneStep::Progress(Default::default()))}let result=child.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();self.work=child.last_turn_work();let step=result?;if child.terminal_is_empty(){self.dependency=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();self.work=owner.last_turn_work();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("invocation origin fields support controlled retirement")));self.work=1;self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.dependency.is_none()&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowInvocationOriginWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original invocation origin requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

