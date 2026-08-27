//! 🧵️ Note-owned retained command microstate and exact publication contracts.

use crate::artifacts::note::schema::NoteIdOwner;
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use crate::editor::note::commands::{delete_block, duplicate_block, ink_apply_events, patch_blocks};
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::editor::note::{NoteCommand, NoteDispatchCtx, NotePlayApp, NOTE_INTERACTION_BLOCKS};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, EditorApp, Emit, Fault, FaultCode, FaultOrigin, HistoryView};

//#region 🔖️Contract
pub const NOTE_RETAINED_PAYLOAD_SCHEMA: &str = "semio.note.retained-command.v1";
pub const NOTE_RETAINED_RAW_BYTES: usize = 65_536;
pub const NOTE_RETAINED_MAXIMUM_UNITS: usize = 4_096;

pub const NOTE_AUDITED_TOOL_IDS: &[&str] = &[
    "setGridVisible",
    "setGridSpacing",
    "setGridSubdivisions",
    "setGridOpacity",
    "setSnapEnabled",
    "setSnapGridSpacing",
    "setPencilWidth",
    "setEraserRadius",
    "addBlock",
    "moveBlock",
    "deleteBlock",
    "deleteSelection",
    "duplicateBlock",
    "duplicateSelection",
    "patchBlocks",
    "setActiveExample",
    "setFixtureJson",
    "inkApplyEvents",
    "engagementSubmit",
    "nudgeSelection",
    "nudgeSelectionUp",
    "nudgeSelectionDown",
    "nudgeSelectionLeft",
    "nudgeSelectionRight",
    "nudgeSelectionUpFast",
    "nudgeSelectionDownFast",
    "nudgeSelectionLeftFast",
    "nudgeSelectionRightFast",
    "setCamera",
    "setCameraZoom",
    "setActiveUtility",
    "setLocale",
    "engagementInput",
    "navigatorEngagementInput",
    "saveDownload",
    "loadRequest",
];

pub const NOTE_RETAINED_TOOL_IDS: &[&str] = &[
    "setGridVisible",
    "setGridSpacing",
    "setCamera",
    "setCameraZoom",
    "setActiveUtility",
    "setLocale",
    "engagementInput",
    "navigatorEngagementInput",
    "loadRequest",
];

pub const NOTE_AUDITED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setGridSubdivisions", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setGridOpacity", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSnapEnabled", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSnapGridSpacing", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setPencilWidth", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setEraserRadius", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "duplicateBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchBlocks", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "inkApplyEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionUp", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionDown", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeft", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionRight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionUpFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionDownFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeftFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionRightFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "navigatorEngagementInput", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveDownload", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

pub const NOTE_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "navigatorEngagementInput", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn note_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(NOTE_RETAINED_RAW_BYTES, NOTE_RETAINED_MAXIMUM_UNITS, 1, 262_144, 7_500, 1, 1)
}
//#endregion 🔖️Contract

//#region 🧭️Units
#[derive(Clone)]
struct NoteCommandUnit {
    command: NoteCommand,
    selected_block_ids: Vec<String>,
}

fn selected_block_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction
        .selection
        .get(NOTE_INTERACTION_BLOCKS)
        .into_iter()
        .flat_map(|selection| selection.ids.iter())
        .filter_map(|id| crate::artifacts::note::schema::block_id_from_tree_row_id(id))
        .collect()
}

fn selection_units(command: &NoteCommand, selected: &[String]) -> Option<Vec<NoteCommandUnit>> {
    let is_selection_command = matches!(
        command,
        NoteCommand::DeleteSelection(_)
            | NoteCommand::DuplicateSelection(_)
            | NoteCommand::NudgeSelection(_)
            | NoteCommand::NudgeSelectionUp(_)
            | NoteCommand::NudgeSelectionDown(_)
            | NoteCommand::NudgeSelectionLeft(_)
            | NoteCommand::NudgeSelectionRight(_)
            | NoteCommand::NudgeSelectionUpFast(_)
            | NoteCommand::NudgeSelectionDownFast(_)
            | NoteCommand::NudgeSelectionLeftFast(_)
            | NoteCommand::NudgeSelectionRightFast(_)
    );
    is_selection_command.then(|| {
        if selected.is_empty() {
            vec![NoteCommandUnit { command: command.clone(), selected_block_ids: Vec::new() }]
        } else {
            selected.iter().map(|id| NoteCommandUnit { command: command.clone(), selected_block_ids: vec![id.clone()] }).collect()
        }
    })
}

fn note_command_units(command: &NoteCommand, selected: &[String]) -> Result<Vec<NoteCommandUnit>, Fault> {
    if let Some(units) = selection_units(command, selected) {
        return Ok(units);
    }
    match command {
        NoteCommand::PatchBlocks(payload) => Ok(if payload.block_ids.is_empty() {
            vec![NoteCommandUnit { command: command.clone(), selected_block_ids: selected.to_vec() }]
        } else {
            payload
                .block_ids
                .iter()
                .map(|block_id| NoteCommandUnit {
                    command: NoteCommand::PatchBlocks(patch_blocks::PatchBlocks { block_ids: vec![block_id.clone()], field: payload.field.clone(), value: payload.value.clone() }),
                    selected_block_ids: selected.to_vec(),
                })
                .collect()
        }),
        NoteCommand::InkApplyEvents(payload) => {
            let events = serde_json::from_str::<serde_json::Value>(&payload.events_json)
                .ok()
                .and_then(|value| value.as_array().cloned())
                .unwrap_or_default();
            Ok(if events.is_empty() {
                vec![NoteCommandUnit { command: command.clone(), selected_block_ids: selected.to_vec() }]
            } else {
                events
                    .into_iter()
                    .map(|event| NoteCommandUnit {
                        command: NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: serde_json::Value::Array(vec![event]).to_string(), phase: payload.phase.clone(), select_ids: payload.select_ids.clone() }),
                        selected_block_ids: selected.to_vec(),
                    })
                    .collect()
            })
        }
        _ => Ok(vec![NoteCommandUnit { command: command.clone(), selected_block_ids: selected.to_vec() }]),
    }
}
//#endregion 🧭️Units

//#region 🧵️Work
struct NoteCommandWork {
    tool_id: &'static str,
    units: Vec<NoteCommandUnit>,
    cursor: usize,
    replay_target: Option<usize>,
    projection: Option<NoteSnapshot>,
    accumulated: Emit<crate::artifacts::note::op::NoteMutation, NoteConfigMutation>,
    id_owner: Option<NoteIdOwner>,
    workspace_identity: u64,
    complete: bool,
    closing: bool,
}

impl NoteCommandWork {
    fn new(tool_id: &'static str, command: &NoteCommand, _snapshot: &NoteSnapshot, interaction: &protocol::InteractionState, operation: &AppOperationContext) -> Result<Self, Fault> {
        let units = note_command_units(command, &selected_block_ids(interaction))?;
        if units.is_empty() || units.len() > NOTE_RETAINED_MAXIMUM_UNITS {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.extent"), "Note command exceeds its fixed semantic-unit envelope"));
        }
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Ok(Self {
            tool_id,
            units,
            cursor: 0,
            replay_target: None,
            projection: None,
            accumulated: Emit::default(),
            id_owner: Some(NoteIdOwner::new(scope, 0)),
            workspace_identity,
            complete: false,
            closing: false,
        })
    }

    fn append(&mut self, mut emit: Emit<crate::artifacts::note::op::NoteMutation, NoteConfigMutation>) -> Result<(), Fault> {
        if self.accumulated.description.is_some() && emit.description.is_some() && self.accumulated.description != emit.description {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.description"), "Note semantic units produced incompatible edit descriptions"));
        }
        if self.accumulated.coalesce_key.is_some() && emit.coalesce_key.is_some() && self.accumulated.coalesce_key != emit.coalesce_key {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.coalesce"), "Note semantic units produced incompatible coalescing owners"));
        }
        if self.accumulated.description.is_none() {
            self.accumulated.description = emit.description.take();
        }
        if self.accumulated.coalesce_key.is_none() {
            self.accumulated.coalesce_key = emit.coalesce_key.take();
        }
        self.accumulated.artifact_mutations.append(&mut emit.artifact_mutations);
        self.accumulated.config_mutations.append(&mut emit.config_mutations);
        self.accumulated.draft_mutations.append(&mut emit.draft_mutations);
        self.accumulated.effects.append(&mut emit.effects);
        self.accumulated.events.append(&mut emit.events);
        self.accumulated.child_emits.append(&mut emit.child_emits);
        self.accumulated.tasks.append(&mut emit.tasks);
        Ok(())
    }

    fn release_one(&mut self) -> bool {
        if self.units.pop().is_some() {
            return true;
        }
        if self.accumulated.artifact_mutations.pop().is_some()
            || self.accumulated.config_mutations.pop().is_some()
            || self.accumulated.draft_mutations.pop().is_some()
            || self.accumulated.effects.pop().is_some()
            || self.accumulated.events.pop().is_some()
            || self.accumulated.child_emits.pop().is_some()
            || self.accumulated.tasks.pop().is_some()
            || self.accumulated.description.take().is_some()
            || self.accumulated.coalesce_key.take().is_some()
        {
            return true;
        }
        if self.projection.take().is_some() || self.id_owner.take().is_some() {
            return true;
        }
        false
    }
}

impl ArtifactCommandWork<EditorApp<NotePlayApp>> for NoteCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    fn extent(&self, _command: &NoteCommand, _snapshot: &NoteSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<EditorApp<NotePlayApp>>>) -> Option<usize> {
        Some(self.units.len())
    }

    fn step(
        &mut self,
        _command: &NoteCommand,
        snapshot: &NoteSnapshot,
        config: &NoteConfig,
        history: &HistoryView,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
        _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<EditorApp<NotePlayApp>>>,
        operation: &AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<EditorApp<NotePlayApp>>, Fault> {
        if self.complete || self.cursor >= self.units.len() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.repeated"), "Note retained work was stepped after completion"));
        }
        let unit = &self.units[self.cursor];
        let projection = self.projection.as_ref().unwrap_or(snapshot);
        let id_owner = self.id_owner.as_mut().ok_or_else(|| Fault::from("note-retained-id-owner-missing"))?;
        let mut ctx = NoteDispatchCtx { selected_block_ids: unit.selected_block_ids.clone(), id_owner: id_owner.clone() };
        let emit = unit.command.dispatch(&ArtifactView::with_operation(projection, history, operation.clone()), &ConfigView { snapshot: config }, &mut ctx)?;
        *id_owner = ctx.id_owner;
        if self.cursor + 1 < self.units.len() {
            for mutation in &emit.artifact_mutations {
                let current = self.projection.as_ref().unwrap_or(snapshot);
                self.projection = Some(crate::artifacts::note::schema::mutations::apply_note_mutation(current, mutation).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("note.retained.projection"), error.to_string()))?);
            }
        }
        self.append(emit)?;
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "note-command-replay", preview: b"{\"en\":\"Restoring Note command\",\"de\":\"Notizbefehl wird wiederhergestellt\"}" });
            }
        }
        if self.cursor < self.units.len() {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "note-command-semantic-unit", preview: b"{\"en\":\"Applying Note command\",\"de\":\"Notizbefehl wird angewendet\"}" });
        }
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(std::mem::take(&mut self.accumulated)))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 32 {
            return Err(Fault::from("note-retained-checkpoint-capacity"));
        }
        target[..32].fill(0);
        target[..4].copy_from_slice(b"NRC1");
        target[4] = u8::from(self.complete);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        target[24..32].copy_from_slice(&(self.units.len() as u64).to_le_bytes());
        Ok(32)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 32 || &checkpoint[..4] != b"NRC1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(Fault::from("note-retained-checkpoint-invalid"));
        }
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("note-retained-checkpoint-cursor"))?) as usize;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("note-retained-checkpoint-identity"))?);
        let extent = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("note-retained-checkpoint-extent"))?) as usize;
        if identity != self.workspace_identity || extent != self.units.len() || cursor > extent {
            return Err(Fault::from("note-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.complete = false;
        self.accumulated = Emit::default();
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.release_one() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.units.is_empty()
            && self.accumulated.artifact_mutations.is_empty()
            && self.accumulated.config_mutations.is_empty()
            && self.accumulated.draft_mutations.is_empty()
            && self.accumulated.effects.is_empty()
            && self.accumulated.events.is_empty()
            && self.accumulated.child_emits.is_empty()
            && self.accumulated.tasks.is_empty()
            && self.accumulated.description.is_none()
            && self.accumulated.coalesce_key.is_none()
            && self.projection.is_none()
            && self.id_owner.is_none()
    }
}
//#endregion 🧵️Work

//#region 🏭️Factory
pub struct NoteCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl NoteCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: NOTE_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for NoteCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<NotePlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<NotePlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        NOTE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        note_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > NOTE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Note retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for NoteCommandJobFactory {
    type Owner = EditorApp<NotePlayApp>;
    const TOOL_IDS: &'static [&'static str] = NOTE_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = NOTE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NOTE_RETAINED_PUBLICATION_CONTRACTS;
}

