use super::*;

fn column(id: &str, lane: usize, parent: Option<&str>) -> HistoryColumnJson {
    HistoryColumnJson { checkpoint_id: id.to_string(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: parent.map(str::to_string), description: None, lane }
}

#[test]
fn lane_count_is_max_lane_plus_one() {
    let columns = vec![column("a", 0, None), column("b", 2, Some("a"))];
    assert_eq!(history_lane_count(&columns), 3);
}

#[test]
fn lane_count_defaults_to_one_for_empty_columns() {
    assert_eq!(history_lane_count(&[]), 1);
}

#[test]
fn lane_x_centers_graph_when_single_lane() {
    let width = history_graph_width(1);
    assert_eq!(history_lane_x(0, 1, width), width * 0.5);
}

#[test]
fn linear_history_guides_stay_on_single_lane() {
    let columns = vec![column("c", 0, Some("b")), column("b", 0, Some("a")), column("a", 0, None)];
    let guides = history_row_lane_guides(&columns, 1);
    assert!(guides.iter().all(|row| row[0]), "a linear single-lane history must keep every row's lane-0 guide active");
}

#[test]
fn fork_guides_propagate_through_elbow_row() {
    let columns = vec![column("c", 1, Some("a")), column("b", 0, None), column("a", 0, None)];
    let guides = history_row_lane_guides(&columns, 2);
    assert!(guides[0][1], "the forking checkpoint's own row must show its lane");
    assert!(guides[1][1] || guides[1][0], "the elbow row must carry a guide on at least one of the two connected lanes");
}

#[test]
fn columns_json_tolerates_missing_optional_fields() {
    let json = r#"[{"checkpointId":"only-required"}]"#;
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(json).unwrap();
    assert_eq!(columns.len(), 1);
    assert_eq!(columns[0].checkpoint_id, "only-required");
    assert_eq!(columns[0].lane, 0);
    assert!(columns[0].labels.is_empty());
    assert!(columns[0].authors.is_empty());
    assert!(columns[0].parent_checkpoint_id.is_none());
    assert!(columns[0].description.is_none());
}

//#region GraphTimelinePaintTests
#[test]
fn avatar_initials_take_the_first_letter_of_the_first_two_words() {
    assert_eq!(graph_timeline_avatar_initials("Jane Doe"), "JD");
    assert_eq!(graph_timeline_avatar_initials("cher"), "C");
    assert_eq!(graph_timeline_avatar_initials("  "), "?");
    assert_eq!(graph_timeline_avatar_initials(""), "?");
    assert_eq!(graph_timeline_avatar_initials("Ada Lovelace Byron"), "AL");
}

fn author_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/👥️graph-timeline-authors/🔣️.json"))).expect("shared GraphTimeline author fixture")
}

