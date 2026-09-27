use super::*;

fn column(id: &str, lane: usize, parent: Option<&str>) -> HistoryColumnJson {
    HistoryColumnJson { checkpoint_id: id.to_string(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: parent.map(str::to_string), description: None, mutation_level: None, lane }
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

fn layout_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🌳️GraphTimelineHost/🧫️fixtures/🎨️layout/🔣️.json")).expect("shared GraphTimeline layout fixture")
}

fn drain_actions(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    crate::collect_fixture_actions(input)
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
    let theme = Theme::default();
    let avatar_size = theme.control_height_small;
    let avatar_overlap = theme.root_rem_pixels * 0.5;
    assert_eq!(fixture["avatarSize"].as_f64(), Some(avatar_size as f64));
    assert_eq!(fixture["overlap"].as_f64(), Some(avatar_overlap as f64));
    assert_eq!(fixture["advance"].as_f64(), Some((avatar_size - avatar_overlap) as f64));
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
    let avatar_size = Theme::default().control_height_small;
    let avatar_overlap = Theme::default().root_rem_pixels * 0.5;
    let unresolved_fallbacks =
        unresolved.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == 1.0 && (instance.rect[2] - (avatar_size - 2.0)).abs() < f32::EPSILON && (instance.rect[3] - (avatar_size - 2.0)).abs() < f32::EPSILON).count();
    assert_eq!(unresolved_fallbacks, 2, "both unresolved authors retain their own initials circles");

    let painted = paint_author_scene(&author_scene(authors.clone()));
    let mut avatar_borders: Vec<[f32; 4]> =
        painted.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == 1.0 && instance.rect[2] == avatar_size && instance.rect[3] == avatar_size).map(|instance| instance.rect).collect();
    avatar_borders.sort_by(|left, right| left[0].total_cmp(&right[0]));
    assert_eq!(avatar_borders.len(), 2, "each authored entry owns one circle");
    assert_eq!(avatar_borders[1][0] - avatar_borders[0][0], avatar_size - avatar_overlap);
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

#[test]
fn shared_layout_contract_drives_paint_hit_accessibility_and_mutation_badge_geometry() {
    let fixture = layout_fixture();
    let columns: Vec<HistoryColumnJson> = serde_json::from_value(fixture["columns"].clone()).expect("GraphTimeline layout columns");
    let expected = &fixture["expected"];
    let viewport = &fixture["viewport"];
    let bounds = Rect::new(0.0, 0.0, viewport["width"].as_f64().unwrap() as f32, viewport["height"].as_f64().unwrap() as f32);
    let theme = Theme::default();
    let layout = graph_timeline_layout(bounds, &columns, &theme);
    let close = |actual: f32, expected: &serde_json::Value| (actual - expected.as_f64().unwrap() as f32).abs() < 0.001;
    assert!(close(layout.inner.x, &expected["hostPaddingPx"]));
    assert!(close(layout.row_height, &expected["rowHeightPx"]));
    assert!(close(layout.graph_width, &expected["graphWidthPx"]));
    assert!(close(layout.graph_column_width, &expected["graphColumnWidthPx"]));
    assert!((layout.label_track_width - expected["labelTrackPx"].as_f64().unwrap() as f32).abs() <= expected["labelTrackTolerancePx"].as_f64().unwrap() as f32);
    assert!(layout.label_track_width < bounds.w * expected["labelTrackMaximumViewportFraction"].as_f64().unwrap() as f32);
    assert!(close(layout.avatar_size, &expected["avatarSizePx"]));
    assert!(close(layout.avatar_overlap, &expected["avatarOverlapPx"]));
    let checkpoint = columns.iter().find(|column| column.checkpoint_id == expected["checkpointId"].as_str().unwrap()).expect("layout checkpoint");
    let author_left = (history_lane_x(checkpoint.lane, history_lane_count(&columns), layout.graph_width) - 12.0).max(0.0);
    assert!(close(author_left, &expected["authorLeftPx"]));
    assert_eq!(checkpoint.mutation_level.as_deref(), expected["mutationLevel"].as_str());

    let mut scene = author_scene(json!([]));
    scene.host_id = "timeline-layout".into();
    scene.controller_id = "timeline-layout-controller".into();
    scene.graph_timeline.as_mut().unwrap().columns_json = fixture["columns"].to_string();
    let y = layout.inner.y + layout.row_height * 0.5;
    let graph_x = layout.inner.x + layout.label_track_width + layout.graph_width * 0.5;
    assert_eq!(graph_timeline_hit(&scene, bounds, graph_x, y, &theme).expect("graph checkpoint hit").control_id, "timeline-layout.history.c4");
    assert!(graph_timeline_hit(&scene, bounds, layout.inner.x + layout.selectable_width + 0.1, y, &theme).is_none(), "description stays inert");
    let control = graph_timeline_accessibility_controls(&scene, bounds, &theme).into_iter().next().expect("checkpoint accessibility");
    assert_eq!(control.rect, Rect::new(layout.inner.x, layout.inner.y, layout.selectable_width, layout.row_height));

    let painted = paint_author_scene(&scene);
    let warning = theme.warning.with_alpha(0.2);
    assert!(
        painted.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| instance.color == [warning.r, warning.g, warning.b, warning.a]),
        "the warning mutation badge must use the semantic warning tone",
    );
}
//#endregion GraphTimelinePaintTests

