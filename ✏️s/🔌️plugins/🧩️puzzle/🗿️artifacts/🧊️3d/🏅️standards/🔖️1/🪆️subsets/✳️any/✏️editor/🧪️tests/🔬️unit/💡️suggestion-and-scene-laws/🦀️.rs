/// 💡️ Suggestion and scene laws: the context menu, the vortex suggestion popup, the paged world scene carrier, the per-window utilities and the read-only brush search.

//#region 🔖️Suggestions
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
    assert!(menu_json.contains("openVortexSuggestions"));
    assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
    assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
    assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
    let mut app = app().await;
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
    assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id.as_str(), "flag": "hidden", "value": true })), None).await.expect("setTargetVolumeFlag");
    let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
    assert_eq!(hidden, Some(true));
}

/// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
/// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
/// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
    assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
    assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
}

/// 🎯️ Wave B11 (checklist §15, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8): a right-click
/// on an object the document had NOT selected produced an empty plugin menu, so the shell's own fallback
/// ("Set Active Example", "Export", …) took over the viewport and none of `duplicate`/`select-same-kind`/
/// `zoom`/`hide-show`/`lock-unlock`/`delete` was ever reachable. `ContextMenuSurfaceTarget.hits` is the
/// pointer target the host already sends; the menu must read it, not only `selection`.
#[semio_framework_async_macros::async_test]
async fn right_clicking_an_unselected_object_opens_that_object_menu_not_an_empty_one() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let menu = context_menu_for_hit(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let mut ids: Vec<String> = Vec::new();
    let mut walk: Vec<&semio_framework_plugin::ContextMenuItemSpec> = menu.iter().collect();
    while let Some(item) = walk.pop() {
        ids.push(item.id.clone());
        walk.extend(item.children.iter().flatten());
    }
    for row in ["duplicate", "select-same-kind", "zoom", "hide-show", "lock-unlock", "delete"] {
        assert!(ids.iter().any(|id| id == row), "the object vocabulary must be reachable from a bare hit, {row} missing from {ids:?}");
    }
}

/// 🎯️ …and a hit INSIDE the live selection never narrows the menu to the one row under the cursor: the
/// whole selection stays the subject, so a "Delete (3 objects)" row can never silently become a one-object
/// delete just because the press landed on a particular member.
#[semio_framework_async_macros::async_test]
async fn a_hit_inside_the_selection_keeps_the_whole_selection_as_the_menu_subject() {
    use semio_framework_plugin::ContextMenuHit;
    use semio_framework_plugin::ContextMenuRequest;
    use semio_framework_plugin::ContextMenuSelectionGroup;
    use semio_framework_plugin::ContextMenuSurfaceTarget;
    use semio_framework_plugin::UiMenuRef;
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).expect("objects").iter().filter_map(|object| object.get("id").and_then(Value::as_str)).map(str::to_string).collect();
    assert!(ids.len() >= 2, "this law needs a multi-object selection, got {ids:?}");
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "world3d".into(),
            kind: "world3d".into(),
            hits: vec![ContextMenuHit { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), id: ids[0].clone(), label: None }],
            selection: vec![ContextMenuSelectionGroup { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: ids.clone() }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let menu = app.context_menu(&request, &view).await;
    let delete = menu.iter().find(|item| item.id == "delete").expect("the object menu carries a delete row");
    let label = delete.label.clone().unwrap_or_default();
    assert!(label.contains(&ids.len().to_string()), "the delete row must still name the whole selection, got {label:?} for {ids:?}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_opens_the_suggestion_popup() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 34.0 })), None).await.expect("openVortexSuggestions");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
        result.requested_effects,
    );
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
    assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
    assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_records_explicit_window_id() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await);
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let before_count = object_count(&app);
    dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
    let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await.expect("acceptSuggestion");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
    assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_the_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🖱️ Hovering a row in the suggestion popup moves the tracked candidate index an accept places, without
