/// 🌽️ UI scope and utility laws: the command scope classes and the add-object, target-volume and utility switch verbs.


//#region 🔖️UiScope
/// 🐢️ THE scope law: every declared MUTATING command's `UiDirtyScope` names the panels its
/// mutation changes. A document edit moves the artifact outliner (the object roster), the field
/// inspector (the selected entity's own fields) and the framework history panel (one command row per
/// dispatch) — so a mutation whose scope is `Partial` and omits any of those three leaves that panel
/// showing pre-edit content until some unrelated later action happens to repaint it.
///
/// 🧯️ Why this is a law and not a review note: before wave N every hand-written `Partial` in this app
/// carried `panel_bodies: Vec::new()` (`📓️2026-09-09-wave-L-…md` §5.3) and the shared dispatcher's
/// default was `Full`, so the only two states a command could be in were "repaint the whole shell" and
/// "silently stale panels". `puzzle3d_command_scope_class` is now the one place that decides, and this
/// law is what stops a narrower class from being handed to a verb that edits the document.
///
/// 🈳️ `Chrome` (i.e. `Full`) always satisfies this law — it is the widest, always-correct answer, and a
/// newly declared command that nobody has classified yet is slow rather than wrong.
#[semio_framework_async_macros::async_test]
async fn command_scope_classes_name_the_panels_they_change() {
    use crate::editor::puzzle3d::puzzle3d_command_scope_class;
    use crate::editor::puzzle3d::puzzle3d_scope;
    let definition = create_puzzle3d_app();
    let mut mutating = 0_usize;
    let mut narrowed = 0_usize;
    for action in dispatchable_actions(&definition) {
        let class = puzzle3d_command_scope_class(&action.id);
        let scope = puzzle3d_scope(class);
        if action.kind != ActionKind::Mutation {
            continue;
        }
        mutating += 1;
        match scope {
            UiDirtyScope::Full => {}
            UiDirtyScope::None => panic!("{} is a declared Mutation, so it cannot paint nothing", action.id),
            UiDirtyScope::Partial { ref panel_bodies, ref window_bodies, .. } => {
                narrowed += 1;
                assert_eq!(window_bodies, &vec![main::BODY_KEY.to_string()], "{} edits the document, so the world body is dirty", action.id);
                for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                    assert!(panel_bodies.iter().any(|named| named == body), "{} is a declared Mutation with a narrowed scope, so it must name the {body} panel body: {panel_bodies:?}", action.id);
                }
            }
        }
    }
    assert!(mutating > 0, "the app declares mutating commands");
    assert!(narrowed >= 10, "the scope table must actually narrow the mutating verbs, not fall back to Full for all of them: {narrowed}/{mutating}");
}

/// 🕹️ A pure SELECTION change repaints the field inspector — the panel that renders the selected
/// entity's own fields — and the outliner that marks the selected rows, but never the catalogue, which
/// lists object KINDS and cannot move when only the selection moves.
#[semio_framework_async_macros::async_test]
async fn selection_scope_names_the_inspector_and_not_the_catalogue() {
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    let UiDirtyScope::Partial { panel_bodies, measures, utilities, engagements, labels, .. } = puzzle3d_scope(Puzzle3dScopeClass::Selection) else {
        panic!("the selection class is a narrowed scope");
    };
    assert!(panel_bodies.iter().any(|body| body == inspection::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == artifact::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "{panel_bodies:?}");
    assert!(!panel_bodies.iter().any(|body| body == catalogue::BODY_KEY), "a selection cannot change the kind roster: {panel_bodies:?}");
    assert!(measures, "selection-dependent window measures are re-read");
    assert!(!utilities);
    assert!(!engagements);
    assert!(!labels);
}

/// 🎮️ Wave B34: the six framework interaction verbs are declared out of this app's own scope table
/// through `ArtifactEditor::interaction_scope`, and each one names EXACTLY the lanes it moves. Before
/// this wave the framework answered `UiDirtyScope::Full` for all six — including `interactionHover`,
/// which fires on pointer motion — so every mouse move over the viewport repainted every window body,
/// every panel body, the utilities/tools/engagements rails, the labels and the measures
/// (`📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2: 24 interaction ingresses and 3
/// whole-shell completions per 80 s lane, which is what a document mutation's own refresh pass then
/// queued behind for 10–14 s).
#[semio_framework_async_macros::async_test]
async fn interaction_verbs_declare_exactly_the_lanes_they_move() {
    use crate::editor::puzzle3d::puzzle3d_interaction_chrome_scope;
    use crate::editor::puzzle3d::puzzle3d_selection_scope;
    use crate::editor::puzzle3d::puzzle3d_viewport_scope;
    use crate::editor::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN;
    use semio_framework_plugin::InteractionVerb;
    let domain = [PUZZLE3D_INTERACTION_DOMAIN];
    assert_eq!(Puzzle3dPlayApp::interaction_scope(InteractionVerb::Hover, &domain), Some(puzzle3d_viewport_scope()), "a hover moves the world body's hover lane and nothing else");
    for verb in [InteractionVerb::Select, InteractionVerb::ClearSelection, InteractionVerb::SelectAll] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_selection_scope()), "{verb:?} moves the declared selection lanes");
    }
    for verb in [InteractionVerb::SetSelectionMode, InteractionVerb::SetGranularity] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_interaction_chrome_scope()), "{verb:?} moves the window's Select chrome");
    }
    for verb in InteractionVerb::ALL {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &["mesh"]), None, "{verb:?} on a domain this app never declared must fall back to the framework scope");
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &[]), None, "{verb:?} touching no domain must fall back to the framework scope");
    }
}

