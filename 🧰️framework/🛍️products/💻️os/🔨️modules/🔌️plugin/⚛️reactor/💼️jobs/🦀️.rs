//! 🧵️ Registered jobs retain original requests, checkpoints, typed failures and output until caller-funded closure.
//! Admission and execution borrow the same original context; each producer declares its own five-axis demand.
//! Checkpoint publication and builtin semantic decoding still require migration to funded continuations.

use semio_framework_value_derive::{FromValue, ToValue};
use std::cell::{Cell,RefCell};
use std::future::Future;
use std::task::{Context, Poll};

// 🧊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): the three remaining well-known cold job
// kinds design-abi.md §2 names (`semio.infer`/`semio.mutation-plan`/`semio.migrate` — `semio.compose`
// is the live `compose-await` packet's, not this one's). Each is one submodule under this directory
// (mirrors `⚛️reactor`'s own one-`component.rs`-per-directory convention) registered into
// `builtin_registry()` below, exactly like `job_io_run`/`job_io_sniff` already are.
#[path = "💡️infer/🦀️.rs"]
mod infer;
#[path = "🔀️migrate/🦀️.rs"]
mod migrate;
#[path = "🧬️mutation-plan/🦀️.rs"]
mod mutation_plan;

//#region 🔖️PublicTypes

/// 🗺️ Absorbed from the peer's guest export `io-run` (single hop, this plugin's own registry —
/// never chains into another plugin; multi-hop routing is the host's `io-run` EFFECT, not this
/// job kind).
pub const JOB_KIND_IO_RUN: &str = "semio.io-run";
/// 🗺️ Absorbed from the peer's guest export `io-sniff`.
pub const JOB_KIND_IO_SNIFF: &str = "semio.io-sniff";
/// 💡️ design-abi.md §2: absorbed from the deleted `contributor.artifact-infer` export — see
/// `💡️infer/🦀️.rs`.
pub const JOB_KIND_INFER: &str = "semio.infer";
/// 🧬️ design-abi.md §2: absorbed from the deleted `contributor.artifact-mutation-plan` export —
/// see `🧬️mutation-plan/🦀️.rs`.
pub const JOB_KIND_MUTATION_PLAN: &str = "semio.mutation-plan";
/// 🔀️ design-abi.md §2: absorbed from the deleted `migrate-artifact` export — see
/// `🔀️migrate/🦀️.rs`.
pub const JOB_KIND_MIGRATE: &str = "semio.migrate";

/// 📜️ Every kind a plugin gets for free, in the order `builtin_registry` installs them. This is
/// the list the admitted-set law compares the live registry against, so a builtin added without a
/// bounded state machine fails that law rather than shipping as a dead route.
pub const BUILTIN_JOB_KINDS: [&str; 5] = [JOB_KIND_IO_RUN, JOB_KIND_IO_SNIFF, JOB_KIND_INFER, JOB_KIND_MUTATION_PLAN, JOB_KIND_MIGRATE];

/// ⛽️ Plain-Rust mirror of `jobs.wit`'s `record job-budget` — this module is NOT gated to
/// `component-guest`/wasm32-wasip2 (it must compile and unit-test natively), so it cannot name the
/// WIT-generated type directly; the WIT boundary conversion lives in the (leased) `JobsGuest` impl
/// in `🔌️plugin/🦀️.rs`, field-for-field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, FromValue, serde::Serialize, ToValue)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// ▶️ Plain-Rust mirror of `jobs.wit`'s `variant job-step` — see `JobBudget`'s doc for why this
/// isn't the WIT-generated type itself.
#[derive(Debug,serde::Deserialize, FromValue, serde::Serialize, ToValue,semio_framework_value::RetireOwned)]
pub enum JobStep {
    Running(Option<Vec<u8>>),
    Done(Vec<u8>),
    Failed(Vec<u8>),
}

/// ⛽️ Price of one decode/validate state action: a bounded parse of `input` that allocates nothing
/// the request did not already carry.
pub const WORK_UNITS_VALIDATE: u64 = 1;
/// ⛽️ Price of one bounded retirement action — a single `close_step` page grant.
pub const WORK_UNITS_RETIRE: u64 = 1;
/// ⛽️ Price of one execute state action: a whole unchunked native dispatch
/// (`io_run`/`io_identify`/`wire_artifact_infer`/`wire_artifact_mutation_plan`/`migrate_document`).
/// None of those is itself chunked, so the machine declares the price up front and a caller whose
/// grant cannot cover it is refused before the call rather than after it overran.
pub const WORK_UNITS_EXECUTE: u64 = 1_024;
/// ⛽️ Price of one interactive-inference pump action: one bounded `MountedWorkerJobSession` step
/// submitted to the shared worker pool.
pub const WORK_UNITS_PUMP: u64 = 64;

/// 🧩️ Production job protocol. A registered owner advances one explicit bounded state-machine
/// opportunity per `step-job`; there is no other body shape, in any build.
pub use semio_framework_os_kernel::io::io_mechanism::IoRunControl;
pub use crate::store::sqlite_snapshot::SqliteSnapshotControl;

