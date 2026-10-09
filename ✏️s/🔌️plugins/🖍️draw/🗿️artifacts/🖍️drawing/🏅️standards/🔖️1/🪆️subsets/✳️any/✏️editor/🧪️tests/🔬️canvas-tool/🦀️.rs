//! 🧪️ The canvas tool — hit testing, lasso, drafts, drags and handle transforms as tool transactions — exercised as a child
//! module of the `canvas-pointer-down` command, whose projected bundle stays exactly its `🦀️.rs`.
use super::*;

#[test]
fn nested_curve_selection_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️selection/🔣️.json")).unwrap();
    let segments: Vec<PathSegment> = serde_json::from_value(fixture["segments"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut path = filled_path("Curve", segments.clone());
        crate::schema::layer_base_mut(&mut path).id = "curve".into();
        crate::schema::layer_base_mut(&mut path).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});
        let mut group = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Parent")).to_string().into()).expect("nonempty authored identity"), "Parent");
        let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
        let [x,y,scale_x,scale_y,rotation]: [f64;5] = serde_json::from_value(case["parent"].clone()).unwrap();
        body.base.transform = crate::DrawingTransform { x,y,scale_x,scale_y,rotation, shear: 0.0 };
        body.base.visible = case["visible"].as_bool().unwrap_or(true);
        body.base.locked = case["locked"].as_bool().unwrap_or(false);
        body.children.push(path);
        let document = DrawingSnapshot { layers: vec![group].into(), ..Default::default() };
        let point = serde_json::from_value(case["point"].clone()).unwrap();
        let mut query = TracePointerJob::new_query(&document, point, 0.0, case["includeControls"].as_bool().unwrap_or(false));
        finish_cached(&mut query,&document);
        assert_eq!(query.best.as_ref().map(|hit| hit.layer_id.as_str()), if case["selected"] == true { Some("curve") } else { None }, "{}", case["name"]);
    }
}

#[test]
fn point_selection_keeps_the_frontmost_equal_candidate() {
    let mut back = stroked_path("Back", vec![PathSegment::Move { to: [0.0,0.0] },PathSegment::Line { to: [10.0,10.0] }]);
    let mut front = back.clone();
    crate::schema::layer_base_mut(&mut back).id = "back".into();
    crate::schema::layer_base_mut(&mut front).id = "front".into();
    let document = DrawingSnapshot { layers: vec![back,front].into(), ..Default::default() };
    let mut query = TracePointerJob::new_query(&document,[5.0,5.0],0.0,false);
    finish_cached(&mut query,&document);
    assert_eq!(query.best.unwrap().layer_id,"front");
}

#[test]
fn lasso_samples_yield_and_select_the_polygon_instead_of_its_rectangle() {
    use crate::editor::drawing::commands::canvas_pointer_move::CanvasPointerMove;
    let inside=stroked_path("Inside",vec![PathSegment::Move { to: [1.0,1.0] },PathSegment::Line { to: [2.0,2.0] }]);
    let outside=stroked_path("Outside",vec![PathSegment::Move { to: [6.0,6.0] },PathSegment::Line { to: [7.0,7.0] }]);
    let expected=layer_id(&inside).to_string();
    let document=DrawingSnapshot { layers: vec![inside,outside].into(),..Default::default() };
    let mut session=crate::editor::drawing::identity_test::session("selectLasso","");
    session.window_config.viewport=store::Viewport2d { x:0.0,y:0.0,zoom:1.0 };
    session.press(pointer("selectLasso",[0.0,0.0])).unwrap();
    let movement=CanvasPointerMove { shift: false, alt: false,  x:50.0,y:60.0,width:100.0,height:100.0,samples:vec![[60.0,50.0],[50.0,60.0]] };
    assert!(session.advance_lasso_move(&movement).unwrap().is_none());
    assert_eq!(session.tool.context().points.len(),2);
    assert!(session.advance_lasso_move(&movement).unwrap().is_some());
    assert_eq!(session.tool.context().points.iter().copied().collect::<Vec<_>>(),vec![[0.0,0.0],[10.0,0.0],[0.0,10.0]]);
    let polygon=LassoPolygon::from_points(&session.tool.context().points,[0.0,0.0]);
    let mut query=TracePointerJob::new_lasso(&document,polygon);
    finish_cached(&mut query,&document);
    assert_eq!(query.hits.iter().cloned().collect::<Vec<_>>(),vec![expected]);
    session.escape().unwrap();
    assert_eq!(session.preview().phase,DrawingGesturePreviewPhase::Idle);
}

