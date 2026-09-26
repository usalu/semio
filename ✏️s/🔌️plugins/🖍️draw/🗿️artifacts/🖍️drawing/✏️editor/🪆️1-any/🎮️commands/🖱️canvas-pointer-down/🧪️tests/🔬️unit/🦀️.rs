use super::*;

#[test]
fn nested_curve_selection_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️selection/🔣️.json")).unwrap();
    let segments: Vec<PathSegment> = serde_json::from_value(fixture["segments"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut path = create_drawing_path_layer("Curve", segments.clone());
        crate::schema::layer_base_mut(&mut path).id = "curve".into();
        let mut group = crate::schema::create_drawing_group_layer("Parent");
        let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
        let [x,y,scale_x,scale_y,rotation]: [f64;5] = serde_json::from_value(case["parent"].clone()).unwrap();
        body.base.transform = crate::DrawingTransform { x,y,scale_x,scale_y,rotation };
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
    let movement=CanvasPointerMove { x:50.0,y:60.0,width:100.0,height:100.0,samples:vec![[60.0,50.0],[50.0,60.0]] };
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
    body.base.transform = crate::DrawingTransform { x:100.0,y:200.0,scale_x:2.0,scale_y:4.0,rotation:std::f64::consts::FRAC_PI_2 };
    body.children.push(path);
    let document = DrawingSnapshot { layers:vec![group],..Default::default() };
    let before = document.clone();
    let start = [80.0,210.0];
    let end = [88.0,220.0];
    let mut query = TracePointerJob::new_query(&document,start,0.0,false);
    while !query.advance(&document) {}
    let mut session = DrawingSession::with_active_utility("selectDirect");
    session.step_gesture(drawing_gesture::Event::PointerDown { utility:"selectDirect".into(),world:start,shift:false,ctrl:false,meta:false },&document,&NoConfig::default());
    session.prepare_layer_move(&document,query.best.as_ref(),start);
    session.move_layer_preview(end);
    assert_eq!(session.preview().translation,Some((id.clone(),[8.0,10.0])));
    assert_eq!(document,before);
    let emit = session.finish_layer_move(end,&document,&NoConfig::default()).unwrap();
    assert_eq!(emit.artifact_mutations.len(),1);
    assert_eq!(emit.description.as_deref(),Some("Move layer"));
    assert!(session.preview().translation.is_none());
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
        session.prepare_layer_move(&document,query.best.as_ref(),[0.0,0.0]);
        let emit = if cancelled {
            session.move_layer_preview([30.0,20.0]);
            crate::editor::drawing::commands::canvas_pointer_up::cancel_gesture(&mut session,&document,&NoConfig::default())
        } else { session.finish_layer_move([1.0,1.0],&document,&NoConfig::default()).unwrap() };
        assert!(emit.artifact_mutations.is_empty());
        assert!(session.preview().translation.is_none());
        assert!(session.gesture.matches("idle"));
    }
    eprintln!("[DEBUG] direct drag cancellation and clicks leave the document unchanged");
}
