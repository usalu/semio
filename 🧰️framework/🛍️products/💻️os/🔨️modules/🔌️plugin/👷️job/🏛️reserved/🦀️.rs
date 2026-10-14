//! 🏛️ Framework reserved jobs, their routes, commits and pumps.
use crate::tool_contract::ToolOwnerWitness;
use crate::tool_contract::ArtifactReservedJob;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::tool_contract::BOUNDED_FIRST_STEP_FACTORY;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use crate::window_kits::AppActionRegistry;
use crate::app_views::CANCEL_TYPED_OPERATION_ACTION_ID;
use crate::app_contract::is_time_travel_action_id;
use crate::tool_jobs::is_tool_run_action_id;
use crate::interaction_topology::INTERACTION_ACTION_IDS;
#[cfg(any(test, feature = "artifact-app-testing"))]
use super::window_transient::WindowTransientOwnerRegistry;
use semio_framework_value::DslValue;
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{kernel::InvocationResult, Fault, FaultCode, FaultFrom, FaultOrigin, CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, RECORD_TUTORIAL_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SELECT_ALL_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::ViewModel;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)]
use crate::artifact_instance::retire_document_window_registry_demands;
#[cfg(test)]
use crate::artifact_instance::retire_document_window_registry_step;
use crate::app_contract::ArtifactApp;
use crate::app_emission::ActionMeta;
use crate::child_view::MemberKey;
use crate::sdk_fault::plugin_sdk_fault;
use crate::tool_cancellation::ToolCancellationLease;
use crate::tool_contract::ArtifactBoundedFirstStepProof;
use crate::tool_contract::ArtifactBoundedToolFault;
use crate::tool_contract::ArtifactOwnedToolJobFactory;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::ArtifactToolPublicationContract;
use crate::tool_contract::ArtifactToolPublicationLane;
use crate::tool_contract::ArtifactToolRegistration;
use crate::typed_operation::TypedCommandFullOperationJobFactory;
#[cfg(test)]
use crate::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
#[cfg(test)]
use crate::bounded_ownership::ArtifactFixedRegistry;
#[cfg(test)]
use crate::bounded_ownership::PluginLifecycleStep;
#[cfg(test)]
use crate::bounded_ownership::plugin_demand_grant;
#[cfg(test)]
use crate::document_ingress::ActiveArtifactEnvelopeIngress;
#[cfg(test)]
use crate::tool_cancellation::ArtifactDocumentAuthority;
#[cfg(test)]
use crate::tool_cancellation::ToolCancellationHandle;
#[cfg(test)]
use crate::tool_cancellation::ToolOperationKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FrameworkReservedStage {
    Admitted,
    Cursor,
    Retiring,
}

const FRAMEWORK_RESERVED_STATE_BYTES: usize = 34;