/// 🖱️ A press of `utility` at `world` without modifiers.
fn pointer(utility: &str, world: [f64;2]) -> DrawingPointer {
    DrawingPointer { utility: utility.into(), world, shift: false, alt: false, ctrl: false, meta: false }
}

/// 🧱️ The committed base a test release yields against.
fn base(document: &DrawingSnapshot) -> DrawingToolBase {
    DrawingToolBase { document: Arc::new(document.clone()), operation: None }
}

#[test]
fn long_lasso_decimates_within_fixed_capacity() {
    let mut context=DrawingToolContext::default();
    for index in 0..10_000 { append_lasso_point(&mut context,[index as f64,(index as f64/30.0).sin()*20.0],0.1); }
    assert!(!context.points_overflowed);
    assert!(context.points.len()<=DRAWING_GESTURE_PREVIEW_POINT_CAPACITY);
    assert_eq!(context.points[0],[0.0,0.0]);
    let polygon=LassoPolygon::from_points(&context.points,[10_000.0,0.0]);
    assert_eq!(polygon.as_slice().last(),Some(&[10_000.0,0.0]));
}

#[test]
fn shape_identity_is_replay_stable_and_scoped_to_the_durable_app_operation() {
    let operation = AppOperationContext {
        app_instance_id: 7,
        parent_document_id: "drawing-document".into(),
        operation_id: 11,
        generation: 13,
        canonical_base_revision: [17; 32],
        authoring_seed: "authoring-seed-test".into(),
    };
    let geometry = [10.0, 20.0, 30.0, 40.0];
    let first = admitted_shape_key("shapeRect", geometry, 3, Some(&operation));
    assert_eq!(first, admitted_shape_key("shapeRect", geometry, 3, Some(&operation)), "replaying the same admitted operation is deterministic");
    assert_ne!(first, admitted_shape_key("shapeRect", geometry, 3, Some(&AppOperationContext { app_instance_id: 8, ..operation.clone() })), "a different live app instance cannot collide at the same document revision and local operation ordinal");
    assert_ne!(first, admitted_shape_key("shapeRect", geometry, 4, Some(&operation)), "a second same-geometry creation in the same document uses its document-local ordinal");
}

#[semio_framework_async_macros::async_test]
async fn trace_pointer_step_consumes_at_most_the_fixed_work_budget() {
    let mut document = crate::standards::v1::subsets::any::schema::default_drawing_document("bounded-trace", None);
    let mut segments = vec![PathSegment::Move { to: [0.0, 0.0] }];
    segments.extend((0..256).map(|index| PathSegment::Line { to: [index as f64, (index%2)as f64*8.0] }));
    document.layers = vec![filled_path("long-path", segments)].into();
    let mut job = TracePointerJob::new(7, &document, [4.0, 4.0]);

    assert!(!advance_cached(&mut job,&document));
    assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(!job.work.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn continuation_work_helpers_have_synchronous_compile_shape() {
    let _: fn(&mut TracePointerJob, &DrawingSnapshot, &MountedSceneQuery) -> bool = TracePointerJob::advance;
    let _: fn(Result<ToolStep<DrawingMutation>, ToolRefusal>) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> = drawing_tool_emit;
}

#[test]
fn stale_generation_and_wrong_owner_cannot_take_retained_trace() {
    let document = crate::standards::v1::subsets::any::schema::default_drawing_document("fresh-trace", None);
    let mut session = DrawingSession::default();
    assert!(session.retain_trace_pointer(TracePointerJob::new(11, &document, [0.0, 0.0])).is_ok());
    let base = format!("unbound:{}", document.id);
    assert!(session.take_trace_pointer(0, &document.id, 0, 12, &base).is_none());
    assert!(session.take_trace_pointer(1, &document.id, 0, 11, &base).is_none());
    assert!(session.take_trace_pointer(0, "foreign", 0, 11, &base).is_none());
    assert!(session.take_trace_pointer(0, &document.id, 0, 11, &base).is_some());
}

#[test]
fn wide_admitted_scene_roots_and_groups_keep_pointer_work_bounded() {
    let leaf = filled_path("leaf", vec![PathSegment::Move { to: [0.0, 0.0] }]);
    let mut roots = crate::standards::v1::subsets::any::schema::default_drawing_document("wide-roots", None);
    roots.layers=(0..1024).map(|i|{let mut layer=leaf.clone();crate::schema::layer_base_mut(&mut layer).id=format!("root-{i}").into();layer}).collect();
    let mut roots_job = TracePointerJob::new(8, &roots, [0.0, 0.0]);
    advance_cached(&mut roots_job,&roots);
    assert_eq!(roots_job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(roots_job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);

    let mut group = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("wide")).to_string().into()).expect("nonempty authored identity"), "wide");
    let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
    body.children=(0..1023).map(|i|{let mut layer=leaf.clone();crate::schema::layer_base_mut(&mut layer).id=format!("child-{i}").into();layer}).collect();
    let mut document = crate::standards::v1::subsets::any::schema::default_drawing_document("wide-group", None);
    document.layers = vec![group].into();
    let mut job = TracePointerJob::new(9, &document, [0.0, 0.0]);
    advance_cached(&mut job,&document);
    assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
    assert!(job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);
}

