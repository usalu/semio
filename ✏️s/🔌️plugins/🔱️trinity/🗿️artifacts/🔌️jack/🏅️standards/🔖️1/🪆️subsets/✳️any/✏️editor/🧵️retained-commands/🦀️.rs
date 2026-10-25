//! 🧵️ Jack retained commands: the window configuration, transient and document tool ids and limits, the three bounded job factories and the transient command work.

use super::*;
use crate::editor::jack::commands;

//#region 🔖️TrinityJackPlayApp
//#region 🧵️RetainedConfigCommands
pub const JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS: &[&str] = &["nodeGraphViewport", "setLodMode"];

pub const JACK_RETAINED_TRANSIENT_TOOL_IDS: &[&str] = &["textSelect"];

const JACK_RETAINED_PAYLOAD_SCHEMA: &str = "trinity.graph.config-command.v1";

pub const JACK_RETAINED_RAW_BYTES: usize = 8_192;

pub const JACK_RETAINED_WORK_ITEMS: usize = 64;

const JACK_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
];

const JACK_RETAINED_TRANSIENT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "textSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] }];

fn jack_retained_config_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JACK_RETAINED_RAW_BYTES, 64, JACK_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

pub fn jack_retained_window_config_extent(command: &TrinityJackCommand, _snapshot: &JackSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        TrinityJackCommand::SetViewport { viewport, .. } if viewport.validate().is_ok() => 1,
        TrinityJackCommand::SetViewport { .. } => return None,
        TrinityJackCommand::SetLodMode { value } => value.len(),
        _ => return None,
    };
    (bytes <= JACK_RETAINED_RAW_BYTES).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
pub fn jack_retained_window_config_reduce<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(
    command: &TrinityJackCommand,
    _snapshot: &JackSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityJackPlayApp<C>>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<TrinityGraphMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TrinityJackCommand::SetViewport { viewport, .. } => commands::set_viewport::set_viewport(viewport, context.and_then(|context| context.view_state.as_ref())),
        TrinityJackCommand::SetLodMode { value } => commands::set_lod_mode::set_lod_mode(value, context.and_then(|context| context.view_state.as_ref())),
        _ => Err(Fault::from("jack-retained-config-route-mismatch")),
    }
}

