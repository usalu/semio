//! 🕳️ Retirement slots and nodes: the session state constants, the worker job retirement slots, wake and header, the retirement pump and the retirement node phase and pump. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
pub(super) const SESSION_TRANSITION: u8 = 0;

pub(super) const SESSION_IDLE: u8 = 1;

pub(super) const SESSION_SUBMITTED: u8 = 2;

pub(super) const SESSION_OUTCOME: u8 = 3;

pub(super) const SESSION_TERMINAL: u8 = 4;

pub(super) const SESSION_REJECTED: u8 = 5;

pub(super) const SESSION_CHECKED_OUT: u8 = 6;

pub(super) const SESSION_CLOSE: u8 = 7;

pub(super) const SESSION_EMPTY: u8 = 8;

pub const WORKER_JOB_SESSION_SLOTS: usize = 256;

#[repr(C)]
pub(super) struct WorkerJobRetirementHeader {
    pub(super) slot: usize,
    pub(super) phase: unsafe fn(*mut WorkerJobRetirementHeader) -> WorkerJobClosePhase,
    pub(super) pump: unsafe fn(*mut WorkerJobRetirementHeader, &mut StepContext<'_>) -> Result<WorkerJobCloseStep,OriginalTurnFailure>,
    pub(super) destroy: unsafe fn(*mut WorkerJobRetirementHeader),
}

const WORKER_JOB_RETIREMENT_RESERVED: *mut WorkerJobRetirementHeader = std::ptr::without_provenance_mut(1);

pub(super) static WORKER_JOB_RETIREMENT_SLOTS: [AtomicPtr<WorkerJobRetirementHeader>; WORKER_JOB_SESSION_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; WORKER_JOB_SESSION_SLOTS];

pub(super) static WORKER_JOB_RETIREMENT_WAKE: AtomicBool = AtomicBool::new(false);

pub(super) fn reserve_worker_job_retirement_slot() -> Option<usize> {
    WORKER_JOB_RETIREMENT_SLOTS.iter().enumerate().find_map(|(index, slot)| slot.compare_exchange(std::ptr::null_mut(), WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| index))
}

/// 🧹️ Whether any dropped worker-job session still parks a node in the process-wide retirement array.
/// A host reads this to decide that it is NOT idle: the parked node holds one of the
/// [`WORKER_JOB_SESSION_SLOTS`] admissions every later session competes for, and only
/// [`pump_worker_job_retirements`] gives it back.
pub fn worker_job_retirements_are_parked() -> bool {
    WORKER_JOB_RETIREMENT_SLOTS.iter().any(|slot| {
        let pointer = slot.load(Ordering::Acquire);
        !pointer.is_null() && pointer != WORKER_JOB_RETIREMENT_RESERVED
    })
}

pub fn take_worker_job_retirement_wake() -> bool {
    WORKER_JOB_RETIREMENT_WAKE.swap(false, Ordering::AcqRel)
}

/// 🔭️ Observes one parked original owner under exclusive slot admission without funding it.
pub fn worker_job_retirement_phase(maximum_sessions:usize)->Option<WorkerJobClosePhase>{
    let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}
        let pointer=slot.load(Ordering::Acquire);
        if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}
        inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let phase=unsafe{((*pointer).phase)(pointer)};
        slot.store(pointer,Ordering::Release);
        return Some(phase);
    }
    worker_job_retirements_are_parked().then_some(WorkerJobClosePhase::CheckedOut)
}

pub fn pump_worker_job_retirements(maximum_sessions:usize,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
    if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}let pointer=slot.load(Ordering::Acquire);if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let step=match unsafe{((*pointer).pump)(pointer,cx)}{Ok(step)=>step,Err(failure)=>{slot.store(pointer,Ordering::Release);WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);return Err(failure)}};
        if matches!(step,WorkerJobCloseStep::Complete{..}){slot.store(std::ptr::null_mut(),Ordering::Release);unsafe{((*pointer).destroy)(pointer)}}else{slot.store(pointer,Ordering::Release)}
        WORKER_JOB_RETIREMENT_WAKE.store(worker_job_retirements_are_parked(),Ordering::Release);return receive_original_worker_close_effects(cx,before,match step{WorkerJobCloseStep::Complete{progress}if worker_job_retirements_are_parked()=>WorkerJobCloseStep::Pending{progress},step=>step})
    }
    Ok(if worker_job_retirements_are_parked(){WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);WorkerJobCloseStep::Blocked}else{WorkerJobCloseStep::Complete{progress:Default::default()}})
}

