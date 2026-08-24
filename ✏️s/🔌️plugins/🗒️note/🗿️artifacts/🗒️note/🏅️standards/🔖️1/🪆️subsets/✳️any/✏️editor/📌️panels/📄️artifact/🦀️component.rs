//! 📄️ Note play app panel — the document tree: every block, with quick-add rows.

use crate::artifacts::note::schema::{block_icon, block_kind, block_name, block_tree_row_id, block_visible};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::NOTE_INTERACTION_BLOCKS;
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_DOCUMENT: &str = "note.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(NOTE_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `note-play-block:{id}` targets `NotePlayApp::interaction_topology` declares for the "blocks"
/// domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
async fn block_tree_item(block: &NoteBlockNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let nested = match block {
        NoteBlockNode::Group { children, .. } if !children.is_empty() => Some(children.iter().map(block_tree_item).collect()?),
        _ => None,
    };
    UiTreeItemNode {
        icon_id: Some(block_icon(block_kind(block)).into()),
        default_open: Some(matches!(block, NoteBlockNode::Group { .. })),
        draggable: Some(true),
        items: nested,
        dimmed: if block_visible(block) { None } else { Some(true) },
        ..tree_item_desc(block_tree_row_id(block), Label::data(block_name(block)), Some(block_kind(block).into()))?
    }
}

pub async fn render(document: &NoteSnapshot, labels: &NotePlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action_rows: Vec<UiTreeItemNode> = [("text", labels.add_text, "type"), ("table", labels.add_table, "table-2"), ("math", labels.add_math, "note-math"), ("image", labels.add_image, "image"), ("group", labels.add_group, "folder-plus")]
        .into_iter()
        .map(|(kind, label, icon)| UiTreeItemNode { icon_id: Some(icon.into()), menu: None, ..tree_item_with_action(format!("note-play-blocks.add.{kind}"), label, None, crate::editor::note::note_action("addBlock", Some(json!({ "kind": kind }))))? })
        .collect();
    let block_items: Vec<UiTreeItemNode> =
        if document.blocks.is_empty() { vec![UiTreeItemNode { icon_id: Some("sticky-note".into()), ..tree_item("note-play-blocks.empty", labels.document_empty)? }] } else { document.blocks.iter().map(block_tree_item).collect()? };
    PanelTreeBuilder::new("note-play-blocks")?.section("note-play-blocks", Some(labels.document.into()), true, [action_rows, block_items].concat())?.interaction_domain(NOTE_INTERACTION_BLOCKS)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::note::testkit::{note_app, render as render_body};
    use crate::editor::note::NOTE_PLAY_BODY_DOCUMENT as BODY_DOCUMENT;
    use semio_framework_plugin::PluginApp;

    /// 🩹️ Pre-existing bug fixed here (confirmed via `git log --date=iso`: `SetActiveExample`'s
    /// `reset_document_effect`/`Effect::LoadDocument` conversion — and this very test — both
    /// predate this ticket's dispatch to note, unrelated to composition). Dispatching a command only
    /// ever RETURNS a `Effect::LoadDocument` as data for a real host to re-apply; `dispatch_typed`
    /// never loops it back into the same app instance, so `app.snapshot()`/subsequent `render()` never
    /// reflected it — this assertion could never have passed as originally written, on ANY content.
    /// Fixed the same way writer's own `app_with_jack()` and cad's `two_instances_converge_…` tests
    /// already do: call `PluginApp::load_document_pack` directly, the same technique a real host uses
    /// when it receives the effect.
    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree() {
        let mut app = note_app();
        let document = crate::artifacts::note::schema::semio_example_snapshot();
        let envelope = store::create_document_envelope::<crate::artifacts::note::NoteSnapshot, crate::artifacts::note::NoteMutation>(&document.schema.clone(), &document.id.clone(), document, None);
        let files = store::print_document_pack(&envelope).expect("print semio example document pack");
        app.load_document_pack(&files).expect("load semio example");
        let json = render_body(&mut app, BODY_DOCUMENT);
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Welcome"));
    }

    #[semio_framework_async_macros::async_test]
    async fn note_labels_resolve_native_by_default() {
        let mut app = note_app();
        let document_json = render_body(&mut app, BODY_DOCUMENT);
        assert!(document_json.contains("Add Text"));
        assert!(document_json.contains("Add Table"));
        assert!(document_json.contains("Add Math"));
        assert!(document_json.contains("Add Image"));
        assert!(document_json.contains("Add Group"));
        assert!(document_json.contains("Drop blocks here"));
    }
}
//#endregion 🧪️Tests
