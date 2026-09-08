
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
fn row_height_grows_when_detail_is_present() {
    let theme = Theme::dark();
    let without_detail = EventFeedEntryJson { id: "a".into(), timestamp_ms: 0, icon_id: String::new(), title: "a".into(), detail: None, tone: None };
    let with_detail = EventFeedEntryJson { id: "b".into(), timestamp_ms: 0, icon_id: String::new(), title: "b".into(), detail: Some("more".into()), tone: None };
    let row_h = theme.control_height;
    assert!(event_feed_row_height(&with_detail, row_h, &theme) > event_feed_row_height(&without_detail, row_h, &theme));
}

#[test]
fn time_of_day_wraps_within_a_day() {
    assert_eq!(event_feed_time_of_day_utc(0), "00:00:00");
    assert_eq!(event_feed_time_of_day_utc(3_661_000), "01:01:01");
    assert_eq!(event_feed_time_of_day_utc(86_400_000), "00:00:00");
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
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
