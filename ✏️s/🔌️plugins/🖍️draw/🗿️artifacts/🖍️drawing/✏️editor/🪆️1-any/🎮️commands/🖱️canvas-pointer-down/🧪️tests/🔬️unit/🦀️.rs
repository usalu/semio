use super::*;

#[test]
fn nested_curve_selection_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️selection/🔣️.json")).unwrap();
    let segments: Vec<PathSegment> = serde_json::from_value(fixture["segments"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut path = create_drawing_path_layer("Curve", segments.clone());
        crate::schema::layer_base_mut(&mut path).id = "curve".into();
        crate::schema::layer_base_mut(&mut path).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});
        let mut group = crate::schema::create_drawing_group_layer("Parent");
        let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
        let [x,y,scale_x,scale_y,rotation]: [f64;5] = serde_json::from_value(case["parent"].clone()).unwrap();
        body.base.transform = crate::DrawingTransform { x,y,scale_x,scale_y,rotation, shear: 0.0 };
        body.base.visible = case["visible"].as_bool().unwrap_or(true);
        body.base.locked = case["locked"].as_bool().unwrap_or(false);
        body.children.push(path);
        let document = DrawingSnapshot { layers: vec![group], ..Default::default() };
        let point = serde_json::from_value(case["point"].clone()).unwrap();
        let mut query = TracePointerJob::new_query(&document, point, 0.0, case["includeControls"].as_bool().unwrap_or(false));
        while !query.advance(&document) {}
        assert_eq!(query.best.as_ref().map(|hit| hit.layer_id.as_str()), if case["selected"] == true { Some("curve") } else { None }, "{}", case["name"]);
        eprintln!("[DEBUG] curve selection case {} completed in {} work units", case["name"], query.completed_work);
    }
}

#[test]
fn point_selection_keeps_the_frontmost_equal_candidate() {
    let mut back = create_drawing_path_layer("Back", vec![PathSegment::Move { to: [0.0,0.0] },PathSegment::Line { to: [10.0,10.0] }]);
    let mut front = back.clone();
    crate::schema::layer_base_mut(&mut back).id = "back".into();
    crate::schema::layer_base_mut(&mut front).id = "front".into();
    let document = DrawingSnapshot { layers: vec![back,front], ..Default::default() };
    let mut query = TracePointerJob::new_query(&document,[5.0,5.0],0.0,false);
    while !query.advance(&document) {}
    assert_eq!(query.best.unwrap().layer_id,"front");
}

#[test]
fn lasso_samples_yield_and_select_the_polygon_instead_of_its_rectangle() {
    use crate::editor::drawing::commands::canvas_pointer_move::CanvasPointerMove;
    let inside=create_drawing_path_layer("Inside",vec![PathSegment::Move { to: [1.0,1.0] },PathSegment::Line { to: [2.0,2.0] }]);
    let outside=create_drawing_path_layer("Outside",vec![PathSegment::Move { to: [6.0,6.0] },PathSegment::Line { to: [7.0,7.0] }]);
    let expected=layer_id(&inside).to_string();
    let document=DrawingSnapshot { layers: vec![inside,outside],..Default::default() };
    let mut session=DrawingSession::with_active_utility("selectLasso");
    session.window_config.viewport=store::Viewport2d { x:0.0,y:0.0,zoom:1.0 };
    session.step_gesture(drawing_gesture::Event::PointerDown { utility:"selectLasso".into(),world:[0.0,0.0],shift:false,ctrl:false,meta:false },&document,&NoConfig::default());
    let movement=CanvasPointerMove { shift: false, alt: false,  x:50.0,y:60.0,width:100.0,height:100.0,samples:vec![[60.0,50.0],[50.0,60.0]] };
    assert!(session.advance_lasso_move(&movement,&document,&NoConfig::default()).is_none());
    assert_eq!(session.gesture.context.points.len(),2);
    assert!(session.advance_lasso_move(&movement,&document,&NoConfig::default()).is_some());
    assert_eq!(session.gesture.context.points.iter().copied().collect::<Vec<_>>(),vec![[0.0,0.0],[10.0,0.0],[0.0,10.0]]);
    let polygon=LassoPolygon::from_points(&session.gesture.context.points,[0.0,0.0]);
    let mut query=TracePointerJob::new_lasso(&document,polygon);
    while !query.advance(&document) {}
    assert_eq!(query.hits.iter().cloned().collect::<Vec<_>>(),vec![expected]);
    session.step_gesture(drawing_gesture::Event::Escape,&document,&NoConfig::default());
    assert_eq!(session.preview().phase,DrawingGesturePreviewPhase::Idle);
    eprintln!("[DEBUG] lasso consumed the complete pointer batch and selected only enclosed geometry");
}

