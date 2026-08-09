//! 🔍️ Note play app panel — the properties inspector for the current selection.

use crate::apps::note::terminology::NotePlayLabels;
use crate::artifacts::note::engine::{block_bounds, block_id, block_name, block_visible, find_block, flatten_blocks};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_stack_vertical, ui_text, ActionDescriptor, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiToggleNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_PROPERTIES: &str = "note.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(NOTE_PLAY_BODY_PROPERTIES.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_patch(block_ids: &[String], field: &str) -> ActionDescriptor {
    crate::apps::note::note_action("patchBlocks", Some(serde_json::json!({ "blockIds": block_ids, "field": field })))
}

fn inspector_text_field(block_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_number_field(block_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: inspector_patch(block_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

pub fn render(document: &NoteSnapshot, selected_ids: &[String], active_utility_id: &str, labels: &NotePlayLabels) -> UiNode {
    let blocks: Vec<&NoteBlockNode> = selected_ids.iter().filter_map(|id| find_block(&document.blocks, id)).collect();
    if blocks.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("Schema: {}", document.schema))),
            ui_text(Label::data(format!("Blocks: {}", flatten_blocks(&document.blocks).len()))),
            ui_text(Label::data(format!("Utility: {active_utility_id}"))),
            ui_text(Label::data(format!("Snap: {}", if document.snap_enabled.unwrap_or(false) { format!("{}px", document.snap_grid_spacing.unwrap_or(8.0)) } else { "off".into() }))),
        ]);
    }
    let block_ids: Vec<String> = blocks.iter().map(|block| block_id(block).into()).collect();
    let names: Vec<String> = blocks.iter().map(|block| block_name(block).into()).collect();
    let xs: Vec<f64> = blocks.iter().map(|block| block_bounds(block).0).collect();
    let ys: Vec<f64> = blocks.iter().map(|block| block_bounds(block).1).collect();
    let widths: Vec<f64> = blocks.iter().map(|block| block_bounds(block).2).collect();
    let heights: Vec<f64> = blocks.iter().map(|block| block_bounds(block).3).collect();
    let visibles: Vec<bool> = blocks.iter().map(|block| block_visible(block)).collect();
    let locked: Vec<bool> = blocks
        .iter()
        .map(|block| match block {
            NoteBlockNode::Text { locked, .. } | NoteBlockNode::Image { locked, .. } | NoteBlockNode::Table { locked, .. } | NoteBlockNode::Math { locked, .. } | NoteBlockNode::Ink { locked, .. } | NoteBlockNode::Group { locked, .. } => *locked,
        })
        .collect();
    let visible_mixed = ui_inspector_mixed_toggle(&visibles);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "note-properties.block".into(),
        label: labels.inspector_block.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&block_ids, "note-properties.name", labels.field_name, &names, "name"),
            inspector_number_field(&block_ids, "note-properties.x", labels.field_x, &xs, "x"),
            inspector_number_field(&block_ids, "note-properties.y", labels.field_y, &ys, "y"),
            inspector_number_field(&block_ids, "note-properties.width", labels.field_width, &widths, "width"),
            inspector_number_field(&block_ids, "note-properties.height", labels.field_height, &heights, "height"),
            UiNode::Field(UiFieldNode {
                id: "note-properties.visible".into(),
                label: labels.field_visible.into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.visible.toggle".into(),
                    icon_id: "eye".into(),
                    text: None,
                    on_change: inspector_patch(&block_ids, "visible"),
                    presence: UiPresence::selected(visible_mixed.uniform && visible_mixed.pressed),
                    menu: None,
                })),
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "note-properties.locked".into(),
                label: labels.field_locked.into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "note-properties.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: inspector_patch(&block_ids, "locked"),
                    presence: UiPresence::selected(locked_mixed.uniform && locked_mixed.pressed),
                    menu: None,
                })),
                presence: UiPresence::default(),
                menu: None,
            }),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::note::testkit::{note_app, render as render_body};
    use crate::apps::note::NOTE_PLAY_BODY_PROPERTIES as BODY_PROPERTIES;

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = note_app();
        assert!(render_body(&mut app, "note.play.nope").contains("Unknown body"));
    }

    #[test]
    fn empty_selection_renders_summary_fallback() {
        let mut app = note_app();
        let json = render_body(&mut app, BODY_PROPERTIES);
        assert!(json.contains("Utility:"));
    }
}
//#endregion 🧪️Tests
