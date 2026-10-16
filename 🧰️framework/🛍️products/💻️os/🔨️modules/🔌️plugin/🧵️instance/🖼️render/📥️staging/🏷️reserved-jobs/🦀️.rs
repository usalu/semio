//! 🎽️ Staged `reserved-jobs` members of the `instance-render` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🏷️ The inputs of the reserved action and media jobs.
use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderCtx<'ctx, A, M, X> {
    pub(super) async fn build_artifact_reserved_action_job(&mut self, action: &str, args: Option<&DslValue>, raw_wire: Vec<u8>, meta: &ActionMeta) -> Result<Option<(ArtifactReservedToolJob, ArtifactToolCompletion<A>)>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let parent_document_id = self.stores.store.envelope().id.clone();
        let canonical_base_revision = self.stores.store.content_revision();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tools.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.tools.qualified_tool_proof(&*self.stores, action)?.schema_id(),
            contract: self.tools.qualified_tool_proof(&*self.stores, action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id,
            canonical_base_revision,
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_composition.child_content_root),
            raw_wire,
            input: ArtifactReservedToolInput::Action { args: args.cloned(), interaction: self.interaction.interaction_selection_snapshot(&*self.stores), hover: self.interaction.interaction_hover.clone() },
            completion: completion.clone(),
        };
        Ok(A::build_reserved_tool_job(request)?.map(|job| (job, completion)))
    }

    pub(super) async fn build_artifact_reserved_media_job(&mut self, port: &str, media: Media, meta: &ActionMeta) -> Result<(ArtifactReservedToolJob, ArtifactToolCompletion<A>), Fault> {
        let action = "import-media";
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tools.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.tools.qualified_tool_proof(&*self.stores, action)?.schema_id(),
            contract: self.tools.qualified_tool_proof(&*self.stores, action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id: self.stores.store.envelope().id.clone(),
            canonical_base_revision: self.stores.store.content_revision(),
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_composition.child_content_root),
            raw_wire: Vec::new(),
            input: ArtifactReservedToolInput::Media { port: port.to_string(), media },
            completion: completion.clone(),
        };
        let job = A::build_reserved_tool_job(request)?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-reserved-builder"), format!("media port '{port}' is registered but has no concrete resumable importer")))?;
        Ok((job, completion))
    }
}