pub trait BoundedJob {
    fn step(&mut self, budget: JobBudget, original: &mut IoRunControl<'_, '_>, snapshot: &mut SqliteSnapshotControl<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<JobStep,semio_framework_value::ValueError>;
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>;
    fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>;
    fn cancel(&mut self);
    fn checkpoint(&self) -> Option<Vec<u8>>;
    fn terminal_drop_is_shallow(&self) -> bool;
}

/// 🏭️ Non-capturing constructor for one registered retained state machine: the job id, the raw
/// `start-job` input, and — only on a checkpoint-restore replay — the bytes this kind last handed
/// back from `BoundedJob::checkpoint()`. An `Err` carries the fault bytes the first `step_job`
/// answers with, so a refused admission surfaces exactly where a refused first step would.
pub type BoundedJobAdmission = fn(u64, &mut Option<Vec<u8>>, &mut Option<Vec<u8>>, &mut semio_framework_job::StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>, semio_framework_value::ValueError>;
pub type BoundedJobDemand = fn(u64, &Option<Vec<u8>>, &Option<Vec<u8>>, &semio_framework_job::StepContext<'_>) -> Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>;

/// 📐️ A registered owner supplies its admission and exact original demand together.
#[derive(Clone,Copy)]
pub struct BoundedJobFactory {pub admit:BoundedJobAdmission,pub demands:BoundedJobDemand}

/// 📏️ Quotes the real boxed frame and original handle transfers over all five currencies.
pub(crate) fn original_job_admission_demands<T:BoundedJob>(input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>)->Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneGrant,ValueError,ValueRefusalKind};
    if input.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job admission has no owned input"))}
    Ok(RetainedCloneGrant{maximum_items:2+usize::from(restored.is_some()),maximum_copy_bytes:0,maximum_capacity_bytes:std::mem::size_of::<T>(),maximum_release_bytes:0,maximum_depth:1})
}

/// 🪺️ Admits the exact boxed state and transfers original input handles without a body copy.
pub(crate) fn admit_original_job<T:BoundedJob+'static>(input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>,build:impl FnOnce(Vec<u8>,Option<Vec<u8>>)->T)->Result<Option<Box<dyn BoundedJob>>,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
    let demand=original_job_admission_demands::<T>(input,restored)?;let grant=cx.retained_grant();
    if cx.is_cancelled()||grant.maximum_items<demand.maximum_items||grant.maximum_depth<demand.maximum_depth||grant.maximum_capacity_bytes<demand.maximum_capacity_bytes{return Ok(None)}
    cx.consume_retained(RetainedCloneProgress{copied_items:demand.maximum_items,retained_capacity_bytes:demand.maximum_capacity_bytes,..Default::default()})?;
    let owner=Box::new(build(input.take().unwrap(),restored.take()));
    Ok(Some(owner))
}

/// 🔎️ One phase body of a builtin two-phase machine. Synchronous on purpose: a bounded step may
/// not suspend, so the framework call it wraps is settled inside the step by `settle_in_step`.
pub(crate) type BuiltinPhaseFn = fn(&[u8]) -> Result<Vec<u8>, semio_framework::Fault>;

