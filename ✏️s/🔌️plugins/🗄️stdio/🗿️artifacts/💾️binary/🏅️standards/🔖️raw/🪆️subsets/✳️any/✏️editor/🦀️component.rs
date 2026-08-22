//! 💾️ Binary editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.binary@raw/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `BinarySnapshot` is the simplest
//! possible document: one opaque `bytes: Vec<u8>` buffer. One window, `🪟️main` (`TextWindowKit`),
//! renders it as a capped hex dump; its `replace-text` action funnels through the one typed command
//! this surface declares, `BinaryEditorCommand::ReplaceText`, which parses the hex text back into
//! bytes and emits a whole-buffer `BinaryMutation::Splice` (see the window's own doc comment for the
//! honest cap/truncation scope note).

use crate::artifacts::binary::{BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::editor::binary::modes::edit;
use crate::editor::binary::modes::edit::windows::main;
#[cfg(test)]
use semio_framework_plugin::UiNode;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.binary@raw/*`, verified against this artifact's
/// own `🏅️standards/🔖️raw/🪆️subsets/✳️any` location on disk. No reusable const exists on the
/// artifact's own root `🦀️component.rs` (checked before adding this), so it is inlined here and
/// reused for both `impl ArtifactEditor` and the manifest below.
pub const BINARY_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum BinaryEditorCommand {
    #[dsl(key = "replace-binary-text")]
    ReplaceText { text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for BinaryEditorCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for BinaryEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️TextParse
/// 📐️ Parses `render()`'s hex dump back into bytes: strips `#`-prefixed comment lines and
/// whitespace, then decodes the remaining contiguous hex — same convention
/// `BinarySnapshot::parse_dsl` already uses. `None` on odd length or an invalid hex digit — the
/// caller treats that as a documented no-op, never a partial apply.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_hex_dump(text: &str) -> Option<Vec<u8>> {
    let hex: String = text.lines().filter(|line| !line.trim_start().starts_with('#')).collect::<Vec<_>>().join("").chars().filter(|c| !c.is_whitespace()).collect();
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        let high = high.to_digit(16)?;
        let low = low.to_digit(16)?;
        bytes.push(((high << 4) | low) as u8);
    }
    Some(bytes)
}
//#endregion 🔖️TextParse

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BinaryEditor;

impl ArtifactEditor for BinaryEditor {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BinaryEditorCommand;

    const DIALECT: Dialect = BINARY_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BINARY_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> BinarySnapshot {
        BinarySnapshot::default()
    }

    /// ✏️ Parses the hex text and, if well-formed, replaces the WHOLE buffer via
    /// `BinaryMutation::Splice { offset: 0, remove_len: <old len>, insert: <parsed> }`. Malformed
    /// hex (odd length or an invalid digit) is a documented no-op (`Emit::default()`), never a panic.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let BinaryEditorCommand::ReplaceText { text } = command;
        let Some(parsed) = parse_hex_dump(text) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![BinaryMutation::Splice { offset: 0, remove_len: doc.snapshot.bytes.len(), insert: parsed }], description: Some("Replace bytes".into()), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_binary_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(BINARY_EDITOR_DIALECT).document(["stdio", "binary"]).icon_id("binary").mode_def(edit::definition()).default_mode_id(edit::BINARY_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_binary_editor_builds_a_definition_for_the_editor_role() {
        let def = create_binary_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, BINARY_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<BinaryEditor as ArtifactEditor>::DIALECT, BINARY_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_binary_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_hex_dump_round_trips_a_rendered_snapshot() {
        let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
        let UiNode::ComponentScene(node) = main::render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        let parsed = parse_hex_dump(&scene.buffer).expect("well-formed hex dump must parse");
        assert_eq!(parsed, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_hex_dump_rejects_odd_length_hex() {
        assert!(parse_hex_dump("abc").is_none());
    }
}
//#endregion 🧪️Tests
