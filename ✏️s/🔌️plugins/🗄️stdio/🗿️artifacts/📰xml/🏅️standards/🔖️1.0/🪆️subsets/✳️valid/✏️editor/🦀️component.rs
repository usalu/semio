//! ✏️ Xml editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.xml@1.0/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main` (`TreeWindowKit`),
//! directly editing `Text` nodes of `XmlSnapshot.doc` through the artifact's own
//! `XmlMutation::SetText` — `set-node` on an `Element`/`CData`/`Comment`/`ProcessingInstruction`
//! node is a documented no-op (`SetAttribute`/`InsertElement`/`RemoveElement` stay unreachable
//! through this first-pass window).

use crate::artifacts::xml::schema::mutations::XmlNodePath;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::artifacts::xml::{XmlMutation, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use crate::editor::xml_valid::modes::edit;
use crate::editor::xml_valid::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const XML_VALID_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `/`-joined
/// child-index path encoding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum XmlValidEditorCommand {
    SetNode { node_id: String, value: String },
}

async fn decode_node_id(node_id: &str) -> Result<Vec<usize>, String> {
    if node_id.is_empty() {
        return Ok(Vec::new());
    }
    node_id.split('/').map(|segment| segment.parse::<usize>().map_err(|error| error.to_string())).collect()
}

async fn resolve_node<'a>(root: &'a XmlNode, path: &[usize]) -> Option<&'a XmlNode> {
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
    async fn print_op(&self) -> String {
        let XmlValidEditorCommand::SetNode { node_id, value } = self;
        format!("set-node node-id={} value={}", node_id.replace(' ', "%20"), value.replace(' ', "%20"))
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
        Ok(XmlValidEditorCommand::SetNode { node_id, value })
    }
}

impl protocol::OpBinary for XmlValidEditorCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "xml editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XmlValidEditor;

impl ArtifactEditor for XmlValidEditor {
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
    type Command = XmlValidEditorCommand;

    const DIALECT: Dialect = XML_VALID_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XML_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> XmlSnapshot {
        XmlSnapshot::default()
    }

    /// ✏️ Only a `Text` node found at `node_id` accepts `set-node` — anything else (unparseable
    /// id, missing node, non-`Text` node) is a documented no-op (`Emit::default()`).
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let XmlValidEditorCommand::SetNode { node_id, value } = command;
        let Ok(path) = decode_node_id(node_id) else { return Ok(Emit::default()) };
        let Some(root) = &doc.snapshot.doc.root else { return Ok(Emit::default()) };
        let Some(XmlNode::Text { .. }) = resolve_node(root, &path) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![XmlMutation::SetText { path: XmlNodePath(path), text: value.clone() }], description: Some(format!("Set node {node_id}")), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub async fn create_xml_valid_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(XML_VALID_EDITOR_DIALECT)
        .document(["semio", "stdio", "xml"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::XML_VALID_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_xml_valid_editor_builds_a_definition_for_the_editor_role() {
        let def = create_xml_valid_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, XML_VALID_EDITOR_DIALECT.into());
    }

    #[test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<XmlValidEditor as ArtifactEditor>::DIALECT, XML_VALID_EDITOR_DIALECT);
    }

    #[test]
    async fn editor_declares_the_tree_window() {
        let def = create_xml_valid_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[test]
    async fn decode_node_id_roundtrips_root_and_nested() {
        assert_eq!(decode_node_id("").unwrap(), Vec::<usize>::new());
        assert_eq!(decode_node_id("0/2").unwrap(), vec![0, 2]);
        assert!(decode_node_id("bad").is_err());
    }

    #[test]
    async fn op_text_roundtrip() {
        let command = XmlValidEditorCommand::SetNode { node_id: "0/2".into(), value: "hello world".into() };
        let printed = <XmlValidEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <XmlValidEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