pub(crate) enum BuiltinExecuteFn {
    Pure(BuiltinPhaseFn),
    OriginalIo(fn(&[u8], &mut IoRunControl<'_, '_>, &mut SqliteSnapshotControl<'_>) -> Result<Vec<u8>, semio_framework::Fault>),
}


//#endregion

//#region 🔖️Registry

#[derive(Clone,Copy)]
struct RegisteredJobKind {kind:&'static str,factory:BoundedJobFactory}
const REGISTERED_JOB_KINDS:usize=super::requests::REQUEST_SLOTS;
crate::component_persistent_local! {
    static KIND_REGISTRY: RefCell<[Option<RegisteredJobKind>;REGISTERED_JOB_KINDS]> = RefCell::new([const {None};REGISTERED_JOB_KINDS]);
    static KIND_REGISTRY_EPOCH: Cell<u64> = Cell::new(1);
}

fn builtin_factory(kind:&str)->Option<BoundedJobFactory>{Some(match kind{
    JOB_KIND_IO_RUN=>BoundedJobFactory{admit:job_io_run,demands:two_phase_job_demands},
    JOB_KIND_IO_SNIFF=>BoundedJobFactory{admit:job_io_sniff,demands:two_phase_job_demands},
    JOB_KIND_INFER=>BoundedJobFactory{admit:infer::job_infer,demands:infer::job_infer_demands},
    JOB_KIND_MUTATION_PLAN=>BoundedJobFactory{admit:mutation_plan::job_mutation_plan,demands:two_phase_job_demands},
    JOB_KIND_MIGRATE=>BoundedJobFactory{admit:migrate::job_migrate,demands:two_phase_job_demands},
    semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND=>BoundedJobFactory{admit:crate::plugin_runtime::framework_reserved_job_factory,demands:crate::plugin_runtime::framework_reserved_job_demands},
    _=>return None,
})}

fn kind_at(index:usize)->Option<RegisteredJobKind>{
    if index<REGISTERED_JOB_KINDS{return KIND_REGISTRY.with(|registry|registry.borrow()[index])}
    let kind=match index-REGISTERED_JOB_KINDS{0=>JOB_KIND_IO_RUN,1=>JOB_KIND_IO_SNIFF,2=>JOB_KIND_INFER,3=>JOB_KIND_MUTATION_PLAN,4=>JOB_KIND_MIGRATE,5=>semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND,_=>return None};
    Some(RegisteredJobKind{kind,factory:builtin_factory(kind).unwrap()})
}

enum OriginalJobKindLookupStep{Pending,Found(RegisteredJobKind),Missing}
/// 🔎️ Original source bytes and static candidate bytes each consume their own funded turn.
struct OriginalJobKindLookup{epoch:u64,index:usize,candidate:Option<RegisteredJobKind>,offset:usize,left:Option<u8>}
impl OriginalJobKindLookup{
    fn new()->Self{Self{epoch:KIND_REGISTRY_EPOCH.with(Cell::get),index:0,candidate:None,offset:0,left:None}}
    fn demand(&self,source:&str)->semio_framework_value::RetainedCloneGrant{let copy=usize::from(self.epoch==KIND_REGISTRY_EPOCH.with(Cell::get)&&self.candidate.is_some_and(|candidate|self.offset<source.len()&&self.offset<candidate.kind.len()));semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
    fn advance(&mut self,source:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<OriginalJobKindLookupStep,semio_framework_value::ValueError>{
        use semio_framework_value::RetainedCloneProgress;
        let demand=self.demand(source);let grant=cx.retained_grant();if cx.is_cancelled()||grant.maximum_items<demand.maximum_items||grant.maximum_copy_bytes<demand.maximum_copy_bytes||grant.maximum_depth<demand.maximum_depth{return Ok(OriginalJobKindLookupStep::Pending)}
        let epoch=KIND_REGISTRY_EPOCH.with(Cell::get);
        if self.epoch!=epoch{self.epoch=epoch;self.index=0;self.candidate=None;self.offset=0;self.left=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(OriginalJobKindLookupStep::Pending)}
        let Some(candidate)=self.candidate else{if self.index>=REGISTERED_JOB_KINDS+6{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(OriginalJobKindLookupStep::Missing)}self.candidate=kind_at(self.index);if self.candidate.is_none(){self.index+=1}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(OriginalJobKindLookupStep::Pending)};
        if self.offset==source.len()||self.offset==candidate.kind.len(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;if self.offset==source.len()&&self.offset==candidate.kind.len(){return Ok(OriginalJobKindLookupStep::Found(candidate))}self.index+=1;self.candidate=None;self.offset=0;self.left=None;return Ok(OriginalJobKindLookupStep::Pending)}
        if let Some(left)=self.left.take(){let right=candidate.kind.as_bytes()[self.offset];if left==right{self.offset+=1}else{self.index+=1;self.candidate=None;self.offset=0}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?}else{self.left=Some(source.as_bytes()[self.offset]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?}
        Ok(OriginalJobKindLookupStep::Pending)
    }
}

/// 📤️ Installs only static producer metadata; saturation retains the declaration with its caller.
pub fn register_bounded_job_kind(kind:&'static str,factory:BoundedJobFactory)->bool{
    let Some(next_epoch)=KIND_REGISTRY_EPOCH.with(Cell::get).checked_add(1)else{return false};
    KIND_REGISTRY.with(|registry|{let mut registry=registry.borrow_mut();
        if let Some(entry)=registry.iter_mut().flatten().find(|entry|entry.kind==kind){entry.factory=factory;KIND_REGISTRY_EPOCH.with(|epoch|epoch.set(next_epoch));return true}
        let Some(slot)=registry.iter_mut().find(|slot|slot.is_none())else{return false};
        *slot=Some(RegisteredJobKind{kind,factory});KIND_REGISTRY_EPOCH.with(|epoch|epoch.set(next_epoch));true
    })
}

//#endregion

//#region 🔖️Slots

/// 📥️ One original request and checkpoint stay owned until admitted and fully closed.
#[derive(semio_framework_value::RetireOwned)]
pub struct OriginalJobAdmission {
    pub job:u64,
    pub kind:String,
    pub input:Option<Vec<u8>>,
    pub checkpoint:Option<Vec<u8>>,
}

enum JobBody {
    Resolving(OriginalJobKindLookup),
    Preparing(BoundedJobFactory),
    Bounded(Box<dyn BoundedJob>),
    AdmissionFailed(crate::component::extension_invocation_failure::RetainedExtensionFaultReply),
}

struct JobSlot {
    job:u64,
    identity:(u64,u64),
    source:Option<OriginalJobAdmission>,
    source_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<OriginalJobAdmission>>,
    body:Option<JobBody>,
    outcome:Option<JobStep>,
    outcome_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<JobStep>>,
    rejected_output:Option<Vec<u8>>,
    rejected_output_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<u8>>>,
    cancelled:bool,
    cancellation_fault_started:bool,
}
const JOB_SLOTS:usize=super::requests::REQUEST_SLOTS;
crate::component_persistent_local! {
    static JOBS: RefCell<[Option<JobSlot>;JOB_SLOTS]> = RefCell::new([const {None};JOB_SLOTS]);
}

//#endregion

//#region 🔖️Lifecycle

/// 🔎️ Quotes the registered original producer with the same retained request and caller identity.
pub fn job_admission_demands(job:u64,cx:&semio_framework_job::StepContext<'_>)->Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>{
    use semio_framework_value::{ValueError,ValueRefusalKind};
    JOBS.with(|jobs|{let jobs=jobs.borrow();let slot=jobs.iter().flatten().find(|slot|slot.job==job).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"job has no original admission owner"))?;
        if slot.identity!=(cx.operation().0,cx.generation().0){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"job admission quote differs from original identity"))}
        let source=slot.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"job admission quote lost its original source"))?;
        match slot.body.as_ref(){Some(JobBody::Resolving(lookup))=>Ok(lookup.demand(&source.kind)),Some(JobBody::Preparing(factory))=>(factory.demands)(job,&source.input,&source.checkpoint,cx),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"job admission is no longer preparing"))}
    })
}

