use super::*;

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
