//! 📰️ Event feed and graph timeline scenes.

use super::*;

//#region EventFeed
/// 🪶️ Mirrors a `SurfaceKind::EventFeed` entry (`{id, timestampMs, iconId, title, detail?, tone?}`,
/// `ui_wgpu::wgpu::EventFeedScene`'s doc comment / `EventFeedEntry` in `framework/core/js/index.ts`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFeedEntryJson {
    pub id: String,
    #[serde(default)]
    pub timestamp_ms: i64,
    #[serde(default)]
    pub icon_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub tone: Option<String>,
}

pub fn host_temporal_now_ms() -> i64 {
    crate::host_base::app_now_ms() as i64
}

pub fn event_feed_time_id(timestamp_ms: i64) -> String {
    format!("event-feed:{timestamp_ms}")
}

pub fn event_feed_time_request(entries: &[EventFeedEntryJson]) -> ui_contract::HostTemporalFormatRequestV1 {
    let mut values = Vec::new();
    for timestamp_ms in entries.iter().map(|entry| entry.timestamp_ms).filter(|timestamp_ms| (ui_contract::HOST_TEMPORAL_FORMAT_MIN_TIMESTAMP_MS..=ui_contract::HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS).contains(timestamp_ms)) {
        let id = event_feed_time_id(timestamp_ms);
        if !values.iter().any(|value: &ui_contract::HostTemporalValueV1| value.id == id) {
            values.push(ui_contract::HostTemporalValueV1 { id, source: ui_contract::HostTemporalSourceV1::EpochMs { timestamp_ms }, format: ui_contract::HostTemporalFormatV1::Time });
        }
        if values.len() == ui_contract::HOST_TEMPORAL_FORMAT_MAX_VALUES {
            break;
        }
    }
    ui_contract::HostTemporalFormatRequestV1 { now_ms: host_temporal_now_ms(), values }
}

pub fn publish_host_temporal_reply(token: AdmittedSurfaceToken, generation: u64, request: &ui_contract::HostTemporalFormatRequestV1, reply: ui_contract::HostTemporalFormatReplyV1) -> bool {
    SCENE_STATE.with(|cell| cell.borrow_mut().get_token_mut(token).is_some_and(|state| state.host_temporal.publish(generation, request, reply)))
}

fn refuse_host_temporal_reply(token: AdmittedSurfaceToken, generation: u64) {
    SCENE_STATE.with(|cell| {
        if let Some(state) = cell.borrow_mut().get_token_mut(token) {
            state.host_temporal.refuse(generation);
        }
    });
}

pub fn host_temporal_reply(host_id: &str, request: &ui_contract::HostTemporalFormatRequestV1) -> Option<ui_contract::HostTemporalFormatReplyV1> {
    if !request.validate() {
        return None;
    }
    let pending = SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let generation = surfaces.get_or_insert_with(host_id.to_string(), SceneSurfaceState::default).and_then(|state| state.host_temporal.observe(request))?;
        Some((surfaces.token(host_id)?, generation))
    });
    if let Some((token, generation)) = pending {
        #[cfg(target_arch = "wasm32")]
        {
            let host_id = host_id.to_string();
            let request = request.clone();
            crate::host_base::spawn_app_task(async move {
                let envelope = json!({ "op": "format-temporal-values", "nowMs": request.now_ms, "values": &request.values }).to_string();
                match crate::host_base::host_io_call(&envelope, None).await.ok().and_then(|answer| serde_json::from_str::<ui_contract::HostTemporalFormatReplyV1>(&answer).ok()) {
                    Some(reply) => {
                        let _ = publish_host_temporal_reply(token, generation, &request, reply);
                    }
                    None => refuse_host_temporal_reply(token, generation),
                }
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        match native_host_temporal_formatter().and_then(|formatter| formatter.format_temporal_values(request).ok()) {
            Some(reply) => {
                let _ = publish_host_temporal_reply(token, generation, request, reply);
            }
            None => refuse_host_temporal_reply(token, generation),
        }
    }
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).and_then(|state| state.host_temporal.visible().cloned()).filter(|reply| reply.matches(request)))
}

pub fn seal_host_temporal_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.host_temporal.seal(epoch);
        }
    });
}

pub fn acknowledge_host_temporal_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.host_temporal.acknowledge(epoch);
        }
    });
}

pub fn discard_host_temporal_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.host_temporal.discard(epoch);
        }
    });
}

pub fn stage_vfs_accessibility_controls(host_id: &str, controls: Vec<VfsAccessibilityControl>) {
    mutate_scene_state(host_id, |state| state.vfs_accessibility.stage(controls));
}

pub fn stage_block_list_accessibility_controls(host_id: &str, controls: Vec<BlockListAccessibilityControl>) {
    mutate_scene_state(host_id, |state| state.block_list_accessibility.stage(controls));
}

