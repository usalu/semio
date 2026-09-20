use super::*;

/// 🗂️ A configured `fileNodeKinds[kindId].icon` must win over the kind-name fallback table —
/// previously `vfs_glyph_icon` only checked `.is_some()` and always returned `"folder"` for any
/// kind with a custom icon configured, discarding the actual icon id.
#[test]
fn configured_kind_icon_is_used_verbatim_not_collapsed_to_folder() {
    let schema: VfsSchema = serde_json::from_str(r#"{"fileNodeKinds":{"asset":{"icon":"box"}}}"#).unwrap();
    let row = json!({ "fileNodeKindId": "asset" });
    assert_eq!(vfs_glyph_icon(&schema, &row), "box");
}

#[test]
fn folder_and_instance_kinds_fall_back_to_their_built_in_glyphs() {
    let schema: VfsSchema = serde_json::from_str("{}").unwrap();
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "folder" })), "folder");
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "instance" })), "box");
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "other" })), "file-text");
}

#[test]
fn missing_file_node_kind_id_defaults_to_the_file_kind() {
    let schema: VfsSchema = serde_json::from_str("{}").unwrap();
    assert_eq!(vfs_glyph_icon(&schema, &json!({})), "file-text");
}

//#region VirtualFileSystemPointerTests
fn vfs_scene(surface_id: &str, rows: Value) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::VirtualFileSystem,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: Some(ui_wgpu::wgpu::VirtualFileSystemScene { schema_json: "{}".into(), rows_json: rows.to_string(), selected_row_ids_json: None, hovered_row_id: None, empty_message: None, drag_drop_enabled: None }),
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

fn vfs_row_center_y(index: usize) -> f32 {
    let theme = Theme::default();
    theme.control_height * 1.33 + theme.control_height * (index as f32 + 0.5)
}

/// 🗂️ `VirtualFileSystemHost`'s `onSelectionChange` sends `selectRows` with `{ surfaceId, ids }`.
#[test]
fn row_press_dispatches_select_rows_with_the_row_id() {
    let node = vfs_scene("vfs-press-rows", json!([{ "id": "n1", "name": "Alpha" }, { "id": "n2", "name": "Beta" }]));
    let theme = Theme::default();
    let hit = vfs_hit(&node, Rect::new(0.0, 0.0, 400.0, 300.0), 120.0, vfs_row_center_y(1), &theme, true, SceneModifiers::default()).expect("row hit");
    assert_eq!(hit.control_id, "vfs-press-rows.vfs.n2");
    let action = hit.action.expect("selectRows action");
    assert_eq!(action.action, "selectRows");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("surfaceId").and_then(semio_framework::DslValue::as_str), Some("vfs-press-rows"));
    let ids = args.get("ids").and_then(|ids| ids.as_array().map(|entries| entries.iter().filter_map(semio_framework::DslValue::as_str).map(str::to_string).collect::<Vec<_>>())).expect("ids array");
    assert_eq!(ids, vec!["n2".to_string()]);
}

/// 📁️ A press on an expandable row's chevron toggles renderer-local expansion instead of dispatching.
#[test]
fn chevron_press_toggles_expansion_and_dispatches_nothing() {
    let node = vfs_scene("vfs-press-chevron", json!([{ "id": "n1", "name": "Folder", "hasChildren": true }, { "id": "n2", "name": "Child", "parentId": "n1" }]));
    let theme = Theme::default();
    let hit = vfs_hit(&node, Rect::new(0.0, 0.0, 400.0, 300.0), theme.padding_standard + 4.0, vfs_row_center_y(0), &theme, true, SceneModifiers::default()).expect("chevron hit");
    assert_eq!(hit.toggle_expanded.as_deref(), Some("n1"));
    assert!(hit.action.is_none());
}

/// 🚪️ A double-click on a row whose `navigateUri` is an instance uri opens that instance.
#[test]
fn double_click_on_an_instance_row_opens_the_instance() {
    let node = vfs_scene("vfs-press-open", json!([{ "id": "n1", "name": "Alpha", "navigateUri": "os://instance/inst-7" }]));
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let y = vfs_row_center_y(0);
    assert!(scene_double_click_action(&node, bounds, 120.0, y).is_none(), "the first press only arms the double-click window");
    let action = scene_double_click_action(&node, bounds, 120.0, y).expect("openInstance on the repeat press");
    assert_eq!(action.action, "openInstance");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("instanceId")).and_then(semio_framework::DslValue::as_str), Some("inst-7"));
}
//#endregion VirtualFileSystemPointerTests

//#region VirtualFileSystemModifierTests
/// 🗂️ React `⚙️VirtualFileSystem/🟦️.tsx:518-521` folds the DOM pointer event into
/// `additiveKey: event.metaKey || event.ctrlKey` and `rangeKey: event.shiftKey` before
/// `getVirtualFileSystemNextSelectionState` runs. `vfs_selection_for_click` is that function's twin,
/// and until `UiEvent`'s pointer variants carried `EventModifiers` this call site could only pass
/// `(false, false)` — so ctrl-toggle and shift-extend were unreachable, not merely unbound
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1).
#[test]
fn ctrl_click_extends_the_selection_where_a_plain_click_replaces_it() {
    let node = vfs_scene("vfs-modifier-ctrl", json!([{ "id": "n1", "name": "Alpha" }, { "id": "n2", "name": "Beta" }]));
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let ids_of = |hit: SceneListHit| {
        hit.action.expect("selectRows").args.as_ref().and_then(|args| args.get("ids")).and_then(|ids| ids.as_array().map(|entries| entries.iter().filter_map(semio_framework::DslValue::as_str).map(str::to_string).collect::<Vec<_>>())).expect("ids")
    };
    let first = vfs_hit(&node, bounds, 120.0, vfs_row_center_y(0), &theme, true, SceneModifiers::default()).expect("row 0");
    assert_eq!(ids_of(first), vec!["n1".to_string()]);
    let additive = vfs_hit(&node, bounds, 120.0, vfs_row_center_y(1), &theme, true, SceneModifiers { ctrl: true, ..SceneModifiers::default() }).expect("row 1");
    assert_eq!(ids_of(additive), vec!["n1".to_string(), "n2".to_string()], "ctrl adds to the live selection instead of replacing it");
    let plain = vfs_hit(&node, bounds, 120.0, vfs_row_center_y(0), &theme, true, SceneModifiers::default()).expect("row 0 again");
    assert_eq!(ids_of(plain), vec!["n1".to_string()], "an unmodified click still replaces");
}

/// 🍎️ React reads `event.metaKey || event.ctrlKey` as ONE additive key, so cmd and ctrl are the same
/// gesture; `alt` is neither additive nor a range on this surface.
#[test]
fn the_additive_key_is_meta_or_ctrl_and_nothing_else() {
    assert!(SceneModifiers { meta: true, ..SceneModifiers::default() }.additive());
    assert!(SceneModifiers { ctrl: true, ..SceneModifiers::default() }.additive());
    assert!(!SceneModifiers { shift: true, ..SceneModifiers::default() }.additive());
    assert!(!SceneModifiers { alt: true, ..SceneModifiers::default() }.additive());
}
//#endregion VirtualFileSystemModifierTests
