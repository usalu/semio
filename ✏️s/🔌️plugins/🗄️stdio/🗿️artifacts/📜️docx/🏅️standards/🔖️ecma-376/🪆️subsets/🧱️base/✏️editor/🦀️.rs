//! ✏️ Docx editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.docx@ecma-376/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`DocumentWindowKit`), rendering one draft per paragraph run and editing it through the
//! artifact's own `DocxMutation::SetRunText`.

use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit::windows::main;
use crate::schema::mutations::{docx_run_formatting, edit_rules, prepare_addressed_xml_mutation, set_run_formatting, set_run_text, DocxRunFormatting, DocxXmlAddress};
use crate::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::ArtifactCommandInputs;
use semio_framework_plugin::retained_command::ArtifactCommandWork;
use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
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
pub(crate) mod preparation;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/*` (the unrestricted `any` subset). Duplicated
/// (not imported) in the sibling read-only surface root — never shared through this module, so
/// that surface can never depend on this one.
pub const DOCX_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `page` addresses `DocxDocument.body` and `item`
/// addresses the paragraph's original run ordinal.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum DocxEditorCommand {
    SetPage { address: DocxXmlAddress, text: String },
    SetRunFormatting { address: DocxXmlAddress, bold: bool, italic: bool, underline: bool },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(DocxEditorCommand, "docx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DocxEditorCommand, ["set-page", "set-run-formatting"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ Maps one strictly addressed, revision-checked paragraph draft to a reversible mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, Fault> {
    preparation::prepare_set_run_text(snapshot, address, text).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.paged-owner-required"), message))
}

pub(crate) fn build_set_run_formatting_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, bold: bool, italic: bool, underline: bool) -> Result<Option<DocxMutation>, Fault> {
    let current = docx_run_formatting(snapshot, address).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-run-formatting.address"), error.into_message()))?;
    if current == (DocxRunFormatting { bold: Some(bold), italic: Some(italic), underline: Some(underline) }) {
        return Ok(None);
    }
    let mutation = DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: address.clone(), bold, italic, underline });
    let prepared = prepare_addressed_xml_mutation(snapshot, &mutation).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-run-formatting.address"), error.into_message()))?;
    Ok(prepared.changed.then_some(mutation))
}

fn docx_action_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

pub(crate) fn required_docx_xml_address(args: Option<&semio_framework_value::DslValue>) -> Result<DocxXmlAddress, Fault> {
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

pub(crate) fn required_docx_bool_argument(args: Option<&semio_framework_value::DslValue>, name: &str) -> Result<bool, Fault> {
    let semio_framework_value::DslValue::Object(arguments) = args.ok_or_else(|| docx_action_fault("stdio.docx.set-run-formatting.arguments", "set-run-formatting requires an argument object"))? else {
        return Err(docx_action_fault("stdio.docx.set-run-formatting.arguments", "set-run-formatting requires an argument object"));
    };
    let mut matches = arguments.iter().filter(|(key, _)| key == name).map(|(_, value)| value);
    let value = matches.next().ok_or_else(|| docx_action_fault("stdio.docx.set-run-formatting.field-required", format!("set-run-formatting requires '{name}'")))?;
    if matches.next().is_some() {
        return Err(docx_action_fault("stdio.docx.set-run-formatting.field-duplicate", format!("set-run-formatting field '{name}' must occur exactly once")));
    }
    match value {
        semio_framework_value::DslValue::Bool(value) => Ok(*value),
        _ => Err(docx_action_fault("stdio.docx.set-run-formatting.field-type", format!("set-run-formatting field '{name}' must be boolean"))),
    }
}

pub(crate) const DOCX_TEXT_WORK_PAGE_BYTES: usize = 4_096;

struct DocxSetPageWork {
    tool_id: &'static str,
    copied_text: semio_s_artifact_stdio_contract::editing::RetainedTextCopy,
    validated: bool,
    complete: bool,
    closing: bool,
}

impl Default for DocxSetPageWork {
    fn default() -> Self {
        Self { tool_id: "set-page", copied_text: Default::default(), validated: false, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<DocxEditor>> for DocxSetPageWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &<DocxEditor as ArtifactEditor>::Command,
        _snapshot: &DocxSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<DocxEditor>>>,
    ) -> Option<usize> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address, text }) => {
                let address_bytes = address.part_path.len().checked_add(address.expected_name.len())?.checked_add(address.revision.len())?.checked_add(address.node_path.len().checked_mul(std::mem::size_of::<usize>())?)?;
                (address_bytes <= DOCX_TEXT_WORK_PAGE_BYTES && text.len() <= semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES).then_some(text.len().div_ceil(DOCX_TEXT_WORK_PAGE_BYTES).saturating_add(1))
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address, .. }) => {
                let address_bytes = address.part_path.len().checked_add(address.expected_name.len())?.checked_add(address.revision.len())?.checked_add(address.node_path.len().checked_mul(std::mem::size_of::<usize>())?)?;
                (address_bytes <= DOCX_TEXT_WORK_PAGE_BYTES).then_some(1)
            }
            _ => None,
        }
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<DocxEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        docx_work_demand(input.command)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<DocxEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<DocxEditor>>, Fault> {
        if self.closing || self.complete {
            return Err(Fault::from("stdio.docx.set-page.work-closed"));
        }
        if let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address, bold, italic, underline }) = input.command {
            let mutation = build_set_run_formatting_mutation(input.snapshot, address, *bold, *italic, *underline)?;
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: mutation.into_iter().collect(), ..Default::default() }));
        }
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address, text }) = input.command else { return Err(Fault::from("stdio.docx.command-mismatch")) };
        if !self.validated {
            let Some(_) = build_set_page_mutation(input.snapshot, address, text)? else {
                self.complete = true;
                return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
            };
            self.validated = true;
        }
        self.copied_text.advance(text, DOCX_TEXT_WORK_PAGE_BYTES).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.copy-refused"), message))?;
        if !self.copied_text.is_complete() {
            return Ok(ArtifactCommandWorkStep::Replay { stage: "docx-copy-run-text", preview: br#"{"en":"Preparing paragraph text","de":"Absatztext wird vorbereitet"}"# });
        }
        let copied = self.copied_text.take().ok_or_else(|| Fault::from("stdio.docx.set-page.copy-incomplete"))?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: vec![DocxMutation::SetRunText(set_run_text::SetRunText { address: address.clone(), text: copied })], ..Default::default() }))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        self.copied_text.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.copied_text.close_demands().copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.copied_text.close_demands().capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.copied_text.close_demands().release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.copied_text.close_demands().depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.copied_text.terminal_is_empty()
    }
}

/// 📏️ The copy and capacity quote of the address strings one formatting mutation owns.
pub(crate) fn docx_address_demand(address: &DocxXmlAddress) -> semio_framework_value::RetirementDemand {
    let bytes = address.part_path.len().saturating_add(address.expected_name.len()).saturating_add(address.revision.len()).saturating_add(address.node_path.len().saturating_mul(std::mem::size_of::<usize>()));
    semio_framework_value::RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes.saturating_add(std::mem::size_of::<DocxMutation>()), release_bytes: 0, depth: 1 }
}

/// 📏️ The quote of one set-page turn: one admitted text page plus the address and mutation owners.
pub(crate) fn docx_set_page_demand(address: &DocxXmlAddress, text: &str) -> semio_framework_value::RetirementDemand {
    let mut demand = docx_address_demand(address);
    let page = text.len().min(DOCX_TEXT_WORK_PAGE_BYTES);
    demand.copy_bytes = demand.copy_bytes.max(page);
    demand.capacity_bytes = demand.capacity_bytes.saturating_add(text.len());
    demand
}

fn docx_work_demand(command: &<DocxEditor as ArtifactEditor>::Command) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    match command {
        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address, text }) => Ok(docx_set_page_demand(address, text)),
        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address, .. }) => Ok(docx_address_demand(address)),
        _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "stdio.docx.command-mismatch")),
    }
}

fn docx_set_page_work(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<DocxEditor>>> {
    Box::new(DocxSetPageWork { tool_id, ..Default::default() })
}

macro_rules! canonical_docx_set_page_work {
    ($work:ident, $factory:ident, $editor:ty, $page_command:path, $formatting_command:path) => {
        struct $work {
            tool_id: &'static str,
            copied_text: semio_s_artifact_stdio_contract::editing::RetainedTextCopy,
            validated: bool,
            complete: bool,
            closing: bool,
        }

        impl Default for $work {
            fn default() -> Self {
                Self { tool_id: "set-page", copied_text: Default::default(), validated: false, complete: false, closing: false }
            }
        }

        impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<$editor>> for $work {
            fn tool_id(&self) -> &'static str {
                self.tool_id
            }

            fn extent(
                &self,
                command: &<$editor as semio_framework_plugin::ArtifactEditor>::Command,
                _snapshot: &DocxSnapshot,
                _interaction: &protocol::InteractionState,
                _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<$editor>>>,
            ) -> Option<usize> {
                match command {
                    semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($page_command { address, text }) => {
                        let address_bytes = address.part_path.len().checked_add(address.expected_name.len())?.checked_add(address.revision.len())?.checked_add(address.node_path.len().checked_mul(std::mem::size_of::<usize>())?)?;
                        (address_bytes <= crate::editor::docx::standards::v_ecma_376::subsets::base::DOCX_TEXT_WORK_PAGE_BYTES && text.len() <= semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES)
                            .then_some(text.len().div_ceil(crate::editor::docx::standards::v_ecma_376::subsets::base::DOCX_TEXT_WORK_PAGE_BYTES).saturating_add(1))
                    }
                    semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($formatting_command { address, .. }) => {
                        let address_bytes = address.part_path.len().checked_add(address.expected_name.len())?.checked_add(address.revision.len())?.checked_add(address.node_path.len().checked_mul(std::mem::size_of::<usize>())?)?;
                        (address_bytes <= crate::editor::docx::standards::v_ecma_376::subsets::base::DOCX_TEXT_WORK_PAGE_BYTES).then_some(1)
                    }
                    _ => None,
                }
            }

            fn work_demands(&self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<$editor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
                match input.command {
                    semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($page_command { address, text }) => Ok(crate::editor::docx::standards::v_ecma_376::subsets::base::docx_set_page_demand(address, text)),
                    semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($formatting_command { address, .. }) => Ok(crate::editor::docx::standards::v_ecma_376::subsets::base::docx_address_demand(address)),
                    _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "stdio.docx.command-mismatch")),
                }
            }

            fn step(
                &mut self,
                input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<$editor>>,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<$editor>>, semio_framework_plugin::Fault> {
                if self.closing || self.complete {
                    return Err(semio_framework_plugin::Fault::from("stdio.docx.set-page.work-closed"));
                }
                if let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($formatting_command { address, bold, italic, underline }) = input.command {
                    let mutation = crate::editor::docx::standards::v_ecma_376::subsets::base::build_set_run_formatting_mutation(input.snapshot, address, *bold, *italic, *underline)?;
                    self.complete = true;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(semio_framework_plugin::Emit { artifact_mutations: mutation.into_iter().collect(), ..Default::default() }));
                }
                let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($page_command { address, text }) = input.command else {
                    return Err(semio_framework_plugin::Fault::from("stdio.docx.command-mismatch"));
                };
                if !self.validated {
                    let Some(_) = build_set_page_mutation(input.snapshot, address, text)? else {
                        self.complete = true;
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(semio_framework_plugin::Emit::default()));
                    };
                    self.validated = true;
                }
                self.copied_text
                    .advance(text, crate::editor::docx::standards::v_ecma_376::subsets::base::DOCX_TEXT_WORK_PAGE_BYTES)
                    .map_err(|message| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.copy-refused"), message))?;
                if !self.copied_text.is_complete() {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage: "docx-copy-run-text", preview: br#"{"en":"Preparing paragraph text","de":"Absatztext wird vorbereitet"}"# });
                }
                let copied = self.copied_text.take().ok_or_else(|| semio_framework_plugin::Fault::from("stdio.docx.set-page.copy-incomplete"))?;
                self.complete = true;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(semio_framework_plugin::Emit {
                    artifact_mutations: vec![DocxMutation::SetRunText(crate::schema::mutations::set_run_text::SetRunText { address: address.clone(), text: copied })],
                    ..Default::default()
                }))
            }

            fn begin_close(&mut self) {
                self.closing = true;
            }

            fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
                if !self.closing {
                    return semio_framework_job::InteractiveJobCloseStep::Blocked;
                }
                self.copied_text.close_step(grant)
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(self.copied_text.close_demands().copy_bytes)
            }

            fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
                Ok(self.copied_text.close_demands().capacity_bytes)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(self.copied_text.close_demands().release_bytes)
            }

            fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(self.copied_text.close_demands().depth)
            }

            fn terminal_is_empty(&self) -> bool {
                self.closing && self.copied_text.terminal_is_empty()
            }
        }

        fn $factory(tool_id: &'static str) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<$editor>>> {
            Box::new($work { tool_id, ..Default::default() })
        }
    };
}

pub(crate) use canonical_docx_set_page_work;
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DocxEditor;

impl ArtifactEditor for DocxEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DocxEditorCommand>;

    const DIALECT: Dialect = DOCX_EDITOR_DIALECT;
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
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.docx@ecma-376/*#editor",
        artifact_schema: "stdio.docx",
        preparation: "stdio-docx-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            DocxEditorCommand::SetPage { .. } => "set-page",
            DocxEditorCommand::SetRunFormatting { .. } => "set-run-formatting",
        })
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let address = required_docx_xml_address(args)?;
                let text = semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")?;
                Ok(DocxEditorCommand::SetPage { address, text })
            }
            "set-run-formatting" => Ok(DocxEditorCommand::SetRunFormatting {
                address: required_docx_xml_address(args)?,
                bold: required_docx_bool_argument(args, "bold")?,
                italic: required_docx_bool_argument(args, "italic")?,
                underline: required_docx_bool_argument(args, "underline")?,
            }),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.unhandled-action"), format!("unknown docx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DocxSnapshot {
        crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(crate::schema::snapshot::DocxDocument {
            body: vec![crate::schema::snapshot::DocxBlock::Paragraph(crate::schema::snapshot::DocxParagraph::default())],
            styles: Vec::new(),
        })
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
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address, text }) => {
                let Some(mutation) = build_set_page_mutation(doc.snapshot, address, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address, bold, italic, underline }) => {
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
                "s.stdio.docx@ecma-376/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for DocxEditor {
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
    editor: DocxEditor,
    tools: ["set-page", "set-run-formatting"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let mutation = match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address, text }) => build_set_page_mutation(snapshot, address, text)?,
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address, bold, italic, underline }) => build_set_run_formatting_mutation(snapshot, address, *bold, *italic, *underline)?,
            _ => return Err(Fault::from("stdio-docx-native-edit-command-mismatch")),
        };
        Ok(Emit { artifact_mutations: mutation.into_iter().collect(), ..Default::default() })
    },
    work: docx_set_page_work,
    preparation_route: preparation::route,
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_docx_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(DOCX_EDITOR_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::DOCX_EDIT_MODE_ID)
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
