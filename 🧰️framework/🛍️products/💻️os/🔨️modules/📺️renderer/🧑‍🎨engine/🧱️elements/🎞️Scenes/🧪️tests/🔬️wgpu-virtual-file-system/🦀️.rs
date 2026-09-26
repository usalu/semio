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
        host_id: surface_id.into(),
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

/// 🧬️ Raw scene rows, renderer-local expansion, and navigation routes share one neutral law with React.
#[test]
fn raw_tree_expansion_and_navigation_match_the_shared_react_law() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/📁️virtual-file-system-interaction/🔣️.json"))).expect("shared VFS interaction fixture");
    let rows = fixture["rows"].as_array().expect("rows");
    for vector in fixture["visibility"].as_array().expect("visibility") {
        let expanded: BTreeSet<String> = vector["expandedRowIds"].as_array().expect("expanded ids").iter().filter_map(Value::as_str).map(str::to_string).collect();
        let visible = build_vfs_visible_rows(rows, &expanded);
        let actual_ids: Vec<String> = visible.iter().map(|entry| vfs_row_id(&entry.row)).collect();
        let actual_levels: Vec<u32> = visible.iter().map(|entry| entry.level).collect();
        let expected_ids: Vec<String> = vector["visibleRowIds"].as_array().expect("visible ids").iter().filter_map(Value::as_str).map(str::to_string).collect();
        let expected_levels: Vec<u32> = vector["levels"].as_array().expect("levels").iter().filter_map(Value::as_u64).map(|level| level as u32).collect();
        assert_eq!(actual_ids, expected_ids, "{}", vector["id"]);
        assert_eq!(actual_levels, expected_levels, "{}", vector["id"]);
    }
    let node = vfs_scene("vfs-shared-law", Value::Array(rows.clone()));
    for vector in fixture["navigation"].as_array().expect("navigation") {
        let row_id = vector["rowId"].as_str().expect("row id");
        let row = rows.iter().find(|row| row.get("id").and_then(Value::as_str) == Some(row_id)).expect("fixture row");
        let actual = vfs_double_click_action(&node, row);
        let Some(expected_action) = vector["action"].as_str() else {
            assert!(actual.is_none(), "{row_id}");
            continue;
        };
        let action = actual.expect("navigation action");
        assert_eq!(action.action, expected_action, "{row_id}");
        for (key, expected) in vector["args"].as_object().expect("args") {
            assert_eq!(action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework::DslValue::as_str), expected.as_str(), "{row_id}:{key}");
        }
    }
}

#[test]
fn accepted_vfs_controls_match_the_shared_localized_keyboard_law() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/📁️virtual-file-system-interaction/🔣️.json"))).expect("shared VFS interaction fixture");
    let german = fixture["chrome"].as_array().expect("chrome packs").iter().find(|pack| pack["locale"] == "de").expect("German pack");
    let host_id = "vfs-accepted-controls";
    let row = VfsAccessibilityControl { key: format!("{host_id}.vfs.folder"), row_id: "folder".into(), label: "Models".into(), kind: VfsAccessibilityControlKind::Row, rect: Rect::new(0.0, 40.0, 320.0, 24.0), expanded: None };
    let chevron = VfsAccessibilityControl {
        key: format!("{host_id}.vfs.chevron.folder"),
        row_id: "folder".into(),
        label: german["collapse"].as_str().expect("collapse label").into(),
        kind: VfsAccessibilityControlKind::Chevron,
        rect: Rect::new(8.0, 40.0, 14.0, 24.0),
        expanded: Some(true),
    };
    stage_vfs_accessibility_controls(host_id, vec![row.clone(), chevron.clone()]);
    seal_vfs_accessibility_candidates(77);
    acknowledge_vfs_accessibility_candidates(77);
    assert_eq!(accepted_vfs_accessibility_controls(host_id), vec![row, chevron]);
    let scene = vfs_scene(host_id, fixture["rows"].clone());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for law in fixture["keyboard"].as_array().expect("keyboard laws") {
        let kind = if law["control"] == "chevron" { VfsAccessibilityControlKind::Chevron } else { VfsAccessibilityControlKind::Row };
        let key = if law["key"] == "enter" { ui_wgpu::wgpu::KeyAction::Enter } else { ui_wgpu::wgpu::KeyAction::Space(true) };
        let target = VfsAccessibilityFocusTarget { row_id: "folder".into(), kind };
        let outcome = vfs_accessibility_apply_key(&scene, &target, &key, &mut input).expect("accepted control").expect("bounded action");
        let expected = if law["result"] == "toggle" { VfsAccessibilityKeyOutcome::Consumed } else { VfsAccessibilityKeyOutcome::Unhandled };
        assert_eq!(outcome, expected, "{} {}", law["control"], law["key"]);
    }
    let row_target = VfsAccessibilityFocusTarget { row_id: "folder".into(), kind: VfsAccessibilityControlKind::Row };
    vfs_accessibility_activate(&scene, &row_target, &mut input).expect("accepted row").expect("selection action");
    let action = input.take_action_step().expect("action authority").expect("selectRows").into_descriptor().expect("descriptor");
    assert_eq!(action.action, "selectRows");
    assert!(input.take_action_step().expect("action authority").is_none(), "row activation never invents the double-click navigation action");
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

