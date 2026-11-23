//! 📝️ Text and structure nodes: the retained paint steps of the text, stack, separator, key value, field, section and group nodes. See [`super`].

use super::*;

/// 🪜️ Paints one step of a text node: the weighted body or emphasized run.
pub(super) fn paint_text_node_step(
    text: &semio_ui_declaration::ui::UiTextNode,
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
        let weight = semio_ui_text::TextWeight::of(emphasize);
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
    stack: &semio_ui_declaration::ui::UiStackNode,
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
    key_value: &semio_ui_declaration::ui::UiKeyValueNode,
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
    field: &semio_ui_declaration::ui::UiFieldNode,
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
        0 => metrics.label.map(|band| (field.label.as_str(), chrome_band_rect(bounds, band), semio_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.text, semio_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)),
        1 if field.required.unwrap_or(false) => metrics.label.map(|band| {
            let label_width = atlas.measure_text(field.label.as_str(), semio_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE).0;
            let mut rect = chrome_band_rect(bounds, band);
            rect.x += label_width + theme.gap_standard * 0.5;
            rect.w = (rect.w - label_width - theme.gap_standard * 0.5).max(0.0);
            ("*", rect, semio_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.error, semio_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)
        }),
        2 => field
            .description
            .as_deref()
            .zip(metrics.description)
            .map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.text_muted, semio_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
        3 => field.error.as_deref().zip(metrics.error).map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.error, semio_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
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
    section: &semio_ui_declaration::ui::UiSectionNode,
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
        match retained_text_node_step_weighted(label.as_str(), chrome_band_rect(bounds, title), semio_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE, theme.text, semio_ui_text::TextWeight::Semibold, atlas, draw, cursor) {
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
    group: &semio_ui_declaration::ui::UiGroupNode,
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

/// 🃏️ One card of a native stack as the painter sees it: its absolute frame rect, its chip texts and the chip widths.
struct CardGeometry {
    rect: Rect,
    title: String,
    footer: Option<String>,
    title_chip: f32,
    footer_chip: f32,
    disabled: bool,
    authored_hover: bool,
    routed_hover: bool,
}

/// 📐️ The cards of the stack `id`, with every chip clamped to its frame.
fn card_stack_geometry(tree: &UiTree, id: NodeId, bounds: Rect, atlas: &mut FontAtlas, theme: &Theme) -> Vec<CardGeometry> {
    let kids: Vec<_> = tree.children(id).filter_map(|child| {
        let node = tree.node(child)?;
        let UiNode::OverviewCard(card) = &node.spec.0 else { return None };
        Some((card, tree.accepted_layout(child)?, hover_owner(node.flags)))
    }).collect();
    let pad = theme.padding_standard;
    kids.iter().map(|(card, layout, routed_hover)| {
        let limit = layout.width.max(0.0);
        let chip = |text: &str, atlas: &mut FontAtlas| (atlas.measure_text(text, theme.font_size_body).0 + pad * 2.0).min(limit);
        CardGeometry {
            rect: Rect::new(bounds.x + layout.x, bounds.y + layout.y, layout.width, layout.height),
            title: card.title.as_str().to_string(),
            footer: card.footer.clone(),
            title_chip: chip(card.title.as_str(), atlas),
            footer_chip: card.footer.as_deref().map_or(0.0, |footer| chip(footer, atlas)),
            disabled: card.presence.state == UiState::Disabled,
            authored_hover: card.presence.hover,
            routed_hover: *routed_hover,
        }
    }).collect()
}

fn card_stack_cards(geometry: &[CardGeometry], row: f64) -> Vec<semio_ui_wgpu_light::card_stack::CardStackCard> {
    use semio_ui_wgpu_light::card_stack::CardStackCard;
    geometry.iter().map(|card| CardStackCard { height: f64::from(card.rect.h), row, top_chip: f64::from(card.title_chip), bottom_chip: if card.footer.is_some() { f64::from(card.footer_chip) } else { 0.0 } }).collect()
}

/// 🃏️ Paints one step of a native card stack: every card frame and chip inner edge as shared-border lines (each coincident line once), the chip glass inside each frame, then every chip text.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_card_stack_node_step(
    tree: &UiTree,
    id: NodeId,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    pointer: Option<(f32, f32)>,
) -> RetainedNodePaintStep {
    let geometry = card_stack_geometry(tree, id, bounds, atlas, theme);
    let row = f64::from(theme.control_height);
    let texts = geometry.len() * 2;
    if cursor.phase == 0 {
        let Some(first) = geometry.first() else {
            cursor.advance(cursor.phase + 1);
            return RetainedNodePaintStep::Pending;
        };
        let frames = semio_ui_wgpu_light::card_stack::card_stack_frames(f64::from(first.rect.x), f64::from(first.rect.y), f64::from(first.rect.w), &card_stack_cards(&geometry, row));
        let hovered = pointer.map(|(x, y)| frames.iter().position(|frame| semio_ui_wgpu_light::card_stack::card_frame_contains(frame, f64::from(x), f64::from(y))));
        let items = semio_ui_wgpu_light::card_stack::card_stack_paint_items(&frames);
        let result = retained_fixed_output_budgeted(draw, items, |draw| {
            use semio_ui_draw_contract::SharedBorderState;
            semio_ui_wgpu_light::card_stack::paint_card_stack(draw, &frames, theme.stroke_hairline, theme.glass(semio_ui_theme::Level::Window), |index| {
                let card = &geometry[index];
                let routed_hover = hovered.map_or(card.routed_hover, |owner| owner == Some(index));
                if !card.disabled && (card.authored_hover || routed_hover) {
                    (theme.border_emphasized, SharedBorderState::Hover)
                } else {
                    (theme.border_normal, SharedBorderState::Normal)
                }
            });
        });
        cursor.advance(cursor.phase + 1);
        return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
    }
    if usize::from(cursor.phase) <= texts {
        let slot = usize::from(cursor.phase) - 1;
        let card = &geometry[slot / 2];
        let pad = theme.padding_standard;
        let line = theme.font_size_body * 1.35;
        let (text, x, chip, y, color) = if slot % 2 == 0 {
            (Some(card.title.as_str()), card.rect.x, card.title_chip, card.rect.y, theme.text)
        } else {
            let y = card.rect.y + card.rect.h - theme.control_height;
            (card.footer.as_deref(), card.rect.x + card.rect.w - card.footer_chip, card.footer_chip, y, theme.text_muted)
        };
        let Some(text) = text else {
            cursor.advance(cursor.phase + 1);
            return RetainedNodePaintStep::Pending;
        };
        let rect = Rect::new(x + pad, y + (theme.control_height - line) * 0.5, (chip - pad * 2.0).max(0.0), line);
        return match retained_text_node_step_weighted(text, rect, theme.font_size_body, color, semio_ui_text::TextWeight::of(slot % 2 == 0), atlas, draw, cursor) {
            RetainedNodePaintStep::Complete => {
                cursor.advance(cursor.phase + 1);
                RetainedNodePaintStep::Pending
            }
            step => step,
        };
    }
    retained_presence_step(draw, bounds, theme, presence)
}
