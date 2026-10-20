//! 🎛️ Drawing editor unit laws over selection edits: group and layer drag, created layers, initial framing, text edits, select all, transform handles and node drag.

use super::*;
use semio_s_artifact_draw_drawing_editor_interaction as interaction;

#[semio_framework_async_macros::async_test]
async fn selected_group_and_layer_drag_preserves_selection_and_one_history_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("selectDirect".into());
        let mut child=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut child).id="child".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut child).attributes.fill=Some(semio_s_artifact_draw_drawing_model::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});
        let mut group=semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut group).id="group".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut group).transform.scale_x=2.0;
        if let DrawingLayerNode::Group(body)=&mut group { body.children.push(child); }
        let mut other=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Other")).to_string().into()).expect("nonempty authored identity"), "Other");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut other).id="other".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut other).attributes.fill=Some(semio_s_artifact_draw_drawing_model::FillStyle::Solid {color:[0.0,0.0,1.0,1.0]});
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut other).transform.x=400.0;
        let before=DrawingSnapshot { id:"selection-drag".into(),layers:vec![group,other].into(),..Default::default() };
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera { camera:store::Viewport2d { x:0.0,y:0.0,zoom:1.0 } }),&meta).await;
        let ids=vec!["group".to_string(),"child".to_string(),"other".to_string()];
        let targets=serde_json::to_string(&ids.iter().map(|id|serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":id})).collect::<Vec<_>>()).unwrap();
        let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.unwrap();
        let selected=selected_strokes(&app).await;
        assert_eq!(selected.len(),3);
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x:420.0,y:320.0,width:800.0,height:600.0,..Default::default() }),&meta).await;
        assert_eq!(selected_strokes(&app).await,selected);
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x:450.0,y:340.0,width:800.0,height:600.0,samples:vec![] }),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        for (id,x,scale) in [("child",30.0,2.0),("other",430.0,1.0)] {
            let node=records.iter().find(|node|node["id"]==id).unwrap();
            assert_eq!(node["transform"][0].as_f64(),Some(scale),"interior drag preserves scale");
            assert!((node["transform"][4].as_f64().unwrap()-x).abs()<1e-10);
            assert!((node["transform"][5].as_f64().unwrap()-20.0).abs()<1e-10);
        }
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x:450.0,y:340.0,width:800.0,height:600.0,shift:false,ctrl:false,meta:false,cancelled }),&meta).await;
        if cancelled {
            assert_eq!(app.snapshot().unwrap(),before);
            assert!(app.history_snapshot().await.expect("history").upserts.iter().all(|row|row.transaction.is_none()),"a cancelled drag leaves zero trace");
        } else {
            assert_one_artifact_publication(&receipt);
            let rows=app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row|row.transaction.is_some()).collect::<Vec<_>>();
            assert_eq!(rows.len(),1,"one drag is one transaction row: {rows:?}");
            let transaction=rows[0].transaction.as_ref().unwrap();
            assert!(transaction.id.starts_with("tx-") && transaction.tool=="s.draw.drawing@1/*#editor#selectDirect","{transaction:?}");
            assert!(rows[0].op_lines.len()==1 && rows[0].op_lines[0].starts_with("drag-layers"),"one relative drag leaf: {:?}",rows[0].op_lines);
            assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native,semio_framework_ui_locale::Locale::En),"Drag 2 layers by (30, 20)");
            assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native,semio_framework_ui_locale::Locale::De),"2 Ebenen um (30; 20) ziehen");
            let after=app.snapshot().unwrap();
            assert!((semio_s_artifact_draw_drawing_model::layer_base(&after.layers[0]).transform.x-30.0).abs()<1e-10);
            assert!((semio_s_artifact_draw_drawing_model::layer_base(&after.layers[1]).transform.x-430.0).abs()<1e-10);
            assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(semio_s_artifact_draw_drawing_model::find_drawing_layer(&after,"child").unwrap()).transform.x,0.0);
            artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
            assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut app.0, "redo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
            assert_eq!(app.snapshot().unwrap(),after);
        }
        assert_eq!(selected_strokes(&app).await,selected);
    }
}

