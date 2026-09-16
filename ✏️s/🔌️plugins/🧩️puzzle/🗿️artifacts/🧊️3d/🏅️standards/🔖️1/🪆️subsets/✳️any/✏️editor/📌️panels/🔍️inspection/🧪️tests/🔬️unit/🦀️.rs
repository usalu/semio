
    use super::*;
    use crate::editor::puzzle3d::config::Puzzle3dRuntime;
    use crate::editor::puzzle3d::terminology::puzzle3d_labels;
    use crate::editor::puzzle3d::{empty_fixture, Puzzle3dObject, Puzzle3dScene, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_VORTEX};
    use semio_framework_plugin::{TreeWindowRequest, ViewModel};

    fn labels() -> &'static Puzzle3dLabels {
        puzzle3d_labels(&ViewModel { terminology: semio_framework_plugin::Terminology::Native, ..Default::default() }).expect("admitted host axis")
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

    /// 🪟️ One host window request for this body.
    fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
        TreeWindowRequest { body_key: BODY_KEY.to_string(), node_key: node_key.to_string(), open, offset, rows }
    }

    fn hosted(requests: Vec<TreeWindowRequest>) -> ViewModel {
        ViewModel { tree_windows: requests, ..Default::default() }
    }

    fn scene(ids: &[String]) -> (Puzzle3dScene, Puzzle3dInteractionSnapshot) {
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
            })
            .collect();
        let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime::default(), active_utility: String::new() };
        let interaction = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: ids.to_vec(), hovered: Vec::new() };
        (scene, interaction)
    }

    fn inspect(scene: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot, view: &ViewModel) -> BuiltNode {
        drain();
        render(scene, interaction, labels(), &TreeWindows::for_body(view, BODY_KEY)).expect("inspector body")
    }

    fn json_of(node: BuiltNode) -> String {
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("inspector projection");
        drain();
        json
    }

    fn tree_of(json: &str) -> serde_json::Value {
        serde_json::from_str(json).expect("the inspector projection is JSON")
    }

    fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node);
        }
        node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
    }

    fn child_keys(node: &serde_json::Value) -> Vec<String> {
        node["children"].as_array().map(|children| children.iter().filter_map(|child| child["key"].as_str().map(str::to_owned)).collect()).unwrap_or_default()
    }

    fn window_of(node: &serde_json::Value) -> (usize, usize) {
        let window = &node["component"]["window"];
        let total = window["total"].as_u64().unwrap_or_else(|| panic!("a container stamps its window: {node}"));
        (total as usize, window["offset"].as_u64().unwrap_or(0) as usize)
    }

    fn unhosted_body(scene: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> BuiltNode {
        drain();
        render(scene, interaction, labels(), &TreeWindows::unhosted()).expect("inspector body")
    }

    #[test]
    fn leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row() {
        let (scene, mut interaction) = scene(&["seed-left-001".into()]);
        interaction.granularity.clear();
        let node = unhosted_body(&scene, &interaction);
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
        let (scene, mut interaction) = scene(&["seed-left-001".into()]);
        interaction.granularity.clear();
        interaction.selected = vec!["seed-left-001".into()];
        let node = unhosted_body(&scene, &interaction);
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
        let (scene, mut interaction) = scene(&["seed-left-001".into()]);
        interaction.granularity = PUZZLE3D_GRANULARITY_VORTEX.into();
        let node = unhosted_body(&scene, &interaction);
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
        let (mut scene, mut interaction) = scene(&["seed-left-001".into()]);
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
        let node = unhosted_body(&scene, &interaction);
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
        let (scene, mut interaction) = scene(&["seed-left-001".into()]);
        interaction.granularity.clear();
        interaction.selected = vec!["seed-left-001".into()];
        interaction.hovered.clear();
        let node = unhosted_body(&scene, &interaction);
        let mut rows = Vec::new();
        keys(&node, &mut rows);
        assert!(rows.iter().any(|key| key.ends_with("object.id")), "leftover.ids must render object.id: {rows:?}");
        assert!(rows.iter().any(|key| key.ends_with("object.locked")), "leftover.ids must assemble object.locked flag_row: {rows:?}");
        assert!(!rows.iter().any(|key| key.ends_with(".empty")), "leftover.ids must not keep the empty summary: {rows:?}");
        drop(node);
        drain();
    }

    //#region 🪟️WindowLaws
    /// 🪟️ A wide selection's id list is a WINDOW, not a truncation: the section stamps every selected
    /// id and materialises only the host's slice, keyed by the raw id. The old build capped at
    /// `IDS_ROWS` and closed with a `+N` whose `setPanelPage` cursor died with the command.
    #[test]
    fn a_wide_selection_stamps_every_id_and_materialises_only_its_window() {
        let ids: Vec<String> = (0..120).map(|index| format!("object-{index}")).collect();
        let (scene, interaction) = scene(&ids);
        let view = hosted(vec![request(IDS_SECTION, Some(true), 0, 8)]);
        let json = json_of(inspect(&scene, &interaction, &view));
        let tree = tree_of(&json);
        let section = node_at(&tree, IDS_SECTION).expect("the ids section");
        assert_eq!(window_of(section), (ids.len(), 0), "the ids section reports the WHOLE selection: {section}");
        let expected: Vec<String> = (0..8).map(|index| format!("{IDS_SECTION}.object-{index}")).collect();
        assert_eq!(child_keys(section), expected, "exactly the eight rows the host asked for, keyed by the raw id: {section}");
        assert!(node_at(&tree, "puzzle3d-play-inspector.object.id").is_some(), "the entity's own fields are still assembled beside the id window: {tree}");
        assert!(!json.contains(".more"), "no continuation row survives anywhere: {json}");
        assert!(!json.contains("\"+"), "and no `+N` label either: {json}");
    }

    /// 🪟️ Collapsed, the id list costs one stamped `total` and no row at all.
    #[test]
    fn a_closed_id_section_stamps_its_total_and_materialises_no_child() {
        let ids: Vec<String> = (0..40).map(|index| format!("object-{index}")).collect();
        let (scene, interaction) = scene(&ids);
        let view = hosted(vec![request(IDS_SECTION, Some(false), 0, 64)]);
        let tree = tree_of(&json_of(inspect(&scene, &interaction, &view)));
        let section = node_at(&tree, IDS_SECTION).expect("the ids section");
        assert_eq!(window_of(section), (ids.len(), 0), "a closed ids section still reports the whole selection: {section}");
        assert!(child_keys(section).is_empty(), "and materialises none of it: {section}");
    }

    /// 🪟️ Scrolling the id list resolves to exactly `[offset, offset + rows)`, keyed by the raw ids —
    /// never renumbered by the offset the way the old `ids.{index}` keys were.
    #[test]
    fn a_tree_window_request_materialises_exactly_its_slice_of_the_id_list() {
        let ids: Vec<String> = (0..90).map(|index| format!("object-{index}")).collect();
        let (scene, interaction) = scene(&ids);
        let view = hosted(vec![request(IDS_SECTION, Some(true), 60, 3)]);
        let tree = tree_of(&json_of(inspect(&scene, &interaction, &view)));
        let section = node_at(&tree, IDS_SECTION).expect("the ids section");
        assert_eq!(window_of(section), (90, 60), "the section reports all 90 ids and where the slice starts: {section}");
        let expected: Vec<String> = (60..63).map(|index| format!("{IDS_SECTION}.object-{index}")).collect();
        assert_eq!(child_keys(section), expected, "exactly entries [60, 63) are built: {section}");
    }

    /// 🪟️ A selection of exactly one still gets its id window — and an entity with no id list at all
    /// (a vortex) gets no ids section, so the inspector never shows an empty header.
    #[test]
    fn the_ids_section_appears_only_when_the_selection_names_ids() {
        let (one_scene, one_interaction) = scene(&["object-0".into()]);
        let tree = tree_of(&json_of(unhosted_body(&one_scene, &one_interaction)));
        assert_eq!(window_of(node_at(&tree, IDS_SECTION).expect("the ids section")), (1, 0), "{tree}");

        let (mut vortex_scene, mut vortex_interaction) = scene(&["object-0".into()]);
        vortex_scene.fixture.objects[0].vortices.push(Puzzle3dVortex { id: "vortex-0".into(), vortex_kind: Some("edge".into()), ..Default::default() });
        vortex_interaction.granularity = PUZZLE3D_GRANULARITY_VORTEX.into();
        vortex_interaction.selected = vec!["object-0:vortex-0".into()];
        let tree = tree_of(&json_of(unhosted_body(&vortex_scene, &vortex_interaction)));
        assert!(node_at(&tree, IDS_SECTION).is_none(), "a vortex selection carries no id list, so no ids section is authored: {tree}");
        assert!(node_at(&tree, "puzzle3d-play-inspector.vortex.full-id").is_some(), "but the vortex's own fields are: {tree}");
    }
    //#endregion 🪟️WindowLaws
