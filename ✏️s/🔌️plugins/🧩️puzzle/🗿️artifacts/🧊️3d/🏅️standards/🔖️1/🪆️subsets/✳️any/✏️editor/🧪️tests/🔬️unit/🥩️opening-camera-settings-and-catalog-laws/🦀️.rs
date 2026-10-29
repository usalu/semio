/// 🥩️ Opening camera, settings panel scope, rendered-vs-mutated document, shipped kind catalog and snapshot cost laws.

//#region 📷️OpeningCamera
/// 📷️ Reads one pane's published pose straight off the world lane it renders — the exact string
/// `World3dHost` parses into `data-camera-json`.
async fn published_camera(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    camera_of(&render_window(app, window_id).await)
}

fn camera_position(camera: &Value) -> Vec<f64> {
    camera.get("position").and_then(Value::as_array).map(|axes| axes.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
}

/// 📷️ Every pane opens on a real pose of its own before the user has touched anything.
///
/// `Puzzle3dCamera::default()` is all zeros, and `World3dHost` frames each pane LOCALLY and
/// deliberately never dispatches that framing back (`🌐️World3dHost/🟦️.tsx`'s `WorldAutoFit`), so
/// until this wave the world lane of both panes carried position == target == `[0,0,0]` until the
/// first `setCamera` landed: no view direction at boot, and two panes of one document publishing the
/// SAME empty pose (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §4.1 measured it live, wave B15
/// fixed it).
#[semio_framework_async_macros::async_test]
async fn every_pane_opens_on_its_own_framed_camera_before_any_gesture() {
    let mut app = app().await;
    let top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    let perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    for (window_id, camera) in [(main::WINDOW_INSTANCE_TOP, &top), (main::WINDOW_INSTANCE_PERSPECTIVE, &perspective)] {
        let position = camera_position(camera);
        assert_eq!(position.len(), 3, "{window_id} publishes a three-axis camera position: {camera}");
        assert!(position.iter().any(|axis| axis.abs() > 1e-6), "{window_id} opens on a real pose, never the all-zero default: {camera}");
        assert_ne!(camera.get("position"), camera.get("target"), "{window_id}'s opening camera must look at something other than where it stands: {camera}");
    }
    assert_ne!(top, perspective, "the Top and Perspective panes of ONE document open on DIFFERENT poses: top={top} perspective={perspective}");
    assert_eq!(top.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the Top pane opens under the orthographic plan its layout template declares: {top}");
    assert_eq!(top.pointer("/projection/orientation/view").and_then(Value::as_str), Some("top"), "the Top pane looks straight down: {top}");
    assert_eq!(perspective.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the Perspective pane keeps the three-point default: {perspective}");
}

/// 📷️ Swapping the example re-frames the pane on what the NEW document holds, instead of leaving the
/// camera framed on bounds nothing occupies any more.
#[semio_framework_async_macros::async_test]
async fn switching_the_example_reframes_every_unposed_pane_on_the_new_document() {
    let mut app = app().await;
    let before_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let before_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("switch to the Nakagin example");

    let after_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let after_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_ne!(before_perspective, after_perspective, "the pane the switch was dispatched from re-frames: before={before_perspective} after={after_perspective}");
    assert_ne!(before_top, after_top, "a sibling pane the user never posed re-frames on the new document too: before={before_top} after={after_top}");
    assert_ne!(after_top, after_perspective, "re-framing keeps the two panes distinct: top={after_top} perspective={after_perspective}");
}

/// 📷️ A framed pane is framed ONCE: an opening pose is not a gesture, so re-reading the same pane
/// twice, and dispatching an unrelated window option in between, must publish the same pose.
#[semio_framework_async_macros::async_test]
async fn an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set() {
    let mut app = app().await;
    let opening = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("an unrelated window option");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, opening, "an unrelated option must not move the opening pose");

    let posed = json!({ "camera": { "position": [11.0, -7.0, 5.0], "target": [1.0, 2.0, 3.0], "zoom": 1.0 } });
    dispatch(&mut app, "setCamera", Some(&posed), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("the user poses the camera");
    let user = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(camera_position(&user), vec![11.0, -7.0, 5.0], "a real gesture owns the pane's pose from then on: {user}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, user, "and the framing never takes it back: {user}");
}

/// 📽️ Flipping ONE pane's projection repaints THAT pane's published camera and no sibling's.
///
/// A projection is not a label on the pose — it IS the pose: `setProjection` re-derives position and
/// up from the new orientation around the unchanged target (`🎮️commands/📽️set-projection`), and the
/// `WindowOption` scope it publishes under repaints the world body, so the world lane's `cameraJson`
/// — what `World3dHost` parses into `data-camera-json` — must differ in the same settle. The browser
/// probe read `projection-repaints-camera` as bit-identical (ticket 26/09/02 wave B38 §1.3); this law
/// is what says whether the guest owes that, and it does not — the probe sampled the camera ~100 ms
/// after the click, while a measures-rail control's own round trip is documented at 0.7 s and up
/// (`🛠️ShellHelpers/🎚️measure-controls`'s `useWindowMeasureDraft`), so the value it saw flip was that
/// control's optimistic DRAFT and the camera it read was the pre-dispatch one.
#[semio_framework_async_macros::async_test]
async fn flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone() {
    let mut app = app().await;
    let before = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let sibling_before = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_eq!(before.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the law starts from the perspective pane's own three-point default: {before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "orthographicView", "value": "plan" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection orthographicView=plan");

    let after = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(after.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the flipped pane publishes the projection it was given: {after}");
    assert_eq!(after.pointer("/projection/orientation/view").and_then(Value::as_str), Some("plan"), "and the orientation it was given: {after}");
    assert_ne!(camera_position(&after), camera_position(&before), "an orthographic plan looks straight down — the POSE must move, not only its label: before={before} after={after}");
    assert_ne!(after.get("up"), before.get("up"), "and the up vector is re-derived with it: before={before} after={after}");
    let eye = camera_position(&after);
    let focus = camera_position(&after.get("target").cloned().map(|target| json!({ "position": target })).expect("the camera lane carries its target"));
    assert!((eye[0] - focus[0]).abs() < 1e-9 && (eye[1] - focus[1]).abs() < 1e-9 && eye[2] > focus[2], "a plan view stands directly above what it looks at: eye={eye:?} target={focus:?}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await, sibling_before, "the sibling pane's camera lane is untouched by another pane's projection: {sibling_before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "perspectiveKind", "value": "threePoint" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection perspectiveKind=threePoint");
    let restored = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(restored.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "flipping back is symmetric: {restored}");
    assert_ne!(camera_position(&restored), camera_position(&after), "and moves the pose back off the plan axis: after={after} restored={restored}");
}
//#endregion 📷️OpeningCamera

//#region ⚙️SettingsPanelScope
/// ⚙️ The Settings panel addresses the pane the user is LOOKING at.
///
/// 🎯️ An app-level panel body is rendered through `ViewModel::for_panel()`, which clears `window_id`
/// by design — so before this wave `puzzle3d_addressed_window_id` fell through to the roster's first
/// entry, the base window KIND, and every stepper baked THAT into its args: a Settings edit landed on
/// a pane nobody has open (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured
/// `setGridSpacing window=Some("puzzle3d-main")` live). `focused_window_id` survives `for_panel` and
/// is what the panel resolves through now.
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind() {
    fn node_by_key(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| node_by_key(child, key)),
            Value::Array(items) => items.iter().find_map(|item| node_by_key(item, key)),
            _ => None,
        }
    }
    let mut app = app().await;
    for focused in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(focused)).await;
        let rendered = panel.to_string();
        assert!(rendered.contains(focused), "the Settings section names the pane it is addressed at, so the user can see which one a bump will retune: {rendered}");
        for field in ["contact-tolerance", "proximity-radius", "chunk-size", "grid-spacing"] {
            let control = node_by_key(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} stepper: {panel}"));
            let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
            assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(focused), "{field} tags the focused pane, never the base window kind: {bindings:?}");
        }
    }
    let unfocused = render_panel_body(&mut app, settings_panel::BODY_KEY, None).await;
    let control = node_by_key(&unfocused, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).expect("the grid spacing stepper");
    let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
    let addressed = bindings[0].pointer("/args/windowId").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_ne!(addressed, main::WINDOW_KIND_ID, "with nothing focused the panel addresses a LIVE pane, never the base window kind nobody has open: {bindings:?}");
    assert!([main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE].contains(&addressed.as_str()), "and that pane is one this app really has open: {addressed:?} {bindings:?}");
}

/// ⚙️ A Settings bump reaches the focused pane's own rail and leaves the sibling pane alone — the
/// split-window law A1/A2 §19 asked for, played through the id the panel itself baked.
#[semio_framework_async_macros::async_test]
async fn a_settings_bump_retunes_only_the_focused_panes_rail() {
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let top_before = find_measure_slider(app.window_measures(&top_view).await.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id);
    let perspective_before = find_measure_slider(app.window_measures(&perspective_view).await.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id);
    assert_eq!(top_before, perspective_before, "both panes start on the same spacing, so a divergence below can only come from the addressing");

    let bumped = top_before.expect("the rail publishes a spacing") + 0.5;
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_TOP, "value": bumped })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("bump the focused pane's spacing");

    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let measures = app.window_measures(&top_view).await;
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id), Some(bumped), "the focused pane's own rail carries what the Settings stepper published");
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), perspective_before, "the pane the user is NOT looking at keeps its own spacing");
}

