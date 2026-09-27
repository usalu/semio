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
    let layout = event_feed_layout(Rect::new(0.0, 0.0, 400.0, 200.0), &theme);
    let without_detail = EventFeedEntryJson { id: "a".into(), timestamp_ms: 0, icon_id: String::new(), title: "a".into(), detail: None, tone: None };
    let with_detail = EventFeedEntryJson { id: "b".into(), timestamp_ms: 0, icon_id: String::new(), title: "b".into(), detail: Some("more".into()), tone: None };
    assert_eq!(event_feed_row_height(&without_detail, &layout), theme.padding_standard * 2.0 + theme.root_rem_pixels);
    assert_eq!(event_feed_row_height(&with_detail, &layout), theme.padding_standard * 2.0 + theme.root_rem_pixels * 2.0);
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
    assert_eq!(state.visible().and_then(|reply| reply.label("relative")), Some("next:relative"));
    assert_eq!(state.accepted.as_ref().and_then(|reply| reply.label("relative")), Some("first:relative"));
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
    assert_eq!(event_feed_tone_color(Some("fatal"), &theme), theme.error);
    assert_eq!(event_feed_tone_color(Some("success"), &theme), theme.success);
    assert_eq!(event_feed_tone_color(Some("warning"), &theme), theme.warning);
    assert_eq!(event_feed_tone_color(None, &theme), theme.text);
    assert_eq!(event_feed_tone_color(Some("unknown-tone"), &theme), theme.text);
}

#[test]
fn neutral_visual_contract_drives_card_paint_hit_ax_and_semantic_tones() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📡️EventFeedHost/🧫️fixtures/🎨️layout/🔣️.json"))).expect("EventFeed visual layout fixture");
    let entries: Vec<EventFeedEntryJson> = serde_json::from_value(fixture["entries"].clone()).expect("visual entries");
    let expected = &fixture["expected"];
    let viewport = &fixture["viewport"];
    let bounds = Rect::new(0.0, 0.0, viewport["width"].as_f64().unwrap() as f32, viewport["height"].as_f64().unwrap() as f32);
    let theme = Theme::default();
    let layout = event_feed_layout(bounds, &theme);
    let close = |actual: f32, key: &str| (actual - expected[key].as_f64().unwrap() as f32).abs() < 0.001;
    assert!(close(layout.inner.x, "hostPaddingPx"));
    assert!(close(layout.card_gap, "cardGapPx"));
    assert!(close(layout.card_padding, "cardPaddingPx"));
    assert!(close(theme.border_radius, "cardRadiusPx"));
    assert!(close(layout.icon_size, "iconSizePx"));
    assert!(close(layout.time_font_size, "timeFontSizePx"));
    assert!(close(event_feed_row_height(&entries[0], &layout), "detailCardHeightPx"));
    assert!(close(event_feed_row_height(&entries[1], &layout), "plainCardHeightPx"));
    assert_eq!(event_feed_tone_color(Some("info"), &theme), theme.text);
    assert_eq!(event_feed_tone_color(Some("success"), &theme), theme.success);
    assert_eq!(event_feed_tone_color(Some("warning"), &theme), theme.warning);
    assert_eq!(event_feed_tone_color(Some("error"), &theme), theme.error);
    assert_eq!(event_feed_tone_color(Some("fatal"), &theme), theme.error);
    let medium = ui_wgpu::wgpu::TextWeight::Medium.synthetic_offset(theme.font_size_small).unwrap();
    let semibold = ui_wgpu::wgpu::TextWeight::Semibold.synthetic_offset(theme.font_size_small).unwrap();
    assert!(medium > 0.0 && medium < semibold, "font-medium stays lighter than fatal font-semibold");

    let scene = UiComponentSceneNode {
        host_id: "feed-visual-layout".into(),
        surface_id: fixture["surfaceId"].as_str().unwrap().into(),
        controller_id: fixture["controllerId"].as_str().unwrap().into(),
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
        event_feed: Some(ui_wgpu::wgpu::EventFeedScene {
            entries_json: fixture["entries"].to_string(),
            follow: None,
            activate_action: fixture["activateAction"].as_str().map(str::to_owned),
            domain_id: None,
        }),
        block_list: None,
        menu: None,
    };
    let heights: Vec<f32> = entries.iter().map(|entry| event_feed_row_height(entry, &layout)).collect();
    let controls = event_feed_accessibility_controls(&scene, &layout, &entries, &heights, 0.0, None);
    assert_eq!(controls[0].rect, Rect::new(layout.inner.x, layout.inner.y, layout.inner.w, heights[0]));
    assert_eq!(controls[1].rect.y, layout.inner.y + heights[0] + layout.card_gap);
    let gap_y = layout.inner.y + heights[0] + layout.card_gap * 0.5;
    assert!(event_feed_hit(&scene, bounds, layout.inner.x + 1.0, gap_y, &theme).is_none(), "the card gap is not actionable");
    assert!(event_feed_hit(&scene, bounds, layout.inner.x - 1.0, layout.inner.y + 1.0, &theme).is_none(), "host padding is not a card");
    let second_y = controls[1].rect.y + 1.0;
    assert_eq!(event_feed_hit(&scene, bounds, layout.inner.x + 1.0, second_y, &theme).expect("second card").control_id, "feed-visual-layout.feed.success");

    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_event_feed(&scene, bounds, &mut ctx);
    }
    let painted_cards: Vec<Rect> = input.staged_hits().iter().filter(|target| target.event.is_some()).map(|target| target.rect).collect();
    assert_eq!(painted_cards, controls.iter().map(|control| control.rect).collect::<Vec<_>>(), "paint, pointer and AX retain the same visible card rectangles");
    let colors = instance_colors(&draw);
    assert!(colors.contains(&theme.success));
    assert!(colors.contains(&theme.warning));
    assert!(colors.contains(&theme.error));

    let partial_scroll = heights[0] * 0.5;
    set_scroll_offset(&scene.host_id, "feed", partial_scroll);
    let clipped_controls = event_feed_accessibility_controls(&scene, &layout, &entries, &heights, partial_scroll, None);
    let mut clipped_draw = ui_wgpu::wgpu::DrawList::default();
    let mut clipped_atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut clipped_input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut clipped_draw, None, &mut clipped_atlas, None, &mut clipped_input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_event_feed(&scene, bounds, &mut ctx);
    }
    let clipped_hits: Vec<Rect> = clipped_input.staged_hits().iter().filter(|target| target.event.is_some()).map(|target| target.rect).collect();
    assert_eq!(clipped_hits, clipped_controls.iter().map(|control| control.rect).collect::<Vec<_>>(), "partially visible cards share clipped pointer and AX rectangles");
    assert_eq!(clipped_hits[0], Rect::new(layout.inner.x, layout.inner.y, layout.inner.w, heights[0] - partial_scroll));
    set_scroll_offset(&scene.host_id, "feed", 0.0);
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
    let layout = event_feed_layout(bounds, &theme);
    let first_height = event_feed_row_height(&serde_json::from_value(json!({ "id": "e1", "title": "Built" })).unwrap(), &layout);
    let hit = event_feed_hit(&scene, bounds, layout.inner.x + 1.0, layout.inner.y + first_height + layout.card_gap + 1.0, &theme).expect("second entry hit");
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
    let bounds = Rect::new(0.0, 0.0, 400.0, 200.0);
    let layout = event_feed_layout(bounds, &theme);
    let hit = event_feed_hit(&scene, bounds, layout.inner.x + 1.0, layout.inner.y + 1.0, &theme).expect("entry hit");
    assert_eq!(hit.control_id, "feed-press-inert.feed.e1");
    assert!(hit.action.is_none());
}