macro_rules! framework_reserved_job {
    ($job:ident, $factory:ident, $route:literal, $route_index:literal, $raw:expr, $items:expr, $work:expr, $output:expr, lanes: [$($lane:ident),+]) => {
        pub struct $job {
            pub(crate) raw: Vec<u8>,
            envelope_cursor: usize,
            item_cursor: usize,
            total_items: usize,
            stage: FrameworkReservedStage,
            contract: semio_framework::ToolExecutionContract,
            operation: Option<semio_framework_job::Operation>,
            state: [u8; FRAMEWORK_RESERVED_STATE_BYTES],
            pending: Option<crate::tool_contract::ReservedOutcomeKind>,
            outcomes: crate::tool_contract::ReservedJobOutcomes,
            closing: bool,
        }

        impl $job {
            #[allow(dead_code)]
            pub(crate) fn new(raw: Vec<u8>, total_items: usize) -> Self {
                Self {
                    raw,
                    envelope_cursor: 0,
                    item_cursor: 0,
                    total_items,
                    stage: FrameworkReservedStage::Admitted,
                    contract: semio_framework::ToolExecutionContract::resumable($raw, $items, $work, $output, 7_500, 1, 1),
                    operation: None,
                    state: [0; FRAMEWORK_RESERVED_STATE_BYTES],
                    pending: None,
                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
                    closing: false,
                }
            }

            fn write_state(&mut self) {
                self.state[0] = $route_index;
                self.state[1] = self.stage as u8;
                self.state[2..10].copy_from_slice(&(self.envelope_cursor as u64).to_le_bytes());
                self.state[10..18].copy_from_slice(&(self.raw.len() as u64).to_le_bytes());
                self.state[18..26].copy_from_slice(&(self.item_cursor as u64).to_le_bytes());
                self.state[26..34].copy_from_slice(&(self.total_items as u64).to_le_bytes());
            }

            fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
                if !self.outcomes.terminal_is_empty() {
                    return self.outcomes.close_demands();
                }
                Ok(super::reserved_job_bytes::demands(&self.raw, &self.operation))
            }
        }

        impl semio_framework_job::InteractiveJob for $job {
            fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
                if cx.is_cancelled() {
                    return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
                }
                if self.outcomes.is_delivered() {
                    self.outcomes.retire(cx)?;
                    return Ok(None);
                }
                if let Some(kind) = self.pending {
                    let source: &[u8] = match kind {
                        crate::tool_contract::ReservedOutcomeKind::Fault => concat!("framework reserved ", $route, " output exceeds its exact cap").as_bytes(),
                        crate::tool_contract::ReservedOutcomeKind::Commit => &self.raw,
                        _ => &self.state,
                    };
                    let outcome = self.outcomes.advance(kind, source, cx)?;
                    if outcome.is_some() {
                        self.pending = None;
                    }
                    return Ok(outcome);
                }
                match self.stage {
                    FrameworkReservedStage::Admitted => {
                        self.stage = FrameworkReservedStage::Cursor;
                        cx.set_stage(concat!("framework-reserved-", $route, "-envelope"));
                        self.write_state();
                        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Preview);
                    }
                    FrameworkReservedStage::Cursor => {
                        let envelope_remaining = self.raw.len().saturating_sub(self.envelope_cursor);
                        let item_remaining = self.total_items.saturating_sub(self.item_cursor);
                        let remaining = if envelope_remaining != 0 { envelope_remaining } else { item_remaining };
                        let units = remaining.min(4_096).min(cx.fuel_remaining() as usize).min(self.contract.max_work_units_per_step as usize);
                        if envelope_remaining != 0 {
                            self.envelope_cursor += units;
                        } else {
                            self.item_cursor += units;
                        }
                        cx.consume_fuel(units as u64);
                        if cx.is_cancelled() {
                            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
                        }
                        if self.envelope_cursor >= self.raw.len() && self.item_cursor >= self.total_items {
                            self.stage = FrameworkReservedStage::Retiring;
                            cx.set_stage(concat!("framework-reserved-", $route, "-commit-ready"));
                        }
                        self.write_state();
                        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Checkpoint { applied_progress: self.envelope_cursor.saturating_add(self.item_cursor) as u64 });
                    }
                    FrameworkReservedStage::Retiring => {
                        self.pending = Some(if self.raw.len() > self.contract.max_output_bytes { crate::tool_contract::ReservedOutcomeKind::Fault } else { crate::tool_contract::ReservedOutcomeKind::Commit });
                    }
                }
                Ok(None)
            }

            fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
                self.outcomes.borrow_outcome(descriptor)
            }

            fn begin_close(&mut self) {
                self.closing = true;
            }

            fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
                use semio_framework_job::InteractiveJobCloseStep as Step;
                self.closing=true;
                let demand=self.close_demand()?;
                let remaining=cx.retained_grant();
                if !cx.should_yield()&&remaining.maximum_items>0&&remaining.maximum_depth<demand.depth{return Err(semio_framework_job::OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::DepthLimit,"original reserved close exceeds caller depth")))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{self.pending=None;if !self.outcomes.terminal_is_empty(){self.outcomes.close_step(grant)}else{super::reserved_job_bytes::close(&mut self.raw,&mut self.operation,grant)}})?;
                Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
            }

            fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.copy_bytes)}
            fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.close_demand()?.capacity_bytes)}
            fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.release_bytes)}
            fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.depth)}

            fn terminal_is_empty(&self) -> bool {
                self.closing && self.raw.is_empty() && self.raw.capacity()==0 && self.operation.is_none() && self.outcomes.terminal_is_empty()
            }
        }

        impl ArtifactReservedJob for $job {}

        pub struct $factory<A: ArtifactApp> {
            keys: Vec<semio_framework::ToolFactoryKey>,
            marker: std::marker::PhantomData<fn() -> A>,
        }

        impl<A: ArtifactApp> $factory<A> {
            fn new(controller_id: &str) -> Self {
                Self { keys: vec![semio_framework::ToolFactoryKey::new(controller_id, $route)], marker: std::marker::PhantomData }
            }
        }

        impl<A: ArtifactApp> semio_framework::ToolJobFactory for $factory<A> {
            type Payload = ArtifactReservedToolJob;
            type Job = ArtifactReservedToolJob;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                &self.keys
            }

            fn payload_schema_id(&self) -> &str {
                concat!("framework.reserved.", $route, ".v1")
            }

            fn classification(&self) -> semio_framework::InteractiveJobClassification {
                semio_framework::InteractiveJobClassification::Migrated
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolExecutionContract::resumable($raw, $items, $work, $output, 7_500, 1, 1)
            }

            fn create_job(&mut self, operation: semio_framework_job::Operation, mut payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
                payload.bind_operation(operation)?;
                Ok(payload)
            }
        }

        impl<A: ArtifactApp> ArtifactOwnedToolJobFactory for $factory<A> {
            type Owner = A;
            const TOOL_IDS: &'static [&'static str] = &[$route];
            const DOCUMENT_SCHEMA: &'static str = A::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: $route, lanes: &[$(ArtifactToolPublicationLane::$lane),+] }];
        }
    };
}

framework_reserved_job!(FrameworkUndoJob, FrameworkUndoJobFactory, "undo", 0, 1_024, 16, 1_024, 1_024, lanes: [HostOnly]);

framework_reserved_job!(FrameworkRedoJob, FrameworkRedoJobFactory, "redo", 1, 1_024, 16, 1_024, 1_024, lanes: [HostOnly]);

framework_reserved_job!(FrameworkCommitCheckpointJob, FrameworkCommitCheckpointJobFactory, "commitCheckpoint", 2, 16_384, 128, 4_096, 16_384, lanes: [HostOnly]);

framework_reserved_job!(FrameworkCreateAlternativeJob, FrameworkCreateAlternativeJobFactory, "createAlternative", 3, 8_192, 32, 4_096, 8_192, lanes: [HostOnly]);

framework_reserved_job!(FrameworkSwitchAlternativeJob, FrameworkSwitchAlternativeJobFactory, "switchAlternative", 4, 8_192, 32, 4_096, 8_192, lanes: [HostOnly]);

framework_reserved_job!(FrameworkCheckoutCheckpointJob, FrameworkCheckoutCheckpointJobFactory, "checkoutCheckpoint", 5, 8_192, 32, 4_096, 8_192, lanes: [HostOnly]);

framework_reserved_job!(FrameworkRevertToCommandJob, FrameworkRevertToCommandJobFactory, "revertToCommand", 6, 8_192, 32, 256, 8_192, lanes: [HostOnly]);

framework_reserved_job!(FrameworkCopyJob, FrameworkCopyJobFactory, "copy", 7, 8_192, 4_096, 256, semio_framework::kernel::CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES, lanes: [HostOnly]);

framework_reserved_job!(FrameworkCutJob, FrameworkCutJobFactory, "cut", 8, 8_192, 4_096, 256, semio_framework::kernel::CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES, lanes: [Artifact, Child]);

framework_reserved_job!(FrameworkPasteJob, FrameworkPasteJobFactory, "paste", 9, semio_framework::kernel::CLIPBOARD_PASTE_MAX_WIRE_BYTES, 4_096, 4_096, semio_framework::kernel::CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES*2, lanes: [Artifact, Child]);

