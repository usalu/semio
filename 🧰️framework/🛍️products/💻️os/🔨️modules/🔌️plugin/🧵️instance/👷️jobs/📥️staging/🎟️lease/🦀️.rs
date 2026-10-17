//! 🫛️ Staged slice of the `lease` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_operation::ToolCancellationLease;
use semio_framework_ui_contract::*;

/// 🎟️ Original caller admission of one retained tool key before producer donation.
#[doc(hidden)]
pub struct OriginalToolLeaseAdmission {
    pub source:ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub verb:ManuallyDrop<Option<String>>,
    pub operation:Option<semio_framework_job::Operation>,
    pub lease:ManuallyDrop<Option<ToolCancellationLease>>,
    pub retirement:ManuallyDrop<Option<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>>,
    pub instance:u32,
    pub identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),
    pub keyed:bool,
    pub copied:bool,
    pub returning:bool,
}

impl Drop for OriginalToolLeaseAdmission {
    fn drop(&mut self){assert!(std::thread::panicking()||(self.source.is_none()&&self.verb.is_none()&&self.lease.is_none()&&self.retirement.is_none()),"original tool lease admission still retains its issuing source")}
}