/// 🐁️ The hover half, pinned field by field: a pointer move paints the world body ALONE — no panel, no
/// rail, no label, not even the window measures. The pick half names exactly the three selection panels
/// (`selection_scope_names_the_inspector_and_not_the_catalogue` pins their identity) and no fourth one,
/// so widening either answer has to widen this law with it.
#[semio_framework_async_macros::async_test]
async fn a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels() {
    use crate::editor::puzzle3d::puzzle3d_interaction_chrome_scope;
    use crate::editor::puzzle3d::puzzle3d_selection_scope;
    use crate::editor::puzzle3d::puzzle3d_viewport_scope;
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_viewport_scope() else {
        panic!("the hover scope is a narrowed scope");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert!(panel_bodies.is_empty(), "a hover cannot move a panel: {panel_bodies:?}");
    assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot move the shell chrome");
    let UiDirtyScope::Partial { panel_bodies, .. } = puzzle3d_selection_scope() else {
        panic!("the selection scope is a narrowed scope");
    };
    assert_eq!(panel_bodies.len(), 3, "a pick moves exactly the inspector, the outliner and the history: {panel_bodies:?}");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(panel_bodies.iter().any(|named| named == body), "a pick moves {body}: {panel_bodies:?}");
    }
    let UiDirtyScope::Partial { panel_bodies, measures, .. } = puzzle3d_interaction_chrome_scope() else {
        panic!("the interaction chrome scope is a narrowed scope");
    };
    assert!(panel_bodies.is_empty(), "no panel renders the active selection mode or granularity: {panel_bodies:?}");
    assert!(measures, "the measures rail binds both Select controls");
}

/// 🈳️ The two narrow non-document classes deliberately name NO panel body — a camera move and a fill
/// count or distribution slider each touch the world body (and at most the tool/window measures) and
/// nothing else; pointing the brush suggestions run at a vortex paints nothing at all. Pins the claim their doc comments make, so widening one
/// of them has to widen this law too.
#[semio_framework_async_macros::async_test]
async fn viewport_and_fill_option_scope_classes_name_no_panel_body() {
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    for class in [Puzzle3dScopeClass::Viewport, Puzzle3dScopeClass::FillOptions] {
        let UiDirtyScope::Partial { panel_bodies, window_bodies, .. } = puzzle3d_scope(class) else {
            panic!("{class:?} is a narrowed scope");
        };
        assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()], "{class:?}");
        assert!(panel_bodies.is_empty(), "{class:?} claims to paint no panel: {panel_bodies:?}");
    }
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Quiet), UiDirtyScope::None));
    assert_eq!(crate::editor::puzzle3d::puzzle3d_command_scope_class("targetBrushSuggestions"), Puzzle3dScopeClass::Quiet);
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Chrome), UiDirtyScope::Full));
}

