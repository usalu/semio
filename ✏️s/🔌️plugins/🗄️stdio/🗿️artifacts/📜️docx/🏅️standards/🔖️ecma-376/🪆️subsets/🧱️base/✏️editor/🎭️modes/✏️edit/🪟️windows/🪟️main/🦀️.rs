//! 📄️ Canonical DOCX text drafts with revision-bound run-formatting controls.

use crate::schema::mutations::{docx_run_formatting, docx_top_level_block_count, docx_top_level_text_targets, DocxRunFormatting, DocxTextTargetKind, DocxXmlAddress};
use crate::DocxSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionId, ActionKind, BuiltNode, PluginAssemblyError, TreeWindows, UiListBuilder, UiMapBuilder, UiText, UiValue, WindowKindDefinition};
use semio_framework_ui_locale::{Locale, LocalizedLabel};

pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
pub const BASE_CONTROLLER_ID: &str = "s.stdio.docx@ecma-376/*#editor";
pub const SET_RUN_FORMATTING_ACTION: &str = "set-run-formatting";

fn address_argument_definition() -> ActionArgDef {
    ActionArgDef::object(
        "address",
        LocalizedLabel::native("Canonical XML address", "Kanonische XML-Adresse"),
        vec![
            ActionArgDef::text("partPath", LocalizedLabel::native("Part path", "Teilpfad")).required(),
            ActionArgDef::any("nodePath", LocalizedLabel::native("Node path", "Knotenpfad")).required(),
            ActionArgDef::text("expectedName", LocalizedLabel::native("Expected element", "Erwartetes Element")).required(),
            ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")),
        ],
    )
    .required()
}

/// 🧱️ Declares canonical text and direct run-formatting actions from their schema fields.
pub fn definition() -> WindowKindDefinition {
    let mut definition = DocumentWindowKit::editable_window_kind_with_args(vec![address_argument_definition()]);
    definition.label = LocalizedLabel::native("Document", "Dokument");
    definition.icon_id = "file-text".into();
    let mut formatting = ActionDefinition::bounded_catalog(SET_RUN_FORMATTING_ACTION, LocalizedLabel::native("Set run formatting", "Textlaufformatierung setzen"), ActionKind::Mutation)
        .with_args(vec![
            address_argument_definition(),
            ActionArgDef::toggle("bold", LocalizedLabel::native("Bold", "Fett")).required(),
            ActionArgDef::toggle("italic", LocalizedLabel::native("Italic", "Kursiv")).required(),
            ActionArgDef::toggle("underline", LocalizedLabel::native("Underline", "Unterstrichen")).required(),
        ])
        .describe(LocalizedLabel::native("Sets direct bold, italic, and underline formatting when the run revision still matches.", "Setzt direkte Fett-, Kursiv- und Unterstreichungsformatierung, wenn die Textlaufrevision noch übereinstimmt."))
        .in_palette(false);
    formatting.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
    definition.actions.push(formatting);
    definition
}

fn address_value(address: &DocxXmlAddress) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut path = UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("stdio.docx.address.path", "node path capacity"))?;
    for index in &address.node_path {
        path.push(UiValue::Number(*index as f64)).map_err(|_| PluginAssemblyError::new("stdio.docx.address.path", "node path capacity"))?;
    }
    let text = |value: &str, code: &'static str| UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new(code, "address text exceeds the UI text bound"));
    let mut value = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("stdio.docx.address", "address map capacity"))?;
    value.try_insert("partPath".into(), text(&address.part_path, "stdio.docx.address.part-path")?).map_err(|_| PluginAssemblyError::new("stdio.docx.address", "part path capacity"))?;
    value.try_insert("nodePath".into(), UiValue::List(path.finish())).map_err(|_| PluginAssemblyError::new("stdio.docx.address", "node path capacity"))?;
    value.try_insert("expectedName".into(), text(&address.expected_name, "stdio.docx.address.expected-name")?).map_err(|_| PluginAssemblyError::new("stdio.docx.address", "expected name capacity"))?;
    value.try_insert("revision".into(), text(&address.revision, "stdio.docx.address.revision")?).map_err(|_| PluginAssemblyError::new("stdio.docx.address", "revision capacity"))?;
    Ok(UiValue::Map(value.finish()))
}

fn address_arguments(address: &DocxXmlAddress) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("stdio.docx.arguments", "argument map capacity"))?;
    arguments.try_insert("address".into(), address_value(address)?).map_err(|_| PluginAssemblyError::new("stdio.docx.arguments", "address argument capacity"))?;
    Ok(UiValue::Map(arguments.finish()))
}

fn formatting_arguments(address: &DocxXmlAddress, bold: bool, italic: bool, underline: bool) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("stdio.docx.formatting.arguments", "formatting argument capacity"))?;
    arguments.try_insert("address".into(), address_value(address)?).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.arguments", "address argument capacity"))?;
    arguments.try_insert("bold".into(), UiValue::Bool(bold)).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.arguments", "bold argument capacity"))?;
    arguments.try_insert("italic".into(), UiValue::Bool(italic)).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.arguments", "italic argument capacity"))?;
    arguments.try_insert("underline".into(), UiValue::Bool(underline)).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.arguments", "underline argument capacity"))?;
    Ok(UiValue::Map(arguments.finish()))
}