#[test]
fn neutral_accessible_entries_stage_clipped_labels_and_exact_actions() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📡️EventFeedHost/🧫️fixtures/♿️accessible-entry/🔣️.json"))).expect("EventFeed accessible entry fixture");
    let theme = Theme::default();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let entries_json = json!([row["entry"].clone()]).to_string();
        let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&entries_json).unwrap();
        let scene = UiComponentSceneNode {
            host_id: fixture["hostId"].as_str().unwrap().into(),
            surface_id: fixture["surfaceId"].as_str().unwrap().into(),
            controller_id: fixture["controllerId"].as_str().unwrap().into(),
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
            event_feed: Some(ui_wgpu::wgpu::EventFeedScene { entries_json, follow: None, activate_action: row["activateAction"].as_str().map(str::to_owned), domain_id: None }),
            block_list: None,
            menu: None,
        };
        let time_request = event_feed_time_request(&entries);
        let time_reply = ui_contract::HostTemporalFormatReplyV1 {
            profile: ui_contract::HostTemporalProfileV1 { locale: "de-DE".into(), time_zone: "Europe/Berlin".into(), hour_cycle: ui_contract::HostHourCycleV1::H23, profile_revision: 9 },
            labels: vec![ui_contract::HostTemporalLabelV1 { id: time_request.values[0].id.clone(), text: fixture["acceptedTimeLabel"].as_str().unwrap().into() }],
        };
        let bounds = Rect::new(4.0, 10.0, 300.0, theme.control_height);
        let layout = event_feed_layout(bounds, &theme);
        let controls = event_feed_accessibility_controls(&scene, &layout, &entries, &[event_feed_row_height(&entries[0], &layout)], 0.0, Some(&time_reply));
        let expected_label = event_feed_accessibility_label(&entries[0], Some(fixture["acceptedTimeLabel"].as_str().unwrap()));
        assert_eq!(controls.len(), 1);
        assert_eq!(controls[0].key, row["expected"]["key"].as_str().unwrap());
        assert_eq!(controls[0].label, row["expected"]["accessibleName"].as_str().unwrap_or(&expected_label));
        assert_eq!(controls[0].rect, layout.inner, "a partially visible detail card publishes only its clipped accepted rect");
        assert_eq!(controls[0].action.is_some(), row["expected"]["action"].is_object());
        stage_event_feed_accessibility_controls(&scene.host_id, controls);
        let epoch = 910 + index as u64;
        seal_event_feed_accessibility_candidates(epoch);
        acknowledge_event_feed_accessibility_candidates(epoch);
        if let Some(expected) = row["expected"]["action"].as_object() {
            let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
            event_feed_accessibility_activate(&scene, row["expected"]["key"].as_str().unwrap(), &mut input).expect("accepted control").expect("bounded action");
            let actions = crate::collect_fixture_actions(&mut input);
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0].controller_id, expected["controllerId"].as_str().unwrap());
            assert_eq!(actions[0].action, expected["action"].as_str().unwrap());
            let args = actions[0].args.as_ref().unwrap();
            assert_eq!(args.get("surfaceId").and_then(semio_framework::DslValue::as_str), expected["args"]["surfaceId"].as_str());
            assert_eq!(args.get("id").and_then(semio_framework::DslValue::as_str), expected["args"]["id"].as_str());
        } else {
            let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
            assert!(event_feed_accessibility_activate(&scene, row["expected"]["key"].as_str().unwrap(), &mut input).is_none());
            assert!(crate::collect_fixture_actions(&mut input).is_empty());
        }
    }
}
//#endregion EventFeedPointerTests
