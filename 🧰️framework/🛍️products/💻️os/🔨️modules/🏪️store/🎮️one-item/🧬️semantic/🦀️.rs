//! 🧬️ Bounded semantic edit capabilities shared by publication and replay producers.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::post::OriginalPostRead;
pub use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
pub enum PagedOneItemEditStep<M> {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress, M),
    CompleteRows(RetainedCloneProgress, Vec<M>),
    CompleteOutcome(RetainedCloneProgress, Vec<M>, Vec<semio_framework_os_spr::MutationMessage>),
    RefusedOutcome(RetainedCloneProgress, Vec<semio_framework_os_spr::MutationMessage>),
}

/// 🧭️ One retained semantic unit prepares a real typed diff over original immutable sources; the shared central applier commits its candidate.
pub trait PagedOneItemEdit<S, M>: Default + Send + 'static {
    const PREFIX: &'static str;
    fn recognizes(mutation: &M) -> bool;
    fn preflight(mutation: &M) -> Result<usize, String>;
    fn post_mode(&self)->PagedOneItemPostMode;
    fn advance_demands(&self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:PagedOneItemPostRead<'_,S>,body:usize)->Result<RetirementDemand,ValueError>;
    fn advance(&mut self, before: RetainedCloneRef<'_, S>, mutation: RetainedCloneRef<'_, M>, post: PagedOneItemPost<'_,S>, body:usize,grant: RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<M>, ValueError>;
    fn begin_close(&mut self);
    fn close_demands(&self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