// 🔌️ Route ordinal 10. `dispatch_import_media` still resolves `qualified_tool_proof("import-media")`
// and admits its exact wire against it, so without this registration EVERY app's `import_media`
// answers `interactive-job.missing-factory` (restored: it was dropped in 860e015bf6 while all
// four of its call sites stayed — ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
framework_reserved_job!(FrameworkImportMediaJob, FrameworkImportMediaJobFactory, "import-media", 10, 8_388_608, 8_192, 4_096, 8_388_608, lanes: [Artifact, Child]);

framework_reserved_job!(FrameworkNoteShellCommandJob, FrameworkNoteShellCommandJobFactory, "noteShellCommand", 11, 65_536, 64, 4_096, 65_536, lanes: [HostOnly]);

framework_reserved_job!(FrameworkSetHistoryCommandFilterJob, FrameworkSetHistoryCommandFilterJobFactory, "setHistoryCommandFilter", 12, 4_096, 16, 1_024, 4_096, lanes: [HostOnly]);

framework_reserved_job!(FrameworkRecordTutorialJob, FrameworkRecordTutorialJobFactory, "recordTutorial", 13, 4_096, 16, 1_024, 4_096, lanes: [HostOnly]);

framework_reserved_job!(FrameworkClearSelectionJob, FrameworkClearSelectionJobFactory, "clearSelection", 14, 4_096, 4_096, 256, 65_536, lanes: [HostOnly]);

framework_reserved_job!(FrameworkInteractionHoverJob, FrameworkInteractionHoverJobFactory, "interactionHover", 15, 65_536, 4_096, 256, 65_536, lanes: [HostOnly]);

framework_reserved_job!(FrameworkInteractionSelectJob, FrameworkInteractionSelectJobFactory, "interactionSelect", 16, 65_536, 4_096, 256, 65_536, lanes: [HostOnly]);

framework_reserved_job!(FrameworkSelectAllJob, FrameworkSelectAllJobFactory, "selectAll", 17, 4_096, 4_096, 256, 65_536, lanes: [HostOnly]);

framework_reserved_job!(FrameworkSetInteractionGranularityJob, FrameworkSetInteractionGranularityJobFactory, "setInteractionGranularity", 18, 16_384, 64, 1_024, 16_384, lanes: [HostOnly]);

framework_reserved_job!(FrameworkSetSelectionModeJob, FrameworkSetSelectionModeJobFactory, "setSelectionMode", 19, 16_384, 64, 1_024, 16_384, lanes: [HostOnly]);

/// 🎛️ The one place a framework-reserved route id becomes its retained job. Deliberately a plain
/// non-`async` fn OUTSIDE `dispatch_framework_reserved_action`'s generator: every arm erases into
/// the same [`ArtifactReservedToolJob`], so one `match` here costs one small frame, whereas the
/// same arms spelled inline each owned a separate, non-overlapping `run_framework_reserved_job`
/// future slot in the single resume frame — 1.80 MiB of stack for ONE dispatch in an unoptimized
/// build, more than a default 2 MiB thread or an 8 MiB wasm shadow stack can absorb once the rest
/// of the route is layered on top (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️selection-overflow-2026-09-09.md`). `copy`/`cut`/`paste` reach it only when the app declares
/// no reserved job of its own.
/// 🧮️ The ONE work capacity every framework-reserved route declares. Its durable lane is a
/// bounded config store whose preflight ([`admit_bounded_config_mutation`]) declares one
/// POINT-INVERTIBLE item — a forward row plus an inverse row — so a reserved gesture of `n`
/// items costs `n` × that, in the very unit `ArtifactStore::fold_batch_item` measures. The
/// ceiling is the store's own one-item maximum expressed in items, so a gesture this refuses is
/// one the store could never admit either. `interactionSelect` and its five interaction siblings
/// used to declare ONE work item against that TWO-row footprint, which is the same
/// declaration-vs-arithmetic split `📓️fold-contract-2026-09-10.md` fixed one layer down
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const FRAMEWORK_RESERVED_ROUTE_CAPACITY: crate::retained_command::ArtifactRetainedWorkCapacity =
    crate::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS / store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);