/// 🤝️ The actual caller keeps its exact request on denial or saturation; admission transfers handles only.
pub async fn start_job(admission:&mut Option<OriginalJobAdmission>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
    let job=admission.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"job admission has no original request"))?.job;
    let grant=cx.retained_grant();if cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
    JOBS.with(|jobs|{
        let mut jobs=jobs.borrow_mut();
        if jobs.iter().flatten().any(|slot|slot.job==job){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"job id already has an original owner"))}
        let Some(destination)=jobs.iter_mut().find(|slot|slot.is_none())else{return Ok(false)};
        let body=JobBody::Resolving(OriginalJobKindLookup::new());
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
        *destination=Some(JobSlot{job,identity:(cx.operation().0,cx.generation().0),source:admission.take(),source_close:None,body:Some(body),outcome:None,outcome_close:None,rejected_output:None,rejected_output_close:None,cancelled:false,cancellation_fault_started:false});
        Ok(true)
    })
}

/// 🛑️ Cancellation signals the same source and owner; only funded close turns remove its slot.
pub async fn cancel_job(job:u64){JOBS.with(|jobs|{if let Some(slot)=jobs.borrow_mut().iter_mut().flatten().find(|slot|slot.job==job){slot.cancelled=true;if let Some(JobBody::Bounded(owner))=&mut slot.body{owner.cancel()}}});}

pub(crate) fn close_original<T:semio_framework_value::retirement::RetireOwned>(source:&mut Option<T>,closing:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneProgress,retirement::controlled::ControlledRetirement};
    let grant=cx.retained_grant();if source.is_none()&&closing.is_none(){return Ok(true)}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
    if let Some(original)=source.take(){
        match ControlledRetirement::new(original){Ok(owner)=>*closing=Some(owner),Err((error,original))=>{*source=Some(original);return Err(error)}}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)
    }
    let owner=closing.as_mut().unwrap();
    if owner.terminal_is_empty(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*closing=None;return Ok(true)}
    let step=owner.step(grant).map_err(|error|{match cx.consume_retained(error.retained_progress()){Ok(())=>error,Err(refusal)=>refusal}})?;
    cx.consume_retained(step.progress())?;Ok(false)
}