pub fn stage_event_feed_accessibility_controls(host_id: &str, controls: Vec<EventFeedAccessibilityControl>) {
    mutate_scene_state(host_id, |state| state.event_feed_accessibility.stage(controls));
}

pub fn stage_graph_timeline_accessibility_controls(host_id: &str, controls: Vec<GraphTimelineAccessibilityControl>) {
    mutate_scene_state(host_id, |state| state.graph_timeline_accessibility.stage(controls));
}

pub fn stage_table_stepper_accessibility_cells(host_id: &str, cells: Vec<TableStepperAccessibilityCell>) {
    mutate_scene_state(host_id, |state| state.table_accessibility.stage(cells));
}

pub fn stage_table_editable_text_accessibility_cells(host_id: &str, cells: Vec<TableEditableTextAccessibilityCell>) {
    mutate_scene_state(host_id, |state| state.table_editable_accessibility.stage(cells));
}

pub fn stage_table_button_accessibility_cells(host_id: &str, cells: Vec<TableButtonAccessibilityCell>) {
    mutate_scene_state(host_id, |state| state.table_button_accessibility.stage(cells));
}

pub fn accepted_table_stepper_accessibility_cells(host_id: &str) -> Vec<TableStepperAccessibilityCell> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.table_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn accepted_table_editable_text_accessibility_cells(host_id: &str) -> Vec<TableEditableTextAccessibilityCell> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.table_editable_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn accepted_table_button_accessibility_cells(host_id: &str) -> Vec<TableButtonAccessibilityCell> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.table_button_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn seal_table_stepper_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_accessibility.seal(epoch);
        }
    });
}

pub fn seal_table_editable_text_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_editable_accessibility.seal(epoch);
        }
    });
}

pub fn seal_table_button_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_button_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_table_stepper_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_accessibility.acknowledge(epoch);
        }
    });
}

pub fn acknowledge_table_editable_text_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_editable_accessibility.acknowledge(epoch);
        }
    });
}

pub fn acknowledge_table_button_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_button_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_table_stepper_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_accessibility.discard(epoch);
        }
    });
}

pub fn discard_table_editable_text_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_editable_accessibility.discard(epoch);
        }
    });
}

pub fn discard_table_button_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.table_button_accessibility.discard(epoch);
        }
    });
}

pub fn accepted_vfs_accessibility_controls(host_id: &str) -> Vec<VfsAccessibilityControl> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.vfs_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn accepted_block_list_accessibility_controls(host_id: &str) -> Vec<BlockListAccessibilityControl> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.block_list_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn accepted_event_feed_accessibility_controls(host_id: &str) -> Vec<EventFeedAccessibilityControl> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.event_feed_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn accepted_graph_timeline_accessibility_controls(host_id: &str) -> Vec<GraphTimelineAccessibilityControl> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.graph_timeline_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn vfs_accessibility_control_survives_candidate(host_id: &str, target: &VfsAccessibilityFocusTarget, epoch: u64) -> bool {
    SCENE_STATE.with(|cell| {
        let surfaces = cell.borrow();
        let Some(presentation) = surfaces.get(host_id).map(|state| &state.vfs_accessibility) else { return false };
        let controls = if presentation.candidate_epoch == Some(epoch) { presentation.candidate.as_deref().unwrap_or_default() } else { presentation.accepted.as_slice() };
        controls.iter().any(|control| control.row_id == target.row_id && control.kind == target.kind)
    })
}

pub fn seal_vfs_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.vfs_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_vfs_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.vfs_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_vfs_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.vfs_accessibility.discard(epoch);
        }
    });
}

pub fn seal_block_list_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.block_list_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_block_list_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.block_list_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_block_list_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.block_list_accessibility.discard(epoch);
        }
    });
}

pub fn stage_world3d_accessibility_controls(host_id: &str, controls: Vec<World3dAccessibilityControl>) {
    mutate_scene_state(host_id, |state| {
        if state.world3d_accessibility.candidate.as_ref().or(state.world3d_accessibility.queued_candidate.as_ref()).is_none_or(|staged| *staged != controls) && state.world3d_accessibility.accepted != controls {
            state.world3d_accessibility.stage(controls);
        }
    });
}

pub fn accepted_world3d_accessibility_controls(host_id: &str) -> Vec<World3dAccessibilityControl> {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.world3d_accessibility.accepted.clone()).unwrap_or_default())
}

pub fn seal_world3d_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.world3d_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_world3d_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.world3d_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_world3d_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.world3d_accessibility.discard(epoch);
        }
    });
}

pub fn seal_event_feed_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.event_feed_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_event_feed_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.event_feed_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_event_feed_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.event_feed_accessibility.discard(epoch);
        }
    });
}

