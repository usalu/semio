//! ✏️ Json editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.json@rfc8259/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TreeWindowKit`), directly editing ANY node of `JsonSnapshot.value` through the artifact's own
//! `JsonMutation::SetScalar` — the frozen `set-node` command always replaces the whole subtree
//! addressed by the node's path, never merges into an existing object/array (documented scope: this
//! is a "replace this node" editor, not a structural insert/remove editor — `SetMember`/
//! `RemoveMember`/`InsertArrayElement`/`RemoveArrayElement` stay unreachable through this window).

use crate::editor::json_i_json::modes::edit;
use crate::editor::json_i_json::modes::edit::windows::main;
use crate::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
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
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const JSON_I_JSON_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `k=`/`i=` path
/// encoding (see `main::encode_path_id`), decoded back into a real `JsonPath` in `handle`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum JsonIJsonIJsonEditorCommand {
    SetNode {
        node_id: String,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

/// 🧭️ `main::encode_path_id`'s inverse. `node_id` is the addressed node's window PATH — its
/// ancestors' sibling keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR` — and
/// `main::JSON_ROOT_NODE_ID` alone is the root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_path_id(node_id: &str) -> Result<String, String> {
    if node_id == main::JSON_ROOT_NODE_ID {
        return Ok("/value".into());
    }
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    let mut segments = node_id.split(separator);
    if segments.next() != Some(main::JSON_ROOT_NODE_ID) {
        return Err("i-json editor command: node path must begin with exactly one root segment".into());
    }
    segments.try_fold("/value".to_string(), |mut path, segment| {
        if segment.is_empty() || segment == main::JSON_ROOT_NODE_ID {
            return Err(format!("i-json editor command: non-canonical path segment {segment:?}"));
        }
        let (field, raw) = segment.strip_prefix("m=").map(|value| ("members", value)).or_else(|| segment.strip_prefix("i=").map(|value| ("items", value))).ok_or_else(|| format!("json editor command: bad path segment {segment:?}"))?;
        let index = raw.parse::<usize>().map_err(|error| error.to_string())?;
        if index.to_string() != raw {
            return Err(format!("json editor command: non-canonical path segment {segment:?}"));
        }
        path.push('/');
        path.push_str(field);
        path.push('/');
        path.push_str(raw);
        if field == "members" {
            path.push_str("/value");
        }
        Ok(path)
    })
}

impl protocol::OpText for JsonIJsonIJsonEditorCommand {
    fn print_op(&self) -> String {
        match self {
            JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value } => {
                format!("set-node node-id={} revision={} value={}", crate::schema::diff::hex_encode(node_id.as_bytes()), crate::schema::diff::hex_encode(revision.as_bytes()), crate::schema::diff::hex_encode(value.as_bytes()))
            }
            JsonIJsonIJsonEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", crate::schema::diff::hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            JsonIJsonIJsonEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", crate::schema::diff::hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let bytes = crate::schema::diff::hex_decode(rest).map_err(|error| store::TextError::new(format!("i-json editor command: invalid id hex: {error}"), dsl::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("i-json editor command: invalid id utf8: {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(JsonIJsonIJsonEditorCommand::SetActiveExample { example_id });
        }
        if let Some(raw) = line.strip_prefix("snapshot-edit event=") {
            let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| store::TextError::new(format!("i-json editor command: invalid snapshot edit hex: {error}"), dsl::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| store::TextError::new(format!("i-json editor command: invalid snapshot edit: {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(JsonIJsonIJsonEditorCommand::EditSnapshot { event });
        }
        let rest = line.strip_prefix("set-node ").ok_or_else(|| store::TextError::new(format!("json editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("json editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            match key {
                "node-id" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| store::TextError::new(format!("i-json editor command: invalid node id hex: {error}"), dsl::TextSpan::at(1, 1)))?;
                    node_id = Some(String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("i-json editor command: invalid node id utf8: {error}"), dsl::TextSpan::at(1, 1)))?);
                }
                "value" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| store::TextError::new(format!("i-json editor command: invalid value hex: {error}"), dsl::TextSpan::at(1, 1)))?;
                    value = Some(String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("i-json editor command: invalid value utf8: {error}"), dsl::TextSpan::at(1, 1)))?);
                }
                "revision" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| store::TextError::new(format!("i-json editor command: invalid revision hex: {error}"), dsl::TextSpan::at(1, 1)))?;
                    revision = Some(String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("i-json editor command: invalid revision utf8: {error}"), dsl::TextSpan::at(1, 1)))?);
                }
                _ => {}
            }
        }
        let (node_id, revision, value) =
            node_id.zip(revision).zip(value).map(|((node_id, revision), value)| (node_id, revision, value)).ok_or_else(|| store::TextError::new("json editor command: missing node-id/revision/value", dsl::TextSpan::at(1, 1)))?;
        Ok(JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value })
    }
}

