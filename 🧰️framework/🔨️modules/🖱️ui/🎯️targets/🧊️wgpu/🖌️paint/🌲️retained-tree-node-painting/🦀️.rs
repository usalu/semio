//! 🌲️ Retained tree node step: the tree node paint step and the `paint_node_step` entry point. See [`super`].

use super::*;

// #region paint
pub(super) fn retained_tree_node_step(
    retained: &UiTree,
    tree_id: NodeId,
    tree: &UiTreeNode,
    bounds: Rect,
    theme: &Theme,
    driver_drag: UiDriverDrag,
    reversed: bool,
    inline: ui_contract::FlowInline,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    if tree.sections.len() > RETAINED_NODE_COLLECTION_ITEMS {
        return RetainedNodePaintStep::Fault;
    }
    let metrics = TreeRowMetrics::for_presentation(theme, tree.presentation).with_inline(inline);
    let font_size = if tree.presentation == ui_contract::TreePresentation::Compact { SIZE_TINY } else { theme.font_size_body };
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| draw.push_scissor(bounds));
            let content_height = retained_tree_content_height(retained, tree_id, tree, &metrics);
            cursor.row_y = if reversed { bounds.y + (bounds.h - content_height).max(0.0) } else { bounds.y };
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => {
            let Some(section) = retained_tree_section_at(tree, cursor.section, reversed) else {
                cursor.advance(10);
                return RetainedNodePaintStep::Pending;
            };
            if section.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            if !section.presence.visible() {
                cursor.section += 1;
                cursor.advance(1);
                return RetainedNodePaintStep::Pending;
            }
            if semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id).is_some() {
                return retained_table_header_step(retained, tree_id, tree, bounds, theme, reversed, inline, &metrics, atlas, draw, cursor);
            }
            let open = retained.tree_section_open(tree_id, &section.id, semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section));
            let header_height = tree_section_header_height(section, &metrics);
            let section_height = retained.explicit_child(tree_id, &section.id).map_or(header_height, |section_id| semio_framework_ui_wgpu_mounted_layout::live_tree_section_height(retained, section_id, section, &metrics));
            let header_y = if reversed { cursor.row_y + section_height - header_height } else { cursor.row_y };
            if !cursor.chrome {
                cursor.chrome = true;
                let Some(label) = section.label.as_ref() else { return RetainedNodePaintStep::Pending };
                let color = if open { theme.text_element } else { theme.text_muted };
                let result = retained_fixed_output(draw, |draw| {
                    if let Some(icons) = icons {
                        let offset = TREE_TOGGLE_WIDTH + theme.gap_standard;
                        let x = if inline.is_rtl() { bounds.x + bounds.w - offset - TREE_ICON_SIZE } else { bounds.x + offset };
                        push_icon(draw, icons, "folder", x, header_y + (header_height - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, color);
                    }
                });
                let _ = label;
                return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
            }
            let Some(label) = section.label.as_ref() else {
                cursor.advance(2);
                return RetainedNodePaintStep::Pending;
            };
            let color = if open { theme.text_element } else { theme.text_muted };
            let offset = TREE_TOGGLE_WIDTH + TREE_ICON_SIZE + theme.gap_standard * 2.0;
            let toolbar_reserve = section.header_toolbar.as_ref().map_or(0.0, |_| metrics.control_width + theme.gap_standard);
            let label_x = if inline.is_rtl() { bounds.x + toolbar_reserve } else { bounds.x + offset };
            let label_bounds = Rect::new(label_x, header_y, (bounds.w - offset - toolbar_reserve).max(0.0), header_height);
            let header_font_size = if tree.presentation == ui_contract::TreePresentation::Compact { SIZE_TINY } else { theme.font_size_small };
            match retained_tree_text_step(label.as_str(), label_bounds, header_font_size, color, inline, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    if !reversed {
                        cursor.row_y += header_height;
                    }
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        2 => {
            let Some(section) = retained_tree_section_at(tree, cursor.section, reversed) else { return RetainedNodePaintStep::Fault };
            let open = retained.tree_section_open(tree_id, &section.id, semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section));
            if !open || section.items.is_empty() {
                if reversed {
                    cursor.row_y += tree_section_header_height(section, &metrics);
                }
                cursor.section += 1;
                cursor.advance(1);
            } else {
                cursor.depth = 1;
                cursor.path[0] = 0;
                cursor.advance(3);
            }
            RetainedNodePaintStep::Pending
        }
        3 => {
            let Some(item) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            if item.items.as_ref().is_some_and(|items| items.len() > RETAINED_NODE_COLLECTION_ITEMS)
                || item.actions.as_ref().is_some_and(|actions| actions.len() > RETAINED_NODE_COLLECTION_ITEMS)
                || item.control.as_ref().is_some_and(|control| !retained_control_is_bounded(control))
            {
                return RetainedNodePaintStep::Fault;
            }
            let Some(visited) = cursor.visited.checked_add(1) else { return RetainedNodePaintStep::Fault };
            if visited > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            cursor.visited = visited;
            if !item.presence.visible() {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            }
            let Some((row, open)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let selected = item.presence.selected;
            let previewed = item.presence.state == UiState::Previewed;
            let on_hover_fill = previewed || item.presence.hover;
            let result = retained_fixed_output(draw, |draw| {
                if selected {
                    draw.push_rounded([row.x, row.y, row.w, row.h], theme.selected, theme.border_radius);
                } else if on_hover_fill {
                    draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
                }
                let ring_color = if selected { theme.selected } else { theme.border_normal };
                match item.presence.status {
                    UiStatus::Loading => paint_loading_border(draw, row, ring_color, theme),
                    UiStatus::Waiting => paint_waiting_border(draw, row, ring_color, theme),
                    UiStatus::Finished => draw.push_finished_border([row.x, row.y, row.w, row.h], ring_color, theme.border_radius, theme.stroke_hairline),
                    UiStatus::Idle => {}
                }
                if matches!(item.presence.state, UiState::Introducing | UiState::Celebrating) {
                    let celebrating = item.presence.state == UiState::Celebrating;
                    let turns = if celebrating { celebrate_turns(draw.clock_seconds(), theme.celebrate_duration_seconds) } else { 0.0 };
                    let ring = if celebrating { celebrate_color(theme, turns) } else { theme.accent };
                    let stroke = if celebrating { celebrate_stroke(theme, turns) } else { theme.stroke_hairline };
                    draw.push_introducing_border([row.x, row.y, row.w, row.h], ring, theme.border_radius, stroke);
                }
                let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
                if item.items.as_ref().is_some_and(|items| !items.is_empty()) {
                    if let Some(icons) = icons {
                        let chevron = if open {
                            if reversed {
                                "chevron-up"
                            } else {
                                "chevron-down"
                            }
                        } else if reversed == inline.is_rtl() {
                            "chevron-right"
                        } else {
                            "chevron-left"
                        };
                        let x = if inline.is_rtl() { bounds.x + bounds.w - (indent - bounds.x) } else { indent - TREE_TOGGLE_WIDTH };
                        push_icon(draw, icons, chevron, x, row.y + (row.h - SIZE_TINY) * 0.5, SIZE_TINY, foreground_on_fill(theme, theme.text_element, selected, on_hover_fill));
                    }
                }
                if let (Some(icons), Some(icon_id)) = (icons, item.icon_id) {
                    let color = foreground_on_fill(theme, item.tone.and_then(|tone| theme.tone_ink(tone)).unwrap_or(theme.text_element), selected, on_hover_fill);
                    let x = if inline.is_rtl() { bounds.x + bounds.w - (indent - bounds.x) - TREE_ICON_SIZE } else { indent };
                    push_icon(draw, icons, icon_id.as_str(), x, row.y + (row.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, color);
                }
            });
            cursor.advance(4);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        4 => {
            let Some(item) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            if !item.presence.visible() {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            }
            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            if semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id).is_some() {
                return retained_table_cells_step(retained, tree_id, tree, item, row, font_size, theme, inline, &metrics, reversed, atlas, draw, cursor);
            }
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
            let selected = item.presence.selected;
            let previewed = item.presence.state == UiState::Previewed;
            let on_hover_fill = previewed || item.presence.hover;
            let color = foreground_on_fill(theme, item.tone.and_then(|tone| theme.tone_ink(tone)).unwrap_or(theme.text_element), selected, on_hover_fill);
            let color = if item.dimmed.unwrap_or(false) || item.presence.state == UiState::Disabled { color.with_alpha(color.a * 0.5) } else { color };
            let trailing = if driver_drag == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
            let value_width = if item.control.is_some() || item.content_lines.is_some() || item.inline_toolbar.is_some() { metrics.control_width + metrics.gap * 2.0 } else { 0.0 };
            let label_width = (bounds.x + bounds.w - trailing - label_x - value_width).max(0.0);
            let label_x = if inline.is_rtl() { bounds.x + value_width + trailing } else { label_x };
            match retained_tree_text_step(item.label.as_str(), Rect::new(label_x, row.y, label_width, row.h), font_size, color, inline, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(5);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        5 => {
            let Some(item) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            let noted;
            let description = match (item.description.as_deref(), retained.presence_note(&item.id)) {
                (Some(description), Some(note)) => {
                    noted = format!("{description} · {note}");
                    noted.as_str()
                }
                (Some(line), None) | (None, Some(line)) => line,
                (None, None) => {
                    cursor.advance(6);
                    return RetainedNodePaintStep::Pending;
                }
            };
            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let offset = item.label.as_str().len().min(RETAINED_NODE_COLLECTION_ITEMS) as f32 * theme.font_size_body * 0.5;
            let description_x = indent + TREE_ICON_SIZE + theme.gap_standard + offset;
            let trailing = if driver_drag == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
            let value_width = if item.control.is_some() || item.content_lines.is_some() || item.inline_toolbar.is_some() { metrics.control_width + metrics.gap * 2.0 } else { 0.0 };
            let description_width = (bounds.x + bounds.w - trailing - value_width - description_x).max(1.0);
            let emphasized = item.presence.selected || item.presence.state == UiState::Previewed || item.presence.hover;
            let description_ink = foreground_on_fill(theme, theme.text_muted, item.presence.selected, emphasized && !item.presence.selected);
            match retained_tree_text_step(description, Rect::new(description_x, row.y, description_width, row.h), theme.font_size_small, description_ink, inline, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(6);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        6 => {
            let Some(item) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            if !cursor.chrome {
                cursor.chrome = true;
                if driver_drag == UiDriverDrag::Handle {
                    if let (Some(icons), Some(role)) = (icons, tree_drag_role(item)) {
                        let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
                        let handle = tree_drag_handle_rect(bounds.w, &metrics.for_item(item));
                        let icon = if role == TreeDragRole::Transfer { "move" } else { "grip-vertical" };
                        let on_hover_fill = item.presence.state == UiState::Previewed || item.presence.hover;
                        let color = foreground_on_fill(theme, theme.text_muted, item.presence.selected, on_hover_fill);
                        let result = retained_fixed_output(draw, |draw| push_icon(draw, icons, icon, bounds.x + handle.x, row.y + handle.y, handle.w, color));
                        return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
                    }
                }
            }
            let actions = item.actions.as_deref().unwrap_or(&[]);
            if cursor.item >= actions.len() {
                cursor.advance(7);
                return RetainedNodePaintStep::Pending;
            }
            let action = &actions[actions.len() - cursor.item - 1];
            cursor.item += 1;
            if action.placement() == UiTreeActionPlacement::Menu {
                return RetainedNodePaintStep::Pending;
            }
            let trailing = if driver_drag == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let slot = semio_framework_ui_wgpu_layout::layout::tree_row_action_rect(row.w, row.h, cursor.item, trailing, &metrics);
            let result = retained_fixed_output(draw, |draw| {
                if let Some(icons) = icons {
                    let on_hover_fill = item.presence.state == UiState::Previewed || item.presence.hover;
                    let color = foreground_on_fill(theme, theme.text_element, item.presence.selected, on_hover_fill);
                    let color = if action.disabled { color.with_alpha(color.a * 0.5) } else { color };
                    push_icon(draw, icons, action.icon_id.as_str(), row.x + slot.x, row.y + slot.y, slot.w, color);
                }
            });
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        7 => {
            cursor.advance(8);
            RetainedNodePaintStep::Pending
        }
        8 => {
            let Some(item) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            let Some((_, open)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            if item.presence.visible() && (!reversed || !open) {
                let advance = retained_tree_row_advance(retained, tree_id, tree, item, cursor, &metrics, reversed);
                cursor.row_y += advance;
            }
            cursor.selected = None;
            if open {
                if cursor.depth >= RETAINED_TREE_DEPTH {
                    return RetainedNodePaintStep::Fault;
                }
                cursor.path[cursor.depth] = 0;
                cursor.depth += 1;
                cursor.advance(3);
            } else {
                cursor.ascending = true;
                cursor.advance(9);
            }
            RetainedNodePaintStep::Pending
        }
        9 => {
            if !cursor.ascending || cursor.depth == 0 {
                return RetainedNodePaintStep::Fault;
            }
            let Some(siblings) = retained_tree_sibling_count(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
            let level = cursor.depth - 1;
            if cursor.path[level] + 1 < siblings {
                cursor.path[level] += 1;
                cursor.advance(3);
            } else {
                cursor.path[level] = 0;
                cursor.depth -= 1;
                if reversed && cursor.depth > 0 {
                    let Some(parent) = retained_tree_item_at(tree, cursor, reversed) else { return RetainedNodePaintStep::Fault };
                    cursor.row_y += metrics.for_item(parent).row_height;
                }
                if cursor.depth == 0 {
                    if reversed {
                        if let Some(section) = retained_tree_section_at(tree, cursor.section, reversed) {
                            cursor.row_y += tree_section_header_height(section, &metrics);
                        }
                    }
                    cursor.section += 1;
                    cursor.advance(1);
                }
            }
            RetainedNodePaintStep::Pending
        }
        10 => {
            let result = retained_fixed_output(draw, DrawList::pop_scissor);
            cursor.advance(11);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        _ => retained_presence_step(draw, bounds, theme, &tree.presence),
    }
}

/// 🎨️ Advances one exact glyph or one independently pre-admitted bounded widget child.
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
pub fn paint_node_step(
    tree: &UiTree,
    id: NodeId,
    origin_x: f32,
    origin_y: f32,
    theme: &Theme,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    has_scene_host: bool,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    paint_node_step_with_driver(tree, id, origin_x, origin_y, theme, atlas, icons, has_scene_host, UiDriverDrag::Handle, false, ui_contract::FlowInline::Ltr, draw, cursor)
}
// #endregion paint
