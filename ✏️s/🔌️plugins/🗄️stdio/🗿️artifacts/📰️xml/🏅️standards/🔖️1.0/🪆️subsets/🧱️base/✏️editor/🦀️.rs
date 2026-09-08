//! ✏️ Xml editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.xml@1.0/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main` (`TreeWindowKit`),
//! directly editing `Text` nodes of `XmlSnapshot.doc` through the artifact's own
//! `XmlMutation::SetText` — `set-node` on an `Element`/`CData`/`Comment`/`ProcessingInstruction`
//! node is a documented no-op (`SetAttribute`/`InsertElement`/`RemoveElement` stay unreachable
//! through this first-pass window).

use crate::schema::mutations::{SetTextMutation, SetTextPayload, XmlNodePath};
use crate::schema::snapshot::XmlNode;
use crate::{XmlMutation, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use crate::editor::xml_any::modes::edit;
use crate::editor::xml_any::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const XML_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `/`-joined
/// child-index path encoding.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum XmlAnyEditorCommand {
    SetNode { node_id: String, value: String },
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_node_id(node_id: &str) -> Result<Vec<usize>, String> {
    if node_id.is_empty() {
        return Ok(Vec::new());
    }
    node_id.split('/').map(|segment| segment.parse::<usize>().map_err(|error| error.to_string())).collect()
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

impl protocol::OpText for XmlAnyEditorCommand {
    fn print_op(&self) -> String {
        let XmlAnyEditorCommand::SetNode { node_id, value } = self;
        format!("set-node node-id={} value={}", node_id.replace(' ', "%20"), value.replace(' ', "%20"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-node ").ok_or_else(|| store::TextError::new(format!("xml editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("xml editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("%20", " ");
            match key {
                "node-id" => node_id = Some(decoded),
                "value" => value = Some(decoded),
                _ => {}
            }
        }
        let (node_id, value) = node_id.zip(value).ok_or_else(|| store::TextError::new("xml editor command: missing node-id/value", dsl::TextSpan::at(1, 1)))?;
        Ok(XmlAnyEditorCommand::SetNode { node_id, value })
    }
}

impl protocol::OpBinary for XmlAnyEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XmlAnyEditor;

impl ArtifactEditor for XmlAnyEditor {
    type Snapshot = XmlSnapshot;
    type Mutation = XmlMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = XmlAnyEditorCommand;

    const DIALECT: Dialect = XML_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XML_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> XmlSnapshot {
        XmlSnapshot::default()
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
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let XmlAnyEditorCommand::SetNode { node_id, value } = command;
        let Ok(path) = decode_node_id(node_id) else { return Ok(Emit::default()) };
        let Some(root) = &doc.snapshot.doc.root else { return Ok(Emit::default()) };
        let Some(XmlNode::Text { .. }) = resolve_node(root, &path) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![XmlMutation::SetText(SetTextMutation::Apply(SetTextPayload { path: XmlNodePath(path), text: value.clone() }))], description: Some(format!("Set node {node_id}")), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xml_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(XML_EDITOR_DIALECT)
        .document(["semio", "stdio", "xml"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::XML_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