#[test]
fn retained_trace_interruption_cancel_and_repeated_cancel_are_exact() {
    let document = crate::standards::v1::subsets::any::schema::default_drawing_document("retained", None);
    let mut session = DrawingSession::default();
    assert!(session.retain_trace_pointer(TracePointerJob::new(91, &document, [4.0, 5.0])).is_ok());
    assert!(!session.cancel_trace_pointer(0, &document.id, 90));
    assert!(session.cancel_trace_pointer(0, &document.id, 91));
    assert!(!session.cancel_trace_pointer(0, &document.id, 91));
}

#[test]
fn marquee_maximum_plus_one_faults_without_unbounded_growth() {
    let mut document = crate::standards::v1::subsets::any::schema::default_drawing_document("marquee-max", None);
    document.layers = (0..=DRAWING_QUERY_HIT_CAPACITY).map(|index| stroked_path(&format!("hit-{index}"), vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }])).collect();
    let mut query = TracePointerJob::new_marquee(&document, [-128.0, -128.0], [128.0, 128.0], true);
    let mut turns = 0;
    finish_cached_with(&mut query,&document,|_,done|{if !done{turns+=1;assert!(turns<10_000);}});
    assert!(turns > 1, "the adversarial tree must yield across turns");
    assert!(query.overflowed, "maximum plus one must fault rather than publish a partial selection");
    assert!(query.hits.is_empty(),"failed cache query never exposes partial selection");
}

#[test]
fn a_pen_draft_commits_one_path_layer_as_one_transaction() {
    let document = crate::standards::v1::subsets::any::schema::default_drawing_document("draft", None);
    let mut session = crate::editor::drawing::identity_test::session("pen", "draft-seed");
    for point in [[0.0,0.0],[10.0,0.0],[10.0,10.0]] { assert!(session.press(pointer("pen",point)).unwrap().artifact_mutations.is_empty()); }
    assert!(session.tool.matches("drafting"));
    let emit = session.finish_draft(base(&document)).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 1, "one draft is one create-layer");
    let transaction = emit.transaction.as_ref().expect("the draft commits as one tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.draw.drawing@1/*#editor#pen", "{transaction:?}");
    let DrawingMutation::CreateLayer(created) = &emit.artifact_mutations[0] else { panic!("creation") };
    let DrawingLayerNode::Path(path) = &*created.layer else { panic!("a pen draft is a path") };
    assert_eq!(path.segments, vec![PathSegment::Move { to: [0.0,0.0] }, PathSegment::Line { to: [10.0,0.0] }, PathSegment::Line { to: [10.0,10.0] }].into());
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })), "a committed creation returns to the default utility");
    assert!(session.tool.at_rest());
    let mut single = crate::editor::drawing::identity_test::session("pen", "");
    single.press(pointer("pen",[0.0,0.0])).unwrap();
    assert!(single.finish_draft(base(&document)).unwrap().artifact_mutations.is_empty(), "one point commits nothing");
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_tool_inputs_settle_in_one_microstep() {
    let mut sink = Vec::new();
    let mut snapshot = machine::init::<canvas_tool::CanvasTool>((), &mut sink);
    for utility in ["selectMarquee", "shapeRect", "pen", "shapePolygon", "trace"] {
        let mut sink = Vec::new();
        let report = machine::macrostep(&mut snapshot, canvas_tool::Event::PointerDown(pointer(utility, [1.0, 2.0])), &mut sink, &mut machine::NullInspector);
        assert!(report.microsteps <= 1, "{utility} used {} microsteps", report.microsteps);
    }
}

