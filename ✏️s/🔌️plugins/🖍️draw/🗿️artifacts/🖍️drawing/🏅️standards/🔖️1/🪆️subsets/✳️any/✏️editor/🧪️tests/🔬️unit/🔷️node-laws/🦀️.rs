//! 🔷️ Drawing editor unit laws over node edits: point click, keyboard nudges, fill rule, alignment, layer stack, ungroup, delete, node marquee and PNG export.

use super::*;
use semio_framework_test_fixtures::fixture_text;

#[semio_framework_async_macros::async_test]
async fn point_click_persists_and_local_position_rebinds_but_topology_edit_prunes() {
    use semio_s_artifact_draw_drawing_geometry::editing::{PathEdit, PathPoint};
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let mut layer=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Line")).to_string().into()).expect("nonempty authored identity"), "Line",vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,0.0]}].into());
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).id="path".into();
    let before=DrawingSnapshot {id:"point-selection".into(),layers:vec![layer].into(),..Default::default()};
    load_drawing_fixture(&mut app,&before);
    settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
    let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"path"})]).unwrap();
    let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
    artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:410.0,y:300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
    settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:410.0,y:300.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
    assert_eq!(app.snapshot().unwrap(),before);
    let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
    assert_eq!(selected.len(),1);
    let point=interaction::points::parse_point_id(&selected[0]).unwrap();
    assert_eq!((point.layer_id,point.index,point.point),("path",1,PathPoint::Anchor));
    settled(&mut app,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Coordinate {index:1,point:PathPoint::Anchor,axis:PathAxis::X,value:15.0})}),&meta).await;
    let rebound=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
    assert_eq!(rebound.len(),1);assert_ne!(selected,rebound);
    let point=interaction::points::parse_point_id(&rebound[0]).unwrap();
    assert_eq!((point.layer_id,point.index,point.point),("path",1,PathPoint::Anchor));
    settled(&mut app,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Reverse)}),&meta).await;
    assert!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.is_empty());
    assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);
}

#[semio_framework_async_macros::async_test]
async fn every_keyboard_nudge_moves_document_axes_and_undoes_once() {
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🎮️commands/🕹️nudge-selection/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture.as_array().unwrap() {
        let (mut app,meta)=inline_selection_app().await;
        let mut layer=semio_s_artifact_draw_drawing_model::create_layer_by_kind(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("shape:rect")).to_string().into()).expect("nonempty authored identity"), "shape:rect");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).id="shape".into();
        let before=DrawingSnapshot {id:"nudge".into(),layers:vec![layer].into(),..Default::default()};
        load_drawing_fixture(&mut app,&before);
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"shape"})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        let command=args_bridge::command_from_action(row["action"].as_str().unwrap(),None).unwrap();
        store::os_store::test_support::assert_op_line_round_trip(&command);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let (_,receipt)=settled(&mut app,command,&meta).await;
        assert_one_artifact_publication(&receipt);
        let after=app.snapshot().unwrap();
        let transform=&semio_s_artifact_draw_drawing_model::layer_base(&after.layers[0]).transform;
        assert_eq!([transform.x,transform.y],serde_json::from_value::<[f64;2]>(row["delta"].clone()).unwrap());
        artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        assert_eq!(app.snapshot().unwrap(),before);
        artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        assert_eq!(app.snapshot().unwrap(),after);
    }
}

