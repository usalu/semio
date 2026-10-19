//! 🎨️ Paint node step with driver: the per-variant node painting of `paint_node_step_with_driver`. See [`super`].

use super::*;

#[path = "📝️text-and-structure-nodes/🦀️.rs"]
mod text_and_structure_nodes;
use text_and_structure_nodes::{paint_text_node_step, paint_stack_node_step, paint_separator_node_step, paint_key_value_node_step, paint_field_node_step, paint_section_node_step, paint_group_node_step};

#[path = "🎛️control-nodes/🦀️.rs"]
mod control_nodes;
use control_nodes::{paint_button_node_step, paint_input_node_step, paint_select_node_step, paint_toggle_node_step, paint_icon_select_node_step};

#[path = "🎚️slider-stepper-and-progress-nodes/🦀️.rs"]
mod slider_stepper_and_progress_nodes;
use slider_stepper_and_progress_nodes::{paint_slider_node_step, paint_number_stepper_node_step, paint_ring_node_step, paint_progress_node_step};

#[path = "🖼️media-and-slot-nodes/🦀️.rs"]
mod media_and_slot_nodes;
use media_and_slot_nodes::{paint_image_node_step, paint_component_scene_node_step, paint_external_slot_node_step};

// #region paint
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the canonical driver drag axis")]
pub fn paint_node_step_with_driver(
    tree: &UiTree,
    id: NodeId,
    origin_x: f32,
    origin_y: f32,
    theme: &Theme,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    has_scene_host: bool,
    driver_drag: UiDriverDrag,
    block_reversed: bool,
    inline: ui_contract::FlowInline,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    if cursor.node.is_none() {
        cursor.begin(id, origin_x, origin_y);
        return RetainedNodePaintStep::Pending;
    }
    if cursor.node != Some(id) || cursor.origin_x != origin_x || cursor.origin_y != origin_y {
        return RetainedNodePaintStep::Fault;
    }
    let Some(node) = tree.node(id) else { return RetainedNodePaintStep::Fault };
    let unscoped_body_font_size = theme.font_size_body;
    let mut scoped_theme = *theme;
    if semio_framework_ui_wgpu_mounted_layout::owning_tree_spec(tree, id).is_some_and(|owner| owner.presentation == ui_contract::TreePresentation::Compact) {
        scoped_theme.font_size_body = SIZE_TINY;
        scoped_theme.font_size_small = SIZE_TINY;
    }
    let theme = &scoped_theme;
    let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault };
    let bounds = Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height);
    let presence = node.spec.0.presence();
    if !presence.visible() {
        return cursor.finish();
    }
    let mut flags = node.flags;
    if presence.state != UiState::Disabled {
        flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
    }
    if skeleton_replaces_content(&node.spec.0, presence) {
        if cursor.phase == 0 {
            let step = retained_skeleton_step(&node.spec.0, bounds, theme, draw);
            cursor.advance(1);
            return step;
        }
        return match retained_presence_step_with_selection(draw, bounds, theme, presence, node_selection_outline(&node.spec.0)) {
            RetainedNodePaintStep::Complete => cursor.finish(),
            step => step,
        };
    }
    let step =
        match &node.spec.0 {
            UiNode::Text(text) => paint_text_node_step(text, atlas, draw, cursor, theme, bounds, presence),
            UiNode::Stack(stack) => paint_stack_node_step(stack, tree, id, draw, cursor, theme, bounds, presence, flags),
            UiNode::Separator(_) => paint_separator_node_step(draw, cursor, theme, bounds, presence),
            UiNode::Button(button) => paint_button_node_step(button, atlas, icons, draw, cursor, theme, bounds, presence, flags),
            UiNode::Input(input_node) => paint_input_node_step(input_node, atlas, draw, cursor, node, theme, bounds, presence, flags),
            UiNode::Select(select) => paint_select_node_step(select, tree, id, atlas, icons, draw, cursor, node, theme, bounds, presence, flags),
            UiNode::Toggle(toggle) => paint_toggle_node_step(toggle, atlas, icons, inline, draw, cursor, node, theme, bounds, presence, flags),
            UiNode::KeyValue(key_value) => paint_key_value_node_step(key_value, atlas, draw, cursor, theme, bounds, presence),
            UiNode::Slider(slider) => paint_slider_node_step(slider, atlas, inline, draw, cursor, node, unscoped_body_font_size, theme, bounds, presence, layout.inline_suffix_width),
            UiNode::NumberStepper(stepper) => paint_number_stepper_node_step(stepper, atlas, icons, inline, draw, cursor, node, theme, bounds, presence, flags),
            UiNode::Ring(ring) => paint_ring_node_step(ring, draw, cursor, theme, bounds, presence),
            UiNode::Progress(progress) => paint_progress_node_step(progress, draw, cursor, theme, bounds, presence),
            UiNode::IconSelect(select) => paint_icon_select_node_step(select, atlas, draw, cursor, node, theme, bounds, presence),
            UiNode::Field(field) => paint_field_node_step(field, tree, id, atlas, draw, cursor, theme, bounds, presence),
            UiNode::Section(section) => paint_section_node_step(section, tree, id, atlas, draw, cursor, theme, bounds, presence),
            UiNode::Group(_) if semio_framework_ui_wgpu_mounted_layout::owning_tree_spec(tree, id).is_some() => retained_presence_step(draw, bounds, theme, presence),
            UiNode::Group(group) => paint_group_node_step(group, atlas, icons, draw, cursor, theme, bounds, presence),
            UiNode::Tree(tree_node) => retained_tree_node_step(tree, id, tree_node, bounds, theme, driver_drag, block_reversed, inline, atlas, icons, draw, cursor),
            UiNode::Image(image) => paint_image_node_step(image, atlas, has_scene_host, draw, cursor, theme, bounds, presence),
            UiNode::ComponentScene(scene) => paint_component_scene_node_step(scene, atlas, has_scene_host, draw, cursor, theme, bounds, presence),
            UiNode::ExternalSlot(slot) => paint_external_slot_node_step(slot, atlas, draw, cursor, theme, bounds, presence),
        };
    match step {
        RetainedNodePaintStep::Complete => cursor.finish(),
        step => step,
    }
}
// #endregion paint
