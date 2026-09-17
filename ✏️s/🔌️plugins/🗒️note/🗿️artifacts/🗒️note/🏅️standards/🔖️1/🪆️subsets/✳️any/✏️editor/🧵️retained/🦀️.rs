//! 🧵️ Note-owned retained command microstate and exact publication contracts.

use crate::editor::note::commands::{ink_apply_events, patch_blocks};
use crate::editor::note::{NoteCommand, NoteDispatchCtx, NotePlayApp, NOTE_INTERACTION_BLOCKS};
use crate::schema::NoteIdOwner;
use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, EditorApp, Emit, EphemeralEmit, Fault, FaultCode, FaultOrigin};

//#region 🔖️Contract
pub const NOTE_RETAINED_PAYLOAD_SCHEMA: &str = "semio.note.retained-command.v1";
pub const NOTE_RETAINED_RAW_BYTES: usize = 65_536;
pub const NOTE_RETAINED_MAXIMUM_UNITS: usize = 4_096;
pub const NOTE_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 16_384;

/// 🧵️ Every note verb is a retained tool (ticket 26/09/17/NOTE-PLUGIN-END-TO-END): the framework refuses UI
/// dispatch of any command not classified `Migrated`, which left every document verb dead in the shell.
pub const NOTE_RETAINED_TOOL_IDS: &[&str] = &[
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
    "engagementInput",
    "navigatorEngagementInput",
    "saveDownload",
    "loadRequest",
];

pub const NOTE_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
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
    ArtifactToolPublicationContract { tool_id: "inkApplyEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionUp", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionDown", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeft", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionRight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionUpFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionDownFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeftFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nudgeSelectionRightFast", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "navigatorEngagementInput", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveDownload", lanes: &[ArtifactToolPublicationLane::HostOnly] },
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
    interaction.selection.get(NOTE_INTERACTION_BLOCKS).into_iter().flat_map(|selection| selection.ids.iter()).filter_map(|id| crate::schema::block_id_from_tree_row_id(id)).collect()
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

fn note_command_units(command: &NoteCommand, selected: &[String]) -> Vec<NoteCommandUnit> {
    if let Some(units) = selection_units(command, selected) {
        return units;
    }
    match command {
        NoteCommand::PatchBlocks(payload) => {
            if payload.block_ids.is_empty() {
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
            }
        }
        NoteCommand::InkApplyEvents(payload) => {
            let events = serde_json::from_str::<serde_json::Value>(&payload.events_json).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default();
            if events.is_empty() {
                vec![NoteCommandUnit { command: command.clone(), selected_block_ids: selected.to_vec() }]
            } else {
                events
                    .into_iter()
                    .map(|event| NoteCommandUnit {
                        command: NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: serde_json::Value::Array(vec![event]).to_string(), phase: payload.phase.clone(), select_ids: payload.select_ids.clone() }),
                        selected_block_ids: selected.to_vec(),
                    })
                    .collect()
            }
        }
        _ => vec![NoteCommandUnit { command: command.clone(), selected_block_ids: selected.to_vec() }],
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
    accumulated: Emit<crate::op::NoteMutation, semio_framework_plugin::NoConfigMutation>,
    ephemeral: EphemeralEmit<EditorApp<NotePlayApp>>,
    id_owner: Option<NoteIdOwner>,
    workspace_identity: u64,
    complete: bool,
    closing: bool,
}

impl NoteCommandWork {
    fn new(tool_id: &'static str, command: &NoteCommand, _snapshot: &NoteSnapshot, interaction: &protocol::InteractionState, operation: &AppOperationContext) -> Result<Self, Fault> {
        let units = note_command_units(command, &selected_block_ids(interaction));
        if units.is_empty() || units.len() > NOTE_RETAINED_MAXIMUM_UNITS {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.extent"), "Note command exceeds its fixed semantic-unit envelope"));
        }
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Ok(Self { tool_id, units, cursor: 0, replay_target: None, projection: None, accumulated: Emit::default(), ephemeral: EphemeralEmit::default(), id_owner: Some(NoteIdOwner::new(scope, 0)), workspace_identity, complete: false, closing: false })
    }

    fn append(&mut self, mut emit: Emit<crate::op::NoteMutation, semio_framework_plugin::NoConfigMutation>) -> Result<(), Fault> {
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
        self.accumulated.window_config_mutations.append(&mut emit.window_config_mutations);
        self.accumulated.draft_mutations.append(&mut emit.draft_mutations);
        self.accumulated.effects.append(&mut emit.effects);
        self.accumulated.events.append(&mut emit.events);
        self.accumulated.child_emits.append(&mut emit.child_emits);
        self.accumulated.interaction_writes.append(&mut emit.interaction_writes);
        Ok(())
    }