fn framework_reserved_route_job(action: &str, raw: Vec<u8>, work_items: usize) -> Result<ArtifactReservedToolJob, Fault> {
    Ok(match action {
        "undo" => ArtifactReservedToolJob::new(FrameworkUndoJob::new(raw, work_items)),
        "redo" => ArtifactReservedToolJob::new(FrameworkRedoJob::new(raw, work_items)),
        "commitCheckpoint" => ArtifactReservedToolJob::new(FrameworkCommitCheckpointJob::new(raw, work_items)),
        "createAlternative" => ArtifactReservedToolJob::new(FrameworkCreateAlternativeJob::new(raw, work_items)),
        "switchAlternative" => ArtifactReservedToolJob::new(FrameworkSwitchAlternativeJob::new(raw, work_items)),
        "checkoutCheckpoint" => ArtifactReservedToolJob::new(FrameworkCheckoutCheckpointJob::new(raw, work_items)),
        REVERT_TO_COMMAND_ACTION_ID => ArtifactReservedToolJob::new(FrameworkRevertToCommandJob::new(raw, work_items)),
        "copy" => ArtifactReservedToolJob::new(FrameworkCopyJob::new(raw, work_items)),
        "cut" => ArtifactReservedToolJob::new(FrameworkCutJob::new(raw, work_items)),
        "paste" => ArtifactReservedToolJob::new(FrameworkPasteJob::new(raw, work_items)),
        NOTE_SHELL_COMMAND_ACTION_ID => ArtifactReservedToolJob::new(FrameworkNoteShellCommandJob::new(raw, work_items)),
        SET_HISTORY_COMMAND_FILTER_ACTION_ID => ArtifactReservedToolJob::new(FrameworkSetHistoryCommandFilterJob::new(raw, work_items)),
        RECORD_TUTORIAL_ACTION_ID => ArtifactReservedToolJob::new(FrameworkRecordTutorialJob::new(raw, work_items)),
        CLEAR_SELECTION_ACTION_ID => ArtifactReservedToolJob::new(FrameworkClearSelectionJob::new(raw, work_items)),
        INTERACTION_HOVER_ACTION_ID => ArtifactReservedToolJob::new(FrameworkInteractionHoverJob::new(raw, work_items)),
        INTERACTION_SELECT_ACTION_ID => ArtifactReservedToolJob::new(FrameworkInteractionSelectJob::new(raw, work_items)),
        SELECT_ALL_ACTION_ID => ArtifactReservedToolJob::new(FrameworkSelectAllJob::new(raw, work_items)),
        SET_INTERACTION_GRANULARITY_ACTION_ID => ArtifactReservedToolJob::new(FrameworkSetInteractionGranularityJob::new(raw, work_items)),
        SET_SELECTION_MODE_ACTION_ID => ArtifactReservedToolJob::new(FrameworkSetSelectionModeJob::new(raw, work_items)),
        _ => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-reserved"), format!("unknown framework reserved route '{action}'"))),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FrameworkSharedActionDescriptorRoute {
    action_id: &'static str,
    schema_id: &'static str,
}

const FRAMEWORK_SHARED_ACTION_DESCRIPTOR_ROUTES: [FrameworkSharedActionDescriptorRoute; 12] = [
    FrameworkSharedActionDescriptorRoute { action_id: "copy", schema_id: "framework.reserved.copy.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "cut", schema_id: "framework.reserved.cut.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "paste", schema_id: "framework.reserved.paste.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "noteShellCommand", schema_id: "framework.reserved.noteShellCommand.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "setHistoryCommandFilter", schema_id: "framework.reserved.setHistoryCommandFilter.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "recordTutorial", schema_id: "framework.reserved.recordTutorial.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "clearSelection", schema_id: "framework.reserved.clearSelection.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "interactionHover", schema_id: "framework.reserved.interactionHover.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "interactionSelect", schema_id: "framework.reserved.interactionSelect.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "selectAll", schema_id: "framework.reserved.selectAll.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "setInteractionGranularity", schema_id: "framework.reserved.setInteractionGranularity.v1" },
    FrameworkSharedActionDescriptorRoute { action_id: "setSelectionMode", schema_id: "framework.reserved.setSelectionMode.v1" },
];

fn framework_shared_action_descriptor_schema_id(action_id: &str) -> Option<&'static str> {
    FRAMEWORK_SHARED_ACTION_DESCRIPTOR_ROUTES.iter().find(|route| route.action_id == action_id).map(|route| route.schema_id)
}

pub(crate) fn join_framework_shared_action_dispositions(definition: &mut AppDefinition) {
    for action in definition.actions.iter_mut().chain(definition.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
        if framework_shared_action_descriptor_schema_id(&action.id).is_some() {
            action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
        }
    }
}

#[path="⚠️configuration/🦀️.rs"]
mod original_configuration_failure;
pub(crate) use original_configuration_failure::OriginalConfigurationFailure;
pub(crate) const FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES:usize=8*1024*1024;

pub(crate) enum FrameworkConfigurationBinaryStage {
    Envelope,
    Decode,
    Complete,
}

/// 🔓️ Shared result slot for a retained configuration command decoder.
type ConfigurationDecodeCompletion<M> = std::sync::Arc<std::sync::Mutex<Option<Result<ArtifactCommand<M>, semio_framework_job::OriginalTurnFailure>>>>;

pub(crate) struct ConfigurationDecodeRetirement<M: semio_framework_value::retirement::RetireOwned> {
    pub(crate) original: ManuallyDrop<Option<ConfigurationDecodeCompletion<M>>>,
    pub(crate) payload: Option<ControlledRetirement<Option<Result<ArtifactCommand<M>, semio_framework_job::OriginalTurnFailure>>>>,
}

impl<M: semio_framework_value::retirement::RetireOwned> ConfigurationDecodeRetirement<M> {
    pub(crate) fn terminal_is_empty(&self) -> bool { self.original.is_none() && self.payload.is_none() }
    pub(crate) fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.original.is_some() {
            let release = std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<std::sync::Mutex<Option<Result<ArtifactCommand<M>, semio_framework_job::OriginalTurnFailure>>>>()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "configuration completion Arc extent overflow"))?.0.pad_to_align().size();
            return Ok(RetirementDemand { copy_bytes: size_of::<std::sync::Mutex<Option<Result<ArtifactCommand<M>, semio_framework_job::OriginalTurnFailure>>>>() + size_of::<Option<Result<ArtifactCommand<M>, semio_framework_job::OriginalTurnFailure>>>() + size_of::<Option<ConfigurationDecodeCompletion<M>>>(), release_bytes: release, depth: 2, ..Default::default() });
        }
        if let Some(owner) = self.payload.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(&self.payload), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "configuration completion depth overflow"))? });
        }
        Ok(Default::default())
    }
    pub(crate) fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demand(grant.maximum_copy_bytes)?;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.original.is_some() {
            let original = self.original.take().expect("admitted original configuration completion");
            let Some(mutex) = std::sync::Arc::into_inner(original) else { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<ConfigurationDecodeCompletion<M>>>(), ..empty })); };
            let original = mutex.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.payload = Some(ControlledRetirement::new(original).unwrap_or_else(|_| unreachable!("configuration command and fault declare controlled retirement")));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..empty }));
        }
        let owner = self.payload.as_mut().expect("original configuration completion payload");
        if owner.terminal_is_empty() { drop(self.payload.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "configuration completion payload")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
}

impl<M: semio_framework_value::retirement::RetireOwned> Drop for ConfigurationDecodeRetirement<M> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "configuration completion requires its original controlled close"); }
}

pub(crate) struct FrameworkConfigurationBinaryJob<A: ArtifactApp> {
    pub(crate) raw: Vec<u8>,
    pub(crate) cursor: usize,
    pub(crate) stage: FrameworkConfigurationBinaryStage,
    pub(crate) operation: Option<semio_framework_job::Operation>,
    pub(crate) decoded: ManuallyDrop<Option<ConfigurationDecodeCompletion<A::ConfigMutation>>>,
    pub(crate) decoded_close: Option<ConfigurationDecodeRetirement<A::ConfigMutation>>,
    pub(crate) receiving:ManuallyDrop<Option<store::os_store::command_format::receiving::RetainedArtifactCommandReceiving<A::ConfigMutation>>>,
    pub(crate) native:semio_framework_job::OriginalNativeDecodeSource,
    pub(crate) failure:ManuallyDrop<Option<OriginalConfigurationFailure>>,
    pub(crate) state: [u8; 8],
    pub(crate) pending: Option<crate::tool_contract::ReservedOutcomeKind>,
    pub(crate) outcomes: crate::tool_contract::ReservedJobOutcomes,
    pub(crate) closing: bool,
}

