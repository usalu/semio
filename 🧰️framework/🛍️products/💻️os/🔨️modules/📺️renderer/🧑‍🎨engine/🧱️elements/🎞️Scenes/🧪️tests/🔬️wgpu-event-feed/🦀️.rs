use super::*;

#[test]
fn entries_json_tolerates_missing_optional_fields() {
    let json = r#"[{"id":"only-required"}]"#;
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(json).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, "only-required");
    assert_eq!(entries[0].timestamp_ms, 0);
    assert!(entries[0].icon_id.is_empty());
    assert!(entries[0].title.is_empty());
    assert!(entries[0].detail.is_none());
    assert!(entries[0].tone.is_none());
}

#[test]
fn entries_json_parses_the_full_wire_shape() {
    let json = r#"[{"id":"e1","timestampMs":1000,"iconId":"bell","title":"Built","detail":"3 warnings","tone":"success"}]"#;
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(json).unwrap();
    assert_eq!(entries[0].timestamp_ms, 1000);
    assert_eq!(entries[0].icon_id, "bell");
    assert_eq!(entries[0].title, "Built");
    assert_eq!(entries[0].detail.as_deref(), Some("3 warnings"));
    assert_eq!(entries[0].tone.as_deref(), Some("success"));
}

#[test]
fn epoch_zero_is_a_valid_event_feed_time_source() {
    let entries = vec![EventFeedEntryJson { id: "epoch".into(), timestamp_ms: 0, icon_id: String::new(), title: String::new(), detail: None, tone: None }];
    let request = event_feed_time_request(&entries);
    assert_eq!(request.values.len(), 1);
    assert_eq!(request.values[0].source, ui_contract::HostTemporalSourceV1::EpochMs { timestamp_ms: 0 });
}

#[test]
fn row_height_grows_when_detail_is_present() {
    let theme = Theme::dark();
    let without_detail = EventFeedEntryJson { id: "a".into(), timestamp_ms: 0, icon_id: String::new(), title: "a".into(), detail: None, tone: None };
    let with_detail = EventFeedEntryJson { id: "b".into(), timestamp_ms: 0, icon_id: String::new(), title: "b".into(), detail: Some("more".into()), tone: None };
    let row_h = theme.control_height;
    assert!(event_feed_row_height(&with_detail, row_h, &theme) > event_feed_row_height(&without_detail, row_h, &theme));
}

fn temporal_reply(revision: u64, request: &ui_contract::HostTemporalFormatRequestV1, prefix: &str) -> ui_contract::HostTemporalFormatReplyV1 {
    ui_contract::HostTemporalFormatReplyV1 {
        profile: ui_contract::HostTemporalProfileV1 { locale: "de-DE".into(), time_zone: "Europe/Berlin".into(), hour_cycle: ui_contract::HostHourCycleV1::H23, profile_revision: revision },
        labels: request.values.iter().map(|value| ui_contract::HostTemporalLabelV1 { id: value.id.clone(), text: format!("{prefix}:{}", value.id) }).collect(),
    }
}

#[test]
fn temporal_labels_publish_only_with_the_accepted_frame_and_stale_profiles_lose() {
    let entries = vec![
        EventFeedEntryJson { id: "before".into(), timestamp_ms: 1_772_956_399_000, icon_id: String::new(), title: String::new(), detail: None, tone: None },
        EventFeedEntryJson { id: "after".into(), timestamp_ms: 1_772_956_401_000, icon_id: String::new(), title: String::new(), detail: None, tone: None },
    ];
    let request = event_feed_time_request(&entries);
    let first_id = event_feed_time_id(entries[0].timestamp_ms);
    let mut state = HostTemporalPresentation::default();
    let generation = state.observe(&request).expect("first request");
    assert!(state.visible().is_none(), "missing host presentation suppresses the UTC fallback");
    assert!(state.publish(generation, &request, temporal_reply(8, &request, "host")));
    state.seal(40);
    assert_eq!(state.visible().and_then(|reply| reply.label(&first_id)), Some("host:event-feed:1772956399000"));
    state.discard(40);
    assert!(state.accepted.is_none(), "a rejected frame cannot publish its temporal cache");
    state.seal(41);
    state.acknowledge(41);
    assert_eq!(state.accepted.as_ref().map(|reply| reply.profile.profile_revision), Some(8));
    state.last_request_ms = 0.0;
    let stale_generation = state.observe(&request).expect("profile refresh");
    assert!(!state.publish(stale_generation, &request, temporal_reply(7, &request, "stale")));
    assert_eq!(state.accepted.as_ref().and_then(|reply| reply.label(&first_id)), Some("host:event-feed:1772956399000"));
}

