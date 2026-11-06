//! 🌱️ Completion birth retains the actual native Arc between original allocation and paid header handoff.
use super::{ArtifactApp,ArtifactToolCompletion,Cell,CompletionCell,ManuallyDrop,Arc,Mutex};
use semio_framework_job::{StepContext,OriginalTurnFailure,InteractiveJobCloseStep};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::mem::size_of;

type Scope=(semio_framework::action_bus::OriginalToolSourceBinding,(u64,u64));
pub struct OriginalCompletionBirth<A:ArtifactApp>{source:Option<Scope>,input:Option<Cell<A>>,born:ManuallyDrop<Option<Arc<Cell<A>>>>,allocation:semio_framework_value::native_decoding::NativeSharedAdmission<Cell<A>>,complete:bool}
impl<A:ArtifactApp> Default for OriginalCompletionBirth<A>{fn default()->Self{Self{source:None,input:Some(Cell{value:Mutex::new(None),authoring:Mutex::new(None)}),born:ManuallyDrop::new(None),allocation:Default::default(),complete:false}}}
impl<A:ArtifactApp> OriginalCompletionBirth<A>{
 pub fn output_ready(&self)->bool{self.allocation.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.born.is_some()}
 pub fn shared_admission(&mut self)->&mut semio_framework_value::native_decoding::NativeSharedAdmission<Cell<A>>{&mut self.allocation}
 pub fn advance(&mut self,output:&mut Option<ArtifactToolCompletion<A>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some((source,decoder))=self.source{if !source.matches(cx)||cx.native_receiving_identity()!=Some(decoder){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth refused another original caller or decoder").into())}}
  if !self.allocation.can_advance(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth allocation callback requires resolution").into())}
  if self.complete{if output.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth lost its original handoff").into())}return Ok(true)}
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth receiver is occupied").into())}
  cx.original_source_parent()?;let grant=cx.retained_grant();
  if self.source.is_none(){let bytes=2*size_of::<Option<Scope>>();let binding=semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx);semio_framework_job::admit_original_construction(cx,&mut self.source,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|control|{let result=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"completion birth requires its original decoder recipient")).map(|recipient|((binding,(control.ledger_identity(),recipient)),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));RetainedWorkOutcome{result,work:1}})?;return Ok(false)}
  if self.born.is_none(){
   let mut work=0;let result=cx.with_native_receiving::<_,semio_framework_value_native_owned::OriginalReceivingPrimary>(|native|{let turn=native.receive_arc(&mut self.input,&mut self.born,&mut self.allocation);work=turn.work;turn.result});
   let progress=result.as_ref().copied().unwrap_or_else(|error|error.retained_progress());let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);
   return match result{Ok(_)=>{receipt?;Ok(false)},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
  }
  let bytes=size_of::<ArtifactToolCompletion<A>>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
  *output=Some(ArtifactToolCompletion{inner:CompletionCell{inner:ManuallyDrop::new(self.born.take().unwrap()),returned:false}});self.complete=true;
  cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(true)
 }
 pub fn close_demands(&self)->RetirementDemand{if self.allocation.has_birth(){return self.allocation.close_demand().unwrap_or_default()}RetirementDemand{release_bytes:if self.born.is_some(){semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<Cell<A>>()}else{0},depth:usize::from(self.born.is_some()||self.input.is_some()),..Default::default()}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.source.is_some_and(|(source,decoder)|!source.matches(cx)||cx.native_receiving_identity()!=Some(decoder)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth close refused another original caller or decoder").into())}
  if self.allocation.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth allocation callback requires resolution").into())}
  if self.allocation.has_birth(){let demand=self.allocation.close_demand()?;return semio_framework_job::close_original_retained_step(cx,demand,|grant|self.allocation.close_step(grant));}
  let demand=self.close_demands();semio_framework_job::close_original_retained_step(cx,demand,|_|{let released_bytes=if let Some(root)=self.born.take(){assert_eq!(Arc::strong_count(&root),1);drop(root);demand.release_bytes}else{0};self.input=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})),work:1}})
 }
 pub fn terminal_is_empty(&self)->bool{self.allocation.terminal_is_empty()&&self.input.is_none()&&self.born.is_none()}
}
impl<A:ArtifactApp> Drop for OriginalCompletionBirth<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.born.is_none(),"born completion Arc requires original funded handoff or close");if self.born.is_none(){unsafe{ManuallyDrop::drop(&mut self.born)}}}}
