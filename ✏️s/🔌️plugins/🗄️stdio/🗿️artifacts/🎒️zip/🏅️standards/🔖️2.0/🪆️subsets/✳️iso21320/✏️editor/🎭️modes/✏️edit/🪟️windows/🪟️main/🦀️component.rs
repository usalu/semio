//! 🎒️ Zip editor (2.0/✳️iso21320) — the `main` window: the archive as a directly editable tree,
//! built from the framework `TreeWindowKit` (contract §2.6). Same `ZipSnapshot` shape as the
//! sibling ✳️any subset (ISO/IEC 21320-1 is a validation-gated dialect stamp on the SAME schema, not
//! a new one — see this subset's own `🧬️schema/🦀️component.rs` doc comment). Root node addresses
//! the archive-level `comment`; one leaf per `ZipEntry`, labeled with its name and decompressed byte
//! size. Scope note: `set-node` can rename the comment or an entry's NAME, never an entry's byte
//! payload — real per-byte content editing isn't representable through a label-editing tree
//! control, so it stays out of this first pass (documented honestly, matching energy's own
//! `SetStructureField` scope note).

use crate::artifacts::zip::ZipSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;

/// 🌳️ The root node's fixed id — the one `set-node` target that renames the archive's own
/// `comment`, shared by `render` and the surface root's `ZipEditorCommand::SetNode` dispatch.
pub const COMMENT_NODE_ID: &str = "comment";
/// 🌳️ Prefix for an entry leaf's node id — `"{ENTRY_NODE_PREFIX}{index}"` indexes `ZipSnapshot.entries`.
pub const ENTRY_NODE_PREFIX: &str = "entry:";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::zip::iso21320::create_zip_iso21320_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Archive", "Archiv"), icon_id: "archive".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `ZipSnapshot -> UiNode`: root = the archive comment (a real `set-node` edit target), one
/// leaf per entry labeled `"{name} ({n} bytes)"` (the leaf's NAME is a real `set-node` edit target
/// via `ENTRY_NODE_PREFIX`; the byte count is a read-only label, not addressable).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &ZipSnapshot) -> UiNode {
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
    async fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_the_comment_root_and_one_leaf_per_entry() {
        let document = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec() }], comment: "an archive".into(), ..ZipSnapshot::default() };
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        assert_eq!(root.id, COMMENT_NODE_ID);
        let children = root.items.as_ref().expect("root has children");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, format!("{ENTRY_NODE_PREFIX}0"));
    }
}
//#endregion 🧪️Tests
