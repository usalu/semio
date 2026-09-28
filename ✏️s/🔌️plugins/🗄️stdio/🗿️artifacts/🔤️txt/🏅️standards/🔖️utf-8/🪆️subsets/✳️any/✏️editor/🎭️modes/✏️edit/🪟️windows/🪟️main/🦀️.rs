//! 📄️ Txt editor — `main` window: a real, directly editable whole-document text buffer, built
//! from the framework `TextWindowKit` (contract §2.6). `TxtSnapshot.lines` is joined with the
//! document's own `line_ending` on render, and re-split the same way on `replace-text`.

use crate::TxtSnapshot;
use semio_framework_plugin::app::{TextDraftView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, Locale, LocalizedLabel, UiMapBuilder, UiText, UiValue, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::txt::create_txt_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Text", "Text"), icon_id: "type".into(), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `TxtSnapshot -> BuiltNode`: `lines` joined by the document's own `line_ending`, with a
/// trailing terminator when `trailing_newline` is set — the exact same join the artifact's own
/// codec uses to re-serialize, so what's shown here IS what re-encoding would emit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TxtSnapshot, locale: Locale, revision: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (apply, discard, conflict, applying, cancel, failed) = match locale {
        Locale::En => ("Apply", "Discard", "The document changed. Copy your draft or discard it before applying.", "Preparing draft", "Cancel", "The draft could not be applied"),
        Locale::De => ("Anwenden", "Verwerfen", "Das Dokument wurde geändert. Kopieren Sie Ihren Entwurf oder verwerfen Sie ihn vor dem Anwenden.", "Entwurf wird vorbereitet", "Abbrechen", "Entwurf konnte nicht angewendet werden"),
    };
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("txt-window.revision-arguments", "argument map capacity"))?;
    arguments
        .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("txt-window.revision", "revision exceeds UI text capacity"))?))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("txt-window.revision-arguments", "argument map capacity"))?;
    TextWindowKit::render_draft(&TextDraftView {
        surface_id: WINDOW_KIND_ID.into(),
        text: document.to_body(),
        language: Some("text".into()),
        action_id: "textEdit".into(),
        argument: "text".into(),
        arguments: Some(UiValue::Map(arguments.finish())),
        apply_label: apply.into(),
        discard_label: discard.into(),
        conflict_label: conflict.into(),
        applying_label: applying.into(),
        cancel_label: cancel.into(),
        failed_label: failed.into(),
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
