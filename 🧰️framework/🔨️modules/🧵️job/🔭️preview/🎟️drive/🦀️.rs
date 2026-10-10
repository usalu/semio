//! 🔭️ Original caller turns drive one native preview through borrowed results and paid physical closure.
use super::*;
/// 🕰️ Caller-authored limits stay fixed across every preview transition.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JobPreviewLimits{pub maximum_turns:u64,pub maximum_wall_us:u64}
/// 🚦️ A terminal result is available only after the original session is physically empty.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum JobPreviewStatus{Complete,Cancelled,Fault,Budget}
/// 📊️ Counts actual published semantic outcomes independently of preparation turns.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JobPreviewVerdict{pub status:JobPreviewStatus,pub outcomes:u64}
/// 🤝️ A Fault remains borrowed from the same native producer until its consumer accepts the input.
pub enum JobPreviewStep<'a>{Pending,Fault{detail:&'a RetainedJobPayload},Complete(JobPreviewVerdict)}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum JobPreviewPhase{Run,Read,FaultInput,Acknowledge,Resume,Close,Complete}
/// 🧭️ Holds only metadata while the caller retains the original Batch session and authority.
pub struct JobPreviewDriver{
 limits:JobPreviewLimits,binding:Option<(usize,OperationId,Generation,u64)>,phase:JobPreviewPhase,
 turns:u64,outcomes:u64,terminal:bool,status:JobPreviewStatus,
 receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
}
impl JobPreviewDriver{
 pub fn new(limits:JobPreviewLimits)->Self{Self{limits,binding:None,phase:JobPreviewPhase::Run,turns:0,outcomes:0,terminal:false,status:JobPreviewStatus::Complete,receipt:None}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"preview requires its original session, identity and caller receipt")}
 fn metadata(cx:&mut StepContext<'_>)->Result<bool,ValueError>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 fn matches<J:InteractiveJob+'static>(&self,session:&BatchJobSession<J>,cx:&StepContext<'_>)->bool{self.binding.is_some_and(|(pointer,operation,generation,_)|pointer==session as*const _ as usize&&operation==cx.operation()&&generation==cx.generation())}
 fn verdict(&self)->JobPreviewVerdict{JobPreviewVerdict{status:self.status,outcomes:self.outcomes}}
 fn close_intent<J:InteractiveJob+'static>(&mut self,session:&mut BatchJobSession<J>){session.begin_close();self.phase=JobPreviewPhase::Close;}
 /// 📬️ Consumes the original raw callback receipt once before any semantic loan or acknowledgement.
 fn receive(&mut self,cx:&mut StepContext<'_>,original:RetainedCloneGrant)->Result<(),ValueError>{
  let(issued,progress)=self.receipt.take().ok_or_else(Self::invalid)?;
  let fits=issued.maximum_items<=original.maximum_items&&issued.maximum_copy_bytes<=original.maximum_copy_bytes&&issued.maximum_capacity_bytes<=original.maximum_capacity_bytes&&issued.maximum_release_bytes<=original.maximum_release_bytes&&issued.maximum_depth<=original.maximum_depth&&progress.fits(issued);
  cx.consume_retained(progress)?;
  if !fits{return Err(Self::invalid().with_retained_progress(progress))}Ok(())
 }
 /// ▶️ Advances one funded transition without constructing another context, root or authority.
 pub fn advance<'session,J:InteractiveJob+'static>(&mut self,session:&'session mut BatchJobSession<J>,cx:&mut StepContext<'_>)->Result<JobPreviewStep<'session>,ValueError>{
  if let Some((pointer,operation,generation,_))=self.binding{if pointer!=session as*const _ as usize||operation!=cx.operation()||generation!=cx.generation(){return Err(Self::invalid())}}
  if self.phase==JobPreviewPhase::Complete{return Ok(JobPreviewStep::Complete(self.verdict()))}
  if self.receipt.is_some(){return Err(Self::invalid())}
  if cx.should_yield(){return Ok(JobPreviewStep::Pending)}
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobPreviewStep::Pending)}
  if self.binding.is_none(){
   let identity=match session.original_identity(){Ok(Some(identity))=>identity,Ok(None)|Err(_)=>return Err(Self::invalid())};
   if identity!=(cx.operation(),cx.generation())||self.limits.maximum_turns==0||self.limits.maximum_wall_us==0{return Err(Self::invalid())}
   let deadline=cx.now_us().and_then(|now|now.checked_add(self.limits.maximum_wall_us)).ok_or_else(Self::invalid)?;
   if Self::metadata(cx)?{self.binding=Some((session as*const _ as usize,identity.0,identity.1,deadline));}
   return Ok(JobPreviewStep::Pending);
  }
  match self.phase{
   JobPreviewPhase::Run=>{
    let deadline=self.binding.expect("same original preview binding").3;
    if cx.is_cancelled()||self.turns>=self.limits.maximum_turns||cx.now_us().is_none_or(|now|now>=deadline){
     if Self::metadata(cx)?{self.status=if cx.is_cancelled(){JobPreviewStatus::Cancelled}else{JobPreviewStatus::Budget};self.close_intent(session);}
     return Ok(JobPreviewStep::Pending);
    }
    let poll=match session.step(grant){Ok(poll)=>poll,Err(_)=>return Ok(JobPreviewStep::Pending)};
    if matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal){
     if !session.checkout_outcome(){return Err(Self::invalid())}
     self.receipt=session.take_checked_out_retained_step_receipt();self.receive(cx,grant)?;
     self.turns=self.turns.checked_add(1).ok_or_else(Self::invalid)?;self.phase=JobPreviewPhase::Read;
    }
    Ok(JobPreviewStep::Pending)
   },
   JobPreviewPhase::Read=>{
    let outcome=session.checked_out_outcome()?;
    if !Self::metadata(cx)?{return Ok(JobPreviewStep::Pending)}
    let Some(outcome)=outcome else{self.phase=JobPreviewPhase::Resume;return Ok(JobPreviewStep::Pending)};
    self.outcomes=self.outcomes.checked_add(1).ok_or_else(Self::invalid)?;self.terminal=outcome.is_terminal();
    match outcome{
     JobOutcomeView::Fault{detail,..}=>{self.status=JobPreviewStatus::Fault;self.phase=JobPreviewPhase::FaultInput;Ok(JobPreviewStep::Fault{detail})},
     JobOutcomeView::Cancelled{..}=>{self.status=JobPreviewStatus::Cancelled;self.phase=JobPreviewPhase::Acknowledge;Ok(JobPreviewStep::Pending)},
     _=>{self.phase=JobPreviewPhase::Acknowledge;Ok(JobPreviewStep::Pending)}
    }
   },
   JobPreviewPhase::FaultInput=>match session.checked_out_outcome()?{Some(JobOutcomeView::Fault{detail,..})=>Ok(JobPreviewStep::Fault{detail}),_=>Err(Self::invalid())},
   JobPreviewPhase::Acknowledge=>{
    let step=session.acknowledge_outcome(grant);cx.consume_retained(step.progress())?;
    if matches!(step,RetainedCloneStep::Complete(_)){self.phase=JobPreviewPhase::Resume;}
    Ok(JobPreviewStep::Pending)
   },
   JobPreviewPhase::Resume=>{
    if !Self::metadata(cx)?{return Ok(JobPreviewStep::Pending)}
    if self.terminal{self.close_intent(session);}else if session.resume().is_ok(){self.phase=JobPreviewPhase::Run;}
    Ok(JobPreviewStep::Pending)
   },
   JobPreviewPhase::Close=>{
    if session.terminal_is_empty(){if Self::metadata(cx)?{self.phase=JobPreviewPhase::Complete;self.binding=None;return Ok(JobPreviewStep::Complete(self.verdict()))}return Ok(JobPreviewStep::Pending)}
    if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap_or(false){
     match session.return_original_cancel_alias_step(cx.original_cancel_token(),grant){
      Ok(Some(step))=>{cx.consume_retained(step.progress())?;return Ok(JobPreviewStep::Pending)},
      Ok(None)|Err(WorkerJobDemandError::Contention(_))=>return Ok(JobPreviewStep::Pending),
      Err(WorkerJobDemandError::Refused(error))=>{cx.consume_retained(error.retained_progress())?;return Err(error)}
     }
    }
    let step=session.close_step(grant);cx.consume_retained(step.progress())?;
    match step{WorkerJobCloseStep::Refused{kind,progress}=>Err(ValueError::literal(kind,"original preview session physical closure refused").with_retained_progress(progress)),WorkerJobCloseStep::Complete{..}if !session.terminal_is_empty()=>Err(Self::invalid()),_=>Ok(JobPreviewStep::Pending)}
   },
   JobPreviewPhase::Complete=>Ok(JobPreviewStep::Complete(self.verdict())),
  }
 }
 /// ✅️ The caller signals that its original borrowed Fault input has been retained before ACK.
 pub fn accept_fault_input<J:InteractiveJob+'static>(&mut self,session:&BatchJobSession<J>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  if !self.matches(session,cx)||self.phase!=JobPreviewPhase::FaultInput{return Err(Self::invalid())}
  if cx.should_yield()||!Self::metadata(cx)?{return Ok(false)}self.phase=JobPreviewPhase::Acknowledge;Ok(true)
 }
 /// 🛑️ Cancellation changes intent and retains every original producer owner for paid closure.
 pub fn cancel<J:InteractiveJob+'static>(&mut self,session:&mut BatchJobSession<J>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  if !self.matches(session,cx){return Err(Self::invalid())}
  if cx.should_yield()||!Self::metadata(cx)?{return Ok(false)}self.status=JobPreviewStatus::Cancelled;self.close_intent(session);Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.phase==JobPreviewPhase::Complete&&self.binding.is_none()&&self.receipt.is_none()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

