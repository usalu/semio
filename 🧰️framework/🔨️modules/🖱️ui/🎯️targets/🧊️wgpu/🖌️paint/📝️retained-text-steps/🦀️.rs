//! 📝️ Retained text steps: the fixed output budgets, the select close route, the caret, presence and text node steps, the tree row lookups and the table header and cell steps. See [`super`].

use super::*;

// #region paint
pub(super) fn retained_fixed_output(draw: &mut DrawList, paint: impl FnOnce(&mut DrawList)) -> Result<(), semio_framework_ui_draw_contract::RetainedOutputError> {
    retained_fixed_output_budgeted(draw, RETAINED_NODE_FIXED_OUTPUT_ITEMS, paint)
}

pub(super) fn retained_fixed_output_budgeted(draw: &mut DrawList, items: usize, paint: impl FnOnce(&mut DrawList)) -> Result<(), semio_framework_ui_draw_contract::RetainedOutputError> {
    draw.begin_retained_output(items, RETAINED_NODE_FIXED_OUTPUT_BYTES)?;
    paint(draw);
    draw.finish_retained_output().map(|_| ())
}

pub(super) fn retained_select_close_route(draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> Result<(), semio_framework_ui_draw_contract::RetainedOutputError> {
    if !cursor.popup_route {
        return Ok(());
    }
    let result = retained_fixed_output(draw, |draw| {
        draw.pop_scissor();
        draw.end_glass_content();
    });
    if result.is_ok() {
        cursor.popup_route = false;
    }
    result
}

/// 🔢️ Measures one scalar per opportunity and centers the clipped stepper label.
pub(super) fn retained_stepper_text_step(value: &str, mut bounds: Rect, color: Rgba, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(cursor.measure_byte) {
        return RetainedNodePaintStep::Fault;
    }
    if let Some(scalar) = value[cursor.measure_byte..].chars().next() {
        let end = cursor.measure_byte + scalar.len_utf8();
        cursor.measure_width += atlas.pen_advance(value, cursor.measure_byte, theme.font_size_body);
        cursor.measure_byte = end;
        return RetainedNodePaintStep::Pending;
    }
    bounds.x += (bounds.w - cursor.measure_width).max(0.0) * 0.5;
    bounds.w = bounds.w.min(cursor.measure_width);
    bounds.y += (bounds.h - theme.font_size_body) * 0.5;
    match paint_retained_glyph_step_flowed(value, bounds, theme.font_size_body, color, RetainedTextFlow::Clip, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

#[derive(Clone, Copy)]
pub(super) enum RetainedCaretAlignment {
    Start,
    Center,
    End,
}

/// 🪟️ The line of a multi-line text a single-row field shows — the one holding `caret` (the first without one) — and
/// the caret's place inside it.
pub fn text_line_view(text: &str, caret: Option<usize>) -> (&str, usize) {
    let at = caret.filter(|caret| text.is_char_boundary(*caret)).unwrap_or(0).min(text.len());
    let start = text[..at].rfind('\n').map_or(0, |index| index + 1);
    let end = text[at..].find('\n').map_or(text.len(), |index| at + index);
    (&text[start..end], at - start)
}

pub(super) fn retained_caret_step(value: &str, caret: usize, bounds: Rect, size: f32, alignment: RetainedCaretAlignment, color: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(caret) || !value.is_char_boundary(cursor.measure_byte) {
        return RetainedNodePaintStep::Fault;
    }
    if let Some(scalar) = value[cursor.measure_byte..].chars().next() {
        let end = cursor.measure_byte + scalar.len_utf8();
        let width = atlas.pen_advance(value, cursor.measure_byte, size);
        cursor.measure_width += width;
        if end <= caret {
            cursor.caret_width += width;
        }
        cursor.measure_byte = end;
        return RetainedNodePaintStep::Pending;
    }
    let origin = match alignment {
        RetainedCaretAlignment::Start => bounds.x,
        RetainedCaretAlignment::Center => bounds.x + (bounds.w - cursor.measure_width).max(0.0) * 0.5,
        RetainedCaretAlignment::End => bounds.x + (bounds.w - cursor.measure_width).max(0.0),
    };
    let height = (size * 1.2).min(bounds.h);
    let x = (origin + cursor.caret_width).min(bounds.x + bounds.w - 1.0);
    if retained_fixed_output(draw, |draw| draw.push_solid([x, bounds.y + (bounds.h - height) * 0.5, 1.0, height], color)).is_err() {
        RetainedNodePaintStep::Fault
    } else {
        RetainedNodePaintStep::Complete
    }
}

pub(super) fn retained_presence_step(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: &UiPresence) -> RetainedNodePaintStep {
    retained_presence_step_with_selection(draw, bounds, theme, presence, presence.selected)
}

pub(super) fn retained_presence_step_with_selection(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: &UiPresence, selected: bool) -> RetainedNodePaintStep {
    if retained_fixed_output(draw, |draw| presence_overlay(draw, bounds, theme, presence, selected)).is_err() {
        RetainedNodePaintStep::Fault
    } else {
        RetainedNodePaintStep::Complete
    }
}

pub(super) fn retained_text_node_step(value: &str, bounds: Rect, size: f32, color: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    retained_text_node_step_weighted(value, bounds, size, color, semio_framework_ui_text::TextWeight::Regular, atlas, draw, cursor)
}

/// 🅰️ [`retained_text_node_step`] at an explicit weight — the production seam for React's
/// `font-semibold`. `TextWeight::of(emphasize)` is the whole mapping, exactly as `TextView`
/// (`🗣️Interpreter/🟦️.tsx:1168`) spells it; every other retained run stays `Regular`, which is the
/// byte-identical behaviour this function had before it grew the parameter.
#[allow(clippy::too_many_arguments, reason = "one arg per retained text input; a struct here is a T2 restructure of every retained node call site")]
pub(super) fn retained_text_node_step_weighted(value: &str, bounds: Rect, size: f32, color: Rgba, weight: semio_framework_ui_text::TextWeight, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    match paint_retained_glyph_step_weighted(value, bounds, size, color, RetainedTextFlow::Wrap, weight, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

pub fn chrome_band_rect(bounds: Rect, band: semio_framework_ui_wgpu_layout::flex::ChromeBand) -> Rect {
    Rect::new(bounds.x, bounds.y + band.y, bounds.w, band.height)
}

pub fn section_paint_metrics(tree: &UiTree, id: NodeId, section: &UiSectionNode, bounds: Rect, atlas: &mut FontAtlas) -> semio_framework_ui_wgpu_layout::flex::SectionFieldChromeMetrics {
    let title_height = section.label.as_ref().map(|label| {
        let accepted = semio_framework_ui_wgpu_mounted_layout::retained_section_title_height(tree, id);
        accepted.max(atlas.measure_text_wrapped(label.as_str(), bounds.w.max(1.0), semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE).1)
    });
    semio_framework_ui_wgpu_layout::flex::section_chrome_metrics(title_height, 0.0)
}

pub fn field_paint_metrics(tree: &UiTree, id: NodeId, field: &UiFieldNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas) -> semio_framework_ui_wgpu_layout::flex::SectionFieldChromeMetrics {
    let control = tree.node(id).and_then(|node| node.first_child).and_then(|child| tree.accepted_layout(child));
    let description_height = field.description.as_ref().map(|description| {
        control
            .map(|control| (control.y - semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_LINE_HEIGHT - theme.gap_standard * 2.0).max(semio_framework_ui_text::line_height(semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE)))
            .unwrap_or_else(|| atlas.measure_text_wrapped(description, bounds.w.max(1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE).1)
    });
    let error_height = field.error.as_ref().map(|error| atlas.measure_text_wrapped(error, bounds.w.max(1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE).1);
    semio_framework_ui_wgpu_layout::flex::field_chrome_metrics(description_height, control.map_or(0.0, |control| control.height), error_height, theme.gap_standard)
}

fn retained_tree_index(len: usize, index: usize, reversed: bool) -> Option<usize> {
    if index >= len {
        return None;
    }
    Some(if reversed { len - index - 1 } else { index })
}

/// ↔️ Measures one scalar per opportunity before aligning a tree label to its inline end.
pub(super) fn retained_tree_text_step(value: &str, mut bounds: Rect, size: f32, color: Rgba, inline: ui_contract::FlowInline, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(cursor.measure_byte) {
        return RetainedNodePaintStep::Fault;
    }
    if inline.is_rtl() {
        if let Some(scalar) = value[cursor.measure_byte..].chars().next() {
            let end = cursor.measure_byte + scalar.len_utf8();
            cursor.measure_width += atlas.pen_advance(value, cursor.measure_byte, size);
            cursor.measure_byte = end;
            return RetainedNodePaintStep::Pending;
        }
        bounds.x += (bounds.w - cursor.measure_width).max(0.0);
        bounds.w = bounds.w.min(cursor.measure_width);
    }
    bounds.y += (bounds.h - size) * 0.5;
    match paint_retained_glyph_step_flowed(value, bounds, size, color, RetainedTextFlow::Clip, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

pub(super) fn retained_text_end_step(value: &str, mut bounds: Rect, size: f32, color: Rgba, inline: ui_contract::FlowInline, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(cursor.measure_byte) {
        return RetainedNodePaintStep::Fault;
    }
    if !inline.is_rtl() {
        if let Some(scalar) = value[cursor.measure_byte..].chars().next() {
            let end = cursor.measure_byte + scalar.len_utf8();
            cursor.measure_width += atlas.pen_advance(value, cursor.measure_byte, size);
            cursor.measure_byte = end;
            return RetainedNodePaintStep::Pending;
        }
        bounds.x += (bounds.w - cursor.measure_width).max(0.0);
        bounds.w = bounds.w.min(cursor.measure_width);
    }
    bounds.y += (bounds.h - size) * 0.5;
    match paint_retained_glyph_step_flowed(value, bounds, size, color, RetainedTextFlow::Clip, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

pub(super) fn retained_tree_section_at<'a>(tree: &'a UiTreeNode, index: usize, reversed: bool) -> Option<&'a semio_framework_ui_declaration::ui::UiTreeSectionNode> {
    tree.sections.get(retained_tree_index(tree.sections.len(), index, reversed)?)
}

pub(super) fn retained_tree_item_at<'a>(tree: &'a UiTreeNode, cursor: &RetainedNodePaintCursor, reversed: bool) -> Option<&'a UiTreeItemNode> {
    if cursor.depth == 0 {
        return None;
    }
    let section = retained_tree_section_at(tree, cursor.section, reversed)?;
    let mut item = section.items.get(retained_tree_index(section.items.len(), cursor.path[0], reversed)?)?;
    for level in 1..cursor.depth {
        let items = item.items.as_ref()?;
        item = items.get(retained_tree_index(items.len(), cursor.path[level], reversed)?)?;
    }
    Some(item)
}

fn retained_tree_item_node_at(retained: &UiTree, tree_id: NodeId, tree: &UiTreeNode, cursor: &RetainedNodePaintCursor, reversed: bool) -> Option<NodeId> {
    if cursor.depth == 0 {
        return None;
    }
    let section = retained_tree_section_at(tree, cursor.section, reversed)?;
    let mut parent = if semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id).is_some() { tree_id } else { retained.explicit_child(tree_id, &section.id)? };
    let mut items = section.items.as_slice();
    for level in 0..cursor.depth {
        let item = items.get(retained_tree_index(items.len(), cursor.path[level], reversed)?)?;
        parent = retained.explicit_child(parent, &item.id)?;
        items = item.items.as_deref().unwrap_or(&[]);
    }
    Some(parent)
}

pub(super) fn retained_tree_item_row(retained: &UiTree, tree_id: NodeId, tree: &UiTreeNode, bounds: Rect, cursor: &RetainedNodePaintCursor, metrics: &TreeRowMetrics, reversed: bool) -> Option<(Rect, bool)> {
    let item = retained_tree_item_at(tree, cursor, reversed)?;
    let row_id = retained_tree_item_node_at(retained, tree_id, tree, cursor, reversed)?;
    let open = retained.disclosure_open(row_id).unwrap_or(item.default_open.unwrap_or(false)) && item.items.as_deref().is_some_and(|items| !items.is_empty());
    let height = semio_framework_ui_wgpu_mounted_layout::live_tree_item_height(retained, row_id, item, metrics, cursor.depth - 1);
    let row_height = metrics.for_item(item).row_height;
    let y = if reversed && open { cursor.row_y + height - row_height } else { cursor.row_y };
    Some((Rect::new(bounds.x, y, bounds.w, row_height), open))
}

pub(super) fn retained_tree_sibling_count(tree: &UiTreeNode, cursor: &RetainedNodePaintCursor, reversed: bool) -> Option<usize> {
    if cursor.depth == 0 {
        return None;
    }
    if cursor.depth == 1 {
        return Some(retained_tree_section_at(tree, cursor.section, reversed)?.items.len());
    }
    let section = retained_tree_section_at(tree, cursor.section, reversed)?;
    let mut parent = section.items.get(retained_tree_index(section.items.len(), cursor.path[0], reversed)?)?;
    for level in 1..cursor.depth - 1 {
        let items = parent.items.as_ref()?;
        parent = items.get(retained_tree_index(items.len(), cursor.path[level], reversed)?)?;
    }
    parent.items.as_ref().map(Vec::len)
}

pub(super) fn retained_control_is_bounded(control: &UiControlNode) -> bool {
    match control {
        UiControlNode::Select(node) => node.items.len() <= RETAINED_NODE_COLLECTION_ITEMS,
        UiControlNode::KeyValue(node) => node.entries.len() <= RETAINED_NODE_COLLECTION_ITEMS,
        UiControlNode::Input(_) | UiControlNode::Toggle(_) | UiControlNode::Button(_) | UiControlNode::Slider(_) | UiControlNode::NumberStepper(_) | UiControlNode::Ring(_) | UiControlNode::IconSelect(_) => true,
    }
}

pub(super) fn retained_tree_content_height(retained: &UiTree, tree_id: NodeId, tree: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    semio_framework_ui_wgpu_mounted_layout::retained_tree_height(retained, tree_id, tree, metrics)
}

/// 🔽️ Translates a Select's window-local popup to the exact trigger bounds this paint frame uses.
pub(super) fn retained_select_popup_at_bounds(tree: &UiTree, id: NodeId, bounds: Rect, popup: semio_framework_ui_select_behavior::SelectPopupGeometry) -> Option<semio_framework_ui_select_behavior::SelectPopupGeometry> {
    let local = tree.absolute_rect(id)?;
    Some(popup.translated(bounds.x - local.x, bounds.y - local.y))
}

/// 📊️ Advances a table's column header by one glyph (`cursor.item` is the column): each materialised column label over its
/// `layout::table_column_rect`, then `TableProps::actions_label` over the trailing actions column, in the band
/// `LayoutNodeKind::Tree::header` reserves ahead of the rows; then the rows (phase 2).
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
pub(super) fn retained_table_header_step(
    retained: &UiTree,
    tree_id: NodeId,
    tree: &UiTreeNode,
    bounds: Rect,
    theme: &Theme,
    reversed: bool,
    inline: ui_contract::FlowInline,
    metrics: &TreeRowMetrics,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    let Some(table) = semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id) else { return RetainedNodePaintStep::Fault };
    let header = metrics.header_height;
    let actions = semio_framework_ui_wgpu_mounted_layout::table_actions_width_of(tree, metrics);
    let columns = table.columns.len();
    let (label, rect) = match (table.columns.get(cursor.item), table.actions_label.as_ref()) {
        (Some(label), _) => (label, semio_framework_ui_wgpu_layout::layout::table_column_rect(bounds.w, header, cursor.item, columns, actions, metrics)),
        (None, Some(label)) if cursor.item == columns && actions > 0.0 => (label, Rect::new(if inline.is_rtl() { 0.0 } else { bounds.w - actions }, 0.0, actions, header)),
        _ => {
            if !reversed {
                cursor.row_y += header;
            }
            cursor.advance(2);
            return RetainedNodePaintStep::Pending;
        }
    };
    let y = if reversed { cursor.row_y + retained_tree_content_height(retained, tree_id, tree, metrics) - header } else { cursor.row_y };
    match retained_tree_text_step(label.0.as_str(), Rect::new(bounds.x + rect.x, y, rect.w, header), theme.font_size_small, theme.text_muted, inline, atlas, draw, cursor) {
        RetainedNodePaintStep::Complete => {
            cursor.next_part();
            RetainedNodePaintStep::Pending
        }
        step => step,
    }
}

/// 🏓️ Advances one table row's cells by one glyph (`cursor.item` is the column): column `i` paints `TableRowProps::cells[i]` in
/// its `layout::table_column_rect` unless the row materialised a child for it — an editable row's input, paged draft or
/// read-only surface, which paints itself in the same column (`LayoutNodeKind::TableRow`); then the row's trailing actions.
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
pub(super) fn retained_table_cells_step(
    retained: &UiTree,
    tree_id: NodeId,
    tree: &UiTreeNode,
    item: &UiTreeItemNode,
    row: Rect,
    font_size: f32,
    theme: &Theme,
    inline: ui_contract::FlowInline,
    metrics: &TreeRowMetrics,
    reversed: bool,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    let (Some(table), Some(row_id)) = (semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id), retained_tree_item_node_at(retained, tree_id, tree, cursor, reversed)) else {
        return RetainedNodePaintStep::Fault;
    };
    let columns = table.columns.len();
    let index = cursor.item;
    if index >= columns {
        cursor.advance(5);
        return RetainedNodePaintStep::Pending;
    }
    let materialised = retained.children(row_id).count();
    let Some(text) = semio_framework_ui_wgpu_mounted_layout::document_table_row(retained, row_id).and_then(|props| props.cells.get(index)).filter(|_| index >= materialised) else {
        cursor.next_part();
        return RetainedNodePaintStep::Pending;
    };
    let column = semio_framework_ui_wgpu_layout::layout::table_column_rect(row.w, row.h, index, columns, semio_framework_ui_wgpu_mounted_layout::table_actions_width_of(tree, metrics), metrics);
    let color = foreground_on_fill(theme, theme.text_element, item.presence.selected, item.presence.state == UiState::Previewed || item.presence.hover);
    let color = if item.presence.state == UiState::Disabled { color.with_alpha(color.a * 0.5) } else { color };
    match retained_tree_text_step(text.as_str(), Rect::new(row.x + column.x, row.y + column.y, column.w, column.h), font_size, color, inline, atlas, draw, cursor) {
        RetainedNodePaintStep::Complete => {
            cursor.next_part();
            RetainedNodePaintStep::Pending
        }
        step => step,
    }
}

/// 🌲️ How far the paint cursor moves past one row: a table row's whole laid-out height (`mounted_layout::live_tree_item_height`
/// — a row whose cell hosts a draft or read-only surface is taller), a tree row's own band (its nested rows advance it themselves).
pub(super) fn retained_tree_row_advance(retained: &UiTree, tree_id: NodeId, tree: &UiTreeNode, item: &UiTreeItemNode, cursor: &RetainedNodePaintCursor, metrics: &TreeRowMetrics, reversed: bool) -> f32 {
    let band = metrics.for_item(item).row_height;
    if semio_framework_ui_wgpu_mounted_layout::document_table(retained, tree_id).is_none() {
        return band;
    }
    retained_tree_item_node_at(retained, tree_id, tree, cursor, reversed).map_or(band, |row| semio_framework_ui_wgpu_mounted_layout::live_tree_item_height(retained, row, item, metrics, 0))
}
// #endregion paint