struct RenderPage {
    page: EditableDocumentPage,
    address: DocxXmlAddress,
    formatting: Option<DocxRunFormatting>,
}

fn render_pages(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<Vec<RenderPage>> {
    let mut pages = Vec::new();
    let block_count = docx_top_level_block_count(document).map_err(|error| PluginAssemblyError::new("stdio.docx.block-count", error.into_message()))?;
    for page_index in 0..block_count {
        let targets = docx_top_level_text_targets(document, page_index).map_err(|error| PluginAssemblyError::new("stdio.docx.text-targets", error.into_message()))?;
        for (item_index, target) in targets.into_iter().enumerate() {
            let formatting = match target.kind {
                DocxTextTargetKind::Run => Some(docx_run_formatting(document, &target.address).map_err(|error| PluginAssemblyError::new("stdio.docx.run-formatting", error.into_message()))?),
                DocxTextTargetKind::EmptyParagraph => None,
            };
            let arguments = address_arguments(&target.address)?;
            pages.push(RenderPage { page: EditableDocumentPage::with_arguments(page_index as u32, item_index as u32, target.text, arguments), address: target.address, formatting });
        }
    }
    Ok(pages)
}

pub(crate) fn editable_pages(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<Vec<EditableDocumentPage>> {
    Ok(render_pages(document)?.into_iter().map(|page| page.page).collect())
}

fn localized_control_label(locale: Locale, en: &'static str, de: &'static str) -> &'static str {
    match locale {
        Locale::En => en,
        Locale::De => de,
    }
}

fn direct_formatting_label(locale: Locale, en: &'static str, de: &'static str, state: Option<bool>) -> String {
    let property = localized_control_label(locale, en, de);
    let state = match (locale, state) {
        (Locale::En, None) => "inherited",
        (Locale::En, Some(true)) => "direct on",
        (Locale::En, Some(false)) => "direct off",
        (Locale::De, None) => "geerbt",
        (Locale::De, Some(true)) => "direkt an",
        (Locale::De, Some(false)) => "direkt aus",
    };
    format!("{property} ({state})")
}

fn formatting_toolbar(address: &DocxXmlAddress, formatting: Option<DocxRunFormatting>, controller_id: &str, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<Option<BuiltNode>> {
    let Some(formatting) = formatting else { return Ok(None) };
    let action = ActionId::try_v1(controller_id, SET_RUN_FORMATTING_ACTION).ok_or_else(|| PluginAssemblyError::new("stdio.docx.formatting.action", "formatting action id exceeds the UI bound"))?;
    let bold = formatting.bold.unwrap_or(false);
    let italic = formatting.italic.unwrap_or(false);
    let underline = formatting.underline.unwrap_or(false);
    let controls = [
        ("bold", direct_formatting_label(locale, "Bold", "Fett", formatting.bold), bold, (!bold, italic, underline)),
        ("italic", direct_formatting_label(locale, "Italic", "Kursiv", formatting.italic), italic, (bold, !italic, underline)),
        ("underline", direct_formatting_label(locale, "Underline", "Unterstrichen", formatting.underline), underline, (bold, italic, !underline)),
    ];
    let mut toolbar = ui::row().try_id("run-formatting").map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.toolbar", "formatting toolbar id capacity"))?;
    for (id, label, current, next) in controls {
        let control = ui::toggle(current)
            .try_id(id)
            .map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.control", "formatting control id capacity"))?
            .text(ui::Label(UiText::try_from_str(&label).ok_or_else(|| PluginAssemblyError::new("stdio.docx.formatting.control", "formatting control label capacity"))?))
            .try_on_with(Trigger::Change, action.clone(), formatting_arguments(address, next.0, next.1, next.2)?)
            .map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.control", "formatting control binding capacity"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.control", "formatting control build capacity"))?;
        toolbar = toolbar.try_child(control).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.toolbar", "formatting toolbar child capacity"))?;
    }
    toolbar.try_build().map(Some).map_err(|_| PluginAssemblyError::new("stdio.docx.formatting.toolbar", "formatting toolbar build capacity"))
}

pub(crate) fn render_windowed_for_controller(
    document: &DocxSnapshot,
    windows: &TreeWindows<'_>,
    locale: Locale,
    controller_id: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let pages = render_pages(document)?;
    let mut view_pages = Vec::with_capacity(pages.len());
    let mut accessories = Vec::with_capacity(pages.len());
    for page in pages {
        view_pages.push(page.page);
        accessories.push((page.address, page.formatting));
    }
    DocumentWindowKit::render_editable_windowed_with_accessory(&EditableDocumentView { pages: view_pages, publication_revision }, windows, locale, |ordinal, _| {
        let (address, formatting) = accessories.get(ordinal).ok_or_else(|| PluginAssemblyError::new("stdio.docx.formatting.accessory", "document accessory ordinal is outside the rendered projection"))?;
        formatting_toolbar(address, *formatting, controller_id, locale)
    })
}

pub fn render(document: &DocxSnapshot, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_windowed_for_controller(document, &TreeWindows::unhosted(), Locale::En, BASE_CONTROLLER_ID, publication_revision)
}

pub fn render_windowed(document: &DocxSnapshot, windows: &TreeWindows<'_>, locale: Locale, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_windowed_for_controller(document, windows, locale, BASE_CONTROLLER_ID, publication_revision)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
