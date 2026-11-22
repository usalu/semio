//! 🕸️ Media and slot nodes: the retained paint steps of the image, component scene and external slot nodes. See [`super`].

use super::*;

/// 🖼 Paints one step of an image node: the decoded raster, the host raster or the placeholder chrome.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_image_node_step(
    image: &semio_ui_declaration::ui::UiImageNode,
    atlas: &mut FontAtlas,
    has_scene_host: bool,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    let decoded = if has_scene_host { None } else { matches!(admit_ui_image(image.src.as_str()), UiImageAdmission::Ready).then(|| ui_image_natural_size(image.src.as_str())).flatten() };
    if has_scene_host {
        retained_presence_step(draw, bounds, theme, presence)
    } else if let Some((natural_w, natural_h)) = decoded {
        if cursor.phase == 0 {
            let content = ui_image_content_rect(bounds, natural_w, natural_h);
            let result = retained_fixed_output(draw, |draw| draw.push_raster_quad(image.src.as_str(), [content.x, content.y, content.w, content.h], [0.0, 0.0, 1.0, 1.0], 1.0));
            cursor.advance(2);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        } else {
            retained_presence_step(draw, bounds, theme, presence)
        }
    } else if cursor.phase == 0 && image.source_page.is_some() {
        let track = [bounds.x, bounds.y + (bounds.h - SIZE_TINY) * 0.5, bounds.w, SIZE_TINY];
        let result = retained_fixed_output(draw, |draw| draw.push_rounded(track, theme.muted, theme.border_radius));
        cursor.advance(2);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 1 {
        let label = image.alt.as_ref().map_or("", Label::as_str);
        match retained_text_node_step(label, bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
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

/// 🎬 Paints one step of a component scene node: the scene host placeholder.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_component_scene_node_step(
    scene: &semio_ui_declaration::ui::UiComponentSceneNode,
    atlas: &mut FontAtlas,
    has_scene_host: bool,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if has_scene_host {
        retained_presence_step(draw, bounds, theme, presence)
    } else if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 1 {
        match retained_text_node_step(scene.surface_id.as_str(), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
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

/// 🔌 Paints one step of an external slot node. The reserved media transport owns its rect (the host paints over the panel chrome and any status text) and any extension with a registered host (`register_extension_host`) shows what its host rendered; every other extension has no host, so it shows what React's `ExtensionView` shows: the muted `text-xs` line `Extension unavailable: <extension>`, on no fill.
pub(super) fn paint_external_slot_node_step(
    slot: &semio_ui_declaration::ui::UiExternalSlotNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
    locale: Option<semio_ui_locale::Locale>,
    window: u64,
    document_node: u64,
) -> RetainedNodePaintStep {
    let hosted = slot.host_status.is_some();
    if cursor.phase <= 1 && hosted && !semio_ui_retained_tree::reconcile::extension_host_rows(&slot.body_key, &slot.params_json).is_empty() {
        cursor.advance(2);
        return RetainedNodePaintStep::Pending;
    }
    let panel = slot.body_key == semio_ui_retained_tree::reconcile::MEDIA_TRANSPORT_EXTENSION_ID;
    if cursor.phase == 0 && panel {
        let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if let Some(overlay) = panel.then(|| semio_ui_wgpu_paint_overlay::media_overlay_for(window, document_node)).flatten().filter(|_| cursor.phase >= 1) {
        paint_media_overlay_step(&overlay, atlas, draw, cursor, theme, bounds, presence)
    } else if cursor.phase <= 1 {
        let unavailable = format!("{}: {}", super::ui_common_label(locale, |labels| labels.extension_unavailable), slot.body_key);
        let text = if hosted { slot.host_status.as_deref().unwrap_or(slot.body_key.as_str()) } else { unavailable.as_str() };
        match retained_text_node_step(text, bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
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

/// 🎞️ One step of a native media slot over its panel: the video picture first, then each transport row's box and caption in reading order.
fn paint_media_overlay_step(
    overlay: &semio_ui_wgpu_paint_overlay::MediaOverlay,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    let phase = cursor.phase;
    if phase == 1 {
        let result = overlay.quad.as_ref().map_or(Ok(()), |quad| retained_fixed_output(draw, |draw| draw.push_raster_quad(quad.raster_key.as_str(), quad.rect, [0.0, 0.0, 1.0, 1.0], 1.0)));
        cursor.advance(2);
        return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
    }
    let Some(row) = overlay.rows.get(usize::from((phase - 2) / 2)) else { return retained_presence_step(draw, bounds, theme, presence) };
    if phase % 2 == 0 {
        let result = retained_fixed_output(draw, |draw| semio_ui_wgpu_paint_overlay::paint_media_overlay_row(draw, row, theme));
        cursor.advance(phase + 1);
        return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
    }
    let color = if row.enabled { theme.text } else { theme.text_muted };
    let area = Rect::new(row.rect[0], row.rect[1], row.rect[2], row.rect[3]);
    match retained_text_node_step(row.text.as_str(), area, theme.font_size_small, color, atlas, draw, cursor) {
        RetainedNodePaintStep::Complete => {
            cursor.advance(phase + 1);
            RetainedNodePaintStep::Pending
        }
        step => step,
    }
}