impl protocol::OpBinary for JsonIJsonIJsonEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TreeWindowKit`
    /// mints `set-node`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = JSON_I_JSON_COMMAND_TOOL_IDS;

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
const JSON_I_JSON_KIT_ACTION_ID: &str = "set-node";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-node`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-node` with
/// `interactive-job.missing-factory`.
const JSON_I_JSON_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, JSON_I_JSON_KIT_ACTION_ID];
const JSON_I_JSON_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    JSON_I_JSON_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const JSON_I_JSON_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.json.i-json.tool-command.v1";
const JSON_I_JSON_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-node` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const JSON_I_JSON_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: JSON_I_JSON_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JSON_I_JSON_RETAINED_RAW_BYTES, 4_096, 1, JSON_I_JSON_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::standards::v_rfc8259::subsets::i_json::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_example_snapshot(example_id: &str) -> JsonSnapshot {
    if example_id == crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID {
        <JsonSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v_rfc8259::subsets::i_json::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JsonSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<JsonIJsonIJsonEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(JsonIJsonIJsonEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JsonIJsonIJsonEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        JSON_I_JSON_KIT_ACTION_ID => Ok(JsonIJsonIJsonEditorCommand::SetNode {
            node_id: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "nodeId")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => {
            Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.i-json.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-node)")))
        }
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_command_id(command: &JsonIJsonIJsonEditorCommand) -> &'static str {
    match command {
        JsonIJsonIJsonEditorCommand::SetNode { .. } => JSON_I_JSON_KIT_ACTION_ID,
        JsonIJsonIJsonEditorCommand::EditSnapshot { event } => event.action_id(),
        JsonIJsonIJsonEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_extent(_command: &JsonIJsonIJsonEditorCommand, _snapshot: &JsonSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-node` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_emit(command: &JsonIJsonIJsonEditorCommand, snapshot: &JsonSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let JsonIJsonIJsonEditorCommand::EditSnapshot { event } = command {
        return <JsonIJsonEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    let (node_id, revision, value) = match command {
        JsonIJsonIJsonEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&json_i_json_example_snapshot(example_id), STDIO_JSON_DOCUMENT_SCHEMA)], ..Default::default() })
        }
        JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value } => (node_id, revision, value),
        JsonIJsonIJsonEditorCommand::EditSnapshot { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
    if revision != &current_revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.stale-node-edit"), "the JSON document changed while this node draft was open"));
    }
    let path = decode_path_id(node_id).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.invalid-node-path"), detail))?;
    let parsed = crate::schema::snapshot::parse_json_text(value)
        .map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.invalid-node-value"), format!("value for node '{node_id}' is not valid JSON: {error}")))?;
    let event = SnapshotEditEvent::SetValue { path, value: dsl::ToValue::to_value(&parsed) };
    <JsonIJsonEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, snapshot)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_reduce(
    command: &JsonIJsonIJsonEditorCommand,
    snapshot: &JsonSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JsonIJsonEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    json_i_json_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct JsonIJsonRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl JsonIJsonRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: JSON_I_JSON_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for JsonIJsonRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JsonIJsonEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JsonIJsonEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JSON_I_JSON_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        json_i_json_retained_contract()
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
        if input.declared_bytes() > JSON_I_JSON_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio json i-json retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JsonIJsonRetainedCommandJobFactory {
    type Owner = EditorApp<JsonIJsonEditor>;
    const TOOL_IDS: &'static [&'static str] = JSON_I_JSON_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JSON_I_JSON_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JsonIJsonEditor;

impl ArtifactEditor for JsonIJsonEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::standards::v_rfc8259::subsets::i_json::examples::demo::source()]
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
    type Command = JsonIJsonIJsonEditorCommand;

    const DIALECT: Dialect = JSON_I_JSON_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<JsonIJsonEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs",
        controller: "s.stdio.json@rfc8259/i-json#editor",
        artifact_schema: "stdio.json",
        factory: "JsonIJsonRetainedCommandJobFactory",
        factory_type: JsonIJsonRetainedCommandJobFactory,
        contract: json_i_json_retained_contract(),
        tools: ["setActiveExample", "set-node"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JsonIJsonRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !JSON_I_JSON_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if json_i_json_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-json-i-json-retained-command-tool-mismatch"));
        }
        let tool_id = json_i_json_command_id(&request.command);
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
            json_i_json_command_id,
            JSON_I_JSON_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, json_i_json_retained_reduce, json_i_json_retained_extent)),
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
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-json-i-json-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
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
        json_i_json_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        json_i_json_command_from_action(action, args)
    }

    fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    /// ✏️ An unparseable `node_id` is a documented no-op (`Emit::default()`), never a panic. The
    /// node value is parsed as one complete JSON value, preserving null, boolean, number, string,
    /// array and object kinds. Invalid input fails atomically before publishing a mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        if let Some(event) = <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_event(command) {
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        }
        json_i_json_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                main::render_editor(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), "s.stdio.json@rfc8259/i-json#editor", &revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.json@rfc8259/i-json#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for JsonIJsonEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            JsonIJsonIJsonEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| JsonMutation::PatchSnapshot(crate::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_i_json_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(JSON_I_JSON_EDITOR_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::JSON_I_JSON_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Tree"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID, crate::standards::v_rfc8259::subsets::i_json::examples::demo::label())], crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID))
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
