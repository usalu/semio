//! 📄️ Note play app panel — the document tree: every block, with quick-add rows.

use crate::artifacts::note::schema::{block_icon, block_kind, block_name, block_tree_row_id, block_visible};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::{NOTE_INTERACTION_BLOCKS, NOTE_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, ActionFactory, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

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
fn block_tree_item(block: &NoteBlockNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let nested = match block {
        NoteBlockNode::Group { children, .. } => fixed_nodes(children.iter().map(block_tree_item))?,
        _ => UiFixedList::default(),
    };
    let mut node = tree_item_desc(block_tree_row_id(block), Label::data(block_name(block)), Some(block_kind(block).into()))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(block_icon(block_kind(block))).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block icon admission failed"))?);
        props.default_open = Some(matches!(block, NoteBlockNode::Group { .. }));
        props.draggable = Some(true);
        props.dimmed = Some(!block_visible(block));
    }
    node.base.children = nested;
    Ok(node)
}

fn fixed_nodes(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut nodes = UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note node admission failed"))?;
    }
    Ok(nodes)
}

fn add_block_args(kind: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note action map admission failed"))?;
    let kind = UiText::try_from_str(kind).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note block kind admission failed"))?;
    args.push("kind".into(), kind).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

pub async fn render(document: &NoteSnapshot, labels: &NotePlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut items = UiFixedList::default();
    for (kind, label, icon) in [("text", labels.add_text, "type"), ("table", labels.add_table, "table-2"), ("math", labels.add_math, "note-math"), ("image", labels.add_image, "image"), ("group", labels.add_group, "folder-plus")] {
        let action = ActionFactory::new(NOTE_PLAY_CONTROLLER_ID).action("addBlock", Some(add_block_args(kind)?))?;
        let mut item = tree_item_with_action(format!("note-play-blocks.add.{kind}"), label, None, action)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note add-block icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note action row admission failed"))?;
    }
    if document.blocks.is_empty() {
        let mut item = tree_item("note-play-blocks.empty", labels.document_empty)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(UiText::try_from_str("sticky-note").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "note empty icon admission failed"))?);
        }
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note empty row admission failed"))?;
    } else {
        for block in &document.blocks {
            items.try_push(block_tree_item(block)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note block list admission failed"))?;
        }
    }
    PanelTreeBuilder::new("note-play-blocks")?.section("note-play-blocks", Some(labels.document.into()), true, items)?.interaction_domain(NOTE_INTERACTION_BLOCKS)?.build()
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
