//! 🏗️ The caller's same initializer writes directly into its paid final native authority before a separate semantic handoff.
use super::*;
pub struct WorkerJobSessionPreparation<J:InteractiveJob+'static>{session:ManuallyDrop<Option<WorkerJobSession<J>>>,source_identity:(usize,usize,OperationId,Generation),initialized:bool,closing:bool,close_started:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native job preparation requires the same original initializer and parameter owners")}
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl<J:InteractiveJob+'static> WorkerJobSessionPreparation<J>{
 pub fn try_begin(initializer:&mut impl WorkerJobInitializer<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let original=params.as_ref().ok_or_else(invalid)?;let identity=(original.operation,original.generation);if control.admission_identity()!=identity||!control.admission_cancel().is_none_or(|token|token.is_same_node(&original.cancel)){return Err(invalid())}let grant=control.admission_grant()?;let demand=WorkerJobSession::<J>::admission_demand();if !control.admission_is_open()||original.cancel.is_cancelled_now()||!funds(grant,demand){return Ok(None)}
  let source_identity=(initializer as*mut _ as usize,params as*const _ as usize,identity.0,identity.1);let Some(session)=WorkerJobSession::<J>::birth_storage(identity.0,identity.1,control)?else{return Ok(None)};let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()};let owner=Self{session:ManuallyDrop::new(Some(session)),source_identity,initialized:false,closing:false,close_started:false};control.receive_admission(progress)?;Ok(Some((owner,progress)))
 }
 fn check(&self,initializer:&impl WorkerJobInitializer<J>)->Result<(),ValueError>{if initializer as*const _ as usize!=self.source_identity.0{return Err(invalid())}Ok(())}
 fn authority(&self)->&OriginalWorkerJobAuthority<J>{unsafe{(&*self.session.as_ref().expect("native preparation owns its session").inner.authority.get()).as_ref().expect("native preparation owns final authority")}}
 fn authority_mut(&mut self)->&mut OriginalWorkerJobAuthority<J>{unsafe{(&mut*self.session.as_ref().expect("native preparation owns its session").inner.authority.get()).as_mut().expect("native preparation owns final authority")}}
 pub fn original_failure(&self)->Option<&OriginalTurnFailure>{self.session.as_ref().and_then(|_|self.authority().original_initialization_failure())}
 pub fn advance(&mut self,initializer:&mut impl WorkerJobInitializer<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<(Option<WorkerJobSession<J>>,RetainedCloneProgress),ValueError>{
  self.check(initializer)?;if self.session.is_none(){return Err(invalid())}if self.closing{return Ok((None,Default::default()))}let params_identity=params as*const _ as usize;let original=params.as_ref().ok_or_else(invalid)?;if params_identity!=self.source_identity.1||(original.operation,original.generation)!=(self.source_identity.2,self.source_identity.3)||control.admission_identity()!=(original.operation,original.generation){return Err(invalid())}let grant=control.admission_grant()?;if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,Default::default()))}
  if !self.authority().is_initialized(){let progress=self.authority_mut().advance_initialization(grant);control.receive_admission(progress)?;return Ok((None,progress))}
  if self.initialized{if !initializer.initialization_input_is_empty(){return Err(invalid())}unsafe{self.authority_mut().publish_initialized_source_prepaid(params)}let progress=RetainedCloneProgress{copied_items:1,..Default::default()};control.receive_admission(progress)?;return Ok((self.session.take(),progress))}
  match self.authority_mut().initialize_source_step(initializer,control){Ok(step)=>{self.initialized=step.complete;Ok((None,step.progress))},Err(error)=>{self.initialized=self.authority().source_is_initialized();self.closing=true;Err(error)}}
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demands(&self,initializer:&impl WorkerJobInitializer<J>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  self.check(initializer)?;if self.session.is_none(){return Ok(Default::default())}if !self.initialized&&!initializer.partial_terminal_is_empty(){return initializer.partial_retirement_demands(maximum_copy_bytes)}if !self.close_started||self.session.as_ref().unwrap().terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}self.session.as_ref().unwrap().retirement_demands(maximum_copy_bytes).map_err(|error|match error{WorkerJobDemandError::Refused(error)=>error,WorkerJobDemandError::Contention(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native preparation close retains its exclusive session")})
 }
 pub fn close_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  self.check(initializer)?;if self.session.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !self.closing{return Err(invalid().into())}let grant=cx.retained_grant();let demand=self.close_demands(initializer,grant.maximum_copy_bytes)?;if cx.retirement_should_yield()||!funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.initialized&&!initializer.partial_terminal_is_empty(){let step=self.authority_mut().close_partial_source_step(initializer,cx)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.close_started{let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);if self.initialized{self.authority_mut().begin_initialized_source_close()}else{self.authority_mut().finish_partial_source(initializer)?}self.session.as_ref().unwrap().begin_close();self.close_started=true;return Ok(RetainedCloneStep::Progress(progress))}
  if self.session.as_ref().unwrap().terminal_is_empty(){let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);drop(self.session.take());return Ok(RetainedCloneStep::Complete(progress))}
  match self.session.as_mut().unwrap().close_step(cx)?{WorkerJobCloseStep::Pending{progress}|WorkerJobCloseStep::Complete{progress}=>Ok(RetainedCloneStep::Progress(progress)),WorkerJobCloseStep::Blocked=>Ok(RetainedCloneStep::Progress(Default::default()))}
 }

 pub fn terminal_is_empty(&self)->bool{self.session.is_none()}
}
impl<J:InteractiveJob+'static> Drop for WorkerJobSessionPreparation<J>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native preparation requires original partial initializer closure before abandonment");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.session);}}}}