fn close_body(slot:&mut JobSlot,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
    if !close_original(&mut slot.rejected_output,&mut slot.rejected_output_close,cx)?{return Ok(false)}
    let Some(body)=slot.body.as_mut()else{return Ok(true)};
    match body{
        JobBody::Bounded(owner)=>{if !owner.close_step(cx)?{return Ok(false)}if !owner.terminal_drop_is_shallow(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"job close reports terminal with an original deep owner"))}},
        JobBody::AdmissionFailed(reply)=>{if !reply.terminal_is_empty(){reply.begin_close();if let Some(output)=reply.advance(cx)?{slot.rejected_output=Some(output);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing original diagnostic returned an owned output"))}return Ok(false)}},
        JobBody::Resolving(_)|JobBody::Preparing(_)=>{},
    }
    let release=match body{JobBody::Bounded(owner)=>std::mem::size_of_val(owner.as_ref()),_=>0};let grant=cx.retained_grant();
    if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<release{return Ok(false)}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;drop(slot.body.take());Ok(true)
}

/// ▶️ Every admitted state, retained output, refusal and final release uses the same original context.
pub async fn step_job(job:u64,budget:JobBudget,original:&mut IoRunControl<'_,'_>,snapshot:&mut SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<JobStep,semio_framework_value::ValueError>{
    use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
    JOBS.with(|jobs|{
        let mut jobs=jobs.borrow_mut();let entry=jobs.iter_mut().find(|entry|entry.as_ref().is_some_and(|slot|slot.job==job)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"job has no original registered owner"))?;let slot=entry.as_mut().unwrap();
        if slot.identity!=(cx.operation().0,cx.generation().0){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"job turn belongs to another original caller"))}
        if !slot.cancellation_fault_started&&(slot.cancelled||cx.is_cancelled()){
            if let Some(JobBody::Bounded(owner))=&mut slot.body{owner.cancel()}
            if !close_body(slot,cx)?||!close_original(&mut slot.outcome,&mut slot.outcome_close,cx)?||!close_original(&mut slot.source,&mut slot.source_close,cx)?{return Ok(JobStep::Running(None))}
            slot.cancellation_fault_started=true;slot.body=Some(JobBody::AdmissionFailed(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Value(ValueError::literal(ValueRefusalKind::Canceled,"original job was canceled")))));return Ok(JobStep::Running(None))
        }
        if slot.outcome.is_some(){
            if !close_body(slot,cx)?||!close_original(&mut slot.source,&mut slot.source_close,cx)?{return Ok(JobStep::Running(None))}
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;let outcome=slot.outcome.take().unwrap();*entry=None;return Ok(outcome)
        }
        match slot.body.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original job lost its retained body"))?{
            JobBody::Resolving(lookup)=>{if budget.fuel==0{return Ok(JobStep::Running(None))}let source=slot.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original job lookup lost its retained source"))?;match lookup.advance(&source.kind,cx)?{OriginalJobKindLookupStep::Pending=>{},OriginalJobKindLookupStep::Found(candidate)=>slot.body=Some(JobBody::Preparing(candidate.factory)),OriginalJobKindLookupStep::Missing=>slot.body=Some(JobBody::AdmissionFailed(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Value(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"job kind has no registered original owner")))))}Ok(JobStep::Running(None))},
            JobBody::Preparing(factory)=>{let source=slot.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original job lost its admission source"))?;let demand=(factory.demands)(job,&source.input,&source.checkpoint,cx)?;let grant=cx.retained_grant();if grant.maximum_items<demand.maximum_items||grant.maximum_copy_bytes<demand.maximum_copy_bytes||grant.maximum_capacity_bytes<demand.maximum_capacity_bytes||grant.maximum_release_bytes<demand.maximum_release_bytes||grant.maximum_depth<demand.maximum_depth{return Ok(JobStep::Running(None))}let body=match (factory.admit)(job,&mut source.input,&mut source.checkpoint,cx){Ok(Some(owner))=>Some(JobBody::Bounded(owner)),Ok(None)=>None,Err(error)=>Some(JobBody::AdmissionFailed(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Value(error))))};if let Some(body)=body{slot.body=Some(body)}Ok(JobStep::Running(None))},
            JobBody::AdmissionFailed(reply)=>{if let Some(bytes)=reply.advance(cx)?{slot.outcome=Some(JobStep::Failed(bytes))}Ok(JobStep::Running(None))},
            JobBody::Bounded(owner)=>match owner.step(budget, original, snapshot, cx)?{step@JobStep::Running(_)=>Ok(step),step=>{slot.outcome=Some(step);Ok(JobStep::Running(None))}},
        }
    })
}

//#endregion

//#region 🔖️Checkpoint

/// 📸️ One entry of the checkpoint pack's `jobs: Vec<{job, kind, input, checkpoint}>` — see module
/// doc's checkpoint section and this packet's `## lease-requests` for the exact diff into
/// `⚛️reactor/📸️checkpoint/🦀️.rs` that embeds these.
#[derive(Clone, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub struct JobCheckpointEntry {
    pub job: u64,
    pub kind: String,
    pub input: Option<Vec<u8>>,
    pub checkpoint: Option<Vec<u8>>,
}

/// 📸️ Every job this actor currently has open, in no particular order — `restore_job` (called by
/// the leased `checkpoint::restore` for each entry) is what re-establishes them.
pub async fn checkpoint_jobs() -> Vec<JobCheckpointEntry> {
    JOBS.with(|jobs|jobs.borrow().iter().flatten().filter_map(|slot|{
        let source=slot.source.as_ref()?;
        let checkpoint=match slot.body.as_ref(){Some(JobBody::Bounded(owner))=>owner.checkpoint(),_=>None};
        Some(JobCheckpointEntry{job:slot.job,kind:source.kind.clone(),input:source.input.clone(),checkpoint})
    }).collect())
}

//#endregion

//#region 🔖️Fault

// 🚫️async: E1 pure constructor consumed by sync error-mapping closures (`.map_err(|error| fault(...))`,
// `.ok_or_else(|| fault(...))`) pervasively across this module — see R9. `Fault::new` itself is sync.
fn fault(code: &str, message: impl Into<String>) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(code), message.into())
}

// 🚫️async: E1 — see `fault`'s own comment above; `dsl::encode_fault_bytes` is sync too.
fn fault_bytes(code: &str, message: String) -> Vec<u8> {
    semio_framework_diagnostic::encode_fault_bytes(&fault(code, message))
}

