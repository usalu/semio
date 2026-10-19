/// 🖱️ Menu and engagement laws: the context menu rows, the Add Object dialog, the engagement verbs, the fill and brush arming and the refused placement notice.

//#region 🩹️CheckedDefects
/// 🎯️ Every action id one built context menu dispatches, groups and submenus included.
fn context_menu_action_ids(items: &[semio_framework_plugin::ContextMenuItemSpec], into: &mut Vec<(String, String)>) {
    for item in items {
        if let Some(action) = item.action.as_deref() {
            into.push((item.id.clone(), action.to_string()));
        }
        if let Some(children) = item.children.as_ref() {
            context_menu_action_ids(children, into);
        }
    }
}

/// 🎯️ Every action id `create_puzzle3d_app()` declares — the set `dispatch_action` can actually route.
fn declared_action_ids() -> std::collections::BTreeSet<String> {
    dispatchable_actions(&create_puzzle3d_app()).into_iter().map(|action| action.id.clone()).collect()
}

/// 🖱️ Context-menu rows bypass the registry-validated `Menu::action` builder (they carry
/// `Puzzle3dLabels` text the English-only `ActionDefinition` cannot resolve), so nothing catches a
/// typo'd action id at build time. `📓️2026-09-09-user-feature-checklist.md` §15/§22 and summary #7:
/// "Zoom to Selection" dispatched `zoomToSelection`, which this crate never declares — the registered
/// verb is `focusSelection`. This law walks EVERY selection kind's menu and holds every emitted id
/// inside the declared set, so the whole class cannot come back.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_dispatches_a_declared_action() {
    let declared = declared_action_ids();
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let object_id = first_object_id(&app);
    let vortex_id = first_vortex_full_id(&app);
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let mut zoom_rows = 0usize;
    for (granularity, id) in [
        (PUZZLE3D_GRANULARITY_OBJECT, object_id.as_str()),
        (PUZZLE3D_GRANULARITY_VORTEX, vortex_id.as_str()),
        (PUZZLE3D_GRANULARITY_TARGET_VOLUME, volume_id.as_str()),
        (PUZZLE3D_GRANULARITY_REFERENCE, "reference-1"),
        (PUZZLE3D_GRANULARITY_ATTRACTION, "attraction-1"),
    ] {
        dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
        let menu = context_menu_for_selection(&mut app, granularity, id).await;
        let mut rows = Vec::new();
        context_menu_action_ids(&menu, &mut rows);
        assert!(!rows.is_empty(), "{granularity} selection built a menu with no actionable row");
        for (row, action) in &rows {
            assert!(declared.contains(action), "{granularity} row {row} dispatches undeclared action {action}; declared: {declared:?}");
            if row == "zoom" {
                zoom_rows += 1;
                assert_eq!(action, "focusSelection", "the zoom row must dispatch the registered camera verb");
            }
        }
    }
    assert_eq!(zoom_rows, 3, "object, vortex and reference selections each carry a Zoom to Selection row");
    dispatch(&mut app, "focusSelection", None, None).await.expect("the context menu's zoom row must dispatch");
}