/// ⚙️ The Settings panel must RENDER the pane it is ADDRESSED at. Every stepper tags the focused pane
/// as `windowId` (law above), but the value it shows came from `ConfigView::window`, which is `None` for
/// a panel projection (`ViewModel::for_panel` drops `window_id`) and therefore fell through to
/// `Puzzle3dWindowConfig::default()`. The panel then displayed the DEFAULT spacing while writing to the
/// focused pane: battery #53 read `settings-value-reaches-window-rail settings=10.5 windowRail=12.5`
/// after the rail had been nudged to 12.5 — the panel bumped a 10.0 it had invented
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36).
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_renders_the_focused_panes_own_value_not_a_default() {
    fn control(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| control(child, key)),
            Value::Array(items) => items.iter().find_map(|item| control(item, key)),
            _ => None,
        }
    }
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_PERSPECTIVE, "value": 12.5 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("nudge the perspective pane's spacing");
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let rail = app.window_measures(&perspective_view).await;
    assert_eq!(find_measure_slider(rail.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), Some(12.5), "the rail carries the nudge, so the panel below has something to disagree with");
    let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await;
    let stepper = control(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).unwrap_or_else(|| panic!("the grid spacing stepper: {panel}"));
    let rendered = stepper.get("component").and_then(|props| props.get("value")).and_then(Value::as_f64);
    assert_eq!(rendered, Some(12.5), "the Settings panel renders the focused pane's own spacing, never a default it invented: {stepper}");
}
//#endregion ⚙️SettingsPanelScope

