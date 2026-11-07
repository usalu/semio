//! 🪆️ The SemioFlow owner declares typed reads and its native mutation-batch command without a vector service.
use super::SEMIO_FLOW_DIALECT;
use crate::schema::snapshot::SemioFlowSnapshot;
use semio_plugin::app::{child_read::{ChildReadRegistration,ChildReadCapability},child_editor::{registration::ChildEditorRegistration,source::ChildEditorCapability}};
pub const SEMIO_FLOW_APPLY_MUTATIONS_COMMAND_ID:&str="applyMutations";
pub const SEMIO_FLOW_CHILD_EDITOR_ID:&str="s.stdio.semio@v1/flow#editor";
pub static SEMIO_FLOW_CHILD_READ:ChildReadRegistration<SemioFlowSnapshot>=ChildReadRegistration::new(ChildReadCapability{artifact_kind:SEMIO_FLOW_DIALECT.artifact_kind,standard:SEMIO_FLOW_DIALECT.standard.0,subset:SEMIO_FLOW_DIALECT.subset.0});
/// 📨️ Original parent commands authorize their derived, unchanged native leaves through qualified batch admission.
pub static SEMIO_FLOW_CHILD_APPLY_MUTATIONS:ChildEditorRegistration<SemioFlowSnapshot>=ChildEditorRegistration::new(ChildEditorCapability{artifact_kind:SEMIO_FLOW_DIALECT.artifact_kind,standard:SEMIO_FLOW_DIALECT.standard.0,subset:SEMIO_FLOW_DIALECT.subset.0,editor_id:SEMIO_FLOW_CHILD_EDITOR_ID,command_id:SEMIO_FLOW_APPLY_MUTATIONS_COMMAND_ID});

/// 📐️ Quotes the canonical native batch command before it transfers any original mutation owner.
pub fn semio_flow_mutation_batch_demand(slot_bytes:usize,child_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_plugin::app::ChildEmitPreparation::owned_qualified_batch_birth_demand::<crate::schema::mutations::SemioFlowMutation>(slot_bytes,child_bytes)}
/// 🔐️ The defining editor accepts only its own issued command capability with the unchanged original batch.
pub fn admit_semio_flow_mutation_batch(slot:&str,child_id:&str,operations:&mut Option<Vec<crate::schema::mutations::SemioFlowMutation>>,authority:&mut Option<semio_plugin::app::child_read::command::OriginalChildEditorCommandAuthority>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(semio_plugin::app::ChildEmitPreparation,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{
 let demand=semio_flow_mutation_batch_demand(slot.len(),child_id.len())?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(None)}
 if authority.as_ref().and_then(|authority|authority.stamp()).is_none_or(|stamp|stamp.capability_identity()!=&SEMIO_FLOW_CHILD_APPLY_MUTATIONS.identity as*const _ as usize){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"SemioFlow batch requires its original registered native command authority"))}
 semio_plugin::app::ChildEmitPreparation::admit_owned_batch_qualified::<SemioFlowSnapshot,crate::schema::mutations::SemioFlowMutation>(slot,child_id,operations,authority,grant)
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
