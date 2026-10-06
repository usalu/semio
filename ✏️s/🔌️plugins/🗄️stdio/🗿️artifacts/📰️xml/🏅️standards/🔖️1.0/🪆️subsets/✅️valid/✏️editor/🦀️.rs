//! ✏️ Xml editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.xml@1.0/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main` (`TreeWindowKit`),
//! directly editing `Text` nodes of `XmlSnapshot.doc` through the artifact's own
//! `XmlValidMutation::SetText` — `set-node` on an `Element`/`CData`/`Comment`/`ProcessingInstruction`
//! node is a documented no-op. The vocabulary is this SUBSET's own `XmlValidMutation`, not `✳️any`'s
//! `XmlMutation`: `SetText` is the one kind the two share, and the DOCTYPE/validity kinds
//! (`declare-doctype`, `rename-document-element`, `set-external-subset`, `set-standalone`,
//! `declare-entity`, `undeclare-entity`) stay unreachable through this first-pass window.

use crate::editor::xml_valid::modes::edit;
use crate::editor::xml_valid::modes::edit::windows::main;
use crate::schema::mutations::XmlNodePath;
use crate::schema::snapshot::XmlNode;
use crate::standards::v1_0::subsets::valid::schema::valid_mutations::set_text::SetText;
use crate::standards::v1_0::subsets::valid::schema::XmlValidMutation;
use crate::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
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
pub const XML_VALID_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `/`-joined
/// child-index path encoding.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum XmlValidEditorCommand {
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

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    let bytes = text.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err("odd-length hex string".into());
    }
    fn nibble(byte: u8) -> Result<u8, String> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err(format!("non-hex byte 0x{byte:02x}")),
        }
    }
    bytes.chunks_exact(2).map(|pair| Ok((nibble(pair[0])? << 4) | nibble(pair[1])?)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_node_id(node_id: &str) -> Result<Vec<usize>, String> {
    if node_id == main::XML_ROOT_NODE_ID {
        return Ok(Vec::new());
    }
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    let mut segments = node_id.split(separator);
    if segments.next() != Some(main::XML_ROOT_NODE_ID) {
        return Err("xml valid editor command: node path must begin with exactly one root segment".into());
    }
    segments
        .map(|segment| {
            if segment.is_empty() || segment == main::XML_ROOT_NODE_ID {
                return Err(format!("non-canonical XML node segment {segment:?}"));
            }
            let index = segment.parse::<usize>().map_err(|error| error.to_string())?;
            if index.to_string() != segment {
                return Err(format!("non-canonical XML node index {segment:?}"));
            }
            Ok(index)
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_node<'a>(root: &'a XmlNode, path: &[usize]) -> Option<&'a XmlNode> {
    let mut node = root;
    for &index in path {
        match node {
            XmlNode::Element { children, .. } => node = children.get(index)?,
            _ => return None,
        }
    }
    Some(node)
}

impl protocol::OpText for XmlValidEditorCommand {
    fn print_op(&self) -> String {
        match self {
            XmlValidEditorCommand::SetNode { node_id, revision, value } => {
                format!("set-node node-id={} revision={} value={}", hex_encode(node_id.as_bytes()), hex_encode(revision.as_bytes()), hex_encode(value.as_bytes()))
            }
            XmlValidEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            XmlValidEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let example_id = String::from_utf8(hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid example hex {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid example utf8 {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(XmlValidEditorCommand::SetActiveExample { example_id });
        }
        if let Some(rest) = line.strip_prefix("snapshot-edit event=") {
            let bytes = hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid snapshot edit hex {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid snapshot edit {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(XmlValidEditorCommand::EditSnapshot { event });
        }
        let rest = line.strip_prefix("set-node ").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml editor command: unknown line {line:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml editor command: bad token {token:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let decoded = String::from_utf8(hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid field hex {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("xml valid editor command: invalid field utf8 {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            match key {
                "node-id" => node_id = Some(decoded),
                "revision" => revision = Some(decoded),
                "value" => value = Some(decoded),
                _ => {}
            }
        }
        let (node_id, revision, value) =
            node_id.zip(revision).zip(value).map(|((node_id, revision), value)| (node_id, revision, value)).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "xml editor command: missing node-id/revision/value", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(XmlValidEditorCommand::SetNode { node_id, revision, value })
    }
}

impl protocol::OpBinary for XmlValidEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TreeWindowKit`
    /// mints `set-node`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = XML_VALID_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TreeWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const XML_VALID_KIT_ACTION_ID: &str = "set-node";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-node`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-node` with
/// `interactive-job.missing-factory`.
const XML_VALID_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, XML_VALID_KIT_ACTION_ID];
const XML_VALID_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    XML_VALID_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const XML_VALID_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.xml.valid.tool-command.v1";
const XML_VALID_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-node` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const XML_VALID_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: XML_VALID_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(XML_VALID_RETAINED_RAW_BYTES, 4_096, 1, XML_VALID_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::standards::v1_0::subsets::valid::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_example_snapshot(example_id: &str) -> XmlSnapshot {
    if example_id == crate::standards::v1_0::subsets::valid::examples::demo::ID {
        <XmlSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1_0::subsets::valid::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        XmlSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<XmlValidEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(XmlValidEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(XmlValidEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        XML_VALID_KIT_ACTION_ID => Ok(XmlValidEditorCommand::SetNode {
            node_id: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "nodeId")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => {
            Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml.valid.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-node)")))
        }
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_command_id(command: &XmlValidEditorCommand) -> &'static str {
    match command {
        XmlValidEditorCommand::SetNode { .. } => XML_VALID_KIT_ACTION_ID,
        XmlValidEditorCommand::EditSnapshot { event } => event.action_id(),
        XmlValidEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_retained_extent(_command: &XmlValidEditorCommand, _snapshot: &XmlSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-node` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_emit(command: &XmlValidEditorCommand, snapshot: &XmlSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<XmlValidMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let (node_id, revision, value) = match command {
        XmlValidEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&xml_valid_example_snapshot(example_id), STDIO_XML_DOCUMENT_SCHEMA)], ..Default::default() })
        }
        XmlValidEditorCommand::SetNode { node_id, revision, value } => (node_id, revision, value),
        XmlValidEditorCommand::EditSnapshot { .. } => return Err(Fault::from("stdio-xml-valid-snapshot-edit-routed-to-native-reducer")),
    };
    let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
    if revision != &current_revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml-valid.stale-node-edit"), "the XML document changed while this node draft was open"));
    }
    if node_id == main::XML_ROOT_NODE_ID {
        let next = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(value).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml-valid.invalid-source"), error.to_string()))?;
        if &next == snapshot {
            return Ok(Emit::default());
        }
        return Ok(Emit {
            artifact_mutations: vec![XmlValidMutation::SetSnapshot(crate::standards::v1_0::subsets::valid::schema::valid_mutations::set_snapshot::SetSnapshot { snapshot: next })],
            ..Default::default()
        });
    }
    let path = decode_node_id(node_id).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml-valid.invalid-node-path"), detail))?;
    let root = snapshot.doc.root.as_ref().ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml-valid.missing-root"), "The XML document has no root node to edit."))?;
    let Some(XmlNode::Text { text }) = resolve_node(root, &path) else {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xml-valid.node-is-not-text"), "The addressed XML node is not a text node."));
    };
    if text == value {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![XmlValidMutation::SetText(SetText { path: XmlNodePath(path), text: value.clone() })], ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_valid_retained_reduce(
    command: &XmlValidEditorCommand,
    snapshot: &XmlSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<XmlValidEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<XmlValidMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    xml_valid_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct XmlValidRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl XmlValidRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: XML_VALID_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for XmlValidRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<XmlValidEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<XmlValidEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        XML_VALID_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        xml_valid_retained_contract()
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
        if input.declared_bytes() > XML_VALID_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio xml valid retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for XmlValidRetainedCommandJobFactory {
    type Owner = EditorApp<XmlValidEditor>;
    const TOOL_IDS: &'static [&'static str] = XML_VALID_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XML_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = XML_VALID_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XmlValidEditor;

impl ArtifactEditor for XmlValidEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::standards::v1_0::subsets::valid::examples::demo::source()]
    }
    type Snapshot = XmlSnapshot;
    type Mutation = XmlValidMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = XmlValidEditorCommand;

    const DIALECT: Dialect = XML_VALID_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XML_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.xml@1.0", extension: ".xml", media_type: "application/xml", binary: false })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        snapshot.export_utf8().map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        XmlSnapshot::import_utf8(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(XmlValidMutation::SetSnapshot(
            crate::standards::v1_0::subsets::valid::schema::valid_mutations::set_snapshot::SetSnapshot { snapshot },
        ))
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<XmlValidEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs",
        controller: "s.stdio.xml@1.0/valid#editor",
        artifact_schema: "stdio.xml",
        factory: "XmlValidRetainedCommandJobFactory",
        factory_type: XmlValidRetainedCommandJobFactory,
        contract: xml_valid_retained_contract(),
        tools: ["setActiveExample", "set-node"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(XmlValidRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !XML_VALID_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if xml_valid_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-xml-valid-retained-command-tool-mismatch"));
        }
        let tool_id = xml_valid_command_id(&request.command);
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
            xml_valid_command_id,
            XML_VALID_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, xml_valid_retained_reduce, xml_valid_retained_extent)),
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
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-xml-valid-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
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
        xml_valid_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        xml_valid_command_from_action(action, args)
    }

    fn initial_snapshot() -> XmlSnapshot {
        crate::standards::v1_0::subsets::valid::schema::blank_valid_xml_snapshot()
    }

    /// ✏️ Only a `Text` node found at `node_id` accepts `set-node` — anything else (unparseable
    /// id, missing node, non-`Text` node) is a documented no-op (`Emit::default()`).
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            XmlValidEditorCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            _ => xml_valid_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision)),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_editor(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), "s.stdio.xml@1.0/valid#editor", &revision, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.xml@1.0/valid#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for XmlValidEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            XmlValidEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| XmlValidMutation::PatchSnapshot(crate::standards::v1_0::subsets::valid::schema::valid_mutations::patch_snapshot::PatchSnapshot { patch }), Some(|snapshot| XmlValidMutation::SetSnapshot(crate::standards::v1_0::subsets::valid::schema::valid_mutations::set_snapshot::SetSnapshot { snapshot })))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xml_valid_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(XML_VALID_EDITOR_DIALECT)
        .document(["semio", "stdio", "xml"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::XML_VALID_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "XML"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::standards::v1_0::subsets::valid::examples::demo::ID, crate::standards::v1_0::subsets::valid::examples::demo::label())], crate::standards::v1_0::subsets::valid::examples::demo::ID))
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
