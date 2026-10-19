//! 🧰️ Drawing editor unit laws over utilities: marquee selection, camera, undo round trips, the utility registry, initial utility maps and armed canvas utilities.

use super::*;
use semio_framework_test_fixtures::fixture_text;

fn with_utility(meta: &semio_framework_plugin::ActionMeta, utility: &str) -> semio_framework_plugin::ActionMeta {
    let mut meta = meta.clone();
    meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..meta.view_state.clone().unwrap_or_else(||ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)) });
    meta
}

pub(super) async fn selected_strokes(app: &DrawingAppFixture) -> Vec<String> {
    app.interaction_state().await.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn marquee_select_covers_contained_layer_only() {
    // 🔖 Built through dispatched commands (`add-layer` + `patch-layer` transform fields), never
    let (mut app, meta) = inline_selection_app().await;
    let utility_meta = with_utility(&meta, "selectMarquee");
    let initial_id = layer_id(&app.snapshot().unwrap().layers[0]).to_string();
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &meta).await;

    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    let rect_a_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "10"), ("transformY", "10"), ("transformScaleX", "0.15625"), ("transformScaleY", "0.208333")] {
        settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: rect_a_id.clone(), field: field.into(), value: value.into() }), &meta).await;
    }

    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:ellipse".into() }), &meta).await;
    let ellipse_b_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
    for (field, value) in [("transformX", "200"), ("transformY", "200")] {
        settled(&mut app, DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: ellipse_b_id.clone(), field: field.into(), value: value.into() }), &meta).await;
    }

    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 460.0, y: 360.0, width: 800.0, height: 600.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    assert!(result.mutations.is_empty(), "a pure marquee-select gesture is not a document operation");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert_eq!(selected_strokes(&app).await, vec![rect_a_id.clone()], "only the contained rect is selected, not the outside ellipse");
    artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_writes_runtime_and_emits_no_operations() {
    let (mut app, meta) = inline_selection_app().await;
    let before = app.snapshot().expect("projection");
    let (result, _) = settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 5.0, y: 5.0, zoom: 2.0 } }), &meta).await;
    assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
    assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
    let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [5.0, 5.0, 2.0]);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
    let (mut app, meta) = inline_selection_app().await;
    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 4.0, y: 5.0, zoom: 1.0 } }), &meta).await;
    let (result, _) = settled(&mut app, DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &meta).await;
    assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
    let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().expect("addressed canvas view")).await.expect("render"));
    assert_eq!([scene.camera_x, scene.camera_y, scene.zoom], [4.0, 5.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn add_layer_undo_round_trip_through_wrapper() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    artifact_laws::assert_undo_redo_round_trip(&mut app.0, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
}

#[semio_framework_async_macros::async_test]
async fn path_join_conversion_position_and_translation_each_undo_as_one_edit() {
    use semio_s_artifact_draw_drawing_geometry::editing::{edit_path, PathEdit, PathPoint, PathPointRef};
    for edit in [PathEdit::Join { index:1,other:2 },PathEdit::Convert { index:1,target:SegmentType::Cubic },PathEdit::Position {index:1,point:PathPoint::Anchor,to:[15.0,5.0]},PathEdit::Translate {points:vec![PathPointRef {index:0,point:PathPoint::Anchor},PathPointRef {index:1,point:PathPoint::Anchor}],delta:[3.0,-2.0]}] {
        let mut app=drawing_app().await;
        let before=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move { to:[0.0,0.0] },semio_s_artifact_draw_drawing_model::PathSegment::Line { to:[10.0,0.0] },semio_s_artifact_draw_drawing_model::PathSegment::Move { to:[20.0,0.0] },semio_s_artifact_draw_drawing_model::PathSegment::Line { to:[30.0,0.0] }];
        let after=edit_path(&before,&edit).unwrap();
        let layer=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Editable")).to_string().into()).expect("nonempty authored identity"), "Editable",before.clone().into());
        let id=layer_id(&layer).to_string();
        let snapshot=DrawingSnapshot { id:"path-history".into(),layers:vec![layer].into(),..Default::default() };
        load_drawing_fixture(&mut app,&snapshot);
        let command=DrawingCommand::EditPath(edit_path::EditPath { layer_id:id,edit:Box::new(edit) });
        artifact_laws::assert_undo_redo_round_trip(&mut app.0, command, |app| {
            let snapshot=app.snapshot().unwrap();
            let DrawingLayerNode::Path(path)=&snapshot.layers[0] else { panic!("Expected path") };
            path.segments.iter().cloned().collect::<Vec<_>>()
        }, before, after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn shape_conversion_restores_the_primitive_with_one_undo() {
    let mut app=drawing_app().await;
    let layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Convert me")).to_string().into()).expect("nonempty authored identity"), "Convert me");
    let id=layer_id(&layer).to_string();
    let before=DrawingSnapshot { id:"conversion-history".into(),layers:vec![layer].into(),..Default::default() };
    load_drawing_fixture(&mut app,&before);
    let mut after=before.clone();
    for mutation in edit_selection::plan(&before,&[id.clone()],"toPath").unwrap() { semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&mutation).unwrap(); }
    artifact_laws::assert_undo_redo_round_trip(&mut app.0, DrawingCommand::EditSelection(edit_selection::EditSelection { operation:"toPath".into(),ids:vec![id] }), |app|app.snapshot().unwrap(), before, after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
    let definition = create_drawing_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "editNodes", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect", "editNodes"]);
    let scene = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).expect("canvas window");
    assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
    let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility app action");
    assert!(matches!(set_active_utility.kind, ActionKind::View));
    assert!(semio_framework::window_kind_actions(&definition, scene).iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!scene.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "app action stays canonical instead of being copied into the canvas window");
}

/// 🧰️ Owned guest decoding agrees with independent serde on nullable addressed arms.
#[test]
fn initial_utility_maps_preserve_clears_in_owned_and_serde_guest_contexts() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for locale in ["en", "de"] {
        for row in fixture["cases"].as_array().unwrap() {
            let input = serde_json::json!({"locale":locale,"terminology":"native","activeUtilityByWindowId":row["activeUtilityByWindowId"]});
            let owned = semio_framework_pack_json::from_json_str::<semio_framework::ViewModel>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject);
            let oracle = serde_json::from_value::<semio_framework::ViewModel>(input);
            assert!(owned.is_ok(), "{}: {:?}", row["id"], owned.err());
            assert!(oracle.is_ok(), "{}: {:?}", row["id"], oracle.err());
            let owned = owned.unwrap();
            assert_eq!(owned, oracle.unwrap());
            assert_eq!(serde_json::to_value(&owned.active_utility_by_window_id).unwrap(), row["activeUtilityByWindowId"]);
        }
    }
}

/// 🌅️ The window declares its actual initial arm without inferring the first utility reference.
#[test]
fn initial_window_utility_decisions_follow_the_neutral_contract() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let utilities: Vec<semio_framework::UtilityRef> = serde_json::from_value(row["utilityIds"].clone()).unwrap();
        let initial: Option<semio_framework::UtilityRef> = serde_json::from_value(row["initialUtilityId"].clone()).unwrap();
        let current = serde_json::from_value(row["activeUtilityByWindowId"].clone()).unwrap();
        let input = semio_framework::InitialWindowUtilityInput {
            window_id: row["windowId"].as_str().unwrap(),
            utility_ids: &utilities,
            initial_utility_id: initial.as_ref(),
            active_utility_by_window_id: &current,
            active_tool_id: row["activeToolId"].as_str(),
        };
        let actual = semio_framework::resolve_initial_window_utility(input).unwrap();
        assert_eq!(serde_json::json!({"write":actual.write,"utilityId":actual.utility_id}), row["expected"], "{}", row["id"]);
    }
}

