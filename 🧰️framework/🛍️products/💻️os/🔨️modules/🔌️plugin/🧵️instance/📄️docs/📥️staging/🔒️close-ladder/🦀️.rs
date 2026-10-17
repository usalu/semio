//! 🪂️ Staged slice of the `close-ladder` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use crate::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleStep, plugin_grant_funds};
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

impl<A: ArtifactApp> MediaTransfers<A> {
    pub fn close_segment_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ArtifactDownloadOutput>()+size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadRetirement>(),depth:1,..Default::default()}}

    pub fn close_segment_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PluginLifecycleFailure>{
        let Some((_,id))=self.segmented_downloads.next_id_from(0)else{return Ok(RetainedCloneStep::Progress(Default::default()))};let demand=self.close_segment_demands();if !semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.segmented_closures.can_insert(id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"segmented close receiving slot is occupied").into())}
        let output=self.segmented_downloads.remove(id).expect("quoted original segmented owner");self.segmented_closures.insert_admitted(id,semio_framework_plugin_document_media::media_export::MediaDownloadRetirement::pending(output));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }

    pub fn close_segment_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let Some((_,id))=self.segmented_closures.next_id_from(0)else{return Ok(Default::default())};let original=self.segmented_closures.get(id).expect("retained segmented close owner");if original.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadRetirement>(),depth:1,..Default::default()})}else{original.retirement_demands(body)}}

    pub fn close_segment_retirement_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PluginLifecycleFailure>{
        let Some((_,id))=self.segmented_closures.next_id_from(0)else{return Ok(RetainedCloneStep::Progress(Default::default()))};if let Some(source)=self.snapshot_retirements.get(id){source.validate_retirement(cx)?}
        let original=self.segmented_closures.get_mut(id).expect("retained segmented close owner");if original.terminal_is_empty(){let demand=RetirementDemand{copy_bytes:size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadRetirement>(),depth:1,..Default::default()};if !semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.segmented_closures.remove(id);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}original.close_step(cx,grant).map(ToolOperations::<A>::lifecycle_retained)
    }

    pub fn close_media_step(&mut self, tools: &mut ToolOperations<A>, closures: bool, cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<RetainedCloneStep,PluginLifecycleFailure> {
        let registry = if closures { &mut self.media_closures } else { &mut self.media_exports };
        let Some((_, id)) = registry.next_id_from(0) else { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())) };
        let active = registry.get_mut(id).ok_or_else(|| plugin_sdk_fault("media close authority changed during one fixed step"))?;
        let progress = match active.close_step(cx,grant,&tools.tool_cancellations)? {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => progress,
            PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),
        };
        if !active.terminal_is_empty()? { return Ok(RetainedCloneStep::Progress(progress)); }
        let registry = if closures { &self.media_closures } else { &self.media_exports };
        registry.get(id).ok_or_else(|| plugin_sdk_fault("completed media close lost its owner"))?;
        if self.snapshot_retirements.get(id).is_none(){return Err(plugin_sdk_fault("media export lost its original snapshot retirement owner").into())}
        let registry = if closures { &mut self.media_closures } else { &mut self.media_exports };
        drop(registry.remove(id).ok_or_else(|| plugin_sdk_fault("terminal media owner changed before exact removal"))?);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }
}
