//! 🎚️ Slider, stepper and progress nodes: the retained paint steps of the slider, number stepper, ring and progress nodes. See [`super`].

use super::*;

/// 🎚 Paints one step of a slider node: track, thumb, value and unit label.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_slider_node_step(
    slider: &semio_framework_ui_declaration::ui::UiSliderNode,
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
) -> RetainedNodePaintStep {
    let value = node.state.slider_draft_value.unwrap_or(slider.value);
    let unit_label = slider.unit_label();
    let control = semio_framework_ui_wgpu_layout::layout::slider_control_presentation(bounds, slider.axis_position(value), 0.0, 1.0, unit_label.as_ref().map(|_| inline_suffix_width), theme.gap_standard, inline);
    let presentation = control.slider;
    let refusal = node.state.number_refusal.as_deref().filter(|_| node.state.edit.is_some());
    match cursor.phase {
        0 if refusal.is_some_and(|text| !text.is_empty()) => {
            cursor.advance(1);
            RetainedNodePaintStep::Pending
        }
        0 => {
            let result = retained_fixed_output_budgeted(draw, RETAINED_NODE_FIXED_OUTPUT_ITEMS.max(4 + slider.snaps.len()), |draw| paint_slider_travel(slider, value, presentation, theme, draw));
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
            let color = if refusal.is_some() { theme.error } else if node.state.edit.is_some() { theme.text } else { theme.text_element };
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
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// 🔢 Paints one step of a number stepper node: segments, chrome, icons and value text.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_number_stepper_node_step(
    stepper: &semio_framework_ui_declaration::ui::UiNumberStepperNode,
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
    let [decrement, value_segment, increment] = semio_framework_ui_wgpu_layout::layout::number_stepper_segments(bounds, inline, theme.stroke_hairline);
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
                push_stepper_chrome(draw, bounds, [decrement, value_segment, increment], inline, flags, node.state.stepper_hovered_segment, theme);
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
            let label = if let Some(edit) = node.state.edit.as_ref() {
                let mut label = edit.text.clone();
                if let Some(composition) = edit.composition.as_ref() {
                    label.insert_str(edit.caret, composition);
                }
                label
            } else if stepper.uniform {
                stepper.value_text(stepper.value)
            } else {
                UI_INSPECTOR_MIXED_PLACEHOLDER.to_string()
            };
            let color = if node.state.number_refusal.is_some() && node.state.edit.is_some() { theme.error } else if stepper.uniform || node.state.edit.is_some() { theme.text } else { theme.text_muted };
            match retained_stepper_text_step(&label, value_segment, color, theme, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(4);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        4 => {
            let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                cursor.advance(5);
                return RetainedNodePaintStep::Pending;
            };
            let mut label = edit.text.clone();
            let mut caret = edit.caret;
            if let Some(composition) = edit.composition.as_ref() {
                label.insert_str(edit.caret, composition);
                caret += composition.len();
            }
            match retained_caret_step(&label, caret, value_segment, theme.font_size_body, RetainedCaretAlignment::Center, theme.accent, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(5);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        5 => {
            let result = retained_fixed_output(draw, |draw| push_stepper_icon(draw, icons, "plus", increment, inline, theme));
            cursor.advance(6);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        6 => {
            let result = retained_fixed_output(draw, |draw| draw.pop_scissor());
            cursor.advance(7);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        _ => retained_presence_step(draw, bounds, theme, presence),
    }
}

/// ⭕ Paints one step of a ring node: the progress ring and its label.
pub(super) fn paint_ring_node_step(
    ring: &semio_framework_ui_declaration::ui::UiRingNode,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    let segments = 48usize;
    if cursor.item < segments {
        let cx = bounds.x + bounds.w * 0.5;
        let cy = bounds.y + bounds.h * 0.5;
        let radius = bounds.w.min(bounds.h) * 0.4;
        let a0 = std::f32::consts::TAU * cursor.item as f32 / segments as f32;
        let a1 = std::f32::consts::TAU * (cursor.item + 1) as f32 / segments as f32;
        let result = retained_fixed_output(draw, |draw| draw.push_line(cx + a0.cos() * radius, cy + a0.sin() * radius, cx + a1.cos() * radius, cy + a1.sin() * radius, theme.separator, 2.0));
        cursor.item += 1;
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| {
            let cx = bounds.x + bounds.w * 0.5;
            let cy = bounds.y + bounds.h * 0.5;
            let radius = bounds.w.min(bounds.h) * 0.4;
            let angle = std::f32::consts::TAU * ring.t as f32;
            draw.push_rounded([cx + angle.cos() * radius - 6.0, cy + angle.sin() * radius - 6.0, 12.0, 12.0], theme.accent, 6.0);
        });
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
    progress: &semio_framework_ui_declaration::ui::UiProgressNode,
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
