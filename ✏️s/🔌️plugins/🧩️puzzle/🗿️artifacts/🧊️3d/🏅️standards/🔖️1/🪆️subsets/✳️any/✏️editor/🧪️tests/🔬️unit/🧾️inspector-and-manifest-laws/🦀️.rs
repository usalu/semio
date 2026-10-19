/// 🧾️ Inspector and manifest laws: the selected object and vortex inspector, the patch field actions, the declared windows, dialogs, panel tabs, bridged actions and the German labels.

//#region 🔖️Inspector
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: with nothing selected the inspector
/// shows the document summary; a live `object`-granularity selection switches it to that object's own
/// field group, carrying its real id/origin plus the `hidden`/`locked` `patchInspector` toggles.
#[semio_framework_async_macros::async_test]
async fn selected_object_inspector_renders_that_object_field_group() {
    let mut app = app().await;
    let empty = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(empty.contains("puzzle3d-play-inspector.empty"), "an empty selection shows the document summary: {empty}");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!json.contains("puzzle3d-play-inspector.empty"), "a live selection must replace the document summary: {json}");
    for expected in ["puzzle3d-play-inspector.object.id", "puzzle3d-play-inspector.object.origin", "puzzle3d-play-inspector.object.hidden", "puzzle3d-play-inspector.object.locked"] {
        assert!(json.contains(expected), "inspector must render {expected}: {json}");
    }
    assert!(json.contains(object_id.as_str()), "the rendered field group carries the selected object's own id: {json}");
}

/// 🕹️ Wave B9 lane 2, end to end: the FIRST pick of a fresh session must already be visible to the
/// guest's own render — the object's id and its `hidden`/`locked` `flag_row`s — and a lock taken on
/// that same selection must show as a pressed row. In the browser this hop folded: the pick reached
/// the leftover, `validate_state` pruned it against a topology narrower than the painted document,
/// and the panel kept re-rendering the empty-document summary for 73 s.
#[semio_framework_async_macros::async_test]
async fn first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect of the session");
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!picked.contains("puzzle3d-play-inspector.empty"), "the FIRST pick must replace the document summary: {picked}");
    assert!(picked.contains(object_id.as_str()), "the first pick's own object id must reach the panel: {picked}");
    assert!(picked.contains("puzzle3d-play-inspector.object.locked"), "the first pick must assemble the lock flag_row: {picked}");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), None).await.expect("lock the picked object");
    let locked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(locked.contains("puzzle3d-play-inspector.object.locked"), "a locked selection keeps its lock flag_row: {locked}");
    assert!(
        projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str())).cloned()).and_then(|object| object.get("locked").and_then(Value::as_bool))
            == Some(true),
        "the lock taken on the first pick's selection must land on that object"
    );
}

/// 🕹️ Wave B13: hover after the first pick must keep Inspection on that object — leftover hover
/// used to republish `selectedIds: []` and the panel folded back to the empty summary.
#[semio_framework_async_macros::async_test]
async fn hover_after_first_pick_keeps_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover after first pick");
    let hovered = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!hovered.contains("puzzle3d-play-inspector.empty"), "hover after first pick must keep the object fields: {hovered}");
    assert!(hovered.contains(object_id.as_str()), "hover after first pick must keep the object id: {hovered}");
    assert!(hovered.contains("puzzle3d-play-inspector.object.locked"), "hover after first pick must keep the lock flag_row: {hovered}");
}

/// 🕹️ Background click: empty-target `interactionSelect` with `replace` clears selection; hover may remain.
#[semio_framework_async_macros::async_test]
async fn empty_target_interaction_select_clears_selection_while_hover_remains() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover names the object");
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("empty-target interactionSelect leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let ids = view.get("selectedIds").and_then(|value| value.as_array()).expect("selectedIds");
    assert!(ids.is_empty(), "empty-target interactionSelect must clear selectedIds, got {ids:?} hover={:?}", view.get("hoverTarget"));
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some(object_id.as_str()));
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "empty-target interactionSelect must clear the interaction store"
    );
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(picked.contains("puzzle3d-play-inspector.empty"), "Inspection must fold to the empty summary after background deselect: {picked}");
}