#[semio_framework_async_macros::async_test]
async fn node_keyboard_nudges_rebind_selection_and_use_parent_axes() {
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let source=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Cubic {ctrl1:[2.0,0.0],ctrl2:[8.0,0.0],to:[10.0,0.0]}];
    let mut path=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve",source.clone().into());
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).id="path".into();
    let mut group=semio_s_artifact_draw_drawing_model::create_layer_by_kind(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("group")).to_string().into()).expect("nonempty authored identity"), "group");
    if let DrawingLayerNode::Group(group)=&mut group {
        group.base.id="group".into();group.base.transform.rotation=std::f64::consts::FRAC_PI_2;group.base.transform.scale_x=2.0;group.base.transform.scale_y=4.0;group.base.transform.x=100.0;group.base.transform.y=200.0;group.children=vec![path].into();
    }
    let before=DrawingSnapshot {id:"point-nudge".into(),layers:vec![group].into(),..Default::default()};
    load_drawing_fixture(&mut app,&before);
    let point=interaction::points::point_id("path",&interaction::points::geometry_id(&source).unwrap(),1,semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Anchor).unwrap();
    for (domain,granularity,id) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,"path".to_owned()),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,point)] {
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":granularity,"id":id})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    }
    for step in 1..=2 {
        let (_,receipt)=settled(&mut app,DrawingCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),&meta).await;
        assert_one_artifact_publication(&receipt);
        let after=app.snapshot().unwrap();
        let Some(DrawingLayerNode::Path(path))=semio_s_artifact_draw_drawing_model::find_drawing_layer(&after,"path") else {panic!("Path missing")};
        let semio_s_artifact_draw_drawing_model::PathSegment::Cubic {ctrl1,ctrl2,to}=path.segments[1] else {panic!("Cubic missing")};
        assert_eq!(ctrl1,[2.0,0.0]);
        assert!((to[0]-10.0).abs()<1e-10 && (to[1]+2.5*f64::from(step)).abs()<1e-10);
        assert!((ctrl2[0]-8.0).abs()<1e-10 && (ctrl2[1]-to[1]).abs()<1e-10);
        let selection=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        assert_eq!(selection.len(),1);
        assert_eq!(interaction::points::parse_point_id(&selection[0]).unwrap().geometry,interaction::points::geometry_id(&path.segments).unwrap());
    }
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.snapshot().unwrap(),before);
}

#[semio_framework_async_macros::async_test]
async fn modified_node_picks_and_combined_drag_preserve_layers_and_one_history_edit() {
    for cancelled in [true,false] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let segments=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,0.0]}];
        let mut first=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First",segments.clone().into());
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut first).id="first".into();
        let mut second=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second",segments.into());
        let base=semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second);base.id="second".into();base.transform.x=40.0;base.transform.scale_x=2.0;base.transform.scale_y=2.0;
        let before=DrawingSnapshot {id:"multi-node-drag".into(),layers:vec![first,second].into(),..Default::default()};
        load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        let targets=serde_json::to_string(&[serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"first"}),serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":"second"})]).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        for (x,y,shift,count) in [(410.0,300.0,false,1),(460.0,300.0,true,2),(700.0,500.0,true,2),(460.0,300.0,true,1),(460.0,300.0,true,2)] {
            settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x,y,shift,width:800.0,height:600.0,..Default::default()}),&meta).await;
            settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x,y,shift,width:800.0,height:600.0,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
            assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.len(),count);
            assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
            assert_eq!(app.snapshot().unwrap(),before);
        }
        let selected=app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.clone();
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:410.0,y:300.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:420.0,y:310.0,width:800.0,height:600.0,shift:false,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.snapshot().unwrap(),before);
        let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,meta.view_state.as_ref().unwrap()).await.unwrap());
        let records:Vec<serde_json::Value>=serde_json::from_str(&scene.layers_json).unwrap();
        assert_eq!(records.iter().find(|row|row["id"]=="first").unwrap()["segments"][1]["to"],serde_json::json!([20.0,10.0]));
        assert_eq!(records.iter().find(|row|row["id"]=="second").unwrap()["segments"][1]["to"],serde_json::json!([15.0,5.0]));
        let (_,receipt)=settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:420.0,y:310.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled}),&meta).await;
        if cancelled {assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,selected);} else {
            assert_one_artifact_publication(&receipt);
            let after=app.snapshot().unwrap();assert_ne!(after,before);
            assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids.len(),2);
            artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),before);
            artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),after);
        }
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:700.0,y:500.0,width:800.0,height:600.0,..Default::default()}),&meta).await;
        settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:700.0,y:500.0,width:800.0,height:600.0,shift:false,alt:false,ctrl:false,meta:false,cancelled:false}),&meta).await;
        assert!(app.interaction_state().await.selection.get(DRAWING_POINT_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));
        assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
    }
}