/// switching the host-owned active utility into brush mode. The popup lists the free candidates the brush
/// suggestions run found, each carrying its object kind's colour and icon.
#[semio_framework_async_macros::async_test]
async fn hover_suggestion_moves_the_candidate_index_over_the_free_candidates_the_run_found() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
    assert_eq!(interaction.pointer("/suggestionMenu/pending").and_then(Value::as_bool), Some(false), "a settled search is no longer pending");
    let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
    assert!(!candidates.is_empty(), "suggestion candidates should be present");
    assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
    assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
    assert!(interaction.get("brushPreviewJson").is_none(), "no host-painted ghost: the run's trace shows every candidate");
    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).await.expect("hoverSuggestion");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
    let mut app = app().await;
    let object_count_before = object_count(&app);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let result = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before + 1);
    let placed = projection_of(&app).pointer("/objects").and_then(Value::as_array).and_then(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str)).find(|id| id.starts_with("puzzle3d.suggestion."))).map(str::to_string).expect("the placed object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!((selection.granularity.as_str(), selection.ids), (PUZZLE3D_GRANULARITY_OBJECT, vec![placed]), "accept selects the object it placed");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
        result.requested_effects,
    );
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
    let selected_vortices = vortices_of(&composite).iter().filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)).count();
    assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
}

/// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
/// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
#[semio_framework_async_macros::async_test]
async fn accept_suggestion_closes_menu_even_when_placement_fails() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let before = interaction_of(&render_composite(&mut app).await);
    assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    let object_count_before = object_count(&app);
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex.as_str()" })), None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
}