/// 🪟️ WAVE B56 LAW: a window-addressed `interactionSelect` leftover encode carries the window
/// INSTANCE it was dispatched for, and a windowless dispatch carries none — never a synthetic window.
///
/// 🐛️ The leftover encode named no window at all, so the host had to infer the pane the overlay belonged
/// to; when it could not, the guest's own dirty fell back to the bare surface name `window` and the host
/// mounted a synthetic alias to give that name a context (wave W-G3 §8.29). That alias is the surface whose
/// reconcile reservation refusal starved every real one
/// (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54 §6.4). The instance rides the
/// encode now, so `publishLeftoverWorldSelectionV1` addresses a real pane and nothing has to be inferred
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_window_instance_rides_the_encode() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    for window in [main::WINDOW_INSTANCE_PERSPECTIVE, main::WINDOW_INSTANCE_TOP] {
        let admitted = dispatch_reserved_unsettled(
            &mut app,
            "interactionSelect",
            Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets.clone(), "merge": "replace", "method": "pick" })),
            Some(window),
        )
        .await
        .expect("window-addressed interactionSelect admit");
        let settled = settle_reserved(&mut app, admitted).await.expect("window-addressed interactionSelect leftover");
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        assert_eq!(
            view.get("windowId").and_then(semio_framework_value::DslValue::as_str),
            Some(window),
            "the leftover encode must name the window instance the pick addressed, got {:?}",
            view.get("windowId")
        );
    }
}

/// 🎮 Wave B23: leftover.ids that name an object must reach the snapshot `inspection::render` reads
/// when persist `selection(vortex)` is empty.
#[test]
fn leftover_ids_name_object_empty_persist_vortex_from_state_wires_inspection_snapshot() {
    let mut state = protocol::InteractionState::default();
    state.selection.insert("vortex".into(), protocol::DomainSelection { granularity: String::new(), ids: Vec::new(), anchor_id: None });
    state.selection.insert("leftover".into(), protocol::DomainSelection { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: vec!["seed-left-001".into()], anchor_id: None });
    let interaction = Puzzle3dInteractionSnapshot::from_state(&state, &semio_framework_plugin::app::InteractionHoverState::default());
    assert_eq!(interaction.selected, vec!["seed-left-001".to_string()], "leftover.ids must land on the snapshot Inspection reads");
    assert_eq!(interaction.granularity, PUZZLE3D_GRANULARITY_OBJECT);
}