#[semio_framework_async_macros::async_test]
async fn fill_rule_selection_edit_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let before:DrawingSnapshot=serde_json::from_str(fixture_text!("../../../../../../🎨️style/🧪️tests/🧫️fixtures/🧬️mutations/🌀️set-layer-fill-rule/🌀️evenodd/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mut before=before;
    let mut second=before.layers[0].clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second).id="shape-b".into();
    before.layers.push(second);
    let mut after=before.clone();
    for layer in &mut after.layers {semio_s_artifact_draw_drawing_model::layer_base_mut(layer).attributes.fill_rule=semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero;}
    let ids=before.layers.iter().map(|layer|semio_s_artifact_draw_drawing_model::layer_id(layer).to_string()).collect();
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::PatchLayers(patch_layers::PatchLayers { index:None,layer_ids:ids,field:"fillRule".into(),value:"nonzero".into()}),|app|app.snapshot().unwrap(),before,after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn alignment_across_groups_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let mut first=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut first).id="first".into();
    let mut second=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
    let base=semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second);base.id="second".into();base.transform.x=20.0;base.transform.y=20.0;
    let mut group=semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};
    body.base.id="group".into();body.base.transform.scale_x=2.0;body.base.transform.scale_y=3.0;body.children=vec![first,second].into();
    let mut third=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Third")).to_string().into()).expect("nonempty authored identity"), "Third");
    let base=semio_s_artifact_draw_drawing_model::layer_base_mut(&mut third);base.id="third".into();base.transform.x=-20.0;
    let before=DrawingSnapshot {id:"alignment-history".into(),layers:vec![group,third].into(),..Default::default()};
    let mut after=before.clone();
    let DrawingLayerNode::Group(body)=&mut after.layers[0] else {unreachable!()};
    for layer in &mut body.children {semio_s_artifact_draw_drawing_model::layer_base_mut(layer).transform.x=-10.0;}
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["first".into(),"second".into(),"third".into()],operation:"alignLeft".into()}),|app|app.snapshot().unwrap(),before,after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn layer_stack_steps_undo_as_one_history_entry() {
    for (operation,order) in [("bringForward",["a","d","b","c"]),("sendBackward",["b","c","a","d"])] {
        let mut app=drawing_app().await;
        let layers=["a","b","c","d"].into_iter().map(|id| {
            let mut layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit(((id)).to_string().into()).expect("nonempty authored identity"), id);
            semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).id=id.into();layer
        }).collect::<Vec<_>>();
        let before=DrawingSnapshot {id:"stack-history".into(),layers:layers.into_iter().collect(),..Default::default()};
        let after=DrawingSnapshot {layers:order.into_iter().map(|id|before.layers.iter().find(|layer|semio_s_artifact_draw_drawing_model::layer_id(layer)==id).unwrap().clone()).collect(),..before.clone()};
        load_drawing_fixture(&mut app,&before);
        artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["c".into(),"b".into()],operation:operation.into()}),|app|app.snapshot().unwrap(),before,after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn ungroup_selects_promoted_children_and_undoes_as_one_history_entry() {
    let (mut app,meta)=inline_selection_app().await;
    let mut first=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
    let base=semio_s_artifact_draw_drawing_model::layer_base_mut(&mut first);base.id="first".into();base.transform.x=2.0;
    let mut second=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second).id="second".into();
    let mut group=semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};
    body.base.id="group".into();body.base.transform.x=5.0;body.children=vec![first.clone(),second.clone()].into();
    let before=DrawingSnapshot {id:"ungroup-history".into(),layers:vec![group].into(),..Default::default()};
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut first).transform.x=7.0;
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut second).transform.x=5.0;
    let after=DrawingSnapshot {layers:vec![first,second].into(),..before.clone()};
    load_drawing_fixture(&mut app,&before);
    let (_,receipt)=settled(&mut app,DrawingCommand::EditSelection(edit_selection::EditSelection {ids:vec!["group".into()],operation:"ungroup".into()}),&meta).await;
    assert_one_artifact_publication(&receipt);
    assert_eq!(app.snapshot().unwrap(),after);
    assert_eq!(selected_strokes(&app).await,vec!["first".to_owned(),"second".to_owned()]);
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),before);
    artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),after);
}

