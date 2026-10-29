//! 🤝️ The domain-neutral seam between a retained store initializer and the job that drives it.

use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_spr::Mutation;
use semio_framework_value::{ToValue, FromValue};
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::{RetirementDemand, ValueError};

/// 🏗️ Domain-owned persistent initializer for one completed envelope. A terminal
/// success retains exactly one fully prepared store until the wrapper takes it; fault/cancel may
/// become terminal only after the envelope and every partial owner have been cursor-retired.
///
/// 🔁️ The framework-default implementation is [`super::ReferenceArtifactStoreReplayInitializer`]; an app implements this trait
/// only when its document initialization is more than a ledger replay.
pub trait ArtifactStoreInitializationAuthority<P, M>: Send
where
    P: semio_framework_os_spr::StoreState,
    M: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// ▶️ One bounded initializer turn under the job's retained wallet; `Ok(None)` means the turn could not afford an outcome.
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>;
    /// 🤝️ Resolves the semantic descriptor of an outcome this authority lent on `step`.
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError>;
    /// 📈️ `(operations folded, operations to fold)` of the history this initializer replays; `(0, 0)` when it counts
    /// none.
    fn progress(&self) -> (u64, u64) {
        (0, 0)
    }
    fn request_cancel(&mut self);
    fn take_candidate(&mut self) -> Option<ArtifactStore<P, M>>;
    #[inline(never)]
    fn take_boxed_candidate(&mut self) -> Option<Box<ArtifactStore<P, M>>> {
        self.take_candidate().map(Box::new)
    }
    /// 📐️ Exact indivisible physical extent owed by the next retained close action.
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError>;
    fn begin_close(&mut self);
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>;
    fn terminal_is_empty(&self) -> bool;
}