//#region GraphTimelinePointerTests
/// 🕰️ `🌳️GraphTimelineHost/🟦️.tsx` sends `checkoutCheckpoint` with `{ checkpointId }` only — no
/// `surfaceId` — and the row band a press resolves to is the paint's own `h-workbench` pitch.
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
    let bounds = Rect::new(0.0, 0.0, 400.0, 200.0);
    let row_h = theme.tree_row_height;
    let hit = graph_timeline_hit(&scene, bounds, 20.0, theme.padding_standard + row_h * 1.5, &theme).expect("second row hit");
    assert_eq!(hit.control_id, "timeline-press-test.history.a");
    let action = hit.action.expect("checkoutCheckpoint action");
    assert_eq!(action.action, "checkoutCheckpoint");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("checkpointId").and_then(semio_framework::DslValue::as_str), Some("a"));
    assert!(args.get("surfaceId").is_none(), "React sends checkoutCheckpoint without a surfaceId");
}
//#endregion GraphTimelinePointerTests

#[test]
fn checkpoint_regions_match_react_without_activating_descriptions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🌳️GraphTimelineHost/🧫️fixtures/🎯️checkpoint-hit/🔣️.json")).expect("checkpoint hit fixture");
    let mut scene = author_scene(json!([]));
    scene.host_id = "timeline-checkpoint-regions".into();
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.graph_timeline.as_mut().unwrap().columns_json = fixture["columns"].to_string();
    let values = fixture["bounds"].as_array().unwrap();
    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_graph_timeline(&scene, bounds, &mut ctx);
    }
    let scroll_hit = input.staged_hits().iter().find(|target| target.kind == HitKind::ScrollRegion).expect("full scroll region");
    assert_eq!([scroll_hit.rect.x, scroll_hit.rect.y, scroll_hit.rect.w, scroll_hit.rect.h], [bounds.x, bounds.y, bounds.w, bounds.h]);
    for sample in fixture["cases"].as_array().unwrap() {
        let x = sample["x"].as_f64().unwrap() as f32;
        let y = bounds.y + theme.padding_standard + (sample["row"].as_f64().unwrap() as f32 + 0.5) * theme.tree_row_height;
        let hit = scene_list_hit(&scene, bounds, x, y, &theme, true, SceneModifiers::default(), UiDriverDrag::default());
        let action = hit.and_then(|hit| hit.action).map(|action| json!({ "controllerId": action.controller_id, "action": action.action, "args": action.args })).unwrap_or(serde_json::Value::Null);
        assert_eq!(action, sample["action"], "generic Scene route: {}", sample["id"]);
        let target = input.staged_hits().iter().rev().find(|target| target.event.is_some() && x >= target.rect.x && x < target.rect.x + target.rect.w && y >= target.rect.y && y < target.rect.y + target.rect.h);
        let action = target.and_then(|target| target.event.as_ref()).map(|action| json!({ "controllerId": action.controller_id, "action": action.action, "args": action.args })).unwrap_or(serde_json::Value::Null);
        assert_eq!(action, sample["action"], "painted hit route: {}", sample["id"]);
    }
}

#[test]
fn accepted_checkpoint_accessibility_uses_selectable_geometry_and_exact_action() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🌳️GraphTimelineHost/🧫️fixtures/🎯️checkpoint-hit/🔣️.json")).expect("checkpoint hit fixture");
    let expected = &fixture["accessibility"];
    let mut scene = author_scene(json!([]));
    scene.host_id = fixture["hostId"].as_str().unwrap().into();
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.graph_timeline.as_mut().unwrap().columns_json = fixture["columns"].to_string();
    let values = fixture["bounds"].as_array().unwrap();
    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let theme = Theme::default();
    let controls = graph_timeline_accessibility_controls(&scene, bounds, &theme);
    let control = controls.iter().find(|control| control.checkpoint_id == expected["checkpointId"].as_str().unwrap()).expect("checkpoint accessibility control").clone();
    assert_eq!(control.key, expected["key"].as_str().unwrap());
    assert_eq!(control.label, expected["accessibleName"].as_str().unwrap());
    assert_eq!(serde_json::to_value(&control.action).expect("action json"), expected["action"]);
    let selectable = fixture["cases"].as_array().unwrap().iter().find(|sample| sample["id"] == "graph-select").unwrap();
    let description = fixture["cases"].as_array().unwrap().iter().find(|sample| sample["id"] == "description-text-inert").unwrap();
    let y = bounds.y + theme.padding_standard + 1.5 * theme.tree_row_height;
    assert!(control.rect.contains(selectable["x"].as_f64().unwrap() as f32, y));
    assert!(!control.rect.contains(description["x"].as_f64().unwrap() as f32, y), "description is outside the virtual checkpoint button");

    stage_graph_timeline_accessibility_controls(&scene.host_id, controls);
    seal_graph_timeline_accessibility_candidates(921);
    acknowledge_graph_timeline_accessibility_candidates(921);
    let mut input = InputState::<ActionDescriptor>::default();
    graph_timeline_accessibility_activate(&scene, &control.key, &mut input).expect("accepted checkpoint").expect("bounded action");
    let actions = drain_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), expected["action"]);

    scene.graph_timeline.as_mut().unwrap().columns_json = json!([{ "checkpointId": "checkpoint-b", "labels": ["Head"], "lane": 0 }]).to_string();
    assert!(graph_timeline_accessibility_activate(&scene, &control.key, &mut input).is_none(), "a removed accepted checkpoint cannot activate");
    assert!(drain_actions(&mut input).is_empty());
}
