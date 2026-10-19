//! 🖼️ Media and slot nodes: the retained paint steps of the image, component scene and external slot nodes. See [`super`].

use super::*;

/// 🖼 Paints one step of an image node: the decoded raster, the host raster or the placeholder chrome.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn paint_image_node_step(
    image: &semio_framework_ui_declaration::ui::UiImageNode,
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
    } else if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 1 {
        let label = image.alt.as_ref().map_or(image.id.as_str(), Label::as_str);
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
    scene: &semio_framework_ui_declaration::ui::UiComponentSceneNode,
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

/// 🔌 Paints one step of an external slot node: the slot placeholder chrome.
pub(super) fn paint_external_slot_node_step(
    slot: &semio_framework_ui_declaration::ui::UiExternalSlotNode,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
    theme: &Theme,
    bounds: Rect,
    presence: &UiPresence,
) -> RetainedNodePaintStep {
    if cursor.phase == 0 {
        let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
        cursor.advance(1);
        if result.is_err() {
            RetainedNodePaintStep::Fault
        } else {
            RetainedNodePaintStep::Pending
        }
    } else if cursor.phase == 1 {
        match retained_text_node_step(slot.host_status.as_deref().unwrap_or(slot.body_key.as_str()), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
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
