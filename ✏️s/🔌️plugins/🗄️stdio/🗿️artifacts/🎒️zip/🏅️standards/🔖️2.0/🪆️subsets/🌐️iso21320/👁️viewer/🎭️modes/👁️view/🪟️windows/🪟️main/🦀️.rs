//! 🎒️ Zip viewer (2.0/🌐️iso21320) — the `main` window: the archive as a real, READ-ONLY tree, built
//! from the framework `TreeWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same `ZipSnapshot` read, no edit affordances (`window_kind()`, the
//! read-only variant, not the editable one).

use crate::ZipSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
pub const COMMENT_NODE_ID: &str = "comment";
pub const ENTRY_NODE_PREFIX: &str = "entry:";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::zip::iso21320::create_zip_iso21320_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Archive", "Archiv"), icon_id: "archive".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `ZipSnapshot -> BuiltNode` read: root = the archive comment, one leaf per entry labeled
/// `"{name} ({n} bytes)"`, no edit affordances.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &ZipSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let children = document.entries.iter().enumerate().map(|(index, entry)| TreeNodeView { id: format!("{ENTRY_NODE_PREFIX}{index}"), label: format!("{} ({} bytes)", entry.name, entry.data.len()), children: Vec::new() }).collect();
    let root = TreeNodeView { id: COMMENT_NODE_ID.into(), label: format!("Comment: {}", document.comment), children };
    TreeWindowKit::render(&TreeView { roots: vec![root] })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