/// 🌄️ The actual producer declares an accepted arm independently of toolbar ordering.
#[semio_framework_async_macros::async_test]
async fn initial_canvas_utility_is_explicit_in_the_manifest() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "fresh-draw-does-not-infer-first-reference").unwrap();
    let definition = create_drawing_app();
    let canvas = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).unwrap();
    assert_ne!(canvas.utilities[0].as_str(), row["initialUtilityId"].as_str().unwrap());
    assert_eq!(serde_json::to_value(canvas).unwrap()["initialUtilityId"], row["initialUtilityId"]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(canvas)).unwrap(), serde_json::to_value(canvas).unwrap());
}

#[semio_framework_async_macros::async_test]
async fn strokes_interaction_domain_enumerates_document_layers_on_the_canvas_window() {
    let definition = create_drawing_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == DRAWING_INTERACTION_DOMAIN).expect("strokes interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Topology));
    assert_eq!(domain.selection.methods, vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso]);
    let canvas_window = definition.window_kinds.iter().find(|window| window.id == DRAWING_PLAY_WINDOW_CANVAS).expect("canvas window");
    assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == DRAWING_INTERACTION_DOMAIN));
}

/// 🧰️ Armed tools preserve palette actions; genuine operation authority owns mutation exclusion.
#[semio_framework_async_macros::async_test]
async fn armed_canvas_utilities_keep_declared_document_actions_available() {
    let policy: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🪛️utilities/🎬️actions/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let definition = create_drawing_app();
    let canvas = definition.window_kinds.iter().find(|window| window.id == policy["windowKind"].as_str().unwrap()).unwrap();
    let actions = semio_framework::window_kind_actions(&definition, canvas);
    for id in policy["actions"].as_array().unwrap() {
        assert!(actions.iter().any(|action| action.id == id.as_str().unwrap() && action.in_palette));
    }
    let actual: Vec<_> = definition.utilities.iter().map(|utility| serde_json::json!({"id": utility.id, "allowsActionsWhileActive": utility.allows_actions_while_active})).collect();
    assert_eq!(serde_json::Value::Array(actual), policy["utilities"]);
}

