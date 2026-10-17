//! 👆️ Drawing editor unit laws over pointers: gesture preview, transactions, batched moves, cancelled release and the pointer wire.

use super::*;

//#region 🔖️GesturePreview
#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_while_idle() {
    let session = DrawingSession::default();
    assert_eq!(session.preview().phase, canvas_pointer_down::DrawingGesturePreviewPhase::Idle, "idle has an empty fixed projection");
}

/// 🖱️ A press of `utility` at `world` without modifiers.
pub(super) fn pointer(utility: &str, world: [f64; 2]) -> canvas_pointer_down::DrawingPointer {
    canvas_pointer_down::DrawingPointer { utility: utility.into(), world, shift: false, alt: false, ctrl: false, meta: false }
}

/// 🧱️ The committed base a release yields against.
fn tool_base(document: &DrawingSnapshot) -> canvas_pointer_down::DrawingToolBase {
    canvas_pointer_down::DrawingToolBase { document: std::sync::Arc::new(document.clone()), operation: None }
}

/// 🛠️ LAW (design §5): a shape drag previews from the tool context, publishes nothing per tick and commits ONE
/// transaction of one `create-layer` under `<appId>#shapeRect`, labelled from the leaf in English and German.
#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
    let mut session = semio_s_artifact_draw_drawing_editor::identity_test::session("shapeRect", "preview-seed");
    let document = default_drawing_document("empty", None);
    let down = session.press(pointer("shapeRect", [10.0, 10.0])).expect("press");
    assert!(down.artifact_mutations.is_empty() && down.transaction.is_none(), "pointer-down opens no document operation");
    let preview = session.preview();
    let seq_after_down = preview.sequence;
    assert_eq!(preview.context.start, [10.0, 10.0]);
    assert_eq!(preview.context.cursor, [10.0, 10.0]);

    let moved = session.sample([40.0, 30.0], false, false).expect("move");
    assert!(moved.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations");
    let preview = session.preview();
    assert_eq!(preview.context.cursor, [40.0, 30.0], "preview tracks the live cursor, not the drag start");
    assert!(preview.sequence > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

    let up = session.release("canvasPointerUp", pointer("shapeRect", [40.0, 30.0]), tool_base(&document)).expect("release").expect("a shape release commits");
    assert_eq!(up.artifact_mutations.len(), 1, "pointer-up commits the shape as one real DrawingMutation");
    let transaction = up.transaction.as_ref().expect("the shape commits as one tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.draw.drawing@1/*#editor#shapeRect", "{transaction:?}");

    let DrawingMutation::CreateLayer(created) = &up.artifact_mutations[0] else { panic!("a shape drag creates a layer") };
    let label = <DrawingMutation as protocol::SemanticMutation<DrawingSnapshot>>::label(&up.artifact_mutations[0]);
    let id = layer_id(&created.layer).to_string();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), format!("Create layer \"{id}\""));
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), format!("Ebene \"{id}\" erstellen"));
    assert_eq!(session.preview().phase, canvas_pointer_down::DrawingGesturePreviewPhase::Idle, "the committed projection is terminal idle");
    assert!(session.tool.at_rest());
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_tool_context() {
    let mut session = semio_s_artifact_draw_drawing_editor::identity_test::session("shapeRect", "");
    session.press(pointer("shapeRect", [1.0, 2.0])).expect("press");
    let context_before = session.tool.context().clone();
    let _ = session.preview();
    let _ = session.preview();
    assert_eq!(session.tool.context(), &context_before, "preview must never mutate the live tool context it reads");
}

/// 🔁️ LAW (design §5): two gestures are two transactions with distinct refs; a cancelled gesture leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn two_gestures_are_two_transactions_and_a_cancel_is_zero_trace() {
    let document = default_drawing_document("empty", None);
    let mut session = semio_s_artifact_draw_drawing_editor::identity_test::session("shapeRect", "two-gestures");
    let mut transactions = Vec::new();
    for (start, end) in [([0.0, 0.0], [20.0, 10.0]), ([30.0, 30.0], [60.0, 50.0])] {
        session.press(pointer("shapeRect", start)).expect("press");
        session.sample(end, false, false).expect("move");
        let emit = session.release("canvasPointerUp", pointer("shapeRect", end), tool_base(&document)).expect("release").expect("commit");
        transactions.push(emit.transaction.expect("each gesture commits its own transaction"));
    }
    assert_ne!(transactions[0].id, transactions[1].id);
    assert_eq!(transactions[0].tool, transactions[1].tool);
    session.press(pointer("shapeRect", [0.0, 0.0])).expect("press");
    session.sample([40.0, 40.0], false, false).expect("move");
    let cancelled = session.cancel();
    assert!(cancelled.artifact_mutations.is_empty() && cancelled.transaction.is_none() && cancelled.effects.is_empty());
    assert!(session.tool.at_rest() && session.tool.provisional().is_none());
}

//#region 🧵️BatchedSamplesAndCancel
fn session_with(utility: &str) -> (DrawingSession, DrawingSnapshot, NoConfig, semio_framework_plugin::HistoryView) {
    (semio_s_artifact_draw_drawing_editor::identity_test::session(utility, ""), default_drawing_document("empty", None), NoConfig::default(), semio_framework_plugin::HistoryView::empty())
}