fn author_scene(authors: serde_json::Value) -> UiComponentSceneNode {
    let fixture = author_fixture();
    let checkpoint_id = fixture["checkpointId"].as_str().expect("checkpoint id");
    UiComponentSceneNode {
        host_id: "timeline-author-paint-test".into(),
        surface_id: "timeline-author-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::GraphTimeline,
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
        graph_timeline: Some(ui_wgpu::wgpu::GraphTimelineScene { columns_json: json!([{ "checkpointId": checkpoint_id, "authors": authors, "description": "authored", "lane": 0 }]).to_string() }),
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

fn paint_author_scene(scene: &UiComponentSceneNode) -> ui_wgpu::wgpu::DrawList {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_graph_timeline(scene, Rect::new(0.0, 0.0, 400.0, 100.0), &mut ctx);
    }
    draw
}

#[test]
fn shared_author_fixture_preserves_authored_identity_order_overlap_and_initials() {
    let fixture = author_fixture();
    assert_eq!(fixture["avatarSize"].as_f64(), Some(HISTORY_AVATAR_SIZE as f64));
    assert_eq!(fixture["overlap"].as_f64(), Some(HISTORY_AVATAR_OVERLAP as f64));
    assert_eq!(fixture["advance"].as_f64(), Some((HISTORY_AVATAR_SIZE - HISTORY_AVATAR_OVERLAP) as f64));
    let columns: Vec<HistoryColumnJson> = serde_json::from_value(json!([{ "checkpointId": fixture["checkpointId"], "authors": fixture["authors"] }])).expect("GraphTimeline private author decode");
    let authors = &columns[0].authors;
    assert_eq!(authors.iter().map(|author| author.id.as_str()).collect::<Vec<_>>(), ["ada", "grace"]);
    assert_eq!(authors.iter().map(|author| graph_timeline_avatar_initials(&author.name)).collect::<Vec<_>>(), ["AL", "GH"]);
    assert!(authors[0].avatar.as_deref().is_some_and(|source| source.starts_with("data:image/svg+xml,")));
    assert_eq!(authors[1].avatar, None);
    assert_eq!(history_avatar_image_id("host", "checkpoint", &authors[0], 0), "host.history.avatar.checkpoint.ada");
    let anonymous: HistoryColumnAuthorJson = serde_json::from_value(json!({ "name": "Anonymous", "avatar": "" })).expect("empty source is absent");
    assert_eq!(anonymous.avatar, None);
    assert_eq!(history_avatar_image_id("host", "checkpoint", &anonymous, 3), "host.history.avatar.checkpoint.3");
}

#[test]
fn graph_timeline_paints_every_author_and_never_reuses_a_replaced_avatar_source() {
    let fixture = author_fixture();
    let authors = fixture["authors"].clone();
    let mut unresolved_authors = authors.clone();
    unresolved_authors[0]["avatar"] = fixture["replacementAvatar"].clone();
    let unresolved = paint_author_scene(&author_scene(unresolved_authors));
    assert!(unresolved.layers.iter().all(|layer| layer.raster_instances.is_empty()), "unresolved sources keep authored initials visible");
    let unresolved_fallbacks =
        unresolved.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == 1.0 && (instance.rect[2] - 18.0).abs() < f32::EPSILON && (instance.rect[3] - 18.0).abs() < f32::EPSILON).count();
    assert_eq!(unresolved_fallbacks, 2, "both unresolved authors retain their own initials circles");

    let painted = paint_author_scene(&author_scene(authors.clone()));
    let mut avatar_borders: Vec<[f32; 4]> =
        painted.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == 1.0 && instance.rect[2] == HISTORY_AVATAR_SIZE && instance.rect[3] == HISTORY_AVATAR_SIZE).map(|instance| instance.rect).collect();
    avatar_borders.sort_by(|left, right| left[0].total_cmp(&right[0]));
    assert_eq!(avatar_borders.len(), 2, "each authored entry owns one circle");
    assert_eq!(avatar_borders[1][0] - avatar_borders[0][0], HISTORY_AVATAR_SIZE - HISTORY_AVATAR_OVERLAP);
    let rasters: Vec<_> = painted.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    assert_eq!(rasters.len(), 1, "only Ada has a decoded image source");
    assert!(rasters[0].0.contains("timeline-author-paint-test.history.avatar.checkpoint-authors.ada"));

    let mut replaced_authors = authors;
    replaced_authors[0]["avatar"] = fixture["replacementAvatar"].clone();
    let replaced = paint_author_scene(&author_scene(replaced_authors));
    assert!(replaced.layers.iter().all(|layer| layer.raster_instances.is_empty()), "the old Ada pixels must not survive a changed source");
}

#[test]
fn lane_guide_lines_are_translucent_not_the_opaque_separator_token() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let scene = UiComponentSceneNode {
        host_id: "timeline-paint-test".into(),
        surface_id: "timeline-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::GraphTimeline,
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
        graph_timeline: Some(ui_wgpu::wgpu::GraphTimelineScene {
            columns_json: json!([
                { "checkpointId": "b", "lane": 0, "parentCheckpointId": "a" },
                { "checkpointId": "a", "lane": 0 },
            ])
            .to_string(),
        }),
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_graph_timeline(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), &mut ctx);
    }
    // 🖊️ Note: the per-row bottom hairline (a separate, legitimate divider) still uses the fully
    // opaque `theme.separator`, so this only checks that the translucent guide stroke is present
    // among the parent-connector line's vertices, not that opaque `theme.separator` is absent.
    let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
    let vertex_colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).map(|v| v.color).collect();
    assert!(vertex_colors.contains(&[guide_stroke.r, guide_stroke.g, guide_stroke.b, guide_stroke.a]), "the parent-connector line must use the translucent guide stroke, got {vertex_colors:?}");
}
//#endregion GraphTimelinePaintTests

//#region GraphTimelinePointerTests
/// 🕰️ `🌳️GraphTimelineHost/🟦️.tsx` sends `checkoutCheckpoint` with `{ checkpointId }` only — no
/// `surfaceId` — and the row band a press resolves to is the paint's own `control_height * 1.33` pitch.
#[test]
fn row_press_dispatches_checkout_checkpoint_for_the_row_under_the_pointer() {
    let scene = UiComponentSceneNode {
        host_id: "timeline-press-test".into(),
        surface_id: "timeline-press-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::GraphTimeline,
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
        graph_timeline: Some(ui_wgpu::wgpu::GraphTimelineScene { columns_json: json!([{ "checkpointId": "b", "lane": 0, "parentCheckpointId": "a" }, { "checkpointId": "a", "lane": 0 }]).to_string() }),
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };
    let theme = Theme::default();
    let row_h = theme.control_height * 1.33;
    let hit = graph_timeline_hit(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), row_h * 1.5, &theme).expect("second row hit");
    assert_eq!(hit.control_id, "timeline-press-test.history.a");
    let action = hit.action.expect("checkoutCheckpoint action");
    assert_eq!(action.action, "checkoutCheckpoint");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("checkpointId").and_then(semio_framework::DslValue::as_str), Some("a"));
    assert!(args.get("surfaceId").is_none(), "React sends checkoutCheckpoint without a surfaceId");
}
//#endregion GraphTimelinePointerTests
