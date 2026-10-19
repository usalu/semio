//! 🎛️ Control nodes: the retained paint steps of the button, input, select, toggle and icon select nodes. See [`super`].

use super::*;

/// 🔘 Paints one step of a button node: chrome, border and label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_button_node_step(
    button: &semio_framework_ui_declaration::ui::UiButtonNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
) -> RetainedNodePaintStep {
    match cursor.phase {
        0 => {
            let hovered = flags.contains(NodeFlags::HOVERED);
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, item_bg(theme, false, hovered));
                if let Some(icons) = icons {
                    push_icon(draw, icons, button.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, false, hovered));
                }
            });
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => match retained_text_node_step(button.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(2);
                RetainedNodePaintStep::Pending
            }
            step => step,
        },
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// ⌨ Paints one step of an input node: chrome, text, caret and selection.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_input_node_step(
    input_node: &semio_framework_ui_declaration::ui::UiInputNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    node: &Node,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
) -> RetainedNodePaintStep {
    let shown = input_node.display_factor.filter(|_| input_node.input_kind == "number").and_then(|factor| input_node.value.trim().parse::<f64>().ok().map(|stored| ui_contract::ui_number_display_text(stored, Some(factor), input_node.precision)));
    let refused = node.state.number_refusal.is_some() && node.state.edit.is_some();
    let display = node.state.edit.as_ref().map(|edit| edit.text.as_str()).filter(|text| !text.is_empty()).unwrap_or_else(|| {
        if input_node.value.is_empty() {
            input_node.placeholder.as_ref().map_or("", Label::as_str)
        } else {
            shown.as_deref().unwrap_or(input_node.value.as_str())
        }
    });
    let long_text = input_node.input_kind == "longText";
    let display = if long_text { text_line_view(display, node.state.edit.as_ref().filter(|edit| !edit.text.is_empty()).map(|edit| edit.caret)).0 } else { display };
    let swatch = (input_node.input_kind == "color").then(|| color_input_swatch(bounds, display)).flatten();
    let text_bounds = swatch.map_or(bounds, |(_, _, text_bounds)| text_bounds);
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, bounds, theme, if refused { theme.error } else if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, theme.input_bg);
                if let Some((swatch, color, _)) = swatch {
                    draw.push_rounded([swatch.x, swatch.y, swatch.w, swatch.h], color, 2.0);
                }
            });
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => match retained_text_node_step(display, text_bounds, theme.font_size_body, if refused { theme.error } else if input_node.value.is_empty() { theme.text_muted } else { theme.text }, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(2);
                RetainedNodePaintStep::Pending
            }
            step => step,
        },
        2 => {
            let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                cursor.advance(3);
                return RetainedNodePaintStep::Pending;
            };
            let (caret_line, caret_at) = if long_text { text_line_view(&edit.text, Some(edit.caret)) } else { (edit.text.as_str(), edit.caret) };
            match retained_caret_step(caret_line, caret_at, text_bounds, theme.font_size_body, RetainedCaretAlignment::Start, theme.accent, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(3);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// 🔽 Paints one step of a select node: the closed control and the open popup.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_select_node_step(
    select: &semio_framework_ui_declaration::ui::UiSelectNode,
    tree: &UiTree,
    id: NodeId,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    node: &Node,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
) -> RetainedNodePaintStep {
    if select.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
        return RetainedNodePaintStep::Fault;
    }
    match cursor.phase {
        0 => {
            let hovered = flags.contains(NodeFlags::HOVERED);
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, if hovered { theme.button_hover } else { theme.input_bg });
                if let Some(icons) = icons {
                    push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, foreground_on_fill(theme, theme.text_element, false, hovered));
                }
            });
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 if cursor.item < select.items.len() => {
            if select.items[cursor.item].value == select.value {
                cursor.selected = Some(cursor.item);
            }
            cursor.item += 1;
            RetainedNodePaintStep::Pending
        }
        1 => {
            cursor.advance(2);
            RetainedNodePaintStep::Pending
        }
        2 => {
            let label = cursor.selected.and_then(|index| select.items.get(index)).map(|item| item.label.as_str()).or_else(|| select.placeholder.as_ref().map(Label::as_str)).unwrap_or("Select…");
            match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(if node.state.open { 3 } else { 8 });
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        3 => {
            let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else { return RetainedNodePaintStep::Fault };
            let mut opened = false;
            let result = retained_fixed_output(draw, |draw| {
                let glass = draw.push_glass([popup.menu.x, popup.menu.y, popup.menu.w, popup.menu.h], theme.border_radius, theme.glass(Level::Menu));
                draw.begin_glass_content(glass);
                push_chrome_border(draw, popup.menu, theme.stroke_hairline, theme.border_normal, true, true, true, true);
                draw.push_scissor(semio_framework_ui_select_behavior::select_popup_viewport_rect(popup));
                opened = true;
            });
            cursor.phase = 4;
            cursor.item = popup.first_row;
            cursor.glyph.reset();
            cursor.popup_route = opened;
            if result.is_err() {
                let _ = retained_select_close_route(draw, cursor);
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        4 if node.state.select_popup.is_some_and(|popup| cursor.item < popup.last_row) => {
            let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                let _ = retained_select_close_route(draw, cursor);
                return RetainedNodePaintStep::Fault;
            };
            let row = semio_framework_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
            let item = &select.items[cursor.item];
            let highlighted = node.state.highlighted == Some(cursor.item);
            let result = retained_fixed_output(draw, |draw| {
                if highlighted || item.value == select.value {
                    draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
                }
            });
            cursor.phase = 5;
            cursor.glyph.reset();
            if result.is_err() {
                let _ = retained_select_close_route(draw, cursor);
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        4 => {
            cursor.advance(6);
            RetainedNodePaintStep::Pending
        }
        5 => {
            let Some(item) = select.items.get(cursor.item) else {
                let _ = retained_select_close_route(draw, cursor);
                return RetainedNodePaintStep::Fault;
            };
            let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                let _ = retained_select_close_route(draw, cursor);
                return RetainedNodePaintStep::Fault;
            };
            let painted = semio_framework_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
            let row = Rect::new(painted.x + theme.padding_standard, painted.y, (painted.w - theme.padding_standard).max(0.0), painted.h);
            match retained_text_node_step(item.label.as_str(), row, theme.font_size_body, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.item += 1;
                    cursor.phase = 4;
                    cursor.glyph.reset();
                    RetainedNodePaintStep::Pending
                }
                RetainedNodePaintStep::Fault => {
                    let _ = retained_select_close_route(draw, cursor);
                    RetainedNodePaintStep::Fault
                }
                step => step,
            }
        }
        6 => {
            let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                let _ = retained_select_close_route(draw, cursor);
                return RetainedNodePaintStep::Fault;
            };
            let result = retained_fixed_output(draw, |draw| {
                draw.pop_scissor();
                if let (Some(up), Some(down), Some(icons)) = (popup.up, popup.down, icons) {
                    let chevron = SIZE_TINY;
                    let center_x = popup.menu.x + (popup.menu.w - chevron) * 0.5;
                    push_icon(draw, icons, "chevron-up", center_x, up.y + (up.h - chevron) * 0.5, chevron, theme.text_muted);
                    push_icon(draw, icons, "chevron-down", center_x, down.y + (down.h - chevron) * 0.5, chevron, theme.text_muted);
                }
                draw.end_glass_content();
            });
            cursor.advance(7);
            if result.is_err() {
                let _ = retained_select_close_route(draw, cursor);
                RetainedNodePaintStep::Fault
            } else {
                cursor.popup_route = false;
                RetainedNodePaintStep::Pending
            }
        }
        7 => {
            let result = retained_select_close_route(draw, cursor);
            cursor.advance(8);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// 🔀 Paints one step of a toggle node: the switch or checkbox and its label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_toggle_node_step(
    toggle: &semio_framework_ui_declaration::ui::UiToggleNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    inline: ui_contract::FlowInline,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    node: &Node,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
) -> RetainedNodePaintStep {
    match cursor.phase {
        0 => {
            let pressed = toggle.presence.selected;
            let hovered = flags.contains(NodeFlags::HOVERED);
            let result = retained_fixed_output(draw, |draw| {
                if toggle.appearance == ui_contract::ToggleAppearance::Checkbox {
                    paint_checkbox(toggle, bounds, flags, theme, inline, draw);
                    return;
                }
                push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, item_bg(theme, pressed, hovered));
                if let Some(icons) = icons {
                    push_icon(draw, icons, toggle.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
                }
            });
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => {
            if toggle.appearance == ui_contract::ToggleAppearance::Checkbox {
                cursor.advance(2);
                return RetainedNodePaintStep::Pending;
            }
            let label = toggle.text.as_ref().map_or("", Label::as_str);
            match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ => retained_presence_step_with_selection(draw, bounds, theme, presence, node_selection_outline(&node.spec.0)),
    }
}

/// 🧩 Paints one step of an icon select node: the icon grid and its selection.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_icon_select_node_step(
    select: &semio_framework_ui_declaration::ui::UiIconSelectNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    node: &Node,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| push_control_border(draw, bounds, theme, theme.border_normal, theme.input_bg));
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => match retained_text_node_step(node.state.edit.as_ref().map_or(select.value.as_str(), |edit| edit.text.as_str()), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(2);
                RetainedNodePaintStep::Pending
            }
            step => step,
        },
        2 => {
            let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                cursor.advance(3);
                return RetainedNodePaintStep::Pending;
            };
            match retained_caret_step(&edit.text, edit.caret, bounds, theme.font_size_body, RetainedCaretAlignment::Start, theme.accent, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(3);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}
