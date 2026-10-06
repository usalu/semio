//! \u{1f4c4}️ PDF 1.4 page rendering and commands over its resolved page domain.
use crate::standards::v1_4::subsets::base::schema::{snapshot::{PageDoc, PdfSnapshot}, mutations::{PdfMutation, InsertPage, RemovePage, MovePage, ResizePage, ReplacePageText}};
use protocol::{Mutation, OpText};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, TreeWindows, ActionArgDef, ActionDefinition, ActionKind, WindowKindDefinition, PluginAssemblyError, UiAssemblyResult, Buildable, HasBase, UiText};
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_ui_contract::{BuiltNode, UiPublicationRevision};
use semio_framework_ui_locale::{Locale, LocalizedLabel};
use semio_framework_value::DslValue;

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
const PAGE_ACTIONS: [&str; 4] = ["insert-page", "remove-page", "move-page", "set-page-size"];

fn refusal(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.1.4.page-edit"), message)
}

fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "PDF 1.4 page rendering admission failed")
}

/// \u{1f3f7}️ Resolves the own page command identifier.
pub fn static_action(action: &str) -> &'static str {
    PAGE_ACTIONS.into_iter().find(|candidate| *candidate == action).unwrap_or("page-edit")
}

/// \u{1f9ed}️ Recognizes actions over the own resolved page domain.
pub fn is_page_action(action: &str) -> bool { PAGE_ACTIONS.contains(&action) }

/// \u{1fa9f}️ Declares addressed text drafts and resolved page structure edits.
pub fn window_definition() -> WindowKindDefinition {
    let mut definition = DocumentWindowKit::editable_window_kind();
    definition.label = LocalizedLabel::native("Pages", "Seiten");
    let page = || ActionArgDef::index("page", LocalizedLabel::native("Page", "Seite")).required();
    let width = || ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required();
    let height = || ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required();
    let action = |id, en, de, args| {
        let mut definition = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
        definition.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
        definition
    };
    definition.actions.extend([
        action("insert-page", "Insert Page", "Seite einfügen", vec![page(), width(), height(), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()]),
        action("remove-page", "Remove Page", "Seite entfernen", vec![page()]),
        action("move-page", "Move Page", "Seite verschieben", vec![ActionArgDef::index("from", LocalizedLabel::native("From", "Von")).required(), ActionArgDef::index("to", LocalizedLabel::native("To", "Nach")).required()]),
        action("set-page-size", "Set Page Size", "Seitengröße setzen", vec![page(), width(), height()]),
        action("replace-page-text", "Replace Page Text", "Seitentext ersetzen", vec![page(), ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).required(), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()]),
    ]);
    definition
}

/// \u{1f4c4}️ Projects each complete own text target with the actual publication revision.
pub fn editable_view(document: &PdfSnapshot, publication_revision: UiPublicationRevision) -> Result<EditableDocumentView, Fault> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| {
        let index = u32::try_from(index).map_err(|_| refusal("Page index exceeds the document action address"))?;
        Ok(EditableDocumentPage::new(index, 0, page.text.clone()))
    }).collect::<Result<Vec<_>, Fault>>()?;
    Ok(EditableDocumentView { pages, publication_revision })
}

/// \u{1f4d0}️ Renders dimensions beside each materialized draft without changing its text revision.
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

/// \u{1f510}️ Reads the current text token for an explicitly addressed agent action.
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

fn index(args: Option<&DslValue>, key: &str) -> Result<usize, Fault> {
    usize::try_from(semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, key)?).map_err(|_| refusal("Page index exceeds this platform"))
}

fn number(args: Option<&DslValue>, key: &str) -> Result<f64, Fault> {
    let Some(DslValue::Object(entries)) = args else { return Err(refusal("Page action arguments must be an object")); };
    let mut values = entries.iter().filter(|(name, _)| name == key).map(|(_, value)| value);
    let Some(DslValue::Number(number)) = values.next() else { return Err(refusal(format!("Page action requires numeric '{key}'"))); };
    let value = number.as_f64();
    if values.next().is_some() || !value.is_finite() { return Err(refusal(format!("Page action requires one finite '{key}'"))); }
    Ok(value)
}

/// \u{1f9fe}️ Encodes the actual own mutation as the replayable page action payload.
pub fn edit_from_action(action: &str, args: Option<&DslValue>) -> Result<String, Fault> {
    let mutation = match action {
        "insert-page" => PdfMutation::InsertPage(InsertPage { index: index(args, "page")?, page: PageDoc { width: number(args, "width")?, height: number(args, "height")?, text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? } }),
        "remove-page" => PdfMutation::RemovePage(RemovePage { index: index(args, "page")? }),
        "move-page" => PdfMutation::MovePage(MovePage { from: index(args, "from")?, to: index(args, "to")? }),
        "set-page-size" => PdfMutation::ResizePage(ResizePage { index: index(args, "page")?, width: number(args, "width")?, height: number(args, "height")? }),
        _ => return Err(refusal("Unknown PDF 1.4 page action")),
    };
    Ok(mutation.print_op())
}

/// ▶️ Admits the replayed own mutation against its original page domain.
pub fn emit_page_edit(snapshot: &PdfSnapshot, action: &str, payload: &str) -> Result<semio_framework_plugin::Emit<PdfMutation>, Fault> {
    let mutation = PdfMutation::parse_op(payload).map_err(|error| refusal(error.to_string()))?;
    if !matches!((action, &mutation),
        ("insert-page", PdfMutation::InsertPage(_)) | ("remove-page", PdfMutation::RemovePage(_)) |
        ("move-page", PdfMutation::MovePage(_)) | ("set-page-size", PdfMutation::ResizePage(_))) { return Err(refusal("Page action payload has a different mutation kind")); }
    let outcome = mutation.diff(snapshot);
    if !outcome.messages().is_empty() { return Err(refusal("Page action is outside this document's domain")); }
    Ok(semio_framework_plugin::Emit { artifact_mutations: vec![mutation], ..Default::default() })
}

/// \u{1fa9f}️ Places own page drafts beside complete snapshot details.
pub fn document_layout(main: &str) -> semio_framework_plugin::WindowLayout {
    use semio_framework_plugin::{WindowLayout, WindowLayoutRoot, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutStackNode, WindowLayoutWindowNode};
    let stack = |size, id: &str| WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size: Some(size), active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: id.into(), title: None, instance_id: None, template_id: None, corner: None }] });
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![stack(0.7, main), stack(0.3, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_WINDOW_KIND_ID)] }) }
}

#[cfg(test)]
#[path = "\u{1f9ea}️tests/\u{1f52c}️unit/\u{1f980}️.rs"]
mod tests;
