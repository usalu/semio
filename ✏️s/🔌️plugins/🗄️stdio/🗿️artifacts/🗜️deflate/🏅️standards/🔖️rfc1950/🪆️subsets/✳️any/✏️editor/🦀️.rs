//! 🗜️ Deflate editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.deflate@rfc1950/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `DeflateSnapshot` is a typed RFC1950
//! zlib container: CMF/FLG header fields plus an opaque decompressed `payload`. One window,
//! `🪟️main` (`TextWindowKit`), renders the header fields as an editable `key=value` summary; its
//! `replace-text` action funnels through the one typed command this surface declares,
//! `DeflateEditorCommand::ReplaceText`, which parses the summary back into
//! `SetCompressionParams`/`SetPresetDictionary` (see the window's own doc comment for why `payload`
//! itself is never shown or parsed here — a compressed byte stream has no honest text form).

use crate::artifacts::deflate::schema::mutations::{set_compression_params, set_preset_dictionary};
use crate::artifacts::deflate::{DeflateMutation, DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::editor::deflate::modes::edit;
use crate::editor::deflate::modes::edit::windows::main;
#[cfg(test)]
use semio_framework_plugin::Component;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.deflate@rfc1950/*`, verified against this
/// artifact's own `🏅️standards/🔖️rfc1950/🪆️subsets/✳️any` location on disk. No reusable const
/// exists on the artifact's own root `🦀️.rs` (checked before adding this), so it is
/// inlined here and reused for both `impl ArtifactEditor` and the manifest below.
pub const DEFLATE_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum DeflateEditorCommand {
    #[dsl(key = "replace-deflate-text")]
    ReplaceText { text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for DeflateEditorCommand {
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

impl protocol::OpBinary for DeflateEditorCommand {
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
/// 📐️ Parses `render()`'s `key=value` summary back into `(method, window_bits, level_hint, dict_id)`.
/// `#`-prefixed and blank lines are ignored (the payload byte-count comment). `None` on any missing
/// or malformed required key — the caller treats that as a whole-command no-op, never a partial
/// apply.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_header_summary(text: &str) -> Option<(u8, u8, crate::artifacts::deflate::schema::snapshot::DeflateLevelHint, Option<u32>)> {
    let mut fields = std::collections::BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=')?;
        fields.insert(key.trim(), value.trim());
    }
    let method = fields.get("method")?.parse::<u8>().ok()?;
    let window_bits = fields.get("windowBits")?.parse::<u8>().ok()?;
    let level_hint = main::parse_level_hint(fields.get("levelHint")?)?;
    let dict_id = match fields.get("presetDictionary").copied() {
        None | Some("none") => None,
        Some(other) => Some(other.parse::<u32>().ok()?),
    };
    Some((method, window_bits, level_hint, dict_id))
}
//#endregion 🔖️TextParse

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DeflateEditor;

impl ArtifactEditor for DeflateEditor {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DeflateEditorCommand;

    const DIALECT: Dialect = DEFLATE_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DEFLATE_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> DeflateSnapshot {
        DeflateSnapshot::default()
    }

    /// ✏️ Parses the whole `key=value` summary and, if every required field is present and valid,
    /// emits BOTH `SetCompressionParams` and `SetPresetDictionary` as one gesture. A malformed
    /// summary (missing/unparsable required field) is a documented no-op (`Emit::default()`), never
    /// a partial apply.
    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let DeflateEditorCommand::ReplaceText { text } = command;
        let Some((method, window_bits, level_hint, dict_id)) = parse_header_summary(text) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method, window_bits, level_hint }), DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id })], description: Some("Set compression header".into()), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_deflate_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(DEFLATE_EDITOR_DIALECT)
        .document(["stdio", "deflate"])
        .icon_id("package")
        .mode_def(edit::definition())
        .default_mode_id(edit::DEFLATE_EDIT_MODE_ID)
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
    async fn create_deflate_editor_builds_a_definition_for_the_editor_role() {
        let def = create_deflate_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, DEFLATE_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DeflateEditor as ArtifactEditor>::DIALECT, DEFLATE_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_deflate_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_header_summary_round_trips_a_rendered_snapshot() {
        let document = DeflateSnapshot { compression_method: 8, window_bits: 9, compression_level_hint: crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::Maximum, dict_id: Some(7), payload: vec![9, 9], ..DeflateSnapshot::default() };
        let node = main::render(&document).expect("render");
        let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
        let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
        let (method, window_bits, level_hint, dict_id) = parse_header_summary(&scene.buffer).expect("well-formed summary must parse");
        assert_eq!(method, 8);
        assert_eq!(window_bits, 9);
        assert_eq!(level_hint, crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::Maximum);
        assert_eq!(dict_id, Some(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_header_summary_rejects_a_missing_required_field() {
        assert!(parse_header_summary("method=8\nwindowBits=7").is_none());
    }
}
//#endregion 🧪️Tests
