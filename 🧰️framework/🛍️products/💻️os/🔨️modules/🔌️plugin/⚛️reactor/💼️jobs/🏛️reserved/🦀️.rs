//! 🏛️ The framework-reserved job kind of the job table: its bounded job, admission factory and the registry driver the instance runs it through.
use semio_framework_actor::OriginalJobStep as JobStep;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_job::reserved_jobs::FRAMEWORK_RESERVED_JOB_MAGIC;

/// 🧪️ Drives one framework-reserved spawn-job through the real reactor registry: a genuine original admission under a
/// caller context funded with at least the registry's own quoted grant, then bounded steps until its terminal output.
#[doc(hidden)]
pub fn drive_framework_reserved_registry_job(job: u64, input: Vec<u8>, policy: RetainedCloneGrant) -> Result<Vec<u8>,OriginalTurnFailure> {
    drive_framework_reserved_registry_job_counted(job, input, policy).map(|(output, _)| output)
}

/// 🧪️ The same registry drive, also reporting how many funded `step_job` turns the job needed.
#[doc(hidden)]
pub fn drive_framework_reserved_registry_job_counted(job: u64, input: Vec<u8>, policy: RetainedCloneGrant) -> Result<(Vec<u8>,u32),OriginalTurnFailure> {
    use crate::{start_job, step_job, IoRunControl, JobStep, OriginalJobAdmission, SqliteSnapshotControl, WORK_UNITS_EXECUTE};
    use semio_framework_job::{Generation, OperationId, StepBudget, StepContext};
    use semio_framework_async::poll::resolve_ready;
    const TURNS: usize = 100_000;
    let stall = |detail: &'static str| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-stall"), detail);
    let (operation, generation) = (OperationId(job), Generation(1));
    let widest = |left: RetainedCloneGrant, right: RetainedCloneGrant| RetainedCloneGrant { maximum_items: left.maximum_items.max(right.maximum_items), maximum_copy_bytes: left.maximum_copy_bytes.max(right.maximum_copy_bytes), maximum_capacity_bytes: left.maximum_capacity_bytes.max(right.maximum_capacity_bytes), maximum_release_bytes: left.maximum_release_bytes.max(right.maximum_release_bytes), maximum_depth: left.maximum_depth.max(right.maximum_depth) };
    let mut grant = policy;
    let mut source = Some(OriginalJobAdmission { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: Some(input), checkpoint: None });
    let mut admitted = false;
    for _ in 0..TURNS {
        let mut sequence = 0;
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, generation, StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
        match resolve_ready(start_job(&mut source, &mut cx)) {
            Ok(true) => {
                admitted = true;
                break;
            }
            Ok(false) => {}
            Err(error) => return Err(error.into()),
        }
        grant = widest(grant, crate::job_admission_demands(job, &cx).unwrap_or(grant));
    }
    if !admitted {
        return Err(stall("framework reserved spawn-job was never admitted by the reactor registry").into());
    }
    let mut steps = 0u32;
    for _ in 0..TURNS {
        if let Some(quote) = {
            let mut sequence = 0;
            let mut receipt = RetainedCloneProgress::default();
            let cx = StepContext::new(operation, generation, StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
            crate::job_admission_demands(job, &cx).ok()
        } {
            grant = widest(grant, quote);
        }
        let mut sequence = 0;
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, generation, StepBudget::new(WORK_UNITS_EXECUTE, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
        let mut decode_callback = |_| true;
        let mut encode_callback = |_| true;
        let mut decode = semio_framework_value::NativeDecodeControl::new(grant.maximum_capacity_bytes, &mut decode_callback);
        let mut encode = semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes, &mut encode_callback);
        let mut original = IoRunControl::new(&mut decode, &mut encode, grant);
        let mut snapshot_callback = |_| true;
        let mut snapshot = SqliteSnapshotControl::new(&mut snapshot_callback, Default::default());
        steps += 1;
        match resolve_ready(step_job(job, &mut original, &mut snapshot, &mut cx)) {
            Ok(JobStep::Running(_)) => {}
            Ok(JobStep::Done(bytes)) => return Ok((bytes, steps)),
            Ok(JobStep::Failed(bytes)) => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-failed"), String::from_utf8_lossy(&bytes).into_owned()).into()),
            Err(error)=>return Err(error),
        }
    }
    Err(stall("framework reserved spawn-job did not reach a terminal step").into())
}

pub(crate) fn framework_reserved_job_demands(_job:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_cx:&semio_framework_job::StepContext<'_>)->Result<RetainedCloneGrant,ValueError>{crate::original_job_admission_demands::<FrameworkReservedBoundedJob>(input,restored)}

pub(crate) fn framework_reserved_job_factory(_job:u64,input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Box<dyn crate::BoundedJob>>,ValueError>{
    cx.original_source_parent()?;let binding=semio_framework_job::OriginalSourceBinding::borrow(cx);
    crate::admit_original_job(input,restored,cx,|input,restored|FrameworkReservedBoundedJob::new(input,restored,binding))
}

struct FrameworkReservedBoundedJob {
    binding:semio_framework_job::OriginalSourceBinding,
    pub source:Option<(Vec<u8>,Option<Vec<u8>>)>,
    pub source_close:Option<ControlledRetirement<(Vec<u8>,Option<Vec<u8>>)>>,
    pub output:Option<Vec<u8>>,
    pub output_close:Option<ControlledRetirement<Vec<u8>>>,
    pub phase:u8,
    pub scan:usize,
    pub prefix:usize,
    pub matched:bool,
    pub cancelled:bool,

}

impl FrameworkReservedBoundedJob{
 fn new(input:Vec<u8>,restored:Option<Vec<u8>>,binding:semio_framework_job::OriginalSourceBinding)->Self{Self{binding,source:Some((input,restored)),source_close:None,output:None,output_close:None,phase:0,scan:0,prefix:0,matched:true,cancelled:false}}
 fn validate_original(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),ValueError>{cx.original_source_parent()?;if !self.binding.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job requires its exact original source and clock"))}Ok(())}
 fn step_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_actor::OriginalJobStep,OriginalTurnFailure>{
  use semio_framework_actor::OriginalJobStep as JobStep;
  self.validate_original(cx)?;if !cx.retained_recipient_is_admitted(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job recipient exceeds its original grant").into())}
  if self.cancelled||cx.is_cancelled()||cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(JobStep::Running(None))}
  let before=cx.retained_progress();let work=cx.original_work_units_spent();let result=self.advance_original(cx);
  if cx.retained_progress()!=before&&cx.original_work_units_spent()==work{cx.consume_fuel(1);}
  result
 }
 fn advance_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_actor::OriginalJobStep,OriginalTurnFailure>{
  use semio_framework_actor::OriginalJobStep as JobStep;let grant=cx.retained_grant();
  match self.phase{
   0=>{
    let raw=&self.source.as_ref().unwrap().0;
    if raw.len()<16{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=1;return Ok(JobStep::Running(None))}
    if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}
    self.matched&=raw[self.scan]==FRAMEWORK_RESERVED_JOB_MAGIC[self.scan];self.scan+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;
    if self.scan==FRAMEWORK_RESERVED_JOB_MAGIC.len(){self.prefix=if self.matched{16}else{0};self.phase=1;self.scan=0}
    Ok(JobStep::Running(None))
   },
   1=>{
    let length=self.source.as_ref().unwrap().0.len()-self.prefix;
    let result=semio_framework_job::admit_original_construction(cx,&mut self.output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:length,release_bytes:0,depth:1},|_|{let mut output=Vec::new();output.try_reserve_exact(length).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original reserved job output allocation refused"))?;let capacity=output.capacity();Ok((output,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))});
    if self.output.is_some(){self.phase=2;}result?;Ok(JobStep::Running(None))
   },
   2=>{
    let raw=&self.source.as_ref().unwrap().0;let remaining=raw.len()-self.prefix-self.scan;
    if remaining==0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=3;return Ok(JobStep::Running(None))}
    let count=remaining.min(grant.maximum_copy_bytes).min(64);if count==0{return Ok(JobStep::Running(None))}
    self.output.as_mut().unwrap().extend_from_slice(&raw[self.prefix+self.scan..self.prefix+self.scan+count]);self.scan+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(JobStep::Running(None))
   },
   3=>{if crate::close_original(&mut self.source,&mut self.source_close,cx)?{self.phase=4}Ok(JobStep::Running(None))},
   4=>{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=5;Ok(JobStep::Done(self.output.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job has no ready output"))?))},
   _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job is terminal").into()),
  }
 }
}

impl crate::BoundedJob for FrameworkReservedBoundedJob {
 fn step(&mut self,_original:&mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_,'_>,_snapshot:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_actor::OriginalJobStep,OriginalTurnFailure>{self.step_original(cx)}
 fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_original(cx)?;if self.terminal_drop_is_shallow(){return Ok(true)}
  if !self.cancelled{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.cancelled=true;return Ok(false)}
  Ok(crate::close_original(&mut self.source,&mut self.source_close,cx)?&&crate::close_original(&mut self.output,&mut self.output_close,cx)?)
 }
 fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_drop_is_shallow(){return Ok(Default::default())}
  if !self.cancelled{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(owner)=&self.source_close{return owner.next_demand(copy)}
  if let Some(owner)=&self.output_close{return owner.next_demand(copy)}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn cancel(&mut self){self.cancelled=true}
 fn checkpoint(&self)->Option<Vec<u8>>{None}
 fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.source_close.is_none()&&self.output.is_none()&&self.output_close.is_none()}
}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

