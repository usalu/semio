//! 🌱️ Completion birth retains the actual native Arc between original allocation and paid header handoff.
use super::{ArtifactApp,ArtifactToolCompletion,Cell,CompletionCell,ManuallyDrop,Arc,Mutex};
use semio_framework_job::{StepContext,OriginalTurnFailure,InteractiveJobCloseStep,OriginalConstructionAdmission};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::mem::size_of;

type Scope=(semio_framework::action_bus::OriginalToolSourceBinding,(semio_framework_value::retained_clone::NativeAllocationIdentity,u64));
pub struct OriginalCompletionBirth<A:ArtifactApp>{source:Option<Scope>,input:Option<Cell<A>>,born:ManuallyDrop<Option<Arc<Cell<A>>>>,allocation:semio_framework_value::native_decoding::NativeSharedAdmission<Cell<A>>,construction:OriginalConstructionAdmission,constructed:bool,complete:bool}
impl<A:ArtifactApp> Default for OriginalCompletionBirth<A>{fn default()->Self{Self{source:None,input:Some(Cell{value:Mutex::new(None),authoring:Mutex::new(None)}),born:ManuallyDrop::new(None),allocation:Default::default(),construction:Default::default(),constructed:false,complete:false}}}
impl<A:ArtifactApp> OriginalCompletionBirth<A>{
 pub fn output_ready(&self)->bool{self.allocation.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.born.is_some()}
 pub fn shared_admission(&mut self)->&mut semio_framework_value::native_decoding::NativeSharedAdmission<Cell<A>>{&mut self.allocation}
 pub fn advance(&mut self,output:&mut Option<ArtifactToolCompletion<A>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some((source,decoder))=self.source{if !source.matches(cx)||cx.native_receiving_identity()!=Some(decoder){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth refused another original caller or decoder").into())}}
  if !self.allocation.can_advance(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth allocation callback requires resolution").into())}
  if self.complete{if output.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth lost its original handoff").into())}return Ok(true)}
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth receiver is occupied").into())}
  cx.original_source_parent()?;let grant=cx.retained_grant();
  if !self.constructed{let bytes=2*size_of::<Option<Scope>>();let binding=semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx);let ready=semio_framework_job::admit_original_construction_settled(cx,&mut self.construction,&mut self.source,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},1,|control,_|{let result=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"completion birth requires its original decoder recipient")).map(|recipient|((binding,(control.ledger_identity(),recipient)),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));RetainedWorkOutcome{result,work:1}})?;if ready{self.constructed=true;}return Ok(false)}
  if self.born.is_none(){
   let mut scope=None;let mut stepped=None;
   if !cx.with_native_receiving(&mut scope,|native,_|{stepped=Some(native.receive_arc(&mut self.input,&mut self.born,&mut self.allocation));Ok(())})?{return Ok(false)}
   let invalid=|message:&'static str|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
   let scope=scope.ok_or_else(||invalid("completion birth receiving scope vanished"))?;
   if scope.source_refusal.is_some()||scope.call.is_err(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"completion birth receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
   let turn=stepped.ok_or_else(||invalid("completion birth receiving step vanished"))?;
   semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{work:turn.work,result:turn.result.map(RetainedCloneStep::Progress).map_err(semio_framework_job::OriginalTurnPrimary::Receiving)})?;return Ok(false)
  }
  let bytes=size_of::<ArtifactToolCompletion<A>>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
  *output=Some(ArtifactToolCompletion{inner:CompletionCell{inner:ManuallyDrop::new(self.born.take().unwrap()),returned:false}});self.complete=true;
  cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(true)
 }
 pub fn close_demands(&self)->RetirementDemand{if self.allocation.has_birth(){return self.allocation.close_demand().unwrap_or_default()}RetirementDemand{release_bytes:if self.born.is_some(){semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<Cell<A>>()}else{0},depth:usize::from(self.born.is_some()||self.input.is_some()),..Default::default()}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.source.is_some_and(|(source,decoder)|!source.matches(cx)||cx.native_receiving_identity()!=Some(decoder)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth close refused another original caller or decoder").into())}
  if self.allocation.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"completion birth allocation callback requires resolution").into())}
  if self.owned_is_empty(){let settled=self.construction.terminal_is_empty()||{self.construction.close_step(cx)?;self.construction.terminal_is_empty()};return Ok(if settled{InteractiveJobCloseStep::Complete{progress:Default::default()}}else{InteractiveJobCloseStep::Pending{progress:Default::default()}})}
  if self.allocation.has_birth(){let demand=self.allocation.close_demand()?;return semio_framework_job::close_original_effect_step(cx,demand,|effect|self.allocation.close_step(effect));}
  let demand=self.close_demands();semio_framework_job::close_original_retained_effect_step(cx,demand,|effect|{if !effect.performed(1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}let released_bytes=if let Some(root)=self.born.take(){assert_eq!(Arc::strong_count(&root),1);drop(root);demand.release_bytes}else{0};self.input=None;Ok::<_,ValueError>(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}))})
 }
 fn owned_is_empty(&self)->bool{self.allocation.terminal_is_empty()&&self.input.is_none()&&self.born.is_none()}
 pub fn terminal_is_empty(&self)->bool{self.owned_is_empty()&&self.construction.terminal_is_empty()}
}
impl<A:ArtifactApp> Drop for OriginalCompletionBirth<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.born.is_none(),"born completion Arc requires original funded handoff or close");if self.born.is_none(){unsafe{ManuallyDrop::drop(&mut self.born)}}}}