/// 🧾️ Descriptor tags and avatar fallback text share the React component fixture.
#[test]
fn vfs_descriptor_cells_decode_the_actual_react_tagged_union() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧫️fixtures/🧾️descriptors/🔣️.json"))).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let schema: VfsSchema = serde_json::from_value(json!({
            "fileNodeKinds": { "file": { "descriptors": [{ "id": "cell", "descriptorKindId": law["kind"] }] }, "other": { "descriptors": [] } },
            "descriptorKinds": fixture["kinds"], "descriptorColumnIds": ["cell"]
        }))
        .unwrap();
        let mut row = json!({ "fileNodeKindId": "file", "descriptorValues": { "cell": law["value"] } });
        let text = match vfs_descriptor_value(&schema, &row, "cell") {
            Some(VfsDescriptorValue::Text(text)) => text.to_string(),
            Some(VfsDescriptorValue::Time { iso, .. }) => iso.to_string(),
            Some(VfsDescriptorValue::Avatar { name, icon }) => {
                assert_eq!(name.trim(), law["expect"]["name"].as_str().unwrap());
                assert_eq!(icon, law["expect"]["icon"].as_str());
                vfs_avatar_initials(name)
            }
            None => String::new(),
        };
        assert_eq!(text, law["expect"]["text"].as_str().unwrap(), "{}", law["id"]);
        row["fileNodeKindId"] = json!("other");
        assert!(vfs_descriptor_value(&schema, &row, "cell").is_none(), "a row cannot publish a column undeclared by its kind");
        assert_eq!(vfs_descriptor_label(&schema, "cell"), fixture["kinds"][law["kind"].as_str().unwrap()]["name"].as_str().unwrap());
    }
}

#[test]
fn vfs_descriptor_binding_preserves_the_authored_kind_order() {
    let schema: VfsSchema = serde_json::from_str(r#"{"fileNodeKinds":{"z":{"descriptors":[{"id":"column","descriptorKindId":"text","label":"First"}]},"a":{"descriptors":[{"id":"column","descriptorKindId":"text","label":"Second"}]}},"descriptorKinds":{"text":{"presentation":"text","name":"Text"}}}"#).unwrap();
    assert_eq!(vfs_descriptor_label(&schema, "column"), "First");
}

#[test]
fn vfs_visible_time_cells_form_one_generic_host_temporal_batch() {
    let schema: VfsSchema = serde_json::from_value(json!({
        "descriptorColumnIds": ["created", "updated", "age"],
        "descriptorKinds": {
            "date": { "presentation": "time", "name": "Created", "format": "date" },
            "datetime": { "presentation": "time", "name": "Updated", "format": "datetime" },
            "relative": { "presentation": "time", "name": "Age", "format": "relative" }
        },
        "fileNodeKinds": {
            "file": { "descriptors": [
                { "id": "created", "descriptorKindId": "date" },
                { "id": "updated", "descriptorKindId": "datetime" },
                { "id": "age", "descriptorKindId": "relative" }
            ] }
        }
    }))
    .unwrap();
    let rows = vec![json!({
        "id": "file-1",
        "fileNodeKindId": "file",
        "descriptorValues": {
            "created": { "presentation": "time", "iso": "2026-03-08T07:00:01.000Z" },
            "updated": { "presentation": "time", "iso": "2026-03-08T08:00:01.000Z" },
            "age": { "presentation": "time", "iso": "2026-03-08T09:00:01.000Z" }
        }
    })];
    let visible = build_vfs_visible_rows(&rows, &BTreeSet::new());
    let now_ms = 1_772_953_259_000;
    let request = vfs_temporal_request(&schema, &visible, &schema.descriptor_column_ids, Rect::new(0.0, 0.0, 400.0, 200.0), 24.0, 0.0, now_ms).expect("temporal request");
    assert_eq!(request.now_ms, now_ms, "relative formatting keeps the host clock's exact millisecond domain");
    assert_eq!(request.values.len(), 3);
    assert_eq!(request.values.iter().map(|value| value.format).collect::<Vec<_>>(), vec![ui_contract::HostTemporalFormatV1::Date, ui_contract::HostTemporalFormatV1::DateTime, ui_contract::HostTemporalFormatV1::Relative]);
    assert!(request.validate());
}