#[test]
fn direct_drag_previews_then_commits_one_parametric_drag_transaction() {
    let path = stroked_path("Moving",vec![PathSegment::Move { to:[0.0,0.0] },PathSegment::Line { to:[10.0,10.0] }]);
    let id = layer_id(&path).to_string();
    let mut group = crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Parent")).to_string().into()).expect("nonempty authored identity"), "Parent");
    let DrawingLayerNode::Group(body) = &mut group else { unreachable!() };
    body.base.transform = crate::DrawingTransform { x:100.0,y:200.0,scale_x:2.0,scale_y:4.0,rotation:std::f64::consts::FRAC_PI_2, shear: 0.0 };
    body.children.push(path);
    let document = DrawingSnapshot { layers:vec![group].into(),..Default::default() };
    let before = document.clone();
    let start = [80.0,210.0];
    let end = [88.0,220.0];
    let mut session = crate::editor::drawing::identity_test::session("selectDirect","drag-seed");
    session.window_config.viewport.zoom = 1.0;
    prepare_drag(&mut session,&document,start);
    assert!(session.tool.matches("dragging"));
    assert!(session.sample(end,false,false).unwrap().artifact_mutations.is_empty(), "a drag tick never publishes");
    assert_eq!(session.preview().transformation,Some((vec![id.clone()],[1.0,0.0,0.0,1.0,8.0,10.0])));
    assert_eq!(session.tool.provisional(), Some(&drag_layers(std::iter::once(id.as_str().into()).collect(),8.0,10.0)), "the open transaction holds the net parametric leaf");
    assert_eq!(document,before);
    let emit = session.release("canvasPointerUp",pointer("selectDirect",end),base(&document)).unwrap().unwrap();
    assert_eq!(emit.artifact_mutations,vec![drag_layers(std::iter::once(id.as_str().into()).collect(),8.0,10.0)]);
    assert!(emit.transaction.as_ref().is_some_and(|transaction|transaction.tool=="s.draw.drawing@1/*#editor#selectDirect"));
    assert!(session.preview().transformation.is_none());
    assert!(session.tool.at_rest());
    let mut after = document.clone();
    crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&emit.artifact_mutations[0]).unwrap();
    let transform = &crate::schema::layer_base(crate::schema::find_drawing_layer(&after,&id).unwrap()).transform;
    assert!((transform.x-5.0).abs()<1e-10);
    assert!((transform.y+2.0).abs()<1e-10);
}

#[test]
fn cancelled_direct_drag_and_subthreshold_click_do_not_mutate() {
    let mut layer = crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Moving")).to_string().into()).expect("nonempty authored identity"), "Moving");
    crate::schema::layer_base_mut(&mut layer).attributes.fill=Some(crate::FillStyle::Solid{color:[0.0,0.0,0.0,1.0]});
    let document = DrawingSnapshot { layers:vec![layer].into(),..Default::default() };
    for cancelled in [true,false] {
        let mut session = crate::editor::drawing::identity_test::session("selectDirect","");
        session.window_config.viewport.zoom = 1.0;
        prepare_drag(&mut session,&document,[0.0,0.0]);
        let emit = if cancelled {
            session.sample([30.0,20.0],false,false).unwrap();
            assert!(session.tool.provisional().is_some(), "the drag is open before the cancel");
            session.cancel()
        } else { session.release("canvasPointerUp",pointer("selectDirect",[1.0,1.0]),base(&document)).unwrap().unwrap() };
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "zero trace");
        assert!(session.preview().transformation.is_none());
        assert!(session.tool.at_rest());
        assert!(session.tool.provisional().is_none());
    }
}

#[test]
fn selection_grab_preparation_yields_and_rejects_locked_or_missing_targets_atomically() {
    for invalid in ["locked","hidden","missing","singular"] {
        let mut first=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("First")).to_string().into()).expect("nonempty authored identity"), "First");
        crate::schema::layer_base_mut(&mut first).id="first".into();
        let mut second=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Second")).to_string().into()).expect("nonempty authored identity"), "Second");
        crate::schema::layer_base_mut(&mut second).id="second".into();
        let mut parent=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Parent")).to_string().into()).expect("nonempty authored identity"), "Parent");
        let base=crate::schema::layer_base_mut(&mut parent);
        base.locked=invalid=="locked";
        base.visible=invalid!="hidden";
        base.transform.scale_x=if invalid=="singular" {0.0} else {2.0};
        if let DrawingLayerNode::Group(group)=&mut parent { group.children.push(second); }
        let document=DrawingSnapshot { layers:vec![first,parent].into(),..Default::default() };
        let mut preparation=LayerGrabPreparation::new(&document,vec!["first".into(),if invalid=="missing" {"gone".into()} else {"second".into()}],None);
        let before=document.clone();
        assert_eq!(preparation.advance(&document).unwrap(),false);
        assert_eq!(preparation.targets.len(),1);
        let mut error=false;
        for _ in 0..8 { match preparation.advance(&document) { Err(_)=>{error=true;break;},Ok(true)=>break,Ok(false)=>{} } }
        assert!(error,"{invalid}");
        assert_eq!(document,before);
    }
}