/// 🕹️ A `vortex`-granularity selection switches the inspector to the vortex field group instead — the
/// per-granularity switch the panel's `selected_section` performs.
#[semio_framework_async_macros::async_test]
async fn selected_vortex_inspector_renders_the_vortex_field_group() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(json.contains("puzzle3d-play-inspector.vortex.full-id"), "inspector must render the vortex field group: {json}");
    assert!(json.contains("puzzle3d-play-inspector.vortex.radius"), "inspector must render the vortex radius: {json}");
    assert!(!json.contains("puzzle3d-play-inspector.object.id"), "a vortex selection must not render the object group: {json}");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let before_y = projection_of(&app)
        .get("objects")
        .and_then(|value| value.as_array())
        .and_then(|objects| objects.first())
        .and_then(|object| object.get("origin"))
        .and_then(|value| value.as_array())
        .and_then(|origin| origin.get(1))
        .and_then(|value| value.as_f64())
        .expect("origin.y");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector");
    let projection = projection_of(&app);
    let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
    let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
    let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
    assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
    assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
    let mut app = app().await;
    let id_a = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
    assert_ne!(id_a, id_b, "the added object must be distinct from the first scene_snapshot object");
    let x_a_before = object_origin_x(&app, &id_a);
    let x_b_before = object_origin_x(&app, &id_b);
    assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).await.expect("patchInspector");
    assert_eq!(object_origin_x(&app, &id_a), x_a_before + 3.0, "a delta edit adds to each object's own current x");
    assert_eq!(object_origin_x(&app, &id_b), x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the inspector chrome no longer
/// renders per-entity stepper controls to inspect (see the sibling test's doc comment above), so
/// this now proves the same "resolve selection without embedding ids" contract one level down, at
/// `patchInspector`'s own `interaction.selection(vortex)` fallback (`commands::patch_inspector`) —
/// a bare `field`/`value` patch (no `ids` arg) must resolve against whatever the `vortex` domain's
/// `object` granularity currently holds.
#[semio_framework_async_macros::async_test]
async fn inspector_field_actions_resolve_selection_without_embedding_ids() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector without ids");
    assert_eq!(object_origin_x(&app, &object_id), 42.5, "patchInspector must resolve the patched object from the live selection, not an embedded id");
}
//#endregion 🔖️Inspector

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_main_world_window() {
    let definition = create_puzzle3d_app();
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_the_add_object_dialog() {
    let definition = create_puzzle3d_app();
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
    assert_eq!(dialog.args.len(), 1);
}

/// 📌️ The four declared panel tabs are present (the framework injects its own tabs alongside, so
/// this asserts presence, never a total count).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_four_panel_tabs() {
    let definition = create_puzzle3d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for expected in [artifact::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(body_keys.contains(&expected), "panel tab body {expected} must be declared, got {body_keys:?}");
    }
}

/// 🌉️ Every declared action must bridge through `command_from_action` and round-trip
/// `command_id` via the shared framework harness.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_bridges_to_a_command() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_tests).await;
    assert!(Puzzle3dPlayApp::command_from_action("noSuchAction", None, None).is_err());
}

/// 🌉️ Every declared app action (framework-injected verbs never reach `Puzzle3dCommand::from_action`
/// by design, so they simply fall through the `None` branch below) round-trips through the
/// macro-generated `Puzzle3dCommand::from_action`/`action_id` pair as well as the `ArtifactApp`
/// bridge asserted above.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_round_trips_through_the_command_enum() {
    let definition = create_puzzle3d_app();
    for action in dispatchable_actions(&definition) {
        let Some(command) = Puzzle3dCommand::from_action(&action.id, None, None) else {
            continue;
        };
        assert_eq!(command.action_id(), action.id.as_str(), "declared action {} must round-trip through Puzzle3dCommand", action.id);
    }
}

