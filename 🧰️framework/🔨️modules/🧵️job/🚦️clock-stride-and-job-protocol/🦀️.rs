//! 🚦️ See `README.md`.

use super::*;


pub use semio_framework_job_outcome::{Checkpoint,CommitCandidate,JobFault,StepOutcome};

//#region 🧭️InteractiveJob
pub use semio_framework_job_outcome::InteractiveJobCloseStep;

/// 🧵️ Whether a job may be handed to another thread. Every threaded target elaborates this to
/// `Send`, so `J: InteractiveJob` still proves `J: Send` for [`WorkerJobSessionInner`]'s
/// `unsafe impl` and every pool submission. The browser build (`wasm32`, not `wasip2`) has no
/// second thread to hand a job to at all — its jobs own `Rc<JsValue>` browser handles by design
/// (`🎯️targets/🧊️wgpu/🧵️frame-job`'s frame worker is the single JS worker thread) — so requiring
/// `Send` there would only forbid the one ownership model that target can have.
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
pub trait JobThreadTransfer: Send {}

#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
impl<T: Send + ?Sized> JobThreadTransfer for T {}

/// 💼️ See `README.md`.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait JobThreadTransfer {}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl<T: ?Sized> JobThreadTransfer for T {}

/// 🧩️ See `README.md`.
pub trait InteractiveJob: JobThreadTransfer {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,return_claim:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>;
    fn borrow_outcome<'a>(&'a self, descriptor:&'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError>;
    fn begin_close(&mut self);
    /// 🎟️ See `README.md`.
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
    /// 🧮️ See `README.md`.
    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 📦️ See `README.md`.
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes:usize) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// ♻️ See `README.md`.
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 📪️ See `README.md`.
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 🔎️ See `README.md`.
    fn close_demand_without_owner(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){Ok(0)}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"interactive job retained close demand is undeclared"))}}
    /// 🔔️ See `README.md`.
    fn register_close_wake(&self, _waker: &Waker) -> bool {
        false
    }
    fn terminal_is_empty(&self) -> bool;
}
//#endregion 🧩️InteractiveJob