/// 🌉️ Settles one framework call inside a bounded step. Every native dispatch a builtin kind
/// drives (`io_run`/`io_identify`/`wire_artifact_infer`/`wire_artifact_mutation_plan`/
/// `migrate_document`) is `async` by signature only — `io-async-signatures` made the SIGNATURES
/// uniform without adding a suspension point to any of those bodies — and a bounded step has no
/// executor to park on, by design (module doc, "Why the opaque-future executor is gone"). So the
/// future is polled exactly once; a `Pending` is not a hang here but a typed
/// `job.<kind>.suspended` refusal naming the guarantee a future edit broke.
fn settle_in_step(fault_prefix: &str, future: impl Future<Output = Result<Vec<u8>, semio_framework::Fault>>) -> Result<Vec<u8>, semio_framework::Fault> {
    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = Box::pin(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => Err(fault(&format!("{fault_prefix}.suspended"), "a bounded job step drove a framework call that suspended; a builtin job kind may only drive calls that complete within one step")),
    }
}

//#endregion

//#region 🔖️TwoPhase
/// 🌗️ Shared 2-state machine for `semio.io-run`/`semio.io-sniff`/`semio.infer`'s registry route/
/// `semio.mutation-plan`/`semio.migrate`: none of their underlying native calls is itself chunked
/// — real sub-call preemption is the same "blocked upstream" gap the dormant WFC solver names
/// (`✏️s/🔌️plugins/🌀️procedural/🦀️.rs`'s own comment) — but the JOB ITSELF is genuinely sliceable:
/// two real `step_job` calls, monotonic progress on the first, a correct checkpoint/restore resume,
/// and a declared price per state, not a single call dressed up as a job. `Decode` runs on the
/// FIRST step only (validates `input` and reports identity-shaped progress bytes, then makes
/// `PHASE_DECODED` its checkpoint so a restore starts straight at `Execute`); `Execute` runs on the
/// step after, whether that is the second step of an uninterrupted run or the first step after a
/// restore. Both phases independently re-read `input` from scratch (never threading a decoded value
/// across the step boundary) — a second cheap parse is harmless and keeps `restore` correct without
/// a second, richer checkpoint payload.
pub(crate) const PHASE_DECODED: &[u8] = b"phase.decoded";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TwoPhaseState {
    Decode,
    Execute,
    Complete,
}

pub(crate) struct TwoPhaseBoundedJob {
    fault_prefix: &'static str,
    state: TwoPhaseState,
    input: Vec<u8>,
    restored:Option<Vec<u8>>,
    checkpoint_scan:usize,
    checkpoint_complete:bool,
    failure:Option<crate::component::extension_invocation_failure::RetainedExtensionFaultReply>,
    aborted_output:Option<Vec<u8>>,
    close_payload:Option<(Vec<u8>,Option<Vec<u8>>,Option<Vec<u8>>)>,
    source_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(Vec<u8>,Option<Vec<u8>>,Option<Vec<u8>>)>>,
    decode: BuiltinPhaseFn,
    execute: BuiltinExecuteFn,
    cancelled: bool,
}

impl TwoPhaseBoundedJob {
    /// 🎟️ Admits the machine, starting at `Execute` when the restore bytes say this kind already
    /// reported `PHASE_DECODED` before the actor was torn down.
    pub(crate) fn admit(fault_prefix: &'static str, input: Vec<u8>, restored: Option<Vec<u8>>, decode: BuiltinPhaseFn, execute: BuiltinExecuteFn) -> Self {
        let checkpoint_complete=restored.is_none();
        Self { fault_prefix, state:TwoPhaseState::Decode, input, restored, checkpoint_scan:0,checkpoint_complete,failure:None,aborted_output:None,close_payload:None,source_close:None,decode, execute, cancelled: false }
    }

    /// 🚫️async: E1 pure price table consumed by `step`'s sync budget gate.
    fn price(&self) -> u64 {
        match self.state {
            TwoPhaseState::Decode => WORK_UNITS_VALIDATE,
            TwoPhaseState::Execute => WORK_UNITS_EXECUTE,
            TwoPhaseState::Complete => 0,
        }
    }
}