/// 🗣️ B1: manifest text is baked into `AppDefinition`/`App` as `LocalizedLabel` and resolved
/// directly via `.resolve(Terminology, Locale)` — no shell round-trip needed to assert on it.
#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german_reuse_branded_for_aggregator() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Reuse, Locale::De);
    let actions = || dispatchable_actions(def).into_iter();
    let action = |id: &str| actions().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Bearbeiten");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Aggregator");
    let dialog = def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog");
    assert_eq!(dialog.title.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(dialog.submit_label.resolve(terminology, locale), "Hinzufügen");
    let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
    assert_eq!(arg.label.resolve(terminology, locale), "Art");
    match arg.control() {
        semio_framework_plugin::ActionArgControl::Select { options } => {
            assert!(!options.is_empty() && options.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "the objectKind select offers the declared examples' bounded catalog rows; observed {}", options.len());
            for option in options {
                assert!(!option.value.is_empty(), "every offered object kind names a catalog row");
                assert_eq!(option.label.resolve(terminology, locale), option.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "catalog row {} is document data and must not be re-authored per axis", option.value);
            }
        }
        _ => panic!("objectKind arg is not a select"),
    }
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(action("openVortexSuggestions").label.resolve(terminology, locale), "Verbindungspunkt-Vorschläge öffnen");
    assert_eq!(action("createAttraction").label.resolve(terminology, locale), "Verbindung erstellen");
    assert_eq!(def.utilities.iter().find(|entry| entry.id == semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID).expect("transform utility").label.resolve(terminology, locale), "Transformieren");
    let framework_interaction_actions = [
        INTERACTION_SELECT_ACTION_ID,
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        semio_framework_plugin::CLEAR_SELECTION_ACTION_ID,
        semio_framework_plugin::SELECT_ALL_ACTION_ID,
        semio_framework_plugin::SET_SELECTION_MODE_ACTION_ID,
        semio_framework_plugin::SET_INTERACTION_GRANULARITY_ACTION_ID,
    ];
    for entry in actions() {
        if framework_interaction_actions.contains(&entry.id.as_str()) {
            continue;
        }
        let text = entry.label.resolve(terminology, locale);
        assert!(!text.contains("Hover") && !text.contains("Pick") && !text.contains("hovern"), "leftover English/mistranslation in {}: {text}", entry.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_stay_english_native_without_brand_locks() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Native, Locale::En);
    let action = |id: &str| dispatchable_actions(def).into_iter().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Edit");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Puzzle 3D");
    assert_eq!(def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog").title.resolve(terminology, locale), "Add Object");
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Add Object");
}

#[semio_framework_async_macros::async_test]
async fn document_and_kinds_trees_use_german_reuse_section_labels() {
    let mut app = app().await;
    app.set_label_axes(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Reuse);
    let document_json = render_body(&mut app, artifact::BODY_KEY).await.to_string();
    let kinds = render_body(&mut app, catalogue::BODY_KEY).await.to_string();
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let measures_json = to_json_string(&app.window_measures(&view).await);
    assert!(document_json.contains("Baukomponenten"), "document tree objects section");
    assert!(document_json.contains("Verbindungen"), "document tree attractions section");
    assert!(document_json.contains("Referenzen"), "document tree references section");
    assert!(document_json.contains("Zielvolumina"), "document tree target volumes section");
    assert!(kinds.contains("Kabel"), "catalogue cables section");
    assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
    assert!(!document_json.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
    assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
    assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
    assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
}

#[semio_framework_async_macros::async_test]
async fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
    let definition = create_puzzle3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
    assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
    assert!(!utility_ids.contains(&semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID), "fill is a mode-level tool, not a window utility");
    let window = definition.window_kinds.iter().find(|window| window.id == semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).expect("main window");
    let main_utilities: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(main_utilities.first().copied(), Some(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID));
    assert!(!main_utilities.contains(&"select"));
    assert!(!main_utilities.contains(&semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID), "fill must not be bound to the main window as a utility");
    assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[semio_framework_async_macros::async_test]
async fn tool_registry_declares_fill_tool() {
    let definition = create_puzzle3d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![ToolRef::new(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).await]);
    assert!(dispatchable_actions(&definition).iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🛠️ Wave W-AB, restated by wave B31: an empty `setActiveTool` is a DISARM, so the only verb allowed to
/// emit one is the one whose whole meaning is "leave what is armed". Background and parameter traffic that
/// merely happens WHILE fill is armed must never emit it — a bounce there disarms the tool under the user's
/// hands mid-plan. `engagementAbort` is the exception and states it deliberately
/// (`🎮️commands/🛑️engagement-abort/🦀️.rs`, and
/// `escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool`): before B31 it was listed here as
/// a bounce too, which is exactly why Escape could never leave Fill in the browser.
#[semio_framework_async_macros::async_test]
async fn fill_flow_only_disarms_the_tool_when_the_user_aborts() {
    let empty_tool = |result: &semio_framework_plugin::InvocationResult| {
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty()))
    };
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let count = dispatch(&mut app, "setFillCount", Some(&json!({ "count": 3 })), None).await.expect("setFillCount");
    assert!(!empty_tool(&count), "adjusting the count mid-plan must not bounce-disarm: {:?}", count.requested_effects);
    let abort = dispatch(&mut app, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(empty_tool(&abort), "engagementAbort IS the disarm — it must emit the empty tool effect: {:?}", abort.requested_effects);
}
//#endregion 🔖️Manifest
