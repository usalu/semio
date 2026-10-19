//! 📝️ Text and structure nodes: the retained paint steps of the text, stack, separator, key value, field, section and group nodes. See [`super`].

use super::*;

/// 📝 Paints one step of a text node: the weighted body or emphasized run.
pub(super) fn paint_text_node_step(
    text: &semio_framework_ui_declaration::ui::UiTextNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let emphasize = text.emphasize.unwrap_or(false);
        let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
        let color = theme.text;
        let weight = semio_framework_ui_text::TextWeight::of(emphasize);
        match retained_text_node_step_weighted(text.value.as_str(), bounds, size, color, weight, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(1);
                RetainedNodePaintStep::Pending
            }
            step => step,
        }
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}

/// 🗃 Paints one step of a stack node: its chrome frame and tree item decoration.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_stack_node_step(
    stack: &semio_framework_ui_declaration::ui::UiStackNode,
    tree: &UiTree,
    id: NodeId,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    flags: NodeFlags,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let owns_chrome = tree.authored_tree_item(id).is_none() && tree.authored_tree_section(id).is_none();
        let result = retained_fixed_output(draw, |draw| {
            if owns_chrome {
                paint_stack_frame(stack, bounds, flags, theme, draw);
            }
        });
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

/// ➖ Paints one step of a separator node: the hairline.
pub(super) fn paint_separator_node_step(
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| paint_separator(bounds, theme, draw));
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

/// 🔑 Paints one step of a key value node: the key and its value cell.
pub(super) fn paint_key_value_node_step(
    key_value: &semio_framework_ui_declaration::ui::UiKeyValueNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if key_value.entries.len() > RETAINED_NODE_COLLECTION_ITEMS {
        return RetainedNodePaintStep::Fault;
    }
    let Some(entry) = key_value.entries.get(cursor.item) else {
        cursor.advance(2);
        return match retained_presence_step(draw, bounds, theme, presence) {
            RetainedNodePaintStep::Complete => cursor.finish(),
            step => step,
        };
    };
    let row = Rect::new(bounds.x, bounds.y + cursor.item as f32 * theme.control_height, bounds.w, theme.control_height);
    if cursor.phase == 0 {
        match retained_text_node_step(entry.label.as_str(), row, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.phase = 1;
                cursor.glyph.reset();
                RetainedNodePaintStep::Pending
            }
            step => step,
        }
    } else {
        let value_bounds = Rect::new(row.x + row.w * 0.4, row.y, row.w * 0.6, row.h);
        match retained_text_node_step(entry.value.as_str(), value_bounds, theme.font_size_small, theme.text, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.item += 1;
                cursor.phase = 0;
                cursor.glyph.reset();
                RetainedNodePaintStep::Pending
            }
            step => step,
        }
    }
}

/// 🏷 Paints one step of a field node: label, required marker and help text.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_field_node_step(
    field: &semio_framework_ui_declaration::ui::UiFieldNode,
    tree: &UiTree,
    id: NodeId,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    let metrics = field_paint_metrics(tree, id, field, bounds, theme, atlas);
    let value = match cursor.phase {
        0 => metrics.label.map(|band| (field.label.as_str(), chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.text, semio_framework_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)),
        1 if field.required.unwrap_or(false) => metrics.label.map(|band| {
            let label_width = atlas.measure_text(field.label.as_str(), semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE).0;
            let mut rect = chrome_band_rect(bounds, band);
            rect.x += label_width + theme.gap_standard * 0.5;
            rect.w = (rect.w - label_width - theme.gap_standard * 0.5).max(0.0);
            ("*", rect, semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.error, semio_framework_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)
        }),
        2 => field
            .description
            .as_deref()
            .zip(metrics.description)
            .map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.text_muted, semio_framework_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
        3 => field.error.as_deref().zip(metrics.error).map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.error, semio_framework_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
        _ => None,
    };
    if let Some((value, rect, size, color, weight, flow)) = value {
        match paint_retained_glyph_step_weighted(value, rect, size, color, flow, weight, atlas, draw, &mut cursor.glyph) {
            RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
            RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
            RetainedGlyphStep::Complete => {
                cursor.advance(cursor.phase + 1);
                RetainedNodePaintStep::Pending
            }
        }
    } else if cursor.phase < 4 {
        cursor.advance(cursor.phase + 1);
        RetainedNodePaintStep::Pending
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}

/// 📑 Paints one step of a section node: the header band and title.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_section_node_step(
    section: &semio_framework_ui_declaration::ui::UiSectionNode,
    tree: &UiTree,
    id: NodeId,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let metrics = section_paint_metrics(tree, id, section, bounds, atlas);
        let Some((label, title)) = section.label.as_ref().zip(metrics.title) else {
            cursor.advance(1);
            return RetainedNodePaintStep::Pending;
        };
        match retained_text_node_step_weighted(label.as_str(), chrome_band_rect(bounds, title), semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE, theme.text, semio_framework_ui_text::TextWeight::Semibold, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(1);
                RetainedNodePaintStep::Pending
            }
            step => step,
        }
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}

/// 🧱 Paints one step of a group node: the group frame and title.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_group_node_step(
    group: &semio_framework_ui_declaration::ui::UiGroupNode,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| {
            if let Some(icons) = icons {
                push_icon(draw, icons, if group.default_open.unwrap_or(true) { "chevron-down" } else { "chevron-right" }, bounds.x, bounds.y, ICON_TINY, theme.text_element);
            }
        });
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 1 {
        match retained_text_node_step(group.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(2);
                RetainedNodePaintStep::Pending
            }
            step => step,
        }
    } else {
        retained_presence_step(draw, bounds, theme, presence)
    }
}
