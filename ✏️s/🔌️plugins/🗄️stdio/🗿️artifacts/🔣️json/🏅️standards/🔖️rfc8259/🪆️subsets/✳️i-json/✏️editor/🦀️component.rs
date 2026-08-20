//! ✏️ Json editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.json@rfc8259/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TreeWindowKit`), directly editing ANY node of `JsonSnapshot.value` through the artifact's own
//! `JsonMutation::SetScalar` — the frozen `set-node` command always replaces the whole subtree
//! addressed by the node's path, never merges into an existing object/array (documented scope: this
//! is a "replace this node" editor, not a structural insert/remove editor — `SetMember`/
//! `RemoveMember`/`InsertArrayElement`/`RemoveArrayElement` stay unreachable through this window).

use crate::artifacts::json::schema::mutations::{JsonPath, JsonPathSegment};
use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::editor::json_i_json::modes::edit;
use crate::editor::json_i_json::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const JSON_I_JSON_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `k=`/`i=` path
/// encoding (see `main::encode_path_id`), decoded back into a real `JsonPath` in `handle`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonIJsonIJsonEditorCommand {
    SetNode { node_id: String, value: String },
}

/// 🧭️ `main::encode_path_id`'s inverse — `""` decodes to the empty (root) path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_path_id(node_id: &str) -> Result<JsonPath, String> {
    if node_id.is_empty() {
        return Ok(Vec::new());
    }
    node_id
        .split('/')
        .map(|segment| {
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

impl protocol::OpText for JsonIJsonIJsonEditorCommand {
    async fn print_op(&self) -> String {
        let JsonIJsonIJsonEditorCommand::SetNode { node_id, value } = self;
        format!("set-node node-id={} value={}", node_id.replace(' ', "%20"), value.replace(' ', "%20"))
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
        Ok(JsonIJsonIJsonEditorCommand::SetNode { node_id, value })
    }
}

impl protocol::OpBinary for JsonIJsonIJsonEditorCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).await.into_bytes())
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).await.map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JsonIJsonEditor;

impl ArtifactEditor for JsonIJsonEditor {
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

    async fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    /// ✏️ An unparseable `node_id` is a documented no-op (`Emit::default()`), never a panic. The
    /// new leaf value always lands as `JsonValue::String` — scalar-type-preserving edits (number,
    /// bool) are a documented future scope, not attempted here.
    async fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let JsonIJsonIJsonEditorCommand::SetNode { node_id, value } = command;
        let Ok(path) = decode_path_id(node_id) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![JsonMutation::SetScalar { path, value: JsonValue::String { value: value.clone() } }], description: Some(format!("Set node {node_id}")), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_i_json_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(JSON_I_JSON_EDITOR_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::JSON_I_JSON_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_json_i_json_editor_builds_a_definition_for_the_editor_role() {
        let def = create_json_i_json_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, JSON_I_JSON_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<JsonIJsonEditor as ArtifactEditor>::DIALECT, JSON_I_JSON_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_tree_window() {
        let def = create_json_i_json_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_path_id_roundtrips_root_and_nested() {
        assert_eq!(decode_path_id("").unwrap(), Vec::<JsonPathSegment>::new());
        assert_eq!(decode_path_id("k=a/i=0").unwrap(), vec![JsonPathSegment::Key("a".into()), JsonPathSegment::Index(0)]);
        assert!(decode_path_id("bad").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = JsonIJsonIJsonEditorCommand::SetNode { node_id: "k=a/i=0".into(), value: "hello world".into() };
        let printed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