#[semio_framework_async_macros::async_test]
async fn created_layers_are_painted_and_selected_for_immediate_editing() {
    let (mut app,meta)=inline_selection_app().await;
    for kind in ["shape:rect","shape:ellipse","shape:line","shape:polygon","text"] {
        settled(&mut app,DrawingCommand::AddLayer(add_layer::AddLayer {kind:kind.into()}),&meta).await;
        let snapshot=app.snapshot().unwrap();
        let layer=snapshot.layers.last().unwrap();
        let base=semio_s_artifact_draw_drawing_model::layer_base(layer);
        assert_eq!(selected_strokes(&app).await,vec![base.id.clone()]);
        let scene=semio_s_artifact_draw_drawing_scene::flatten_drawing_document_to_scene_nodes(&snapshot);
        let node=scene.iter().find(|node|node.id==base.id).unwrap();
        assert!(node.fill.is_some() || node.stroke.as_ref().is_some_and(|stroke|stroke.width>0.0));
    }
}

#[test]
fn drawing_canvas_initial_framing_uses_world_bounds_and_respects_restored_navigation() {
    let layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Off origin")).to_string().into()).expect("nonempty authored identity"), "Off origin",vec![semio_s_artifact_draw_drawing_model::PathSegment::Move { to: [-20.0,-30.0] },semio_s_artifact_draw_drawing_model::PathSegment::Line { to: [80.0,70.0] }].into());
    let mut document = semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot { layers: vec![layer].into(),artboard: None,..Default::default() };
    let mut config = canvas_window::config::DrawingCanvasWindowConfig::default();
    let preview = DrawingGesturePreview::default();
    let mut producer=semio_s_artifact_draw_drawing_scene::preparation::DocumentVectorJob::new(&document,geometry_session::limits(),geometry_session::algorithms()).unwrap();
    while !producer.advance(4096).unwrap().done{}
    let plan=producer.result().unwrap();
    drop(producer);
    let mut paint=semio_s_artifact_draw_drawing_scene::paint::scene::ScenePaintJob::new(plan,0.001,geometry_session::paint_limits());
    while !paint.advance(4096).unwrap().done{}
    let(mut close,prepared)=paint.into_retirement();
    while !close.advance(4096).unwrap().done{}
    let plan=prepared.unwrap();
    let scene = canvas_scene_of_tree(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert_eq!(scene.framing.as_ref().unwrap().bounds,[-20.0,-30.0,80.0,70.0]);
    document.artboard = Some(semio_s_artifact_draw_drawing_model::DrawingArtboard { width: 1024.0,height: 1024.0 });
    let scene = canvas_scene_of_tree(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert_eq!(scene.framing.as_ref().unwrap().bounds,[-20.0,-30.0,1024.0,1024.0]);
    config.framed = true;
    config.viewport = store::Viewport2d { x: 777.0,y: -333.0,zoom: 2.0 };
    let scene = canvas_scene_of_tree(semio_framework_plugin::built_to_component_tree(canvas_window::render(Some(&plan),1,&document,&config,&preview,DRAWING_DEFAULT_UTILITY,&[],&[]).unwrap()));
    assert!(scene.framing.is_none());
    assert_eq!((scene.camera_x,scene.camera_y,scene.zoom),(777.0,-333.0,2.0));
    let mut close=semio_s_artifact_draw_drawing_scene::paint::scene::PreparedSceneCloseJob::new(plan);
    while !close.advance(4096).unwrap().done{}
}

#[semio_framework_async_macros::async_test]
async fn text_content_and_size_each_undo_as_one_selection_edit() {
    for (field, value) in [("textContent", "Grüße 🌍\n123"), ("textSize", "36")] {
        let mut app = drawing_app().await;
        let mut first = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
        let mut second = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut first).id = "first".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second).id = "second".into();
        let DrawingLayerNode::Text(text) = &mut second else { unreachable!() };
        text.content = "Different".into();
        text.size = 18.0;
        let before = DrawingSnapshot { id: "text-history".into(), layers: vec![first, second].into(), ..Default::default() };
        let mut after = before.clone();
        for layer in &mut after.layers {
            let DrawingLayerNode::Text(text) = layer else { unreachable!() };
            if field == "textContent" { text.content = value.into(); } else { text.size = 36.0; }
        }
        load_drawing_fixture(&mut app, &before);
        artifact_laws::assert_undo_redo_round_trip(&mut app.0, DrawingCommand::PatchLayers(patch_layers::PatchLayers { index:None, layer_ids: vec!["first".into(), "second".into()], field: field.into(), value: value.into() }), |app| app.snapshot().unwrap(), before, after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn select_all_discovers_unvisited_layers_and_prunes_deleted_selection() {
    let (mut app, meta) = inline_selection_app().await;
    let first = semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
    let second = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
    let ids = vec![layer_id(&first).to_string(), layer_id(&second).to_string()];
    let snapshot = DrawingSnapshot { id: "select-all-topology".into(), layers: vec![first, second].into(), ..Default::default() };
    load_drawing_fixture(&mut app, &snapshot);
    let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "selectAll", None, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("Select All admission");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.expect("Select All publication");
    assert_eq!(selected_strokes(&app).await, ids);
    assert_eq!(app.snapshot().unwrap(), snapshot, "selection must not edit the drawing");
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: ids[0].clone() }), &meta).await;
    assert_eq!(selected_strokes(&app).await, vec![ids[1].clone()]);
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: ids[1].clone() }), &meta).await;
    assert!(selected_strokes(&app).await.is_empty());
    let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "selectAll", None, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("empty Select All admission");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.expect("empty Select All publication");
    assert!(selected_strokes(&app).await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn transform_handles_render_and_commit_once_through_the_registered_editor() {
    for handle in [4_usize,8_usize] {
        for (cancelled,shift,alt) in [(false,false,false),(true,false,false),(false,true,true),(true,true,true)] {
            let (mut app,mut meta)=inline_selection_app().await;
            meta.view_state.as_mut().unwrap().active_utility_id=Some("selectDirect".into());
            let mut child=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Transform target")).to_string().into()).expect("nonempty authored identity"), "Transform target");
            semio_s_artifact_draw_drawing_model::layer_base_mut(&mut child).id="child".into();
            let mut parent=semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Affine parent")).to_string().into()).expect("nonempty authored identity"), "Affine parent");
            semio_s_artifact_draw_drawing_model::layer_base_mut(&mut parent).transform=semio_s_artifact_draw_drawing_model::DrawingTransform {x:20.0,y:30.0,scale_x:2.0,scale_y:3.0,rotation:0.3,shear:0.5};
            let DrawingLayerNode::Group(group)=&mut parent else {unreachable!()};
            group.children.push(child);
            let before=DrawingSnapshot {id:"handle-gesture".into(),layers:vec![parent].into(),..Default::default()};
            load_drawing_fixture(&mut app,&before);
            settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
            let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"child"})]).unwrap();
            let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.unwrap();
            let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
            let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
            assert_eq!(records.iter().filter(|row|row["id"].as_str().is_some_and(|id|id.starts_with("overlay:transform-handle:"))).count(),9);
            let mut producer=semio_s_artifact_draw_drawing_scene::preparation::DocumentVectorJob::new(&before,semio_s_artifact_draw_drawing_editor_interaction::geometry_session::limits(),semio_s_artifact_draw_drawing_editor_interaction::geometry_session::algorithms()).unwrap();
            while !producer.advance(4096).unwrap().done{}let(mut cleanup,plan)=producer.into_retirement();while !cleanup.advance(4096).unwrap().done{}
            let mut producer=semio_s_artifact_draw_drawing_scene::paint::scene::ScenePaintJob::new(plan.unwrap(),0.001,semio_s_artifact_draw_drawing_scene::paint::scene::PaintedSceneLimits{max_nodes:1024,max_segments:65536,max_points:262144,max_contours:65536,max_work:1000000000});
            while !producer.advance(4096).unwrap().done{}let(mut cleanup,prepared)=producer.into_retirement();while !cleanup.advance(4096).unwrap().done{}let prepared=prepared.unwrap();
            let [x,y,r,b]=semio_s_artifact_draw_drawing_scene::paint::scene::query::prepared_selection_bounds(&prepared,&["child".into()]).unwrap().unwrap();let bounds=[x,y,r-x,b-y];
            let mut cleanup=semio_s_artifact_draw_drawing_scene::paint::scene::PreparedSceneCloseJob::new(prepared);while !cleanup.advance(4096).unwrap().done{}
            let start=semio_s_artifact_draw_drawing_geometry::handles::handle_points(bounds,1.0)[handle];
            let center=[bounds[0]+bounds[2]*0.5,bounds[1]+bounds[3]*0.5];
            let end=if handle==8 {[center[0]-(start[1]-center[1]),center[1]+start[0]-center[0]]} else {[start[0]+30.0,start[1]+20.0]};
            let expected=semio_s_artifact_draw_drawing_geometry::handles::handle_matrix(handle,bounds,start,end,shift,alt).unwrap();
            settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:start[0]+400.0,y:start[1]+300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
            settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {shift:!shift,alt:!alt,x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,samples:vec![]}),&meta).await;
            let (_,moved)=settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift, alt, x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,samples:vec![]}),&meta).await;
            assert!(!moved.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(),before);
            let (_,released)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt, x:end[0]+400.0,y:end[1]+300.0,width:800.0,height:600.0,shift,ctrl:false,meta:false,cancelled}),&meta).await;
            if cancelled {assert_eq!(app.snapshot().unwrap(),before);} else {
                assert_one_artifact_publication(&released);
                let after=app.snapshot().unwrap();
                let original=semio_s_artifact_draw_drawing_scene::flatten_drawing_document_to_scene_nodes(&before)[0].transform;
                let actual=semio_s_artifact_draw_drawing_scene::flatten_drawing_document_to_scene_nodes(&after)[0].transform;
                let wanted=semio_s_artifact_draw_drawing_geometry::multiply(expected,original);
                for index in 0..6 {assert!((actual[index]-wanted[index]).abs()<1e-9);}
                artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
                assert_eq!(app.snapshot().unwrap(),before);
                artifact_laws::settle_history_verb(&mut app.0, "redo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
                assert_eq!(app.snapshot().unwrap(),after);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn node_drag_previews_without_mutation_and_commits_one_undoable_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let source=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Cubic {ctrl1:[10.0,20.0],ctrl2:[30.0,20.0],to:[40.0,0.0]}];
        let mut path=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve",source.clone().into());
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).id="curve".into();
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).transform.scale_x=2.0;
        let before=DrawingSnapshot {id:"node-drag".into(),layers:vec![path].into(),..Default::default()};
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"curve"})]).unwrap();
        let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:421.0,y:321.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        assert_eq!(selected.len(),1);
        let selected_point=interaction::points::parse_point_id(&selected[0]).unwrap();
        assert_eq!((selected_point.layer_id,selected_point.index,selected_point.point),("curve",1,semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Control1));
        assert_eq!(selected_point.geometry,interaction::points::geometry_id(&source).unwrap());
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:441.0,y:331.0,width:800.0,height:600.0,shift:false,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        let node=records.iter().find(|node|node["id"]=="curve").unwrap();
        assert_eq!(node["segments"][1]["ctrl1"],serde_json::json!([20.0,30.0]));
        assert!(records.iter().any(|node|node["id"].as_str().is_some_and(|id|id.starts_with("overlay:node:"))));
        assert!(records.iter().any(|node|node["id"]=="overlay:node:curve:selected-controls" && node["segments"].as_array().is_some_and(|segments|!segments.is_empty())));
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:441.0,y:331.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled}),&meta).await;
        if cancelled {assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);} else {
            assert_one_artifact_publication(&receipt);
            let after=app.snapshot().unwrap();
            let DrawingLayerNode::Path(path)=&after.layers[0] else {panic!("Expected path")};
            let rebound=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
            assert_eq!(rebound.len(),1);
            assert_ne!(rebound,selected);
            assert_eq!(interaction::points::parse_point_id(&rebound[0]).unwrap().geometry,interaction::points::geometry_id(&path.segments).unwrap());
            assert_eq!(path.segments[1],semio_s_artifact_draw_drawing_model::PathSegment::Cubic {ctrl1:[20.0,30.0],ctrl2:[30.0,20.0],to:[40.0,0.0]});
            artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut app.0, "redo", meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;assert_eq!(app.snapshot().unwrap(),after);
        }
        assert_eq!(selected_strokes(&app).await,vec!["curve".to_string()]);
    }
}