pub fn seal_graph_timeline_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.graph_timeline_accessibility.seal(epoch);
        }
    });
}

pub fn acknowledge_graph_timeline_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.graph_timeline_accessibility.acknowledge(epoch);
        }
    });
}

pub fn discard_graph_timeline_accessibility_candidates(epoch: u64) {
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.graph_timeline_accessibility.discard(epoch);
        }
    });
}

pub fn event_feed_tone_color(tone: Option<&str>, theme: &Theme) -> Rgba {
    match tone {
        Some("success") => theme.success,
        Some("warning") => theme.warning,
        Some("error") | Some("fatal") => theme.error,
        _ => theme.text,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EventFeedLayout {
    pub inner: Rect,
    pub card_gap: f32,
    pub card_padding: f32,
    line_height: f32,
    pub icon_size: f32,
    pub time_font_size: f32,
}

pub fn event_feed_layout(bounds: Rect, theme: &Theme) -> EventFeedLayout {
    EventFeedLayout {
        inner: bounds.inset(theme.padding_standard),
        card_gap: theme.gap_standard,
        card_padding: theme.padding_standard,
        line_height: theme.root_rem_pixels,
        icon_size: theme.control_height_small,
        time_font_size: theme.root_rem_pixels * 0.625,
    }
}

pub fn event_feed_row_height(entry: &EventFeedEntryJson, layout: &EventFeedLayout) -> f32 {
    layout.card_padding * 2.0 + layout.line_height * if entry.detail.is_some() { 2.0 } else { 1.0 }
}

fn event_feed_visible_card(card: Rect, inner: Rect) -> Option<Rect> {
    let left = card.x.max(inner.x);
    let top = card.y.max(inner.y);
    let right = (card.x + card.w).min(inner.x + inner.w);
    let bottom = (card.y + card.h).min(inner.y + inner.h);
    (right > left && bottom > top).then(|| Rect::new(left, top, right - left, bottom - top))
}
/// 🎯️ Resolves a pointer point inside a `SurfaceKind::EventFeed`: the entry whose variable-height
/// band contains `y`. Row activation sends `{ surfaceId, id }` — the payload
/// `📡️EventFeedHost/🟦️.tsx`'s own `onClick` sends; this renderer used to send `{ entryId }`, which
/// no host reads.
pub fn event_feed_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme) -> Option<SceneListHit> {
    let feed = scene.event_feed.as_ref()?;
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let layout = event_feed_layout(bounds, theme);
    if !layout.inner.contains(x, y) {
        return None;
    }
    let scroll = scroll_offset(&scene.host_id, "feed");
    let mut top = layout.inner.y - scroll;
    for entry in &entries {
        let entry_h = event_feed_row_height(entry, &layout);
        let card = Rect::new(layout.inner.x, top, layout.inner.w, entry_h);
        if event_feed_visible_card(card, layout.inner).is_some_and(|card| card.contains(x, y)) {
            let action = feed.activate_action.as_deref().map(|action| scene_action(scene, action, semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "id": entry.id })));
            return Some(SceneListHit::row(format!("{}.feed.{}", scene.host_id, entry.id), action));
        }
        top += entry_h + layout.card_gap;
    }
    None
}

pub fn event_feed_accessibility_label(entry: &EventFeedEntryJson, time_label: Option<&str>) -> String {
    std::iter::once(entry.title.as_str()).chain(time_label).chain(entry.detail.as_deref()).filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" ")
}

pub fn event_feed_accessibility_controls(
    scene: &UiComponentSceneNode,
    layout: &EventFeedLayout,
    entries: &[EventFeedEntryJson],
    heights: &[f32],
    scroll: f32,
    time_reply: Option<&ui_contract::HostTemporalFormatReplyV1>,
) -> Vec<EventFeedAccessibilityControl> {
    let inner = layout.inner;
    let action = scene.event_feed.as_ref().and_then(|feed| feed.activate_action.as_deref());
    let mut y = inner.y - scroll;
    entries
        .iter()
        .zip(heights)
        .filter_map(|(entry, height)| {
            let card = event_feed_visible_card(Rect::new(inner.x, y, inner.w, *height), inner);
            y += *height + layout.card_gap;
            let rect = card?;
            let key = format!("{}.feed.{}", scene.host_id, entry.id);
            let time_label = time_reply.and_then(|reply| reply.label(&event_feed_time_id(entry.timestamp_ms)));
            Some(EventFeedAccessibilityControl {
                key,
                entry_id: entry.id.clone(),
                label: event_feed_accessibility_label(entry, time_label),
                rect,
                action: action.map(|action| scene_action(scene, action, semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "id": entry.id }))),
            })
        })
        .collect()
}

