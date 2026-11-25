//! 📄️ PDF 1.4 addressed page text drafts over its resolved page domain; the page canvas and page structure live in `semio-s-artifact-stdio-pdf14-canvas`.
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::{PdfMutation, ReplacePageText};
use semio_plugin::{Fault, FaultCode, FaultOrigin, TreeWindows, ActionArgDef, ActionDefinition, ActionKind, WindowKindDefinition, PluginAssemblyError, UiAssemblyResult, Buildable, HasBase, UiText};
use semio_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_plugin::plugin_app_close_prelude as ui;
use semio_ui_contract::{BuiltNode, UiPublicationRevision};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_framework_value::DslValue;

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;

fn refusal(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.page-edit"), message)
}

fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "PDF 1.4 page rendering admission failed")
}

/// 🪟️ Declares the addressed text drafts and their revision-guarded replacement.
pub fn window_definition() -> WindowKindDefinition {
    let mut definition = DocumentWindowKit::editable_window_kind();
    definition.label = LocalizedLabel::native("Page Text", "Seitentext");
    let action = |id, en, de, args| {
        let mut definition = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
        definition.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
        definition
    };
    definition.actions.extend([
        action("replace-page-text", "Replace Page Text", "Seitentext ersetzen", vec![ActionArgDef::index("page", LocalizedLabel::native("Page index", "Seitenindex")).required(), ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).required(), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()]),
    ]);
    definition
}

/// 📄️ Projects each complete own text target with the actual publication revision.
pub fn editable_view(document: &PdfSnapshot, publication_revision: UiPublicationRevision) -> Result<EditableDocumentView, Fault> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| {
        let index = u32::try_from(index).map_err(|_| refusal("Page index exceeds the document action address"))?;
        Ok(EditableDocumentPage::new(index, 0, page.text.clone()))
    }).collect::<Result<Vec<_>, Fault>>()?;
    Ok(EditableDocumentView { pages, publication_revision })
}

/// 📐️ Renders dimensions beside each materialized draft without changing its text revision.
pub fn render_windowed(document: &PdfSnapshot, publication_revision: UiPublicationRevision, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let view = editable_view(document, publication_revision).map_err(|error| PluginAssemblyError::new("stdio.pdf.1.4.page-view", error.message))?;
    DocumentWindowKit::render_editable_windowed_with_accessory(&view, windows, locale, |ordinal, _| {
        let page = &document.pages[ordinal];
        let caption = match locale {
            Locale::En => format!("Width {} × Height {}", semio_framework_dsl::format_f64(page.width), semio_framework_dsl::format_f64(page.height)),
            Locale::De => format!("Breite {} × Höhe {}", semio_framework_dsl::format_f64(page.width), semio_framework_dsl::format_f64(page.height)),
        };
        let label = UiText::try_from_str(&caption).ok_or_else(|| ui_error("stdio.pdf.1.4.page-dimensions-text"))?;
        let node = ui::text(ui::Label(label)).try_id("page-dimensions").map_err(|_| ui_error("stdio.pdf.1.4.page-dimensions-id"))?.try_build().map_err(|_| ui_error("stdio.pdf.1.4.page-dimensions-build"))?;
        Ok(Some(node))
    })
}

/// 🔐️ Reads the current text token for an explicitly addressed agent action.
pub fn agent_target_revision(action: &str, document: &PdfSnapshot, args: &DslValue) -> Result<Option<String>, Fault> {
    if !matches!(action, "set-page" | "replace-page-text") { return Ok(None); }
    let page = index(Some(args), "page")?;
    if action == "set-page" && index(Some(args), "item")? != 0 { return Err(refusal("PageDoc owns exactly one text item")); }
    let target = document.pages.get(page).ok_or_else(|| refusal("Page target does not exist"))?;
    Ok(Some(DocumentWindowKit::text_revision(&target.text)))
}

/// ✏️ Produces one own text mutation after the rendered target revision matches.
pub fn text_edit_mutation(document: &PdfSnapshot, page: u32, item: u32, revision: &str, text: &str) -> Result<Option<PdfMutation>, Fault> {
    if item != 0 { return Err(refusal("PageDoc owns exactly one text item")); }
    let index = usize::try_from(page).map_err(|_| refusal("Page index exceeds this platform"))?;
    let original = document.pages.get(index).ok_or_else(|| refusal("Page target does not exist"))?;
    if DocumentWindowKit::text_revision(&original.text) != revision { return Err(refusal("Page text revision no longer matches")); }
    if original.text == text { return Ok(None); }
    Ok(Some(PdfMutation::ReplacePageText(ReplacePageText { index, text: text.into() })))
}

pub fn index(args: Option<&DslValue>, key: &str) -> Result<usize, Fault> {
    usize::try_from(semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, key)?).map_err(|_| refusal("Page index exceeds this platform"))
}
