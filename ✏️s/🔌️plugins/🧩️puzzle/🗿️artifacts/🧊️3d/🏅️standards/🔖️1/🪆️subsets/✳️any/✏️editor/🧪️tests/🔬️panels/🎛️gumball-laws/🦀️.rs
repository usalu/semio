/// 🎛️ Gumball laws: the gumball arming, the transform utility per window and the gumball gesture transactions.

//#region 🔖️Gumball
/// 🕹️ Framework leftover `interactionView` must not arm the gumball on a bare pick — only the guest
/// `selectionJson` lane may set `gumballActive`, and only when the transform utility is active.
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_does_not_arm_gumball_without_transform_utility() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })),
        Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID),
    )
    .await
    .expect("interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("interactionSelect settle");
    let gumball = settled
        .output
        .get("interactionView")
        .and_then(|view| view.get("gumball"))
        .expect("leftover gumball");
    assert_eq!(gumball.get("active").and_then(semio_framework_value::DslValue::as_bool), Some(false));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `gumballActive` requires BOTH the
/// transform utility active AND a live object (or target-volume) selection — the live read
/// `render_with_request_context` threads in. `transformMode`/`gumballConfig` depend only on the
/// active utility, per window.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform utility plus a live object selection shows the gumball");
    assert_eq!(transform_selection.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()), Some(vec![object_id.clone()]));
    assert_eq!(transform_selection.get("activeObjectId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert_eq!(selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).get("gumballActive").and_then(Value::as_bool), Some(false), "an unattached gumball must never render");

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("reselect");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ Both handle flags off leaves nothing to grab, so the gumball must not render even with a live
/// selection and the transform utility active (`setTransformGumballFlag` is what the user toggles).
#[semio_framework_async_macros::async_test]
async fn gumball_inactive_when_every_handle_flag_is_off() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "move", "pressed": false })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("no move");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("no rotate");
    let selection = selection_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(selection.get("gumballActive").and_then(Value::as_bool), Some(false), "a gumball with no handles must not render");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `transformMode` depends only on
/// each window instance's host-owned `ViewModel` utility map, so it is the per-window-isolation proof.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top).await);
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective).await);
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, labels);
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).await);
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let app_measures = app.window_measures(&view).await;
    let window_measures = app_measures.get(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
}

