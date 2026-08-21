//! 🎒️ Zip editor (2.0/✳️iso21320) — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.zip@2.0/iso21320` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). Reuses the
//! SAME `ZipSnapshot`/`ZipMutation` Rust types as the sibling ✳️any subset — ISO/IEC 21320-1 is a
//! validation-gated dialect stamp on top of that existing schema, not a new one (this subset's own
//! `🧬️schema/🦀️component.rs` doc comment). One window, `🪟️main` (`TreeWindowKit`), renders the
//! archive as a tree; its `set-node` action funnels through the one typed command this surface
//! declares, `ZipEditorCommand::SetNode`, which renames either the archive comment or one entry's
//! name (see the window's own doc comment for the honest scope note).

use crate::artifacts::zip::{ZipMutation, ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use crate::editor::zip::iso21320::modes::edit;
use crate::editor::zip::iso21320::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode,
};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.zip@2.0/iso21320`, verified against this
/// artifact's own `🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320` location on disk, matching the SAME
/// literal already declared as `pub const DIALECT` at
/// `crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::DIALECT` (that module is
/// under `🧬️schema/**`, owned by the sibling peer ticket, so it is not imported from here — this is
/// an independent, verified-matching literal, not a duplication oversight).
pub const ZIP_ISO21320_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`set-node`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ZipEditorCommand {
    #[dsl(key = "set-zip-node")]
    SetNode { node_id: String, value: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for ZipEditorCommand {
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

impl protocol::OpBinary for ZipEditorCommand {
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

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct ZipIso21320Editor;

impl ArtifactEditor for ZipIso21320Editor {
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ZipEditorCommand;

    const DIALECT: Dialect = ZIP_ISO21320_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_ZIP_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> ZipSnapshot {
        ZipSnapshot::default()
    }

    /// ✏️ `node_id == "comment"` renames the archive comment; `"entry:{index}"` renames that entry's
    /// name. An unknown node id or out-of-range entry index is a documented no-op (`Emit::default()`),
    /// never a panic.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let ZipEditorCommand::SetNode { node_id, value } = command;
        if node_id == main::COMMENT_NODE_ID {
            return Ok(Emit { artifact_mutations: vec![ZipMutation::SetArchiveComment { comment: value.clone() }], description: Some("Set comment".into()), ..Default::default() });
        }
        let Some(index_text) = node_id.strip_prefix(main::ENTRY_NODE_PREFIX) else { return Ok(Emit::default()) };
        let Ok(index) = index_text.parse::<usize>() else { return Ok(Emit::default()) };
        let Some(entry) = doc.snapshot.entries.get(index) else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![ZipMutation::RenameEntry { name: entry.name.clone(), new_name: value.clone() }], description: Some("Rename entry".into()), ..Default::default() })
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
pub fn create_zip_iso21320_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(ZIP_ISO21320_EDITOR_DIALECT)
        .document(["stdio", "zip", "iso21320"])
        .icon_id("archive")
        .mode_def(edit::definition())
        .default_mode_id(edit::ZIP_ISO21320_EDIT_MODE_ID)
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
    async fn create_zip_iso21320_editor_builds_a_definition_for_the_editor_role() {
        let def = create_zip_iso21320_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, ZIP_ISO21320_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<ZipIso21320Editor as ArtifactEditor>::DIALECT, ZIP_ISO21320_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_zip_iso21320_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
