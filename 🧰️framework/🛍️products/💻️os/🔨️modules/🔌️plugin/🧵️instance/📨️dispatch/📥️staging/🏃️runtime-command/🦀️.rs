//! 🏃️ Staged `runtime-command` members of the `instance-dispatch` use cases: a command the runtime itself originates is produced, admitted and dispatched through the original typed source across funded turns.
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_app::ArtifactApp;
use crate::dispatch_bundle::DispatchCtx;
use semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource;
use semio_plugin_job::original_declared_tools::DeclaredToolCatalogAlias;
use semio_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::InvocationResult;
use store::{MemberFactory, ReferenceSpaceMember};
use semio_framework_value::{ValueError, ValueRefusalKind};
#[path="close.rs"]
mod close;
pub use close::{close_original_typed_source,original_typed_source_close_demands};

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 📣️ The host event declares its own command birth before entering the shared admitted wire pipeline.
    pub async fn prepare_host_event_typed_command(&self,source:&mut OriginalTypedCommandSource<A>,event:&semio_plugin_decl_emission::HostEvent,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if source.input_ready()||source.receipt.is_some()||source.metadata.is_some(){return Ok(Some(true))}if A::host_event_demands(event).is_none(){return Ok(Some(false))}if !source.admit(cx)?{return Ok(None)}match source.produce_host_event(event,cx)?{None=>return Ok(None),Some(false)=>return Ok(Some(false)),Some(true)=>{}}let command=source.command.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host event command disappeared"))?;let verb=A::command_id(command).await;let catalog=self.tools.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::source).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"host event requires its actual registered catalog"))?;if !source.admit_wire(catalog,verb,cx)?{return Ok(None)}self.media.require_complete_tool_operation_pipeline(source.admission.as_ref().expect("admitted host event wire owns its receipt"))?;Ok(Some(true))
    }
    /// 🏃️ One funded turn of a command the runtime originates (host events, resumed tasks, laws): `make` births the typed command into the producer once,
    /// the producer encodes its exact OpBinary wire, the declared tool catalog admits that wire, and the shared typed pipeline mounts the operation.
    /// `Some` answers the emission receipt of the mounted operation; `None` asks for another turn.
    pub async fn drive_runtime_typed_command(&mut self, source: &mut OriginalTypedCommandSource<A>, make: impl FnOnce() -> Option<Box<A::Command>>, tool_tag: &mut Option<semio_plugin_tool_machine::ToolTag>, metadata: &mut Option<ActionMeta>, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if source.input_ready() || source.receipt.is_some() || source.metadata.is_some() { return self.dispatch_typed_command_inner(source, tool_tag, metadata, identity, cx).await }
        if !self.prepare_runtime_typed_command(source,make,cx).await?{return Ok(None)}
        self.dispatch_typed_command_inner(source, tool_tag, metadata, identity, cx).await
    }
    /// 🎟️ Preparation borrows only the declared catalog, allowing an independently owned event to remain in its defining slot.
    pub async fn prepare_runtime_typed_command(&self,source:&mut OriginalTypedCommandSource<A>,make:impl FnOnce()->Option<Box<A::Command>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if source.input_ready()||source.receipt.is_some()||source.metadata.is_some(){return Ok(true)}
        if !source.admit(cx)?{return Ok(false)}
        if !source.command_ready()&&!source.produce_runtime(make,cx)?{return Ok(false)}
        let command=source.command.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"runtime command admission requires its produced command"))?;
        let verb=A::command_id(command).await;
        let catalog=self.tools.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::source).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"runtime command requires its original committed tool catalog"))?;
        if !source.admit_wire(catalog,verb,cx)?{return Ok(false)}
        self.media.require_complete_tool_operation_pipeline(source.admission.as_ref().expect("admitted wire leaves its receipt in the source"))?;
        Ok(true)
    }
}