pub struct JackRetainedWindowConfigJobFactory<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> {
    composition:std::marker::PhantomData<C>,
    pub keys: Vec<ToolFactoryKey>,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackRetainedWindowConfigJobFactory<C> {
    pub fn new(controller: &str) -> Self {
        Self { composition:std::marker::PhantomData,keys: JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ToolJobFactory for JackRetainedWindowConfigJobFactory<C> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TrinityJackPlayApp<C>>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TrinityJackPlayApp<C>>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JACK_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        jack_retained_config_contract()
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: JACK_RETAINED_RAW_BYTES, checkpoint_resume: false);
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactOwnedToolJobFactory for JackRetainedWindowConfigJobFactory<C> {
    type Owner = EditorApp<TrinityJackPlayApp<C>>;
    const TOOL_IDS: &'static [&'static str] = JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JACK_RETAINED_PUBLICATION_CONTRACTS;
}

pub struct JackRetainedTransientJobFactory<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> {
    composition:std::marker::PhantomData<C>,
    pub keys: Vec<ToolFactoryKey>,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackRetainedTransientJobFactory<C> {
    pub fn new(controller: &str) -> Self {
        Self { composition:std::marker::PhantomData,keys: JACK_RETAINED_TRANSIENT_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ToolJobFactory for JackRetainedTransientJobFactory<C> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TrinityJackPlayApp<C>>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TrinityJackPlayApp<C>>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JACK_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        jack_retained_config_contract()
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: JACK_RETAINED_RAW_BYTES, checkpoint_resume: false);
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactOwnedToolJobFactory for JackRetainedTransientJobFactory<C> {
    type Owner = EditorApp<TrinityJackPlayApp<C>>;
    const TOOL_IDS: &'static [&'static str] = JACK_RETAINED_TRANSIENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JACK_RETAINED_TRANSIENT_PUBLICATION_CONTRACTS;
}

pub struct JackEditorWindowTransientCommandWork {
    pub consumed: bool,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactCommandWork<EditorApp<TrinityJackPlayApp<C>>> for JackEditorWindowTransientCommandWork {
    fn tool_id(&self) -> &'static str {
        "textSelect"
    }

    fn extent(&self, command: &TrinityJackCommand, _snapshot: &JackSnapshot, _interaction: &protocol::InteractionState, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityJackPlayApp<C>>>>) -> Option<usize> {
        matches!(command, TrinityJackCommand::TextSelect { .. }).then_some(()).and(context.and_then(|context| context.view_state.as_ref()).and_then(|view| view.window_id.as_ref()).filter(|window_id| !window_id.is_empty()).map(|_| ())).map(|_| 1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<TrinityJackPlayApp<C>>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<TrinityJackPlayApp<C>>>, semio_framework_job::OriginalTurnFailure> {
        let original:Result<_,Fault>=(||{
        if self.consumed {
            return Err(Fault::from("jack-retained-transient-work-repeated"));
        }
        self.consumed = true;
        let TrinityJackCommand::TextSelect { start, end } = input.command else {
            return Err(Fault::from("jack-retained-transient-route-mismatch"));
        };
        let window_id =
            input.context.and_then(|context| context.view_state.as_ref()).and_then(|view| view.window_id.as_ref()).filter(|window_id| !window_id.is_empty()).cloned().ok_or_else(|| Fault::from("jack-text-selection-requires-concrete-window"))?;
        let mutation = semio_framework_plugin::WindowTransientMutation::of::<JackEditorWindowTransientOwner>(
            window_id,
            crate::editor::jack::transient::SetEditorSelection { selection: Some(crate::editor::jack::transient::JackEditorSelection { start: *start, end: *end }) }.into(),
        );
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] } })
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 🧾️ Document verbs as bounded first-step tools (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END): the framework
/// refuses UI dispatch of every command not classified `Migrated`, which left the whole document surface
/// dead in the shell. `patchNodes`/`deleteSelection` publish granular graph mutations; the example and
/// fixture loaders publish a host-applied `Effect::LoadDocument`.
pub const JACK_RETAINED_DOCUMENT_TOOL_IDS: &[&str] = &["patchNodes", "deleteSelection", "setActiveExample", "loadDocumentJson", "textEdit", "formatDocument"];

const JACK_RETAINED_DOCUMENT_PAYLOAD_SCHEMA: &str = "trinity.graph.document-command.v1";

pub const JACK_RETAINED_DOCUMENT_RAW_BYTES: usize = 32_768;

const JACK_RETAINED_DOCUMENT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "formatDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn jack_retained_document_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JACK_RETAINED_DOCUMENT_RAW_BYTES, 64, JACK_RETAINED_WORK_ITEMS as u64, 262_144, 7_500)
}

pub fn jack_retained_document_extent(command: &TrinityJackCommand, _snapshot: &JackSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        TrinityJackCommand::PatchNodes { node_ids, field, value } => node_ids.iter().map(String::len).try_fold(field.len().checked_add(value.len())?, usize::checked_add)?,
        TrinityJackCommand::DeleteSelection => 1,
        TrinityJackCommand::SetActiveExample { example_id } => example_id.len(),
        TrinityJackCommand::LoadDocumentJson { json } => json.len(),
        TrinityJackCommand::TextEdit { text } if text.len() <= crate::standards::v1::subsets::any::schema::graph::JACK_QUERY_MAXIMUM_BYTES => text.len(),
        TrinityJackCommand::FormatDocument => 1,
        _ => return None,
    };
    (bytes <= JACK_RETAINED_DOCUMENT_RAW_BYTES).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
pub fn jack_retained_document_reduce<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(
    command: &TrinityJackCommand,
    snapshot: &JackSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityJackPlayApp<C>>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<TrinityGraphMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let children = || context.map(|context| context.children.as_ref()).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.jack.content-missing"), "the retained document command carries no child view"));
    Ok(match command {
        TrinityJackCommand::PatchNodes { node_ids, field, value } => commands::patch_nodes::patch_nodes::<C>(snapshot, children()?, node_ids, interaction.selection.get("ast").map_or(&[][..], |selection| selection.ids.as_slice()), field, value)?,
        TrinityJackCommand::DeleteSelection => commands::delete_selection::delete_selection::<C>(snapshot, children()?, interaction.selection.get("ast").map_or(&[][..], |selection| selection.ids.as_slice()))?,
        TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example::set_active_example::<C>(example_id),
        TrinityJackCommand::LoadDocumentJson { json } => commands::load_document_json::load_document_json(json),
        TrinityJackCommand::TextEdit { text } => commands::text_edit::text_edit(text)?,
        TrinityJackCommand::FormatDocument => commands::format_document::format_document(&snapshot.query),
        _ => return Err(Fault::from("jack-retained-document-route-mismatch")),
    })
}

pub struct JackRetainedDocumentJobFactory<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> {
    composition:std::marker::PhantomData<C>,
    pub keys: Vec<ToolFactoryKey>,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackRetainedDocumentJobFactory<C> {
    pub fn new(controller: &str) -> Self {
        Self { composition:std::marker::PhantomData,keys: JACK_RETAINED_DOCUMENT_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ToolJobFactory for JackRetainedDocumentJobFactory<C> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TrinityJackPlayApp<C>>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TrinityJackPlayApp<C>>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JACK_RETAINED_DOCUMENT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        jack_retained_document_contract()
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: JACK_RETAINED_DOCUMENT_RAW_BYTES, checkpoint_resume: false);
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactOwnedToolJobFactory for JackRetainedDocumentJobFactory<C> {
    type Owner = EditorApp<TrinityJackPlayApp<C>>;
    const TOOL_IDS: &'static [&'static str] = JACK_RETAINED_DOCUMENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JACK_RETAINED_DOCUMENT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedConfigCommands
//#endregion 🔖️TrinityJackPlayApp
