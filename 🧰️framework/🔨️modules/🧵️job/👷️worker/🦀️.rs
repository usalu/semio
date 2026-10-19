//! 👷️ Canonical retained worker-session ownership and its private nominal custody.
use semio_framework_job::*;
use semio_framework_job_worker_authority::{OriginalWorkerJobAuthority,OriginalWorkerAuthorityPhase,WorkerJobSource,WorkerJobAdmissionControl,WorkerJobAdmissionContext,WorkerJobInitializer,WorkerJobInitializationStep,BatchDriveConfig,BatchJobParams,WorkerJobCloseStep,receive_original_worker_close_effects,receive_original_worker_close_result};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool,AtomicPtr,AtomicU8,AtomicU32,AtomicU64,AtomicUsize,Ordering};
use std::sync::Arc;
use std::task::Waker;
#[cfg(test)]
macro_rules! admit_original_fixture_owner {
 ($caller:ident,$owner:ident,$job:expr,$params:expr $(,)?)=>{{
  let mut original_job=Some($job);let mut original_params=Some($params);let grant=$caller.remaining_grant();
   $caller.turn(grant,|cx|{let before=cx.retained_progress();match $owner::try_admit_owned(&mut original_job,&mut original_params,cx)?{Some((owner,progress))=>{assert!(original_job.is_none()&&original_params.is_none());assert_eq!(before.checked_add(progress).unwrap(),cx.retained_progress());Ok(owner)},None=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture source retains its unchanged admission sources").into())}})
 }};
}
#[cfg(test)]
const WORKER_TEST_STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:20000,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:128};

fn turn(demand:RetirementDemand,grant:RetainedCloneGrant,owner:impl FnOnce(RetainedCloneGrant)->Result<(RetainedCloneStep,bool),ValueError>)->Result<RetainedCloneStep,ValueError>{semio_framework_value::advance_retirement_turn(demand,grant,owner).map_err(|error|match error{semio_framework_value::RetirementTurnError::Owner(error)|semio_framework_value::RetirementTurnError::Receipt(error)=>error})}

#[path="../♻️session-return/🦀️.rs"]
mod session_return;

#[path="🚪️admission/🦀️.rs"]
mod original_worker_admission;
pub use original_worker_admission::{WorkerJobAdmission,WorkerJobSessionPreparation};
#[path="🔀️erasure/🦀️.rs"]
mod original_session_erasure;
pub use original_session_erasure::{OriginalBatchSessionErasure,ErasedBatchJobSession,OriginalPreviewSession};

#[path="../♻️retirement/🔔️wake/🦀️.rs"]
mod worker_wake_retirement;
pub use worker_wake_retirement::{RetainedWorkerWake,OriginalWorkerWakeIssuer,admit_original_worker_wake,original_worker_wake_admission_demands};
#[cfg(not(target_arch="wasm32"))]
pub use worker_wake_retirement::admit_original_thread_worker_wake;

#[path = "🎫️worker-job-vocabulary/🦀️.rs"]
mod worker_job_vocabulary;
pub use worker_job_vocabulary::{BatchJobSession, MountedWorkerJobPumpFault, MountedWorkerJobSession, WorkerJobClosePhase, WorkerJobContention, WorkerJobDemandError, WorkerJobPoll, WorkerJobSubmitFault, WorkerJobTakeFault, WorkerJobTicket};

#[path = "🧰️mounted-and-batch-sessions/🦀️.rs"]
mod mounted_and_batch_sessions;

#[path = "🕳️retirement-slots-and-nodes/🦀️.rs"]
mod retirement_slots_and_nodes;
use retirement_slots_and_nodes::{SESSION_CHECKED_OUT, SESSION_CLOSE, SESSION_EMPTY, SESSION_IDLE, SESSION_OUTCOME, SESSION_REJECTED, SESSION_SUBMITTED, SESSION_TERMINAL, SESSION_TRANSITION, WORKER_JOB_RETIREMENT_SLOTS, WORKER_JOB_RETIREMENT_WAKE, WorkerJobRetirementHeader, WorkerJobRetirementNode, destroy_worker_job_retirement_node, pump_worker_job_retirement_node, reserve_worker_job_retirement_slot, worker_authority_phase, worker_job_retirement_node_phase};
pub use retirement_slots_and_nodes::{WORKER_JOB_SESSION_SLOTS, pump_worker_job_retirements, take_worker_job_retirement_wake, worker_job_retirement_phase, worker_job_retirements_are_parked};

#[path = "🛂️session-inner-and-admission/🦀️.rs"]
mod session_inner_and_admission;
use session_inner_and_admission::{WorkerJobSessionArc, WorkerJobSessionInner, WorkerJobSubmission};
pub use session_inner_and_admission::{WorkerJobSession, WorkerJobSessionAdmissionRejected};

#[path = "🧵️worker-job-session/🦀️.rs"]
mod worker_job_session;
use worker_job_session::{worker_job_begin_close, worker_job_close_step};

#[path = "🏁️outcome-and-rejection/🦀️.rs"]
mod outcome_and_rejection;
use outcome_and_rejection::{worker_rejection_code, worker_rejection_kind};
pub use outcome_and_rejection::{WorkerJobOutcome, WorkerJobRejected};

/// 🎟️ The [`WORKER_JOB_SESSION_SLOTS`] are process-wide, so one test binary's parallel tests share
/// them. A test that admits sessions holds a shared guard; the one test that owns every slot at
/// once holds the exclusive guard, so neither ever observes the other's admissions.
#[cfg(test)]
static WORKER_SESSION_SLOTS_TEST_AUTHORITY: std::sync::RwLock<()> = std::sync::RwLock::new(());

#[cfg(test)]
fn worker_session_slots_shared() -> std::sync::RwLockReadGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.read().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
fn worker_session_slots_exclusive() -> std::sync::RwLockWriteGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.write().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
mod original_worker_fixture;
#[cfg(any(test,feature="testkit"))]
pub use original_worker_fixture::{OriginalFixtureSession,OriginalWorkerFixtureCaller};

#[cfg(test)]
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
#[cfg(test)]
#[global_allocator]
static JOB_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[cfg(test)]
#[path="🧪️tests/🔬️retained-ownership/🦀️.rs"]
mod retained_ownership_tests;

#[cfg(test)]
#[path="🧪️tests/⛽️staged-projection/🦀️.rs"]
mod original_staged_projection_tests;
#[cfg(test)]
#[path="⏱️budget/🧪️tests/🦀️.rs"]
mod microsecond_budget_tests;

#[cfg(test)]
#[path="../♻️retirement/👷️authority/🧪️tests/🦀️.rs"]
mod worker_authority_retirement_tests;

#[cfg(test)]
#[path="🧪️tests/📏️close-demand/🦀️.rs"]
mod close_demand_tests;
