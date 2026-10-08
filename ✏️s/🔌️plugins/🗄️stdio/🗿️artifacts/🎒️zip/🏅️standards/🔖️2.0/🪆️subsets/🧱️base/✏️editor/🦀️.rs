//! 🎒️ Archive editor with guarded text drafts and complete snapshot Details.

use crate::editor::zip::base::modes::edit;
use crate::editor::zip::base::modes::edit::windows::main;
use crate::schema::mutations::add_entry;
use crate::{ZipMutation, ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_ui_locale::Label;

#[path = "📬️preparation/🦀️.rs"]
mod preparation;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.zip@2.0/*`, verified against this artifact's own
/// `🏅️standards/🔖️2.0/🪆️subsets/🧱️base` location on disk. No reusable standalone `pub const DIALECT`
/// exists for this exact subset (the artifact root has none; the 🧱️base subset's own schema module
/// embeds the same literal inline inside an `impl ArtifactAnalysis` block, not as a free const), so
/// it is inlined here and reused for both `impl ArtifactEditor` and the manifest below.
pub const ZIP_ANY_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`set-node`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum ZipEditorCommand {
    #[dsl(key = "set-zip-node")]
    SetNode { node_id: String, value: String, revision: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for ZipEditorCommand {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for ZipEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(ZipEditorCommand, ["set-node"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct ZipAnyEditor;

impl ArtifactEditor for ZipAnyEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<ZipEditorCommand>;

    const DIALECT: Dialect = ZIP_ANY_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_ZIP_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.zip@2.0/*#editor",
        artifact_schema: "stdio.zip",
        preparation: "stdio-zip-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-node")
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        crate::editor::editing::agent_target_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-node" => {
                let (node_id, value, revision) = crate::editor::editing::edit_arguments(args)?;
                Ok(ZipEditorCommand::SetNode { node_id, value, revision })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.zip.unhandled-action"), format!("unknown zip editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> ZipSnapshot {
        ZipSnapshot::default()
    }

    /// ✏️ Renames one unambiguous entry or changes the archive comment.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(ZipEditorCommand::SetNode { node_id, value, revision }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        crate::editor::editing::edit_node(doc.snapshot, node_id, value, revision)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.zip@2.0/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::BoundedNativeEditingEditor for ZipAnyEditor {
    const NATIVE_TOOL_IDS: &'static [&'static str] = &["set-node"];
    const NATIVE_PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] =
        &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "set-node", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] }];
    const NATIVE_PAYLOAD_SCHEMA: &'static str = "s.stdio.zip.command.set-node.v1";
    const NATIVE_CHECKPOINT_RESUME: bool = true;

    fn native_edit_work(_tool_id: &'static str) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> {
        Box::new(crate::editor::editing::retained::ArchiveTextWork::<Self>::new(|command| {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(ZipEditorCommand::SetNode { node_id, value, revision }) = command else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.zip.command-mismatch"), "archive text editing received another command"));
            };
            Ok((node_id.as_str(), value.as_str(), revision.as_str()))
        }))
    }

    fn native_edit_mutations(command: &Self::Command, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(ZipEditorCommand::SetNode { node_id, value, revision }) = command else {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.zip.command-mismatch"), "archive text editing received another command"));
        };
        crate::editor::editing::edit_node(snapshot, node_id, value, revision)
    }

    fn native_edit_preparation_route(prefix: &'static str) -> Option<semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute<Self::Snapshot, Self::Mutation>> {
        preparation::route(prefix)
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for ZipAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &crate::editor::zip::base::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::InsertValue { path, value } = event else { return Ok(None) };
        let Some(position) = path.strip_prefix("/entries/") else { return Ok(None) };
        let fail = |message: String| Fault::from(message);
        let index = if position == "-" { snapshot.entries.len() } else { position.parse::<usize>().map_err(|error| fail(error.to_string()))? };
        let entry = <crate::schema::snapshot::ZipEntry as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
        Ok(Some(vec![ZipMutation::AddEntry(add_entry::AddEntry { entry, before: snapshot.entries.get(index).map(|following| following.name.clone()) })]))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_zip_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(ZIP_ANY_EDITOR_DIALECT)
        .document(["stdio", "zip", "any"])
        .icon_id("archive")
        .mode_def(edit::definition())
        .default_mode_id(edit::ZIP_ANY_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Archive"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
