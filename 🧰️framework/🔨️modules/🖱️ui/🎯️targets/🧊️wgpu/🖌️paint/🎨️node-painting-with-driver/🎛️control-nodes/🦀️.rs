//! 🎛️ Control nodes: the retained paint steps of the button, input, select, toggle and icon select nodes. See [`super`].

use super::*;
use semio_ui_wgpu_paint_widgets::hover_owner;

/// 🔘 Paints one step of a button node: chrome, border and label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_button_node_step(
    button: &semio_ui_declaration::ui::UiButtonNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
    labels: UiDriverLabels,
) -> RetainedNodePaintStep {
    let icon_only = labels == UiDriverLabels::Icons;
    match cursor.phase {
        0 => {
            let hovered = presence.state != UiState::Disabled && hover_owner(flags);
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, bounds, theme, if hovered { theme.border_emphasized } else if focus_ring_visible(flags) { theme.selected } else { theme.border_normal }, item_bg(theme, false, hovered), if hovered { semio_ui_draw_contract::SharedBorderState::Hover } else if focus_ring_visible(flags) { semio_ui_draw_contract::SharedBorderState::Focus } else { semio_ui_draw_contract::SharedBorderState::Normal });
                if let Some(icons) = icons {
                    let icon_x = if icon_only { bounds.x + (bounds.w - ICON_TINY) * 0.5 } else { bounds.x + theme.padding_standard };
                    push_icon(draw, icons, button.icon_id.as_str(), icon_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, false, hovered));
                }
            });
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 if icon_only => {
            cursor.advance(2);
            RetainedNodePaintStep::Pending
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

/// ⌨ Paints one step of an input node: chrome, selection and preedit highlight, text, caret and refusal message.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_input_node_step(
    input_node: &semio_ui_declaration::ui::UiInputNode,
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
    let edit = node.state.edit.as_ref();
    let refusal = node.state.number_refusal.as_deref().filter(|_| edit.is_some());
    let refused = refusal.is_some();
    let published = if input_node.value.is_empty() { input_node.placeholder.as_ref().map_or("", Label::as_str) } else { shown.as_deref().unwrap_or(input_node.value.as_str()) };
    let view = edit_view(edit, node.state.composition_cursor, published);
    let row = input_refusal_row(node, theme);
    let (frame, swatch, geometry) = input_geometry(input_node, &view.display, bounds, row, theme);
    let scroll = if geometry.multi { (0.0, node.state.scroll_offset.1) } else if view.editing { (node.state.scroll_offset.0, 0.0) } else { (0.0, 0.0) };
    let typed_empty = edit.map_or(input_node.value.is_empty(), |edit| edit.text.is_empty() && edit.composition.as_deref().is_none_or(str::is_empty));
    let color = if refused { theme.error } else if typed_empty { theme.text_muted } else { theme.text };
    let advance = |cursor: &mut RetainedNodePaintCursor, phase: u16| {
        cursor.advance(phase);
        RetainedNodePaintStep::Pending
    };
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, frame, theme, if refused { theme.error } else if focus_ring_visible(flags) { theme.selected } else { theme.border_normal }, theme.input_bg, if refused { semio_ui_draw_contract::SharedBorderState::Invalid } else if focus_ring_visible(flags) { semio_ui_draw_contract::SharedBorderState::Focus } else { semio_ui_draw_contract::SharedBorderState::Normal });
                if let Some((swatch, color, _)) = swatch {
                    draw.push_rounded([swatch.x, swatch.y, swatch.w, swatch.h], color, 2.0);
                }
                draw.push_scissor(frame);
            });
            cursor.advance(if view.editing { 1 } else { 3 });
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => match retained_edit_caret_probe_step(&view, &geometry, atlas, cursor) {
            RetainedNodePaintStep::Complete => advance(cursor, if view.selection.is_some() || view.preedit.is_some() { 2 } else { 3 }),
            step => step,
        },
        2 => match retained_edit_span_step(&view, &geometry, scroll, theme, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => advance(cursor, 3),
            step => step,
        },
        3 => match retained_edit_text_step(&view, &geometry, scroll, color, theme.border_emphasized, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => advance(cursor, 4),
            step => step,
        },
        4 => {
            if !(view.editing && node.state.caret_visible && view.selection.is_none()) {
                return advance(cursor, 5);
            }
            match retained_edit_caret_draw(&geometry, scroll, theme.text, draw, cursor) {
                RetainedNodePaintStep::Complete => advance(cursor, 5),
                step => step,
            }
        }
        5 => {
            let result = retained_fixed_output(draw, |draw| draw.pop_scissor());
            cursor.advance(6);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        6 => {
            let Some(message) = refusal.filter(|message| !message.is_empty()) else { return advance(cursor, 7) };
            let area = Rect::new(frame.x, frame.y + frame.h, frame.w, row);
            match retained_text_node_step(message, area, theme.font_size_small, theme.error, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => advance(cursor, 7),
                step => step,
            }
        }
        _ => retained_presence_step(draw, frame, theme, presence),
    }
}