#[repr(C)]
pub(super) struct WorkerJobRetirementNode<J> {
    pub(super) header: WorkerJobRetirementHeader,
    pub(super) inner: Option<session_return::SessionHandle<J>>,
}

pub(super) fn worker_authority_phase(phase:OriginalWorkerAuthorityPhase)->WorkerJobClosePhase{match phase{OriginalWorkerAuthorityPhase::QuarantinedOutcome=>WorkerJobClosePhase::QuarantinedOutcome,OriginalWorkerAuthorityPhase::Outcome=>WorkerJobClosePhase::Outcome,OriginalWorkerAuthorityPhase::OriginalFailure=>WorkerJobClosePhase::OriginalFailure,OriginalWorkerAuthorityPhase::AdmissionPage|OriginalWorkerAuthorityPhase::Fault=>WorkerJobClosePhase::PreadmittedFault,OriginalWorkerAuthorityPhase::BeginClose=>WorkerJobClosePhase::BeginClose,OriginalWorkerAuthorityPhase::Job=>WorkerJobClosePhase::Job,OriginalWorkerAuthorityPhase::JobRelease=>WorkerJobClosePhase::JobRelease,OriginalWorkerAuthorityPhase::Parameters=>WorkerJobClosePhase::ParamsRelease,OriginalWorkerAuthorityPhase::Ledger=>WorkerJobClosePhase::PayloadLedger,OriginalWorkerAuthorityPhase::Empty=>WorkerJobClosePhase::AuthorityRelease}}

pub(super) unsafe fn worker_job_retirement_node_phase<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader)->WorkerJobClosePhase{
    let node=unsafe{&*pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{return WorkerJobClosePhase::RetirementSlot};
    let phase=inner.phase();
    if phase==SESSION_EMPTY{return WorkerJobClosePhase::SessionArc}
    if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return WorkerJobClosePhase::CheckedOut}
    let authority=unsafe{inner.take_authority()};
    let result=if phase!=SESSION_CLOSE{WorkerJobClosePhase::Open}else{worker_authority_phase(authority.close_phase())};
    unsafe{inner.put_authority(authority,phase)};
    result
}

pub(super) unsafe fn pump_worker_job_retirement_node<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
    let before=cx.retained_progress();let grant=cx.retained_grant();let node=unsafe{&mut *pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{let released_bytes=size_of::<WorkerJobRetirementNode<J>>();return Ok(if grant.maximum_items>0&&grant.maximum_release_bytes>=released_bytes&&grant.maximum_depth>0{WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}}else{WorkerJobCloseStep::Pending{progress:Default::default()}})};
    if matches!(worker_job_begin_close(inner),WorkerJobCloseStep::Blocked){return Ok(WorkerJobCloseStep::Blocked)}
    let step=match worker_job_close_step(inner,cx)?{
        WorkerJobCloseStep::Complete{progress}if progress!=Default::default()=>WorkerJobCloseStep::Pending{progress},
        WorkerJobCloseStep::Complete{..}=>{
            let grant=cx.retained_grant();if !inner.is_unique()||unsafe{(&*inner.waker.get()).is_some()}{return Ok(WorkerJobCloseStep::Blocked)}
            match worker_wake_retirement::close_session_retained_wake(inner,grant){Ok(Some(step))=>return receive_original_worker_close_effects(cx,before,step),Ok(None)=>{},Err(error)=>return receive_original_worker_close_result(cx,before,Err(error))}
            let released_bytes=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<WorkerJobSessionInner<J>>()).expect("worker original Arc layout").0.pad_to_align().size();
            if grant.maximum_items==0||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            match node.inner.take().unwrap().try_return(){Ok(inner)=>{drop(inner);WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}},Err(inner)=>{node.inner=Some(inner);WorkerJobCloseStep::Blocked}}
        },step=>step
    };receive_original_worker_close_effects(cx,before,step)
}

pub(super) unsafe fn destroy_worker_job_retirement_node<J>(pointer: *mut WorkerJobRetirementHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<WorkerJobRetirementNode<J>>()) });
}
//#endregion 🏭️RetainedSessions
