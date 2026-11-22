//! 📨️ Action dispatch of the mounted artifact instance: the invocation of one action address and the resumption of a task emit. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> MembersCtx<'ctx, A, M, X> {
    pub async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &mut Option<ActionMeta>, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {}", address.app_id, owner_app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id).or_else(|| (address.action_id == semio_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID && self.tools.registry.get(&address.action_id).is_none()).then_some(ActionKind::Mutation));
        if reserved_kind.is_none() {
            if !self.tools.registry.has_mode(&address.mode_id).await {
                return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
            }
            if active_mode_id != Some(address.mode_id.as_str()) {
                return Err((plugin_sdk_fault(format!("action {} belongs to inactive mode {}", address.action_id, address.mode_id))).into());
            }
        }
        let kind = match self.tools.registry.window_action(&address.window_kind_id, &address.action_id).await {
            Some(definition) => definition.kind,
            None => reserved_kind.ok_or_else(|| plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id)))?,
        };
        if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
        let log_generation_before = self.history.log_generation;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.as_command().dispatch_action(&address.action_id, source_args, original_fault, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        let Some(result) = result? else{return Ok(None)};
        Ok(Some(self.as_render().finish_recorded(log_generation_before, &address.action_id, result).await))
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes each lane's wire ops back into this
    /// app's concrete `A::Mutation`/`A::ConfigMutation`/`A::DraftMutation` via `OpBinary::
    /// decode_op` (the SAME decode idiom `dispatch_typed_command` uses for `A::Command`) and
    /// dispatches them directly — see the trait method's own doc for why this skips `handle`.
    pub async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        use semio_plugin_reactor_app::app_runtime_helpers::decode_task_emission_lane;
        let log_generation_before = self.history.log_generation;
        let artifact_mutations = decode_task_emission_lane::<A::Mutation>(&artifact_ops)?;
        let config_mutations = decode_task_emission_lane::<A::ConfigMutation>(&config_ops)?;
        let draft_mutations = decode_task_emission_lane::<A::DraftMutation>(&draft_ops)?;
        let result = self.as_dispatch().dispatch_emit("task-resume-emit", Emit { artifact_mutations, config_mutations, draft_mutations, ..Default::default() }, meta, identity).await?;
        Ok(self.as_render().finish_recorded(log_generation_before, "task-resume-emit", result).await)
    }
}
