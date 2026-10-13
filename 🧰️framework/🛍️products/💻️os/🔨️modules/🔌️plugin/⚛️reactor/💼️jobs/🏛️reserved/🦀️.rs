//! 🏛️ The framework-reserved job kind of the job table: its bounded job, admission factory and the registry driver the instance runs it through.
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::sdk_fault::plugin_sdk_fault;
use crate::reserved_jobs::FRAMEWORK_RESERVED_JOB_MAGIC;

/// 🧪️ Drives one framework-reserved spawn-job through the real reactor registry: a genuine original admission under a
/// caller context funded with at least the registry's own quoted grant, then bounded steps until its terminal output.
pub(crate) fn drive_framework_reserved_registry_job(job: u64, input: Vec<u8>, policy: RetainedCloneGrant) -> Result<Vec<u8>, Fault> {
    drive_framework_reserved_registry_job_counted(job, input, policy).map(|(output, _)| output)
}

/// 🧪️ The same registry drive, also reporting how many funded `step_job` turns the job needed.
fn drive_framework_reserved_registry_job_counted(job: u64, input: Vec<u8>, policy: RetainedCloneGrant) -> Result<(Vec<u8>, u32), Fault> {
    use crate::reactor::jobs::{start_job, step_job, IoRunControl, JobBudget, JobStep, OriginalJobAdmission, SqliteSnapshotControl, WORK_UNITS_EXECUTE};
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
            Err(error) => return Err(plugin_sdk_fault(error.to_string())),
        }
        grant = widest(grant, crate::reactor::jobs::job_admission_demands(job, &cx).unwrap_or(grant));
    }
    if !admitted {
        return Err(stall("framework reserved spawn-job was never admitted by the reactor registry"));
    }
    let mut steps = 0u32;
    for _ in 0..TURNS {
        if let Some(quote) = {
            let mut sequence = 0;
            let mut receipt = RetainedCloneProgress::default();
            let cx = StepContext::new(operation, generation, StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
            crate::reactor::jobs::job_admission_demands(job, &cx).ok()
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
        match resolve_ready(step_job(job, JobBudget { fuel: WORK_UNITS_EXECUTE, deadline_ms: 1 }, &mut original, &mut snapshot, &mut cx)) {
            Ok(JobStep::Running(_)) => {}
            Ok(JobStep::Done(bytes)) => return Ok((bytes, steps)),
            Ok(JobStep::Failed(bytes)) => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-failed"), String::from_utf8_lossy(&bytes).into_owned())),
            Err(error) => return Err(plugin_sdk_fault(error.to_string())),
        }
    }
    Err(stall("framework reserved spawn-job did not reach a terminal step"))
}

pub(crate) fn framework_reserved_job_demands(_job:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_cx:&semio_framework_job::StepContext<'_>)->Result<RetainedCloneGrant,ValueError>{crate::reactor::jobs::original_job_admission_demands::<FrameworkReservedBoundedJob>(input,restored)}

pub(crate) fn framework_reserved_job_factory(_job:u64,input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Box<dyn crate::reactor::jobs::BoundedJob>>,ValueError>{
    crate::reactor::jobs::admit_original_job(input,restored,cx,|input,restored|FrameworkReservedBoundedJob{source:Some((input,restored)),source_close:None,output:None,output_close:None,phase:0,scan:0,prefix:0,matched:true,cancelled:false})
}

struct FrameworkReservedBoundedJob {
    source:Option<(Vec<u8>,Option<Vec<u8>>)>,
    source_close:Option<ControlledRetirement<(Vec<u8>,Option<Vec<u8>>)>>,
    output:Option<Vec<u8>>,
    output_close:Option<ControlledRetirement<Vec<u8>>>,
    phase:u8,
    scan:usize,
    prefix:usize,
    matched:bool,
    cancelled:bool,
}

impl crate::reactor::jobs::BoundedJob for FrameworkReservedBoundedJob {
    fn step(&mut self,_budget:crate::reactor::jobs::JobBudget,_original:&mut crate::reactor::jobs::IoRunControl<'_,'_>,_snapshot:&mut crate::reactor::jobs::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<crate::reactor::jobs::JobStep,ValueError>{
        use crate::reactor::jobs::JobStep;
        if self.cancelled||cx.is_cancelled(){return Ok(JobStep::Running(None))}
        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
        match self.phase{
            0=>{
                let raw=&self.source.as_ref().unwrap().0;
                if raw.len()<16{self.phase=1;return Ok(JobStep::Running(None))}
                if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}
                self.matched&=raw[self.scan]==FRAMEWORK_RESERVED_JOB_MAGIC[self.scan];self.scan+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;
                if self.scan==FRAMEWORK_RESERVED_JOB_MAGIC.len(){self.prefix=if self.matched{16}else{0};self.phase=1;self.scan=0}
                Ok(JobStep::Running(None))
            },
            1=>{
                let length=self.source.as_ref().unwrap().0.len()-self.prefix;if grant.maximum_capacity_bytes<length{return Ok(JobStep::Running(None))}
                let output=self.output.get_or_insert_with(Vec::new);let before=output.capacity();output.try_reserve_exact(length).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original reserved job output allocation refused"))?;
                self.phase=2;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity()-before,..Default::default()})?;Ok(JobStep::Running(None))
            },
            2=>{
                let raw=&self.source.as_ref().unwrap().0;let remaining=raw.len()-self.prefix-self.scan;if remaining==0{self.phase=3;return Ok(JobStep::Running(None))}
                let count=remaining.min(grant.maximum_copy_bytes).min(64);if count==0{return Ok(JobStep::Running(None))}
                self.output.as_mut().unwrap().extend_from_slice(&raw[self.prefix+self.scan..self.prefix+self.scan+count]);self.scan+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(JobStep::Running(None))
            },
            3=>{if crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?{self.phase=4}Ok(JobStep::Running(None))},
            4=>{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=5;Ok(JobStep::Done(self.output.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job has no ready output"))?))},
            _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job is terminal")),
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,ValueError>{
        self.cancelled=true;
        Ok(crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?&&crate::reactor::jobs::close_original(&mut self.output,&mut self.output_close,cx)?)
    }

    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=&self.source_close{return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
        if let Some(owner)=&self.output_close{return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
        if self.cancelled{return Ok(RetirementDemand{depth:usize::from(self.source.is_some()||self.output.is_some()),..Default::default()})}
        Ok(match self.phase{0|2=>RetirementDemand{copy_bytes:1,depth:1,..Default::default()},1=>RetirementDemand{capacity_bytes:self.source.as_ref().unwrap().0.len()-self.prefix,depth:1,..Default::default()},3|4=>RetirementDemand{depth:1,..Default::default()},_=>Default::default()})
    }

    fn cancel(&mut self){self.cancelled=true}
    fn checkpoint(&self)->Option<Vec<u8>>{None}
    fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.source_close.is_none()&&self.output.is_none()&&self.output_close.is_none()}
}