#[test]
fn selection_drag_leaf_moves_equal_world_displacements_under_distinct_parents() {
    let mut a=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A");
    crate::schema::layer_base_mut(&mut a).id="a".into();
    let mut b=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B");
    crate::schema::layer_base_mut(&mut b).id="b".into();
    let mut parent=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Parent")).to_string().into()).expect("nonempty authored identity"), "Parent");
    let base=crate::schema::layer_base_mut(&mut parent);
    base.transform.rotation=std::f64::consts::FRAC_PI_2;
    base.transform.scale_x=2.0;
    if let DrawingLayerNode::Group(group)=&mut parent { group.children.push(b); }
    let mut document=DrawingSnapshot { layers:vec![a,parent].into(),..Default::default() };
    let before=crate::schema::flatten_drawing_document_to_scene_nodes(&document);
    let mut preparation=LayerGrabPreparation::new(&document,vec!["a".into(),"b".into()],None);
    let mut complete=false;
    for _ in 0..8 { if preparation.advance(&document).unwrap() {complete=true;break;} }
    assert!(complete);
    let DrawingGrab::Layers { targets, handle: None }=preparation.grab() else { panic!("a layer grab") };
    assert_eq!(targets.iter().map(|id|id.to_string_owner()).collect::<Vec<_>>(),vec!["a".to_string(),"b".to_string()]);
    let leaf=drawing_grab_leaf(&DrawingGrab::Layers { targets, handle: None },[0.0,0.0],[12.0,8.0],false,false).unwrap();
    crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut document,&leaf).unwrap();
    let after=crate::schema::flatten_drawing_document_to_scene_nodes(&document);
    for (before,after) in before.iter().zip(after) {
        assert!((after.transform[4]-before.transform[4]-12.0).abs()<1e-10);
        assert!((after.transform[5]-before.transform[5]-8.0).abs()<1e-10);
    }
}

fn prepare_drag(session: &mut DrawingSession, document: &DrawingSnapshot, start: [f64;2]) {
    session.press(pointer(&session.active_utility_id.clone(),start)).unwrap();
    let mut query=DrawingPointQuery::new("canvasPointerDown",TracePointerJob::new_query(document,start,0.0,false),false,"replace".into(),false);
    finish_cached(&mut query.cursor,document);
    query.drag_start=Some(start);
    session.point_query=Some(query);
    for _ in 0..1000 {
        if session.prepare_grab(document,&[],&[]).unwrap() { session.point_query=None; return; }
    }
    panic!("Grab preparation did not complete");
}

#[test]
fn repeated_pen_and_polygon_drafts_keep_distinct_layers_and_transactions() {
    for utility in ["pen","shapePolygon"] {
        let mut document=DrawingSnapshot::default();
        let (mut ids,mut transactions)=(std::collections::HashSet::new(),std::collections::HashSet::new());
        let mut session=crate::editor::drawing::identity_test::session(utility,"draft-seed");
        for round in 0..3 {
            for point in [[0.0,0.0],[10.0,0.0],[10.0,10.0]] { session.press(pointer(utility,point)).unwrap(); }
            let emit=session.finish_draft(DrawingToolBase { document: Arc::new(document.clone()), operation: Some(AppOperationContext { app_instance_id: 1, parent_document_id: "draft".into(), operation_id: round+1, generation: 1, canonical_base_revision: [round as u8; 32], authoring_seed: "draft-seed".into() }) }).unwrap();
            let DrawingMutation::CreateLayer(created)=&emit.artifact_mutations[0] else {panic!("Expected creation")};
            assert!(ids.insert(layer_id(&created.layer).to_string()),"{utility} reused a completed draft id");
            assert!(transactions.insert(emit.transaction.clone().expect("one transaction per draft").id),"{utility} reused a transaction id");
            for mutation in emit.artifact_mutations {crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut document,&mutation).unwrap();}
        }
        assert_eq!(document.layers.len(),3);
    }
}


