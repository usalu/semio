    /// 📚️ Kind catalog and free handle laws: the document kind catalogs translated into brush candidates, the manifest only documents and the fill without kind catalogs.

    /// 🗂️ Document catalogs (`meta.kindCatalogs.nodes`/`.handles`, every row carrying the document's
    /// mandatory `label`) must reach the engine through `board_kind_catalogs_json` and still produce
    /// brush candidates. Production used to push the document object verbatim, which installs
    /// nothing — the engine reads `nodeKinds`/`handleKinds` and rejects a `label`-bearing row — so
    /// `node_kinds` stayed empty and brush could never place anything.
    #[test]
    fn document_kind_catalogs_translate_into_engine_brush_candidates() {
        let snapshot = json!({
            "meta": {
                "kindCatalogs": {
                    "nodes": [{
                        "id": "brush.kind",
                        "name": "Brush Kind",
                        "label": "Brush Kind",
                        "description": "",
                        "icon": "",
                        "image": "",
                        "unit": "",
                        "abstract": false,
                        "baseKinds": [],
                        "representations": [],
                        "attributes": [],
                        "authors": [],
                        "handles": [{ "id": "t0", "name": "t0", "label": "T0", "description": "", "icon": "", "handleKind": "port", "angle": 3.141592653589793 }]
                    }],
                    "handles": [{ "id": "port", "label": "Port", "compatibleWith": [], "description": "", "icon": "", "color": "#888888", "defaultWireKind": "wire.link" }],
                    "edges": [],
                    "wires": []
                }
            }
        });
        let raw_candidates = brush_candidate_count_for_catalogs(&snapshot["meta"]["kindCatalogs"].to_string());
        assert_eq!(raw_candidates, 0, "pushing the document catalogs verbatim must install nothing, else this guard proves nothing");

        let catalogs = crate::editor::puzzle2d::snapshot::board_kind_catalogs_json(&snapshot).expect("snapshot carries meta.kindCatalogs");
        let translated_candidates = brush_candidate_count_for_catalogs(&catalogs);
        assert!(translated_candidates > 0, "translated document catalogs must yield brush candidates, got {translated_candidates}");
    }

    /// 🖌️ Drives a real brush hover over the one free handle of [`brush_single_free_handle_scene`] and
    /// returns how many candidates the engine offered. Counting candidates is the only sound signal:
    /// `brushPreview` fires even when `node_kinds` is empty — the engine previews the slot regardless
    /// — so asserting on the preview event would pass with the bug still present.
    fn brush_candidate_count_for_catalogs(catalogs_json: &str) -> usize {
        let mut host = BoardHost::new();
        host.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut host.scene, &mut host.outbox, 0.0, 0.0, 1.0);
        host.set_active_utility("brush");
        host.set_suggestion_offset(40.0);
        host.set_brush_node_size(40.0);
        host.style.set_board_kind_catalogs_from_json(catalogs_json).expect("catalog push must be accepted by the engine");
        host.sync_descriptor(&brush_single_free_handle_scene()).unwrap();
        let _ = host.outbox.drain_events_json();
        let anchor = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = host.scene.world_to_screen(anchor + (anchor - Point::new(0.0, 0.0)));
        host.pointer_move_screen(probe.x, probe.y, false, false, false);
        let events: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&host.outbox.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("board events json");
        events
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(semio_framework_pack_json::Value::as_str) == Some("brushCandidates")))
            .and_then(|row| row.get("payload"))
            .and_then(|payload| payload.get("candidates"))
            .and_then(semio_framework_pack_json::Value::as_array)
            .map_or(0, Vec::len)
    }

    /// 🗂️ Every shipped puzzle2d document carries `meta.manifestId` and **no** `meta.kindCatalogs`, so
    /// the engine's catalogs have to resolve from the manifest registry. Production had no such path —
    /// only the test-only `catalogs_json_from_manifest_id` — which left `node_kinds` empty for exactly
    /// the documents the app loads, so translating `meta.kindCatalogs` alone would have fixed nothing.
    #[test]
    fn manifest_only_documents_resolve_engine_kind_catalogs() {
        let snapshot = json!({ "meta": { "manifestId": "nakagin" } });
        let catalogs = crate::editor::puzzle2d::snapshot::board_kind_catalogs_json(&snapshot).expect("nakagin manifest catalogs");
        let parsed: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&catalogs, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("catalog json");
        let node_kinds = parsed["nodeKinds"].as_array().expect("nodeKinds slice");
        assert!(!node_kinds.is_empty(), "the nakagin manifest must contribute node kinds");
        assert!(node_kinds.iter().any(|row| row.get("handles").and_then(semio_framework_pack_json::Value::as_array).is_some_and(|handles| !handles.is_empty())), "manifest node kinds must carry handle templates, else brush_compatible_candidates skips every kind");
        BoardHost::new().style.set_board_kind_catalogs_from_json(&catalogs).expect("manifest catalogs must satisfy the engine contract");
    }

    /// 🌾️ Proves the fill half of the catalog-translation fix: an empty engine `node_kinds` catalog
    /// accepts nothing from `run_fill_job`, and the identical scene fills once
    /// `board_kind_catalogs_json` translates document-shaped `meta.kindCatalogs` into the engine's
    /// `nodeKinds`/`handleKinds` shape. Mirrors [`document_kind_catalogs_translate_into_engine_brush_candidates`]
    /// but drives the fill job instead of a single brush hover.
    #[test]
    fn fill_places_nothing_without_kind_catalogs_and_places_with_them() {
        let mut empty_host = BoardHost::new();
        empty_host.scene.set_size(800, 600, 1.0);
        empty_host.set_suggestion_offset(40.0);
        empty_host.set_brush_node_size(40.0);
        empty_host.sync_descriptor(&brush_single_free_handle_scene()).unwrap();
        let empty_operation = Operation::new(semio_framework_job::OperationId(501), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 501);
        let (empty_placements, _, empty_result) = run_fill_job(&empty_host, 1, empty_operation, 1);
        assert_eq!(empty_result.accepted_count, 0, "fill must accept nothing without engine kind catalogs");
        assert!(empty_placements.is_empty(), "fill must place nothing without engine kind catalogs");

        let snapshot = json!({
            "meta": {
                "kindCatalogs": {
                    "nodes": [{
                        "id": "brush.kind",
                        "name": "Brush Kind",
                        "label": "Brush Kind",
                        "description": "",
                        "icon": "",
                        "image": "",
                        "unit": "",
                        "abstract": false,
                        "baseKinds": [],
                        "representations": [],
                        "attributes": [],
                        "authors": [],
                        "handles": [{ "id": "t0", "name": "t0", "label": "T0", "description": "", "icon": "", "handleKind": "port", "angle": 3.141592653589793 }]
                    }],
                    "handles": [{ "id": "port", "label": "Port", "compatibleWith": [], "description": "", "icon": "", "color": "#888888", "defaultWireKind": "wire.link" }],
                    "edges": [],
                    "wires": []
                }
            }
        });
        let catalogs = crate::editor::puzzle2d::snapshot::board_kind_catalogs_json(&snapshot).expect("snapshot carries meta.kindCatalogs");
        let mut catalog_host = BoardHost::new();
        catalog_host.scene.set_size(800, 600, 1.0);
        catalog_host.set_suggestion_offset(40.0);
        catalog_host.set_brush_node_size(40.0);
        catalog_host.style.set_board_kind_catalogs_from_json(&catalogs).unwrap();
        catalog_host.sync_descriptor(&brush_single_free_handle_scene()).unwrap();
        let catalog_operation = Operation::new(semio_framework_job::OperationId(502), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 502);
        let (catalog_placements, _, catalog_result) = run_fill_job(&catalog_host, 1, catalog_operation, 1);
        assert!(catalog_result.accepted_count > 0, "fill must accept at least one placement once engine kind catalogs are populated");
        assert!(!catalog_placements.is_empty(), "fill must place at least one node once engine kind catalogs are populated");
    }

    /// 🎲️ One circle node carrying a single free `port` handle at angle 0 — the minimal scene a brush
    /// slot can open on.
    fn brush_single_free_handle_scene() -> SceneDescriptor {
        SceneDescriptor {
            nodes: vec![NodeDescriptor {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }].into(),
            handles: vec![HandleDescriptor {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }
