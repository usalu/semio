//! 📋️ Ink clipboard, camera and wheel: the clipboard copy and paste, the pointer cancel, the document and value checks, the camera writers and the wheel handling. See [`super`].

use super::*;

//#region InkCanvas
//#region InkCanvasState
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InkClipboardContentKind {
    Text,
    ImageDataUrl,
}

fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    for block in blocks {
        if ink_item_id(block) == id {
            return Some(block);
        }
        if ink_item_kind(block) == "group" {
            if let Some(found) = block.get("children").and_then(Value::as_array).and_then(|children| find_ink_item(children, id)) {
                return Some(found);
            }
        }
    }
    None
}

pub fn ink_clipboard_copy_text(scene: &UiComponentSceneNode) -> Result<Option<String>, semio_framework_ui_action::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else { return Ok(None) };
    if ink.document_json.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY || ink.selection_json.len() > INK_EVENT_JSON_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    }
    let document: InkDocumentJson = serde_json::from_str(&ink.document_json).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let selection: Vec<String> = serde_json::from_str(&ink.selection_json).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    if selection.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
    }
    let blocks: Vec<Value> = selection.iter().filter_map(|id| find_ink_item(&document.blocks, id).cloned()).collect();
    if blocks.is_empty() {
        return Ok(None);
    }
    let text = serde_json::to_string(&json!({ "schema": "ink.clipboard", "blocks": blocks })).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    if text.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    }
    Ok(Some(text))
}

pub fn ink_clipboard_target(scene: &UiComponentSceneNode, inner: Rect) -> Result<(f64, f64), semio_framework_ui_action::BoundedActionFault> {
    let document = checked_ink_document(scene)?.ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    let camera = ink_camera_checked(scene)?;
    let (x, y) = ink_screen_to_world(camera, inner, inner.x + inner.w / 2.0, inner.y + inner.h / 2.0);
    Ok(ink_maybe_snap_fields(document.snap_enabled, document.snap_grid_spacing, x, y))
}

fn reidentify_ink_clipboard_item(block: &Value) -> Result<Value, semio_framework_ui_action::BoundedActionFault> {
    let object = block.as_object().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    let kind = object.get("kind").and_then(Value::as_str).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    if kind.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
    }
    let mut clone = block.clone();
    let clone_object = clone.as_object_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    clone_object.insert("id".into(), Value::String(create_ink_host_id(kind)));
    if kind == "group" {
        let children = object.get("children").and_then(Value::as_array).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        if children.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
        }
        let children = children.iter().map(reidentify_ink_clipboard_item).collect::<Result<Vec<_>, _>>()?;
        clone_object.insert("children".into(), Value::Array(children));
    }
    Ok(clone)
}

fn write_ink_clipboard_events_action(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, events: &InkEventJsonPages, selected_ids: &[String]) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    if selected_ids.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
    }
    let events_json = events.as_str()?;
    let mut bytes = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "inkApplyEvents", "surfaceId", &scene.surface_id, "eventsJson", events_json, "selectIds"])?;
    for id in selected_ids {
        bytes = bytes.checked_add(semio_framework_ui_action::checked_action_string_bytes(&[id])?).filter(|value| *value <= semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
    }
    let mut reservation = input.reserve_action(&scene.controller_id, "inkApplyEvents", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("eventsJson"), events_json)?;
    builder.begin_array(Some("selectIds"))?;
    for id in selected_ids {
        builder.string(None, id)?;
    }
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish()
}

