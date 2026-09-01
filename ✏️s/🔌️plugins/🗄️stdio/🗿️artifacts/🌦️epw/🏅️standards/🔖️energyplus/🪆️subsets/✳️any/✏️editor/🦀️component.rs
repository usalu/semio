//! 🌦️ EPW editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.epw@energyplus/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `EpwSnapshot` is a flat lossless
//! record: 8 verbatim header lines + a `Vec<EpwRecord>` of hourly weather rows. One window,
//! `🪟️main` (`TableWindowKit`), renders the record list directly — no composed-child indirection is
//! needed since `EpwSnapshot` already IS the persisted document. The window's `set-cell` action
//! funnels through the one typed command this surface declares, `EpwEditorCommand::SetCell`, which
//! maps a column name to its canonical wire index (`EpwRecord::field_at`) and emits
//! `EpwMutation::SetRecordField` directly — no whole-document decode/re-encode round trip is needed
//! (unlike `energy`'s composed-child `Model`, `EpwSnapshot`'s fields ARE the wire fields).

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::set_record_field;
use crate::artifacts::epw::{EpwMutation, EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
use crate::editor::epw::modes::edit;
use crate::editor::epw::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.epw@energyplus/*`, verified against this
/// artifact's own `🏅️standards/🔖️energyplus/🪆️subsets/✳️any` location on disk. No reusable
/// `pub const DIALECT` exists on the artifact's own root `🦀️component.rs` (checked before adding
/// this), so it is inlined here and reused for both `impl ArtifactEditor` and the manifest below.
pub const EPW_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`set-cell`, contract §2.6) can trigger. Scope note: only the 35
/// per-record columns are addressable — the 8 verbatim header lines (LOCATION, DESIGN CONDITIONS, …)
/// have no cell in a flat record table, so they are not yet editable through this surface
/// (documented honestly, matching energy's own `SetStructureField` scope note).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum EpwEditorCommand {
    #[dsl(key = "set-record-cell")]
    SetCell { row: u32, column: String, value: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for EpwEditorCommand {
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

impl protocol::OpBinary for EpwEditorCommand {
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
pub struct EpwEditor;

impl ArtifactEditor for EpwEditor {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EpwEditorCommand;

    const DIALECT: Dialect = EPW_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_EPW_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> EpwSnapshot {
        EpwSnapshot::default()
    }

    /// ✏️ Resolves the addressed column to its canonical wire index and emits one
    /// `EpwMutation::SetRecordField`. An out-of-range row or unknown column is a documented no-op
    /// (`Emit::default()`), never a panic.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let EpwEditorCommand::SetCell { row, column, value } = command;
        let Some(field_index) = main::EPW_TABLE_COLUMNS.iter().position(|candidate| candidate == column) else { return Ok(Emit::default()) };
        if doc.snapshot.records.get(*row as usize).is_none() {
            return Ok(Emit::default());
        }
        Ok(Emit { artifact_mutations: vec![EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: *row as usize, field_index, value: value.clone() })], description: Some(format!("Set {column}")), ..Default::default() })
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
pub fn create_epw_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(EPW_EDITOR_DIALECT).document(["stdio", "epw"]).icon_id("cloud-sun").mode_def(edit::definition()).default_mode_id(edit::EPW_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_epw_editor_builds_a_definition_for_the_editor_role() {
        let def = create_epw_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, EPW_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EpwEditor as ArtifactEditor>::DIALECT, EPW_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_epw_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
