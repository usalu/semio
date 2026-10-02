//! ✏️ Docx transitional editor — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.docx@ecma-376/transitional` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
//! One real window, `🪟️main` (`DocumentWindowKit`), rendering canonical WordprocessingML runs and
//! editing them through the artifact's own `DocxMutation::SetRunText`.

use crate::editor::docx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use crate::schema::mutations::{set_snapshot, DocxXmlAddress};
use crate::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
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

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/transitional`. Duplicated (not imported) in the
/// sibling read-only surface root — never shared through this module, so that surface can never
/// depend on this one.
pub const DOCX_TRANSITIONAL_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. The canonical address binds the XML part,
/// child path, expanded element name, and ancestor structure revision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum DocxTransitionalEditorCommand {
    SetPage { address: DocxXmlAddress, text: String },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(DocxTransitionalEditorCommand, "docx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DocxTransitionalEditorCommand, ["set-page"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ Maps one strictly addressed, revision-checked paragraph draft to a reversible mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, Fault> {
    crate::editor::docx::standards::v_ecma_376::subsets::base::preparation::prepare_set_run_text(snapshot, address, text)
        .map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.paged-owner-required"), message))
}

fn docx_action_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

fn required_docx_xml_address(args: Option<&dsl::DslValue>) -> Result<DocxXmlAddress, Fault> {
    let dsl::DslValue::Object(arguments) = args.ok_or_else(|| docx_action_fault("stdio.docx.set-page.arguments", "set-page requires an argument object"))? else {
        return Err(docx_action_fault("stdio.docx.set-page.arguments", "set-page requires an argument object"));
    };
    let mut addresses = arguments.iter().filter(|(name, _)| name == "address").map(|(_, value)| value);
    let Some(dsl::DslValue::Object(fields)) = addresses.next() else { return Err(docx_action_fault("stdio.docx.set-page.address-required", "set-page requires one canonical address object")) };
    if addresses.next().is_some() {
        return Err(docx_action_fault("stdio.docx.set-page.address-duplicate", "set-page address must occur exactly once"));
    }
    let field = |name: &str| -> Result<&dsl::DslValue, Fault> {
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
        dsl::DslValue::String(value) => Ok(value.clone()),
        _ => Err(docx_action_fault("stdio.docx.set-page.address-field-type", format!("DOCX address field '{name}' must be text"))),
    };
    let dsl::DslValue::Array(indices) = field("nodePath")? else { return Err(docx_action_fault("stdio.docx.set-page.node-path", "DOCX address nodePath must be an index array")) };
    let mut node_path = Vec::with_capacity(indices.len());
    for value in indices {
        let dsl::DslValue::Number(number) = value else { return Err(docx_action_fault("stdio.docx.set-page.node-path-index", "DOCX address nodePath entries must be unsigned integers")) };
        let index = number.as_u64().and_then(|value| usize::try_from(value).ok()).ok_or_else(|| docx_action_fault("stdio.docx.set-page.node-path-index", "DOCX address nodePath entries must fit the native index range"))?;
        node_path.push(index);
    }
    Ok(DocxXmlAddress { part_path: text("partPath")?, node_path, expected_name: text("expectedName")?, revision: text("revision")? })
}

crate::editor::docx::standards::v_ecma_376::subsets::base::canonical_docx_set_page_work!(DocxTransitionalSetPageWork, docx_transitional_set_page_work, DocxTransitionalEditor, DocxTransitionalEditorCommand::SetPage);
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DocxTransitionalEditor;

impl ArtifactEditor for DocxTransitionalEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DocxTransitionalEditorCommand>;

    const DIALECT: Dialect = DOCX_TRANSITIONAL_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DOCX_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/✏️editor/🦀️.rs",
        controller: "s.stdio.docx@ecma-376/transitional#editor",
        artifact_schema: "stdio.docx",
        preparation: "stdio-docx-transitional-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-page")
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let address = required_docx_xml_address(args)?;
                let text = semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")?;
                Ok(DocxTransitionalEditorCommand::SetPage { address, text })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.unhandled-action"), format!("unknown docx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DocxSnapshot {
        DocxSnapshot::default()
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
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxTransitionalEditorCommand::SetPage { address, text }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let Some(mutation) = build_set_page_mutation(doc.snapshot, address, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.docx@ecma-376/transitional#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for DocxTransitionalEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: DocxTransitionalEditor,
    tools: ["set-page"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxTransitionalEditorCommand::SetPage { address, text }) = command else {
            return Err(Fault::from("stdio-docx-native-edit-command-mismatch"));
        };
        let Some(mutation) = build_set_page_mutation(snapshot, address, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
    },
    work: docx_transitional_set_page_work,
    preparation_route: crate::editor::docx::standards::v_ecma_376::subsets::base::preparation::route,
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_docx_transitional_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(DOCX_TRANSITIONAL_EDITOR_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::DOCX_TRANSITIONAL_EDIT_MODE_ID)
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