/// 📏️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave K: a world scene names its built-in meshes by
/// REFERENCE and never carries their tessellation, so mesh geometry cannot spend the fixed
/// `UiFixedBytes` surface payload. Before this wave `world3d_mesh_kind_entry` inlined
/// `mesh_from_kind`'s buffers, and `vortex-marker` alone (an 80-triangle ico sphere printed as JSON
/// floats) took ~26 KiB of the 32 KiB payload — so a window that also carried a resolved suggestion
/// popup failed `scene-surface.encode` outright. Measured on the live popup-open scene AND on the
/// Nakagin catalog, whose 180 objects are the widest mesh set this editor can name.
#[semio_framework_async_macros::async_test]
async fn the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let node = render_composite(&mut app).await;
    assert_eq!(interaction_of(&node).pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let live_meshes = scene_meshes_of(&node);
    let (payload, capacity) = world_surface_payload_bytes(&mut app, main::BODY_KEY).await;
    assert!(payload <= capacity, "the popup-open world scene packs to {payload} bytes, over the fixed surface capacity {capacity}");
    for meshes in [live_meshes, parse(&semio_s_artifact_puzzle_3d_editor_session::world_meshes_json(&nakagin_scene_snapshot()), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("nakagin meshes").as_array().cloned().expect("mesh array")] {
        assert!(!meshes.is_empty(), "a world scene always declares its meshes");
        assert!(to_json_string(&meshes).len() <= PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET, "mesh references must stay a rounding error against the {capacity}-byte surface payload; observed {}", to_json_string(&meshes).len());
        for mesh in &meshes {
            assert!(mesh.get("id").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "every scene mesh is identified: {mesh}");
            assert!(mesh.get("data").is_none(), "a scene mesh must never carry inline geometry: {mesh}");
            assert!(mesh.get("kind").and_then(Value::as_str).is_some() || mesh.get("url").and_then(Value::as_str).is_some(), "every scene mesh resolves by kind or by url: {mesh}");
        }
        assert!(meshes.iter().any(|mesh| mesh.get("kind").and_then(Value::as_str) == Some("vortex-marker")), "the vortex marker is the kind that used to blow the payload and must still be declared");
    }
}

//#region 🚚️PagedSceneCarrier
/// 🚚️ The one language-neutral declaration of the world-3d lane carrier — the SAME file the scene
/// crate's Rust lane table and `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`'s `WORLD3D_SCENE_LANES` are
/// pinned against, read here so the real Nakagin publication is measured against the same bounds the
/// TypeScript assembler enforces.
static PUZZLE3D_WORLD3D_LANE_CONTRACT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json")).expect("committed test data reads"));

/// 🗼️ Loads the Nakagin Capsule Tower example through the REAL typed `setActiveExample` command and
/// drives it to quiescence, exactly as the navbar picker does.
async fn nakagin_app() -> Puzzle3dApp {
    let mut app = app().await;
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample is a declared puzzle3d command");
    let action_meta = app.meta("local");
    let (raw, cx) = app.parts();
    raw.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("setActiveExample mints its retained whole-document operation");
    settle(&mut app).await;
    app
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (a). The Nakagin Capsule Tower with its
/// vortex suggestion popup open is the document that made `scene-surface.encode` refuse the whole
/// refresh (`surface payload exceeds fixed capacity with 57281 bytes`, browser rebuild #25). It now
/// publishes as a small spine inside the fixed-capacity doc plus one individually paged carrier per
/// payload lane, and every one of those carriers must obey the SAME bounds the TypeScript assembler
/// enforces on the other side of the wire — leaf bytes, children per node, and a byte/hash manifest
/// that lets a partially arrived lane be told from a settled one.
#[semio_framework_async_macros::async_test]
async fn the_nakagin_world_scene_publishes_every_lane_under_the_page_cap_with_the_popup_open() {
    let contract: Value = parse(PUZZLE3D_WORLD3D_LANE_CONTRACT.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("world-3d lane contract is json");
    let leaf_bytes = contract.pointer("/carrier/leafBytes").and_then(Value::as_u64).expect("leafBytes") as usize;
    let children_max = contract.pointer("/carrier/childrenMax").and_then(Value::as_u64).expect("childrenMax") as usize;
    let doc_bytes_max = contract.pointer("/carrier/docBytesMax").and_then(Value::as_u64).expect("docBytesMax") as usize;

    let mut app = nakagin_app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");

    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert_eq!(census.capacity, doc_bytes_max, "the doc ceiling this law measures against is the contract's own");
    assert!(census.doc_bytes <= census.capacity, "the Nakagin spine packs to {} bytes, over the fixed surface capacity {}", census.doc_bytes, census.capacity);
    assert!(!census.lanes.is_empty(), "a published world scene always carries its payload lanes");

    let declared: Vec<&str> = contract["lanes"].as_array().expect("lanes").iter().map(|lane| lane["bodyKey"].as_str().expect("bodyKey")).collect();
    for lane in &census.lanes {
        assert!(declared.contains(&lane.key.as_str()), "lane carrier {} is not a declared world-3d lane", lane.key);
        assert!(lane.widest_leaf <= leaf_bytes, "lane {} has a {}-byte text leaf, over the {leaf_bytes}-byte cap", lane.key, lane.widest_leaf);
        assert!(lane.widest_children <= children_max, "lane {} has a node with {} children, over the {children_max} cap", lane.key, lane.widest_children);
        assert_eq!(lane.bytes, lane.declared_bytes as usize, "lane {} carrier text disagrees with the byte count its spine declared", lane.key);
        assert_eq!(lane.declared_hash.len(), 16, "lane {} must declare a 16-hex-digit content hash", lane.key);
        assert!(lane.leaves >= 1, "lane {} publishes at least one leaf", lane.key);
        assert_eq!(lane.leaves > children_max, lane.leaf_depth > 1, "lane {} with {} leaves must page into a nested carrier exactly when it outgrows one level of {children_max} children (observed leaf depth {})", lane.key, lane.leaves, lane.leaf_depth);
    }

    let interaction = parse(census.assembled.interaction_json.as_deref().expect("the interaction lane reassembles"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("interaction lane is json");
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let instances = census.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).expect("the instances lane always publishes");
    assert!(parse(&census.assembled.instances_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json").as_array().is_some_and(|array| array.len() > 100), "Nakagin publishes its whole catalog of objects through the instances lane");
    eprintln!(
        "nakagin popup-open world scene: spine={}B of {}B, {} lanes carrying {}B total ({}), widest lane={}B in {} leaves",
        census.doc_bytes,
        census.capacity,
        census.lanes.len(),
        census.payload_bytes(),
        census.report(),
        instances.bytes,
        instances.leaves
    );
    eprintln!("nakagin lane paging: {}", census.lanes.iter().map(|lane| format!("{}={}leaves@depth{}", lane.key.trim_start_matches(semio_framework_plugin::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.leaves, lane.leaf_depth)).collect::<Vec<_>>().join(" "));
    assert!(census.payload_bytes() > census.capacity, "this law is only meaningful while the Nakagin payload is past what one fixed doc could ever hold");
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (b). A partial refresh that changes only the
/// camera must leave every lane carrier byte-identical, so the reconciler emits no op for any of
/// them; `instances`, `interaction` and `lod` have very different change rates and the whole point of
/// splitting them is that an unchanged lane costs nothing.
#[semio_framework_async_macros::async_test]
async fn a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh() {
    let mut app = nakagin_app().await;
    let before = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [80.0, 80.0, 80.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("setCamera");
    let moved = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let republished: Vec<&str> = moved.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(republished.is_empty(), "a camera move republished {republished:?}");
    assert_ne!(moved.assembled.camera_json, before.assembled.camera_json, "the camera itself must have moved for this law to mean anything");

    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("interactionSelect");
    let picked = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let changed: Vec<&str> = picked.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(!changed.contains(&semio_framework_plugin::World3dSceneLane::Instances.body_key()), "a selection change must not republish the instances lane");
    assert_eq!(picked.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()), before.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()));
}
//#endregion 🚚️PagedSceneCarrier

/// 📏️ A whole scene's mesh declarations, as references, against the 32 KiB fixed surface payload:
/// generous enough for the widest catalog the editor can name, tight enough that a single inlined
/// primitive (26 KiB) fails it.
const PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET: usize = 4 * 1024;

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_sticky_hover() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
/// must leave whatever host-owned utility was already active (e.g. transform) untouched.
#[semio_framework_async_macros::async_test]
async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("activate transform");
    let vortex = first_vortex_full_id(&app);
    let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("openVortexSuggestions");
    assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
    let open_node = render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await;
    let open_interaction = interaction_of(&open_node);
    assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
    assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let accept = dispatch(&mut app, "acceptSuggestion", None, Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("acceptSuggestion");
    assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
    let accept_interaction = interaction_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
}

/// 🎰️ Wave B6: `pending_reserved` is direct-mapped on `job % 64`, and a reserved verb that is NOT an
/// interaction (`noteShellCommand` — the shell records one for every user command) holds its residue
/// class until it settles. `retire_pending_reserved_latest_wins` may only retire interaction pendings,
/// so the colliding class used to fault the whole pick with `framework route 'interactionSelect' has no
/// exact pending spawn slot` (browser battery #44-pre) while 63 classes stood empty. 64 rounds cover
/// every residue class, so at least one of them mints into the occupied one.
#[semio_framework_async_macros::async_test]
async fn interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    dispatch_reserved_unsettled(&mut app, "noteShellCommand", Some(&json!({ "commandId": "framework.shell.b6-probe", "label": "Probe" })), None).await.expect("a non-interaction reserved verb admits and holds its residue class");
    for round in 0..64 {
        let admitted = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.unwrap_or_else(|error| panic!("round {round} must mint into a vacant residue class: {error:?}"));
        settle_reserved(&mut app, admitted).await.unwrap_or_else(|error| panic!("round {round} must commit: {error:?}"));
    }
}

/// 🖱️ Wave W-AB: a vortex pointermove storm admits 70 `interactionHover`s (then a click
/// `interactionSelect`) before Isolated reserved jobs finish — the #38 spawn-admit drop. Latest-wins keeps one
/// pending hover, so the armed brush's suggestions run searches the last target and a brush click places one of
/// its free candidates.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_storm_admits_then_the_brush_run_searches_and_a_click_places() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm admitted at least one hover")).await.expect("latest hover must commit");
    let select_admit = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("vortex click select must admit after the hover storm");
    settle_reserved(&mut app, select_admit).await.expect("select commit");
    target_brush_suggestions(&mut app, &vortex).await;
    assert!(tool_run_trace_records(&app).iter().any(|(_, verdict)| *verdict == semio_framework_tool_run::ToolRunVerdict::Success), "the storm's target has a free candidate on its trace");
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "fullId": vortex.as_str() })), None).await.expect("a brush click accepts");
    assert_eq!(object_count(&app), before + 1, "a brush click places exactly one free candidate");
}

/// 🪟️ Wave B9 lane 1: `ViewModel.active_utility_by_window_id` is keyed by window INSTANCE
/// (`puzzle3d-main-perspective`), and `plugin_refresh_ui` renders EVERY instance under the window
/// KIND's one body key — the instance identity reaches the guest only through
/// `ViewModel::for_window_instance`'s `window_id`. Resolving the roster's first pane instead
/// published that pane's armed utility into every other pane's world lane, which is why the browser
/// read `utility=select preview=null` in the pane the user had armed Brush in.
#[semio_framework_async_macros::async_test]
async fn each_window_instance_publishes_its_own_armed_utility_into_its_world_lane() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    let arm = |utility: &'static str, window: &'static str| json!({ "utilityId": utility, "windowId": window });
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::volume_brush::UTILITY_ID, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("arm the base pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::brush::UTILITY_ID, perspective)), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    let armed = render_window_refresh(&mut app, main::BODY_KEY, perspective).await;
    let unarmed = render_window_refresh(&mut app, main::BODY_KEY, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), "brush", "the pane the user armed must publish its OWN utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "an unarmed pane must not inherit another pane's utility: {}", interaction_of(&unarmed));
}

/// 🪟️ Wave B39: the OTHER host route — `plugin_render_surface`'s `<body>:<windowInstanceId>` form,
/// the one a mounted surface is rendered through (`🪟️surfaces/🦀️.rs`'s per-surface binding). Two
/// mounted instances of one window kind must publish two DISTINCT world bodies, each carrying its
/// own armed utility, so the host's two `📃️UiDocumentStore`s never hold the same record: a body key
/// that resolved the roster's first pane made both surfaces publish one tree, and the host cannot
/// tell two identical trees apart.
#[semio_framework_async_macros::async_test]
async fn two_mounted_instances_publish_two_distinct_world_bodies_with_their_own_utilities() {
    let mut app = app().await;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    let top = main::WINDOW_INSTANCE_TOP;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let armed = render_window(&mut app, perspective).await;
    let unarmed = render_window(&mut app, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), utilities::brush::UTILITY_ID, "the armed pane's own surface body must carry its utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "the sibling pane's surface body must stay unarmed: {}", interaction_of(&unarmed));
    assert_ne!(armed, unarmed, "two mounted instances must publish two distinct world bodies, never one shared tree");
}

/// 🖱️ Wave W-AB: after a committed vortex hover, Alt+right-click (`openVortexSuggestions` with the
/// hovered `fullId`) publishes `suggestionMenu.open` — the guest half of the host gesture.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_then_open_vortex_suggestions_publishes_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("suggestion hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm")).await.expect("latest hover commits for suggestions");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 24.0 })), None).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "vortex hover + openVortexSuggestions must publish the menu: {interaction}");
    assert_eq!(interaction.pointer("/suggestionMenu/vortexFullId").or_else(|| interaction.pointer("/suggestionMenu/vortex_full_id")).and_then(Value::as_str), Some(vortex.as_str()));
}