pub fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<NotePlayApp>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(NoteCommandJobFactory::new(&controller))
}

pub async fn build(request: ArtifactOwnedToolJobRequest<EditorApp<NotePlayApp>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    if !NOTE_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    if request.command.command_id() != request.tool_id {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.tool-mismatch"), "Note command does not match its exact registered tool"));
    }
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let tool_id = request.command.command_id();
    let work = Box::new(NoteCommandWork::new(tool_id, &request.command, &request.snapshot, &request.interaction_state, &operation)?);
    let payload = ArtifactRetainedCommandPayload::try_new_with_context(
        *request.command,
        request.snapshot,
        request.config,
        request.history,
        request.interaction_state,
        request.interaction_hover,
        request.context,
        operation,
        request.completion,
        NoteCommand::command_id,
        NOTE_RETAINED_RAW_BYTES,
        NOTE_RETAINED_MAXIMUM_UNITS,
        work,
    )?;
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🏭️Factory

//#region 🕸️TextChildMaterialization
const NOTE_MATERIALIZATION_STRING_CHUNK_BYTES: usize = 1_024;

#[derive(Default)]
struct NoteStringMaterializationCursor {
    source_cursor: usize,
    bytes: Vec<u8>,
}

impl NoteStringMaterializationCursor {
    fn step(&mut self, source: &str) -> Result<Option<String>, String> {
        if self.source_cursor > source.len() {
            return Err("Note string materialization cursor escaped its exact source".into());
        }
        let end = self.source_cursor.saturating_add(NOTE_MATERIALIZATION_STRING_CHUNK_BYTES).min(source.len());
        self.bytes.extend_from_slice(&source.as_bytes()[self.source_cursor..end]);
        self.source_cursor = end;
        if self.source_cursor != source.len() {
            return Ok(None);
        }
        self.source_cursor = 0;
        String::from_utf8(std::mem::take(&mut self.bytes)).map(Some).map_err(|error| error.to_string())
    }

    fn from_owned(value: String) -> Self {
        Self { source_cursor: value.len(), bytes: value.into_bytes() }
    }

    fn close_step(&mut self, maximum_bytes: usize) -> usize {
        let released = maximum_bytes.min(self.bytes.len());
        self.bytes.truncate(self.bytes.len() - released);
        if self.bytes.is_empty() {
            self.source_cursor = 0;
        }
        released
    }

    fn terminal_is_empty(&self) -> bool {
        self.source_cursor == 0 && self.bytes.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteTextChildMaterializationPhase {
    ChildId,
    ArtifactId,
    ArtifactKind,
    Standard,
    Subset,
    Complete,
}

struct NoteTextChildMaterializationCursor {
    phase: NoteTextChildMaterializationPhase,
    string: NoteStringMaterializationCursor,
    child_id: Option<String>,
    artifact_id: Option<String>,
    artifact_kind: Option<String>,
    standard: Option<String>,
    subset: Option<String>,
    local_owner: Option<std::sync::Arc<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>>,
    retirement: Vec<NoteStringMaterializationCursor>,
    closing: bool,
}

impl NoteTextChildMaterializationCursor {
    fn new(source: &crate::artifacts::note::NoteTextChild) -> Self {
        Self {
            phase: NoteTextChildMaterializationPhase::ChildId,
            string: NoteStringMaterializationCursor::default(),
            child_id: None,
            artifact_id: None,
            artifact_kind: None,
            standard: None,
            subset: None,
            local_owner: source.handle.local_owner::<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>(),
            retirement: Vec::new(),
            closing: false,
        }
    }

    fn step(&mut self, source: &crate::artifacts::note::NoteTextChild) -> Result<Option<store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>>, String> {
        if self.closing {
            return Err("Note text-child materialization was stepped after cancellation".into());
        }
        match self.phase {
            NoteTextChildMaterializationPhase::ChildId => {
                if let Some(value) = self.string.step(&source.handle.child_id)? {
                    self.child_id = Some(value);
                    self.phase = NoteTextChildMaterializationPhase::ArtifactId;
                }
            }
            NoteTextChildMaterializationPhase::ArtifactId => {
                if let Some(value) = self.string.step(&source.handle.target.artifact_id)? {
                    self.artifact_id = Some(value);
                    self.phase = NoteTextChildMaterializationPhase::ArtifactKind;
                }
            }
            NoteTextChildMaterializationPhase::ArtifactKind => {
                if let Some(value) = self.string.step(&source.handle.target.dialect.artifact_kind)? {
                    self.artifact_kind = Some(value);
                    self.phase = NoteTextChildMaterializationPhase::Standard;
                }
            }
            NoteTextChildMaterializationPhase::Standard => {
                if let Some(value) = self.string.step(&source.handle.target.dialect.standard)? {
                    self.standard = Some(value);
                    self.phase = NoteTextChildMaterializationPhase::Subset;
                }
            }
            NoteTextChildMaterializationPhase::Subset => {
                if let Some(value) = self.string.step(&source.handle.target.dialect.subset)? {
                    self.subset = Some(value);
                    self.phase = NoteTextChildMaterializationPhase::Complete;
                }
            }
            NoteTextChildMaterializationPhase::Complete => {
                let target = store::os_io::ArtifactRef {
                    artifact_id: self.artifact_id.take().ok_or_else(|| "Note text-child artifact id owner is absent".to_string())?,
                    dialect: store::os_io::ArtifactDialect {
                        artifact_kind: self.artifact_kind.take().ok_or_else(|| "Note text-child artifact kind owner is absent".to_string())?,
                        standard: self.standard.take().ok_or_else(|| "Note text-child standard owner is absent".to_string())?,
                        subset: self.subset.take().ok_or_else(|| "Note text-child subset owner is absent".to_string())?,
                    },
                };
                let child = store::ArtifactChild::new(self.child_id.take().ok_or_else(|| "Note text-child id owner is absent".to_string())?, target);
                return Ok(Some(match self.local_owner.take() {
                    Some(owner) => child.with_local_owner(owner),
                    None => child,
                }));
            }
        }
        Ok(None)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for value in [self.child_id.take(), self.artifact_id.take(), self.artifact_kind.take(), self.standard.take(), self.subset.take()].into_iter().flatten() {
            self.retirement.push(NoteStringMaterializationCursor::from_owned(value));
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let released = if self.string.terminal_is_empty() {
            self.retirement.last_mut().map_or(0, |cursor| cursor.close_step(maximum_bytes))
        } else {
            self.string.close_step(maximum_bytes)
        };
        if self.retirement.last().is_some_and(NoteStringMaterializationCursor::terminal_is_empty) {
            self.retirement.pop();
        }
        if released != 0 {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
        }
        if self.local_owner.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.string.terminal_is_empty() && self.retirement.is_empty() && self.child_id.is_none() && self.artifact_id.is_none() && self.artifact_kind.is_none() && self.standard.is_none() && self.subset.is_none() && self.local_owner.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteTextContentMaterializationPhase {
    Handle,
    Paragraph,
    RunText,
    RunLink,
    RunComplete,
    Complete,
}

struct NoteTextContentMaterializationCursor {
    phase: NoteTextContentMaterializationPhase,
    handle: NoteTextChildMaterializationCursor,
    materialized_handle: Option<store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>>,
    paragraphs: Vec<crate::artifacts::note::NoteTextParagraph>,
    paragraph_cursor: usize,
    run_cursor: usize,
    text: NoteStringMaterializationCursor,
    link: NoteStringMaterializationCursor,
    active_text: Option<String>,
    active_link: Option<String>,
    retirement: Vec<NoteStringMaterializationCursor>,
    closing: bool,
}

impl NoteTextContentMaterializationCursor {
    fn new(source: &crate::artifacts::note::NoteTextChild) -> Self {
        Self {
            phase: NoteTextContentMaterializationPhase::Handle,
            handle: NoteTextChildMaterializationCursor::new(source),
            materialized_handle: None,
            paragraphs: Vec::new(),
            paragraph_cursor: 0,
            run_cursor: 0,
            text: NoteStringMaterializationCursor::default(),
            link: NoteStringMaterializationCursor::default(),
            active_text: None,
            active_link: None,
            retirement: Vec::new(),
            closing: false,
        }
    }

    fn step(&mut self, source: &crate::artifacts::note::NoteTextChild) -> Result<Option<crate::artifacts::note::NoteTextChild>, String> {
        if self.closing {
            return Err("Note text-content materialization was stepped after cancellation".into());
        }
        match self.phase {
            NoteTextContentMaterializationPhase::Handle => {
                if let Some(handle) = self.handle.step(source)? {
                    self.materialized_handle = Some(handle);
                    self.phase = NoteTextContentMaterializationPhase::Paragraph;
                }
            }
            NoteTextContentMaterializationPhase::Paragraph => {
                if self.paragraph_cursor == source.paragraphs.len() {
                    self.phase = NoteTextContentMaterializationPhase::Complete;
                } else {
                    self.paragraphs.push(crate::artifacts::note::NoteTextParagraph { runs: Vec::new() });
                    self.run_cursor = 0;
                    self.phase = if source.paragraphs[self.paragraph_cursor].runs.is_empty() {
                        self.paragraph_cursor += 1;
                        NoteTextContentMaterializationPhase::Paragraph
                    } else {
                        NoteTextContentMaterializationPhase::RunText
                    };
                }
            }
            NoteTextContentMaterializationPhase::RunText => {
                let run = &source.paragraphs[self.paragraph_cursor].runs[self.run_cursor];
                if let Some(value) = self.text.step(&run.text)? {
                    self.active_text = Some(value);
                    self.phase = if run.link.is_some() { NoteTextContentMaterializationPhase::RunLink } else { NoteTextContentMaterializationPhase::RunComplete };
                }
            }
            NoteTextContentMaterializationPhase::RunLink => {
                let run = &source.paragraphs[self.paragraph_cursor].runs[self.run_cursor];
                if let Some(value) = self.link.step(run.link.as_deref().unwrap_or_default())? {
                    self.active_link = Some(value);
                    self.phase = NoteTextContentMaterializationPhase::RunComplete;
                }
            }
            NoteTextContentMaterializationPhase::RunComplete => {
                let run = &source.paragraphs[self.paragraph_cursor].runs[self.run_cursor];
                self.paragraphs.last_mut().ok_or_else(|| "Note text-content paragraph owner is absent".to_string())?.runs.push(crate::artifacts::note::NoteTextRun {
                    text: self.active_text.take().ok_or_else(|| "Note text-content run owner is absent".to_string())?,
                    bold: run.bold,
                    italic: run.italic,
                    underline: run.underline,
                    link: self.active_link.take(),
                });
                self.run_cursor += 1;
                if self.run_cursor == source.paragraphs[self.paragraph_cursor].runs.len() {
                    self.paragraph_cursor += 1;
                    self.phase = NoteTextContentMaterializationPhase::Paragraph;
                } else {
                    self.phase = NoteTextContentMaterializationPhase::RunText;
                }
            }
            NoteTextContentMaterializationPhase::Complete => {
                return Ok(Some(crate::artifacts::note::NoteTextChild {
                    handle: self.materialized_handle.take().ok_or_else(|| "Note text-content handle owner is absent".to_string())?,
                    paragraphs: std::mem::take(&mut self.paragraphs),
                }));
            }
        }
        Ok(None)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.handle.begin_close();
        for value in [self.active_text.take(), self.active_link.take()].into_iter().flatten() {
            self.retirement.push(NoteStringMaterializationCursor::from_owned(value));
        }
    }

    fn retire_materialized_handle(&mut self) -> bool {
        let Some(mut handle) = self.materialized_handle.take() else { return false };
        for value in [
            std::mem::take(&mut handle.child_id),
            std::mem::take(&mut handle.target.artifact_id),
            std::mem::take(&mut handle.target.dialect.artifact_kind),
            std::mem::take(&mut handle.target.dialect.standard),
            std::mem::take(&mut handle.target.dialect.subset),
        ] {
            self.retirement.push(NoteStringMaterializationCursor::from_owned(value));
        }
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if !self.handle.terminal_is_empty() {
            return self.handle.close_step(maximum_items, maximum_bytes);
        }
        for cursor in [&mut self.text, &mut self.link] {
            if !cursor.terminal_is_empty() {
                let released = cursor.close_step(maximum_bytes);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
            }
        }
        if let Some(cursor) = self.retirement.last_mut() {
            let released = cursor.close_step(maximum_bytes);
            if cursor.terminal_is_empty() {
                self.retirement.pop();
            }
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
        }
        if let Some(run) = self.paragraphs.last_mut().and_then(|paragraph| paragraph.runs.pop()) {
            self.retirement.push(NoteStringMaterializationCursor::from_owned(run.text));
            if let Some(link) = run.link {
                self.retirement.push(NoteStringMaterializationCursor::from_owned(link));
            }
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.paragraphs.last().is_some_and(|paragraph| paragraph.runs.is_empty()) {
            self.paragraphs.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.retire_materialized_handle() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.handle.terminal_is_empty()
            && self.materialized_handle.is_none()
            && self.paragraphs.is_empty()
            && self.text.terminal_is_empty()
            && self.link.terminal_is_empty()
            && self.active_text.is_none()
            && self.active_link.is_none()
            && self.retirement.is_empty()
    }
}

#[derive(Default)]
struct NoteOwnedRetirement {
    strings: Vec<NoteStringMaterializationCursor>,
    blocks: Vec<crate::artifacts::note::NoteBlockNode>,
    block_lists: Vec<Vec<crate::artifacts::note::NoteBlockNode>>,
    text_children: Vec<crate::artifacts::note::NoteTextChild>,
    paragraph_lists: Vec<Vec<crate::artifacts::note::NoteTextParagraph>>,
    run_lists: Vec<Vec<crate::artifacts::note::NoteTextRun>>,
    string_lists: Vec<Vec<String>>,
    table_row_lists: Vec<Vec<Vec<crate::artifacts::note::NoteTableCell>>>,
    table_cell_lists: Vec<Vec<crate::artifacts::note::NoteTableCell>>,
    point_lists: Vec<Vec<[f64; 2]>>,
    assets: Vec<crate::artifacts::note::NoteImageAsset>,
    asset_maps: Vec<std::collections::BTreeMap<String, crate::artifacts::note::NoteImageAsset>>,
    links: Vec<store::ArtifactLink>,
    text_owners: Vec<std::sync::Arc<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>>,
}

impl NoteOwnedRetirement {
    fn push_string(&mut self, value: String) {
        self.strings.push(NoteStringMaterializationCursor::from_owned(value));
    }

    fn push_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_string(value);
        }
    }

    fn push_block(&mut self, block: crate::artifacts::note::NoteBlockNode) {
        self.blocks.push(block);
    }

    fn push_text_child(&mut self, child: crate::artifacts::note::NoteTextChild) {
        self.text_children.push(child);
    }

    fn push_asset(&mut self, asset: crate::artifacts::note::NoteImageAsset) {
        self.assets.push(asset);
    }

    fn push_link(&mut self, link: store::ArtifactLink) {
        self.links.push(link);
    }

    fn push_snapshot(&mut self, snapshot: NoteSnapshot) {
        self.push_string(snapshot.schema);
        self.push_string(snapshot.id);
        self.push_optional_string(snapshot.title);
        self.block_lists.push(snapshot.blocks);
        self.asset_maps.push(snapshot.assets);
        if let Some(link) = snapshot.linked_artifact {
            self.push_link(link);
        }
    }

    fn enqueue_block(&mut self, block: crate::artifacts::note::NoteBlockNode) {
        match block {
            crate::artifacts::note::NoteBlockNode::Text { id, name, content, font_weight, align, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.push_text_child(content);
                self.push_string(font_weight);
                self.push_string(align);
            }
            crate::artifacts::note::NoteBlockNode::Image { id, name, image_key, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.push_string(image_key);
            }
            crate::artifacts::note::NoteBlockNode::Table { id, name, columns, rows, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.string_lists.push(columns);
                self.table_row_lists.push(rows);
            }
            crate::artifacts::note::NoteBlockNode::Math { id, name, tex, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.push_string(tex);
            }
            crate::artifacts::note::NoteBlockNode::Ink { id, name, points, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.point_lists.push(points);
            }
            crate::artifacts::note::NoteBlockNode::Group { id, name, children, .. } => {
                self.push_string(id);
                self.push_string(name);
                self.block_lists.push(children);
            }
        }
    }

    fn enqueue_text_child(&mut self, mut child: crate::artifacts::note::NoteTextChild) {
        self.push_string(std::mem::take(&mut child.handle.child_id));
        self.push_string(std::mem::take(&mut child.handle.target.artifact_id));
        self.push_string(std::mem::take(&mut child.handle.target.dialect.artifact_kind));
        self.push_string(std::mem::take(&mut child.handle.target.dialect.standard));
        self.push_string(std::mem::take(&mut child.handle.target.dialect.subset));
        if let Some(owner) = child
            .handle
            .local_owner::<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>()
        {
            self.text_owners.push(owner);
        }
        self.paragraph_lists.push(child.paragraphs);
    }

    fn enqueue_link(&mut self, link: store::ArtifactLink) {
        self.push_string(link.target.artifact_id);
        self.push_string(link.target.dialect.artifact_kind);
        self.push_string(link.target.dialect.standard);
        self.push_string(link.target.dialect.subset);
        self.push_string(link.role);
        match link.pin {
            store::LinkPin::Head => {}
            store::LinkPin::Checkpoint { id } => self.push_string(id),
            store::LinkPin::Snapshot { blob } => {
                self.push_string(blob.hash);
                self.push_string(blob.media_type);
            }
        }
    }

    fn close_step(&mut self, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if let Some(cursor) = self.strings.last_mut() {
            let released = cursor.close_step(maximum_bytes);
            if cursor.terminal_is_empty() {
                self.strings.pop();
            }
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
        }
        if let Some(block) = self.blocks.pop() {
            self.enqueue_block(block);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(blocks) = self.block_lists.last_mut() {
            if let Some(block) = blocks.pop() {
                self.blocks.push(block);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.block_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(child) = self.text_children.pop() {
            self.enqueue_text_child(child);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(paragraphs) = self.paragraph_lists.last_mut() {
            if let Some(paragraph) = paragraphs.pop() {
                self.run_lists.push(paragraph.runs);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.paragraph_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(runs) = self.run_lists.last_mut() {
            if let Some(run) = runs.pop() {
                self.push_string(run.text);
                self.push_optional_string(run.link);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.run_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(strings) = self.string_lists.last_mut() {
            if let Some(value) = strings.pop() {
                self.push_string(value);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.string_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(rows) = self.table_row_lists.last_mut() {
            if let Some(row) = rows.pop() {
                self.table_cell_lists.push(row);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.table_row_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(cells) = self.table_cell_lists.last_mut() {
            if let Some(cell) = cells.pop() {
                self.push_string(cell.content);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.table_cell_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(points) = self.point_lists.last_mut() {
            if points.pop().is_some() {
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.point_lists.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(asset) = self.assets.pop() {
            self.push_string(asset.mime);
            self.push_string(asset.data);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(assets) = self.asset_maps.last_mut() {
            if let Some((key, asset)) = assets.pop_last() {
                self.push_string(key);
                self.push_asset(asset);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.asset_maps.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(link) = self.links.pop() {
            self.enqueue_link(link);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.text_owners.pop().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.strings.is_empty()
            && self.blocks.is_empty()
            && self.block_lists.is_empty()
            && self.text_children.is_empty()
            && self.paragraph_lists.is_empty()
            && self.run_lists.is_empty()
            && self.string_lists.is_empty()
            && self.table_row_lists.is_empty()
            && self.table_cell_lists.is_empty()
            && self.point_lists.is_empty()
            && self.assets.is_empty()
            && self.asset_maps.is_empty()
            && self.links.is_empty()
            && self.text_owners.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteBlockMaterializationPhase {
    Id,
    Name,
    Payload,
    Complete,
}

enum NoteBlockPayloadMaterialization {
    Text {
        phase: u8,
        content: NoteTextContentMaterializationCursor,
        materialized_content: Option<crate::artifacts::note::NoteTextChild>,
        font_weight: NoteStringMaterializationCursor,
        materialized_font_weight: Option<String>,
        align: NoteStringMaterializationCursor,
        materialized_align: Option<String>,
    },
    Image {
        image_key: NoteStringMaterializationCursor,
        materialized_image_key: Option<String>,
    },
    Table {
        phase: u8,
        column_cursor: usize,
        cell_row_cursor: usize,
        cell_cursor: usize,
        string: NoteStringMaterializationCursor,
        columns: Vec<String>,
        rows: Vec<Vec<crate::artifacts::note::NoteTableCell>>,
    },
    Math {
        tex: NoteStringMaterializationCursor,
        materialized_tex: Option<String>,
    },
    Ink {
        point_cursor: usize,
        points: Vec<[f64; 2]>,
    },
    Group {
        child_cursor: usize,
        active: Option<Box<NoteBlockMaterializationCursor>>,
        children: Vec<crate::artifacts::note::NoteBlockNode>,
    },
}

struct NoteBlockMaterializationCursor {
    phase: NoteBlockMaterializationPhase,
    string: NoteStringMaterializationCursor,
    id: Option<String>,
    name: Option<String>,
    payload: NoteBlockPayloadMaterialization,
    retirement: NoteOwnedRetirement,
    closing: bool,
}

fn note_block_common(source: &crate::artifacts::note::NoteBlockNode) -> (&str, &str, f64, f64, f64, f64, f64, bool, bool) {
    match source {
        crate::artifacts::note::NoteBlockNode::Text { id, name, x, y, width, height, rotation, visible, locked, .. }
        | crate::artifacts::note::NoteBlockNode::Image { id, name, x, y, width, height, rotation, visible, locked, .. }
        | crate::artifacts::note::NoteBlockNode::Table { id, name, x, y, width, height, rotation, visible, locked, .. }
        | crate::artifacts::note::NoteBlockNode::Math { id, name, x, y, width, height, rotation, visible, locked, .. }
        | crate::artifacts::note::NoteBlockNode::Ink { id, name, x, y, width, height, rotation, visible, locked, .. }
        | crate::artifacts::note::NoteBlockNode::Group { id, name, x, y, width, height, rotation, visible, locked, .. } => {
            (id, name, *x, *y, *width, *height, *rotation, *visible, *locked)
        }
    }
}

impl NoteBlockMaterializationCursor {
    fn new(source: &crate::artifacts::note::NoteBlockNode) -> Self {
        let payload = match source {
            crate::artifacts::note::NoteBlockNode::Text { content, .. } => NoteBlockPayloadMaterialization::Text {
                phase: 0,
                content: NoteTextContentMaterializationCursor::new(content),
                materialized_content: None,
                font_weight: NoteStringMaterializationCursor::default(),
                materialized_font_weight: None,
                align: NoteStringMaterializationCursor::default(),
                materialized_align: None,
            },
            crate::artifacts::note::NoteBlockNode::Image { .. } => NoteBlockPayloadMaterialization::Image {
                image_key: NoteStringMaterializationCursor::default(),
                materialized_image_key: None,
            },
            crate::artifacts::note::NoteBlockNode::Table { .. } => NoteBlockPayloadMaterialization::Table {
                phase: 0,
                column_cursor: 0,
                cell_row_cursor: 0,
                cell_cursor: 0,
                string: NoteStringMaterializationCursor::default(),
                columns: Vec::new(),
                rows: Vec::new(),
            },
            crate::artifacts::note::NoteBlockNode::Math { .. } => NoteBlockPayloadMaterialization::Math {
                tex: NoteStringMaterializationCursor::default(),
                materialized_tex: None,
            },
            crate::artifacts::note::NoteBlockNode::Ink { .. } => NoteBlockPayloadMaterialization::Ink { point_cursor: 0, points: Vec::new() },
            crate::artifacts::note::NoteBlockNode::Group { .. } => NoteBlockPayloadMaterialization::Group { child_cursor: 0, active: None, children: Vec::new() },
        };
        Self {
            phase: NoteBlockMaterializationPhase::Id,
            string: NoteStringMaterializationCursor::default(),
            id: None,
            name: None,
            payload,
            retirement: NoteOwnedRetirement::default(),
            closing: false,
        }
    }

    fn step(&mut self, source: &crate::artifacts::note::NoteBlockNode) -> Result<Option<crate::artifacts::note::NoteBlockNode>, String> {
        if self.closing {
            return Err("Note block materialization was stepped after cancellation".into());
        }
        let (source_id, source_name, x, y, width, height, rotation, visible, locked) = note_block_common(source);
        match self.phase {
            NoteBlockMaterializationPhase::Id => {
                if let Some(value) = self.string.step(source_id)? {
                    self.id = Some(value);
                    self.phase = NoteBlockMaterializationPhase::Name;
                }
            }
            NoteBlockMaterializationPhase::Name => {
                if let Some(value) = self.string.step(source_name)? {
                    self.name = Some(value);
                    self.phase = NoteBlockMaterializationPhase::Payload;
                }
            }
            NoteBlockMaterializationPhase::Payload => {
                if self.step_payload(source)? {
                    self.phase = NoteBlockMaterializationPhase::Complete;
                }
            }
            NoteBlockMaterializationPhase::Complete => {
                let id = self.id.take().ok_or_else(|| "Note block id owner is absent".to_string())?;
                let name = self.name.take().ok_or_else(|| "Note block name owner is absent".to_string())?;
                return Ok(Some(match (&mut self.payload, source) {
                    (
                        NoteBlockPayloadMaterialization::Text { materialized_content, materialized_font_weight, materialized_align, .. },
                        crate::artifacts::note::NoteBlockNode::Text { font_size, .. },
                    ) => crate::artifacts::note::NoteBlockNode::Text {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        content: materialized_content.take().ok_or_else(|| "Note text block content owner is absent".to_string())?,
                        font_size: *font_size,
                        font_weight: materialized_font_weight.take().ok_or_else(|| "Note text block font-weight owner is absent".to_string())?,
                        align: materialized_align.take().ok_or_else(|| "Note text block alignment owner is absent".to_string())?,
                    },
                    (
                        NoteBlockPayloadMaterialization::Image { materialized_image_key, .. },
                        crate::artifacts::note::NoteBlockNode::Image { .. },
                    ) => crate::artifacts::note::NoteBlockNode::Image {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        image_key: materialized_image_key.take().ok_or_else(|| "Note image key owner is absent".to_string())?,
                    },
                    (
                        NoteBlockPayloadMaterialization::Table { columns, rows, .. },
                        crate::artifacts::note::NoteBlockNode::Table { .. },
                    ) => crate::artifacts::note::NoteBlockNode::Table {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        columns: std::mem::take(columns),
                        rows: std::mem::take(rows),
                    },
                    (
                        NoteBlockPayloadMaterialization::Math { materialized_tex, .. },
                        crate::artifacts::note::NoteBlockNode::Math { display_mode, .. },
                    ) => crate::artifacts::note::NoteBlockNode::Math {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        tex: materialized_tex.take().ok_or_else(|| "Note math source owner is absent".to_string())?,
                        display_mode: *display_mode,
                    },
                    (
                        NoteBlockPayloadMaterialization::Ink { points, .. },
                        crate::artifacts::note::NoteBlockNode::Ink { stroke_width, color, .. },
                    ) => crate::artifacts::note::NoteBlockNode::Ink {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        points: std::mem::take(points),
                        stroke_width: *stroke_width,
                        color: *color,
                    },
                    (
                        NoteBlockPayloadMaterialization::Group { children, .. },
                        crate::artifacts::note::NoteBlockNode::Group { .. },
                    ) => crate::artifacts::note::NoteBlockNode::Group {
                        id,
                        name,
                        x,
                        y,
                        width,
                        height,
                        rotation,
                        visible,
                        locked,
                        children: std::mem::take(children),
                    },
                    _ => return Err("Note block materialization variant changed beneath its operation-owned cursor".into()),
                }));
            }
        }
        Ok(None)
    }

    fn step_payload(&mut self, source: &crate::artifacts::note::NoteBlockNode) -> Result<bool, String> {
        match (&mut self.payload, source) {
            (
                NoteBlockPayloadMaterialization::Text { phase, content, materialized_content, font_weight, materialized_font_weight, align, materialized_align },
                crate::artifacts::note::NoteBlockNode::Text { content: source_content, font_weight: source_font_weight, align: source_align, .. },
            ) => match *phase {
                0 => {
                    if let Some(value) = content.step(source_content)? {
                        *materialized_content = Some(value);
                        *phase = 1;
                    }
                    Ok(false)
                }
                1 => {
                    if let Some(value) = font_weight.step(source_font_weight)? {
                        *materialized_font_weight = Some(value);
                        *phase = 2;
                    }
                    Ok(false)
                }
                2 => {
                    if let Some(value) = align.step(source_align)? {
                        *materialized_align = Some(value);
                        *phase = 3;
                    }
                    Ok(false)
                }
                _ => Ok(true),
            },
            (
                NoteBlockPayloadMaterialization::Image { image_key, materialized_image_key },
                crate::artifacts::note::NoteBlockNode::Image { image_key: source_image_key, .. },
            ) => {
                if materialized_image_key.is_none() {
                    *materialized_image_key = image_key.step(source_image_key)?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            (
                NoteBlockPayloadMaterialization::Table { phase, column_cursor, cell_row_cursor, cell_cursor, string, columns, rows },
                crate::artifacts::note::NoteBlockNode::Table { columns: source_columns, rows: source_rows, .. },
            ) => match *phase {
                0 => {
                    if *column_cursor == source_columns.len() {
                        *phase = 1;
                    } else if let Some(value) = string.step(&source_columns[*column_cursor])? {
                        columns.push(value);
                        *column_cursor += 1;
                    }
                    Ok(false)
                }
                1 => {
                    if *cell_row_cursor == source_rows.len() {
                        *phase = 3;
                    } else {
                        rows.push(Vec::new());
                        *cell_cursor = 0;
                        *phase = 2;
                    }
                    Ok(false)
                }
                2 => {
                    let source_row = &source_rows[*cell_row_cursor];
                    if *cell_cursor == source_row.len() {
                        *cell_row_cursor += 1;
                        *phase = 1;
                    } else if let Some(content) = string.step(&source_row[*cell_cursor].content)? {
                        rows.last_mut().ok_or_else(|| "Note table row owner is absent".to_string())?.push(crate::artifacts::note::NoteTableCell { content });
                        *cell_cursor += 1;
                    }
                    Ok(false)
                }
                _ => Ok(true),
            },
            (
                NoteBlockPayloadMaterialization::Math { tex, materialized_tex },
                crate::artifacts::note::NoteBlockNode::Math { tex: source_tex, .. },
            ) => {
                if materialized_tex.is_none() {
                    *materialized_tex = tex.step(source_tex)?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            (
                NoteBlockPayloadMaterialization::Ink { point_cursor, points },
                crate::artifacts::note::NoteBlockNode::Ink { points: source_points, .. },
            ) => {
                if *point_cursor == source_points.len() {
                    Ok(true)
                } else {
                    points.push(source_points[*point_cursor]);
                    *point_cursor += 1;
                    Ok(false)
                }
            }
            (
                NoteBlockPayloadMaterialization::Group { child_cursor, active, children },
                crate::artifacts::note::NoteBlockNode::Group { children: source_children, .. },
            ) => {
                if *child_cursor == source_children.len() {
                    return Ok(true);
                }
                if active.is_none() {
                    *active = Some(Box::new(NoteBlockMaterializationCursor::new(&source_children[*child_cursor])));
                    return Ok(false);
                }
                if let Some(child) = active.as_mut().ok_or_else(|| "Note group child cursor is absent".to_string())?.step(&source_children[*child_cursor])? {
                    children.push(child);
                    *active = None;
                    *child_cursor += 1;
                }
                Ok(false)
            }
            _ => Err("Note block payload variant changed beneath its operation-owned cursor".into()),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for value in [self.id.take(), self.name.take()].into_iter().flatten() {
            self.retirement.push_string(value);
        }
        match &mut self.payload {
            NoteBlockPayloadMaterialization::Text { content, materialized_content, materialized_font_weight, materialized_align, .. } => {
                content.begin_close();
                if let Some(value) = materialized_content.take() {
                    self.retirement.push_text_child(value);
                }
                self.retirement.push_optional_string(materialized_font_weight.take());
                self.retirement.push_optional_string(materialized_align.take());
            }
            NoteBlockPayloadMaterialization::Image { materialized_image_key, .. } => self.retirement.push_optional_string(materialized_image_key.take()),
            NoteBlockPayloadMaterialization::Table { columns, rows, .. } => {
                self.retirement.string_lists.push(std::mem::take(columns));
                self.retirement.table_row_lists.push(std::mem::take(rows));
            }
            NoteBlockPayloadMaterialization::Math { materialized_tex, .. } => self.retirement.push_optional_string(materialized_tex.take()),
            NoteBlockPayloadMaterialization::Ink { points, .. } => self.retirement.point_lists.push(std::mem::take(points)),
            NoteBlockPayloadMaterialization::Group { active, children, .. } => {
                if let Some(child) = active.as_mut() {
                    child.begin_close();
                }
                self.retirement.block_lists.push(std::mem::take(children));
            }
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if !self.string.terminal_is_empty() {
            let released = self.string.close_step(maximum_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
        }
        match &mut self.payload {
            NoteBlockPayloadMaterialization::Text { content, font_weight, align, .. } => {
                if !content.terminal_is_empty() {
                    return content.close_step(maximum_items, maximum_bytes);
                }
                for cursor in [font_weight, align] {
                    if !cursor.terminal_is_empty() {
                        let released = cursor.close_step(maximum_bytes);
                        return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                    }
                }
            }
            NoteBlockPayloadMaterialization::Image { image_key, .. } => {
                if !image_key.terminal_is_empty() {
                    let released = image_key.close_step(maximum_bytes);
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                }
            }
            NoteBlockPayloadMaterialization::Table { string, .. } => {
                if !string.terminal_is_empty() {
                    let released = string.close_step(maximum_bytes);
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                }
            }
            NoteBlockPayloadMaterialization::Math { tex, .. } => {
                if !tex.terminal_is_empty() {
                    let released = tex.close_step(maximum_bytes);
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                }
            }
            NoteBlockPayloadMaterialization::Ink { .. } => {}
            NoteBlockPayloadMaterialization::Group { active, .. } => {
                if let Some(child) = active.as_mut() {
                    let step = child.close_step(maximum_items, maximum_bytes);
                    if child.terminal_is_empty() {
                        *active = None;
                    }
                    return step;
                }
            }
        }
        self.retirement.close_step(maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        let payload_empty = match &self.payload {
            NoteBlockPayloadMaterialization::Text {
                content,
                materialized_content,
                font_weight,
                materialized_font_weight,
                align,
                materialized_align,
                ..
            } => {
                content.terminal_is_empty()
                    && materialized_content.is_none()
                    && font_weight.terminal_is_empty()
                    && materialized_font_weight.is_none()
                    && align.terminal_is_empty()
                    && materialized_align.is_none()
            }
            NoteBlockPayloadMaterialization::Image { image_key, materialized_image_key } => image_key.terminal_is_empty() && materialized_image_key.is_none(),
            NoteBlockPayloadMaterialization::Table { string, columns, rows, .. } => string.terminal_is_empty() && columns.is_empty() && rows.is_empty(),
            NoteBlockPayloadMaterialization::Math { tex, materialized_tex } => tex.terminal_is_empty() && materialized_tex.is_none(),
            NoteBlockPayloadMaterialization::Ink { points, .. } => points.is_empty(),
            NoteBlockPayloadMaterialization::Group { active, children, .. } => active.is_none() && children.is_empty(),
        };
        self.closing && self.string.terminal_is_empty() && self.id.is_none() && self.name.is_none() && payload_empty && self.retirement.terminal_is_empty()
    }
}

enum NoteLinkPinMaterialization {
    Head,
    Checkpoint {
        cursor: NoteStringMaterializationCursor,
        id: Option<String>,
    },
    Snapshot {
        phase: u8,
        hash: NoteStringMaterializationCursor,
        materialized_hash: Option<String>,
        media_type: NoteStringMaterializationCursor,
        materialized_media_type: Option<String>,
        size: u64,
    },
}

struct NoteArtifactLinkMaterializationCursor {
    phase: u8,
    string: NoteStringMaterializationCursor,
    artifact_id: Option<String>,
    artifact_kind: Option<String>,
    standard: Option<String>,
    subset: Option<String>,
    role: Option<String>,
    pin: NoteLinkPinMaterialization,
    retirement: NoteOwnedRetirement,
    closing: bool,
}

impl NoteArtifactLinkMaterializationCursor {
    fn new(source: &store::ArtifactLink) -> Self {
        let pin = match &source.pin {
            store::LinkPin::Head => NoteLinkPinMaterialization::Head,
            store::LinkPin::Checkpoint { .. } => NoteLinkPinMaterialization::Checkpoint { cursor: NoteStringMaterializationCursor::default(), id: None },
            store::LinkPin::Snapshot { blob } => NoteLinkPinMaterialization::Snapshot {
                phase: 0,
                hash: NoteStringMaterializationCursor::default(),
                materialized_hash: None,
                media_type: NoteStringMaterializationCursor::default(),
                materialized_media_type: None,
                size: blob.size,
            },
        };
        Self {
            phase: 0,
            string: NoteStringMaterializationCursor::default(),
            artifact_id: None,
            artifact_kind: None,
            standard: None,
            subset: None,
            role: None,
            pin,
            retirement: NoteOwnedRetirement::default(),
            closing: false,
        }
    }

    fn step(&mut self, source: &store::ArtifactLink) -> Result<Option<store::ArtifactLink>, String> {
        if self.closing {
            return Err("Note artifact-link materialization was stepped after cancellation".into());
        }
        let source_string = match self.phase {
            0 => Some(source.target.artifact_id.as_str()),
            1 => Some(source.target.dialect.artifact_kind.as_str()),
            2 => Some(source.target.dialect.standard.as_str()),
            3 => Some(source.target.dialect.subset.as_str()),
            4 => Some(source.role.as_str()),
            _ => None,
        };
        if let Some(source_string) = source_string {
            if let Some(value) = self.string.step(source_string)? {
                match self.phase {
                    0 => self.artifact_id = Some(value),
                    1 => self.artifact_kind = Some(value),
                    2 => self.standard = Some(value),
                    3 => self.subset = Some(value),
                    4 => self.role = Some(value),
                    _ => {}
                }
                self.phase += 1;
            }
            return Ok(None);
        }
        if self.phase == 5 {
            if !Self::step_pin(&mut self.pin, &source.pin)? {
                return Ok(None);
            }
            self.phase = 6;
            return Ok(None);
        }
        let pin = match &mut self.pin {
            NoteLinkPinMaterialization::Head => store::LinkPin::Head,
            NoteLinkPinMaterialization::Checkpoint { id, .. } => {
                store::LinkPin::Checkpoint { id: id.take().ok_or_else(|| "Note link checkpoint owner is absent".to_string())? }
            }
            NoteLinkPinMaterialization::Snapshot { materialized_hash, materialized_media_type, size, .. } => store::LinkPin::Snapshot {
                blob: store::BlobRef {
                    hash: materialized_hash.take().ok_or_else(|| "Note link blob hash owner is absent".to_string())?,
                    size: *size,
                    media_type: materialized_media_type.take().ok_or_else(|| "Note link blob media-type owner is absent".to_string())?,
                },
            },
        };
        Ok(Some(store::ArtifactLink {
            target: store::os_io::ArtifactRef {
                artifact_id: self.artifact_id.take().ok_or_else(|| "Note link artifact id owner is absent".to_string())?,
                dialect: store::os_io::ArtifactDialect {
                    artifact_kind: self.artifact_kind.take().ok_or_else(|| "Note link artifact kind owner is absent".to_string())?,
                    standard: self.standard.take().ok_or_else(|| "Note link standard owner is absent".to_string())?,
                    subset: self.subset.take().ok_or_else(|| "Note link subset owner is absent".to_string())?,
                },
            },
            pin,
            role: self.role.take().ok_or_else(|| "Note link role owner is absent".to_string())?,
        }))
    }

    fn step_pin(pin: &mut NoteLinkPinMaterialization, source: &store::LinkPin) -> Result<bool, String> {
        match (pin, source) {
            (NoteLinkPinMaterialization::Head, store::LinkPin::Head) => Ok(true),
            (NoteLinkPinMaterialization::Checkpoint { cursor, id }, store::LinkPin::Checkpoint { id: source_id }) => {
                if id.is_none() {
                    *id = cursor.step(source_id)?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            (
                NoteLinkPinMaterialization::Snapshot { phase, hash, materialized_hash, media_type, materialized_media_type, size },
                store::LinkPin::Snapshot { blob },
            ) => match *phase {
                0 => {
                    if let Some(value) = hash.step(&blob.hash)? {
                        *materialized_hash = Some(value);
                        *phase = 1;
                    }
                    Ok(false)
                }
                1 => {
                    if *size != blob.size {
                        return Err("Note link blob size changed beneath its operation-owned cursor".into());
                    }
                    if let Some(value) = media_type.step(&blob.media_type)? {
                        *materialized_media_type = Some(value);
                        *phase = 2;
                    }
                    Ok(false)
                }
                _ => Ok(true),
            },
            _ => Err("Note link pin variant changed beneath its operation-owned cursor".into()),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for value in [
            self.artifact_id.take(),
            self.artifact_kind.take(),
            self.standard.take(),
            self.subset.take(),
            self.role.take(),
        ]
        .into_iter()
        .flatten()
        {
            self.retirement.push_string(value);
        }
        match &mut self.pin {
            NoteLinkPinMaterialization::Head => {}
            NoteLinkPinMaterialization::Checkpoint { id, .. } => self.retirement.push_optional_string(id.take()),
            NoteLinkPinMaterialization::Snapshot { materialized_hash, materialized_media_type, .. } => {
                self.retirement.push_optional_string(materialized_hash.take());
                self.retirement.push_optional_string(materialized_media_type.take());
            }
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if !self.string.terminal_is_empty() {
            let released = self.string.close_step(maximum_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
        }
        match &mut self.pin {
            NoteLinkPinMaterialization::Head => {}
            NoteLinkPinMaterialization::Checkpoint { cursor, .. } => {
                if !cursor.terminal_is_empty() {
                    let released = cursor.close_step(maximum_bytes);
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                }
            }
            NoteLinkPinMaterialization::Snapshot { hash, media_type, .. } => {
                for cursor in [hash, media_type] {
                    if !cursor.terminal_is_empty() {
                        let released = cursor.close_step(maximum_bytes);
                        return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
                    }
                }
            }
        }
        self.retirement.close_step(maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        let pin_empty = match &self.pin {
            NoteLinkPinMaterialization::Head => true,
            NoteLinkPinMaterialization::Checkpoint { cursor, id } => cursor.terminal_is_empty() && id.is_none(),
            NoteLinkPinMaterialization::Snapshot { hash, materialized_hash, media_type, materialized_media_type, .. } => {
                hash.terminal_is_empty() && materialized_hash.is_none() && media_type.terminal_is_empty() && materialized_media_type.is_none()
            }
        };
        self.closing
            && self.string.terminal_is_empty()
            && self.artifact_id.is_none()
            && self.artifact_kind.is_none()
            && self.standard.is_none()
            && self.subset.is_none()
            && self.role.is_none()
            && pin_empty
            && self.retirement.terminal_is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteSnapshotMaterializationPhase {
    Schema,
    Id,
    Title,
    Blocks,
    AssetTraversalKey,
    AssetMime,
    AssetData,
    AssetOutputKey,
    AssetComplete,
    LinkedArtifact,
    Complete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NoteSnapshotMaterializationProgress {
    completed_units: usize,
    blocks: usize,
    assets: usize,
    phase: &'static str,
}

struct NoteSnapshotMaterializationCursor {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    phase: NoteSnapshotMaterializationPhase,
    completed_units: usize,
    string: NoteStringMaterializationCursor,
    schema: Option<String>,
    id: Option<String>,
    title: Option<String>,
    block_cursor: usize,
    active_block: Option<Box<NoteBlockMaterializationCursor>>,
    blocks: Vec<crate::artifacts::note::NoteBlockNode>,
    asset_traversal_string: NoteStringMaterializationCursor,
    last_asset_key: Option<String>,
    active_asset_key: Option<String>,
    asset_mime_string: NoteStringMaterializationCursor,
    active_asset_mime: Option<String>,
    asset_data_string: NoteStringMaterializationCursor,
    active_asset_data: Option<String>,
    asset_output_key_string: NoteStringMaterializationCursor,
    active_asset_output_key: Option<String>,
    assets: std::collections::BTreeMap<String, crate::artifacts::note::NoteImageAsset>,
    linked_artifact: Option<NoteArtifactLinkMaterializationCursor>,
    materialized_linked_artifact: Option<store::ArtifactLink>,
    retirement: NoteOwnedRetirement,
    closing: bool,
}

impl NoteSnapshotMaterializationCursor {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, source: &NoteSnapshot) -> Self {
        Self {
            operation,
            generation,
            phase: NoteSnapshotMaterializationPhase::Schema,
            completed_units: 0,
            string: NoteStringMaterializationCursor::default(),
            schema: None,
            id: None,
            title: None,
            block_cursor: 0,
            active_block: None,
            blocks: Vec::new(),
            asset_traversal_string: NoteStringMaterializationCursor::default(),
            last_asset_key: None,
            active_asset_key: None,
            asset_mime_string: NoteStringMaterializationCursor::default(),
            active_asset_mime: None,
            asset_data_string: NoteStringMaterializationCursor::default(),
            active_asset_data: None,
            asset_output_key_string: NoteStringMaterializationCursor::default(),
            active_asset_output_key: None,
            assets: std::collections::BTreeMap::new(),
            linked_artifact: source.linked_artifact.as_ref().map(NoteArtifactLinkMaterializationCursor::new),
            materialized_linked_artifact: None,
            retirement: NoteOwnedRetirement::default(),
            closing: false,
        }
    }

    fn step(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        source: &NoteSnapshot,
    ) -> Result<Option<NoteSnapshot>, String> {
        if self.closing {
            return Err("Note snapshot materialization was stepped after cancellation".into());
        }
        if operation != self.operation || generation != self.generation {
            return Err("Note snapshot materialization rejected stale operation authority".into());
        }
        self.completed_units = self.completed_units.saturating_add(1);
        match self.phase {
            NoteSnapshotMaterializationPhase::Schema => {
                if let Some(value) = self.string.step(&source.schema)? {
                    self.schema = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::Id;
                }
            }
            NoteSnapshotMaterializationPhase::Id => {
                if let Some(value) = self.string.step(&source.id)? {
                    self.id = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::Title;
                }
            }
            NoteSnapshotMaterializationPhase::Title => match source.title.as_deref() {
                Some(source_title) => {
                    if let Some(value) = self.string.step(source_title)? {
                        self.title = Some(value);
                        self.phase = NoteSnapshotMaterializationPhase::Blocks;
                    }
                }
                None => self.phase = NoteSnapshotMaterializationPhase::Blocks,
            },
            NoteSnapshotMaterializationPhase::Blocks => {
                if self.block_cursor == source.blocks.len() {
                    self.phase = NoteSnapshotMaterializationPhase::AssetTraversalKey;
                } else {
                    if self.active_block.is_none() {
                        self.active_block = Some(Box::new(NoteBlockMaterializationCursor::new(&source.blocks[self.block_cursor])));
                        return Ok(None);
                    }
                    if let Some(block) = self
                        .active_block
                        .as_mut()
                        .ok_or_else(|| "Note snapshot block cursor is absent".to_string())?
                        .step(&source.blocks[self.block_cursor])?
                    {
                        self.blocks.push(block);
                        self.active_block = None;
                        self.block_cursor += 1;
                    }
                }
            }
            NoteSnapshotMaterializationPhase::AssetTraversalKey => {
                let source_asset = match self.last_asset_key.as_ref() {
                    Some(last_key) => source
                        .assets
                        .range::<str, _>((std::ops::Bound::Excluded(last_key.as_str()), std::ops::Bound::Unbounded))
                        .next(),
                    None => source.assets.iter().next(),
                };
                let Some((source_key, _)) = source_asset else {
                    self.phase = NoteSnapshotMaterializationPhase::LinkedArtifact;
                    return Ok(None);
                };
                if let Some(value) = self.asset_traversal_string.step(source_key)? {
                    self.active_asset_key = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::AssetMime;
                }
            }
            NoteSnapshotMaterializationPhase::AssetMime => {
                let (_, source_asset) = self.active_source_asset(source)?;
                if let Some(value) = self.asset_mime_string.step(&source_asset.mime)? {
                    self.active_asset_mime = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::AssetData;
                }
            }
            NoteSnapshotMaterializationPhase::AssetData => {
                let (_, source_asset) = self.active_source_asset(source)?;
                if let Some(value) = self.asset_data_string.step(&source_asset.data)? {
                    self.active_asset_data = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::AssetOutputKey;
                }
            }
            NoteSnapshotMaterializationPhase::AssetOutputKey => {
                let (source_key, _) = self.active_source_asset(source)?;
                if let Some(value) = self.asset_output_key_string.step(source_key)? {
                    self.active_asset_output_key = Some(value);
                    self.phase = NoteSnapshotMaterializationPhase::AssetComplete;
                }
            }
            NoteSnapshotMaterializationPhase::AssetComplete => {
                let (_, source_asset) = self.active_source_asset(source)?;
                let key = self.active_asset_output_key.take().ok_or_else(|| "Note asset output key owner is absent".to_string())?;
                let asset = crate::artifacts::note::NoteImageAsset {
                    mime: self.active_asset_mime.take().ok_or_else(|| "Note asset mime owner is absent".to_string())?,
                    data: self.active_asset_data.take().ok_or_else(|| "Note asset data owner is absent".to_string())?,
                    width: source_asset.width,
                    height: source_asset.height,
                };
                if self.assets.insert(key, asset).is_some() {
                    return Err("Note snapshot materialization produced a duplicate asset key".into());
                }
                self.last_asset_key = self.active_asset_key.take();
                self.phase = NoteSnapshotMaterializationPhase::AssetTraversalKey;
            }
            NoteSnapshotMaterializationPhase::LinkedArtifact => match (&mut self.linked_artifact, &source.linked_artifact) {
                (Some(cursor), Some(source_link)) => {
                    if let Some(link) = cursor.step(source_link)? {
                        self.materialized_linked_artifact = Some(link);
                        self.phase = NoteSnapshotMaterializationPhase::Complete;
                    }
                }
                (None, None) => self.phase = NoteSnapshotMaterializationPhase::Complete,
                _ => return Err("Note linked-artifact presence changed beneath its operation-owned cursor".into()),
            },
            NoteSnapshotMaterializationPhase::Complete => {
                return Ok(Some(NoteSnapshot {
                    schema: self.schema.take().ok_or_else(|| "Note snapshot schema owner is absent".to_string())?,
                    id: self.id.take().ok_or_else(|| "Note snapshot id owner is absent".to_string())?,
                    title: self.title.take(),
                    blocks: std::mem::take(&mut self.blocks),
                    grid_visible: source.grid_visible,
                    grid_spacing: source.grid_spacing,
                    grid_subdivisions: source.grid_subdivisions,
                    grid_opacity: source.grid_opacity,
                    snap_enabled: source.snap_enabled,
                    snap_grid_spacing: source.snap_grid_spacing,
                    pencil_width: source.pencil_width,
                    eraser_radius: source.eraser_radius,
                    assets: std::mem::take(&mut self.assets),
                    linked_artifact: self.materialized_linked_artifact.take(),
                }));
            }
        }
        Ok(None)
    }

    fn active_source_asset<'a>(&self, source: &'a NoteSnapshot) -> Result<(&'a str, &'a crate::artifacts::note::NoteImageAsset), String> {
        let active_key = self.active_asset_key.as_deref().ok_or_else(|| "Note active asset traversal key owner is absent".to_string())?;
        source
            .assets
            .get_key_value(active_key)
            .map(|(key, asset)| (key.as_str(), asset))
            .ok_or_else(|| "Note active asset changed beneath its operation-owned cursor".into())
    }

    fn progress(&self) -> NoteSnapshotMaterializationProgress {
        NoteSnapshotMaterializationProgress {
            completed_units: self.completed_units,
            blocks: self.blocks.len(),
            assets: self.assets.len(),
            phase: match self.phase {
                NoteSnapshotMaterializationPhase::Schema => "schema",
                NoteSnapshotMaterializationPhase::Id => "id",
                NoteSnapshotMaterializationPhase::Title => "title",
                NoteSnapshotMaterializationPhase::Blocks => "blocks",
                NoteSnapshotMaterializationPhase::AssetTraversalKey
                | NoteSnapshotMaterializationPhase::AssetMime
                | NoteSnapshotMaterializationPhase::AssetData
                | NoteSnapshotMaterializationPhase::AssetOutputKey
                | NoteSnapshotMaterializationPhase::AssetComplete => "assets",
                NoteSnapshotMaterializationPhase::LinkedArtifact => "linkedArtifact",
                NoteSnapshotMaterializationPhase::Complete => "complete",
            },
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for value in [
            self.schema.take(),
            self.id.take(),
            self.title.take(),
            self.last_asset_key.take(),
            self.active_asset_key.take(),
            self.active_asset_mime.take(),
            self.active_asset_data.take(),
            self.active_asset_output_key.take(),
        ]
        .into_iter()
        .flatten()
        {
            self.retirement.push_string(value);
        }
        if let Some(cursor) = self.active_block.as_mut() {
            cursor.begin_close();
        }
        self.retirement.block_lists.push(std::mem::take(&mut self.blocks));
        self.retirement.asset_maps.push(std::mem::take(&mut self.assets));
        if let Some(cursor) = self.linked_artifact.as_mut() {
            cursor.begin_close();
        }
        if let Some(link) = self.materialized_linked_artifact.take() {
            self.retirement.push_link(link);
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        for cursor in [
            &mut self.string,
            &mut self.asset_traversal_string,
            &mut self.asset_mime_string,
            &mut self.asset_data_string,
            &mut self.asset_output_key_string,
        ] {
            if !cursor.terminal_is_empty() {
                let released = cursor.close_step(maximum_bytes);
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: released };
            }
        }
        if let Some(cursor) = self.active_block.as_mut() {
            let step = cursor.close_step(maximum_items, maximum_bytes);
            if cursor.terminal_is_empty() {
                self.active_block = None;
            }
            return step;
        }
        if let Some(cursor) = self.linked_artifact.as_mut() {
            let step = cursor.close_step(maximum_items, maximum_bytes);
            if cursor.terminal_is_empty() {
                self.linked_artifact = None;
            }
            return step;
        }
        self.retirement.close_step(maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.string.terminal_is_empty()
            && self.schema.is_none()
            && self.id.is_none()
            && self.title.is_none()
            && self.active_block.is_none()
            && self.blocks.is_empty()
            && self.asset_traversal_string.terminal_is_empty()
            && self.last_asset_key.is_none()
            && self.active_asset_key.is_none()
            && self.asset_mime_string.terminal_is_empty()
            && self.active_asset_mime.is_none()
            && self.asset_data_string.terminal_is_empty()
            && self.active_asset_data.is_none()
            && self.asset_output_key_string.terminal_is_empty()
            && self.active_asset_output_key.is_none()
            && self.assets.is_empty()
            && self.linked_artifact.is_none()
            && self.materialized_linked_artifact.is_none()
            && self.retirement.terminal_is_empty()
    }
}
//#endregion 🕸️TextChildMaterialization

//#region 📬️StorePreparation
const NOTE_STORE_PREPARATION_ITEMS: usize = 4;
const NOTE_STORE_PREPARATION_BYTES: usize = 262_144;

type NoteStorePrepare<P, M> = fn(&P, M) -> Result<(P, Vec<M>, M), String>;

pub struct NoteStoreOneItemPreparationFactory<P, M> {
    lane: store::HistoryLane,
    prepare: NoteStorePrepare<P, M>,
}

impl<P, M> NoteStoreOneItemPreparationFactory<P, M> {
    fn new(lane: store::HistoryLane, prepare: NoteStorePrepare<P, M>) -> Self {
        Self { lane, prepare }
    }
}

struct NoteStoreOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepare: NoteStorePrepare<P, M>,
    candidate: Option<(P, Vec<M>, M)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn note_semantic_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("note-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn prepare_note_config(base: &NoteConfig, mutation: NoteConfigMutation) -> Result<(NoteConfig, Vec<NoteConfigMutation>, NoteConfigMutation), String> {
    let mut post = base.clone();
    match &mutation {
        NoteConfigMutation::Snapshot { config } => post = config.clone(),
        NoteConfigMutation::SetEngagementInput { value } => post.engagement_input = value.clone(),
        NoteConfigMutation::SetCamera { camera } => post.camera = camera.clone(),
        NoteConfigMutation::SetActiveUtility { utility_id } => post.active_utility_id = utility_id.clone(),
        NoteConfigMutation::SetLocale { value } => post.locale = value.clone(),
    }
    Ok((post, vec![NoteConfigMutation::Snapshot { config: base.clone() }], mutation))
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for NoteStoreOneItemPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: Clone + serde::Serialize + Send + Sync + 'static,
{
    fn preflight(&self, _mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != self.lane || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Note one-item preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: NOTE_STORE_PREPARATION_ITEMS, retained_bytes: NOTE_STORE_PREPARATION_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != self.lane
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(NoteStoreOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepare: self.prepare,
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for NoteStoreOneItemPreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: Clone + serde::Serialize + Send + Sync + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Note preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Note preparation lost its mutation owner".to_string())?;
                self.candidate = Some((self.prepare)(base.get(), mutation)?);
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 0, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Note preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Note preparation lost its Store authority".to_string())?;
                let edit = note_semantic_edit(forward, inverse, self.description.take(), authority);
                let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 0, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                self.phase = 2;
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Note preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}

const NOTE_ROOT_SCALAR_STORE_MAXIMUM_UNITS: usize = 512;
const NOTE_ROOT_SCALAR_STORE_RETAINED_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;

struct NoteRootScalarPreparationFactory;

impl NoteRootScalarPreparationFactory {
    fn validate_mutation(mutation: &crate::artifacts::note::op::NoteMutation) -> Result<(), String> {
        match mutation {
            crate::artifacts::note::op::NoteMutation::ChangeGridVisible(_) => Ok(()),
            crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(payload) => {
                if let Some(spacing) = payload.new_spacing {
                    if !spacing.is_finite() || spacing <= 0.0 {
                        return Err(format!("Grid spacing must be a positive number, got {spacing}."));
                    }
                }
                Ok(())
            }
            _ => Err("Note one-item Artifact preparation admits only exact retained root-scalar mutations on the document lane".into()),
        }
    }
}

#[derive(Clone, Copy)]
enum NoteRootScalarDiff {
    GridVisible(Option<bool>),
    GridSpacing(Option<f64>),
}

struct NoteRootScalarPreparation {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: [u8; 32],
    base: Option<store::SnapshotRead<NoteSnapshot>>,
    forward: Option<crate::artifacts::note::op::NoteMutation>,
    inverse: Option<Vec<crate::artifacts::note::op::NoteMutation>>,
    diff: Option<NoteRootScalarDiff>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    materialization: NoteSnapshotMaterializationCursor,
    post: Option<NoteSnapshot>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<NoteSnapshot, crate::artifacts::note::op::NoteMutation>>,
    retirement: NoteOwnedRetirement,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    completed_items: u32,
    cancelled: bool,
    closing: bool,
}

impl NoteRootScalarPreparation {
    fn progress(&mut self, cursor: u32) -> store::ArtifactStoreOneItemPreparationStep {
        self.completed_items = self.completed_items.saturating_add(1);
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint {
            cursor,
            completed_items: self.completed_items,
            completed_bytes: u64::from(self.completed_items).saturating_mul(2_048),
            digest: [0; 32],
        };
        store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }

    fn live_authority(&self) -> Result<&store::ArtifactStoreOneItemLiveAuthority, String> {
        let authority = self.authority.as_deref().ok_or_else(|| "Note root-scalar preparation lost its Store authority".to_string())?;
        if authority.operation() != self.operation || authority.generation() != self.generation || authority.base_revision() != self.base_revision {
            return Err("Note root-scalar preparation rejected stale Store authority".into());
        }
        Ok(authority)
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<NoteSnapshot, crate::artifacts::note::op::NoteMutation> for NoteRootScalarPreparationFactory {
    fn preflight(
        &self,
        mutation: &crate::artifacts::note::op::NoteMutation,
        description: Option<&str>,
        lane: store::HistoryLane,
    ) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Note retained root-scalar preparation rejected its lane or description envelope".into());
        }
        Self::validate_mutation(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint {
            work_items: NOTE_ROOT_SCALAR_STORE_MAXIMUM_UNITS,
            retained_bytes: NOTE_ROOT_SCALAR_STORE_RETAINED_BYTES,
        })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<NoteSnapshot, crate::artifacts::note::op::NoteMutation>,
    ) -> Result<
        Box<dyn store::ArtifactStoreOneItemPreparation<NoteSnapshot, crate::artifacts::note::op::NoteMutation>>,
        store::ArtifactStoreOneItemPreparationRequest<NoteSnapshot, crate::artifacts::note::op::NoteMutation>,
    > {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().is_empty()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || Self::validate_mutation(&request.mutation).is_err()
        {
            return Err(request);
        }
        let materialization = NoteSnapshotMaterializationCursor::new(request.operation, request.generation, request.base.get());
        Ok(Box::new(NoteRootScalarPreparation {
            operation: request.operation,
            generation: request.generation,
            base_revision: request.base_revision,
            base: Some(request.base),
            forward: Some(request.mutation),
            inverse: None,
            diff: None,
            description: request.description,
            authority: Some(request.authority),
            materialization,
            post: None,
            prepared: None,
            retirement: NoteOwnedRetirement::default(),
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            completed_items: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<NoteSnapshot, crate::artifacts::note::op::NoteMutation> for NoteRootScalarPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        self.live_authority()?;
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                NoteRootScalarPreparationFactory::validate_mutation(
                    self.forward.as_ref().ok_or_else(|| "Note root-scalar validation lost its forward owner".to_string())?,
                )?;
                self.phase = 1;
                Ok(self.progress(1))
            }
            1 => {
                let base = self.base.as_ref().ok_or_else(|| "Note root-scalar inverse lost its base root".to_string())?.get();
                self.inverse = Some(vec![match self.forward.as_ref().ok_or_else(|| "Note root-scalar inverse lost its forward owner".to_string())? {
                    crate::artifacts::note::op::NoteMutation::ChangeGridVisible(_) => crate::artifacts::note::op::NoteMutation::ChangeGridVisible(
                        crate::artifacts::note::schema::mutations::ChangeGridVisible { new_visible: base.grid_visible },
                    ),
                    crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(_) => crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(
                        crate::artifacts::note::schema::mutations::ChangeGridSpacing { new_spacing: base.grid_spacing },
                    ),
                    _ => return Err("Note root-scalar inverse rejected a non-admitted mutation".into()),
                }]);
                self.phase = 2;
                Ok(self.progress(2))
            }
            2 => {
                self.diff = Some(match self.forward.as_ref().ok_or_else(|| "Note root-scalar diff lost its forward owner".to_string())? {
                    crate::artifacts::note::op::NoteMutation::ChangeGridVisible(payload) => NoteRootScalarDiff::GridVisible(payload.new_visible),
                    crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(payload) => {
                        if let Some(spacing) = payload.new_spacing {
                            if !spacing.is_finite() || spacing <= 0.0 {
                                return Err(format!("Grid spacing must be a positive number, got {spacing}."));
                            }
                        }
                        NoteRootScalarDiff::GridSpacing(payload.new_spacing)
                    }
                    _ => return Err("Note root-scalar diff rejected a non-admitted mutation".into()),
                });
                self.phase = 3;
                Ok(self.progress(3))
            }
            3 => {
                if self.materialization.progress().completed_units >= NOTE_ROOT_SCALAR_STORE_MAXIMUM_UNITS.saturating_sub(8) {
                    return Err("Note root-scalar post-root exceeds its admitted bounded materialization footprint".into());
                }
                let base = self.base.as_ref().ok_or_else(|| "Note root-scalar materialization lost its base root".to_string())?;
                if let Some(post) = self.materialization.step(self.operation, self.generation, base.get())? {
                    self.post = Some(post);
                    self.phase = 4;
                }
                Ok(self.progress(4))
            }
            4 => {
                let post = self.post.as_mut().ok_or_else(|| "Note root-scalar apply lost its materialized post root".to_string())?;
                match self.diff.take().ok_or_else(|| "Note root-scalar apply lost its exact sparse diff".to_string())? {
                    NoteRootScalarDiff::GridVisible(value) => post.grid_visible = value,
                    NoteRootScalarDiff::GridSpacing(value) => post.grid_spacing = value,
                }
                self.phase = 5;
                Ok(self.progress(5))
            }
            5 => {
                self.live_authority()?;
                let authority = std::sync::Arc::clone(
                    self.authority.as_ref().ok_or_else(|| "Note root-scalar seal lost its Store authority".to_string())?,
                );
                let forward = self.forward.take().ok_or_else(|| "Note root-scalar seal lost its forward owner".to_string())?;
                let inverse = self.inverse.take().ok_or_else(|| "Note root-scalar seal lost its inverse owner".to_string())?;
                let edit = note_semantic_edit(forward, inverse, self.description.take(), &authority);
                let post = self.post.take().ok_or_else(|| "Note root-scalar seal lost its post root".to_string())?;
                let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
                self.completed_items = self.completed_items.saturating_add(1);
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint {
                    cursor: 6,
                    completed_items: self.completed_items,
                    completed_bytes: u64::from(self.completed_items).saturating_mul(2_048),
                    digest: prepared.edit_digest(),
                };
                self.prepared = Some(prepared);
                self.phase = 6;
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<NoteSnapshot, crate::artifacts::note::op::NoteMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<NoteSnapshot, crate::artifacts::note::op::NoteMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.materialization.begin_close();
        if let Some(post) = self.post.take() {
            self.retirement.push_snapshot(post);
        }
        if let Some(description) = self.description.take() {
            self.retirement.push_string(description);
        }
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if !self.materialization.terminal_is_empty() {
            return Ok(match self.materialization.close_step(1, grant.maximum_bytes) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes }
                }
                InteractiveJobCloseStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 },
                InteractiveJobCloseStep::Blocked => store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 },
            });
        }
        if !self.retirement.terminal_is_empty() {
            return Ok(match self.retirement.close_step(grant.maximum_bytes) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes }
                }
                InteractiveJobCloseStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 },
                InteractiveJobCloseStep::Blocked => store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 },
            });
        }
        if self.prepared.take().is_some() || self.forward.take().is_some() || self.inverse.take().is_some() || self.diff.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Note root-scalar preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.base.is_none()
            && self.forward.is_none()
            && self.inverse.is_none()
            && self.diff.is_none()
            && self.description.is_none()
            && self.authority.is_none()
            && self.materialization.terminal_is_empty()
            && self.post.is_none()
            && self.prepared.is_none()
            && self.retirement.terminal_is_empty()
    }
}

pub fn artifact_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<NoteSnapshot, crate::artifacts::note::op::NoteMutation>> {
    std::sync::Arc::new(NoteRootScalarPreparationFactory)
}

pub fn config_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<NoteConfig, NoteConfigMutation>> {
    std::sync::Arc::new(NoteStoreOneItemPreparationFactory::new(store::HistoryLane::Document, prepare_note_config))
}
//#endregion 📬️StorePreparation

//#region 🧪️MaterializationTests
#[cfg(test)]
mod materialization_tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

    fn text_child(owner: Option<std::sync::Arc<SemioTextSnapshot>>, child_id: String) -> crate::artifacts::note::NoteTextChild {
        let target = store::os_io::ArtifactRef {
            artifact_id: "note-text-artifact".into(),
            dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() },
        };
        let handle = match owner {
            Some(owner) => store::ArtifactChild::new(child_id, target).with_local_owner(owner),
            None => store::ArtifactChild::new(child_id, target),
        };
        crate::artifacts::note::NoteTextChild { handle, paragraphs: Vec::new() }
    }

    fn materialize(source: &crate::artifacts::note::NoteTextChild) -> store::ArtifactChild<SemioTextSnapshot> {
        let mut cursor = NoteTextChildMaterializationCursor::new(source);
        for _ in 0..64 {
            if let Some(child) = cursor.step(source).expect("bounded child metadata copy") {
                return child;
            }
        }
        panic!("Note text-child materializer did not terminate")
    }

    #[semio_framework_async_macros::async_test]
    async fn text_child_materialization_preserves_present_typed_owner() {
        let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
        let source = text_child(Some(owner.clone()), "child-present".into());
        let materialized = materialize(&source);
        let retained = materialized.local_owner::<SemioTextSnapshot>().expect("typed owner retained");
        assert!(std::sync::Arc::ptr_eq(&owner, &retained));
        assert_eq!(materialized.child_id, source.handle.child_id);
        assert_eq!(materialized.target, source.handle.target);
    }

    #[semio_framework_async_macros::async_test]
    async fn text_child_materialization_preserves_absent_owner() {
        let source = text_child(None, "child-absent".into());
        let materialized = materialize(&source);
        assert!(materialized.local_owner::<SemioTextSnapshot>().is_none());
        assert_eq!(materialized.child_id, source.handle.child_id);
        assert_eq!(materialized.target, source.handle.target);
    }

    #[semio_framework_async_macros::async_test]
    async fn text_child_materialization_cancellation_retires_partial_metadata() {
        let source = text_child(None, "x".repeat(NOTE_MATERIALIZATION_STRING_CHUNK_BYTES * 3));
        let mut cursor = NoteTextChildMaterializationCursor::new(&source);
        assert!(cursor.step(&source).expect("first bounded metadata chunk").is_none());
        cursor.begin_close();
        for _ in 0..8 {
            if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
                break;
            }
        }
        assert!(cursor.terminal_is_empty());
    }

    fn hostile_snapshot(owner: std::sync::Arc<SemioTextSnapshot>) -> NoteSnapshot {
        let text = crate::artifacts::note::NoteBlockNode::Text {
            id: "text-1".into(),
            name: "Text".into(),
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
            rotation: 5.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::NoteTextChild {
                handle: text_child(Some(owner), "child-root".into()).handle,
                paragraphs: vec![crate::artifacts::note::NoteTextParagraph {
                    runs: vec![crate::artifacts::note::NoteTextRun {
                        text: "root text".into(),
                        bold: Some(true),
                        italic: Some(false),
                        underline: None,
                        link: Some("https://semio.tech".into()),
                    }],
                }],
            },
            font_size: 16.0,
            font_weight: "600".into(),
            align: "center".into(),
        };
        let table = crate::artifacts::note::NoteBlockNode::Table {
            id: "table-1".into(),
            name: "Table".into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["a".into(), "b".into()],
            rows: vec![vec![
                crate::artifacts::note::NoteTableCell { content: "one".into() },
                crate::artifacts::note::NoteTableCell { content: "two".into() },
            ]],
        };
        let group = crate::artifacts::note::NoteBlockNode::Group {
            id: "group-1".into(),
            name: "Group".into(),
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            children: vec![
                crate::artifacts::note::NoteBlockNode::Image {
                    id: "image-1".into(),
                    name: "Image".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    image_key: "asset-1".into(),
                },
                crate::artifacts::note::NoteBlockNode::Math {
                    id: "math-1".into(),
                    name: "Math".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    tex: "x^2".into(),
                    display_mode: true,
                },
                crate::artifacts::note::NoteBlockNode::Ink {
                    id: "ink-1".into(),
                    name: "Ink".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    points: vec![[0.0, 1.0], [2.0, 3.0]],
                    stroke_width: 2.0,
                    color: [0.1, 0.2, 0.3, 1.0],
                },
            ],
        };
        let mut assets = std::collections::BTreeMap::new();
        assets.insert(
            "asset-1".into(),
            crate::artifacts::note::NoteImageAsset {
                mime: "image/png".into(),
                data: "data:image/png;base64,abc".into(),
                width: Some(10.0),
                height: Some(20.0),
            },
        );
        NoteSnapshot {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "root".into(),
            title: Some("Hostile root".into()),
            blocks: vec![text, table, group],
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: Some(4.0),
            grid_opacity: Some(0.35),
            snap_enabled: Some(true),
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: Some(12.0),
            assets,
            linked_artifact: Some(store::ArtifactLink {
                target: store::os_io::ArtifactRef {
                    artifact_id: "linked".into(),
                    dialect: store::os_io::ArtifactDialect { artifact_kind: "s.any".into(), standard: "1".into(), subset: "any".into() },
                },
                pin: store::LinkPin::Snapshot {
                    blob: store::BlobRef { hash: "blob-hash".into(), size: 9, media_type: "application/octet-stream".into() },
                },
                role: "any".into(),
            }),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc() {
        let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
        let source = hostile_snapshot(owner.clone());
        let operation = semio_framework_job::OperationId(31);
        let generation = semio_framework_job::Generation(7);
        let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
        let materialized = (0..4_096)
            .find_map(|_| cursor.step(operation, generation, &source).expect("bounded nested snapshot copy"))
            .expect("nested snapshot cursor terminates");
        assert_eq!(materialized, source);
        assert_eq!(cursor.progress().phase, "complete");
        let crate::artifacts::note::NoteBlockNode::Text { content, .. } = &materialized.blocks[0] else {
            panic!("first hostile block must remain text")
        };
        let retained = content.handle.local_owner::<SemioTextSnapshot>().expect("nested typed owner retained");
        assert!(std::sync::Arc::ptr_eq(&owner, &retained));
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_materialization_preserves_absent_typed_owner() {
        let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
        let mut source = hostile_snapshot(owner);
        let crate::artifacts::note::NoteBlockNode::Text { content, .. } = &mut source.blocks[0] else {
            panic!("first hostile block must remain text")
        };
        *content = text_child(None, "child-wire-only".into());
        let operation = semio_framework_job::OperationId(32);
        let generation = semio_framework_job::Generation(8);
        let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
        let materialized = (0..4_096)
            .find_map(|_| cursor.step(operation, generation, &source).expect("bounded wire-only snapshot copy"))
            .expect("wire-only snapshot cursor terminates");
        let crate::artifacts::note::NoteBlockNode::Text { content, .. } = &materialized.blocks[0] else {
            panic!("first hostile block must remain text")
        };
        assert!(content.handle.local_owner::<SemioTextSnapshot>().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness() {
        let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
        let mut source = hostile_snapshot(owner);
        let crate::artifacts::note::NoteBlockNode::Text { content, .. } = &mut source.blocks[0] else {
            panic!("first hostile block must remain text")
        };
        content.handle.child_id = "metadata".repeat(NOTE_MATERIALIZATION_STRING_CHUNK_BYTES);
        let operation = semio_framework_job::OperationId(33);
        let generation = semio_framework_job::Generation(9);
        let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
        for _ in 0..12 {
            assert!(cursor.step(operation, generation, &source).expect("bounded metadata advance").is_none());
        }
        cursor.begin_close();
        for _ in 0..32_768 {
            if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
                break;
            }
        }
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_materialization_rejects_stale_operation_authority_and_retires() {
        let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
        let source = hostile_snapshot(owner);
        let operation = semio_framework_job::OperationId(34);
        let generation = semio_framework_job::Generation(10);
        let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
        assert!(cursor.step(semio_framework_job::OperationId(35), generation, &source).is_err());
        cursor.begin_close();
        for _ in 0..32_768 {
            if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
                break;
            }
        }
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn root_scalar_preflight_admits_only_exact_valid_document_mutations() {
        let factory = NoteRootScalarPreparationFactory;
        let visible = crate::artifacts::note::op::NoteMutation::ChangeGridVisible(
            crate::artifacts::note::schema::mutations::ChangeGridVisible { new_visible: Some(false) },
        );
        let spacing = crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(
            crate::artifacts::note::schema::mutations::ChangeGridSpacing { new_spacing: Some(16.0) },
        );
        let invalid_spacing = crate::artifacts::note::op::NoteMutation::ChangeGridSpacing(
            crate::artifacts::note::schema::mutations::ChangeGridSpacing { new_spacing: Some(f64::INFINITY) },
        );
        let foreign = crate::artifacts::note::op::NoteMutation::RenameNote(
            crate::artifacts::note::schema::mutations::RenameNote { new_title: Some("foreign".into()) },
        );
        assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &visible, None, store::HistoryLane::Document).is_ok());
        assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &spacing, None, store::HistoryLane::Document).is_ok());
        assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &invalid_spacing, None, store::HistoryLane::Document).is_err());
        assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &visible, None, store::HistoryLane::Interaction).is_err());
        assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &foreign, None, store::HistoryLane::Document).is_err());
    }
}
//#endregion 🧪️MaterializationTests
