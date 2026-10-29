//! 🏃️ Staged `runtime-command` members of the `instance-dispatch` use cases: a command the runtime itself originates is produced, admitted and dispatched through the original typed source across funded turns.
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::ArtifactApp;
use crate::dispatch_bundle::DispatchCtx;
use semio_framework_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource;
use semio_framework_plugin_job::original_declared_tools::DeclaredToolCatalogAlias;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::InvocationResult;
use store::{MemberFactory, ReferenceSpaceMember};
use semio_framework_value::{ValueError, ValueRefusalKind};

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 🏃️ One funded turn of a command the runtime originates (host events, resumed tasks, laws): `make` births the typed command into the producer once,
    /// the producer encodes its exact OpBinary wire, the declared tool catalog admits that wire, and the shared typed pipeline mounts the operation.
    /// `Some` answers the emission receipt of the mounted operation; `None` asks for another turn.
    pub async fn drive_runtime_typed_command(&mut self, source: &mut OriginalTypedCommandSource<A>, make: impl FnOnce() -> Option<Box<A::Command>>, tool_tag: &mut Option<semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag>, metadata: &mut Option<ActionMeta>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if source.input_ready() || source.receipt.is_some() || source.metadata.is_some() { return self.dispatch_typed_command_inner(source, tool_tag, metadata, identity, cx).await }
        if !source.admit(cx)? { return Ok(None) }
        if !source.command_ready() && !source.produce_runtime(make, cx)? { return Ok(None) }
        let command = source.command.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "runtime command admission requires its produced command"))?;
        let verb = A::command_id(command).await;
        let catalog = self.tools.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::source).ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "runtime command requires its original committed tool catalog"))?;
        if !source.admit_wire(catalog, verb, cx)? { return Ok(None) }
        self.media.require_complete_tool_operation_pipeline(source.admission.as_ref().expect("admitted wire leaves its receipt in the source"))?;
        self.dispatch_typed_command_inner(source, tool_tag, metadata, identity, cx).await
    }
}