pub fn event_feed_accessibility_activate(scene: &UiComponentSceneNode, key: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Option<Result<(), semio_framework_ui_action::BoundedActionFault>> {
    let control = accepted_event_feed_accessibility_controls(&scene.host_id).into_iter().find(|control| control.key == key)?;
    let action = control.action.as_ref()?;
    let feed = scene.event_feed.as_ref()?;
    let action_name = feed.activate_action.as_deref()?;
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).ok()?;
    let expected = scene_action(scene, action_name, semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "id": control.entry_id }));
    (entries.iter().any(|entry| entry.id == control.entry_id) && *action == expected).then(|| write_scene_action(input, action))
}

/// 📜️ Renders `SurfaceKind::EventFeed`: a scrollable list of cards (`entries_json`), each with an
/// optional icon, time-of-day and weighted tone title, plus an optional detail line. When `follow` is
/// set the feed snaps to its bottom every frame (log-tail behaviour); a manual wheel-scroll on a
/// following feed is overridden on the next render, the same tradeoff a live log tail makes.
pub fn render_event_feed(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(feed) = &scene.event_feed else {
        stage_event_feed_accessibility_controls(&scene.host_id, Vec::new());
        return render_placeholder("event-feed", bounds, ctx);
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let time_request = event_feed_time_request(&entries);
    let time_reply = host_temporal_reply(&scene.host_id, &time_request);
    let layout = event_feed_layout(bounds, theme);
    let inner = layout.inner;
    let pad = layout.card_padding;
    if entries.is_empty() {
        stage_event_feed_accessibility_controls(&scene.host_id, Vec::new());
        draw_text(ctx, "—", inner.x + pad, inner.y + layout.line_height * 0.7, theme.font_size_small, theme.text_muted);
        return;
    }

    let heights: Vec<f32> = entries.iter().map(|entry| event_feed_row_height(entry, &layout)).collect();
    let content_h: f32 = heights.iter().sum::<f32>() + layout.card_gap * heights.len().saturating_sub(1) as f32;
    if feed.follow.unwrap_or(false) {
        set_scroll_offset(&scene.host_id, "feed", (content_h - inner.h).max(0.0));
    }
    let scroll = scroll_offset(&scene.host_id, "feed");
    stage_event_feed_accessibility_controls(&scene.host_id, event_feed_accessibility_controls(scene, &layout, &entries, &heights, scroll, time_reply.as_ref()));
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(scroll_key(&scene.host_id, "feed")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, layout.card_padding * 2.0 + layout.line_height + layout.card_gap));
    let hovered_row = scene_hovered_control_id(&scene.host_id).or_else(|| ctx.input.hovered_id.clone());
    let mut y = inner.y - scroll;
    for entry in entries.iter() {
        let entry_h = event_feed_row_height(entry, &layout);
        if y + entry_h < inner.y || y > inner.y + inner.h {
            y += entry_h + layout.card_gap;
            continue;
        }
        let control_id = format!("{}.feed.{}", scene.host_id, entry.id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, entry_h);
        let visible_row_rect = event_feed_visible_card(row_rect, inner).expect("visible EventFeed card");
        if hovered {
            ctx.draw.push_rounded([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover, theme.border_radius);
        }

        let tone_color = event_feed_tone_color(entry.tone.as_deref(), theme);
        let mut title_x = inner.x + pad;

        if !entry.icon_id.is_empty() {
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(&entry.icon_id) {
                    ctx.draw.push_textured([title_x, y + pad, layout.icon_size, layout.icon_size], uv, foreground_on_fill(theme, theme.text_element, false, hovered));
                    title_x += layout.icon_size + theme.gap_standard;
                }
            }
        }

        let content_right = inner.x + inner.w - pad;
        let time_label = time_reply.as_ref().and_then(|reply| reply.label(&event_feed_time_id(entry.timestamp_ms)));
        let time_x = time_label.map(|label| (content_right - ctx.atlas.measure_text(label, layout.time_font_size).0).max(title_x));
        let title_right = time_x.map_or(content_right, |x| x - theme.gap_standard);
        if title_right > title_x {
            ctx.draw.push_scissor(Rect::new(title_x, y + pad, title_right - title_x, layout.line_height));
            let weight = if entry.tone.as_deref() == Some("fatal") { semio_framework_ui_text::TextWeight::Semibold } else { semio_framework_ui_text::TextWeight::Medium };
            draw_text_weighted(ctx.draw, ctx.atlas, &entry.title, title_x, y + pad + layout.line_height * 0.7, theme.font_size_small, tone_color, weight);
            ctx.draw.pop_scissor();
        }
        if let (Some(time_label), Some(time_x)) = (time_label, time_x) {
            draw_text(ctx, time_label, time_x, y + pad + layout.line_height * 0.7, layout.time_font_size, foreground_on_fill(theme, theme.text_muted, false, hovered));
        }
        if let Some(detail) = &entry.detail {
            ctx.draw.push_scissor(Rect::new(title_x, y + pad + layout.line_height, (content_right - title_x).max(0.0), layout.line_height));
            draw_text(ctx, detail, title_x, y + pad + layout.line_height * 1.7, theme.font_size_small, foreground_on_fill(theme, theme.text_muted, false, hovered));
            ctx.draw.pop_scissor();
        }
        if let Some(action) = &feed.activate_action {
            ctx.input.register_hit(HitTarget {
                rect: visible_row_rect,
                event: Some(scene_action(scene, action, semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "id": entry.id }))),
                control_id: Some(control_id),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
        y += entry_h + layout.card_gap;
    }
    ctx.draw.pop_scissor();
}

//#endregion EventFeed

//#region GraphTimeline
/** 🗄️ Mirrors `store::HistoryColumn` / React `HistoryColumn` (`ui/js/react/index.tsx:19116`). */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumnAuthorJson {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default, deserialize_with = "deserialize_history_avatar")]
    avatar: Option<String>,
}

