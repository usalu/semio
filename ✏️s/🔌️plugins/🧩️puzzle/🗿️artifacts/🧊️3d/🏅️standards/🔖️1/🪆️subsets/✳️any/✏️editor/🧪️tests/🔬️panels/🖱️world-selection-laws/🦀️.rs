/// 🖱️ World selection laws: pick, vortex and same-kind selection, the scoped command refusals and the selection after add, duplicate and delete.

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app().await;
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection paint rides on
/// `selectionJson` only — instance GEOMETRY stays byte-stable across a pick, so the host never has to
/// re-upload meshes just because something was selected.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app().await;
    let instances_before = instances_of(&render_composite(&mut app).await);
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let after = render_composite(&mut app).await;
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id.clone()]));
    assert_eq!(
        selection_of(&after).get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![object_id]),
        "`selectionJson.ids` is what World3dHost paints instance selection from"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selectionJson` carries the exact
/// field names `World3dHost`'s `parseSelection`/`WorldSelectionRecord` reads — `ids` for object
/// instances, `targetVolumeIds`, `referenceSelectedId`, `hoveredId`. Vortex marks deliberately do NOT
/// appear here (that record has no vortex field; see `world_selection_json`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn world_selection_json_carries_the_host_field_names_per_granularity() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&object_id)).await.expect("hover object");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(selection.get("hoveredId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(selection.get("targetVolumeIds").and_then(Value::as_array).map(Vec::len), Some(0));
    assert!(selection.get("vortexIds").is_none(), "the host's WorldSelectionRecord has no vortexIds field");

    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.last().cloned()).and_then(|volume| volume.get("id").and_then(Value::as_str).map(str::to_string)).expect("target volume id");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select target volume");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(
        selection.get("targetVolumeIds").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![volume_id]),
        "a target-volume granularity selection paints through `targetVolumeIds`"
    );
    assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0), "object `ids` stay empty while a target volume is the live granularity");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the world scene binds `domainId` (and
/// its own `domainGranularityId`) so `World3dHost` dispatches the generic `interactionSelect`/
/// `interactionHover` verbs — this crate has no handler for the legacy `worldPick`/`worldSelect`/
/// `setHover` fallbacks the host would otherwise use.
#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_vortex_interaction_domain_and_its_granularity() {
    let mut app = app().await;
    let scene = render_composite(&mut app).await.get("world3d").cloned().unwrap_or(Value::Null);
    assert_eq!(scene.get("domainId").and_then(Value::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(scene.get("domainGranularityId").and_then(Value::as_str), Some(PUZZLE3D_GRANULARITY_OBJECT));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "clicking empty background must clear, never fall back to reselecting the first object");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
/// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
/// `worldPick` command, which could read `object.locked` before deciding, is deleted.
/// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
/// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
/// function's doc) — "never select a locked object" is now entirely a host click→pick
/// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
/// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
#[semio_framework_async_macros::async_test]
async fn world_pick_locked_object_clears_like_background() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
    let instances = instances_of(&render_composite(&mut app).await);
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
    assert_eq!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// reveals a marker only while its own object (or one of its markers) is selected/hovered — the
/// live read `render_with_request_context` threads in as a `Puzzle3dInteractionSnapshot`.
#[semio_framework_async_macros::async_test]
async fn world_vortices_reveal_in_selected_mode_only_for_the_selected_object() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode with nothing selected reveals no vortex marker");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let revealed = vortices_of(&render_composite(&mut app).await);
    assert!(!revealed.is_empty(), "Selected mode must reveal the selected object's own vortex markers");
    assert!(revealed.iter().all(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(object_id.as_str())), "Selected mode must reveal ONLY the selected object's markers");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "clearing the selection hides the markers again");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app).await).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ A selected vortex marker carries its own `selected` flag on `vorticesJson` — the host's
/// `WorldVortexMarkers` reads it off each record (`WorldSelectionRecord` has no vortex field), and a
/// hovered marker carries `hovered` the same way.
#[semio_framework_async_macros::async_test]
async fn world_vortices_carry_their_own_selected_and_hovered_flags() {
    let mut app = app().await;
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selected = vortices_of(&render_composite(&mut app).await);
    let record = selected.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("selected vortex record");
    assert_eq!(record.get("selected").and_then(Value::as_bool), Some(true));
    assert!(selected.iter().filter(|entry| entry.get("fullId").and_then(Value::as_str) != Some(vortex.as_str())).all(|entry| entry.get("selected").and_then(Value::as_bool) == Some(false)), "only the picked marker is selected");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover vortex");
    let hovered = vortices_of(&render_composite(&mut app).await);
    let record = hovered.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("hovered vortex record");
    assert_eq!(record.get("hovered").and_then(Value::as_bool), Some(true));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
    assert_eq!(selection.ids, vec![vortex]);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
/// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
/// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
/// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
/// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
/// toggles a second target back into the selection instead.
#[semio_framework_async_macros::async_test]
async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
    let mut app = app().await;
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "scene_snapshot must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
    let replaced = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
    let invertive = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "Select Same Kind" emits a real
/// `Emit.interaction_writes` entry, which `VcsArtifactApp` applies through the same `next_selection`
/// machine the reserved `interactionSelect` verb uses — so the widened selection is observable on
/// `interaction_state()` and painted into `selectionJson` on the next render.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_selection_to_every_object_of_that_kind() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    for _ in 0..3 {
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    }
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(ids.len(), 3, "three same-kind objects must exist");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &ids[0]).await.expect("select one");
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    let mut widened = selection.ids.clone();
    widened.sort();
    let mut expected = ids.clone();
    expected.sort();
    assert_eq!(widened, expected, "every object of the clicked object's kind is selected");
    let painted = selection_of(&render_composite(&mut app).await).get("ids").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    let mut painted_sorted = painted.clone();
    painted_sorted.sort();
    assert_eq!(painted_sorted, expected, "the widened selection reaches the host through selectionJson");
}

/// 🕹️ "Select Same Kind" with nothing selected still aborts (nothing to widen from) and leaves the
/// selection untouched.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_with_no_selection_leaves_the_selection_untouched() {
    let mut app = app().await;
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "widening from nothing must not invent a selection");
}

/// 🎯️ A selection-scoped command dispatched with NOTHING selected must refuse VISIBLY: exactly one
/// `Effect::Notify` and no document edit. This is the browser defect measured
/// 2026-09-09 21:05 — "Duplicate Selection" from the context menu completed with no clone, no command
/// row and no notice of any kind, which is indistinguishable from a dead menu entry. Every arm that
/// reads the framework-owned selection is covered here, so the refusal cannot be reintroduced one
/// command at a time.
///
/// 🧾️ A command-log ROW is deliberately not asserted absent: `dispatch_emit` records one row per
/// dispatch by design, mutation-kind actions that produced zero operations included (see its own doc
/// in `🔌️plugin/🦀️.rs`), so a refused `Mutation` still shows up in history with no edit id. What must
/// never happen is a silent nothing — that is what the notice below pins.
#[semio_framework_async_macros::async_test]
async fn selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice() {
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
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear the seed gesture's own selection");
    let objects_before = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    for action in ["duplicateSelection", "deleteSelection", "selectSameKindSelection"] {
        let result = dispatch(&mut app, action, None, None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "{action} painted nothing, so it must not force a full refresh");
    }
    for (action, args) in [
        ("translateSelection", json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
        ("rotateSelection", json!({ "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.0 })),
        ("scaleSelection", json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
    ] {
        let result = dispatch(&mut app, action, Some(&args), None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice on its completion lane: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(matches!(result.ui_scope, UiDirtyScope::None), "{action} painted nothing, so its completion must carry UiDirtyScope::None, got {:?}", result.ui_scope);
    }
    let objects_after = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    assert_eq!(objects_after, objects_before, "a refused selection command must leave the document untouched");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "a refused selection command must not invent a selection either"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `duplicateSelection` re-selects the
/// clones it created — the interaction write is applied AFTER the document mutations land, so the new
/// ids are already in `interaction_topology` and survive `validate_state`'s pruning.
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_reselects_the_created_clones() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "duplicateSelection", None, None).await.expect("duplicateSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids.len(), 1, "exactly the one clone is selected");
    assert_ne!(selection.ids.first().map(String::as_str), Some(object_id.as_str()), "the CLONE is selected, not the original");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert!(live_ids.contains(selection.ids.first().expect("clone id")), "the re-selected id must exist in the document");
}

/// 🌱️ `addObjectKind` SELECTS the object it adds — the same `Emit.interaction_writes` contract
/// `duplicateSelection` and `addBrushObject` already honour, on both of its implementations: the
/// cursorized `Puzzle3dAddObjectKindWork` the interactive job runs and the direct reducer arm. Adding an
/// object the user must then hunt for is half a gesture: the catalogue row, the drag-drop and the Add
/// Object dialog all end with the new object nowhere in the inspector, the gumball or the outliner
/// (battery #53 `catalogue-add-selects-new-object added=1`, ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B36). Every id is asserted against the live document, so a write naming a phantom fails here.
#[semio_framework_async_macros::async_test]
async fn add_object_kind_selects_the_object_it_adds() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    let first = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(first.granularity, PUZZLE3D_GRANULARITY_OBJECT, "the add selects at OBJECT granularity: {first:?}");
    assert_eq!(first.ids.len(), 1, "exactly the added object is selected: {first:?}");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("second object");
    let second = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(second.ids.len(), 1, "the second add REPLACES the selection with its own object, it does not widen it: {second:?}");
    assert_ne!(second.ids, first.ids, "the second add selects the object IT created: {second:?} vs {first:?}");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(live_ids.len(), 2, "both adds landed in the document: {live_ids:?}");
    assert!(live_ids.contains(second.ids.first().expect("added id")), "the selected id is the one the document carries: {live_ids:?}");
}

/// 🗑️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: one `deleteSelection` on a live object selection
/// removes that object from the RENDERED world census within the same settle, records exactly one
/// command-log row, and leaves the deleted id out of the framework-owned selection — so pressing
/// `Delete` twice cannot land a second, empty edit on an id the document no longer carries. The
/// browser red this states was `before=2 after=2 waitedMs=30299` with
/// `history patch applied labels=["delete-object id=object-1"]` in the same run: the document moved
/// and the selection did not, so every later delete was a no-op against a phantom.
#[semio_framework_async_macros::async_test]
async fn delete_selection_shrinks_the_world_census_and_drops_the_deleted_id() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("second object");
    let census_before = instance_count(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(census_before, 2, "two added objects must both be in the rendered world census");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let (deleted, settled) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    deleted.expect("deleteSelection");
    assert_eq!(history_rows(&settled), 1, "a delete that removed an object must publish exactly one command-log row");
    assert_eq!(object_count(&app), 1, "the document must carry one object fewer");
    assert_eq!(instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await), census_before - 1, "the rendered world census must shrink by one within the same settle");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert!(!selection.ids.contains(&object_id), "the deleted id must be gone from the selection, else a second Delete is a silent no-op: {:?}", selection.ids);
    let (again, settled_again) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    again.expect("second deleteSelection");
    let again_rows = history_row_labels(&settled_again);
    assert_eq!(history_rows(&settled_again), 1, "a delete with nothing selected logs the command and no edit: {again_rows:?}");
    assert!(again_rows.iter().all(|row| row.contains("deleteSelection") && row.contains("applied=false") && row.contains("ops=0")), "{again_rows:?}");
    assert_eq!(object_count(&app), 1, "and must not touch the document");
}
//#endregion 🔖️WorldSelection