    fn release_one(&mut self) -> bool {
        if self.units.pop().is_some() {
            return true;
        }
        if self.accumulated.artifact_mutations.pop().is_some()
            || self.accumulated.config_mutations.pop().is_some()
            || self.accumulated.window_config_mutations.pop().is_some()
            || self.accumulated.draft_mutations.pop().is_some()
            || self.accumulated.effects.pop().is_some()
            || self.accumulated.events.pop().is_some()
            || self.accumulated.child_emits.pop().is_some()
            || self.accumulated.interaction_writes.pop().is_some()
            || self.ephemeral.presence.pop().is_some()
            || self.ephemeral.transient.pop().is_some()
            || self.ephemeral.window_transient.pop().is_some()
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

    fn extent(&self, _command: &NoteCommand, _snapshot: &NoteSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<NotePlayApp>>>) -> Option<usize> {
        Some(self.units.len())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<NotePlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<NotePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config, history, interaction: _interaction, hover: _hover, context, operation } = *input;
        if self.complete || self.cursor >= self.units.len() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.repeated"), "Note retained work was stepped after completion"));
        }
        let unit = &self.units[self.cursor];
        let projection = self.projection.as_ref().unwrap_or(snapshot);
        let id_owner = self.id_owner.as_mut().ok_or_else(|| Fault::from("note-retained-id-owner-missing"))?;
        let window_owner = context.and_then(|context| context.window_transient.clone());
        let window_before = crate::editor::note::window::transient_from_snapshot(window_owner.as_ref());
        let mut ctx = NoteDispatchCtx {
            selected_block_ids: unit.selected_block_ids.clone(),
            id_owner: id_owner.clone(),
            view_state: context.and_then(|context| context.view_state.clone()),
            window_transient: window_before.clone(),
            window_transient_owner: window_owner,
        };
        let emit = unit.command.dispatch(
            &ArtifactView::with_operation(projection, history, operation.clone()),
            &ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) },
            &mut ctx,
        )?;
        if ctx.window_transient != window_before {
            let owner = ctx.window_transient_owner.as_ref().ok_or_else(|| Fault::from("note-composite-window-transient-owner-required"))?;
            self.ephemeral.window_transient.push(crate::editor::note::window::addressed_transient(owner, ctx.window_transient)?);
        }
        *id_owner = ctx.id_owner;
        if self.cursor + 1 < self.units.len() {
            for mutation in &emit.artifact_mutations {
                let current = self.projection.as_ref().unwrap_or(snapshot);
                self.projection = Some(crate::schema::mutations::apply_note_mutation(current, mutation).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("note.retained.projection"), error.to_string()))?);
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
        let emit = std::mem::take(&mut self.accumulated);
        if self.ephemeral.presence.is_empty() && self.ephemeral.transient.is_empty() && self.ephemeral.window_transient.is_empty() {
            Ok(ArtifactCommandWorkStep::Complete(emit))
        } else {
            Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: std::mem::take(&mut self.ephemeral) })
        }
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
        self.ephemeral = EphemeralEmit::default();
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
            && self.accumulated.window_config_mutations.is_empty()
            && self.accumulated.draft_mutations.is_empty()
            && self.accumulated.effects.is_empty()
            && self.accumulated.events.is_empty()
            && self.accumulated.child_emits.is_empty()
            && self.accumulated.interaction_writes.is_empty()
            && self.ephemeral.presence.is_empty()
            && self.ephemeral.transient.is_empty()
            && self.ephemeral.window_transient.is_empty()
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

pub fn build(request: ArtifactOwnedToolJobRequest<EditorApp<NotePlayApp>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
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
    let payload = ArtifactRetainedCommandPayload::try_new(
        semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        NoteCommand::command_id,
        NOTE_RETAINED_RAW_BYTES,
        NOTE_RETAINED_MAXIMUM_UNITS,
        work,
    )?;
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🏭️Factory

//#region 📬️StorePreparation
/// 📬️ Document-lane publication authority for every retained note verb (ticket 26/09/17/NOTE-PLUGIN-END-TO-END):
/// the bespoke root-scalar factory admitted only grid mutations, so `addBlock`/`inkApplyEvents`/`patchBlocks`
/// failed `admits only exact retained root-scalar mutations` in the running app.
pub fn artifact_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<NoteSnapshot, crate::op::NoteMutation>> {
    semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<NoteSnapshot, crate::op::NoteMutation>("note-artifact-retained", NOTE_ARTIFACT_MUTATION_MAXIMUM_BYTES)
}

//#endregion 📬️StorePreparation