//#region 🔬️B23RenderedVsMutatedDocument
/// 🔬️ Wave B23 — the browser's own sequence, on the two-pane session the browser boots: pick the
/// object, read Inspection through BOTH host render routes (the panel route, which carries no
/// `window_id` at all, and the window-instance route), lock the picked selection through the
/// context-menu path (no explicit ids — the arm reads the selection the render just painted), then
/// translate it and read the world lane the pane publishes.
///
/// 🕳️ Every failing verdict of the `#48` selection-scoped family is one row of this law
/// (`inspection-object-fields id=null`, `locked-flag-row lockChrome=false`,
/// `gumball-scene-delta sceneDelta=false`, `relocate-pose-delta`), and the law is GREEN — the guest
/// keeps the pick across both routes and both mutating dispatches, including the
/// `revalidate_interaction_on_document_change` pass every artifact-intent dispatch runs.
/// What the browser has and this law cannot reach is the surface-context route
/// (`plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`): every test context render
/// helper calls `PluginApp::render` directly with a hand-built `ViewModel`, so a body that is never
/// re-rendered — or re-rendered against another surface's retained view — is invisible here.
#[semio_framework_async_macros::async_test]
async fn a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches() {
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("browser-shaped pick");
    let panel_route = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string();
    assert!(!panel_route.contains("puzzle3d-play-inspector.empty"), "the panel route — the only one a real Inspection refresh uses — must see the pick: {panel_route}");
    assert!(panel_route.contains(object_id.as_str()), "the panel route must name the picked object: {panel_route}");
    assert!(panel_route.contains("puzzle3d-play-inspector.object.locked"), "the panel route must assemble the lock flag_row the context menu drives: {panel_route}");
    let window_route = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!window_route.contains("puzzle3d-play-inspector.empty"), "the window-instance route must see the same pick: {window_route}");

    let before = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("lock the picked object");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "the context-menu lock path must land on the object the pick named");
    dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 3.0, "dy": 0.0, "dz": 0.0 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("translate the picked object");
    let after = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    assert_ne!(before, after, "a translate taken on the SECOND mutating dispatch after the pick must still change the world lane the pane renders");
    assert!(
        !render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string().contains("puzzle3d-play-inspector.empty"),
        "two artifact-intent dispatches run two `revalidate_interaction_on_document_change` passes; neither may eat the pick"
    );
}