#[semio_framework_async_macros::async_test]
async fn group_isolation_selection_edit_undoes_as_one_history_entry() {
    let mut app=drawing_app().await;
    let before:DrawingSnapshot=serde_json::from_str(fixture_text!("../../../../../../🎨️style/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(fixture_text!("../../../../../../🎨️style/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/➡️after/🔣️.json")).unwrap();
    load_drawing_fixture(&mut app,&before);
    artifact_laws::assert_undo_redo_round_trip(&mut *app,DrawingCommand::PatchLayers(patch_layers::PatchLayers { index:None,layer_ids:vec!["group-a".into()],field:"isolation".into(),value:"true".into()}),|app|app.snapshot().unwrap(),before,after, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_selected_nodes_commits_once_clears_points_and_undoes_exactly() {
    let (mut app,mut meta)=inline_selection_app().await;
    meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
    let segments=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,10.0]}];
    let mut path=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path",segments.clone().into());
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).id="path".into();
    let before=DrawingSnapshot {id:"node-delete-history".into(),layers:vec![path].into(),..Default::default()};
    load_drawing_fixture(&mut app,&before);
    let geometry=interaction::points::geometry_id(&segments).unwrap();
    let points=[0,1].iter().map(|index|interaction::points::point_id("path",&geometry,*index,semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Anchor).unwrap()).collect::<Vec<_>>();
    for (domain,granularity,ids) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,vec!["path".into()]),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,points)] {
        let targets=serde_json::to_string(&ids.into_iter().map(|id|serde_json::json!({"granularity":granularity,"id":id})).collect::<Vec<_>>()).unwrap();
        let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();
        artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    }
    let (_,receipt)=settled(&mut app,DrawingCommand::DeleteSelection(delete_selection::DeleteSelection {}),&meta).await;
    assert_one_artifact_publication(&receipt);
    let after=app.snapshot().unwrap();
    let Some(DrawingLayerNode::Path(path))=semio_s_artifact_draw_drawing_model::find_drawing_layer(&after,"path") else {panic!("Path missing")};
    assert_eq!(path.segments,vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[10.0,10.0]}].into());
    assert!(app.interaction_state().await.selection.get(DRAWING_POINT_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));
    assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);
    artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),before);
    artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;assert_eq!(app.snapshot().unwrap(),after);
}