#[test]
fn long_lasso_decimates_within_fixed_capacity() {
    let mut context=GestureContext::default();
    for index in 0..10_000 { append_lasso_point(&mut context,[index as f64,(index as f64/30.0).sin()*20.0],0.1); }
    assert!(!context.points_overflowed);
    assert!(context.points.len()<=DRAWING_GESTURE_PREVIEW_POINT_CAPACITY);
    assert_eq!(context.points[0],[0.0,0.0]);
    let polygon=LassoPolygon::from_points(&context.points,[10_000.0,0.0]);
    assert_eq!(polygon.as_slice().last(),Some(&[10_000.0,0.0]));
}

#[test]
fn shape_identity_is_replay_stable_and_scoped_to_the_durable_app_operation() {
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: 7,
        parent_document_id: "drawing-document".into(),
        operation_id: 11,
        generation: 13,
        canonical_base_revision: [17; 32],
    };
    let geometry = [10.0, 20.0, 30.0, 40.0];
    let first = shape_drag_id("shapeRect", geometry, 3, Some(&operation));
    assert_eq!(first, shape_drag_id("shapeRect", geometry, 3, Some(&operation)), "replaying the same admitted operation is deterministic");
    assert_ne!(first, shape_drag_id("shapeRect", geometry, 3, Some(&semio_framework_plugin::AppOperationContext { app_instance_id: 8, ..operation.clone() })), "a different live app instance cannot collide at the same document revision and local operation ordinal");
    assert_ne!(first, shape_drag_id("shapeRect", geometry, 4, Some(&operation)), "a second same-geometry creation in the same document uses its document-local ordinal");
}

#[semio_framework_async_macros::async_test]
async fn trace_pointer_step_consumes_at_most_the_fixed_work_budget() {
    let mut document = crate::schema::default_drawing_document("bounded-trace", None);
    let mut segments = vec![PathSegment::Move { to: [0.0, 0.0] }];
    segments.extend((0..256).map(|index| PathSegment::Line { to: [index as f64, index as f64] }));
    document.layers = vec![create_drawing_path_layer("long-path", segments)];
    let mut job = TracePointerJob::new(7, &document, [4.0, 4.0]);

    assert!(!job.advance(&document));
    assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(!job.work.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn continuation_work_helpers_have_synchronous_compile_shape() {
    let _: fn(&mut TracePointerJob, &DrawingSnapshot) -> bool = TracePointerJob::advance;
    let _: fn(Vec<DrawingMutation>, &str) -> Emit<DrawingMutation, NoConfigMutation> = commit_with_utility_reset;
    let _: fn(&DrawingLayerNode) -> (f64, f64, f64, f64) = trace_layer_world_bounds;
}

#[test]
fn stale_generation_and_wrong_owner_cannot_take_retained_trace() {
    let document = crate::schema::default_drawing_document("fresh-trace", None);
    let mut session = DrawingSession::default();
    assert!(session.retain_trace_pointer(TracePointerJob::new(11, &document, [0.0, 0.0])).is_ok());
    let base = format!("unbound:{}", document.id);
    assert!(session.take_trace_pointer(0, &document.id, 0, 12, &base).is_none());
    assert!(session.take_trace_pointer(1, &document.id, 0, 11, &base).is_none());
    assert!(session.take_trace_pointer(0, "foreign", 0, 11, &base).is_none());
    assert!(session.take_trace_pointer(0, &document.id, 0, 11, &base).is_some());
}

#[test]
fn wide_roots_and_groups_enqueue_at_most_two_items_per_work_unit() {
    let leaf = create_drawing_path_layer("leaf", vec![PathSegment::Move { to: [0.0, 0.0] }]);
    let mut roots = crate::schema::default_drawing_document("wide-roots", None);
    roots.layers = vec![leaf.clone(); 10_000];
    let mut roots_job = TracePointerJob::new(8, &roots, [0.0, 0.0]);
    roots_job.advance(&roots);
    assert_eq!(roots_job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(roots_job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);

    let mut group = crate::schema::create_drawing_group_layer("wide");
    let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
    body.children = vec![leaf; 10_000];
    let mut document = crate::schema::default_drawing_document("wide-group", None);
    document.layers = vec![group];
    let mut job = TracePointerJob::new(9, &document, [0.0, 0.0]);
    job.advance(&document);
    assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);
}

#[test]
fn retained_trace_interruption_cancel_and_repeated_cancel_are_exact() {
    let document = crate::schema::default_drawing_document("retained", None);
    let mut session = DrawingSession::default();
    assert!(session.retain_trace_pointer(TracePointerJob::new(91, &document, [4.0, 5.0])).is_ok());
    assert!(!session.cancel_trace_pointer(0, &document.id, 90));
    assert!(session.cancel_trace_pointer(0, &document.id, 91));
    assert!(!session.cancel_trace_pointer(0, &document.id, 91));
}

#[test]
fn marquee_maximum_plus_one_faults_without_unbounded_growth() {
    let mut document = crate::schema::default_drawing_document("marquee-max", None);
    document.layers = (0..=DRAWING_QUERY_HIT_CAPACITY).map(|index| create_drawing_path_layer(&format!("hit-{index}"), vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }])).collect();
    let mut query = TracePointerJob::new_marquee(&document, [-128.0, -128.0], [128.0, 128.0], true);
    let mut turns = 0;
    while !query.advance(&document) {
        turns += 1;
        assert!(turns < 10_000);
    }
    assert!(turns > 1, "the adversarial tree must yield across turns");
    assert!(query.overflowed, "maximum plus one must fault rather than publish a partial selection");
    assert_eq!(query.hits.len(), DRAWING_QUERY_HIT_CAPACITY);
}