fn deserialize_history_avatar<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.filter(|source| !source.is_empty()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumnJson {
    pub checkpoint_id: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub authors: Vec<HistoryColumnAuthorJson>,
    #[serde(default)]
    pub parent_checkpoint_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub mutation_level: Option<String>,
    #[serde(default)]
    pub lane: usize,
}

const HISTORY_LANE_PITCH: f32 = 16.0;
const HISTORY_LANE_PAD: f32 = 8.0;
const HISTORY_AUTHOR_SLOT: f32 = 40.0;

fn history_avatar_x(base: f32, index: usize, size: f32, overlap: f32) -> f32 {
    base + index as f32 * (size - overlap)
}

pub fn history_avatar_image_id(host_id: &str, checkpoint_id: &str, author: &HistoryColumnAuthorJson, index: usize) -> String {
    let author_key = if author.id.is_empty() { index.to_string() } else { author.id.clone() };
    format!("{host_id}.history.avatar.{checkpoint_id}.{author_key}")
}

pub fn history_lane_count(columns: &[HistoryColumnJson]) -> usize {
    columns.iter().map(|column| column.lane + 1).max().unwrap_or(1).max(1)
}

pub fn history_graph_width(lane_count: usize) -> f32 {
    (HISTORY_LANE_PAD * 2.0 + lane_count as f32 * HISTORY_LANE_PITCH).max(56.0)
}

pub fn history_lane_x(lane: usize, lane_count: usize, graph_width: f32) -> f32 {
    if lane_count <= 1 {
        return graph_width * 0.5;
    }
    HISTORY_LANE_PAD + lane as f32 * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH * 0.5
}

pub fn history_row_lane_guides(columns: &[HistoryColumnJson], lane_count: usize) -> Vec<Vec<bool>> {
    let mut guides = vec![vec![false; lane_count]; columns.len()];
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();
    for (row_index, column) in columns.iter().enumerate() {
        if column.lane < lane_count {
            guides[row_index][column.lane] = true;
        }
        let Some(parent_row) = column.parent_checkpoint_id.as_deref().and_then(|id| row_by_id.get(id).copied()) else {
            continue;
        };
        let parent_lane = columns[parent_row].lane;
        if column.lane == parent_lane {
            for row in (row_index + 1)..parent_row {
                guides[row][column.lane] = true;
            }
            continue;
        }
        let elbow_row = if row_index + 1 < parent_row { row_index + 1 } else { parent_row };
        for row in (row_index + 1)..=elbow_row {
            if column.lane < lane_count {
                guides[row][column.lane] = true;
            }
        }
        for row in elbow_row..parent_row {
            if parent_lane < lane_count {
                guides[row][parent_lane] = true;
            }
        }
    }
    guides
}

pub fn graph_timeline_avatar_initials(name: &str) -> String {
    let letters: String = name.split_whitespace().filter_map(|word| word.chars().next()).take(2).flat_map(char::to_uppercase).collect();
    if letters.is_empty() {
        "?".to_string()
    } else {
        letters
    }
}

fn paint_history_avatar(ctx: &mut FrameworkWidgetContext<'_>, image_id: &str, author: Option<&HistoryColumnAuthorJson>, x: f32, y: f32, size: f32) {
    let theme = ctx.theme;
    let rect = Rect::new(x, y, size, size);
    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel_border, size * 0.5);
    let inside = [rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, rect.h - 2.0];
    let image = author.and_then(|author| author.avatar.as_deref()).and_then(|source| semio_framework_os_renderer_wgpu_scene_canvas2d::current_ui_image_key(image_id, source));
    if let Some(key) = image {
        ctx.draw.push_rounded_raster_quad(&key, inside, [0.0, 0.0, 1.0, 1.0], 1.0, (size - 2.0) * 0.5);
        return;
    }
    ctx.draw.push_rounded(inside, theme.muted, (size - 2.0) * 0.5);
    let initials = author.map(|author| graph_timeline_avatar_initials(&author.name)).unwrap_or_else(|| "?".to_string());
    let width = ctx.atlas.measure_text(&initials, theme.font_size_small).0;
    draw_text(ctx, &initials, rect.x + (size - width) * 0.5, rect.y + size * 0.7, theme.font_size_small, theme.text);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct GraphTimelineLayout {
    pub inner: Rect,
    pub row_height: f32,
    pub label_track_width: f32,
    pub graph_width: f32,
    pub graph_column_width: f32,
    pub selectable_width: f32,
    pub avatar_size: f32,
    pub avatar_overlap: f32,
}

/// 🏷️ React's checkpoint chips, their `checkpoint` placeholder and the mutation badge are `text-2xs` (`🕰️HistoryTable/🟦️.tsx`).
const GRAPH_TIMELINE_CHIP_FONT_PX: f32 = ui_styling::metrics::typography::TEXT2XS_PX as f32;

/// 📏️ One chip label's width in the face the painter pens it with.
fn graph_timeline_chip_text_width(atlas: &mut semio_framework_ui_text::FontAtlas, text: &str) -> f32 {
    atlas.measure_text(text, GRAPH_TIMELINE_CHIP_FONT_PX).0
}

/// 📏️ The label track React's `auto` grid column resolves to: the widest row's chips (each `px-1.5` around its measured label,
/// `gap-1` apart) or its `checkpoint` placeholder, inside the row's `px-single` padding.
pub fn graph_timeline_label_track_width(columns: &[HistoryColumnJson], theme: &Theme, atlas: &mut semio_framework_ui_text::FontAtlas) -> f32 {
    let chip_padding = theme.root_rem_pixels * 0.375;
    let chip_gap = theme.root_rem_pixels * 0.25;
    columns
        .iter()
        .map(|column| {
            let content = if column.labels.is_empty() {
                graph_timeline_chip_text_width(atlas, "checkpoint")
            } else {
                column.labels.iter().map(|label| graph_timeline_chip_text_width(atlas, label) + chip_padding * 2.0).sum::<f32>() + chip_gap * column.labels.len().saturating_sub(1) as f32
            };
            content + theme.padding_standard * 2.0
        })
        .fold(0.0, f32::max)
}

/// 📏️ [`graph_timeline_label_track_width`] of one scene's own columns.
pub fn graph_timeline_scene_label_track(scene: &UiComponentSceneNode, theme: &Theme, atlas: &mut semio_framework_ui_text::FontAtlas) -> f32 {
    let columns: Vec<HistoryColumnJson> = scene.graph_timeline.as_ref().and_then(|history| serde_json::from_str(&history.columns_json).ok()).unwrap_or_default();
    graph_timeline_label_track_width(&columns, theme, atlas)
}

/// 📏️ The label track `host_id`'s last paint measured (`0` before its first paint).
pub fn graph_timeline_painted_label_track(host_id: &str) -> f32 {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map_or(0.0, |state| state.graph_timeline_label_track))
}

