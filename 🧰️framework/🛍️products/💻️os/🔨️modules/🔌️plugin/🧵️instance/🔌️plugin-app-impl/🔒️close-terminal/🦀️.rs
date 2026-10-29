//! 🔒️ Close terminal of the mounted artifact instance: the conjunction that says every owned store, disposer, lease, reservation and ladder of the instance has drained. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(crate) fn original_typed_tag_pending(&self)->bool{self.original_typed_tag.is_some()||!self.original_typed_tag_close.is_empty()}

    pub(crate) fn original_typed_tag_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if !self.original_typed_tag_close.is_empty(){self.original_typed_tag_close.demands(body)}else{self.original_typed_tag_close.next_owner_demands::<semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag>(true,&None,body)}
    }

    pub(crate) fn close_original_typed_tag(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_value::RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        if !self.original_typed_command.original_binding()?.matches(cx)||Some(self.original_typed_command.original_decoder()?)!=cx.native_receiving_identity(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"direct tool tag requires its original command receiving source").into())}
        let body=self.original_typed_command.original_body()?;let d=self.original_typed_tag_close_demands(body)?;let mut output=semio_framework_value::RetainedCloneStep::Progress(Default::default());
        semio_framework_job::close_original_retained_step(cx,d,|grant|{output=if !self.original_typed_tag_close.is_empty(){self.original_typed_tag_close.select(body)?.step(grant)?}else{self.original_typed_tag_close.begin_granted(&mut self.original_typed_tag,grant)?.unwrap_or(semio_framework_value::RetainedCloneStep::Progress(Default::default()))};Ok(output)})?;Ok(output)
    }

    pub(super) fn close_terminal_is_empty(&self) -> bool {
        self.constructor.constructor_terminal_is_empty()
            && self.close.close_started
            && self.history.pending_command_prune.is_none()
            && !self.history.command_prune_requested()
            && self.history.command_log.pruned_terminal_is_empty()
            && InstanceExtensions::terminal_is_empty(&self.time_travel, &self.tool_runs)
            && self.stores.store.close_owned_store_terminal_is_empty()
            && self.stores.config_store.close_owned_store_terminal_is_empty()
            && self.stores.draft_store.close_owned_store_terminal_is_empty()
            && self.stores.interaction_store.close_owned_store_terminal_is_empty()
            && self.close.close_presence_disposer.is_none()
            && self.close.close_transient_disposer.is_none()
            && self.stores.window_config_store.terminal_is_empty()
            && self.stores.window_transient_store.terminal_is_empty()
            && self.stores.retired_window_transient_stores.is_empty()
            && self.stores.retired_window_transient_stores.empty_backing_byte_demand()==Some(0)
            && self.close.close_interaction_disposer.is_none()
            && self.tools.tool_cancellations.terminal_is_empty()
            && self.pending.pending_document_window_admission.is_none()
            && self.tools.pending_tool_lease_admission.is_none()
            && self.media.pending_media_import_permit.is_none()
            && self.tools.tool_operations.is_empty()
            && self.original_typing.terminal_is_empty()
            && !self.original_emission.pending()
            && !self.original_typed_command.pending()
            && self.original_typed_tag.is_none()
            && self.original_typed_tag_close.is_empty()
            && self.tools.original_emission_admission.is_none()
            && self.tools.original_emission_receipt.is_none()
            && self.child_composition.private_child_groups_terminal_is_empty()
            && self.tools.latest_wins_commands.is_empty()
            && self.tools.typed_operation_reservations.iter().all(Option::is_none)
            && self.tools.latest_wins_order.len() == 0
            && self.tools.latest_wins_keys.terminal_is_empty()
            && self.close.close_instance_operation_owner_drained
            && self.tools.instance_operation_owner.close_terminal_is_empty().unwrap_or(false)
            && self.pending.render_failure_terminal_is_empty()
            && self.tools.agent_lane_preview.is_none()
            && self.media.media_submission_refusal_terminal_is_empty()
            && self.ingress.action_ingress_sources_closed()
            && self.media.media_exports.is_empty()
            && self.pending.pending_reserved.is_empty()
            && self.pending.reserved_commits.is_empty()
            && self.pending.reserved_commit_outcome.is_none()
            && self.media.media_closures.is_empty()
            && self.media.snapshot_retirements.is_empty()
            && self.media.segmented_downloads.is_empty()
            && self.media.segmented_closures.is_empty()
            && self.child_composition.child_content_retirements.is_empty()
            && self.child_composition.child_content_retirements.empty_backing_byte_demand()==Some(0)
            && self.close.close_child_member.is_none()
            && self.child_composition.child_member_retirements.is_empty()
            && self.child_composition.child_member_retirements.empty_backing_byte_demand()==Some(0)
            && self.child_composition.children.is_empty()
            && self.roster.peer_roster_publications.is_empty()
            && self.roster.peer_roster_outcomes.is_empty()
            && self.roster.peer_presence_retirements.is_empty()
            && self.roster.presence_peer_retirements.is_empty()
            && self.ingest.envelope_ingress.is_empty()
            && self.ingest.envelope_decode_jobs.is_empty()
            && self.ingest.store_replacement_jobs.is_empty()
            && self.ingest.document_archive_loads.is_empty()
            && self.ingest.document_archive_loads.empty_backing_byte_demand()==Some(0)
            && self.ingest.envelope_field_decoder_retirements.is_empty()
            && self.ingest.envelope_completed_record_retirements.is_empty()
            && self.ladder_ref().retained_fields_terminal_is_empty()
            && self.original_controller_close.as_ref().is_none_or(|owner|(owner.terminal)(&self.app))
    }
}