#[semio_framework_async_macros::async_test]
async fn node_marquee_preserves_layers_and_supports_merge_and_cancellation() {
    for (shift,ctrl,cancelled,initial,expected) in [(false,false,false,vec![0],vec![1,2]),(true,false,false,vec![1],vec![2]),(false,true,false,vec![0],vec![0,1,2]),(false,false,true,vec![0],vec![0])] {
        let (mut app,mut meta)=inline_selection_app().await;
        meta.view_state.as_mut().unwrap().active_utility_id=Some("editNodes".into());
        let segments=vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,10.0]}];
        let geometry=interaction::points::geometry_id(&segments).unwrap();
        let point=|index|interaction::points::point_id("path",&geometry,index,semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Anchor).unwrap();
        let mut path=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path",segments.into());semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).id="path".into();
        let before=DrawingSnapshot {id:"node-marquee".into(),layers:vec![path].into(),..Default::default()};load_drawing_fixture(&mut app,&before);
        settled(&mut app,DrawingCommand::SetCamera(set_camera::SetCamera {camera:store::Viewport2d {x:0.0,y:0.0,zoom:1.0}}),&meta).await;
        for (domain,granularity,ids) in [(DRAWING_INTERACTION_DOMAIN,DRAWING_INTERACTION_GRANULARITY,vec!["path".into()]),(DRAWING_POINT_DOMAIN,DRAWING_POINT_GRANULARITY,initial.iter().copied().map(point).collect())] {
            let targets=serde_json::to_string(&ids.into_iter().map(|id|serde_json::json!({"granularity":granularity,"id":id})).collect::<Vec<_>>()).unwrap();
            let admission=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":domain,"targets":targets,"merge":"replace","method":"pick"}))),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,admission).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        }
        settled(&mut app,DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {x:404.0,y:292.0,width:800.0,height:600.0,shift,ctrl,..Default::default()}),&meta).await;
        settled(&mut app,DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {x:412.0,y:312.0,width:800.0,height:600.0,shift,alt:false,samples:vec![]}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,initial.iter().copied().map(point).collect::<Vec<_>>());
        assert_eq!(app.snapshot().unwrap(),before);
        settled(&mut app,DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {x:412.0,y:312.0,width:800.0,height:600.0,shift,ctrl,meta:false,alt:false,cancelled}),&meta).await;
        assert_eq!(app.interaction_state().await.selection[DRAWING_POINT_DOMAIN].ids,expected.iter().copied().map(point).collect::<Vec<_>>());
        assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);assert_eq!(app.snapshot().unwrap(),before);
    }
}

#[semio_framework_async_macros::async_test]
async fn mounted_vector_editor_closes_its_registered_read_without_a_live_maintenance_tick(){let mut app=drawing_app().await;let layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Registered geometry")).to_string().into()).expect("nonempty authored identity"), "Registered geometry");let snapshot=DrawingSnapshot{id:"mounted-read-close".into(),layers:vec![layer].into(),..Default::default()};load_drawing_fixture(&mut app,&snapshot);let view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);let scene=canvas_scene(rendered_drawing_canvas(&mut app,None,&view).await.unwrap());let records:serde_json::Value=serde_json::from_str(&scene.layers_json).unwrap();assert!(records.as_array().unwrap().iter().any(|record|record["id"].as_str().is_some_and(|id|layer_id(&snapshot.layers[0]).eq_str(id))));semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);assert!(app.close_terminal_is_empty());}

#[semio_framework_async_macros::async_test]
async fn registered_pointer_yields_while_real_scene_job_is_pending_and_then_selects_once(){
    use semio_framework_plugin::reactor::jobs::{start_job,step_job,cancel_job,JobBudget,JobStep};
    let(mut app,mut meta)=inline_selection_app().await;meta.view_state.as_mut().unwrap().active_utility_id=Some("selectDirect".into());
    let mut layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Pending paint")).to_string().into()).expect("nonempty authored identity"), "Pending paint");semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).attributes.fill=Some(semio_s_artifact_draw_drawing_model::FillStyle::Solid{color:[0.0,0.0,0.0,1.0]});let selected=layer_id(&layer).to_string();
    let before=DrawingSnapshot{id:"pending-pointer".into(),layers:vec![layer].into(),..Default::default()};load_drawing_fixture(&mut app,&before);
    let mut geometry=None;for effect in app.pending_effects(meta.view_state.as_ref()).await{match effect{Effect::CancelJob{job}=>cancel_job(job).await,Effect::SpawnJob{job,kind,input,..}if kind=="semio.draw.mounted-vector"=>{assert!(geometry.replace(job).is_none());start_job(job,&kind,&input).await;},_=>{}}}let geometry=geometry.unwrap();
    assert!(matches!(step_job(geometry,JobBudget{fuel:1,deadline_ms:8}).await,JobStep::Running(_)));
    app.dispatch_typed(DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown{x:400.0,y:300.0,width:800.0,height:600.0,..Default::default()}),&meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    for _ in 0..8{app.maintenance_step(1,4096).unwrap();app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap();assert!(app.has_pending_typed_operations());assert!(app.take_typed_operation_result_page(meta.instance_id).is_none());assert!(app.take_typed_operation_completion().await.unwrap().is_none());assert!(app.interaction_state().await.selection.get(DRAWING_INTERACTION_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));assert_eq!(app.snapshot().unwrap(),before);}
    let(mut units,mut done)=(0,false);for _ in 0..1000000{units+=1;match step_job(geometry,JobBudget{fuel:1,deadline_ms:8}).await{JobStep::Running(_)=>{if units%32==0{app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap();}},JobStep::Done(_)=>{done=true;break;},JobStep::Failed(error)=>panic!("{}",String::from_utf8_lossy(&error))}}assert!(done&&units>1);
    let receipt=artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));assert_eq!(receipt.completions,1);assert_eq!(app.interaction_state().await.selection[DRAWING_INTERACTION_DOMAIN].ids.clone(),vec![selected]);assert_eq!(app.snapshot().unwrap(),before);
    settled(&mut app,DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape{}),&meta).await;artifact_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);assert!(app.close_terminal_is_empty());
}

