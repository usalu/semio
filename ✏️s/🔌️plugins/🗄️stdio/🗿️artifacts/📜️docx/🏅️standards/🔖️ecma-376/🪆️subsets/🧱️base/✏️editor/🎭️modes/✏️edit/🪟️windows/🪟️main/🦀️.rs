//! 📄️ Docx editor — `main` window: a real, directly editable page view of `DocxDocument.body`,
//! built from the framework `DocumentWindowKit` (contract §2.6). Every paragraph run is an
//! independently revision-guarded text target, preserving formatting and sibling run text.

use crate::schema::mutations::{docx_top_level_block_count, docx_top_level_run_at, docx_top_level_run_count, DocxXmlAddress};
use crate::DocxSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiListBuilder;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_docx_editor` (this subset's surface root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        label: LocalizedLabel::native("Document", "Dokument"),
        icon_id: "file-text".into(),
        ..DocumentWindowKit::editable_window_kind_with_args(vec![ActionArgDef::object(
            "address",
            LocalizedLabel::native("Canonical XML address", "Kanonische XML-Adresse"),
            vec![
                ActionArgDef::text("partPath", LocalizedLabel::native("Part path", "Teilpfad")).required(),
                ActionArgDef::any("nodePath", LocalizedLabel::native("Node path", "Knotenpfad")).required(),
                ActionArgDef::text("expectedName", LocalizedLabel::native("Expected element", "Erwartetes Element")).required(),
                ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")),
            ],
        )
        .required()])
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn address_arguments(address: &DocxXmlAddress) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
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
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("stdio.docx.arguments", "argument map capacity"))?;
    arguments.try_insert("address".into(), UiValue::Map(value.finish())).map_err(|_| PluginAssemblyError::new("stdio.docx.arguments", "address argument capacity"))?;
    Ok(UiValue::Map(arguments.finish()))
}

/// ✏️ Builds one faithfully editable draft for every existing paragraph run.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn editable_pages(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<Vec<EditableDocumentPage>> {
    let mut pages = Vec::new();
    let block_count = docx_top_level_block_count(document).map_err(|error| PluginAssemblyError::new("stdio.docx.block-count", error))?;
    for page_index in 0..block_count {
        let run_count = docx_top_level_run_count(document, page_index).map_err(|error| PluginAssemblyError::new("stdio.docx.run-count", error))?;
        for item_index in 0..run_count {
            let run = docx_top_level_run_at(document, page_index, item_index).map_err(|error| PluginAssemblyError::new("stdio.docx.run-address", error))?;
            pages.push(EditableDocumentPage::with_arguments(page_index as u32, item_index as u32, run.text, address_arguments(&run.address)?));
        }
    }
    Ok(pages)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document)? }, &TreeWindows::unhosted(), Locale::En)
}

pub fn render_windowed(document: &DocxSnapshot, windows: &TreeWindows<'_>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document)? }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