fn ink_clipboard_image_events(content: &str, target: (f64, f64)) -> Result<(InkEventJsonPages, Vec<String>), semio_framework_ui_action::BoundedActionFault> {
    let (mime, data) = if content.trim_start().starts_with("<svg") {
        ("image/svg+xml", content.trim())
    } else {
        let Some(header) = content.strip_prefix("data:").and_then(|data| data.split_once(',').map(|parts| parts.0)) else {
            return Err(semio_framework_ui_action::BoundedActionFault::Structure);
        };
        let mime = header.split(';').next().unwrap_or_default();
        if !mime.starts_with("image/") {
            return Err(semio_framework_ui_action::BoundedActionFault::Structure);
        }
        (mime, content)
    };
    if data.len() > INK_EVENT_JSON_BYTE_CAPACITY / 2 {
        return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    }
    let key = format!("asset-{}", create_ink_host_id("image"));
    let mut block = create_ink_item("image", target.0 - 120.0, target.1 - 80.0);
    let id = ink_item_id(&block).to_owned();
    block.as_object_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.insert("imageKey".into(), Value::String(key.clone()));
    let mut events = InkEventJsonPages::default();
    events.push(&json!({ "operation": "putAsset", "key": key, "asset": { "mime": mime, "data": data } }))?;
    events.push(&json!({ "operation": "addBlock", "block": block }))?;
    events.seal()?;
    Ok((events, vec![id]))
}

pub fn ink_clipboard_paste_into(scene: &UiComponentSceneNode, target: (f64, f64), kind: InkClipboardContentKind, content: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    if content.is_empty() || content.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
        return if content.is_empty() { Ok(false) } else { Err(semio_framework_ui_action::BoundedActionFault::ByteCredits) };
    }
    let (mut events, selected_ids) = if kind == InkClipboardContentKind::ImageDataUrl || content.trim_start().starts_with("<svg") {
        ink_clipboard_image_events(content, target)?
    } else if let Ok(payload) = serde_json::from_str::<Value>(content) {
        if payload.get("schema").and_then(Value::as_str) == Some("ink.clipboard") {
            let blocks = payload.get("blocks").and_then(Value::as_array).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            if blocks.len() > INK_INTERACTION_ITEM_CAPACITY {
                return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
            }
            let mut events = InkEventJsonPages::default();
            let mut selected_ids = Vec::with_capacity(blocks.len());
            for block in blocks {
                let mut block = reidentify_ink_clipboard_item(block)?;
                let x = ink_item_num(&block, "x") + target.0;
                let y = ink_item_num(&block, "y") + target.1;
                block = ink_item_with_position(&block, x, y);
                let mut nodes = 0;
                validate_ink_value(&block, 0, &mut nodes)?;
                selected_ids.push(ink_item_id(&block).to_owned());
                events.push(&json!({ "operation": "addBlock", "block": block }))?;
            }
            if selected_ids.is_empty() {
                return Ok(false);
            }
            (events, selected_ids)
        } else {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                return Ok(false);
            }
            let mut block = create_ink_item("text", target.0, target.1);
            let id = ink_item_id(&block).to_owned();
            let paragraphs = trimmed.lines().map(|line| json!({ "runs": [{ "text": line }] })).collect();
            block.as_object_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.insert("paragraphs".into(), Value::Array(paragraphs));
            let mut events = InkEventJsonPages::default();
            events.push(&json!({ "operation": "addBlock", "block": block }))?;
            (events, vec![id])
        }
    } else {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(false);
        }
        let mut block = create_ink_item("text", target.0, target.1);
        let id = ink_item_id(&block).to_owned();
        let paragraphs = trimmed.lines().map(|line| json!({ "runs": [{ "text": line }] })).collect();
        block.as_object_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.insert("paragraphs".into(), Value::Array(paragraphs));
        let mut events = InkEventJsonPages::default();
        events.push(&json!({ "operation": "addBlock", "block": block }))?;
        (events, vec![id])
    };
    events.seal()?;
    write_ink_clipboard_events_action(input, scene, &events, &selected_ids)?;
    Ok(true)
}

pub fn ink_edit_paste_into(scene: &UiComponentSceneNode, text: &str) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    if text.len() > INK_EVENT_JSON_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    }
    let mut state = scene_state(&scene.host_id);
    let Some(mut edit) = state.ink_edit.take() else { return Ok(false) };
    if edit.replace_on_input {
        edit.draft.clear();
        edit.replace_on_input = false;
    }
    edit.draft.push_str(text);
    mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
    Ok(true)
}