fn object_flag(app: &Puzzle3dApp, object_id: &str, flag: &str) -> Option<bool> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get(flag).and_then(Value::as_bool))
}
//#endregion 🔬️B23RenderedVsMutatedDocument

//#region 🔖️Pz2ShippedKindCatalog
/// 🗂️ The authored [`PUZZLE3D_SHIPPED_OBJECT_KINDS`] IS the two shipped examples' own
/// `meta.kindCatalogs` rows, in their own order — the law that lets the `objectKind` select be a
/// `const` instead of a parse of 136 395 B of DSL on the `describe()` path (slice PZ2, 2026-09-22).
/// It pays that parse HERE, once, and fails the moment an example's catalog changes without the
/// authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_object_kinds_are_the_two_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for scene_snapshot in [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT] {
        for entry in puzzle3d_catalog_entries(scene_snapshot, "objects") {
            if derived.len() >= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX {
                break;
            }
            let Some(id) = entry.get("id").and_then(semio_framework_value::DslValue::as_str) else {
                continue;
            };
            if derived.iter().any(|(existing, _)| existing == id) {
                continue;
            }
            derived.push((id.to_string(), catalogue::catalog_entry_label(entry)));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE3D_SHIPPED_OBJECT_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️InitialSnapshotCost
/// 🚀️ LAW (ticket 26/09/23, H12): the initial snapshot costs its document and nothing else. Every construction of
/// this app pays for it — each mount, and every `codec` call a hub makes to create or validate a document, where
/// the interpreter multiplies it — so it must never build a scene, sync a precompute session or parse an example
/// the document does not read. Weighed by this thread's heap peak (`HeapWitness`), deterministic and independent
/// of other laws: the snapshot may peak no higher than converting the default scene_snapshot itself, within 4 KiB.
#[test]
fn the_initial_snapshot_costs_only_its_document() {
    let _ = default_scene_snapshot();
    let peak_of = |work: &dyn Fn()| {
        let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
        semio_framework_trace::reset_heap_peak_on_this_thread();
        work();
        semio_framework_trace::peak_heap_bytes_on_this_thread() - before
    };
    let document = peak_of(&|| drop(Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("typed fixture admits"))));
    let snapshot = peak_of(&|| drop(<Puzzle3dPlayApp as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births")));
    assert!(document > 0, "the witness weighs the document conversion: {document}");
    assert!(snapshot <= document + 4096, "the initial snapshot peaked at {snapshot} B, its document at {document} B: it does work the document never reads");
}
//#endregion 🔖️InitialSnapshotCost

/// 🛡️ The host twin carries every object of the typed document and refuses a scale the typed record cannot admit.
#[test]
fn puzzle3d_host_twin_round_trips_its_vocabulary_and_refuses_bad_scale() {
    let document=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored fixture admits");
    let mut host=puzzle3d_scene_snapshot_from_document(&document);
    let admitted=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).expect("the twin readmits");
    assert_eq!(admitted.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>(),document.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>());
    host.objects[0].scale=Some(semio_framework_value::DslValue::String("invalid".into()));
    assert!(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).is_err());
}