impl<A: ArtifactApp> FrameworkConfigurationBinaryJob<A> {
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.outcomes.terminal_is_empty() { return self.outcomes.close_demands(); }
        if self.failure.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.receiving.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.close_demands(body)}}
        if !self.native.terminal_is_empty(){return self.native.retirement_demands(body)}
        if self.raw.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.raw.capacity(),depth:1,..Default::default()})}
        if let Some(owner) = self.decoded_close.as_ref() { return if owner.terminal_is_empty() { Ok(RetirementDemand { copy_bytes: size_of_val(&self.decoded_close), depth: 1, ..Default::default() }) } else { owner.demand(body) }; }
        if self.decoded.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<ConfigurationDecodeRetirement<A::ConfigMutation>>(), depth: 1, ..Default::default() }); }
        if self.operation.is_some() { return Ok(RetirementDemand { copy_bytes: size_of_val(&self.operation), depth: 1, ..Default::default() }); }
        Ok(Default::default())
    }
}

impl<A: ArtifactApp> semio_framework_job::InteractiveJob for FrameworkConfigurationBinaryJob<A> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use crate::tool_contract::ReservedOutcomeKind;
        if self.failure.is_some(){if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.failure,cx)?{return Err(failure)}return Ok(None)}
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.outcomes.is_delivered() {
            self.outcomes.retire(cx)?;
            return Ok(None);
        }
        if let Some(kind) = self.pending {
            let source: &[u8] = if kind == ReservedOutcomeKind::Commit { &self.raw } else { &self.state };
            let outcome = self.outcomes.advance(kind, source, cx)?;
            if outcome.is_some() {
                self.pending = None;
            }
            return Ok(outcome);
        }
        match self.stage {
            FrameworkConfigurationBinaryStage::Envelope=>{
                cx.set_stage("framework-configuration-binary-source");if !self.native.admit_step(cx)?{return Ok(None)}
                if self.receiving.is_none(){if cx.should_yield(){return Ok(None)}if let Some((owner,progress))=store::os_store::command_format::receiving::RetainedArtifactCommandReceiving::admit(&self.raw,(0,self.raw.len()),cx.retained_grant())?{*self.receiving=Some(owner);cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);self.stage=FrameworkConfigurationBinaryStage::Decode;}}
            },
            FrameworkConfigurationBinaryStage::Decode=>{
                cx.set_stage("framework-configuration-binary-decode");let owner=self.receiving.as_mut().expect("original command receiving frame remains mounted");
                if owner.is_ready(){let grant=cx.retained_grant();if cx.should_yield(){return Ok(None)}let mut command=None;let progress=owner.take_into(&mut command,grant)?;if let Some(command)=command{let output=self.decoded.as_ref().expect("original configuration completion remains mounted");let mut slot=output.lock().unwrap_or_else(std::sync::PoisonError::into_inner);assert!(slot.is_none(),"original configuration completion cannot overwrite its typed source");*slot=Some(Ok(command));self.stage=FrameworkConfigurationBinaryStage::Complete;}cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);return Ok(None)}
                if let Some(outcome)=owner.receive_step(&self.raw,cx,&mut self.native)?{match(outcome.value,outcome.receiving_refusal){(Err(producer),Some(receiving))=>*self.failure=Some(OriginalConfigurationFailure::Native{producer,receiving}),(Err(producer),None)=>return Err(producer.into()),(Ok(_),Some(receiving))=>return Err(receiving),(Ok(progress),None)=>{self.cursor=self.cursor.saturating_add(progress.copied_items);self.state=(self.cursor as u64).to_le_bytes();self.pending=Some(ReservedOutcomeKind::Checkpoint{applied_progress:self.cursor as u64});}}}
            },
            FrameworkConfigurationBinaryStage::Complete => {
                self.pending = Some(ReservedOutcomeKind::Commit);
            }
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        self.closing=true;if self.terminal_is_empty(){return Ok(Step::Complete{progress:Default::default()})}let demand=self.close_demand(cx.retained_grant().maximum_copy_bytes)?;
        let remaining=cx.retained_grant();
        if !cx.should_yield()&&remaining.maximum_items>0&&remaining.maximum_depth<demand.depth{return Err(semio_framework_job::OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::DepthLimit,"original configuration close exceeds caller depth")))}
        if self.outcomes.terminal_is_empty(){
            if self.failure.is_some(){if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.failure,cx)?{return Err(failure)}return Ok(Step::Pending{progress:Default::default()})}
            if let Some(owner)=self.receiving.as_mut(){if !owner.terminal_is_empty(){if let Some(outcome)=owner.close_on_original(cx,&mut self.native)?{let progress=outcome.progress;match(outcome.value,outcome.receiving_refusal){(Err(producer),Some(receiving))=>*self.failure=Some(OriginalConfigurationFailure::Native{producer,receiving}),(Err(producer),None)=>return Err(producer.into()),(Ok(_),Some(receiving))=>return Err(receiving),(Ok(_),None)=>{}}return Ok(Step::Pending{progress})}return Ok(Step::Pending{progress:Default::default()})}}
            if self.receiving.is_none()&&!self.native.terminal_is_empty(){let step=self.native.close_step(cx)?;return Ok(Step::Pending{progress:step.progress()})}
        }
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            self.pending=None;if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}
            if self.receiving.as_ref().is_some_and(|owner|owner.terminal_is_empty()){self.receiving.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
            if self.raw.capacity()!=0{let released_bytes=self.raw.capacity();drop(std::mem::take(&mut self.raw));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,released_bytes,..Default::default()}))}
            if let Some(owner)=self.decoded_close.as_mut(){if !owner.terminal_is_empty(){return owner.step(grant)}drop(self.decoded_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if let Some(original)=self.decoded.take(){self.decoded_close=Some(ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(original)),payload:None});return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            self.operation=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
        })?;
        Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.close_demand(usize::MAX).map(|demand| demand.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.close_demand(body).map(|demand| demand.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { self.close_demand(usize::MAX).map(|demand| demand.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { self.close_demand(usize::MAX).map(|demand| demand.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw.capacity() == 0 && self.decoded.is_none() && self.decoded_close.is_none() && self.receiving.is_none() && self.native.terminal_is_empty() && self.failure.is_none() && self.operation.is_none() && self.outcomes.terminal_is_empty()
    }
}

struct FrameworkConfigurationBinaryJobFactory<A: ArtifactApp> {
    keys: Vec<semio_framework::ToolFactoryKey>,
    marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> FrameworkConfigurationBinaryJobFactory<A> {
    fn new(controller_id: &str) -> Self {
        Self { keys: vec![semio_framework::ToolFactoryKey::new(controller_id, "configuration-binary")], marker: std::marker::PhantomData }
    }
}

impl<A: ArtifactApp> semio_framework::ToolJobFactory for FrameworkConfigurationBinaryJobFactory<A> {
    type Payload = FrameworkConfigurationBinaryJob<A>;
    type Job = FrameworkConfigurationBinaryJob<A>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "framework.reserved.configuration-binary.v1"
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(65_536, 1_024, 4_096, 65_536, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, mut payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        payload.operation = Some(operation);
        Ok(payload)
    }
}

impl<A: ArtifactApp> ArtifactOwnedToolJobFactory for FrameworkConfigurationBinaryJobFactory<A> {
    type Owner = A;
    const TOOL_IDS: &'static [&'static str] = &["configuration-binary"];
    const DOCUMENT_SCHEMA: &'static str = A::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "configuration-binary", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

pub(crate) fn register_framework_reserved_factories<A: ArtifactApp>(registry: &mut ArtifactToolFactoryRegistry<'_, A>) -> Result<(), Fault> {
    let controller_id = registry.controller_id().to_string();
    registry.register(FrameworkUndoJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkRedoJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkCommitCheckpointJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkCreateAlternativeJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkSwitchAlternativeJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkCheckoutCheckpointJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkRevertToCommandJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkCopyJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkCutJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkPasteJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkImportMediaJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkNoteShellCommandJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkSetHistoryCommandFilterJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkRecordTutorialJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkClearSelectionJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkInteractionHoverJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkInteractionSelectJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkSelectAllJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkSetInteractionGranularityJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkSetSelectionModeJobFactory::<A>::new(&controller_id))?;
    registry.register(FrameworkConfigurationBinaryJobFactory::<A>::new(&controller_id))?;
    Ok(())
}

#[derive(Clone)]
pub(crate) struct QualifiedBoundedFirstStepProof {
    pub(crate) row: ArtifactBoundedFirstStepProof,
    pub(crate) owner: ToolOwnerWitness,
}

impl QualifiedBoundedFirstStepProof {
    pub(crate) fn key(&self) -> semio_framework::ToolFactoryKey {
        semio_framework::ToolFactoryKey::new(self.row.controller_id, self.row.tool_id)
    }

    pub(crate) fn schema_id(&self) -> String {
        format!("{}.tool-command.v1", self.row.artifact_schema)
    }

    pub(crate) fn contract(&self) -> semio_framework::ToolExecutionContract {
        self.row.contract
    }

    pub(crate) fn admits<A: ArtifactApp>(&self, admission: &semio_framework::ToolWireAdmission) -> bool {
        self.owner == ToolOwnerWitness::of::<A>()
            && self.row.owner == self.owner
            && admission.key == self.key()
            && admission.schema_id == self.schema_id()
            && admission.contract == self.contract()
            && admission.factory_type_id == std::any::TypeId::of::<TypedCommandFullOperationJobFactory<A>>()
            && admission.factory_type_name == std::any::type_name::<TypedCommandFullOperationJobFactory<A>>()
    }
}

#[derive(Clone)]
pub(crate) enum QualifiedToolProof {
    Bounded(QualifiedBoundedFirstStepProof),
    FrameworkOwned(ArtifactToolRegistration),
    AppOwned(ArtifactToolRegistration),
}

impl QualifiedToolProof {
    pub(crate) fn key(&self) -> semio_framework::ToolFactoryKey {
        match self {
            Self::Bounded(proof) => proof.key(),
            Self::FrameworkOwned(registration) | Self::AppOwned(registration) => registration.key.clone(),
        }
    }

    pub(crate) fn publication_lanes(&self) -> &'static [ArtifactToolPublicationLane] {
        match self {
            Self::Bounded(_) => &[],
            Self::FrameworkOwned(registration) | Self::AppOwned(registration) => registration.publication_lanes,
        }
    }

    pub(crate) fn schema_id(&self) -> String {
        match self {
            Self::Bounded(proof) => proof.schema_id(),
            Self::FrameworkOwned(registration) | Self::AppOwned(registration) => registration.schema_id.clone(),
        }
    }

    pub(crate) fn contract(&self) -> semio_framework::ToolExecutionContract {
        match self {
            Self::Bounded(proof) => proof.contract(),
            Self::FrameworkOwned(registration) | Self::AppOwned(registration) => registration.contract,
        }
    }

    pub(crate) fn admits<A: ArtifactApp>(&self, admission: &semio_framework::ToolWireAdmission) -> bool {
        match self {
            Self::Bounded(proof) => proof.admits::<A>(admission),
            Self::FrameworkOwned(registration) | Self::AppOwned(registration) => {
                registration.owner == ToolOwnerWitness::of::<A>()
                    && admission.key == registration.key
                    && admission.schema_id == registration.schema_id
                    && admission.contract == registration.contract
                    && admission.factory_type_id == registration.factory_type_id
                    && admission.factory_type_name == registration.factory_type_name
            }
        }
    }
}

pub(crate) struct FrameworkReservedCommitPermit {
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) lease: ToolCancellationLease,
    pub(crate) producer_fault: Option<ArtifactBoundedToolFault>,
    pub(crate) identity: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    pub(crate) identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress,
}

#[cfg(test)]
include!("../../🪟️window/🫧️transient/🧪️tests/🔁️document-replacement/🦀️.rs");

#[cfg(test)]
include!("../../🧪️tests/🔬️app-artifact-fixed-registry/🦀️.rs");

#[cfg(test)]
include!("../../🧪️tests/🔬️app-artifact-reserved-tool-job/🦀️.rs");

impl FrameworkReservedCommitPermit {
    pub(crate) fn new(operation: semio_framework_job::Operation, lease: ToolCancellationLease, maximum_identity_bytes: usize) -> Result<Self, Fault> {
        let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !lease.token.is_cancelled_now() && !lease.publication_claim.is_cancelled();
        let control = semio_framework_value::NativeEncodeControl::new(maximum_identity_bytes, &mut observer);
        let identity = control.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        Ok(Self { operation, lease, producer_fault: None, identity: Some(identity), identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 } })
    }

    pub(crate) async fn is_cancelled(&self) -> bool {
        self.lease.is_cancelled().await
    }

    pub(crate) fn finish(mut self) {
        self.lease.finish();
    }
}

pub(crate) const FRAMEWORK_RESERVED_JOB_MAGIC: &[u8; 8] = b"FRRESV01";

pub(crate) fn encode_framework_reserved_job_input(work_items: u64, raw: &[u8]) -> Vec<u8> {
    let mut input = Vec::with_capacity(16 + raw.len());
    input.extend_from_slice(FRAMEWORK_RESERVED_JOB_MAGIC);
    input.extend_from_slice(&work_items.to_le_bytes());
    input.extend_from_slice(raw);
    input
}

pub(crate) struct PendingFrameworkReserved {
    pub(crate) action: String,
    pub(crate) args: Option<DslValue>,
    pub(crate) meta: ActionMeta,
    pub(crate) permit: FrameworkReservedCommitPermit,
}

/// 🧾️ A framework-reserved route whose spawned job finished and whose commit the app now owns.
/// [`VcsArtifactApp::step_framework_reserved_commit`] advances it one bounded unit per call from the
/// reactor's typed-operation continuation, so a commit that fans out over composed children
/// (checkpoint pins, checkout cascade) or walks history (revert) is driven across turns instead of
/// having to finish inside the turn that saw its `JobCompleted`. Every unit re-reads the
/// operation's cancellation lease, and `applied`/`total` is its progress.
pub(crate) struct FrameworkReservedCommit {
    pub(crate) action: String,
    pub(crate) args: Option<DslValue>,
    pub(crate) meta: ActionMeta,
    pub(crate) permit: FrameworkReservedCommitPermit,
    pub(crate) log_generation_before: u64,
    pub(crate) stage: FrameworkReservedCommitStage,
    pub(crate) applied: u64,
    pub(crate) total: u64,
}

/// 🪜️ The next unit a [`FrameworkReservedCommit`] runs.
pub(crate) enum FrameworkReservedCommitStage {
    Validate,
    CheckpointChildren { message: String, authors: Vec<vcs::Author>, keys: Vec<MemberKey>, next: usize, pins: Vec<vcs::CompositionPin> },
    Route { pins: Vec<vcs::CompositionPin> },
    CheckoutChildren { pins: Vec<vcs::CompositionPin>, next: usize, result: InvocationResult },
    Revert { edit_id: String },
}

/// 📊️ Progress of the framework-reserved commit at the head of an app's commit queue: `applied` of
/// `total` bounded units, for the operation that admitted it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameworkReservedCommitProgress {
    pub operation: u64,
    pub applied: u64,
    pub total: u64,
}

/// 🫧️ Yields one mounted plugin-job transition back to the host executor.
pub(crate) async fn plugin_job_yield_once() {
    let mut yielded = false;
    std::future::poll_fn(move |cx| {
        if yielded {
            std::task::Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    })
    .await;
}

/// 🔔️ The paid turns one lent reserved outcome may spend acknowledging and removing its descriptor.
pub(crate) const RESERVED_OUTCOME_ACKNOWLEDGE_TURNS: usize = 8;

/// 🚦️ Whether one mounted-worker pump rejection is scheduler weather rather than a lost transition.
/// The process pool admits a step through a per-worker `try_lock` and a finite per-lane queue, so a
/// worker that happens to be popping its own queue, a momentarily full lane, and a step whose outcome
/// has not landed yet all return an `Err` that says NOTHING about this job — the exact same set the
/// mounted typed-operation path already answers with `AwaitingInput`. Faulting a framework-reserved
/// route on them made every `interactionHover`/`interactionSelect` a coin flip against pool load.
pub(crate) fn reserved_pump_fault_is_transient(fault: &semio_framework_job::MountedWorkerJobPumpFault) -> bool {
    matches!(
        fault,
        semio_framework_job::MountedWorkerJobPumpFault::Submit(semio_framework_job::WorkerJobSubmitFault::Contention(_))
            | semio_framework_job::MountedWorkerJobPumpFault::Submit(semio_framework_job::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated))
            | semio_framework_job::MountedWorkerJobPumpFault::Take(semio_framework_job::WorkerJobTakeFault::Pending)
    )
}

/// 🏷️ The exact rejection a structural pump fault carries, so the route's fault message names the
/// transition it lost instead of a bare code.
pub(crate) fn reserved_pump_fault_detail(fault: &semio_framework_job::MountedWorkerJobPumpFault) -> String {
    match fault {
        semio_framework_job::MountedWorkerJobPumpFault::Submit(submit) => format!("submit {submit:?}"),
        semio_framework_job::MountedWorkerJobPumpFault::Take(take) => format!("take {take:?}"),
        semio_framework_job::MountedWorkerJobPumpFault::MissingTicket => "submitted step lost its exact ticket".to_string(),
        semio_framework_job::MountedWorkerJobPumpFault::CheckedOut => "outcome is already checked out".to_string(),
    }
}

/// 🧾️ Compares a retained fixed-page payload without materializing a whole-buffer adapter.
pub(crate) fn retained_payload_eq_slice(payload: &semio_framework_job::RetainedJobPayload, expected: &[u8]) -> bool {
    if payload.len() != expected.len() {
        return false;
    }
    let mut cursor = 0usize;
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
        let end = cursor.saturating_add(page.len());
        if expected.get(cursor..end) != Some(page) {
            return false;
        }
        cursor = end;
    }
    reader.terminal_is_empty() && cursor == expected.len()
}

