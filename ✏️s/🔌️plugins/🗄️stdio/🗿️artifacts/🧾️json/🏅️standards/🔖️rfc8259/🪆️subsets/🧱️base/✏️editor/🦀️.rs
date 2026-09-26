//! ✏️ Json editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.json@rfc8259/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TreeWindowKit`), directly editing ANY node of `JsonSnapshot.value` through the artifact's own
//! `JsonMutation::SetScalar` — the frozen `set-node` command always replaces the whole subtree
//! addressed by the node's path, never merges into an existing object/array (documented scope: this
//! is a "replace this node" editor, not a structural insert/remove editor — `SetMember`/
//! `RemoveMember`/`InsertArrayElement`/`RemoveArrayElement` stay unreachable through this window).

use crate::editor::json_any::modes::edit;
use crate::editor::json_any::modes::edit::windows::main;
use crate::schema::mutations::{JsonPath, JsonPathSegment, SetScalarMutation, SetScalarPayload};
use crate::schema::snapshot::JsonValue;
use crate::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId,
    SubsetId, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const JSON_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `k=`/`i=` path
/// encoding (see `main::encode_path_id`), decoded back into a real `JsonPath` in `handle`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum JsonAnyEditorCommand {
    SetNode { node_id: String, value: String },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample { example_id: String },
}

/// 🧭️ `main::encode_path_id`'s inverse. `node_id` is the addressed node's window PATH — its
/// ancestors' sibling keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR` — and
/// `main::JSON_ROOT_NODE_ID` alone (or the legacy empty string a pre-window body sent) is the root.
/// A `/`-joined id from the pre-window encoding still decodes, so an in-flight command survives.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_path_id(node_id: &str) -> Result<JsonPath, String> {
    if node_id.is_empty() || node_id == main::JSON_ROOT_NODE_ID {
        return Ok(Vec::new());
    }
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    let segments: Vec<&str> = if node_id.contains(separator) { node_id.split(separator).collect() } else { node_id.split('/').collect() };
    segments
        .into_iter()
        .filter(|segment| !segment.is_empty() && *segment != main::JSON_ROOT_NODE_ID)
        .map(|segment| {
            let segment = main::strip_sibling_ordinal(segment);
            if let Some(key) = segment.strip_prefix("k=") {
                Ok(JsonPathSegment::Key(key.to_string()))
            } else if let Some(index) = segment.strip_prefix("i=") {
                index.parse::<usize>().map(JsonPathSegment::Index).map_err(|error| error.to_string())
            } else {
                Err(format!("json editor command: bad path segment {segment:?}"))
            }
        })
        .collect()
}

impl protocol::OpText for JsonAnyEditorCommand {
    fn print_op(&self) -> String {
        match self {
            JsonAnyEditorCommand::SetNode { node_id, value } => format!("set-node node-id={} value={}", node_id.replace(' ', "%20"), value.replace(' ', "%20")),
            JsonAnyEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", example_id.replace(' ', "%20")),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            return Ok(JsonAnyEditorCommand::SetActiveExample { example_id: rest.replace("%20", " ") });
        }
        let rest = line.strip_prefix("set-node ").ok_or_else(|| store::TextError::new(format!("json editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("json editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("%20", " ");
            match key {
                "node-id" => node_id = Some(decoded),
                "value" => value = Some(decoded),
                _ => {}
            }
        }
        let (node_id, value) = node_id.zip(value).ok_or_else(|| store::TextError::new("json editor command: missing node-id/value", dsl::TextSpan::at(1, 1)))?;
        Ok(JsonAnyEditorCommand::SetNode { node_id, value })
    }
}

impl protocol::OpBinary for JsonAnyEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TreeWindowKit`
    /// mints `set-node`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = JSON_ANY_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TreeWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const JSON_ANY_KIT_ACTION_ID: &str = "set-node";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-node`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-node` with
/// `interactive-job.missing-factory`.
const JSON_ANY_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, JSON_ANY_KIT_ACTION_ID];
const JSON_ANY_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.json.tool-command.v1";
const JSON_ANY_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-node` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const JSON_ANY_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: JSON_ANY_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JSON_ANY_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_example_snapshot(example_id: &str) -> JsonSnapshot {
    if example_id == crate::examples::demo::ID {
        <JsonSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JsonSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<JsonAnyEditorCommand, Fault> {
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JsonAnyEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        JSON_ANY_KIT_ACTION_ID => Ok(JsonAnyEditorCommand::SetNode { node_id: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["nodeId", "node_id", "id"], ""), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") }),
        other => Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.json.unhandled-action"),
            format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-node)"),
        )),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_command_id(command: &JsonAnyEditorCommand) -> &'static str {
    match command {
        JsonAnyEditorCommand::SetNode { .. } => JSON_ANY_KIT_ACTION_ID,
        JsonAnyEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_extent(_command: &JsonAnyEditorCommand, _snapshot: &JsonSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-node` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_emit(command: &JsonAnyEditorCommand, _snapshot: &JsonSnapshot) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let (node_id, value) = match command {
        JsonAnyEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&json_any_example_snapshot(example_id), STDIO_JSON_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            })
        }
        JsonAnyEditorCommand::SetNode { node_id, value } => (node_id, value),
    };
    let Ok(path) = decode_path_id(node_id) else { return Ok(Emit::default()) };
    Ok(Emit { artifact_mutations: vec![JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path, value: JsonValue::String { value: value.clone() } }))], description: Some(format!("Set node {node_id}")), ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_reduce(
    command: &JsonAnyEditorCommand,
    snapshot: &JsonSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JsonAnyEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    json_any_emit(command, snapshot)
}

struct JsonAnyRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl JsonAnyRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: JSON_ANY_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for JsonAnyRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JsonAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JsonAnyEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JSON_ANY_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        json_any_retained_contract()
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
        if input.declared_bytes() > JSON_ANY_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio json retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JsonAnyRetainedCommandJobFactory {
    type Owner = EditorApp<JsonAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = JSON_ANY_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JSON_ANY_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JsonAnyEditor;

impl ArtifactEditor for JsonAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JsonAnyEditorCommand;

    const DIALECT: Dialect = JSON_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<JsonAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.json@rfc8259/*#editor",
        artifact_schema: "stdio.json",
        factory: "JsonAnyRetainedCommandJobFactory",
        factory_type: JsonAnyRetainedCommandJobFactory,
        contract: json_any_retained_contract(),
        tools: ["setActiveExample", "set-node"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JsonAnyRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if !JSON_ANY_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if json_any_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-json-retained-command-tool-mismatch"));
        }
        let tool_id = json_any_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
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
            json_any_command_id,
            JSON_ANY_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, json_any_retained_reduce, json_any_retained_extent)),
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
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-json-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
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
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_JSON_DOCUMENT_SCHEMA, operation, generation))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        json_any_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        json_any_command_from_action(action, args)
    }

    fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    /// ✏️ An unparseable `node_id` is a documented no-op (`Emit::default()`), never a panic. The
    /// new leaf value always lands as `JsonValue::String` — scalar-type-preserving edits (number,
    /// bool) are a documented future scope, not attempted here.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        json_any_emit(command, doc.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(JSON_EDITOR_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::JSON_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
