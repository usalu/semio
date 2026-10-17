/// 🧩️ Accepted control publications: the localized buttons and exact actions of the VFS controls, the block list, the event feed, the world3d overlay, the table row button and the graph checkpoint.

//#region 🔖️SceneCommandTests

#[test]
fn accepted_vfs_controls_publish_localized_buttons_and_reject_a_retired_row_action() {
    let window_id = "accepted-vfs-controls";
    let authored_host_id = "accepted-vfs-host";
    let mut scene_node = component_scene_ui(authored_host_id, semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.controller_id = "vfs-controller".into();
    scene.virtual_file_system = Some(semio_framework_ui_scene_views::VirtualFileSystemScene {
        schema_json: "{}".into(),
        rows_json: serde_json::json!([
            { "id": "folder", "name": "Modelle", "hasChildren": true },
            { "id": "successor", "name": "Nachfolger" }
        ])
        .to_string(),
        selected_row_ids_json: None,
        hovered_row_id: None,
        empty_message: None,
        drag_drop_enabled: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained VFS target")));
    let host_id = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let row =
        semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControl { key: format!("{host_id}.vfs.folder"), row_id: "folder".into(), label: "Modelle".into(), kind: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControlKind::Row, rect: Rect::new(0.0, 40.0, 320.0, 24.0), expanded: None };
    let chevron = semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControl {
        key: format!("{host_id}.vfs.chevron.folder"),
        row_id: "folder".into(),
        label: "Einklappen".into(),
        kind: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControlKind::Chevron,
        rect: Rect::new(8.0, 40.0, 14.0, 24.0),
        expanded: Some(true),
    };
    semio_framework_os_renderer_wgpu_scene_feeds::stage_vfs_accessibility_controls(&host_id, vec![row, chevron]);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_vfs_accessibility_candidates(801);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_vfs_accessibility_candidates(801);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let nodes = published_accessibility_nodes_for_test(window_id);
    let folder = nodes.iter().find(|node| node.key == format!("{host_id}.vfs.folder")).expect("accepted VFS row button");
    let toggle = nodes.iter().find(|node| node.key == format!("{host_id}.vfs.chevron.folder")).expect("accepted VFS chevron button");
    assert_eq!((folder.role.as_str(), folder.label.as_deref(), folder.focusable, folder.actionable), ("button", Some("Modelle"), true, true));
    assert_eq!((toggle.role.as_str(), toggle.label.as_deref(), toggle.expanded), ("button", Some("Einklappen"), Some(true)));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, toggle.node_id, &toggle.key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, &mut input).is_some());
    assert!(apply_focused_vfs_control_key(&ui_wgpu::wgpu::KeyAction::Enter, &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "chevron keyboard activation owns renderer-local expansion only");
    assert!(dispatch_accessibility_event(window_id, generation, folder.node_id, &folder.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["selectRows"]);

    let successor = semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControl {
        key: format!("{host_id}.vfs.successor"),
        row_id: "successor".into(),
        label: "Nachfolger".into(),
        kind: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControlKind::Row,
        rect: Rect::new(0.0, 40.0, 320.0, 24.0),
        expanded: None,
    };
    semio_framework_os_renderer_wgpu_scene_feeds::stage_vfs_accessibility_controls(&host_id, vec![successor]);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_vfs_accessibility_candidates(802);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_vfs_accessibility_candidates(802);
    assert!(!apply_focused_vfs_control_key(&ui_wgpu::wgpu::KeyAction::Enter, &mut input));
    assert!(FOCUSED_VFS_CONTROL.with(|cell| cell.borrow().is_none()), "a row missing from the newly accepted semantic frame retires focus");
    assert!(dispatch_accessibility_event(window_id, generation, folder.node_id, &folder.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "the stale accepted address cannot dispatch");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let successor = published_accessibility_nodes_for_test(window_id).into_iter().find(|node| node.key == format!("{host_id}.vfs.successor")).expect("successor accepted row");
    assert!(dispatch_accessibility_event(window_id, generation, successor.node_id, &successor.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["selectRows"]);
}

#[test]
fn accepted_block_list_buttons_publish_localized_actions_and_reject_a_retired_palette_entry() {
    let window_id = "accepted-block-list-controls";
    let authored_host_id = "accepted-block-list-host";
    let mut scene_node = component_scene_ui(authored_host_id, semio_framework_ui_declaration::ui::SurfaceKind::BlockList);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.controller_id = "block-list-controller".into();
    scene.block_list = Some(semio_framework_ui_scene_views::BlockListScene {
        steps: serde_json::from_value(serde_json::json!([{ "id": "basics", "title": "Basics", "blocks": [] }])).expect("typed section"),
        palette: serde_json::from_value(serde_json::json!([{ "blockKind": "filter", "label": "Filter", "iconId": "funnel" }])).expect("typed palette"),
        selected_id: None,
        dragging_id: None,
        domain_id: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained BlockList target")));
    let host_id = retained_scene_host_id(window_id, node);
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let add_step = semio_framework_os_renderer_wgpu_scene_state::BlockListAccessibilityControl {
        key: format!("{host_id}.addStep"),
        label: "Schritt hinzufügen".into(),
        rect: Rect::new(180.0, 4.0, 120.0, 24.0),
        action: ActionDescriptor { controller_id: "block-list-controller".into(), action: "addStep".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({}) },
    };
    let palette = semio_framework_os_renderer_wgpu_scene_state::BlockListAccessibilityControl {
        key: format!("{host_id}.palette.filter"),
        label: "Filter".into(),
        rect: Rect::new(320.0, 4.0, 100.0, 24.0),
        action: ActionDescriptor { controller_id: "block-list-controller".into(), action: "addBlock".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "stepId": "basics", "kind": "filter" }) },
    };
    semio_framework_os_renderer_wgpu_scene_feeds::stage_block_list_accessibility_controls(&host_id, vec![add_step.clone(), palette.clone()]);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_block_list_accessibility_candidates(803);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_block_list_accessibility_candidates(803);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let nodes = published_accessibility_nodes_for_test(window_id);
    let add_step_node = nodes.iter().find(|entry| entry.key == add_step.key).expect("accepted add-step button");
    let palette_node = nodes.iter().find(|entry| entry.key == palette.key).expect("accepted palette button");
    assert_eq!((add_step_node.role.as_str(), add_step_node.label.as_deref(), add_step_node.tabbable, add_step_node.actionable), ("button", Some("Schritt hinzufügen"), true, true));
    assert_eq!((palette_node.role.as_str(), palette_node.label.as_deref()), ("button", Some("Filter")));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, palette_node.node_id, &palette_node.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "addBlock");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("basics"));
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("kind")).and_then(semio_framework::DslValue::as_str), Some("filter"));

    semio_framework_os_renderer_wgpu_scene_feeds::stage_block_list_accessibility_controls(&host_id, vec![add_step]);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_block_list_accessibility_candidates(804);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_block_list_accessibility_candidates(804);
    assert!(dispatch_accessibility_event(window_id, generation, palette_node.node_id, &palette_node.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_event_feed_rows_publish_tabbable_buttons_read_only_text_and_exact_actions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📡️EventFeedHost/🧪️tests/🧫️fixtures/♿️accessible-entry/🔣️.json")).expect("EventFeed accessible entry fixture");
    let actionable = &fixture["cases"][0];
    let passive = &fixture["cases"][1];
    let window_id = "accepted-event-feed-controls";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, semio_framework_ui_declaration::ui::SurfaceKind::EventFeed);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.event_feed = Some(semio_framework_ui_scene_views::EventFeedScene {
        entries_json: serde_json::json!([actionable["entry"].clone()]).to_string(),
        follow: None,
        activate_action: actionable["activateAction"].as_str().map(str::to_owned),
        domain_id: None,
    });
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained EventFeed target")));
    let retained_host = retained_scene_host_id(window_id, node);
    let retained_surface = retained_scene(window_id, node).surface_id;
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    let expected_action = &actionable["expected"]["action"];
    let control = semio_framework_os_renderer_wgpu_scene_state::EventFeedAccessibilityControl {
        key: rebased_scene_key(actionable["expected"]["key"].as_str().unwrap(), host_id, &retained_host),
        entry_id: actionable["entry"]["id"].as_str().unwrap().into(),
        label: actionable["expected"]["accessibleName"].as_str().unwrap().into(),
        rect: Rect::new(0.0, 12.0, 320.0, 24.0),
        action: Some(ActionDescriptor {
            controller_id: expected_action["controllerId"].as_str().unwrap().into(),
            action: expected_action["action"].as_str().unwrap().into(),
            args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({
                "surfaceId": retained_surface.as_str(),
                "id": expected_action["args"]["id"].as_str().unwrap()
            }),
        }),
    };
    semio_framework_os_renderer_wgpu_scene_feeds::stage_event_feed_accessibility_controls(&retained_host, vec![control.clone()]);

    let passive_window_id = "accepted-passive-event-feed-controls";
    let passive_host_id = "feed-ax-passive-host";
    let mut passive_scene_node = component_scene_ui(passive_host_id, semio_framework_ui_declaration::ui::SurfaceKind::EventFeed);
    let UiNode::ComponentScene(passive_scene) = &mut passive_scene_node else { unreachable!() };
    passive_scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    passive_scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    passive_scene.event_feed = Some(semio_framework_ui_scene_views::EventFeedScene { entries_json: serde_json::json!([passive["entry"].clone()]).to_string(), follow: None, activate_action: None, domain_id: None });
    let passive_scene_node = seed_scene_window_with(passive_window_id, passive_scene_node);
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(passive_window_id, passive_scene_node).expect("retained passive EventFeed target")));
    let passive_retained_host = retained_scene_host_id(passive_window_id, passive_scene_node);
    let passive_key = format!("{passive_retained_host}.feed.{}", passive["entry"]["id"].as_str().unwrap());
    semio_framework_os_renderer_wgpu_scene_feeds::stage_event_feed_accessibility_controls(
        &passive_retained_host,
        vec![semio_framework_os_renderer_wgpu_scene_state::EventFeedAccessibilityControl {
            key: passive_key.clone(),
            entry_id: passive["entry"]["id"].as_str().unwrap().into(),
            label: passive["expected"]["accessibleName"].as_str().unwrap().into(),
            rect: Rect::new(0.0, 12.0, 320.0, 24.0),
            action: None,
        }],
    );

    semio_framework_os_renderer_wgpu_scene_feeds::seal_event_feed_accessibility_candidates(805);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_event_feed_accessibility_candidates(805);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    note_accessibility_visible_document(passive_window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted EventFeed button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), ("button", Some(control.label.as_str()), true, true, true));
    let paragraph = published_accessibility_nodes_for_test(passive_window_id).into_iter().find(|entry| entry.key == passive_key).expect("accepted passive EventFeed text");
    assert_eq!((paragraph.role.as_str(), paragraph.focusable, paragraph.tabbable, paragraph.actionable), ("paragraph", false, false, false));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].controller_id, expected_action["controllerId"].as_str().unwrap());
    assert_eq!(actions[0].action, expected_action["action"].as_str().unwrap());
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(retained_surface.as_str()), "the feed's action names the surface its document mounted in");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("id")).and_then(semio_framework::DslValue::as_str), expected_action["args"]["id"].as_str());

    semio_framework_os_renderer_wgpu_scene_feeds::stage_event_feed_accessibility_controls(&retained_host, Vec::new());
    semio_framework_os_renderer_wgpu_scene_feeds::seal_event_feed_accessibility_candidates(806);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_event_feed_accessibility_candidates(806);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_world3d_overlay_controls_publish_read_only_nodes_and_a_polite_mismatch_status() {
    let window_id = "accepted-world3d-overlay-controls";
    let scene_node = component_scene_ui("world-ax-host", semio_framework_ui_declaration::ui::SurfaceKind::World3d);
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained World3d target")));
    let host = retained_scene_host_id(window_id, node);
    let control = |key: &str, role: &'static str, label: &str, depth: u8, polite: bool| semio_framework_os_renderer_wgpu_scene_state::World3dAccessibilityControl { key: format!("{host}::{key}"), role, label: label.into(), depth, polite };
    semio_framework_os_renderer_wgpu_scene_feeds::stage_world3d_accessibility_controls(
        &host,
        vec![
            control("annotations", "list", "Messungen", 0, false),
            control("annotation::dim-width", "listitem", "Bemaßung: Breite 40 mm", 1, false),
            control("scalar-field-mismatch", "status", "Das Skalarfeld passt nicht zum Netz", 0, true),
        ],
    );
    semio_framework_os_renderer_wgpu_scene_feeds::seal_world3d_accessibility_candidates(811);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_world3d_accessibility_candidates(811);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let nodes = published_accessibility_nodes_for_test(window_id);
    let find = |key: &str| nodes.iter().find(|entry| entry.key == format!("{host}::{key}")).unwrap_or_else(|| panic!("published {key}"));
    let list = find("annotations");
    let item = find("annotation::dim-width");
    let status = find("scalar-field-mismatch");
    assert_eq!((list.role.as_str(), list.label.as_deref(), list.live.as_str()), ("list", Some("Messungen"), "off"));
    assert_eq!((item.role.as_str(), item.label.as_deref(), item.depth, list.depth + 1 == item.depth), ("listitem", Some("Bemaßung: Breite 40 mm"), item.depth, true));
    assert_eq!((status.role.as_str(), status.live.as_str()), ("status", "polite"), "the mismatch is announced politely");
    assert!(nodes.iter().filter(|entry| entry.key.starts_with(&host)).all(|entry| !entry.focusable && !entry.actionable), "the overlay text is read-only");
    semio_framework_os_renderer_wgpu_scene_feeds::stage_world3d_accessibility_controls(&host, Vec::new());
    semio_framework_os_renderer_wgpu_scene_feeds::seal_world3d_accessibility_candidates(812);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_world3d_accessibility_candidates(812);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    assert!(published_accessibility_nodes_for_test(window_id).iter().all(|entry| entry.key != format!("{host}::annotations")), "retired overlay nodes leave the tree");
}

