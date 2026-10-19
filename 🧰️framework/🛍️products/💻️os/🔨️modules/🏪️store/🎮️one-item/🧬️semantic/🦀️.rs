//! 🧬️ Bounded semantic edit capabilities shared by publication and replay producers.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
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
    fn advance_demands(&self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:&S,body:usize)->Result<RetirementDemand,ValueError>;
    fn advance(&mut self, before: RetainedCloneRef<'_, S>, mutation: RetainedCloneRef<'_, M>, post: &mut S, body:usize,grant: RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<M>, ValueError>;
    fn begin_close(&mut self);
    fn close_demands(&self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

