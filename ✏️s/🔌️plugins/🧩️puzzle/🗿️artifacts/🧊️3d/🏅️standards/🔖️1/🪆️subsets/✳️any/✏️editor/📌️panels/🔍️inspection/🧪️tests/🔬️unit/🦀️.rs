
    use super::*;
    use crate::editor::puzzle3d::terminology::puzzle3d_labels;
    use crate::editor::puzzle3d::config::Puzzle3dRuntime;
    use crate::editor::puzzle3d::{empty_fixture, Puzzle3dObject, Puzzle3dScene, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_VORTEX};

    fn labels() -> &'static Puzzle3dLabels {
        puzzle3d_labels(&semio_framework_plugin::ViewModel { terminology: semio_framework_plugin::Terminology::Native, ..Default::default() }).expect("admitted host axis")
    }

    fn drain() {
        for _ in 0..4096 {
            if semio_framework_ui_contract::close_built_node_page_one() {
                break;
            }
        }
        for _ in 0..4096 {
            if semio_framework_ui_contract::close_ui_value_page_one() {
                break;
            }
        }
    }

    fn keys(node: &BuiltNode, out: &mut Vec<String>) {
        out.push(node.key.as_str().to_string());
        for child in node.children.iter() {
            keys(child, out);
        }
    }

    fn scene(ids: &[String], page: u32) -> (Puzzle3dScene, Puzzle3dInteractionSnapshot) {
        let mut fixture = empty_fixture();
        fixture.objects = ids
            .iter()
            .map(|id| Puzzle3dObject {
                id: id.clone(),
                label: None,
                object_kind: Some("Object".into()),
                origin: [0.0, 0.0, 0.0],
                orientation: None,
                scale: None,
                mesh_url: None,
                vortices: Vec::new(),
                hidden: false,
                locked: false,
                reveal_index: None,
            })
            .collect();
        let mut runtime = Puzzle3dRuntime::default();
        if page > 0 {
            runtime.panel_pages.insert(IDS_SECTION.to_string(), page);
        }
        let scene = Puzzle3dScene { fixture, runtime, active_utility: String::new() };
        let interaction = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: ids.to_vec(), hovered: Vec::new() };
        (scene, interaction)
    }

    #[test]
    fn leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row() {
        drain();
        let (scene, mut interaction) = scene(&["seed-left-001".into()], 0);
        interaction.granularity.clear();
        let node = render(&scene, &interaction, labels()).expect("leftover inspect");
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(
            rows.iter().any(|key| key == "puzzle3d-play-inspector.object.id" || key.ends_with("puzzle3d-play-inspector.object.id")),
            "leftover-only selection must render namespaced object.id: {rows:?}"
        );
        assert!(
            rows.iter().any(|key| key == "puzzle3d-play-inspector.object.locked" || key.ends_with("puzzle3d-play-inspector.object.locked")),
            "leftover-only selection must assemble namespaced object.locked flag_row: {rows:?}"
        );
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "leftover selectedIds must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }

    #[test]
    fn leftover_ids_overlay_when_vortex_selection_empty_wires_object_fields_and_lock_row() {
        drain();
        let (scene, mut interaction) = scene(&["seed-left-001".into()], 0);
        interaction.granularity.clear();
        interaction.selected = vec!["seed-left-001".into()];
        let node = render(&scene, &interaction, labels()).expect("leftover.ids inspect");
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(
            rows.iter().any(|key| key == "puzzle3d-play-inspector.object.id" || key.ends_with("puzzle3d-play-inspector.object.id")),
            "leftover.ids must render namespaced object.id: {rows:?}"
        );
        assert!(
            rows.iter().any(|key| key == "puzzle3d-play-inspector.object.locked" || key.ends_with("puzzle3d-play-inspector.object.locked")),
            "leftover.ids must assemble namespaced object.locked flag_row: {rows:?}"
        );
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "leftover.ids must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }

    #[test]
    fn leftover_vortex_granularity_unresolved_falls_back_to_object_fields() {
        drain();
        let (scene, mut interaction) = scene(&["seed-left-001".into()], 0);
        interaction.granularity = PUZZLE3D_GRANULARITY_VORTEX.into();
        let node = render(&scene, &interaction, labels()).expect("vortex-mismatch inspect");
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(rows.iter().any(|key| key.ends_with("object.id")), "unresolved vortex granularity must fall back to object.id: {rows:?}");
        assert!(rows.iter().any(|key| key.ends_with("object.locked")), "unresolved vortex granularity must assemble object.locked: {rows:?}");
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "unresolved vortex granularity must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }

    #[test]
    fn leftover_selected_vortex_uuid_falls_through_to_object_fields() {
        drain();
        let (mut scene, mut interaction) = scene(&["seed-left-001".into()], 0);
        scene.fixture.objects[0].vortices.push(Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        });
        interaction.granularity = PUZZLE3D_GRANULARITY_VORTEX.into();
        interaction.selected = vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()];
        let node = render(&scene, &interaction, labels()).expect("vortex-uuid inspect");
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(rows.iter().any(|key| key.ends_with("object.id")), "leftover selected vortex uuid must fall through to object.id: {rows:?}");
        assert!(rows.iter().any(|key| key.ends_with("object.locked")), "leftover selected vortex uuid must assemble object.locked: {rows:?}");
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "leftover selected vortex uuid must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }


    #[test]
    fn leftover_ids_name_object_empty_persist_vortex_paints_object_fields_and_lock_row() {
        drain();
        let (scene, mut interaction) = scene(&["seed-left-001".into()], 0);
        interaction.granularity.clear();
        interaction.selected = vec!["seed-left-001".into()];
        interaction.hovered.clear();
        let node = render(&scene, &interaction, labels()).expect("leftover.ids empty persist vortex");
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(rows.iter().any(|key| key.ends_with("object.id")), "leftover.ids must render object.id: {rows:?}");
        assert!(rows.iter().any(|key| key.ends_with("object.locked")), "leftover.ids must assemble object.locked flag_row: {rows:?}");
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "leftover.ids must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }

    #[test]
    fn inspection_ids_page_is_bounded_and_the_continuation_advances() {
        let ids: Vec<String> = (0..IDS_ROWS * 2 + 3).map(|index| format!("object-{index}")).collect();
        drain();
        let (first_scene, first_interaction) = scene(&ids, 0);
        let first = render(&first_scene, &first_interaction, labels()).expect("page 0");
        let mut rows = Vec::new();
        keys(&first, &mut rows);
        let id_rows = rows.iter().filter(|key| key.contains(".ids.") && !key.ends_with(".more")).count();
        assert!(id_rows <= IDS_ROWS, "the inspector must page ids, got {id_rows} id rows: {rows:?}");
        assert!(rows.iter().any(|key| key.ends_with(".ids.more")), "page 0 must offer +N: {rows:?}");
        drop(first);
        drain();
        let (next_scene, next_interaction) = scene(&ids, 1);
        let next = render(&next_scene, &next_interaction, labels()).expect("page 1");
        rows.clear();
        keys(&next, &mut rows);
        assert!(rows.iter().any(|key| key.ends_with(".ids.more")), "page 1 still has a further page: {rows:?}");
        drop(next);
        drain();
        let last = ((ids.len() - 1) / IDS_ROWS) as u32;
        let (tail_scene, tail_interaction) = scene(&ids, last);
        let tail = render(&tail_scene, &tail_interaction, labels()).expect("last page");
        rows.clear();
        keys(&tail, &mut rows);
        assert!(!rows.iter().any(|key| key.ends_with(".ids.more")), "the last ids page must drop +N: {rows:?}");
        drop(tail);
        drain();
    }