#[test]
fn relative_clock_refresh_preserves_the_accepted_label_until_the_exact_next_reply() {
    let first = ui_contract::HostTemporalFormatRequestV1 {
        now_ms: 1_772_953_200_000,
        values: vec![ui_contract::HostTemporalValueV1 { id: "relative".into(), source: ui_contract::HostTemporalSourceV1::Iso { iso: "2026-03-08T09:00:00.000Z".into() }, format: ui_contract::HostTemporalFormatV1::Relative }],
    };
    let mut state = HostTemporalPresentation::default();
    let generation = state.observe(&first).expect("initial exact request");
    assert!(state.publish(generation, &first, temporal_reply(1, &first, "first")));
    state.seal(51);
    state.acknowledge(51);
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("first:relative"));

    state.last_request_ms = 0.0;
    let within_same_second = ui_contract::HostTemporalFormatRequestV1 { now_ms: first.now_ms + 999, values: first.values.clone() };
    assert!(state.observe(&within_same_second).is_none(), "a sub-second clock movement cannot publish a reply for a different exact request");
    assert_eq!(state.request.as_ref().map(|request| request.now_ms), Some(first.now_ms));
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("first:relative"));

    state.last_request_ms = 0.0;
    let next = ui_contract::HostTemporalFormatRequestV1 { now_ms: first.now_ms + 1_000, values: first.values.clone() };
    let next_generation = state.observe(&next).expect("next exact clock request");
    assert_eq!(state.request.as_ref().map(|request| request.now_ms), Some(next.now_ms));
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("first:relative"));
    assert!(state.publish(next_generation, &next, temporal_reply(1, &next, "next")));
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("first:relative"));
    state.seal(52);
    state.acknowledge(52);
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("next:relative"));
}

#[test]
fn temporal_completion_is_fenced_by_the_admitted_surface_token() {
    let host_id = "temporal-remount-token-law";
    let entries = vec![EventFeedEntryJson { id: "event".into(), timestamp_ms: 1_772_956_399_000, icon_id: String::new(), title: String::new(), detail: None, tone: None }];
    let request = event_feed_time_request(&entries);
    SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let _ = surfaces.remove(host_id);
        surfaces.get_or_insert_with(host_id.to_string(), SceneSurfaceState::default).expect("first owner");
    });
    let (first_token, first_generation) = SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let token = surfaces.token(host_id).expect("first token");
        let generation = surfaces.get_token_mut(token).unwrap().host_temporal.observe(&request).expect("first request");
        (token, generation)
    });
    SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        surfaces.remove(host_id).expect("retired first owner");
        surfaces.get_or_insert_with(host_id.to_string(), SceneSurfaceState::default).expect("replacement owner");
    });
    let (second_token, second_generation) = SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let token = surfaces.token(host_id).expect("replacement token");
        let generation = surfaces.get_token_mut(token).unwrap().host_temporal.observe(&request).expect("replacement request");
        (token, generation)
    });
    assert_ne!(first_token, second_token);
    assert!(!publish_host_temporal_reply(first_token, first_generation, &request, temporal_reply(1, &request, "retired")));
    SCENE_STATE.with(|cell| assert!(cell.borrow().get_token(second_token).unwrap().host_temporal.candidate.is_none()));
    assert!(publish_host_temporal_reply(second_token, second_generation, &request, temporal_reply(1, &request, "live")));
    SCENE_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let state = surfaces.get_token_mut(second_token).unwrap();
        state.host_temporal.seal(91);
        state.host_temporal.acknowledge(91);
        assert_eq!(state.host_temporal.accepted.as_ref().and_then(|reply| reply.label(&event_feed_time_id(entries[0].timestamp_ms))), Some("live:event-feed:1772956399000"));
        surfaces.remove(host_id).expect("test owner cleanup");
    });
}

