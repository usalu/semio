
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
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