/// ⏯️ Lane W2-C: the armed brush pointing at a vortex starts a READ-ONLY tool run. Every candidate it tests is a
/// trace record with its final verdict, the committed document never moves and no undo entry appears; pointing at
/// another vortex retargets the same run; leaving every vortex aborts it without a trace in the document.
#[semio_framework_async_macros::async_test]
async fn the_armed_brush_runs_a_read_only_search_that_traces_every_candidate_and_leaving_aborts_it() {
    use semio_framework_tool_run::ToolRunVerdict;
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("brush");
    let committed = committed_document(&app);
    let vortices = vortex_full_ids(&app);
    let (first, last) = (vortices.first().cloned().expect("a vortex"), vortices.last().cloned().expect("a vortex"));
    assert!(app.tool_run_presence().is_none(), "arming the brush alone starts nothing");
    target_brush_suggestions(&mut app, &first).await;
    let presence = app.tool_run_presence().expect("a brush run");
    assert_eq!(presence.tool_id, utilities::brush::UTILITY_ID);
    let records = tool_run_trace_records(&app);
    assert_eq!(records.len() as u64, presence.total.expect("a counted search"), "every candidate of the target is on the trace");
    assert!(records.iter().all(|(_, verdict)| matches!(verdict, ToolRunVerdict::Success | ToolRunVerdict::Danger | ToolRunVerdict::Warning)), "a settled search leaves no candidate under test: {records:?}");
    assert_eq!(committed_document(&app), committed, "a read-only run never touches the document");
    let run = tool_run_trace_run(&app);
    target_brush_suggestions(&mut app, &last).await;
    assert_eq!(tool_run_trace_run(&app), run, "a new target retargets the same run");
    dispatch(&mut app, "targetBrushSuggestions", Some(&json!({})), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("leave every vortex");
    pump_brush_suggestions(&mut app, "leaving aborts the run", |presence| presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Aborted)).await;
    assert_eq!(committed_document(&app), committed, "the aborted search left the document byte-identical");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "and left no undo entry behind");
    target_brush_suggestions(&mut app, &first).await;
    assert_ne!(tool_run_trace_run(&app), run, "pointing at a vortex again starts a fresh run");
}

