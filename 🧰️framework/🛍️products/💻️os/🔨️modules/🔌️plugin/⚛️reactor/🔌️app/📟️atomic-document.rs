//! 📟️ Funds the actor's atomic document transport without taking its Store publication authority.

use semio_framework::FaultFrom;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementPollExit};
use semio_os_kernel::os_spr::{ColdMutation, MergeReport, StoreMutationFor, StoreState};
use semio_os_kernel::os_store::{ArtifactStore, AtomicStoreReceive};
use semio_os_kernel::os_store::backbone::atomic_transport::{AtomicBackboneInbox, AtomicInboxStep};
use semio_os_kernel::os_vcs::VcsError;

/// 📥️ Drives a bounded slice and returns one completely authenticated edit to its caller.
fn advance_atomic_inbox(
    inbox: &mut AtomicBackboneInbox, original: &mut Option<Vec<u8>>, document: &str, binding: [u8; 32],
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    if inbox.cleanup_due() {
        cx.with_original_retirement(|cx| {
            for _ in 0..64 {
                if !inbox.cleanup_due() || cx.retirement_should_yield() { break; }
                let grant = cx.retained_grant();
                let Some((poll, performed)) = cx.with_original_effect(grant, |_, effect| inbox.close(effect))? else { break; };
                cx.consume_work(performed);
                let receipt = match RetirementPollExit::resume(poll) {
                    Ok(step) => step.progress(),
                    Err(RetirementPollExit::Pending(_)) => break,
                    Err(RetirementPollExit::Failed(failure)) => return Err(failure.into_value_error().into()),
                };
                cx.consume_retained(receipt)?;
                if receipt == Default::default() { break; }
            }
            Ok(())
        })?;
        return Ok(());
    }
    for _ in 0..64 {
        if cx.is_cancelled() || cx.should_yield() { break; }
        let grant = cx.retained_grant();
        let Some((result, performed)) = cx.with_original_effect(grant, |_, effect| {
            if original.is_some() {
                inbox.take_hot(original, effect).map(|receipt| (AtomicInboxStep::Runnable, receipt))
            } else {
                inbox.advance(&mut None, binding, document, effect)
            }
        })? else { break; };
        cx.consume_work(performed);
        let (state, receipt) = result.map_err(|error| error.into_fault())?;
        cx.consume_retained(receipt)?;
        match state {
            AtomicInboxStep::Waiting => break,
            AtomicInboxStep::Runnable if receipt == Default::default() => break,
            AtomicInboxStep::Runnable => {}
            AtomicInboxStep::Ready => break,
        }
    }
    Ok(())
}

/// 📬️ Folds the bound original group through the same atomic Store frontier as a retained replica.
pub async fn fold_atomic_inbox<P, M>(
    store: &mut ArtifactStore<P, M>, inbox: &mut AtomicBackboneInbox, original: &mut Option<Vec<u8>>,
    binding: [u8; 32], cx: &mut StepContext<'_>,
) -> Result<Option<MergeReport>, OriginalTurnFailure>
where
    P: StoreState + Clone,
    M: StoreMutationFor<P> + ColdMutation<P>,
{
    advance_atomic_inbox(inbox, original, store.envelope().id.as_str(), binding, cx)?;
    let Some(owned) = inbox.ready() else { return Ok(None); };
    let merged = store.ingest_original_atomic(owned).await;
    inbox.accept_ready();
    match merged.map_err(|error| error.into_fault())? {
        AtomicStoreReceive::Finished(report) => Ok(Some(report)),
        AtomicStoreReceive::AwaitingDependencies => Err(VcsError::Backbone("atomic document delivery is ahead of its causal frontier".into()).into_fault().into()),
    }
}

/// 📤️ Reports only the source-funded work actually performed by the attached Store sender.
pub fn drive_document_outbox(
    cx: &mut StepContext<'_>,
    mut advance: impl FnMut(&mut RetainedAllocationEffect<'_>) -> Result<Option<RetainedCloneProgress>, VcsError>,
) -> Result<(), OriginalTurnFailure> {
    for _ in 0..64 {
        if cx.should_yield() { break; }
        let grant = cx.retained_grant();
        let Some((result, performed)) = cx.with_original_effect(grant, |_, effect| advance(effect))? else { break; };
        cx.consume_work(performed);
        let Some(receipt) = result.map_err(|error| error.into_fault())? else { break; };
        cx.consume_retained(receipt)?;
        if receipt == Default::default() { break; }
    }
    Ok(())
}
