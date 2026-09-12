
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
            reveal_index: None,
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

    #[test]
    fn brush_preview_target_falls_back_to_session_live_target() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.set_brush_live_target(Some("seed-left-001:v0".into()));
        let envelope = Puzzle3dScene { fixture: forest_store(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
        assert_eq!(world_brush_preview_target(&session, &envelope, &Puzzle3dInteractionSnapshot::default()).as_deref(), Some("seed-left-001:v0"));
    }