/// 🗂️ `📓️2026-09-09-user-feature-checklist.md` §23/summary #9: both the "Add Object" dialog and the
/// standalone `addObjectKind` arg form hardcoded ONE static option (`"Object"`), a kind neither
/// example's catalog declares — so the dialog could not add a single real object kind. Both forms must
/// now offer exactly the declared examples' own catalog rows, bounded by
/// [`PUZZLE3D_OBJECT_KIND_OPTIONS_MAX`], with a default that IS one of them.
#[semio_framework_async_macros::async_test]
async fn the_add_object_dialog_offers_every_object_kind_of_both_examples() {
    use semio_framework_plugin::ArgSchema;
    let definition = create_puzzle3d_app();
    let expected: Vec<String> = [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT]
        .into_iter()
        .flat_map(|scene_snapshot| puzzle3d_kind_ids(scene_snapshot, "objects"))
        .fold(Vec::new(), |mut ids, id| {
            if !ids.contains(&id) {
                ids.push(id);
            }
            ids
        });
    assert!(expected.len() >= 2, "both shipped examples must declare object kinds, got {expected:?}");
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    let standalone = dispatchable_actions(&definition).into_iter().find(|action| action.id == "addObjectKind").expect("addObjectKind declared").args.clone();
    for (surface, args) in [("dialog", dialog.args.clone()), ("action", standalone)] {
        let arg = args.iter().find(|arg| arg.id == "objectKind").unwrap_or_else(|| panic!("{surface} declares an objectKind arg"));
        let ArgSchema::String { options, .. } = &arg.schema else { panic!("{surface}'s objectKind must stay a select") };
        let values: Vec<String> = options.iter().map(|option| option.value.clone()).collect();
        assert_eq!(values, expected, "{surface} must offer the live catalog's object kinds");
        assert!(values.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "{surface} exceeded its fixed option ceiling");
        let default = arg.default.as_ref().and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
        assert!(values.contains(&default), "{surface}'s default {default} is not one of its own options");
    }
    assert!(dialog.args.iter().all(|arg| arg.required), "the dialog's kind select stays required");
}

/// 🗨️ Wave B11 (checklist §23, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7): the shell
/// fallback menu / command palette published TWO rows carrying the identical "Add Object…" label —
/// `addObjectKind` (arg-carrying, so the shell redirects it to the bare action pane) and
/// `openAddObjectDialog` (the declared dialog). The arg-carrying one came FIRST, so every user and
/// every probe clicking "Add Object…" got the action pane and the dialog was unreachable. Exactly one
/// add-object row may be palette/menu vocabulary, and it must be the dialog opener; it must also be
/// declared ahead of `addObjectKind`, because the shell menu keeps its first leaves at top level and
/// folds the rest into "More ›".
#[semio_framework_async_macros::async_test]
async fn exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog() {
    let definition = create_puzzle3d_app();
    let actions: Vec<&ActionDefinition> = dispatchable_actions(&definition);
    let find = |id: &str| *actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    let opener = find("openAddObjectDialog");
    let parametrized = find("addObjectKind");
    assert_eq!(opener.kind, ActionKind::Shell, "the dialog opener emits an effect, never a document edit");
    assert!(opener.in_palette, "the dialog opener IS the user-facing add-object row");
    assert!(!parametrized.in_palette, "the dialog's parametrized verb must not publish a second identical row");
    fn label(action: &ActionDefinition) -> &str {
        action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En)
    }
    assert!(label(opener).starts_with(label(parametrized)), "both rows describe the same verb, which is exactly why only one may be offered: {:?} vs {:?}", label(opener), label(parametrized));
    let visible: Vec<&str> = actions.iter().filter(|action| action.in_palette && label(action).starts_with("Add Object")).map(|action| action.id.as_str()).collect();
    assert_eq!(visible, vec!["openAddObjectDialog"], "exactly one add-object row may reach a menu");
    let order = |id: &str| actions.iter().position(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    assert!(order("openAddObjectDialog") < order("addObjectKind"), "the dialog opener must be declared ahead of the verb it drives so the shell menu keeps it at top level");
    let mut app = app().await;
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("the add-object row must dispatch");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "addObject")),
        "clicking the add-object row must open the declared dialog: {:?}",
        result.requested_effects,
    );
}