fn object_orientation(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("orientation").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_scale(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("scale").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

/// 🧾️ The applied history rows one settle upserted that carry document ops.

fn english(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

/// 🌱️ An empty scene with one fresh object, the object selected and the Transform utility active.
async fn gumball_app() -> (Puzzle3dApp, String) {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)).await.expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    (app, object_id)
}

/// 🛠️ One gumball drag — the ONE pose delta both hosts dispatch on release — is ONE tool transaction: one
/// edit, one history row stamped with the transform tool's `TransactionRef`, whose op is the parametric
/// `drag-selection` over the gesture's literal ids and offset, labelled from the leaf in English and German.
/// One undo restores the pose exactly.
#[semio_framework_async_macros::async_test]
async fn one_gumball_translate_is_one_edit_one_row_and_one_transaction() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert_eq!(rows.len(), 1, "one gesture, one history row: {}", history_row_labels(&settled).join(" | "));
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle3d@1/*#editor#translateSelection", "the transform tool authored it: {transaction:?}");
    assert!(rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "the one op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 item by (6, 0, 0)".to_string(), "1 Element um (6; 0; 0) ziehen".to_string()), "the row is labelled from the leaf");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "the offset lands verbatim on the document");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole gesture");
}

/// 🧮️ Two gestures are two transactions — two rows with distinct refs — and one undo takes back only the
/// second: gestures never coalesce into one another.
#[semio_framework_async_macros::async_test]
async fn two_gumball_gestures_are_two_transactions() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (_, first) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (_, second) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (first, second) = (edit_rows(&first), edit_rows(&second));
    assert_eq!((first.len(), second.len()), (1, 1), "each gesture is its own row");
    assert_ne!(first[0].transaction.as_ref().map(|transaction| transaction.id.clone()), second[0].transaction.as_ref().map(|transaction| transaction.id.clone()), "two gestures mint two transactions");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 1.0).abs() < 1e-9, "one undo takes back only the second gesture");
}

/// 🔄️ A gumball turn and a gumball scaling are ONE transaction each, yielding `rotate-selection` and
/// `scale-selection`, and one undo restores the exact base pose (absolute setters, never a negated angle).
#[semio_framework_async_macros::async_test]
async fn gumball_rotate_and_scale_are_one_transaction_each() {
    let (mut app, object_id) = gumball_app().await;
    let (orientation, scale) = (object_orientation(&app, &object_id), object_scale(&app, &object_id));
    let (_, turned) = dispatch_reporting(&mut app, "rotateSelection", Some(&json!({ "ids": [object_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 0.75 })), None).await;
    let rows = edit_rows(&turned);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("rotate-selection") && rows[0].transaction.is_some(), "one turn, one transaction: {rows:?}");
    assert_ne!(object_orientation(&app, &object_id), orientation, "the turn moves the pose");
    dispatch(&mut app, "undo", None, None).await.expect("undo turn");
    assert_eq!(object_orientation(&app, &object_id), orientation, "one undo restores the orientation");
    let (_, grown) = dispatch_reporting(&mut app, "scaleSelection", Some(&json!({ "ids": [object_id.as_str()], "sx": 1.5, "sy": 1.0, "sz": 1.0 })), None).await;
    let rows = edit_rows(&grown);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("scale-selection") && rows[0].transaction.is_some(), "one scaling, one transaction: {rows:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo scaling");
    assert_eq!(object_scale(&app, &object_id), scale, "one undo restores the scale");
}

/// 🧲️ A gumball drag of an attracting object re-solves the attraction graph inside the SAME transaction: on
/// Nakagin (which ships no attraction, so the law first attracts the compatible door pair `25b0dba0-…:link` →
/// `5f0266bc-…:sl0_d0`) the object the attraction hangs off the dragged one is re-placed from it — one row, one
/// `drag-selection` op whose leaf carries the follower — and one undo restores both poses exactly.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_carries_its_attracted_objects_in_the_same_transaction() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    dispatch(&mut app, "createAttraction", Some(&json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).await.expect("attract the compatible door pair");
    let scene_snapshot = puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed());
    let owner = |full_id: &str| full_id.split(':').next().unwrap_or_default().to_string();
    let (parent, child) = scene_snapshot
        .attractions
        .iter()
        .map(|attraction| (owner(&attraction.attracting), owner(&attraction.attracted)))
        .find(|(parent, child)| parent != child && scene_snapshot.objects.iter().any(|object| &object.id == child && !object.locked) && scene_snapshot.objects.iter().any(|object| &object.id == parent && !object.locked))
        .expect("Nakagin carries an attraction between two unlocked objects");
    let (parent_start, child_start) = (object_origin(&app, &parent), object_origin(&app, &child));
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [parent.as_str()], "dx": 0.0, "dy": 0.0, "dz": 2.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].transaction.is_some() && rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "one gesture, one row, one parametric op: {rows:?}");
    assert!((object_origin(&app, &parent)[2] - parent_start[2] - 2.0).abs() < 1e-9, "the dragged object lifts by the offset");
    assert_ne!(object_origin(&app, &child), child_start, "the attracted object follows its moved parent");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!((object_origin(&app, &parent), object_origin(&app, &child)), (parent_start, child_start), "one undo restores the dragged object AND its follower");
}

/// 🧯️ A motionless release — the drag that went nowhere — leaves zero trace: no edit, no row, no notice.
#[semio_framework_async_macros::async_test]
async fn a_motionless_gumball_release_leaves_zero_trace() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 0.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let result = result.expect("a motionless release completes");
    let noticed = result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. }));
    assert!(edit_rows(&settled).is_empty() && result.mutations.is_empty() && !noticed, "nothing moved, nothing recorded: {}", history_row_labels(&settled).join(" | "));
}

/// 🛠️ LAW (audit Z4) — the World3d host corpus' LOCAL one-shot cases (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`,
/// `live: false`, the protocol puzzle 3d speaks: one net delta on release, no stream phase): every verb dispatch the host
/// owes for a scripted gesture, sent with its exact wire args (the targets swapped for the scene_snapshot's own object), publishes
/// exactly the case's guest edits and moves the object by its offset; a gesture that dispatches nothing publishes nothing.
#[semio_framework_async_macros::async_test]
async fn the_world3d_local_gumball_cases_land_as_their_guest_edits() {
    let corpus = parse(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-live-protocol.json")).expect("committed test data reads").as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the protocol corpus parses");
    let cases: Vec<&Value> = corpus.get("cases").and_then(Value::as_array).expect("cases").iter().filter(|case| case.get("live").and_then(Value::as_bool) == Some(false)).collect();
    assert_eq!(cases.len(), 4, "the corpus carries the four local one-shot cases");
    for case in cases {
        let name = case.get("name").and_then(Value::as_str).expect("name");
        let (mut app, object_id) = gumball_app().await;
        let before = object_origin(&app, &object_id);
        let mut edits = 0;
        for dispatch in case.get("steps").and_then(Value::as_array).expect("steps").iter().filter_map(|step| step.get("dispatch")).filter(|dispatch| !dispatch.is_null()) {
            let action = dispatch.get("action").and_then(Value::as_str).expect("action");
            let mut args = dispatch.get("args").cloned().expect("args");
            args.as_object_mut().expect("object args").insert("ids", json!([object_id.as_str()]));
            let (result, settled) = dispatch_reporting(&mut app, action, Some(&args), None).await;
            result.unwrap_or_else(|fault| panic!("{name}: {action} dispatches from the host's wire args: {fault:?}"));
            edits += edit_rows(&settled).len();
        }
        let guest = case.get("guest");
        let expected_edits = guest.and_then(|guest| guest.get("edits")).and_then(Value::as_u64).unwrap_or(0);
        let offset: Vec<f64> = guest.and_then(|guest| guest.get("offset")).and_then(Value::as_array).map_or_else(|| vec![0.0; 3], |values| values.iter().filter_map(Value::as_f64).collect());
        let after = object_origin(&app, &object_id);
        assert_eq!(edits as u64, expected_edits, "{name}: the published edits");
        assert!((0..3).all(|axis| (after[axis] - before[axis] - offset[axis]).abs() < 1e-9), "{name}: the object's offset {:?} -> {:?}, expected {offset:?}", before, after);
    }
}

//#endregion 🔖️Gumball