/// 🛑️ Document publication retires armed-tool previews before a stale pointer release.
#[semio_framework_async_macros::async_test]
async fn armed_canvas_utilities_retire_gestures_after_document_actions() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🪛️utilities/🎬️actions/🧪️tests/🧫️fixtures/🛑️interruption/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let utility = row["utility"].as_str().unwrap();
        let (mut app, mut meta) = inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id = Some(utility.into());
        let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Source")).to_string().into()).expect("nonempty authored identity"), "Source");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).id = "source".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).attributes.fill = Some(semio_s_artifact_draw_drawing_model::FillStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] });
        let before = DrawingSnapshot { id: "action-interruption".into(), layers: vec![layer].into(), ..Default::default() };
        load_drawing_fixture(&mut app, &before);
        settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
        let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": DRAWING_INTERACTION_GRANULARITY, "id": "source" })]).unwrap();
        let admission = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": DRAWING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" }))), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.unwrap();
        let press = &row["press"];
        let movement = &row["move"];
        settled(&mut app, DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: press[0].as_f64().unwrap(), y: press[1].as_f64().unwrap(), width: 800.0, height: 600.0, ..Default::default() }), &meta).await;
        settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: movement[0].as_f64().unwrap(), y: movement[1].as_f64().unwrap(), width: 800.0, height: 600.0, shift: false, alt: false, samples: vec![] }), &meta).await;
        assert_eq!(app.snapshot().unwrap(), before, "{utility}: live gesture has no document edit");
        assert_eq!(app.history_snapshot().await.unwrap().edit_count, 0, "{utility}: live preview publishes no document edit");
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().unwrap()).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        if row["preview"] == "transform" {
            assert_eq!(records.iter().find(|record| record["id"] == "source").unwrap()["transform"], serde_json::json!([1.0, 0.0, 0.0, 1.0, 30.0, 20.0]));
        } else {
            assert!(records.iter().any(|record| record["id"] == row["preview"]), "{utility}: actual live preview is present");
        }
        let (_, publication) = settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: fixture["layerKind"].as_str().unwrap().into() }), &meta).await;
        assert_one_artifact_publication(&publication);
        let after = app.snapshot().unwrap();
        assert_eq!(after.layers.len(), 2);
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, meta.view_state.as_ref().unwrap()).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        assert!(!records.iter().any(|record| record["id"] == "overlay:preview" || record["id"] == "overlay:marquee"), "{utility}: stale overlay is retired");
        assert_eq!(records.iter().find(|record| record["id"] == "source").unwrap()["transform"], serde_json::json!([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]));
        settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: movement[0].as_f64().unwrap(), y: movement[1].as_f64().unwrap(), width: 800.0, height: 600.0, shift: false, alt: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
        assert_eq!(app.snapshot().unwrap(), after, "{utility}: stale release cannot publish old geometry");
        let history = app.history_snapshot().await.unwrap();
        assert_eq!(history.edit_count, 1, "{utility}: only the palette action publishes a document edit");
        let edits: Vec<_> = history.upserts.iter().filter(|row| row.edit_id.is_some()).collect();
        assert_eq!(edits.len(), 1, "{utility}: one document edit row among the session commands: {:?}", history.upserts);
        assert_eq!(edits[0].op_count, 1);
        assert_eq!(edits[0].op_lines.len(), 1);
        assert!(edits[0].op_lines[0].starts_with("create-layer"));
        artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert_eq!(app.snapshot().unwrap(), before);
        artifact_laws::settle_history_verb(&mut app.0, "redo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert_eq!(app.snapshot().unwrap(), after);
        artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
        assert!(app.close_terminal_is_empty(), "{utility}: interrupted gesture and document owners retire completely");
    }
}