/// 🔽 Paints one step of a select node: the closed control and the open popup.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_select_node_step(
    select: &semio_ui_declaration::ui::UiSelectNode,
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
    presentation: RetainedPaintPresentation,
    inline: ui_contract::FlowInline,
    hover_pointer: Option<(f32, f32)>,
) -> RetainedNodePaintStep {
    if select.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
        return RetainedNodePaintStep::Fault;
    }
    if select.appearance == ui_contract::SelectAppearance::Segmented {
        return paint_segmented_select_node_step(select, atlas, icons, draw, cursor, theme, bounds, presence, flags, presentation.driver.labels, inline, hover_pointer);
    }
    match cursor.phase {
        0 => {
            let hovered = presence.state != UiState::Disabled && hover_owner(flags);
            let result = retained_fixed_output(draw, |draw| {
                push_control_border(draw, bounds, theme, if hovered { theme.border_emphasized } else if focus_ring_visible(flags) { theme.selected } else { theme.border_normal }, if hovered { theme.button_hover } else { theme.input_bg }, if hovered { semio_ui_draw_contract::SharedBorderState::Hover } else if focus_ring_visible(flags) { semio_ui_draw_contract::SharedBorderState::Focus } else { semio_ui_draw_contract::SharedBorderState::Normal });
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
            let label = cursor.selected.and_then(|index| select.items.get(index)).map(|item| item.label.as_str()).or_else(|| select.placeholder.as_ref().map(Label::as_str)).unwrap_or_else(|| ui_common_label(presentation.locale, |labels| labels.select));
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
                push_chrome_border(draw, popup.menu, theme.stroke_hairline, theme.border_normal, true, true, true, true, semio_ui_draw_contract::SharedBorderState::Normal);
                draw.push_scissor(semio_ui_select_behavior::select_popup_viewport_rect(popup));
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
            let row = semio_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
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
            let painted = semio_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
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

/// 🔘️ Paints one step of a segmented select: React's radio `ToggleGroup` — every option a segment of one bordered group, the chosen one pressed, each with its caption beside a `circle-dot` icon (icon only under the `icons` driver), the focus ring on the chosen segment.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
fn paint_segmented_select_node_step(
    select: &semio_ui_declaration::ui::UiSelectNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
    labels: UiDriverLabels,
    inline: ui_contract::FlowInline,
    hover_pointer: Option<(f32, f32)>,
) -> RetainedNodePaintStep {
    let count = select.items.len();
    let rtl = inline.is_rtl();
    let enabled = presence.state != UiState::Disabled;
    let icon_only = labels == UiDriverLabels::Icons;
    let item = cursor.item;
    match cursor.phase {
        0 if item < count => {
            let segment = semio_ui_select_behavior::select_segment_rect(bounds, count, item, rtl);
            let selected = select.items[item].value == select.value;
            let hovered = enabled && hover_owner(flags) && hover_pointer.is_some_and(|(x, y)| segment.contains(x, y));
            let focused = selected && focus_ring_visible(flags);
            let result = retained_fixed_output(draw, |draw| {
                draw.push_solid([segment.x, segment.y, segment.w, segment.h], item_bg(theme, selected, hovered));
                if item > 0 {
                    let edge = if rtl { segment.x + segment.w - theme.stroke_hairline } else { segment.x };
                    draw.push_solid([edge, segment.y, theme.stroke_hairline, segment.h], theme.border_normal);
                }
                if focused {
                    push_control_border(draw, segment, theme, theme.selected, theme.panel.with_alpha(0.0), semio_ui_draw_contract::SharedBorderState::Focus);
                }
                if let Some(icons) = icons {
                    let icon_x = if icon_only { segment.x + (segment.w - ICON_TINY) * 0.5 } else if rtl { segment.x + theme.padding_standard } else { segment.x + segment.w - theme.padding_standard - ICON_TINY };
                    push_icon(draw, icons, "circle-dot", icon_x, segment.y + (segment.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, selected, hovered));
                }
            });
            cursor.phase = 1;
            cursor.glyph.reset();
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        0 => {
            let result = retained_fixed_output(draw, |draw| push_control_border(draw, bounds, theme, if enabled && hover_owner(flags) { theme.border_emphasized } else { theme.border_normal }, theme.panel.with_alpha(0.0), semio_ui_draw_contract::SharedBorderState::Normal));
            cursor.advance(2);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 if icon_only => {
            cursor.item += 1;
            cursor.phase = 0;
            RetainedNodePaintStep::Pending
        }
        1 => {
            let segment = semio_ui_select_behavior::select_segment_rect(bounds, count, item, rtl);
            let selected = select.items[item].value == select.value;
            let hovered = enabled && hover_owner(flags) && hover_pointer.is_some_and(|(x, y)| segment.contains(x, y));
            let reserved = theme.padding_standard * 2.0 + ICON_TINY + theme.gap_standard;
            let text_x = if rtl { segment.x + theme.padding_standard + ICON_TINY + theme.gap_standard } else { segment.x + theme.padding_standard };
            let text = Rect::new(text_x, bounds.y + (bounds.h - theme.font_size_small) * 0.5, (segment.w - reserved).max(0.0), theme.font_size_small * 1.5);
            match retained_text_node_step(select.items[item].label.as_str(), text, theme.font_size_small, item_text(theme, selected, hovered), atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.item += 1;
                    cursor.phase = 0;
                    cursor.glyph.reset();
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// 🔀 Paints one step of a toggle node: the switch or checkbox and its label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_toggle_node_step(
    toggle: &semio_ui_declaration::ui::UiToggleNode,
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
            let hovered = presence.state != UiState::Disabled && hover_owner(flags);
            let result = retained_fixed_output(draw, |draw| {
                if toggle.appearance == ui_contract::ToggleAppearance::Checkbox {
                    paint_checkbox(toggle, bounds, flags, theme, inline, draw);
                    return;
                }
                push_control_border(draw, bounds, theme, if hovered { theme.border_emphasized } else if focus_ring_visible(flags) { theme.selected } else { theme.border_normal }, item_bg(theme, pressed, hovered), if hovered { semio_ui_draw_contract::SharedBorderState::Hover } else if focus_ring_visible(flags) { semio_ui_draw_contract::SharedBorderState::Focus } else if pressed { semio_ui_draw_contract::SharedBorderState::Active } else { semio_ui_draw_contract::SharedBorderState::Normal });
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

/// 🧩 Paints one step of an icon select node: only what the selector owns itself, the preview cell with the decoded icon at `size={56}` (or its
/// glyph text, or `—`). The mode select, editor and import/clear buttons are its synthesized child cells, painted by the walk as the
/// real `Select`, `Input` and `Button` nodes they are. While locked (`disabled || !uniform`) the cell reads at React's `opacity-60`.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_icon_select_node_step(
    select: &semio_ui_declaration::ui::UiIconSelectNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    let locked = select.locked();
    let dim = |color: Rgba| if locked { color.with_alpha(color.a * 0.6) } else { color };
    let (mode, _) = semio_ui_declaration::ui::UiIconSelectNode::decode_mode(&select.value);
    let rows = semio_ui_wgpu_layout::layout::icon_select_rows(bounds, mode.editor_rows());
    match cursor.phase {
        0 => {
            let muted = dim(theme.muted.with_alpha(theme.muted.a * 0.3));
            let result = retained_fixed_output(draw, |draw| push_control_border(draw, rows.preview, theme, dim(theme.border_normal), muted, semio_ui_draw_contract::SharedBorderState::Normal));
            cursor.advance(4);
            if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending }
        }
        4 => {
            let value = select.value.trim();
            if let Some(icons) = icons.filter(|atlas| select.uniform && atlas.icon_uv(value).is_some()) {
                let size = ICON_PREVIEW_SIZE.min(rows.preview.h);
                let result = retained_fixed_output(draw, |draw| push_icon(draw, icons, value, rows.preview.x + (rows.preview.w - size) * 0.5, rows.preview.y + (rows.preview.h - size) * 0.5, size, dim(theme.text_element)));
                cursor.advance(6);
                return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
            }
            cursor.advance(5);
            RetainedNodePaintStep::Pending
        }
        5 => {
            let (mode, inner) = if select.uniform { semio_ui_declaration::ui::UiIconSelectNode::decode_mode(&select.value) } else { (semio_ui_declaration::ui::IconSelectorMode::Math, String::new()) };
            let glyphs = matches!(mode, semio_ui_declaration::ui::IconSelectorMode::Emoji | semio_ui_declaration::ui::IconSelectorMode::Text);
            let (label, color) = if glyphs && !inner.is_empty() { (inner, theme.text) } else { ("—".to_string(), theme.text_muted) };
            let small = Theme { font_size_body: theme.font_size_small, ..*theme };
            match retained_stepper_text_step(&label, rows.preview, dim(color), &small, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(6);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ if presence.state == UiState::Disabled => retained_presence_step(draw, bounds, theme, &UiPresence { state: UiState::Normal, ..presence.clone() }),
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// 🖼️ React's `<Icon size={56} />` preview edge.
const ICON_PREVIEW_SIZE: f32 = 56.0;