pub(crate) const HISTORY_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];

pub(crate) const CLIPBOARD_ACTION_IDS: [&str; 3] = ["copy", "cut", "paste"];

pub(crate) fn is_framework_reserved_action_id(action: &str) -> bool {
    HISTORY_ACTION_IDS.contains(&action)
        || CLIPBOARD_ACTION_IDS.contains(&action)
        || INTERACTION_ACTION_IDS.contains(&action)
        || is_tool_run_action_id(action)
        || is_time_travel_action_id(action)
        || action == CANCEL_TYPED_OPERATION_ACTION_ID
        || action == semio_framework::HOST_EVENT_ACTION_ID
        || matches!(action, REVERT_TO_COMMAND_ACTION_ID | SET_HISTORY_COMMAND_FILTER_ACTION_ID | NOTE_SHELL_COMMAND_ACTION_ID | RECORD_TUTORIAL_ACTION_ID)
        || matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID)
}

/// 📜️ Joins the app's declared bounded-first-step proofs to the live registry and its exact factory registrations.
pub(crate) fn tool_job_registration<A: ArtifactApp>(
    registry: &AppActionRegistry,
    runtime_controller_id: &str,
    artifact_schema: &str,
    generated_ids: &[&str],
    bus: &semio_framework::ActionBus,
    registrations: &BTreeMap<String, ArtifactToolRegistration>,
) -> Result<(String, Vec<QualifiedBoundedFirstStepProof>), Fault> {
    validate_tool_job_rows::<A>(registry, runtime_controller_id, artifact_schema, generated_ids, bus, registrations, A::bounded_first_step_tool_proofs())
}