/// 🪣️ `setFillCount` is configuration only — a live fill run holds its placements in the framework ledger —
/// so it declares the fill-options scope: the world body plus the tool and window measures, and no panel.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_the_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty() || panel_bodies == [FRAMEWORK_HISTORY_BODY_KEY], "the count moves no document panel: {panel_bodies:?}");
            assert!(tools, "the fill-count entry lives in the fill tool's measures");
            assert!(measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_scene_snapshot(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).await.expect("setObjectKindWeight");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(measures, "distribution sliders live in tool + window measures");
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
    }
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
// and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
// `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
// push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
// through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
//#endregion 🔖️UiScope

//#region 🔖️Utilities
#[semio_framework_async_macros::async_test]
async fn add_object_kind_honors_drop_origin() {
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1);
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
    let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
    assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
    assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
    assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
}

#[semio_framework_async_macros::async_test]
async fn add_object_kind_materializes_the_declared_kind_default() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    let catalog = projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("the declared default kind catalog was materialized");
    assert!(catalog.iter().any(|row| row.get("id").and_then(Value::as_str) == Some("Object")), "the created object references a catalogued kind, never a dangling one: {catalog:?}");
}

/// 🌲️ The catalogue's own gesture on the DEFAULT document: pressing (or dropping) a `concrete-forest`
/// kind row must place an instance that carries that kind's `representations[].url` as its own
/// `meshUrl`. Every other `addObjectKind` law fires `"Object"`, a kind no shipped catalog declares, so
/// none of them ever read a representation — which is how the `meshUrl`-only read in
/// `🎮️commands/🌱️add-object-kind` survived: a placed object with no mesh identity renders nothing and
/// its fill/brush candidates read `mesh-unavailable`.
#[semio_framework_async_macros::async_test]
async fn adding_a_catalogued_concrete_forest_kind_places_an_object_carrying_its_mesh_url() {
    let scene_snapshot = crate::editor::puzzle3d::default_scene_snapshot();
    let entry = crate::editor::puzzle3d::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let kind_id = entry.get("id").and_then(semio_framework_value::DslValue::as_str).expect("catalogued kind id").to_string();
    let expected = entry
        .get("representations")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|representation| representation.get("url").and_then(semio_framework_value::DslValue::as_str))
        .find(|url| !url.is_empty())
        .expect("the catalogued kind names a representation url")
        .to_string();
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": kind_id.clone(), "origin": [4.0, 5.0, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "a catalogued kind places exactly one instance");
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("the placed object").clone();
    assert_eq!(object.get("objectKind").and_then(Value::as_str), Some(kind_id.as_str()), "the placed instance references the catalogued kind");
    assert_eq!(object.get("meshUrl").and_then(Value::as_str), Some(expected.as_str()), "the placed instance must carry its kind's representation url as its mesh identity");
}

/// 🌱️ A catalog TEMPLATE seats its vortex at `point`; a PLACED vortex carries `position`. The
/// non-migrated `addObjectKind` route seeded from `position` alone, so every seat of every catalogued
/// kind collapsed onto the object origin and the object it placed had a degenerate rim no brush could
/// use. Pins the real seats of a real shipped kind.
#[test]
fn catalogued_kind_templates_seed_vortices_at_their_catalog_points() {
    let scene_snapshot = crate::editor::puzzle3d::default_scene_snapshot();
    let entry = crate::editor::puzzle3d::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let seats = crate::editor::puzzle3d::puzzle3d_vortices_from_kind_template(&entry);
    let templates = entry.get("vortices").and_then(|value| value.as_array()).expect("the catalogued kind declares vortex templates");
    assert_eq!(seats.len(), templates.len(), "one seeded vortex per declared template");
    assert!(seats.iter().any(|seat| seat.position != [0.0, 0.0, 0.0]), "a catalogued kind's seats must not all collapse onto the object origin: {seats:?}");
    for (seat, template) in seats.iter().zip(templates) {
        let point: [f64; 3] = template.get("point").and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).expect("every template declares its point");
        assert_eq!(seat.position, point, "seeded vortex {} must sit on its template point", seat.id);
    }
}