pub fn ink_pointer_cancel_into(surface_id: &str) {
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.pointer_was_down = false;
        state.ink_gesture_open = false;
        state.ink_marquee_points.clear();
        state.ink_overrides.clear();
    });
}

#[cfg(any(test, feature = "testkit"))]
pub fn ink_pointer_state_is_clear(surface_id: &str) -> bool {
    let state = scene_state(surface_id);
    !state.pointer_was_down && state.drag.is_none() && !state.ink_gesture_open && state.ink_marquee_points.is_empty() && state.ink_overrides.is_empty()
}

pub(super) fn checked_ink_document(scene: &UiComponentSceneNode) -> Result<Option<InkInteractionDocument>, semio_framework_ui_action::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(None);
    };
    if ink.document_json.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY || ink.selection_json.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    }
    let document = serde_json::from_str::<InkDocumentJson>(&ink.document_json).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let mut nodes = 0usize;
    for block in &document.blocks {
        validate_ink_value(block, 0, &mut nodes)?;
    }
    for (key, asset) in &document.assets {
        if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
        }
        validate_ink_value(asset, 0, &mut nodes)?;
    }
    if document.blocks.len() > INK_INTERACTION_ITEM_CAPACITY || document.assets.len() > INK_INTERACTION_ITEM_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
    }
    let source = ink.document_json.clone();
    let mut spans = Box::new(std::array::from_fn(|_| None));
    let mut span_len = 0usize;
    collect_ink_document_block_spans(source.as_bytes(), &mut spans, &mut span_len)?;
    Ok(Some(InkInteractionDocument {
        source,
        spans,
        span_len,
        schema: document.schema,
        id: document.id,
        camera: document.camera,
        active_utility: document.active_utility,
        snap_enabled: document.snap_enabled,
        snap_grid_spacing: document.snap_grid_spacing,
        eraser_radius: document.eraser_radius,
    }))
}

pub fn validate_ink_value(value: &Value, depth: usize, nodes: &mut usize) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    if depth > semio_framework_ui_action::ACTION_DEPTH_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::DepthCredits);
    }
    *nodes = nodes.checked_add(1).ok_or(semio_framework_ui_action::BoundedActionFault::NodeCredits)?;
    if *nodes > INK_INTERACTION_ITEM_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::NodeCredits);
    }
    match value {
        Value::Array(values) => {
            for value in values {
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::Object(entries) => {
            for (key, value) in entries {
                if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                    return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
                }
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::String(value) if value.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY => return Err(semio_framework_ui_action::BoundedActionFault::StringCredits),
        _ => {}
    }
    Ok(())
}

fn ink_camera_checked(scene: &UiComponentSceneNode) -> Result<InkCameraF, semio_framework_ui_action::BoundedActionFault> {
    if let Some(camera) = SCENE_STATE.with(|cell| cell.borrow().get(&scene.host_id).and_then(|state| state.ink_camera)) {
        return Ok(InkCameraF { x: camera.0, y: camera.1, zoom: camera.2 });
    }
    Ok(checked_ink_document(scene)?.map(|document| InkCameraF::from(document.camera)).unwrap_or_default())
}

pub(super) fn write_ink_camera_action(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, camera: InkCameraF, mutate: impl FnOnce()) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "setCamera", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.begin_object(Some("camera"))?;
    builder.number(Some("x"), camera.x)?;
    builder.number(Some("y"), camera.y)?;
    builder.number(Some("zoom"), camera.zoom)?;
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

pub fn ink_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(false);
    };
    if ink.view_mode == "navigator" || !inner.contains(x, y) {
        return Ok(false);
    }
    let camera = ink_camera_checked(scene)?;
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let next = InkCameraF { x: (x - inner.x) as f64 - wx * next_zoom, y: (y - inner.y) as f64 - wy * next_zoom, zoom: next_zoom };
    write_ink_camera_action(input, scene, next, || {
        mutate_scene_state(&scene.host_id, |state| {
            state.ink_camera = Some((next.x, next.y, next.zoom));
        });
    })?;
    Ok(true)
}
//#endregion InkCanvasState
//#endregion InkCanvas