/// 📐️ One accepted geometry authority shared by GraphTimeline paint, pointer, scrolling and AT.
pub fn graph_timeline_layout(bounds: Rect, columns: &[HistoryColumnJson], theme: &Theme, label_track: f32) -> GraphTimelineLayout {
    let inner = bounds.inset(theme.padding_standard);
    let graph_width = history_graph_width(history_lane_count(columns));
    let graph_column_width = graph_width + HISTORY_AUTHOR_SLOT;
    let label_track_width = label_track.min((inner.w - graph_column_width).max(0.0));
    GraphTimelineLayout {
        inner,
        row_height: theme.tree_row_height,
        label_track_width,
        graph_width,
        graph_column_width,
        selectable_width: (label_track_width + graph_column_width).min(inner.w).max(0.0),
        avatar_size: theme.control_height_small,
        avatar_overlap: theme.root_rem_pixels * 0.5,
    }
}

fn graph_timeline_mutation_tone(level: &str, theme: &Theme) -> Option<(Rgba, Rgba)> {
    match level {
        "info" => Some((theme.accent, theme.active_foreground)),
        "warning" => Some((theme.warning.with_alpha(0.2), theme.warning)),
        "error" => Some((theme.error.with_alpha(0.2), theme.error)),
        "fatal" => Some((theme.error, theme.active_foreground)),
        _ => None,
    }
}

fn graph_timeline_accessible_name(column: &HistoryColumnJson) -> String {
    if column.labels.is_empty() { column.checkpoint_id.clone() } else { column.labels.join(", ") }
}

