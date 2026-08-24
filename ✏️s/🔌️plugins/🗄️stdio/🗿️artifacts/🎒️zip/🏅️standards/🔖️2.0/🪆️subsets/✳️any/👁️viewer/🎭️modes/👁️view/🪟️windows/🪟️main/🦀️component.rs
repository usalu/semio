//! 🎒️ Zip viewer (2.0/✳️any) — the `main` window: the archive as a real, READ-ONLY tree, built from
//! the framework `TreeWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same `ZipSnapshot` read, no edit affordances (`window_kind()`, the
//! read-only variant, not the editable one).

use crate::artifacts::zip::ZipSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
pub const COMMENT_NODE_ID: &str = "comment";
pub const ENTRY_NODE_PREFIX: &str = "entry:";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::zip::any::create_zip_any_viewer`.
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
mod tests {
    use super::*;
    use crate::artifacts::zip::schema::snapshot::ZipEntry;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_read_only_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_the_comment_root_and_one_leaf_per_entry() {
        let document = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec() }], comment: "an archive".into(), ..ZipSnapshot::default() };
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        assert_eq!(root.id, COMMENT_NODE_ID);
        let children = root.items.as_ref().expect("root has children");
        assert_eq!(children.len(), 1);
    }
}
//#endregion 🧪️Tests