/// 🔒️ Lane W2-C: a suggestion popup opened outside brush mode starts the search for its vortex, and closing the
/// popup aborts it; accepting places exactly one free candidate as ONE undo entry.
#[semio_framework_async_macros::async_test]
async fn the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    pump_brush_suggestions(&mut app, "closing aborts the run", |presence| presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Aborted)).await;
    let committed = committed_document(&app);
    let objects = object_count(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("reopen");
    pump_brush_suggestions(&mut app, "the reopened popup's search settles", brush_suggestions_settled).await;
    let (_, settled) = dispatch_reporting(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await;
    assert_eq!(object_count(&app), objects + 1, "an accept places exactly one candidate");
    assert_eq!(history_rows(&settled), 1, "an accept is one command row, got {:?}", history_row_labels(&settled));
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "one undo removes the accepted placement");
}

/// ⏯️ Lane W2-C: the brush declares a read-only `instance3d` tool run through the manifest, restarted by a document
/// or settings change, reading exactly the shared settings its job reads; its Utility Options keep no picker of
/// their own, and no suggestion tick survives.
#[test]
fn the_brush_utility_declares_a_read_only_tool_run_through_the_manifest() {
    use semio_s_artifact_puzzle_3d_model::BrushSuggestionsRunReason;
    use semio_framework_tool_run::ToolRunRebasePolicy;
    use semio_framework_tool_run::ToolRunReconfigurePolicy;
    use semio_framework_tool_run::ToolRunTraceKind;
    let definition = create_puzzle3d_app();
    let brush = definition.utilities.iter().find(|utility| utility.id == utilities::brush::UTILITY_ID).expect("brush utility");
    let run = brush.run.as_ref().expect("the brush declares its ToolRunDefinition");
    run.validate().expect("the brush run definition validates");
    assert!(!run.mutating, "a candidate search never writes the document");
    assert_eq!((run.rebase, run.reconfigure, run.trace), (ToolRunRebasePolicy::Restart, ToolRunReconfigurePolicy::Restart, ToolRunTraceKind::Instance3d));
    assert_eq!((run.run_job.as_str(), run.revalidate_job.as_ref()), (utilities::brush::RUN_JOB_KIND, None));
    assert_eq!(run.settings.config, vec!["/contactTolerance", "/objectKindWeights", "/vortexKindWeights"]);
    for (reason, declared) in BrushSuggestionsRunReason::ALL.iter().zip(&run.reasons) {
        assert_eq!((declared.code, declared.id.as_str(), declared.verdict), (reason.code(), reason.id(), reason.verdict()));
    }
    let actions: Vec<&str> = dispatchable_actions(&definition).into_iter().map(|action| action.id.as_str()).collect();
    assert!(!actions.contains(&"suggestionsTick"), "the suggestions tick loop is gone");
    assert!(actions.contains(&"targetBrushSuggestions"));
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(find_measure_select(&main::window_measures(&scene, labels), "puzzle3d-brush-placement"), None, "the candidate search's readout is the framework ToolRun panel, not a picker");
}

/// 🤝️ leftover InteractionView after vortex-domain interactionHover must carry the vortex full id.
#[semio_framework_async_macros::async_test]
async fn interaction_hover_leftover_carries_vortex_full_id() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex interactionHover admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("vortex interactionHover leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some(vortex.as_str()), "leftover hoverTarget must be the vortex full id, got {hover:?}");
    assert_eq!(hover.get("domain").and_then(semio_framework_value::DslValue::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "next scene render must project leftover hover onto hoveredVortexFullId");
}

//#endregion 🔖️Suggestions