#[test]
fn selected_handle_previews_are_ephemeral_and_release_one_parametric_leaf() {
    let layer=filled_path("Box",vec![PathSegment::Move {to:[10.0,20.0]},PathSegment::Line {to:[110.0,20.0]},PathSegment::Line {to:[110.0,100.0]},PathSegment::Line {to:[10.0,100.0]},PathSegment::Close]);
    let id=layer_id(&layer).to_string();
    let document=DrawingSnapshot {layers:vec![layer].into(),..Default::default()};
    let before=document.clone();
    for (start,end,kind) in [([110.0,100.0],[210.0,180.0],"scale-layers"),([60.0,-8.0],[128.0,60.0],"rotate-layers")] {
        for cancel in [false,true] {
            let mut cursor=TracePointerJob::new_query(&document,start,0.0,false);
            cursor.retain_selection_bounds(&[id.clone()]).unwrap();
            finish_cached(&mut cursor,&document);
            assert_eq!(cursor.selection_bounds,Some([10.0,20.0,100.0,80.0]));
            let mut query=DrawingPointQuery::new("canvasPointerDown",cursor,false,"replace".into(),false);
            query.drag_start=Some(start);
            let mut session=crate::editor::drawing::identity_test::session("selectDirect","");
            session.window_config.viewport.zoom=1.0;
            session.press(pointer("selectDirect",start)).unwrap();
            session.point_query=Some(query);
            while !session.prepare_grab(&document,&[id.clone()],&[]).unwrap() {}
            assert!(matches!(session.tool.context().grab,Some(DrawingGrab::Layers { handle: Some(_), .. })));
            session.sample(end,false,false).unwrap();
            let preview=session.preview().transformation.unwrap();
            let scene=crate::schema::flatten_drawing_document_with_transformation(&document,Some(&preview));
            assert_eq!(scene[0].transform,preview.1);
            assert_eq!(document,before);
            let emit=if cancel {session.cancel()} else {session.release("canvasPointerUp",pointer("selectDirect",end),base(&document)).unwrap().unwrap()};
            assert!(session.preview().transformation.is_none());
            assert!(session.tool.at_rest());
            if cancel {assert!(emit.artifact_mutations.is_empty());} else {
                assert_eq!(emit.artifact_mutations.len(),1);
                assert!(match kind {"scale-layers"=>matches!(emit.artifact_mutations[0],DrawingMutation::ScaleLayers(_)),_=>matches!(emit.artifact_mutations[0],DrawingMutation::RotateLayers(_))},"{kind}");
                let mut after=document.clone();
                crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&emit.artifact_mutations[0]).unwrap();
                let actual=crate::schema::drawing_transform_to_matrix(&crate::schema::layer_base(&after.layers[0]).transform);
                for index in 0..6 {assert!((actual[index]-preview.1[index]).abs()<1e-10);}
            }
        }
    }
}

#[test]
fn point_selection_reaches_the_painted_path_behind_a_concave_frame() {
    let points=[[0.0,0.0],[100.0,0.0],[100.0,100.0],[80.0,100.0],[80.0,20.0],[0.0,20.0]];
    let mut segments=points.into_iter().enumerate().map(|(i,to)|if i==0{PathSegment::Move {to}}else{PathSegment::Line {to}}).collect::<Vec<_>>();
    segments.push(PathSegment::Close);
    let mut frame=filled_path("Red Frame",segments);
    let mut back=filled_path("Orange",vec![PathSegment::Move {to:[10.0,30.0]},PathSegment::Line {to:[40.0,30.0]},PathSegment::Line {to:[40.0,80.0]},PathSegment::Line {to:[10.0,80.0]},PathSegment::Close]);
    for layer in [&mut frame,&mut back]{crate::schema::layer_base_mut(layer).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,1.0]});}
    let expected=layer_id(&back).to_string();
    let document=DrawingSnapshot {layers:vec![back,frame].into(),..Default::default()};
    let mut query=TracePointerJob::new_query(&document,[20.0,50.0],1.0,false);
    finish_cached(&mut query,&document);
    assert_eq!(query.best.unwrap().layer_id,expected);
}

#[test]
fn primitive_picking_uses_painted_contours() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/🧮️geometry/🎯️picking/🧫️fixtures/🔷️shapes/🔣️.json")).unwrap();
    for sample in fixture.as_array().unwrap() {
        let kind=sample["kind"].as_str().unwrap();
        let mut shape=crate::DrawingShapeBody {base:crate::schema::default_layer_base(crate::schema::identity::DrawingIdentity::admit((("shape")).to_string().into()).expect("nonempty authored identity"), "shape"),shape_kind:kind.into(),rect:None,ellipse:None,circle:None,line:None,polygon:None};
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
        shape.base.attributes.stroke=if kind=="line"{Some(crate::StrokeStyle{color:[0.0,0.0,0.0,1.0],width:sample["radius"].as_f64().unwrap()*2.0,cap:crate::StrokeCap::Butt,join:crate::StrokeJoin::Miter,dash:None})}else{None};
        let document=DrawingSnapshot {layers:vec![DrawingLayerNode::Shape(shape)].into(),..Default::default()};
        let mut query=TracePointerJob::new_query(&document,serde_json::from_value(sample["point"].clone()).unwrap(),sample["radius"].as_f64().unwrap(),false);
        finish_cached(&mut query,&document);
        assert_eq!(query.best.is_some(),sample["expected"].as_bool().unwrap(),"{}",sample["name"]);
        assert!(!query.overflowed);
    }
}