#[test]
fn draft_cursor_advances_one_point_per_turn_and_hands_back_one_commit() {
    let document = crate::schema::default_drawing_document("draft-cursor", None);
    let mut points = UiFixedList::default();
    for index in 0..DRAWING_GESTURE_PREVIEW_POINT_CAPACITY {
        assert!(points.try_push([index as f64, index as f64]).is_ok());
    }
    let mut query = DrawingDraftQuery::new("canvasCommitDraft", "pen".into(), points);
    assert!(query.advance(&document).is_none());
    assert_eq!(query.cursor, 1);
    for _ in 1..DRAWING_GESTURE_PREVIEW_POINT_CAPACITY {
        assert!(query.advance(&document).is_none());
    }
    let emit = query.advance(&document).expect("the bounded final turn hands back one commit");
    assert_eq!(emit.artifact_mutations.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_fsm_inputs_settle_in_one_microstep() {
    let mut session = DrawingSession::default();
    for utility in ["selectMarquee", "shapeRect", "pen", "shapePolygon", "trace"] {
        let mut sink = Vec::new();
        let report = fsm::macrostep(&mut session.gesture, drawing_gesture::Event::PointerDown { utility: utility.into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &mut sink, &mut fsm::NullInspector);
        assert!(report.microsteps <= 1, "{utility} used {} microsteps", report.microsteps);
    }
}

#[test]
fn direct_drag_previews_then_commits_parent_space_translation_once() {
    let path = create_drawing_path_layer("Moving",vec![PathSegment::Move { to:[0.0,0.0] },PathSegment::Line { to:[10.0,10.0] }]);
    let id = layer_id(&path).to_string();
    let mut group = crate::schema::create_drawing_group_layer("Parent");
    let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
    body.base.transform = crate::DrawingTransform { x:100.0,y:200.0,scale_x:2.0,scale_y:4.0,rotation:std::f64::consts::FRAC_PI_2, shear: 0.0 };
    body.children.push(path);
    let document = DrawingSnapshot { layers:vec![group],..Default::default() };
    let before = document.clone();
    let start = [80.0,210.0];
    let end = [88.0,220.0];
    let mut query = TracePointerJob::new_query(&document,start,0.0,false);
    while !query.advance(&document) {}
    let mut session = DrawingSession::with_active_utility("selectDirect");
    session.step_gesture(drawing_gesture::Event::PointerDown { utility:"selectDirect".into(),world:start,shift:false,ctrl:false,meta:false },&document,&NoConfig::default());
    prepare_drag(&mut session,&document,start);
    session.move_layer_preview(end);
    assert_eq!(session.preview().transformation,Some((vec![id.clone()],[1.0,0.0,0.0,1.0,8.0,10.0])));
    assert_eq!(document,before);
    let emit = session.finish_layer_move(end,&document,&NoConfig::default()).unwrap();
    assert_eq!(emit.artifact_mutations.len(),1);
    assert_eq!(emit.description.as_deref(),Some("Move selection"));
    assert!(session.preview().transformation.is_none());
    assert!(session.gesture.matches("idle"));
    let DrawingMutation::UpdateLayerTransform(update) = &emit.artifact_mutations[0] else { panic!("Expected one transform edit") };
    assert!((update.transform.x-5.0).abs()<1e-10);
    assert!((update.transform.y+2.0).abs()<1e-10);
    assert_eq!(update.layer_id,id);
    eprintln!("[DEBUG] direct drag retained its preview and emitted one parent-space transform on release");
}

#[test]
fn cancelled_direct_drag_and_subthreshold_click_do_not_mutate() {
    let layer = crate::schema::create_drawing_shape_layer_rect("Moving");
    let document = DrawingSnapshot { layers:vec![layer],..Default::default() };
    let mut query = TracePointerJob::new_query(&document,[0.0,0.0],0.0,false);
    while !query.advance(&document) {}
    assert!(query.best.is_some());
    for cancelled in [true,false] {
        let mut session = DrawingSession::with_active_utility("selectDirect");
        session.step_gesture(drawing_gesture::Event::PointerDown { utility:"selectDirect".into(),world:[0.0,0.0],shift:false,ctrl:false,meta:false },&document,&NoConfig::default());
        prepare_drag(&mut session,&document,[0.0,0.0]);
        let emit = if cancelled {
            session.move_layer_preview([30.0,20.0]);
            crate::editor::drawing::commands::canvas_pointer_up::cancel_gesture(&mut session,&document,&NoConfig::default())
        } else { session.finish_layer_move([1.0,1.0],&document,&NoConfig::default()).unwrap() };
        assert!(emit.artifact_mutations.is_empty());
        assert!(session.preview().transformation.is_none());
        assert!(session.gesture.matches("idle"));
    }
    eprintln!("[DEBUG] direct drag cancellation and clicks leave the document unchanged");
}

#[test]
fn selection_move_preparation_yields_and_rejects_locked_or_missing_targets_atomically() {
    for invalid in ["locked","hidden","missing","singular"] {
        let mut first=crate::schema::create_drawing_shape_layer_rect("First");
        crate::schema::layer_base_mut(&mut first).id="first".into();
        let mut second=crate::schema::create_drawing_shape_layer_rect("Second");
        crate::schema::layer_base_mut(&mut second).id="second".into();
        let mut parent=crate::schema::create_drawing_group_layer("Parent");
        let base=crate::schema::layer_base_mut(&mut parent);
        base.locked=invalid=="locked";
        base.visible=invalid!="hidden";
        base.transform.scale_x=if invalid=="singular" {0.0} else {2.0};
        if let DrawingLayerNode::Group(group)=&mut parent { group.children.push(second); }
        let document=DrawingSnapshot { layers:vec![first,parent],..Default::default() };
        let mut preparation=LayerMovePreparation { ids:vec!["first".into(),if invalid=="missing" {"gone".into()} else {"second".into()}],next:TracePath::root(0),found:0,movement:LayerMove { targets:Vec::new(),start:[0.0,0.0],cursor:[0.0,0.0],active:false,handle:None } };
        let before=document.clone();
        assert_eq!(preparation.advance(&document).unwrap(),false);
        assert_eq!(preparation.movement.targets.len(),1);
        let mut error=false;
        for _ in 0..8 { match preparation.advance(&document) { Err(_)=>{error=true;break;},Ok(true)=>break,Ok(false)=>{} } }
        assert!(error,"{invalid}");
        assert_eq!(document,before);
    }
    eprintln!("[DEBUG] selected movement yields and refuses invalid targets before any mutation");
}

#[test]
fn selection_move_commits_equal_world_displacements_under_distinct_parents() {
    let mut a=crate::schema::create_drawing_shape_layer_rect("A");
    crate::schema::layer_base_mut(&mut a).id="a".into();
    let mut b=crate::schema::create_drawing_shape_layer_rect("B");
    crate::schema::layer_base_mut(&mut b).id="b".into();
    let mut parent=crate::schema::create_drawing_group_layer("Parent");
    let base=crate::schema::layer_base_mut(&mut parent);
    base.transform.rotation=std::f64::consts::FRAC_PI_2;
    base.transform.scale_x=2.0;
    if let DrawingLayerNode::Group(group)=&mut parent { group.children.push(b); }
    let mut document=DrawingSnapshot { layers:vec![a,parent],..Default::default() };
    let before=crate::schema::flatten_drawing_document_to_scene_nodes(&document);
    let mut preparation=LayerMovePreparation { ids:vec!["a".into(),"b".into()],next:TracePath::root(0),found:0,movement:LayerMove { targets:Vec::new(),start:[0.0,0.0],cursor:[0.0,0.0],active:false,handle:None } };
    let mut complete=false;
    for _ in 0..8 { if preparation.advance(&document).unwrap() {complete=true;break;} }
    assert!(complete);
    let mut session=DrawingSession::with_active_utility("selectDirect");
    session.layer_move=Some(preparation.movement);
    let emit=session.finish_layer_move([12.0,8.0],&document,&NoConfig::default()).unwrap();
    assert_eq!(emit.artifact_mutations.len(),2);
    for mutation in emit.artifact_mutations { crate::mutations::apply_drawing_mutation(&mut document,&mutation).unwrap(); }
    let after=crate::schema::flatten_drawing_document_to_scene_nodes(&document);
    for (before,after) in before.iter().zip(after) {
        assert!((after.transform[4]-before.transform[4]-12.0).abs()<1e-10);
        assert!((after.transform[5]-before.transform[5]-8.0).abs()<1e-10);
    }
}

fn prepare_drag(session: &mut DrawingSession, document: &DrawingSnapshot, start: [f64;2]) {
    let mut query=DrawingPointQuery::new("canvasPointerDown",TracePointerJob::new_query(document,start,0.0,false),false,"replace".into(),false);
    while !query.cursor.advance(document) {}
    query.drag_start=Some(start);
    session.point_query=Some(query);
    for _ in 0..1000 {
        if session.prepare_layer_move(document,&[],&[]).unwrap() { session.point_query=None; return; }
    }
    panic!("Movement preparation did not complete");
}

#[test]
fn repeated_pen_and_polygon_drafts_keep_distinct_layers() {
    for utility in ["pen","shapePolygon"] {
        let mut document=DrawingSnapshot::default();
        let mut ids=std::collections::HashSet::new();
        for _ in 0..3 {
            let mut points=UiFixedList::default();
            for point in [[0.0,0.0],[10.0,0.0],[10.0,10.0]] { points.try_push(point).unwrap(); }
            let mut query=DrawingDraftQuery::new("canvasCommitDraft",utility.into(),points);
            let emit=loop {if let Some(emit)=query.advance(&document) {break emit;} };
            let DrawingMutation::CreateLayer(created)=&emit.artifact_mutations[0] else {panic!("Expected creation")};
            assert!(ids.insert(layer_id(&created.layer).to_string()),"{utility} reused a completed draft id");
            for mutation in emit.artifact_mutations {crate::mutations::apply_drawing_mutation(&mut document,&mutation).unwrap();}
        }
        assert_eq!(document.layers.len(),3);
    }
}


#[test]
fn selected_handle_previews_are_ephemeral_and_release_one_absolute_transform() {
    let layer=create_drawing_path_layer("Box",vec![PathSegment::Move {to:[10.0,20.0]},PathSegment::Line {to:[110.0,20.0]},PathSegment::Line {to:[110.0,100.0]},PathSegment::Line {to:[10.0,100.0]},PathSegment::Close]);
    let id=layer_id(&layer).to_string();
    let document=DrawingSnapshot {layers:vec![layer],..Default::default()};
    let before=document.clone();
    for (start,end,label) in [([110.0,100.0],[210.0,180.0],"Resize selection"),([60.0,-8.0],[128.0,60.0],"Rotate selection")] {
        for cancel in [false,true] {
            let mut cursor=TracePointerJob::new_query(&document,start,0.0,false);
            cursor.retain_selection_bounds(&[id.clone()]).unwrap();
            while !cursor.advance(&document) {}
            assert_eq!(cursor.selection_bounds,Some([10.0,20.0,100.0,80.0]));
            let mut query=DrawingPointQuery::new("canvasPointerDown",cursor,false,"replace".into(),false);
            query.drag_start=Some(start);
            let mut session=DrawingSession::with_active_utility("selectDirect");
            session.point_query=Some(query);
            session.step_gesture(drawing_gesture::Event::PointerDown {utility:"selectDirect".into(),world:start,shift:false,ctrl:false,meta:false},&document,&NoConfig::default());
            while !session.prepare_layer_move(&document,&[id.clone()],&[]).unwrap() {}
            assert!(session.layer_move.as_ref().unwrap().handle.is_some());
            session.move_layer_preview(end);
            let preview=session.preview().transformation.unwrap();
            let scene=crate::schema::flatten_drawing_document_with_transformation(&document,Some(&preview));
            assert_eq!(scene[0].transform,preview.1);
            assert_eq!(document,before);
            let emit=if cancel {crate::editor::drawing::commands::canvas_pointer_up::cancel_gesture(&mut session,&document,&NoConfig::default())} else {session.finish_layer_move(end,&document,&NoConfig::default()).unwrap()};
            assert!(session.preview().transformation.is_none());
            assert!(session.gesture.matches("idle"));
            if cancel {assert!(emit.artifact_mutations.is_empty());} else {
                assert_eq!(emit.artifact_mutations.len(),1);
                assert_eq!(emit.description.as_deref(),Some(label));
                let DrawingMutation::UpdateLayerTransform(update)=&emit.artifact_mutations[0] else {panic!("transform mutation")};
                let actual=crate::schema::drawing_transform_to_matrix(&update.transform);
                for index in 0..6 {assert!((actual[index]-preview.1[index]).abs()<1e-10);}
            }
        }
    }
    eprintln!("[DEBUG] resize and rotation preserve the snapshot during preview and emit one transform only on release");
}

#[test]
fn point_selection_reaches_the_painted_path_behind_a_concave_frame() {
    let points=[[0.0,0.0],[100.0,0.0],[100.0,100.0],[80.0,100.0],[80.0,20.0],[0.0,20.0]];
    let mut segments=points.into_iter().enumerate().map(|(i,to)|if i==0{PathSegment::Move {to}}else{PathSegment::Line {to}}).collect::<Vec<_>>();
    segments.push(PathSegment::Close);
    let mut frame=create_drawing_path_layer("Red Frame",segments);
    let mut back=create_drawing_path_layer("Orange",vec![PathSegment::Move {to:[10.0,30.0]},PathSegment::Line {to:[40.0,30.0]},PathSegment::Line {to:[40.0,80.0]},PathSegment::Line {to:[10.0,80.0]},PathSegment::Close]);
    for layer in [&mut frame,&mut back]{crate::schema::layer_base_mut(layer).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});}
    let expected=layer_id(&back).to_string();
    let document=DrawingSnapshot {layers:vec![back,frame],..Default::default()};
    let mut query=TracePointerJob::new_query(&document,[20.0,50.0],1.0,false);
    while !query.advance(&document){}
    assert_eq!(query.best.unwrap().layer_id,expected);
    eprintln!("[DEBUG] Concave frame picking reaches the painted layer behind its empty bounding area");
}

#[test]
fn primitive_picking_uses_painted_contours() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/🎯️picking/🧫️fixtures/🔷️shapes/🔣️.json")).unwrap();
    for sample in fixture.as_array().unwrap() {
        let kind=sample["kind"].as_str().unwrap();
        let mut shape=crate::DrawingShapeBody {base:crate::schema::default_layer_base("shape"),shape_kind:kind.into(),rect:None,ellipse:None,circle:None,line:None,polygon:None};
        shape.base.transform=crate::schema::geometry::affine::drawing_matrix_to_transform(serde_json::from_value(sample["matrix"].clone()).unwrap());
        let geometry=sample["geometry"].clone();
        match kind {
            "rect"=>shape.rect=Some(serde_json::from_value(geometry).unwrap()),
            "ellipse"=>shape.ellipse=Some(serde_json::from_value(geometry).unwrap()),
            "circle"=>shape.circle=Some(serde_json::from_value(geometry).unwrap()),
            "line"=>shape.line=Some(serde_json::from_value(geometry).unwrap()),
            "polygon"=>shape.polygon=Some(serde_json::from_value(geometry).unwrap()),
            _=>unreachable!(),
        }
        shape.base.attributes.fill=sample["fill"].as_bool().unwrap().then_some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});
        shape.base.attributes.stroke=None;
        let document=DrawingSnapshot {layers:vec![DrawingLayerNode::Shape(shape)],..Default::default()};
        let mut query=TracePointerJob::new_query(&document,serde_json::from_value(sample["point"].clone()).unwrap(),sample["radius"].as_f64().unwrap(),false);
        while !query.advance(&document) {}
        assert_eq!(query.best.is_some(),sample["expected"].as_bool().unwrap(),"{}",sample["name"]);
        assert!(!query.overflowed);
        eprintln!("[DEBUG] primitive pick {} completed in {} work units",sample["name"],query.completed_work);
    }
}

