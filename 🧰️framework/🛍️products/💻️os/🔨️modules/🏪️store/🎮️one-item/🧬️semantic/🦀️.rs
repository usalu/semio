//! 🧬️ Bounded semantic edit capabilities shared by publication and replay producers.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::post::OriginalPostRead;
pub use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
pub enum PagedOneItemEditStep<M> {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress, M),
    CompleteRows(RetainedCloneProgress, Vec<M>),
    CompleteOutcome(RetainedCloneProgress, Vec<M>, Vec<semio_os_spr::MutationMessage>),
    RefusedOutcome(RetainedCloneProgress, Vec<semio_os_spr::MutationMessage>),
}

/// 🧭️ One retained semantic unit prepares a real typed diff over original immutable sources; the shared central applier commits its candidate.
pub trait PagedOneItemEdit<S, M>: Default + Send + 'static {
    const PREFIX: &'static str;
    fn recognizes(mutation: &M) -> bool;
    fn preflight(mutation: &M) -> Result<usize, String>;
    fn post_mode(&self)->PagedOneItemPostMode;
    fn advance_demands(&self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:PagedOneItemPostRead<'_,S>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
    /// 🎟️ Reserves and commits the entire call through the original receiving control, including partial failures.
    fn advance(&mut self, before: RetainedCloneRef<'_, S>, mutation: RetainedCloneRef<'_, M>, post: PagedOneItemPost<'_,S>, body:usize,grant: RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<M>, semio_framework_value::OriginalReceivingPrimary>;
    /// ⛽️ Entire latest advance work, including nested work and partial failures; denied calls reset it.
    fn last_turn_work(&self)->u64;
    /// 🧷️ Borrows only an actually pending original receiving callback.
    fn has_pending_receiving_recovery(&self)->bool;
    fn pending_receiving_demand(&self)->Option<RetirementDemand>;
    fn pending_receiving_recovery(&mut self)->Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>>;
    /// 🧾️ Finishes the same original receiving frame before owner retirement.
    fn finish_receiving_recovery(&mut self,control:&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>;
    fn begin_close(&mut self);
    fn close_demands(&self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_work_demand(&self, mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<u64,ValueError>;
    fn close_step(&mut self, mutations: &dyn ArtifactOwnedValueRetirementFactory<M>, grant: RetainedCloneGrant, actual:&mut Option<usize>) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure>;
    fn terminal_is_empty(&self) -> bool;
}