#[test]
fn compound_path_picking_uses_the_authored_fill_rule() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/🎨️fill/🌀️rule/🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let segments=crate::standards::v1::subsets::any::io::import::deserializers::artifacts::svg::v1_1::any::path::parse_editable_svg_path(row["path"].as_str().unwrap()).unwrap();
        let mut layer=filled_path("Compound",segments);
        let attributes=&mut crate::schema::layer_base_mut(&mut layer).attributes;
        attributes.fill=Some(crate::FillStyle::Solid {color:[0.0,0.0,0.0,1.0]});
        attributes.fill_rule=crate::FillRule::parse(row["rule"].as_str().unwrap()).unwrap();
        let document=DrawingSnapshot {layers:vec![layer].into(),..Default::default()};
        let mut query=TracePointerJob::new_query(&document,[50.0,50.0],0.0,false);
        finish_cached(&mut query,&document);
        assert!(!query.overflowed);
        assert_eq!(query.best.is_some(),row["hit"].as_bool().unwrap(),"{}",row["name"]);
    }
}

#[test]
fn node_marquee_collects_snapshot_bound_anchors_in_bounded_steps() {
    let mut segments=vec![PathSegment::Move {to:[0.0,0.0]}];
    for index in 1..5000 {segments.push(PathSegment::Line {to:[index as f64,0.0]});}
    let geometry=points::geometry_id(&segments).unwrap();
    let mut path=filled_path("Path",segments);crate::schema::layer_base_mut(&mut path).id="path".into();
    let document=DrawingSnapshot {layers:vec![path].into(),..Default::default()};
    let mut query=TracePointerJob::new_marquee(&document,[4.0,-1.0],[6.0,1.0],false);
    query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
    let mut steps=0;
    let mut work=query.completed_work;finish_cached_with(&mut query,&document,|query,_|{assert!(query.completed_work-work<=TRACE_POINTER_WORK_PER_STEP);work=query.completed_work;steps+=1;});
    assert!(steps>1);assert!(!query.overflowed);
    assert_eq!(query.hits.iter().cloned().collect::<Vec<_>>(),(4..=6).map(|index|points::point_id("path",&geometry,index,PathPoint::Anchor).unwrap()).collect::<Vec<_>>());
}

#[test]
fn empty_node_press_starts_marquee_without_changing_point_selection() {
    let path=filled_path("Path",vec![PathSegment::Move {to:[0.0,0.0]},PathSegment::Line {to:[10.0,0.0]}]);
    let id=layer_id(&path).to_string();let document=DrawingSnapshot {layers:vec![path].into(),..Default::default()};
    let mut session=crate::editor::drawing::identity_test::session("editNodes","");
    session.press(pointer("editNodes",[-20.0,-20.0])).unwrap();
    let mut cursor=TracePointerJob::new_query(&document,[-20.0,-20.0],1.0,false);cursor.node_editing=true;cursor.selected_ids=vec![id.clone()];finish_cached(&mut cursor,&document);
    let mut query=DrawingPointQuery::new("canvasPointerDown",cursor,false,"replace".into(),false);query.drag_start=Some([-20.0,-20.0]);session.point_query=Some(query);
    assert!(session.prepare_grab(&document,&[id],&[]).unwrap());assert!(session.node_marquee.is_some());assert!(session.tool.matches("marqueeing"));assert!(session.point_query.as_ref().unwrap().node_selection.is_none());
    session.escape().unwrap();assert!(session.node_marquee.is_none());assert!(session.tool.at_rest());
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
}

