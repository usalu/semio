//! 📊️ Xlsx viewer (ecma-376/🔒️strict) — sparse, windowed, read-only worksheet grids.

use crate::XlsxSnapshot;
use semio_framework_plugin::app::{TableWindowKit, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_xlsx_strict_viewer` (subset root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Workbook", "Arbeitsmappe"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure sparse-grid projection with no command-driven cell edits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::viewer::xlsx::standards::v_ecma_376::subsets::base::modes::view::windows::main::render(document, locale, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
