//! 🧻️ Staged slice of the `typed-operations` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🧾️ Typed operations of the mounted artifact instance: typed-operation publication, typed-command dispatch, configuration and the history panel.
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_command::command_admission::AdmittedToolCommand;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use crate::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_job::reserved_jobs::QualifiedToolProof;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> MediaTransfers<A> {
    pub fn require_tool_operation_authority(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        let contract = admission.proof.contract();
        let exact = admission.decoded_items <= contract.max_decoded_items
            && contract.max_work_units_per_step != 0
            && contract.max_output_bytes != 0
            && contract.max_step_micros != 0
            && contract.max_step_micros < semio_framework::ToolExecutionContract::INTERACTIVE_MAX_STEP_MICROS
            && matches!(contract.cancellation, semio_framework::ToolCancellationPolicy::PerOperation)
            && matches!(contract.freshness, semio_framework::ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure)
            && matches!(contract.shape, semio_framework::ToolExecutionShape::Resumable | semio_framework::ToolExecutionShape::BoundedFirstStep);
        if !exact {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.incomplete-operation-authority"), format!("typed command '{}' has no complete bounded prepare/reducer/commit authority", admission.verb)));
        }
        Ok(())
    }

    pub fn require_complete_tool_operation_pipeline(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        self.require_tool_operation_authority(admission)?;
        match admission.proof {
            QualifiedToolProof::AppOwned(_) => Ok(()),
            QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {
                Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{}' has no exact app-owned retained decoder/reducer factory", admission.verb)))
            }
        }
    }

    /// 🎰️ Whether every fixed typed-operation and segmented-output authority owns nothing in this one
    /// residue class of `id % ARTIFACT_LIVE_OUTPUT_SLOTS`.
    pub fn typed_operation_slot_is_vacant(&self, pending: &PendingReserved<A>, tools: &ToolOperations<A>, slot: usize) -> bool {
        tools.tool_operations.slot_is_vacant(slot)
            && tools.typed_operation_reservations[slot].is_none()
            && tools.latest_wins_commands.slot_is_vacant(slot)
            && pending.pending_reserved.slot_is_vacant(slot)
            && self.segmented_downloads.slot_is_vacant(slot)
            && self.segmented_closures.slot_is_vacant(slot)
    }

    /// 🎰️ How many of the fixed residue classes currently own a live typed operation or segmented
    /// output — the exact complement of what [`Self::admit_typed_operation_slot`] can still hand out,
    /// so `0` is the only value that proves every admitted slot reached its release path.
    pub fn live_typed_operation_slots(&self, pending: &PendingReserved<A>, tools: &ToolOperations<A>) -> usize {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter(|slot| !self.typed_operation_slot_is_vacant(pending, tools, *slot)).count()
    }

    /// 🎫️ Pre-admits the exact operation slot BEFORE an operation id exists. Every typed authority is a
    /// direct-mapped table addressed by `id % ARTIFACT_LIVE_OUTPUT_SLOTS`, so minting a shared-counter id
    /// first and then testing its one residue class refuses an action whenever an unrelated live
    /// operation happens to share that class — a hover/tick/camera storm then fails while 63 slots sit
    /// empty. Ingress therefore chooses the vacant slot and mints the id for it, and refusal now means
    /// actual saturation of all fixed slots.
    pub fn admit_typed_operation_slot(&self, pending: &PendingReserved<A>, tools: &ToolOperations<A>) -> Option<semio_framework_job::OperationId> {
        if !tools.latest_wins_order.allocation_admitted || tools.latest_wins_order.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            return None;
        }
        let slot = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.typed_operation_slot_is_vacant(pending, tools, *slot))?;
        Some(semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64))
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🔎️ Whether any live envelope decode still has worker close or step work this turn can run.
    pub(crate) fn has_runnable_artifact_envelope_decode(&self) -> bool {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| self.envelope_decode_jobs.entry(index).is_some_and(|(_, active)| active.has_runnable_work()))
    }

    pub fn has_runnable_artifact_envelope_decode_worker_step(&self) -> bool {
        self.has_runnable_artifact_envelope_decode()
    }
}