pub fn graph_timeline_accessibility_controls(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme, label_track: f32) -> Vec<GraphTimelineAccessibilityControl> {
    let Some(history) = scene.graph_timeline.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<HistoryColumnJson>>(&history.columns_json) else { return Vec::new() };
    let layout = graph_timeline_layout(bounds, &columns, theme, label_track);
    let inner = layout.inner;
    let scroll = scroll_offset(&scene.host_id, "history");
    columns
        .iter()
        .enumerate()
        .filter_map(|(row_index, column)| {
            let y = inner.y + row_index as f32 * layout.row_height - scroll;
            let clipped_y = y.max(inner.y);
            let rect = Rect::new(inner.x, clipped_y, layout.selectable_width, (y + layout.row_height).min(inner.y + inner.h) - clipped_y);
            (rect.w > 0.0 && rect.h > 0.0).then(|| {
                let action = scene_action(scene, "checkoutCheckpoint", semio_framework::dsl_value!({ "checkpointId": column.checkpoint_id }));
                GraphTimelineAccessibilityControl {
                    key: format!("{}.history.{}", scene.host_id, column.checkpoint_id),
                    checkpoint_id: column.checkpoint_id.clone(),
                    label: graph_timeline_accessible_name(column),
                    rect,
                    action,
                }
            })
        })
        .collect()
}

pub fn graph_timeline_accessibility_activate(
    scene: &UiComponentSceneNode,
    key: &str,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Option<Result<(), semio_framework_ui_action::BoundedActionFault>> {
    let control = accepted_graph_timeline_accessibility_controls(&scene.host_id).into_iter().find(|control| control.key == key)?;
    let history = scene.graph_timeline.as_ref()?;
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).ok()?;
    let column = columns.iter().find(|column| column.checkpoint_id == control.checkpoint_id)?;
    let expected_key = format!("{}.history.{}", scene.host_id, column.checkpoint_id);
    let action = scene_action(scene, "checkoutCheckpoint", semio_framework::dsl_value!({ "checkpointId": column.checkpoint_id }));
    (control.key == expected_key && control.label == graph_timeline_accessible_name(column) && control.action == action).then(|| write_scene_action(input, &control.action))
}