/// 🛍️ Both doors onto the default document carry the same catalogue: the INITIAL snapshot the app
/// boots with, and `setActiveExample("concrete-forest")` — which replaces the catalogs through their
/// own `replace_kind_catalogs` mutation and could therefore drop them independently of the scene_snapshot.
#[semio_framework_async_macros::async_test]
async fn the_initial_snapshot_and_set_active_example_both_carry_the_concrete_forest_catalogue() {
    let declared: Vec<String> = crate::editor::puzzle3d::puzzle3d_catalog_entries(&crate::editor::puzzle3d::default_scene_snapshot(), "objects")
        .iter()
        .filter_map(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str).map(str::to_string))
        .collect();
    assert!(!declared.is_empty(), "the concrete-forest scene_snapshot catalogues object kinds");
    let catalogued = |app: &Puzzle3dApp| -> Vec<String> {
        projection_of(app).pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).map(|rows| rows.iter().filter_map(|row| row.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
    };
    let mut app = app().await;
    assert_eq!(catalogued(&app), declared, "the app boots on the concrete-forest catalogue");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_CONCRETE_FOREST })), None).await.expect("concrete-forest");
    assert_eq!(catalogued(&app), declared, "switching back to concrete-forest restores its catalogue");
}

/// 🧊️ Wave B11 (checklist §10, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12): the Volume
/// Brush armed and its W/D/H sliders moved, but Alt+click added nothing. This is the guest half of that
/// gesture — an `addTargetVolume` carrying the host's grid-snapped ground origin must land ONE volume
/// there, sized by the utility's own voxel dimensions times the grid spacing.
#[semio_framework_async_macros::async_test]
async fn alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::volume_brush::UTILITY_ID })), None).await.expect("arm the volume brush");
    for (axis, value) in [("w", 2.0), ("d", 3.0), ("h", 4.0)] {
        dispatch(&mut app, "setVoxelDims", Some(&json!({ "axis": axis, "value": value })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("voxel dimension");
    }
    let before = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [3.0, -2.0, 0.0] })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    assert_eq!(volumes(&app), before + 1, "one Alt+click places exactly one target volume");
    let projection = projection_of(&app);
    let volume = projection.get("targetVolumes").and_then(Value::as_array).and_then(|entries| entries.last()).expect("the placed volume");
    let scale: Vec<f64> = volume.get("scale").and_then(Value::as_array).expect("the volume carries a scale").iter().filter_map(Value::as_f64).collect();
    assert_eq!(scale.len(), 3, "a voxel volume is a three-axis box: {scale:?}");
    assert!(scale[1] > scale[0] && scale[2] > scale[1], "the W/D/H steppers must size the placed box in their own order: {scale:?}");
    let origin: Vec<f64> = volume.get("origin").and_then(Value::as_array).expect("the volume carries an origin").iter().filter_map(Value::as_f64).collect();
    assert_eq!(origin.len(), 3, "the volume lands at the pointed ground point: {origin:?}");
    let whole = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0, 10, 0] })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("addTargetVolume with integer coordinates");
    assert_eq!(volumes(&app), whole + 1, "an integer-valued ground point must place a volume exactly like a fractional one");
}

/// 🧯️ …and a dispatch that carries no usable ground point answers with ONE localized notice instead of
/// returning silently, so a miss is distinguishable from a dead gesture.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    let before = volumes(&app);
    let result = dispatch(&mut app, "addTargetVolume", None, Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("a missing origin is a refusal, not a fault");
    let notices: Vec<&String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an origin-less placement must say why: {:?}", result.requested_effects);
    assert_ne!(notices[0].as_str(), PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert_eq!(volumes(&app), before, "a refused placement adds nothing");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[test]
fn set_active_utility_dirties_the_world_body() {
    use crate::editor::puzzle3d::puzzle3d_command_scope_class;
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    assert_eq!(puzzle3d_command_scope_class(SET_ACTIVE_UTILITY_ACTION_ID), Puzzle3dScopeClass::Viewport, "utility switch must republish the world body (interaction + vortices), not Full chrome and not None");
    match puzzle3d_scope(Puzzle3dScopeClass::Viewport) {
        UiDirtyScope::Partial { window_bodies, .. } => assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]),
        other => panic!("utility switch scope must be the world body, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None);
    let options = engagement.options.as_deref().unwrap_or(&[]);
    assert!(options.is_empty(), "the puzzle3d engagement must not publish window quick-action options");
    assert!(options.iter().all(|option| !matches!(option.id.as_str(), "select" | "brush" | "fill" | "volumeBrush" | "worldRelocate" | "shell-menu.action.openAddObjectDialog")), "the puzzle3d engagement must not re-expose utilities or add-object chrome as options");
}

#[semio_framework_async_macros::async_test]
async fn transform_engagement_does_not_block_background_deselect() {
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into() };
    assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None).session_active, Some(false));
}

//#endregion 🔖️Utilities
