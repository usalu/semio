//! 🌳️ Bounded ancestry and canonical dependency JSON over the job's exact borrowed scene.
use super::*;
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowSourceIdentity};
use semio_framework_pack_json::{JsonBorrowedWriteCursor,JsonWriteSource};
use std::mem::{ManuallyDrop,size_of};
#[path="🪞️view/🦀️.rs"]
mod view;
use view::FlowDependencyView;
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct DependencyFields{ancestors:Vec<u8>,widgets:Vec<usize>,synapses:Vec<usize>,writer:JsonBorrowedWriteCursor,output:Option<String>}
pub struct FlowInferenceDependencyWork{fields:ManuallyDrop<Option<DependencyFields>>,retirement:Option<ControlledRetirement<DependencyFields>>,binding:Option<FlowSourceIdentity>,target:Option<(usize,usize)>,operation:Option<semio_framework_job::OriginalSourceBinding>,phase:u8,visit:usize,scan:usize,member:usize,eligible:bool,changed:bool,candidate:Option<usize>,closing:bool,progress:RetainedCloneProgress,work:u64}
impl FlowInferenceDependencyWork{
 pub fn new()->Self{Self{fields:ManuallyDrop::new(Some(DependencyFields{ancestors:Vec::new(),widgets:Vec::new(),synapses:Vec::new(),writer:JsonBorrowedWriteCursor::new(),output:None})),retirement:None,binding:None,target:None,operation:None,phase:0,visit:0,scan:0,member:0,eligible:false,changed:false,candidate:None,closing:false,progress:Default::default(),work:0}}
 fn check(&self,source:&dyn FlowSnapshotSource,target:&str)->Result<(),ValueError>{if self.binding.is_some_and(|binding|binding!=source.source_identity())||self.target.is_some_and(|binding|binding!=(target.as_ptr()as usize,target.len())){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inference dependency source changed"))}Ok(())}
 fn bytes(length:usize,item:usize)->Result<usize,ValueError>{length.checked_mul(item).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original dependency source extent overflows"))}
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,target:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.check(source,target)?;let fields=self.fields.as_ref().unwrap();
  if self.phase==7{let view=FlowDependencyView{source,widgets:&fields.widgets,synapses:&fields.synapses};let mut demand=fields.writer.normal_step_demands(&view)?;demand.depth+=1;return Ok(demand)}
  let mut demand=RetirementDemand{depth:1,..Default::default()};
  match self.phase{0=>demand.capacity_bytes=source.synapses().len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original dependency ancestry extent overflows"))?,1 if fields.ancestors.len()<=source.synapses().len()=>demand.copy_bytes=1,2 if self.visit<fields.ancestors.len()&&fields.ancestors[self.visit]!=0&&self.scan<source.synapses().len()=>demand.copy_bytes=1,3=>demand.capacity_bytes=Self::bytes(source.widgets().len(),size_of::<usize>())?,4=>demand.capacity_bytes=Self::bytes(source.synapses().len(),size_of::<usize>())?,5 if self.scan==source.widgets().len()&&self.candidate.is_some()=>demand.copy_bytes=size_of::<usize>(),6 if self.scan==source.synapses().len()&&self.candidate.is_some()=>demand.copy_bytes=size_of::<usize>(),_=>{}}Ok(demand)
 }
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,target:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<String>,semio_framework_job::OriginalTurnFailure>{
  let operation=semio_framework_job::OriginalSourceBinding::borrow(cx);if self.operation.is_some_and(|original|!original.matches(cx)){return Err(semio_framework_job::OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation dependency operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let mut total=RetainedCloneProgress::default();let mut total_work=0;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){self.progress=total;self.work=total_work;return Ok(None)}
   let result=self.advance(source,target,cx.retained_grant());let progress=self.progress;let work=self.work;total_work+=work;
   if work!=0||progress!=Default::default(){self.operation=Some(operation)}
   if work!=0{cx.consume_fuel(work)}
   let sum=match total.checked_add(progress){Ok(sum)=>sum,Err(error)=>{if let Ok(Some(output))=result{self.restore_output(output)}self.progress=total;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))}};
   match result{
    Ok(output)=>{if let Err(error)=cx.consume_retained(progress){if let Some(output)=output{self.restore_output(output)}self.progress=sum;self.work=total_work;return Err(error.into())}total=sum;if let Some(output)=output{self.progress=total;self.work=total_work;return Ok(Some(output))}if progress==Default::default(){self.progress=total;self.work=total_work;return Ok(None)}},
    Err(error)=>{self.progress=sum;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))},
   }
  }self.progress=total;self.work=total_work;Ok(None)
 }

 fn ancestor<'a>(source:&'a dyn FlowSnapshotSource,target:&'a str,index:usize)->&'a str{if index==0{target}else{&source.synapses()[index-1].from}}
 pub fn advance(&mut self,source:&dyn FlowSnapshotSource,target:&str,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
  self.progress=Default::default();self.work=0;if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original inference dependency is closing"))}let demand=self.next_demands(source,target,grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.binding=Some(source.source_identity());self.target=Some((target.as_ptr()as usize,target.len()));self.work=1;self.progress.copied_items=1;let fields=self.fields.as_mut().unwrap();
  match self.phase{
   0=>{fields.ancestors.try_reserve_exact(source.synapses().len()+1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original ancestry allocation refused"))?;self.progress.retained_capacity_bytes=fields.ancestors.capacity();self.phase=1},
   1=>{if fields.ancestors.len()<=source.synapses().len(){fields.ancestors.push(u8::from(fields.ancestors.is_empty()));self.progress.copied_bytes=1}else{self.phase=2}},
   2=>{
    if self.visit==fields.ancestors.len(){if self.changed{self.changed=false;self.visit=0}else{self.phase=3}self.scan=0}
    else if fields.ancestors[self.visit]==0{self.visit+=1;self.scan=0}
    else if self.scan==source.synapses().len(){self.visit+=1;self.scan=0}
    else{let edge=&source.synapses()[self.scan];if edge.to==Self::ancestor(source,target,self.visit)&&fields.ancestors[self.scan+1]==0{fields.ancestors[self.scan+1]=1;self.changed=true;self.progress.copied_bytes=1}self.scan+=1}
   },
   3=>{fields.widgets.try_reserve_exact(source.widgets().len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original dependency widget ordinal allocation refused"))?;self.progress.retained_capacity_bytes=fields.widgets.capacity()*size_of::<usize>();self.phase=4},
   4=>{fields.synapses.try_reserve_exact(source.synapses().len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original dependency wire ordinal allocation refused"))?;self.progress.retained_capacity_bytes=fields.synapses.capacity()*size_of::<usize>();self.phase=5;self.scan=0;self.member=0},
   5=>{
    if self.scan==source.widgets().len(){if let Some(candidate)=self.candidate.take(){fields.widgets.push(candidate);self.progress.copied_bytes=size_of::<usize>();self.scan=0;self.member=0;self.eligible=false}else{self.phase=6;self.scan=0}}
    else{let id=widget_id_for(&source.widgets()[self.scan]);
     if fields.widgets.last().is_some_and(|index|id<=widget_id_for(&source.widgets()[*index])){self.scan+=1;self.member=0;self.eligible=false}
     else if !self.eligible&&self.member<fields.ancestors.len(){self.eligible=fields.ancestors[self.member]!=0&&id==Self::ancestor(source,target,self.member);self.member+=1}
     else{if self.eligible&&self.candidate.is_none_or(|index|id<=widget_id_for(&source.widgets()[index])){self.candidate=Some(self.scan)}self.scan+=1;self.member=0;self.eligible=false}
    }
   },
   6=>{
    if self.scan==source.synapses().len(){if let Some(candidate)=self.candidate.take(){fields.synapses.push(candidate);self.progress.copied_bytes=size_of::<usize>();self.scan=0}else{self.phase=7}}
    else{let id=&source.synapses()[self.scan].id;if fields.ancestors[self.scan+1]!=0&&fields.synapses.last().is_none_or(|index|id>&source.synapses()[*index].id)&&self.candidate.is_none_or(|index|id<=&source.synapses()[index].id){self.candidate=Some(self.scan)}self.scan+=1}
   },
   7=>{let view=FlowDependencyView{source,widgets:&fields.widgets,synapses:&fields.synapses};let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(isize::MAX as usize,&mut accept);let result=fields.writer.step(&view,1,&mut control,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=fields.writer.normal_step_progress();self.work=self.progress.copied_items as u64;if let Some(output)=result?{fields.output=Some(output);self.phase=8}},
   8=>{self.phase=9;return Ok(fields.output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original dependency output was transferred")),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:String){assert_eq!(self.phase,9);self.fields.as_mut().unwrap().output=Some(output);self.phase=8}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.progress=Default::default();self.work=0;if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original dependency close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();self.work=owner.last_turn_work();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("dependency fields support controlled retirement")));self.work=1;self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowInferenceDependencyWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original inference dependency requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}


#[cfg(any(test,feature="testkit"))]
impl FlowInferenceDependencyWork{
 /// 🔬️ Exposes the finite original ancestry frontier without lending mutable cursor custody.
 pub fn test_diagnostic(&self)->(u8,usize,usize,usize,Option<usize>){(self.phase,self.visit,self.scan,self.member,self.candidate)}
}