impl BoundedJob for TwoPhaseBoundedJob {
    fn step(&mut self,budget:JobBudget,original:&mut IoRunControl<'_,'_>,snapshot:&mut SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<JobStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
        if self.cancelled||cx.is_cancelled(){return Ok(JobStep::Running(None))}
        if let Some(reply)=&mut self.failure{return Ok(match reply.advance(cx)?{Some(bytes)=>JobStep::Failed(bytes),None=>JobStep::Running(None)})}
        if !self.checkpoint_complete{
            let source=self.restored.as_ref().unwrap();
            if source.len()!=PHASE_DECODED.len(){self.failure=Some(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Value(ValueError::literal(ValueRefusalKind::InvalidValue,"original job checkpoint has an invalid phase"))));return Ok(JobStep::Running(None))}
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
            let matched=source[self.checkpoint_scan]==PHASE_DECODED[self.checkpoint_scan];cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;self.checkpoint_scan+=1;
            if !matched{self.failure=Some(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Value(ValueError::literal(ValueRefusalKind::InvalidValue,"original job checkpoint has an unknown phase"))))}else if self.checkpoint_scan==PHASE_DECODED.len(){self.checkpoint_complete=true;self.state=TwoPhaseState::Execute}
            return Ok(JobStep::Running(None))
        }
        if budget.fuel<self.price(){return Ok(JobStep::Running(None))}
        let result=match self.state{
            TwoPhaseState::Decode=>(self.decode)(&self.input),
            TwoPhaseState::Execute=>match self.execute{BuiltinExecuteFn::Pure(execute)=>execute(&self.input),BuiltinExecuteFn::OriginalIo(execute)=>execute(&self.input,original,snapshot)},
            TwoPhaseState::Complete=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job has no remaining state action")),
        };
        Ok(match result{
            Ok(bytes)=>{if self.state==TwoPhaseState::Decode{self.state=TwoPhaseState::Execute;JobStep::Running(Some(bytes))}else{self.state=TwoPhaseState::Complete;JobStep::Done(bytes)}},
            Err(error)=>{self.state=TwoPhaseState::Complete;self.failure=Some(crate::component::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::component::extension_invocation_failure::ExtensionInvocationCause::Fault(error)));JobStep::Running(None)},
        })
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
        if let Some(reply)=&mut self.failure{
            if !reply.terminal_is_empty(){reply.begin_close();if let Some(bytes)=reply.advance(cx)?{self.aborted_output=Some(bytes);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing original job failure returned unpublished bytes"))}return Ok(false)}
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.failure=None;return Ok(false)
        }
        if self.close_payload.is_none()&&self.source_close.is_none()&&(self.input.capacity()!=0||self.restored.is_some()||self.aborted_output.is_some()){
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}self.close_payload=Some((std::mem::take(&mut self.input),self.restored.take(),self.aborted_output.take()));
        }
        close_original(&mut self.close_payload,&mut self.source_close,cx)
    }

    fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::RetirementDemand;
        if let Some(reply)=&self.failure{return reply.demands(copy)}
        if let Some(owner)=&self.source_close{return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
        Ok(RetirementDemand{depth:usize::from(self.close_payload.is_some()||self.input.capacity()!=0||self.restored.is_some()||self.aborted_output.is_some()),..Default::default()})
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        matches!(self.state, TwoPhaseState::Execute).then(|| PHASE_DECODED.to_vec())
    }

    fn terminal_drop_is_shallow(&self)->bool{self.input.capacity()==0&&self.restored.is_none()&&self.failure.is_none()&&self.aborted_output.is_none()&&self.close_payload.is_none()&&self.source_close.is_none()}
}
//#endregion

//#region 🔖️BuiltinKinds