#[test]
fn compound_path_picking_uses_the_authored_fill_rule() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🌀️rule/🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let segments=crate::standards::v1::subsets::any::io::import::deserializers::artifacts::svg::v1_1::any::path::parse_editable_svg_path(row["path"].as_str().unwrap()).unwrap();
        let mut layer=create_drawing_path_layer("Compound",segments);
        let attributes=&mut crate::schema::layer_base_mut(&mut layer).attributes;
        attributes.fill=Some(crate::FillStyle::Solid {color:[0.0,0.0,0.0,1.0]});
        attributes.fill_rule=crate::FillRule::parse(row["rule"].as_str().unwrap()).unwrap();
        let document=DrawingSnapshot {layers:vec![layer],..Default::default()};
        let mut query=TracePointerJob::new_query(&document,[50.0,50.0],0.0,false);
        while !query.advance(&document) {}
        assert!(!query.overflowed);
        assert_eq!(query.best.is_some(),row["hit"].as_bool().unwrap(),"{}",row["name"]);
    }
    eprintln!("[DEBUG] compound path picking follows the same authored fill rule as scene paint");
}

#[test]
fn node_marquee_collects_snapshot_bound_anchors_in_bounded_steps() {
    let mut segments=vec![PathSegment::Move {to:[0.0,0.0]}];
    for index in 1..5000 {segments.push(PathSegment::Line {to:[index as f64,0.0]});}
    let geometry=points::geometry_id(&segments).unwrap();
    let mut path=create_drawing_path_layer("Path",segments);crate::schema::layer_base_mut(&mut path).id="path".into();
    let document=DrawingSnapshot {layers:vec![path],..Default::default()};
    let mut query=TracePointerJob::new_marquee(&document,[4.0,-1.0],[6.0,1.0],false);
    query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
    let mut steps=0;
    loop {let before=query.completed_work;let done=query.advance(&document);assert!(query.completed_work-before<=TRACE_POINTER_WORK_PER_STEP);steps+=1;if done {break;}}
    assert!(steps>1);assert!(!query.overflowed);
    assert_eq!(query.hits.iter().cloned().collect::<Vec<_>>(),(4..=6).map(|index|points::point_id("path",&geometry,index,PathPoint::Anchor).unwrap()).collect::<Vec<_>>());
    eprintln!("[DEBUG] 5000-anchor marquee hashes and selects incrementally in {steps} bounded slices");
}