#[test]
fn accepted_table_row_button_publishes_one_tabbable_exact_action_and_rejects_stale_addresses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📊️Table/🧪️tests/🧫️fixtures/🔘️button-accessibility/🔣️.json")).expect("Table button accessibility fixture");
    let window_id = "accepted-table-row-button";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, semio_framework_ui_declaration::ui::SurfaceKind::Table);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.table = Some(semio_framework_ui_scene_views::TableScene::base(fixture["columns"].to_string(), serde_json::Value::Array(vec![fixture["row"].clone()]).to_string()));
    semio_framework_os_renderer_wgpu_scene_state::remember_scene_theme(&Theme::default());
    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let controls = semio_framework_os_renderer_wgpu_scene_tables::table_button_accessibility_cells(&retained, Rect::new(0.0, 0.0, 400.0, 300.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert_eq!(controls.len(), 1);
    let control = controls[0].clone();
    assert_eq!(control.key, rebased_scene_key(fixture["expected"]["key"].as_str().unwrap(), host_id, &retained.host_id), "the button key is the fixture's, scoped by the engine-minted host");
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained Table target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    semio_framework_os_renderer_wgpu_scene_feeds::stage_table_button_accessibility_cells(&retained.host_id, controls);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_table_button_accessibility_candidates(902);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_button_accessibility_candidates(902);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted Table row button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), ("button", fixture["expected"]["label"].as_str(), true, true, true));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation + 1, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale window generation is inert");
    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("table-ax-host.row.stale.actions.0", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), fixture["expected"]["action"]);

    semio_framework_os_renderer_wgpu_scene_feeds::stage_table_button_accessibility_cells(&retained.host_id, Vec::new());
    semio_framework_os_renderer_wgpu_scene_feeds::seal_table_button_accessibility_candidates(903);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_button_accessibility_candidates(903);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}

