
    use super::*;
    use crate::editor::puzzle3d::{empty_fixture, PUZZLE3D_VORTEX_SHOW_SELECTED};

    fn forest_table_object() -> Puzzle3dObject {
        let positions = [
            [4.05001, 4.676537, 3.0],
            [6.75001, 4.676537, 3.0],
            [9.45001, 4.676537, 3.0],
            [6.75001, 0.0, 3.0],
            [4.05001, 0.0, 3.0],
            [1.35001, 0.0, 3.0],
            [9.45001, 2.338269, 3.0],
            [2.70001, 2.338269, 0.0],
            [2.70001, 2.338269, 3.0],
            [8.10001, 2.338269, 0.0],
            [8.10001, 2.338269, 3.0],
        ];
        Puzzle3dObject {
            id: "seed-left-001".into(),
            label: None,
            object_kind: None,
            origin: [0.0; 3],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: positions
                .iter()
                .copied()
                .enumerate()
                .map(|(index, position)| Puzzle3dVortex { id: format!("seed-left-001:v{index}"), position, radius: Some(0.36), ..Default::default() })
                .collect(),
            hidden: false,
            locked: false,
        }
    }

    fn forest_store() -> Puzzle3dFixture {
        let mut fixture = empty_fixture();
        fixture.objects.push(forest_table_object());
        fixture
    }

    fn parse_records(json: &str) -> Vec<Value> {
        serde_json::from_str(json).expect("vorticesJson")
    }

    #[test]
    fn world_vortices_json_carries_store_vortices_when_show_is_always() {
        let fixture = forest_store();
        assert_eq!(fixture.objects[0].vortices.len(), 11, "Concrete Forest seed-left-001 ships 11 vortex records");
        let mut runtime = Puzzle3dRuntime::default();
        runtime.vortex_show = PUZZLE3D_VORTEX_SHOW_ALWAYS.into();
        let records = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), ""));
        assert_eq!(records.len(), 11);
    }

    #[test]
    fn world_vortices_json_carries_store_vortices_when_brush_is_armed() {
        let fixture = forest_store();
        let runtime = Puzzle3dRuntime::default();
        assert_eq!(runtime.vortex_show, PUZZLE3D_VORTEX_SHOW_SELECTED);
        let brush = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "brush"));
        assert_eq!(brush.len(), 11);
        let volume = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "volumeBrush"));
        assert_eq!(volume.len(), 11);
    }

    #[test]
    fn world_vortices_json_stays_empty_in_selected_mode_without_a_touch_or_brush() {
        let fixture = forest_store();
        let runtime = Puzzle3dRuntime::default();
        let idle = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), ""));
        assert_eq!(idle.len(), 0);
        let transform = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "transform"));
        assert_eq!(transform.len(), 0);
    }

    /// 🖌️ The suggestion popup lists exactly the free candidates the brush suggestions run published for the
    /// popup's vortex, in candidate order, and reads pending only while that search has found nothing yet.
    #[test]
    fn the_suggestion_popup_lists_the_free_candidates_the_brush_run_published_for_its_vortex() {
        use crate::editor::puzzle3d::config::Puzzle3dSuggestionMenu;
        use crate::editor::puzzle3d::precompute::brush::{BrushSuggestionVerdict, BrushSuggestionsFound};
        use crate::standards::v1::subsets::any::schema::BrushPreviewState;
        let session = Puzzle3dPrecomputeSession::new();
        let runtime = Puzzle3dRuntime { suggestion_menu: Some(Puzzle3dSuggestionMenu { x: 1.0, y: 2.0, window_id: WINDOW_KIND_ID.into(), vortex_full_id: "seed-left-001:v0".into() }), ..Puzzle3dRuntime::default() };
        let envelope = Puzzle3dScene { fixture: forest_store(), runtime, active_utility: "select".into() };
        let preview = |kind: &str, source: usize| Some(BrushPreviewState { target_vortex_full_id: "seed-left-001:v0".into(), object_kind_id: kind.into(), source_vortex_index: source, mesh_url: "/box.glb".into(), origin: [0.0; 3], orientation: [0.0, 0.0, 0.0, 1.0], scale: None });
        let menu = |found: Option<&BrushSuggestionsFound>| serde_json::from_str::<Value>(&world_interaction_json(&envelope, &session, &Puzzle3dInteractionSnapshot::default(), found)).expect("interactionJson")["suggestionMenu"].clone();
        let mut found = BrushSuggestionsFound { writer: (1, 0), target: "seed-left-001:v0".into(), previews: vec![preview("A", 0), preview("B", 1), preview("C", 0)], verdicts: vec![BrushSuggestionVerdict::Collision, BrushSuggestionVerdict::Pending, BrushSuggestionVerdict::Pending], done: false };
        assert_eq!(menu(Some(&found))["pending"], json!(true), "nothing free yet while the search runs reads pending");
        found.verdicts[1] = BrushSuggestionVerdict::Free;
        let listed = menu(Some(&found));
        assert_eq!(listed["pending"], json!(false), "a free candidate is listed the moment it is known");
        assert_eq!(listed["candidates"].as_array().map(|rows| rows.iter().map(|row| row["objectLabel"].clone()).collect::<Vec<_>>()), Some(vec![json!("B")]));
        found.verdicts[2] = BrushSuggestionVerdict::Free;
        found.done = true;
        assert_eq!(menu(Some(&found))["candidates"].as_array().map(Vec::len), Some(2));
        let other = BrushSuggestionsFound { target: "seed-left-001:v1".into(), ..found.clone() };
        assert_eq!(menu(Some(&other))["candidates"], json!([]), "another vortex's candidates never reach this popup");
        assert_eq!(menu(None)["pending"], json!(true), "a popup whose run has not published yet is pending");
        assert_eq!(menu(Some(&BrushSuggestionsFound { verdicts: vec![BrushSuggestionVerdict::Collision; 3], ..found })), json!({ "open": true, "x": 1.0, "y": 2.0, "windowId": WINDOW_KIND_ID, "vortexFullId": "seed-left-001:v0", "pending": false, "candidates": [] }), "a finished search with nothing free is an empty, settled popup");
    }

    #[test]
    fn world_interaction_json_carries_no_fill_run_state() {
        let session = Puzzle3dPrecomputeSession::new();
        let envelope = Puzzle3dScene { fixture: forest_store(), runtime: Puzzle3dRuntime::default(), active_utility: "fill".into() };
        let value: Value = serde_json::from_str(&world_interaction_json(&envelope, &session, &Puzzle3dInteractionSnapshot::default(), None)).expect("interactionJson");
        for retired in ["fillBuild", "revealCutoffs", "brushPreviewJson"] {
            assert!(value.get(retired).is_none(), "{retired}: a fill run shows its process through the framework tool run lane and panel only");
        }
    }

    #[test]
    fn world_references_json_carries_infinite_asset_urls_the_dev_server_can_serve() {
        use crate::editor::puzzle3d::default_fixture;
        let json = world_references_json(&default_fixture());
        let records: Vec<Value> = serde_json::from_str(&json).expect("referencesJson");
        let masterarbeit = records.iter().find(|row| row["id"] == "ref-masterarbeit").expect("ref-masterarbeit");
        assert_eq!(
            masterarbeit["url"].as_str(),
            Some("/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg"),
            "masterarbeit reference must use the infinite-assets taxonomy path so WorldReferenceLayer can load the texture"
        );
        assert_eq!(masterarbeit["hidden"], json!(false));
    }

    #[test]
    fn meshes_json_is_published_in_exactly_the_mesh_lane_order_trace_subjects_index() {
        let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT).expect("example parses");
        let fixture = crate::editor::puzzle3d::puzzle3d_fixture_from_snapshot(&snapshot);
        let lane = mesh_lane(&fixture);
        assert_eq!(&lane[..2], [PUZZLE3D_FALLBACK_MESH_KIND, VORTEX_MARKER_MESH_KIND]);
        assert!(lane[2..].windows(2).all(|pair| pair[0] < pair[1]), "mesh urls follow in one stable sorted order");
        let meshes: Vec<Value> = serde_json::from_str(&world_meshes_json(&fixture)).expect("meshesJson");
        let published: Vec<String> = meshes.iter().map(|mesh| mesh.get("url").or_else(|| mesh.get("kind")).and_then(Value::as_str).expect("mesh identity").to_string()).collect();
        assert_eq!(published, lane, "a trace subject's mesh index must name meshesJson[index]");
        assert_eq!(mesh_lane(&fixture), lane, "the lane is deterministic across calls");
    }