/// 📤️ The registered PNG command publishes exact neutral pixels once and changes no source or history.
#[semio_framework_async_macros::async_test]
async fn png_export_command_publishes_one_real_download_from_the_registered_factory(){
 let sources:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();let mut samples:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../🎮️commands/📤️export-document/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let styles:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../🎮️commands/📤️export-document/🧪️tests/🧫️fixtures/🎨️styles/🔣️.json")).unwrap();for style in styles{samples.push(serde_json::json!({"name":style["name"],"source":"rectangle","width":2,"height":1,"transparent":true,"expected":style["expected"],"layers":style["layers"],"channelTolerance":style["channelTolerance"]}));}
 for sample in samples{let row=sources.iter().find(|row|row["name"]==sample["source"]).unwrap();let mut authored=row["document"].clone();if let Some(layers)=sample.get("layers"){authored["layers"]=layers.clone();}authored["artboard"]=serde_json::json!({"width":2,"height":1});let document:DrawingSnapshot=serde_json::from_value(authored).unwrap();let(mut app,meta)=inline_selection_app().await;load_drawing_fixture(&mut app,&document);let before=app.snapshot().unwrap();let edits=app.history_snapshot().await.unwrap().edit_count;let command=export_document::ExportDocument{format:"png".into(),width:Some(2),height:Some(1),transparent:Some(sample["transparent"].as_bool().unwrap())};let(result,receipt)=settled(&mut app,DrawingCommand::ExportDocument(command),&meta).await;
  assert!(result.mutations.is_empty());assert_eq!(receipt.completions,1);assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact));let[Effect::DownloadMediaExport{filename,mime_type,data,encoding}]=receipt.effects.as_slice()else{panic!("one sealed PNG download required")};assert!(filename.ends_with(".png"));assert_eq!(mime_type,"image/png");assert_eq!(encoding.as_deref(),Some("base64"));let bytes=base64_codec::base64_standard_decode(data).unwrap();let mut decoder=png::Decoder::new(std::io::Cursor::new(&bytes)).read_info().unwrap();let mut pixels=vec![0;decoder.output_buffer_size()];let info=decoder.next_frame(&mut pixels).unwrap();assert_eq!((info.width,info.height),(2,1));let expected:Vec<u8>=serde_json::from_value(sample["expected"].clone()).unwrap();let tolerance=sample["channelTolerance"].as_u64().unwrap_or(0)as u8;assert_eq!(info.buffer_size(),expected.len());for(actual,expected)in pixels[..info.buffer_size()].iter().zip(expected.iter()){assert!(actual.abs_diff(*expected)<=tolerance);}assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.history_snapshot().await.unwrap().edit_count,edits);artifact_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);assert!(app.close_terminal_is_empty());
 }
}