/// 🗣️ `📓️2026-09-09-user-feature-checklist.md` §14/summary #10: the engagement input advertised
/// `clear`/`rectangle`/`lasso` while the parser silently dropped all three. The placeholder is now
/// DERIVED from the verb list, and every verb in it must do something observable: the three marquee
/// verbs move the viewport's `selection.method`, and `clear` empties the framework-owned domain.
#[semio_framework_async_macros::async_test]
async fn every_advertised_engagement_verb_is_implemented() {
    use semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_ENGAGEMENT_VERBS;
    let mut app = app().await;
    let envelope = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let placeholder = main::engagement(&envelope, &Puzzle3dLabels::NATIVE_EN, None).input.and_then(|input| input.placeholder).unwrap_or_default();
    for verb in PUZZLE3D_ENGAGEMENT_VERBS {
        assert!(placeholder.contains(verb), "the engagement placeholder must advertise {verb}: {placeholder}");
    }
    let method_of = |node: &Value| selection_of(node).get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(method_of(&render_composite(&mut app).await), PUZZLE3D_SELECTION_METHOD_PICK, "a fresh window sweeps with the pick default");
    for method in [PUZZLE3D_SELECTION_METHOD_RECTANGLE, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK] {
        dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": method })), None).await.unwrap_or_else(|error| panic!("engagementSubmit {method}: {error:?}"));
        let rendered = render_composite(&mut app).await;
        assert_eq!(method_of(&rendered), method, "typing {method} must move the viewport marquee method");
    }
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let selected = |node: &Value| selection_of(node).get("ids").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
    assert_eq!(selected(&render_composite(&mut app).await), 1, "the object must be selected before clear");
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "clear" })), None).await.expect("engagementSubmit clear");
    assert_eq!(selected(&render_composite(&mut app).await), 0, "typing clear must empty the framework-owned selection");
}

/// 🪣️ Wave B13, `📓️2026-09-11-wave-B1-battery-extension.md` §14 `engagement-fill-verb`: typing `fill <n>`
/// has to ARM the Fill tool, not merely park a utility name in the scene. Fill is a mode-level TOOL, so the
/// only thing that can press its `#tool.fill` tab is an `Effect::SetActiveTool` leaving the turn; `brush` is
/// a window UTILITY and must take the `Effect::SetActiveUtility` lane instead. This law pins both lanes plus
/// the count hand-off, because the two are one `match` apart in `engagement_submit` and the browser
/// symptom of getting it wrong (`#tool.fill aria-pressed=null`) is indistinguishable from a dead input.
#[semio_framework_async_macros::async_test]
async fn the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count() {
    let mut app = app().await;
    let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "fill 5" })), None).await.expect("engagementSubmit fill 5");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID)),
        "typing `fill 5` must arm the fill tool: {:?}",
        result.requested_effects,
    );
    assert!(
        !result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })),
        "fill is a tool, so it must not also claim a window utility: {:?}",
        result.requested_effects,
    );
    let count = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "setFillCount" => args.as_ref().and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64),
            _ => None,
        })
        .expect("typing `fill 5` must hand the count to the retained setFillCount command");
    assert_eq!(count, 5.0, "the typed count reaches setFillCount verbatim");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, args: Some(args), .. } if action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID && args.get(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID).and_then(semio_framework_value::DslValue::as_str) == Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))),
        "typing `fill 5` starts a fill tool run through the framework: {:?}",
        result.requested_effects,
    );
    let brush = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush");
    assert!(
        brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "brush")),
        "typing `brush` must claim the brush window utility: {:?}",
        brush.requested_effects,
    );
    assert!(
        !brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. })),
        "leaving fill is the host's own `setActiveTool \"\"`, never a guest tool effect: {:?}",
        brush.requested_effects,
    );
}

/// 🧰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35, checklist §14 `engagement-brush-verb`: `fill`
/// arms and `brush` does not, because `fill` is a MODE-level tool whose effect carries no window and
/// `brush` is a window utility whose effect carries one. `active_utility_by_window_id` is keyed by
/// window INSTANCE (wave B9), and a pane reads only its own key — so an effect addressed at the bare
/// window KIND (`puzzle3d-main`) arms a key nothing ever reads and the world lane keeps publishing
/// `activeUtility=select` forever, which is exactly the browser verdict. The sibling law above pins
/// WHICH effect the verb emits; this one pins WHERE it lands, on the multi-pane roster the shell
/// actually runs (the engagement line is window chrome, and an app-level panel resolves through
/// `focused_window_id`, wave B15 — both must resolve to an instance, never to the kind).
#[semio_framework_async_macros::async_test]
async fn the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(top)).await.expect("grow the roster to two panes");
    let armed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), Some(perspective)).await.expect("engagementSubmit brush at the pane");
    let addressed: Vec<&String> = armed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert_eq!(addressed, vec![&perspective.to_string()], "the pane-addressed brush verb must arm that pane's own key: {:?}", armed.requested_effects);
    let unaddressed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush with no window of its own");
    let fallback: Vec<&String> = unaddressed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert!(
        fallback.iter().all(|window_id| !semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_window_id_is_kind(window_id)),
        "a verb that carries no window of its own must still resolve a pane INSTANCE, never the bare kind: {fallback:?}",
    );
}

