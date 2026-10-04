//! ✏️ `md` editor (any) — `ArtifactEditor` surface built on the frozen
//! `TextWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `replace-text` action: the incoming text is the CommonMark source the main window edits (or the artifact's own
//! DSL envelope, `print_dsl`/`parse_dsl`, when it carries the preamble); the window is an explicit draft, so one Apply is ONE edit of the net block leaves the applied text means (design §13.2 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), never a whole-document replace per delivery.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::md::modes::edit;
use crate::editor::md::modes::edit::windows::main;
use crate::standards::v_commonmark::subsets::any::schema::mutations::set_snapshot::SetSnapshot;
use crate::standards::v_commonmark::subsets::any::schema::mutations::{insert_block, remove_block, replace_block, set_inlines, MdMutation, MdPathStep};
use crate::standards::v_commonmark::subsets::any::schema::snapshot::MdBlock;
use crate::standards::v_commonmark::subsets::any::schema::snapshot::MdSnapshot;
use crate::{MD_DIALECT, STDIO_MD_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum MdEditCommand {
    ReplaceText {
        text: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpBinary for MdEditCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TextWindowKit`
    /// mints `replace-text`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = MD_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TextWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const MD_KIT_ACTION_ID: &str = "textEdit";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `replace-text`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `replace-text` with
/// `interactive-job.missing-factory`.
const MD_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, MD_KIT_ACTION_ID];
const MD_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    MD_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const MD_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.md.tool-command.v1";
/// 📏️ `replace-text` carries the whole buffer, so the wire bound is the largest document this route
/// admits — kept under the guest's 64 KiB contiguous-request ceiling.
const MD_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `replace-text` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const MD_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: MD_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(MD_RETAINED_RAW_BYTES, 4_096, 1, MD_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_example_snapshot(example_id: &str) -> MdSnapshot {
    if example_id == crate::examples::demo::ID {
        <MdSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        MdSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<MdEditCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(MdEditCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(MdEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        MD_KIT_ACTION_ID => Ok(MdEditCommand::ReplaceText { text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? }),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.md.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, replace-text)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_command_id(command: &MdEditCommand) -> &'static str {
    match command {
        MdEditCommand::ReplaceText { .. } => MD_KIT_ACTION_ID,
        MdEditCommand::EditSnapshot { event } => event.action_id(),
        MdEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_extent(_command: &MdEditCommand, _snapshot: &MdSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `replace-text` (the explicit Apply of the text window's draft) becomes the net block leaves that carry the
/// committed document to the applied text ([`md_net_mutations`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_emit(command: &MdEditCommand, snapshot: &MdSnapshot) -> Result<Emit<MdMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        MdEditCommand::ReplaceText { text } => match md_applied_text(text) {
            Ok(next) => Ok(Emit::mutations(md_net_mutations(snapshot, &next))),
            Err(error) => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.md.invalid-text"), error.to_string())),
        },
        MdEditCommand::EditSnapshot { .. } => Err(Fault::from("stdio-md-snapshot-edit-routed-to-native-reducer")),
        MdEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&md_example_snapshot(example_id), STDIO_MD_DOCUMENT_SCHEMA)], ..Default::default() }),
    }
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_reduce(
    command: &MdEditCommand,
    snapshot: &MdSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<MdEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<MdMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    md_emit(command, snapshot)
}

struct MdRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl MdRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: MD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for MdRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<MdEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<MdEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        MD_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        md_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > MD_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio md retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for MdRetainedCommandJobFactory {
    type Owner = EditorApp<MdEditor>;
    const TOOL_IDS: &'static [&'static str] = MD_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MD_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = MD_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🧮️NetLeaves
/// 📥️ The document an applied text means: the artifact's own DSL envelope when the text carries its preamble (an agent's
/// whole-document write), else the CommonMark source the main window edits.
fn md_applied_text(text: &str) -> Result<MdSnapshot, semio_framework_diagnostic::TextError> {
    match store::semio_format::split_text_preamble(text) {
        Ok(_) => <MdSnapshot as store::ArtifactDsl>::parse_dsl(text),
        Err(_) => Ok(MdSnapshot::from_text(text)),
    }
}

/// 🧮️ The net leaves of one applied text: the block edits that carry `base` to exactly `next`, in application order. Blocks
/// unchanged at either end of a container stay untouched; a changed paragraph, or a heading that keeps its level, re-sets its
/// inlines; a block quote, and a list that keeps its shape and item count, recurse into their own blocks; any other changed
/// block is replaced; surplus blocks are removed (last first) or inserted. History therefore edits the block an author
/// changed, never the whole document. The one genuine whole-document replacement is an applied DSL envelope that names
/// another document schema: it is `set-snapshot`. The main window's Apply and the document-details editor both commit
/// through here.
fn md_net_mutations(base: &MdSnapshot, next: &MdSnapshot) -> Vec<MdMutation> {
    if base.schema != next.schema {
        return vec![MdMutation::SetSnapshot(SetSnapshot { snapshot: next.clone() })];
    }
    let mut leaves = Vec::new();
    md_net_blocks(&[], &base.blocks, &next.blocks, &mut leaves);
    leaves
}

fn md_net_blocks(path: &[MdPathStep], old: &[MdBlock], new: &[MdBlock], leaves: &mut Vec<MdMutation>) {
    let prefix = old.iter().zip(new).take_while(|(before, after)| before == after).count();
    let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(before, after)| before == after).count();
    let (old_middle, new_middle) = (&old[prefix..old.len() - suffix], &new[prefix..new.len() - suffix]);
    let paired = old_middle.len().min(new_middle.len());
    for (offset, (before, after)) in old_middle.iter().zip(new_middle).enumerate() {
        md_net_block(path, prefix + offset, before, after, leaves);
    }
    for offset in (paired..old_middle.len()).rev() {
        leaves.push(MdMutation::RemoveBlock(remove_block::RemoveBlock { path: path.to_vec(), index: prefix + offset }));
    }
    for (offset, block) in new_middle.iter().enumerate().skip(paired) {
        leaves.push(MdMutation::InsertBlock(insert_block::InsertBlock { path: path.to_vec(), index: prefix + offset, block: block.clone() }));
    }
}

fn md_net_block(path: &[MdPathStep], index: usize, before: &MdBlock, after: &MdBlock, leaves: &mut Vec<MdMutation>) {
    let nested = |step: MdPathStep| path.iter().cloned().chain(std::iter::once(step)).collect::<Vec<_>>();
    match (before, after) {
        _ if before == after => {}
        (MdBlock::Paragraph { .. }, MdBlock::Paragraph { inlines }) => leaves.push(MdMutation::SetInlines(set_inlines::SetInlines { path: path.to_vec(), index, inlines: inlines.clone() })),
        (MdBlock::Heading { level, .. }, MdBlock::Heading { level: next_level, inlines }) if level == next_level => leaves.push(MdMutation::SetInlines(set_inlines::SetInlines { path: path.to_vec(), index, inlines: inlines.clone() })),
        (MdBlock::BlockQuote { blocks }, MdBlock::BlockQuote { blocks: next_blocks }) => md_net_blocks(&nested(MdPathStep::BlockQuote { index }), blocks, next_blocks, leaves),
        (MdBlock::List { ordered, start, tight, items }, MdBlock::List { ordered: next_ordered, start: next_start, tight: next_tight, items: next_items })
            if (ordered, start, tight) == (next_ordered, next_start, next_tight) && items.len() == next_items.len() =>
        {
            for (item, (blocks, next_blocks)) in items.iter().zip(next_items).enumerate() {
                md_net_blocks(&nested(MdPathStep::ListItem { index, item }), blocks, next_blocks, leaves);
            }
        }
        _ => leaves.push(MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: path.to_vec(), index, block: after.clone() })),
    }
}
//#endregion 🧮️NetLeaves

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct MdEditor;

impl ArtifactEditor for MdEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = MdEditCommand;

    const DIALECT: Dialect = MD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MD_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.md@commonmark", extension: ".md", media_type: "text/markdown", binary: false })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        Ok(snapshot.to_text().into_bytes())
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))?;
        Ok(MdSnapshot::from_text(text))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(MdMutation::SetSnapshot(SetSnapshot { snapshot }))
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<MdEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.md@commonmark/*#editor",
        artifact_schema: "stdio.md",
        factory: "MdRetainedCommandJobFactory",
        factory_type: MdRetainedCommandJobFactory,
        contract: md_retained_contract(),
        tools: ["setActiveExample", "textEdit"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(MdRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !MD_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if md_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-md-retained-command-tool-mismatch"));
        }
        let tool_id = md_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
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
            md_command_id,
            MD_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, md_retained_reduce, md_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-md-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        md_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        md_command_from_action(action, args)
    }

    fn initial_snapshot() -> Self::Snapshot {
        MdSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            MdEditCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            _ => md_emit(command, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.md@commonmark/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for MdEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            MdEditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_net(event, snapshot, md_net_mutations)
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_md_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(MD_DIALECT)
        .document(["semio", "md"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Markdown"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