/// 🌉️ `input` is the JSON-encoded `{source, target, payload}` the WIT guest export `io-run` used
/// to take as three separate params; `Ok` carries the JSON-encoded `io_schema::IoPayload` result,
/// matching the old export's ok return exactly.
// 🧬️ `FromValue` only: this struct is decoded exclusively by `dsl::os_pack::json::from_json_str`
// (`from_json_str<T: FromValue>`), never by serde. The `serde::Deserialize` derive was vestigial and
// was the sole reason `io_schema::IoPayload` still had to implement `serde::Deserialize`.
#[derive(::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
struct IoRunInput {
    source: String,
    target: String,
    payload: semio_framework::io_schema::IoPayload,
}

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory`, whose shape a factory must
// match exactly; the admission body is a pure constructor call.
fn two_phase_job_demands(_job:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_cx:&semio_framework_job::StepContext<'_>)->Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>{original_job_admission_demands::<TwoPhaseBoundedJob>(input,restored)}

fn job_io_run(_job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>,semio_framework_value::ValueError> {
    admit_original_job(input,restored,cx,|input,restored|TwoPhaseBoundedJob::admit("job.io-run", input, restored, decode_io_run, BuiltinExecuteFn::OriginalIo(execute_io_run)))
}

// 🚫️async: E4 fn-pointer slot — see `job_io_run`'s own comment above; same factory shape.
fn job_io_sniff(_job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>,semio_framework_value::ValueError> {
    admit_original_job(input,restored,cx,|input,restored|TwoPhaseBoundedJob::admit("job.io-sniff", input, restored, decode_io_sniff, BuiltinExecuteFn::Pure(execute_io_sniff)))
}

/// 🔎️ Validates `input` decodes as `{source, target, payload}` and reports the hop identity as the
/// first state action's progress bytes, so a malformed request fails before the io registry is
/// ever consulted — the same decode fault code the pre-bounded `run_io_run` raised.
// 🚫️async: E4 phase slot — `BuiltinPhaseFn` is synchronous by contract (module doc, "Budget"); the
// hop parse it performs has no await of its own.
fn decode_io_run(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    decode_io_hop(input, "job.io-run", JOB_KIND_IO_RUN)
}

// 🚫️async: E4 phase slot — see `decode_io_run`; only the fault code and kind name differ.
fn decode_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    decode_io_hop(input, "job.io-sniff", JOB_KIND_IO_SNIFF)
}

// 🚫️async: E1 shared pure parse consumed by the two sync phase slots above; `code` is the kind's
// fault-code stem, so a malformed envelope keeps the pre-bounded `<stem>.decode` code and an
// unparseable dialect coordinate keeps the bare `<stem>` code the execute body used to raise.
fn decode_io_hop(input: &[u8], code: &str, kind: &str) -> Result<Vec<u8>, semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let decode_code = format!("{code}.decode");
    let input_text = std::str::from_utf8(input).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let IoRunInput { source, target, .. } = semio_framework_pack_json::from_json_str::<IoRunInput>(input_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let source = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault(code, message))?;
    let target = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault(code, message))?;
    Ok(format!("{}->{}", source.to_coordinate(), target.to_coordinate()).into_bytes())
}

// 🚫️async: E4 phase slot — see `decode_io_run`; the native call inside is settled by `settle_in_step`.
fn execute_io_run(input: &[u8], original: &mut IoRunControl<'_, '_>, snapshot: &mut SqliteSnapshotControl<'_>) -> Result<Vec<u8>, semio_framework::Fault> {
    settle_in_step("job.io-run", run_io_run(input, original, snapshot))
}

// 🚫️async: E4 phase slot — see `decode_io_run`; the native call inside is settled by `settle_in_step`.
fn execute_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    settle_in_step("job.io-sniff", run_io_sniff(input))
}

#[derive(semio_framework_value::RetireOwned)]
struct JobIoReceiving {
    input: Option<IoRunInput>,
    source: Option<semio_framework_artifact_reference::ArtifactDialect>,
    target: Option<semio_framework_artifact_reference::ArtifactDialect>,
    route: Option<semio_framework::io_schema::IoRoute>,
    payload: Option<semio_framework::io_schema::IoPayload>,
    outcome: Option<semio_framework::io_schema::IoResult<semio_framework::io_schema::IoPayload>>,
}

/// 🌉️ Retains the original parsed request and exact hop result inside its admitted native recipient.
async fn run_io_run(input: &[u8], original: &mut IoRunControl<'_, '_>, snapshot: &mut SqliteSnapshotControl<'_>) -> Result<Vec<u8>, semio_framework::Fault> {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    original.receive_nested::<JobIoReceiving, Result<Vec<u8>, semio_framework::Fault>>(|slot, run| {
        *slot = Some(JobIoReceiving { input: None, source: None, target: None, route: None, payload: None, outcome: None });
        let frame = slot.as_mut().unwrap();
        Ok(settle_in_step("job.io-run", async {
            let text = std::str::from_utf8(input).map_err(|_| fault("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")))?;
            frame.input = Some(semio_framework_pack_json::from_json_str::<IoRunInput>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| fault("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")))?);
            let request = frame.input.as_ref().unwrap();
            frame.source = Some(semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&request.source).map_err(|message| fault("job.io-run", message))?);
            frame.target = Some(semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&request.target).map_err(|message| fault("job.io-run", message))?);
            let source = frame.source.as_ref().unwrap();
            let target = frame.target.as_ref().unwrap();
            let descriptor = semio_framework_os_kernel::io::io_mechanism::io_entries().into_iter().find(|entry| &entry.from == source && &entry.into == target).ok_or_else(|| fault("job.io-run", format!("no local io entry for hop {} -> {}", source.to_coordinate(), target.to_coordinate())))?;
            let fidelity = descriptor.fidelity;
            frame.route = Some(semio_framework::io_schema::IoRoute { hops: vec![descriptor], fidelity });
            frame.payload = Some(std::mem::replace(&mut frame.input.as_mut().unwrap().payload, semio_framework::io_schema::IoPayload::Binary(Vec::new())));
            frame.outcome = Some(semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control(frame.route.as_ref().unwrap(), &mut frame.payload, run, snapshot).await);
            match frame.outcome.as_ref().unwrap() {
                Ok(outcome) => Ok(semio_framework_pack_json::to_json_string(&outcome.value).into_bytes()),
                Err(error) => Err(fault("job.io-run", error.cause.message.as_ref())),
            }
        }))
    }).map_err(|error| semio_framework_diagnostic::FaultFrom::to_fault(&error))?
}

/// 🔍️ Body unchanged from the pre-rewrite `run_io_sniff` — `Ok` carries a single-byte `Vec<u8>` of
/// `io_schema::Confidence::rank()` (`0..=3`), matching the old export's `u8` return.
async fn run_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let input_text = std::str::from_utf8(input).map_err(|_| fault("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")))?;
    let IoRunInput { source, target, payload } =
        semio_framework_pack_json::from_json_str::<IoRunInput>(input_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| fault("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")))?;
    let source = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault("job.io-sniff", message))?;
    let target = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault("job.io-sniff", message))?;
    let carrier = semio_framework_artifact_reference::ArtifactDialect::from(match &payload {
        semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
        semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
    });
    if source != carrier {
        return Ok(vec![semio_framework::io_schema::Confidence::None.rank()]);
    }
    let confidence = semio_framework_os_kernel::io::io_mechanism::io_identify(&payload).await.into_iter().find(|(dialect, _)| *dialect == target).map_or(semio_framework::io_schema::Confidence::None, |(_, confidence)| confidence);
    Ok(vec![confidence.rank()])
}

//#endregion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🎟️admission/🦀️.rs"]
mod original_job_admission_tests;