#[test]
fn node_marquee_rejects_overflow_and_excludes_hidden_or_locked_ancestors() {
    let segments=(0..=DRAWING_QUERY_HIT_CAPACITY).map(|index|if index==0 {PathSegment::Move {to:[0.0,0.0]}}else {PathSegment::Line {to:[index as f64,0.0]}}).collect();
    let mut path=filled_path("Many",segments);crate::schema::layer_base_mut(&mut path).id="path".into();
    let document=DrawingSnapshot {layers:vec![path].into(),..Default::default()};
    let mut query=TracePointerJob::new_marquee(&document,[-1.0,-1.0],[1000.0,1.0],false);query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
    finish_cached(&mut query,&document);
    assert!(query.overflowed);assert!(query.hits.is_empty());
    for (visible,locked) in [(false,false),(true,true),(true,false)] {
        let mut path=filled_path("Child",vec![PathSegment::Move {to:[0.0,0.0]}]);crate::schema::layer_base_mut(&mut path).id="path".into();
        let mut group=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
        if let DrawingLayerNode::Group(group)=&mut group {group.base.visible=visible;group.base.locked=locked;group.children.push(path);}
        let document=DrawingSnapshot {layers:vec![group].into(),..Default::default()};
        let mut query=TracePointerJob::new_marquee(&document,[-1.0,-1.0],[1.0,1.0],false);query.node_area=true;query.node_editing=true;query.selected_ids=vec!["path".into()];
        finish_cached(&mut query,&document);
        assert!(!query.overflowed);assert_eq!(query.hits.len(),usize::from(visible&&!locked));
    }
}

fn stroked_path(name:&str,segments:Vec<PathSegment>)->DrawingLayerNode{let mut layer=crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit(((name)).to_string().into()).expect("nonempty authored identity"), name,segments.into());crate::schema::layer_base_mut(&mut layer).attributes.stroke=Some(crate::StrokeStyle{color:[0.0,0.0,0.0,1.0],width:1.0,cap:crate::StrokeCap::Butt,join:crate::StrokeJoin::Miter,dash:None});layer}
fn filled_path(name:&str,segments:Vec<PathSegment>)->DrawingLayerNode{let mut layer=crate::standards::v1::subsets::any::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit(((name)).to_string().into()).expect("nonempty authored identity"), name,segments.into());crate::schema::layer_base_mut(&mut layer).attributes.fill=Some(crate::FillStyle::Solid{color:[0.0,0.0,0.0,1.0]});layer}
fn prepared(document:&DrawingSnapshot)->crate::schema::scene_paint::scene::PreparedScene{
 let mut vector=crate::schema::scene_preparation::DocumentVectorJob::new(document,crate::editor::drawing::geometry_session::limits(),crate::editor::drawing::geometry_session::algorithms()).unwrap();while !vector.advance(4096).unwrap().done{}let(mut close,plan)=vector.into_retirement();while !close.advance(4096).unwrap().done{}
 let mut paint=crate::schema::scene_paint::scene::ScenePaintJob::new(plan.unwrap(),0.001,crate::editor::drawing::geometry_session::paint_limits());while !paint.advance(4096).unwrap().done{}let(mut close,scene)=paint.into_retirement();while !close.advance(4096).unwrap().done{}scene.unwrap()
}
fn advance_cached(job:&mut TracePointerJob,document:&DrawingSnapshot)->bool{
 let scene=prepared(document);let borrowed=MountedSceneQuery{scene:&scene,source:crate::schema::scene_identity::SceneIdentity{instance:710034,base:11,generation:1,revision:[1;32]},build:1};let done=job.advance(document,&borrowed);
 let mut close=crate::schema::scene_paint::scene::PreparedSceneCloseJob::new(scene);while !close.advance(4096).unwrap().done{}done
}

/// 🗂️ Query continuations borrow one genuinely prepared immutable scene until its owner closes.
fn finish_cached(job:&mut TracePointerJob,document:&DrawingSnapshot){finish_cached_with(job,document,|_,_|{});}
fn finish_cached_with(job:&mut TracePointerJob,document:&DrawingSnapshot,mut observe:impl FnMut(&TracePointerJob,bool)){
 let scene=prepared(document);let borrowed=MountedSceneQuery{scene:&scene,source:crate::schema::scene_identity::SceneIdentity{instance:710034,base:11,generation:1,revision:[1;32]},build:1};loop{let done=job.advance(document,&borrowed);observe(job,done);if done{break;}}let mut close=crate::schema::scene_paint::scene::PreparedSceneCloseJob::new(scene);while !close.advance(4096).unwrap().done{}assert!(close.terminal_is_empty());
}

fn admitted_shape_key(utility:&str,geometry:[f64;4],ordinal:usize,operation:Option<&AppOperationContext>)->String{let mut observer=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut observer);crate::standards::v1::subsets::any::io::text::identity::creation::shape_identity(utility,geometry,ordinal,operation.expect("explicit fixture operation"),&mut control).unwrap().into_key().to_string_owner()}