#[test]
fn accepted_graph_checkpoint_publishes_one_tabbable_exact_action_and_rejects_stale_addresses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🌳️GraphTimelineHost/🧪️tests/🧫️fixtures/🎯️checkpoint-hit/🔣️.json")).expect("GraphTimeline checkpoint fixture");
    let expected = &fixture["accessibility"];
    let window_id = "accepted-graph-checkpoint";
    let host_id = fixture["hostId"].as_str().unwrap();
    let mut scene_node = component_scene_ui(host_id, semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline);
    let UiNode::ComponentScene(scene) = &mut scene_node else { unreachable!() };
    scene.surface_id = fixture["surfaceId"].as_str().unwrap().into();
    scene.controller_id = fixture["controllerId"].as_str().unwrap().into();
    scene.graph_timeline = Some(semio_framework_ui_scene_views::GraphTimelineScene { columns_json: fixture["columns"].to_string() });
    let values = fixture["bounds"].as_array().unwrap();
    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let label_track = semio_framework_os_renderer_wgpu_scene_feeds::graph_timeline_scene_label_track(&retained, &Theme::default(), &mut semio_framework_ui_text::FontAtlas::shaped_default());
    let controls = semio_framework_os_renderer_wgpu_scene_feeds::graph_timeline_accessibility_controls(&retained, bounds, &Theme::default(), label_track);
    let control = controls.iter().find(|control| control.key == rebased_scene_key(expected["key"].as_str().unwrap(), host_id, &retained.host_id)).expect("expected checkpoint").clone();
    assert!(semio_framework_os_renderer_wgpu_scene_state::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained GraphTimeline target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    semio_framework_os_renderer_wgpu_scene_feeds::stage_graph_timeline_accessibility_controls(&retained.host_id, controls);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_graph_timeline_accessibility_candidates(922);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_graph_timeline_accessibility_candidates(922);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    publish_accessibility_visible_documents();
    let button = published_accessibility_nodes_for_test(window_id).into_iter().find(|entry| entry.key == control.key).expect("accepted GraphTimeline checkpoint button");
    assert_eq!((button.role.as_str(), button.label.as_deref(), button.focusable, button.tabbable, button.actionable), (expected["role"].as_str().unwrap(), expected["accessibleName"].as_str(), true, true, true));

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(dispatch_accessibility_event(window_id, generation + 1, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale window generation is inert");
    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("history-host.history.stale", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_some());
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), expected["action"]);

    semio_framework_os_renderer_wgpu_scene_feeds::stage_graph_timeline_accessibility_controls(&retained.host_id, Vec::new());
    semio_framework_os_renderer_wgpu_scene_feeds::seal_graph_timeline_accessibility_candidates(923);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_graph_timeline_accessibility_candidates(923);
    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, &button.key, ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none());
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️SceneCommandTests