/// 🧾️ Admits each proof row only when owner, controller, schema, factory and generated migration join exactly.
pub(crate) fn validate_tool_job_rows<A: ArtifactApp>(
    registry: &AppActionRegistry,
    runtime_controller_id: &str,
    artifact_schema: &str,
    generated_ids: &[&str],
    bus: &semio_framework::ActionBus,
    registrations: &BTreeMap<String, ArtifactToolRegistration>,
    rows: Vec<ArtifactBoundedFirstStepProof>,
) -> Result<(String, Vec<QualifiedBoundedFirstStepProof>), Fault> {
    if !registry.controller_id.is_empty() && registry.controller_id != runtime_controller_id {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.catalog-controller"), "tool proof catalog controller does not match the live registry controller"));
    }
    let controller_id = runtime_controller_id.to_string();
    if registry.is_registry_less() {
        return Ok((controller_id, Vec::new()));
    }
    let migrated = registry.migrated_tool_ids();
    let expected = generated_ids.iter().filter(|id| **id != "typed-command" && migrated.contains(**id)).copied().collect::<BTreeSet<_>>();
    let owner = ToolOwnerWitness::of::<A>();
    let mut seen = BTreeSet::new();
    let mut proofs = Vec::new();
    for row in rows {
        let unique = seen.insert(row.tool_id);
        let registered = registrations.get(row.tool_id);
        let generic = row.factory == BOUNDED_FIRST_STEP_FACTORY && row.factory_type_id.is_none() && row.factory_type_name.is_none() && registered.is_none();
        let exact_registered = registered.is_some_and(|registration| {
            row.factory_type_id == Some(registration.factory_type_id)
                && row.factory_type_name == Some(registration.factory_type_name)
                && row.factory == registration.factory_type_name.rsplit("::").next().unwrap_or_default()
                && registration.owner == owner
                && registration.key.controller_id == runtime_controller_id
                && registration.key.tool_id == row.tool_id
                && registration.contract == row.contract
                && bus.admit_exact_wire(runtime_controller_id, row.tool_id, &registration.schema_id, &[]).is_ok_and(|admission| QualifiedToolProof::AppOwned(registration.clone()).admits::<A>(&admission))
        });
        let authoritative = row.owner == owner && row.controller_id == runtime_controller_id && row.artifact_schema == artifact_schema && (generic || exact_registered) && !row.owner_file.is_empty() && expected.contains(row.tool_id) && unique;
        if !authoritative {
            return Err(Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.catalog-authority"),
                format!(
                    "tool factory proof rejected tool '{}': owner='{}' expected_owner='{}', controller='{}' expected_controller='{}', schema='{}' expected_schema='{}', factory='{}' registered_factory='{}', owner_file_present={}, generated_migrated={}, unique={unique}, typed_join={exact_registered}; catalog-authority-detail: owner_witness={:?} expected_witness={:?} owner_eq={} controller_eq={} schema_eq={} generic={generic} migrated={migrated:?} generated_ids={generated_ids:?}",
                    row.tool_id,
                    row.owner.owner_type_name,
                    owner.owner_type_name,
                    row.controller_id,
                    runtime_controller_id,
                    row.artifact_schema,
                    artifact_schema,
                    row.factory,
                    registered.map_or(BOUNDED_FIRST_STEP_FACTORY, |registration| registration.factory_type_name),
                    !row.owner_file.is_empty(),
                    expected.contains(row.tool_id),
                    row.owner,
                    owner,
                    row.owner == owner,
                    row.controller_id == runtime_controller_id,
                    row.artifact_schema == artifact_schema,
                ),
            ));
        }
        if generic {
            proofs.push(QualifiedBoundedFirstStepProof { row, owner });
        }
    }
    if seen != expected {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.catalog-incomplete"), "a migrated generated command lacks its exact owner-local bounded reducer proof"));
    }
    Ok((controller_id, proofs))
}