/// 🧵️ LAW (design L4 / §2 D): a batch of four samples leaves the shape drag exactly where four
/// separate moves left it — the cursor follows the LAST sample, not an intermediate one.
#[semio_framework_async_macros::async_test]
async fn a_batched_move_drives_the_gesture_to_its_last_sample() {
    let path = [[420.0, 320.0], [480.0, 300.0], [520.0, 380.0], [460.0, 360.0]];
    let run = |batched: bool| {
        let (mut session, document, config, history) = session_with("shapeRect");
        let view = ArtifactView::with_operation(&document, &history,semio_s_artifact_draw_drawing_editor::identity_test::operation());
        let cfg = ConfigView { snapshot: &config, window: None };
        session.press(pointer("shapeRect", [0.0, 0.0])).expect("press");
        if batched {
            let [x, y] = path[3];
            let emit = canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false,  x, y, width: 800.0, height: 600.0, samples: path.to_vec() }, &view, &cfg, &mut session).expect("batched move");
            assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "a mid-drag batch emits no operation");
        } else {
            for [x, y] in path {
                canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false,  x, y, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
            }
        }
        session.tool.context().clone()
    };
    let one_per_event = run(false);
    let one_batch = run(true);
    assert_eq!(one_batch.cursor, one_per_event.cursor, "the batch ends on the same cursor as four separate moves");
    assert_eq!(one_batch.start, one_per_event.start);
    let (session, ..) = session_with("shapeRect");
    let expected = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, 460.0, 360.0, 800.0, 600.0);
    assert_eq!(one_batch.cursor, [expected.0, expected.1], "the cursor is the LAST sample of the batch");
}

/// 🚫️ LAW (design §2 D): a cancelled release drops a live drag and commits/selects nothing.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_release_commits_nothing_and_leaves_the_gesture_idle() {
    for utility in ["shapeRect", "selectMarquee"] {
        let (mut session, document, config, history) = session_with(utility);
        let view = ArtifactView::with_operation(&document, &history,semio_s_artifact_draw_drawing_editor::identity_test::operation());
        let cfg = ConfigView { snapshot: &config, window: None };
        session.press(pointer(utility, [0.0, 0.0])).expect("press");
        canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 600.0, y: 500.0, width: 800.0, height: 600.0, samples: Vec::new() }, &view, &cfg, &mut session).expect("move");
        assert!(!session.tool.at_rest(), "{utility}: the drag is live before the cancel");
        let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { alt: false, x: 600.0, y: 500.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{utility}: a cancel never commits");
        assert!(emit.effects.is_empty(), "{utility}: a cancel never selects or resets the utility");
        assert!(session.tool.at_rest(), "{utility}: no gesture survives a cancel");
        assert!(session.point_query.is_none(), "{utility}: no marquee/pick query is retained");
    }
    let (mut session, document, config, history) = session_with("selectDirect");
    let view = ArtifactView::with_operation(&document, &history,semio_s_artifact_draw_drawing_editor::identity_test::operation());
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { alt: false, x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: true }, &view, &cfg, &mut session).expect("cancel");
    assert!(emit.effects.is_empty() && emit.artifact_mutations.is_empty(), "an idle cancel never falls back to a pick");
    assert!(session.tool.at_rest());
}

/// 🧵️ LAW: a legacy one-per-event wire (no `samples`, no `cancelled`) decodes as one sample at
/// `(x, y)` and a real release.
#[semio_framework_async_macros::async_test]
async fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    use semio_framework_value::FromValue;
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(legacy.clone()).expect("legacy move decodes");
    assert!(moved.samples.is_empty());
    assert_eq!(moved.samples_or_last(), vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.last_sample(), [5.0, 6.0]);
    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::Object(vec![("x".into(), f(3.0)), ("y".into(), f(4.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(2.0, 2.5), pair(3.0, 4.0)]))]);
    let moved = canvas_pointer_move::CanvasPointerMove::from_value(batched).expect("batched move decodes");
    assert_eq!(moved.samples, vec![[1.0, 1.5], [2.0, 2.5], [3.0, 4.0]]);
    assert_eq!(moved.last_sample(), [3.0, 4.0]);
    let modified = canvas_pointer_move::CanvasPointerMove::from_value(semio_framework_value::DslValue::Object(vec![("x".into(),f(3.0)),("y".into(),f(4.0)),("width".into(),f(800.0)),("height".into(),f(600.0)),("shift".into(),semio_framework_value::DslValue::Bool(true)),("alt".into(),semio_framework_value::DslValue::Bool(true))])).expect("live modifiers decode");
    assert!(modified.shift && modified.alt);
    let legacy_up = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0)), ("width".into(), f(800.0)), ("height".into(), f(600.0)), ("shift".into(), semio_framework_value::DslValue::Bool(false)), ("ctrl".into(), semio_framework_value::DslValue::Bool(false)), ("meta".into(), semio_framework_value::DslValue::Bool(false))]);
    let released = canvas_pointer_up::CanvasPointerUp::from_value(legacy_up).expect("legacy release decodes");
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
}
//#endregion 🧵️BatchedSamplesAndCancel
//#endregion 🔖️GesturePreview
