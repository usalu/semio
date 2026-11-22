//! 🎛️ Slider, stepper and progress nodes: the retained paint steps of the slider, number stepper, ring and progress nodes. See [`super`].

use super::*;
use std::borrow::Cow;

/// 🎚 Paints one step of a slider node: track, thumb, value and unit label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_slider_node_step(
    slider: &semio_ui_declaration::ui::UiSliderNode,
    atlas: &mut FontAtlas,
    inline: ui_contract::FlowInline,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    node: &Node,
    unscoped_body_font_size: f32,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    inline_suffix_width: f32,
    flags: NodeFlags,
    hover_pointer: Option<(f32, f32)>,
) -> RetainedNodePaintStep {
    let value = node.state.slider_draft_value.unwrap_or(slider.value);
    let unit_label = slider.unit_label();
    let control = semio_ui_wgpu_layout::layout::slider_control_presentation(bounds, slider.axis_position(value), 0.0, 1.0, unit_label.as_ref().map(|_| inline_suffix_width), theme.gap_standard, inline);
    let presentation = control.slider;
    let disabled = presence.state == UiState::Disabled;
    let pointer_in = |cell: Rect| hover_pointer.map_or(true, |(x, y)| cell.contains(x, y));
    let hovered = flags.contains(NodeFlags::HOVERED) && pointer_in(presentation.track_cell);
    let travel = SliderTravelState { hovered, dragging: node.state.scrub_gesture.is_some(), focus_visible: focus_ring_visible(flags), disabled };
    let refusal = node.state.number_refusal.as_deref().filter(|_| node.state.edit.is_some());
    match cursor.phase {
        0 if refusal.is_some_and(|text| !text.is_empty()) => {
            cursor.advance(1);
            RetainedNodePaintStep::Pending
        }
        0 => {
            let result = retained_fixed_output_budgeted(draw, RETAINED_NODE_FIXED_OUTPUT_ITEMS.max(8 + slider.snaps.len()), |draw| paint_slider_travel(slider, value, presentation, theme, travel, draw));
            cursor.advance(2);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => match retained_text_node_step(refusal.unwrap_or_default(), presentation.track_cell, theme.font_size_small, theme.error, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(2);
                RetainedNodePaintStep::Pending
            }
            step => step,
        },
        2 => {
            let formatted = slider.readout(value);
            let label = node.state.edit.as_ref().map_or(formatted.as_str(), |edit| edit.text.as_str());
            let value_hovered = flags.contains(NodeFlags::HOVERED) && hover_pointer.is_some_and(|(x, y)| presentation.value_cell.contains(x, y));
            let color = if refusal.is_some() { theme.error } else if node.state.edit.is_some() || value_hovered { theme.text } else { theme.text_element };
            let size = if node.state.edit.is_some() { unscoped_body_font_size } else { theme.font_size_small };
            match retained_text_end_step(label, presentation.value_cell, size, color, inline, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(3);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        3 => {
            let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                cursor.advance(4);
                return RetainedNodePaintStep::Pending;
            };
            let alignment = if inline.is_rtl() { RetainedCaretAlignment::Start } else { RetainedCaretAlignment::End };
            match retained_caret_step(&edit.text, edit.caret, presentation.value_cell, unscoped_body_font_size, alignment, theme.accent, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(4);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        4 => {
            let Some(label) = unit_label else {
                cursor.advance(5);
                return RetainedNodePaintStep::Pending;
            };
            let Some(unit_cell) = control.unit_cell else { return RetainedNodePaintStep::Fault };
            match retained_tree_text_step(&label, unit_cell, unscoped_body_font_size, theme.text_muted, inline, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(5);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        _ if disabled => retained_presence_step(draw, bounds, theme, &UiPresence { state: UiState::Normal, ..presence.clone() }),
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// ➖️ What a mixed stepper shows when the host names no label, as React's `mixedLabel || \"—\"`.
const STEPPER_MIXED_FALLBACK: &str = "—";

/// 🔭️ A stepper's live view: its edit buffer, else its value text, else the mixed caption.
fn stepper_view<'a>(stepper: &semio_ui_declaration::ui::UiNumberStepperNode, node: &'a Node, mixed: &'a str) -> EditView<'a> {
    let published = if stepper.uniform { Cow::Owned(stepper.value_text(stepper.value)) } else { Cow::Borrowed(mixed) };
    let mut view = edit_view(node.state.edit.as_ref(), node.state.composition_cursor, "", );
    if !view.editing {
        view.display = published;
    }
    view
}

/// 🎯️ The centred single-row text rectangle a stepper's value is painted in.
fn stepper_geometry(view: &EditView, segment: Rect, theme: &Theme, atlas: &mut FontAtlas) -> EditGeometry {
    let width = atlas.measure_range(&view.display, 0, view.display.len(), theme.font_size_body).min(segment.w);
    EditGeometry { inner: Rect::new(segment.x + (segment.w - width).max(0.0) * 0.5, segment.y, width, segment.h), size: theme.font_size_body, multi: false, mono: false }
}

/// 🔢 Paints one step of a number stepper node: segments, chrome, icons and value text.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_number_stepper_node_step(
    stepper: &semio_ui_declaration::ui::UiNumberStepperNode,
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
    presentation: RetainedPaintPresentation,
) -> RetainedNodePaintStep {
    let mixed = Some(ui_common_label(presentation.locale, |labels| labels.mixed_values)).filter(|label| !label.is_empty()).unwrap_or(STEPPER_MIXED_FALLBACK);
    let [decrement, value_segment, increment] = semio_ui_wgpu_layout::layout::number_stepper_segments(bounds, inline, theme.stroke_hairline);
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| draw.push_scissor(bounds));
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => {
            let result = retained_fixed_output_budgeted(draw, RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS, |draw| {
                push_stepper_chrome(draw, bounds, [decrement, value_segment, increment], inline, flags, if presence.state == UiState::Disabled { None } else { node.state.stepper_hovered_segment }, theme);
            });
            cursor.advance(2);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        2 => {
            let result = retained_fixed_output(draw, |draw| push_stepper_icon(draw, icons, "minus", decrement, inline, theme));
            cursor.advance(3);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        3 => {
            let view = stepper_view(stepper, node, mixed);
            if view.selection.is_none() && view.preedit.is_none() {
                cursor.advance(4);
                return RetainedNodePaintStep::Pending;
            }
            let geometry = stepper_geometry(&view, value_segment, theme, atlas);
            match retained_edit_span_step(&view, &geometry, (0.0, 0.0), theme, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(4);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        4 => {
            let view = stepper_view(stepper, node, mixed);
            let color = if node.state.number_refusal.is_some() && node.state.edit.is_some() { theme.error } else if stepper.uniform || node.state.edit.is_some() { theme.text } else { theme.text_muted };
            let selection = view.selection;
            let byte = cursor.glyph.byte();
            let tint = if selection.is_some_and(|(start, end)| byte >= start && byte < end) { theme.border_emphasized } else { color };
            match retained_stepper_text_step(&view.display, value_segment, tint, theme, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(5);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        5 => {
            let view = stepper_view(stepper, node, mixed);
            if !(view.editing && node.state.caret_visible && view.selection.is_none()) {
                cursor.advance(7);
                return RetainedNodePaintStep::Pending;
            }
            let geometry = stepper_geometry(&view, value_segment, theme, atlas);
            match retained_edit_caret_probe_step(&view, &geometry, atlas, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(6);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        6 => {
            let view = stepper_view(stepper, node, mixed);
            let geometry = stepper_geometry(&view, value_segment, theme, atlas);
            match retained_edit_caret_draw(&geometry, (0.0, 0.0), theme.text, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(7);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        7 => {
            let result = retained_fixed_output(draw, |draw| push_stepper_icon(draw, icons, "plus", increment, inline, theme));
            cursor.advance(8);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        8 => {
            let result = retained_fixed_output(draw, |draw| draw.pop_scissor());
            cursor.advance(9);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// ⭕ Paints one step of a ring node: the track chords, then the selected orb — `RingView` always passes `selected: true`.
pub(super) fn paint_ring_node_step(
    ring: &semio_ui_declaration::ui::UiRingNode,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.item < RING_TRACK_SEGMENTS {
        let result = retained_fixed_output(draw, |draw| paint_ring_track_segment(draw, bounds, cursor.item, theme));
        cursor.item += 1;
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| paint_ring_orb(draw, bounds, ring.t, true, false, presence.state == UiState::Disabled, theme));
        cursor.phase = 1;
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}

/// 📶 Paints one step of a progress node: the bar track and fill.
pub(super) fn paint_progress_node_step(
    progress: &semio_ui_declaration::ui::UiProgressNode,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| paint_progress(progress, bounds, theme, draw));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}
