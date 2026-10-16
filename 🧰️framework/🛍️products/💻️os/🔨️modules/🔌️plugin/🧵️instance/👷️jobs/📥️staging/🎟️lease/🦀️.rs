//! 🫛️ Staged slice of the `lease` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
use super::*;

/// 🎟️ Original caller admission of one retained tool key before producer donation.
pub(crate) struct OriginalToolLeaseAdmission {
    pub(crate) source:ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub(crate) verb:ManuallyDrop<Option<String>>,
    pub(crate) operation:Option<semio_framework_job::Operation>,
    pub(crate) lease:ManuallyDrop<Option<ToolCancellationLease>>,
    pub(crate) retirement:ManuallyDrop<Option<crate::tool_cancellation::ToolCancellationLeaseRetirement>>,
    pub(crate) instance:u32,
    pub(crate) identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),
    pub(crate) keyed:bool,
    pub(crate) copied:bool,
    pub(crate) returning:bool,
}

impl Drop for OriginalToolLeaseAdmission {
    fn drop(&mut self){assert!(std::thread::panicking()||(self.source.is_none()&&self.verb.is_none()&&self.lease.is_none()&&self.retirement.is_none()),"original tool lease admission still retains its issuing source")}
}