#[test]
fn empty_node_press_starts_marquee_without_changing_point_selection() {
    let path=create_drawing_path_layer("Path",vec![PathSegment::Move {to:[0.0,0.0]},PathSegment::Line {to:[10.0,0.0]}]);
    let id=layer_id(&path).to_string();let document=DrawingSnapshot {layers:vec![path],..Default::default()};
    let mut session=DrawingSession::with_active_utility("editNodes");
    let mut cursor=TracePointerJob::new_query(&document,[-20.0,-20.0],1.0,false);cursor.node_editing=true;cursor.selected_ids=vec![id.clone()];while !cursor.advance(&document) {}
    let mut query=DrawingPointQuery::new("canvasPointerDown",cursor,false,"replace".into(),false);query.drag_start=Some([-20.0,-20.0]);session.point_query=Some(query);
    assert!(session.prepare_layer_move(&document,&[id],&[]).unwrap());assert!(session.node_marquee.is_some());assert!(session.gesture.matches("marqueeing"));assert!(session.point_query.as_ref().unwrap().node_selection.is_none());
    session.step_gesture(drawing_gesture::Event::Escape,&document,&NoConfig::default());assert!(session.node_marquee.is_none());assert!(session.gesture.matches("idle"));
}

#[test]
fn node_publication_uses_merged_point_targets_and_actual_json_byte_limit() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️point-publication/🔣️.json")).unwrap();
    let ids:Vec<String>=serde_json::from_value(fixture["ids"].clone()).unwrap();
    let document=DrawingSnapshot::default();
    let mut cursor=TracePointerJob::new_marquee(&document,[0.0,0.0],[1.0,1.0],false);
    cursor.hits.try_push("discarded-candidate".into()).unwrap();
    let mut query=DrawingPointQuery::new("canvasPointerUp",cursor,false,"replace".into(),true);
    query.node_selection=Some(ids.clone());query.preserve_selection=true;
    for _ in &ids {assert!(matches!(query.publication_step(),DrawingQueryPublication::Pending));}
    let DrawingQueryPublication::Complete(targets)=query.publication_step() else {panic!("Expected complete point publication")};
    assert_eq!(serde_json::from_str::<serde_json::Value>(&targets).unwrap(),fixture["targets"]);
    let mut query=DrawingPointQuery::new("canvasPointerUp",TracePointerJob::new_query(&document,[0.0,0.0],0.0,false),false,"replace".into(),false);
    let oversized=format!("{}:{}:0:anchor","\"".repeat(5000),"0".repeat(64));
    assert!(oversized.len()<DRAWING_QUERY_TARGET_BYTES);
    query.node_selection=Some(vec![oversized]);
    assert!(matches!(query.publication_step(),DrawingQueryPublication::Fault));
    eprintln!("[DEBUG] node publication emits point granularity after merging and refuses escaped output over its byte budget");
}