#[test]
fn known_tones_resolve_to_distinct_theme_tokens() {
    let theme = Theme::dark();
    assert_eq!(event_feed_tone_color(Some("error"), &theme), theme.error);
    assert_eq!(event_feed_tone_color(Some("success"), &theme), theme.accent);
    assert_eq!(event_feed_tone_color(None, &theme), theme.text_muted);
    assert_eq!(event_feed_tone_color(Some("unknown-tone"), &theme), theme.text_muted);
}

//#region EventFeedPaintTests
/// 🧰️ Renders `render_event_feed` with one entry of the given `tone` and returns its `DrawList`,
/// so the title glyph's tint can be inspected directly — matches `FEED_TONE_CLASS`'s title-span
/// coloring in `event-feed-host.tsx`.
fn render_feed_entry(tone: Option<&str>) -> (ui_wgpu::wgpu::DrawList, Theme) {
    let entry = json!({ "id": "e1", "title": "Built", "tone": tone });
    let scene = UiComponentSceneNode {
        host_id: "feed-paint-test".into(),
        surface_id: "feed-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::EventFeed,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: Some(ui_wgpu::wgpu::EventFeedScene { entries_json: json!([entry]).to_string(), follow: None, activate_action: None, domain_id: None }),
        block_list: None,
        menu: None,
    };
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_event_feed(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), &mut ctx);
    }
    (draw, theme)
}

fn instance_colors(draw: &ui_wgpu::wgpu::DrawList) -> Vec<Rgba> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3])).collect()
}

#[test]
fn info_tone_title_is_full_brightness_foreground_not_muted() {
    let (draw, theme) = render_feed_entry(None);
    let colors = instance_colors(&draw);
    assert!(colors.contains(&theme.text), "an info/no-tone title must render at full theme.text brightness, matching FEED_TONE_CLASS's `info` case, got {colors:?}");
}

#[test]
fn error_tone_title_is_tinted_theme_error() {
    let (draw, theme) = render_feed_entry(Some("error"));
    let colors = instance_colors(&draw);
    assert!(colors.contains(&theme.error), "an error-tone title must be tinted theme.error, got {colors:?}");
}
//#endregion EventFeedPaintTests

//#region EventFeedPointerTests
/// 📡️ `📡️EventFeedHost/🟦️.tsx:117-126` sends `{ surfaceId, id }` on row activation. This renderer
/// used to send `{ entryId }` — a key no host reads, so every feed row activation was inert.
#[test]
fn row_activation_sends_surface_id_and_id_like_react() {
    let entries = json!([{ "id": "e1", "title": "Built" }, { "id": "e2", "title": "Failed" }]).to_string();
    let scene = UiComponentSceneNode {
        host_id: "feed-press-test".into(),
        surface_id: "feed-press-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::EventFeed,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: Some(ui_wgpu::wgpu::EventFeedScene { entries_json: entries, follow: None, activate_action: Some("openEntry".into()), domain_id: None }),
        block_list: None,
        menu: None,
    };
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 400.0, 200.0);
    let hit = event_feed_hit(&scene, bounds, theme.control_height * 1.5, &theme).expect("second entry hit");
    assert_eq!(hit.control_id, "feed-press-test.feed.e2");
    let action = hit.action.expect("activate action");
    assert_eq!(action.action, "openEntry");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("surfaceId").and_then(semio_framework::DslValue::as_str), Some("feed-press-test"));
    assert_eq!(args.get("id").and_then(semio_framework::DslValue::as_str), Some("e2"));
    assert!(args.get("entryId").is_none(), "the legacy entryId key must be gone");
}

/// 🪶️ A feed with no `activateAction` still resolves a hover target, but dispatches nothing.
#[test]
fn rows_without_an_activate_action_resolve_hover_only() {
    let (_, _) = render_feed_entry(None);
    let entries = json!([{ "id": "e1", "title": "Built" }]).to_string();
    let mut scene = UiComponentSceneNode {
        host_id: "feed-press-inert".into(),
        surface_id: "feed-press-inert".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::EventFeed,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };
    scene.event_feed = Some(ui_wgpu::wgpu::EventFeedScene { entries_json: entries, follow: None, activate_action: None, domain_id: None });
    let theme = Theme::default();
    let hit = event_feed_hit(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), theme.control_height * 0.5, &theme).expect("entry hit");
    assert_eq!(hit.control_id, "feed-press-inert.feed.e1");
    assert!(hit.action.is_none());
}
//#endregion EventFeedPointerTests