/// 🕰️ Renders `SurfaceKind::GraphTimeline`: the checkpoint history graph — lane guides, parent
/// elbows, a commit dot, author initials and the checkpoint's description per row. Row selection
/// dispatches `checkoutCheckpoint` with `{ checkpointId }`, the payload `🌳️GraphTimelineHost/🟦️.tsx`
/// hands `HistoryTable`'s `onSelectCheckpoint`.
///
/// 🪢️ `color-mix(in oklab, var(--muted-foreground) 40%, transparent)` on the lane guides and
/// parent connectors in `graph-timeline-host.tsx` — a translucent line; opaque `theme.separator`
/// previously read as visibly heavier than React's thin, faded rail.
pub fn render_graph_timeline(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let columns: Vec<HistoryColumnJson> = scene.graph_timeline.as_ref().and_then(|history| serde_json::from_str(&history.columns_json).ok()).unwrap_or_default();
    let label_track = graph_timeline_label_track_width(&columns, theme, ctx.atlas);
    mutate_scene_state(&scene.host_id, |state| state.graph_timeline_label_track = label_track);
    stage_graph_timeline_accessibility_controls(&scene.host_id, graph_timeline_accessibility_controls(scene, bounds, theme, label_track));
    if scene.graph_timeline.is_none() {
        return render_placeholder("graph-timeline", bounds, ctx);
    }
    let layout = graph_timeline_layout(bounds, &columns, theme, label_track);
    let inner = layout.inner;
    let row_h = layout.row_height;
    let pad = theme.padding_standard;
    if columns.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }
    let lane_count = history_lane_count(&columns);
    let graph_width = layout.graph_width;
    let graph_col_w = layout.graph_column_width;
    let labels_col_w = layout.label_track_width;
    let selectable_w = layout.selectable_width;
    let guides = history_row_lane_guides(&columns, lane_count);
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();

    let scroll = scroll_offset(&scene.host_id, "history");
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(scroll_key(&scene.host_id, "history")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, row_h));
    let hovered_row = scene_hovered_control_id(&scene.host_id).or_else(|| ctx.input.hovered_id.clone());
    let graph_x0 = inner.x + labels_col_w;
    let desc_x = inner.x + labels_col_w + graph_col_w;

    for (row_index, column) in columns.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let control_id = format!("{}.history.{}", scene.host_id, column.checkpoint_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, row_h);
        let selectable_rect = Rect::new(inner.x, y.max(inner.y), selectable_w, (y + row_h).min(inner.y + inner.h) - y.max(inner.y));
        if hovered {
            ctx.draw.push_solid([selectable_rect.x, selectable_rect.y, selectable_rect.w, selectable_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let mut label_x = inner.x + pad;
        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, foreground_on_fill(theme, theme.text_muted, false, hovered));
        } else {
            for label in &column.labels {
                let chip_w = (graph_timeline_chip_text_width(ctx.atlas, label) + theme.root_rem_pixels * 0.75).min((inner.x + labels_col_w - pad - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                let chip_h = theme.control_height_small;
                ctx.draw.push_rounded([label_x, y + (row_h - chip_h) * 0.5, chip_w, chip_h], theme.accent, theme.border_radius);
                draw_text(ctx, label, label_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, theme.active_foreground);
                label_x += chip_w + theme.root_rem_pixels * 0.25;
            }
        }

        let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
        for lane in 0..lane_count {
            if guides[row_index][lane] {
                let lx = graph_x0 + history_lane_x(lane, lane_count, graph_width);
                ctx.draw.push_line(lx, y, lx, y + row_h, guide_stroke, 1.0);
            }
        }
        if let Some(parent_id) = column.parent_checkpoint_id.as_deref() {
            if let Some(&parent_row) = row_by_id.get(parent_id) {
                let x0 = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
                let parent_lane = columns[parent_row].lane;
                let x1 = graph_x0 + history_lane_x(parent_lane, lane_count, graph_width);
                let y0 = y + row_h * 0.5;
                let y1 = inner.y + parent_row as f32 * row_h - scroll + row_h * 0.5;
                if (x0 - x1).abs() < 0.5 {
                    ctx.draw.push_line(x0, y0, x1, y1, guide_stroke, 1.5);
                } else {
                    let elbow_y = y + row_h;
                    ctx.draw.push_line(x0, y0, x0, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x0, elbow_y, x1, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x1, elbow_y, x1, y1, guide_stroke, 1.5);
                }
            }
        }
        let dot_x = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([dot_x - 3.0, dot_y - 3.0, 6.0, 6.0], theme.text, 3.0);

        let avatar_x = graph_x0 + (history_lane_x(column.lane, lane_count, graph_width) - 12.0).max(0.0);
        let avatar_y = y + row_h * 0.5 - layout.avatar_size * 0.5;
        if column.authors.is_empty() {
            paint_history_avatar(ctx, &format!("{}.history.avatar.{}.unknown", scene.host_id, column.checkpoint_id), None, avatar_x, avatar_y, layout.avatar_size);
        } else {
            for (author_index, author) in column.authors.iter().enumerate() {
                let image_id = history_avatar_image_id(&scene.host_id, &column.checkpoint_id, author, author_index);
                paint_history_avatar(ctx, &image_id, Some(author), history_avatar_x(avatar_x, author_index, layout.avatar_size, layout.avatar_overlap), avatar_y, layout.avatar_size);
            }
        }

        let mut description_x = desc_x + pad;
        if let Some((level, (fill, text))) = column.mutation_level.as_deref().and_then(|level| graph_timeline_mutation_tone(level, theme).map(|tone| (level, tone))) {
            let label = level.to_uppercase();
            let badge_w = graph_timeline_chip_text_width(ctx.atlas, &label) + theme.root_rem_pixels * 0.75;
            let badge_h = layout.avatar_size;
            ctx.draw.push_rounded([description_x, y + (row_h - badge_h) * 0.5, badge_w, badge_h], fill, theme.border_radius);
            draw_text(ctx, &label, description_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, text);
            description_x += badge_w + theme.root_rem_pixels * 0.25;
        }
        if let Some(description) = &column.description {
            draw_text(ctx, description, description_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }

        ctx.input.register_hit(HitTarget {
            rect: selectable_rect,
            event: Some(scene_action(scene, "checkoutCheckpoint", semio_framework::dsl_value!({ "checkpointId": column.checkpoint_id }))),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}
/// 🎯️ Resolves the painted labels and graph area; description cells never check out a checkpoint.
pub fn graph_timeline_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme) -> Option<SceneListHit> {
    let history = scene.graph_timeline.as_ref()?;
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let layout = graph_timeline_layout(bounds, &columns, theme, graph_timeline_painted_label_track(&scene.host_id));
    let inner = layout.inner;
    if x < inner.x || x >= inner.x + layout.selectable_width || y < inner.y || y >= inner.y + inner.h {
        return None;
    }
    let scroll = scroll_offset(&scene.host_id, "history");
    let index = usize::try_from(((y - inner.y + scroll) / layout.row_height.max(1.0)).floor() as i64).ok()?;
    let column = columns.get(index)?;
    let action = scene_action(scene, "checkoutCheckpoint", semio_framework::dsl_value!({ "checkpointId": column.checkpoint_id }));
    Some(SceneListHit::row(format!("{}.history.{}", scene.host_id, column.checkpoint_id), Some(action)))
}

//#endregion GraphTimeline