#[test]
fn node_marquee_rejects_overflow_and_excludes_hidden_or_locked_ancestors() {
    let segments=(0..=DRAWING_QUERY_HIT_CAPACITY).map(|index|if index==0 {PathSegment::Move {to:[0.0,0.0]}}else {PathSegment::Line {to:[index as f64,0.0]}}).collect();
    let mut path=create_drawing_path_layer("Many",segments);crate::schema::layer_base_mut(&mut path).id="path".into();
    let document=DrawingSnapshot {layers:vec![path],..Default::default()};
    let mut query=TracePointerJob::new_marquee(&document,[-1.0,-1.0],[1000.0,1.0],false);query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
    while !query.advance(&document) {}
    assert!(query.overflowed);assert!(query.hits.is_empty());
    for (visible,locked) in [(false,false),(true,true),(true,false)] {
        let mut path=create_drawing_path_layer("Child",vec![PathSegment::Move {to:[0.0,0.0]}]);crate::schema::layer_base_mut(&mut path).id="path".into();
        let mut group=crate::schema::create_drawing_group_layer("Group");
        if let DrawingLayerNode::Group(group)=&mut group {group.base.visible=visible;group.base.locked=locked;group.children.push(path);}
        let document=DrawingSnapshot {layers:vec![group],..Default::default()};
        let mut query=TracePointerJob::new_marquee(&document,[-1.0,-1.0],[1.0,1.0],false);query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
        while !query.advance(&document) {}
        assert!(!query.overflowed);assert_eq!(query.hits.len(),usize::from(visible&&!locked));
    }
}