/// 🛑 Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31, checklist §12/§14 `Abort`: Escape while the Fill
/// TOOL is armed has to LEAVE it — abort the run it shows and disarm the tool — instead of
/// returning untouched, which is what `engagement_abort` did while `puzzle3d_fill_tool_active` and why
/// the browser read `engagement-abort ... activeUtility=fill` after every `fill <n>` (wave B29 §2.5).
/// The disarm is the app's own explicit effect, and the abort leaves NO tool and NO utility armed —
/// the same place the shell's own Escape leaves them (`ShellHost` dispatches `setActiveTool ""`), not
/// a remembered previous tool this app keeps no history of. The `brush`-claims-a-utility law above
/// still holds: a plain verb switch never emits a tool effect, only this abort does.
#[semio_framework_async_macros::async_test]
async fn escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool() {
    let mut armed = app().await;
    dispatch(&mut armed, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let aborted = dispatch(&mut armed, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(
        aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "Escape with fill armed must disarm the tool: {:?}",
        aborted.requested_effects,
    );
    assert!(
        !aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID)),
        "and must never re-arm the tool it is leaving: {:?}",
        aborted.requested_effects,
    );
    let active_utility = |node: &Value| selection_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    let rendered = render_window(&mut armed, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await;
    assert_eq!(active_utility(&rendered), PUZZLE3D_DEFAULT_UTILITY, "the abort leaves no utility armed either: {}", selection_of(&rendered));

    let repeated = dispatch(&mut armed, "engagementAbort", None, None).await.expect("second engagementAbort");
    assert!(
        repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "while the host still reports fill armed the disarm repeats rather than giving up: {:?}",
        repeated.requested_effects,
    );
    assert!(
        !repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "and aborts no run, because none is shown: {:?}",
        repeated.requested_effects,
    );

    let mut unarmed = app().await;
    let untouched = dispatch(&mut unarmed, "engagementAbort", None, None).await.expect("engagementAbort with nothing armed");
    assert!(
        !untouched.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. }) || matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "an abort with no tool armed owes no tool or abort effect: {:?}",
        untouched.requested_effects,
    );
}

/// 🧯️ `📓️2026-09-09-user-feature-checklist.md` §9/§13/summary #13: a placement that produces nothing
/// used to produce NOTHING — no fault, no notice, no visible change. Every refusal path of the two
/// placement verbs must now carry exactly one `Effect::Notify`, and a SUCCESSFUL placement must carry
/// none (a notice on the happy path would be noise, and would hide the real ones).
#[semio_framework_async_macros::async_test]
async fn a_refused_placement_surfaces_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    let orphan = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion without a target still completes");
    assert_eq!(notices(&orphan).len(), 1, "an accept with no target must say so: {:?}", orphan.requested_effects);
    let unknown = dispatch(
        &mut app,
        "addBrushObject",
        Some(&json!({ "targetVortexFullId": "no-such-object:v0", "objectKindId": "no-such-kind", "sourceVortexIndex": 0, "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0] })),
        None,
    )
    .await
    .expect("addBrushObject on an unknown kind still completes");
    assert_eq!(notices(&unknown).len(), 1, "a brush placement onto an undeclared kind must say so: {:?}", unknown.requested_effects);
    for message in notices(&orphan).into_iter().chain(notices(&unknown)) {
        assert_ne!(message, PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored locale/terminology axis, so the notice must be real prose");
        assert!(!message.is_empty(), "a notice must carry text");
    }
    let placed = dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    assert!(notices(&placed).is_empty(), "a successful action must not raise a notice: {:?}", placed.requested_effects);
}

//#endregion 🩹️CheckedDefects
