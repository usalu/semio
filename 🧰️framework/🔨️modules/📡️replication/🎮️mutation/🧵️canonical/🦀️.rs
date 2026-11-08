//! 🪪️ Semantic canonical authority and typed identities retain their original domain owners.
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::ArtifactCanonicalEditIdentityCursor;
#[path="🛂️authority/🦀️.rs"]
mod authority;
pub use authority::{ArtifactCanonicalEditAuthority,ArtifactCanonicalEditAuthorityCursor};
use super::{Edit,MutationMeta};

semio_framework_value::artifact_retire_struct!(MutationMeta{mutation_id,dependencies,base_version,author_id,timestamp,undo_policy,payload_hash,semantic_kind,label,group_id,origin,transaction});

/// ⏱️ Records the work a reported total still owes the shared effect, keeping every refusal and Pending child.
pub(crate) fn settle_exit(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,base:u128,result:Result<semio_framework_value::RetainedCloneStep,semio_framework_value::RetirementPollExit>,work:u128)->semio_framework_value::OriginalRetirementPoll{
    use semio_framework_value::{OriginalRetirementFailure,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,ValueError,ValueRefusalKind};
    let done=effect.performed_work().saturating_sub(base);
    let settlement:Result<(),ValueError>=if done>work{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical cursor recorded more work than it reports"))}else{u64::try_from(work-done).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"canonical cursor work does not fit one original unit")).and_then(|owed|if owed==0||effect.performed(owed)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"canonical cursor work exceeded its original action"))})};
    let complete=|result|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
    match(result,settlement){
        (Ok(step),Ok(()))=>complete(Ok(step)),
        (Ok(step),Err(error))=>complete(Err(OriginalRetirementFailure::from_work_refusal(error.with_retained_progress(step.progress())))),
        (Err(RetirementPollExit::Failed(failure)),Ok(()))=>complete(Err(failure)),
        (Err(RetirementPollExit::Failed(failure)),Err(error))=>complete(Err(OriginalRetirementFailure::from_owned_parts(None,Some(error),None,None,Some(Box::new(failure)),None))),
        (Err(RetirementPollExit::Pending(child)),Ok(()))=>RetirementPollExit::finish::<semio_framework_value::RetainedCloneStep>(Err(RetirementPollExit::Pending(child)),work),
        (Err(RetirementPollExit::Pending(_)),Err(error))=>complete(Err(OriginalRetirementFailure::from_work_refusal(error))),
    }
}
