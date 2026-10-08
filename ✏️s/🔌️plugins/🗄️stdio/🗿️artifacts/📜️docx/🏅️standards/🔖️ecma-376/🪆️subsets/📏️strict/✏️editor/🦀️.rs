//! ✏️ Docx strict editor — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.docx@ecma-376/strict` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One
//! real window, `🪟️main` (`DocumentWindowKit`), rendering canonical WordprocessingML runs and
//! editing them through the artifact's own `DocxMutation::SetRunText`.

use crate::editor::docx::standards::v_ecma_376::subsets::strict::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::strict::modes::edit::windows::main;
use crate::schema::mutations::{edit_rules, DocxXmlAddress};
use crate::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactBuilder;
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

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/strict`. Duplicated (not imported) in the
/// sibling read-only surface root — never shared through this module, so that surface can never
/// depend on this one.
pub const DOCX_STRICT_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. The canonical address binds the XML part,
/// child path, expanded element name, and ancestor structure revision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum DocxStrictEditorCommand {
    SetPage { address: DocxXmlAddress, text: String },
    SetRunFormatting { address: DocxXmlAddress, bold: bool, italic: bool, underline: bool },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(DocxStrictEditorCommand, "docx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DocxStrictEditorCommand, ["set-page", "set-run-formatting"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ Maps one strictly addressed, revision-checked paragraph draft to a reversible mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, Fault> {
    crate::editor::docx::standards::v_ecma_376::subsets::base::preparation::prepare_set_run_text(snapshot, address, text)
        .map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.paged-owner-required"), message))
}

fn build_set_run_formatting_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, bold: bool, italic: bool, underline: bool) -> Result<Option<DocxMutation>, Fault> {
    crate::editor::docx::standards::v_ecma_376::subsets::base::build_set_run_formatting_mutation(snapshot, address, bold, italic, underline)
}

fn docx_action_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

fn required_docx_xml_address(args: Option<&semio_framework_value::DslValue>) -> Result<DocxXmlAddress, Fault> {
    let semio_framework_value::DslValue::Object(arguments) = args.ok_or_else(|| docx_action_fault("stdio.docx.set-page.arguments", "set-page requires an argument object"))? else {
        return Err(docx_action_fault("stdio.docx.set-page.arguments", "set-page requires an argument object"));
    };
    let mut addresses = arguments.iter().filter(|(name, _)| name == "address").map(|(_, value)| value);
    let Some(semio_framework_value::DslValue::Object(fields)) = addresses.next() else { return Err(docx_action_fault("stdio.docx.set-page.address-required", "set-page requires one canonical address object")) };
    if addresses.next().is_some() {
        return Err(docx_action_fault("stdio.docx.set-page.address-duplicate", "set-page address must occur exactly once"));
    }
    let field = |name: &str| -> Result<&semio_framework_value::DslValue, Fault> {
        let mut matches = fields.iter().filter(|(key, _)| key == name).map(|(_, value)| value);
        let value = matches.next().ok_or_else(|| docx_action_fault("stdio.docx.set-page.address-field", format!("DOCX address requires '{name}'")))?;
        if matches.next().is_some() {
            return Err(docx_action_fault("stdio.docx.set-page.address-field-duplicate", format!("DOCX address field '{name}' must occur exactly once")));
        }
        Ok(value)
    };
    if fields.len() != 4 {
        return Err(docx_action_fault("stdio.docx.set-page.address-fields", "DOCX address requires exactly partPath, nodePath, expectedName, and revision"));
    }
    let text = |name: &str| match field(name)? {
        semio_framework_value::DslValue::String(value) => Ok(value.clone()),
        _ => Err(docx_action_fault("stdio.docx.set-page.address-field-type", format!("DOCX address field '{name}' must be text"))),
    };
    let semio_framework_value::DslValue::Array(indices) = field("nodePath")? else { return Err(docx_action_fault("stdio.docx.set-page.node-path", "DOCX address nodePath must be an index array")) };
    let mut node_path = Vec::with_capacity(indices.len());
    for value in indices {
        let semio_framework_value::DslValue::Number(number) = value else { return Err(docx_action_fault("stdio.docx.set-page.node-path-index", "DOCX address nodePath entries must be unsigned integers")) };
        let index = number.as_u64().and_then(|value| usize::try_from(value).ok()).ok_or_else(|| docx_action_fault("stdio.docx.set-page.node-path-index", "DOCX address nodePath entries must fit the native index range"))?;
        node_path.push(index);
    }
    Ok(DocxXmlAddress { part_path: text("partPath")?, node_path, expected_name: text("expectedName")?, revision: text("revision")? })
}

crate::editor::docx::standards::v_ecma_376::subsets::base::canonical_docx_set_page_work!(DocxStrictSetPageWork, docx_strict_set_page_work, DocxStrictEditor, DocxStrictEditorCommand::SetPage, DocxStrictEditorCommand::SetRunFormatting);
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DocxStrictEditor;

impl ArtifactEditor for DocxStrictEditor {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DocxStrictEditorCommand>;

    const DIALECT: Dialect = DOCX_STRICT_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DOCX_DOCUMENT_SCHEMA;

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec {
            format_kind: "s.stdio.docx@ecma-376",
            extension: ".docx",
            media_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            binary: true,
        })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_docx(snapshot).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_docx(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/✏️editor/🦀️.rs",
        controller: "s.stdio.docx@ecma-376/strict#editor",
        artifact_schema: "stdio.docx",
        preparation: "stdio-docx-strict-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            DocxStrictEditorCommand::SetPage { .. } => "set-page",
            DocxStrictEditorCommand::SetRunFormatting { .. } => "set-run-formatting",
        })
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let address = required_docx_xml_address(args)?;
                let text = semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")?;
                Ok(DocxStrictEditorCommand::SetPage { address, text })
            }
            "set-run-formatting" => Ok(DocxStrictEditorCommand::SetRunFormatting {
                address: required_docx_xml_address(args)?,
                bold: crate::editor::docx::standards::v_ecma_376::subsets::base::required_docx_bool_argument(args, "bold")?,
                italic: crate::editor::docx::standards::v_ecma_376::subsets::base::required_docx_bool_argument(args, "italic")?,
                underline: crate::editor::docx::standards::v_ecma_376::subsets::base::required_docx_bool_argument(args, "underline")?,
            }),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.unhandled-action"), format!("unknown docx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DocxSnapshot {
        crate::standards::v_ecma_376::subsets::strict::io::DocxStrictBuilderConstruction::empty().add_paragraph(crate::schema::snapshot::DocxParagraph::default()).build().expect("the schema-authored initial strict DOCX is conformant")
    }

    /// ✏️ Replaces the addressed paragraph's text after its optimistic revision matches.
    /// The exact block/run structure is retained and invalid or stale targets return a fault.
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
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxStrictEditorCommand::SetPage { address, text }) => {
                let Some(mutation) = build_set_page_mutation(doc.snapshot, address, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxStrictEditorCommand::SetRunFormatting { address, bold, italic, underline }) => {
                let Some(mutation) = build_set_run_formatting_mutation(doc.snapshot, address, *bold, *italic, *underline)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.docx@ecma-376/strict#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for DocxStrictEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &edit_rules::EDIT_RULES
    }

    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot).map_err(|error| Fault::from(error.to_string()))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: DocxStrictEditor,
    tools: ["set-page", "set-run-formatting"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let mutation = match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxStrictEditorCommand::SetPage { address, text }) => build_set_page_mutation(snapshot, address, text)?,
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxStrictEditorCommand::SetRunFormatting { address, bold, italic, underline }) => build_set_run_formatting_mutation(snapshot, address, *bold, *italic, *underline)?,
            _ => return Err(Fault::from("stdio-docx-native-edit-command-mismatch")),
        };
        Ok(Emit { artifact_mutations: mutation.into_iter().collect(), ..Default::default() })
    },
    work: docx_strict_set_page_work,
    preparation_route: crate::editor::docx::standards::v_ecma_376::subsets::base::preparation::route,
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_docx_strict_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(DOCX_STRICT_EDITOR_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::DOCX_STRICT_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Document"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
