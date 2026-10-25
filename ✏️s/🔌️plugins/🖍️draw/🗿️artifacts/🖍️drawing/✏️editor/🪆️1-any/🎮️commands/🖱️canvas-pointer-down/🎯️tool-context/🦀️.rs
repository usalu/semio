//! 🎯️ Drawing tool context: the lasso polygon, the grab and tool context records, the interaction effects and the gesture preview layers.

use super::*;

//#region 🔖️ToolContext
pub const DRAWING_GESTURE_PREVIEW_POINT_CAPACITY: usize = 256;

/// 🪢 Bounded lasso polygon retained during incremental document traversal.
#[derive(Clone, Debug)]
pub struct LassoPolygon {
    pub(super) points: [[f64;2];DRAWING_GESTURE_PREVIEW_POINT_CAPACITY],
    pub(super) len: usize,
}

impl LassoPolygon {
    pub(super) fn from_points(points: &UiFixedList<[f64;2],DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, end: [f64;2]) -> Self {
        let mut polygon=Self { points: [[0.0;2];DRAWING_GESTURE_PREVIEW_POINT_CAPACITY],len: points.len() };
        for (index,point) in points.iter().enumerate() { polygon.points[index]=*point; }
        if polygon.len==0 || polygon.points[polygon.len-1]!=end {
            let index=polygon.len.min(DRAWING_GESTURE_PREVIEW_POINT_CAPACITY-1);
            polygon.points[index]=end;
            polygon.len=(index+1).min(DRAWING_GESTURE_PREVIEW_POINT_CAPACITY);
        }
        polygon
    }
    pub(super) fn as_slice(&self) -> &[[f64;2]] { &self.points[..self.len] }
}

pub(super) fn append_lasso_point(context: &mut DrawingToolContext, point: [f64;2], spacing: f64) {
    context.lasso_spacing=context.lasso_spacing.max(spacing).max(1e-9);
    if let Some(previous)=context.points.get(context.points.len().saturating_sub(1)) {
        if (point[0]-previous[0]).hypot(point[1]-previous[1])<context.lasso_spacing { return; }
    }
    if context.points.len()==DRAWING_GESTURE_PREVIEW_POINT_CAPACITY {
        let retained=context.points.len()/2;
        for index in 0..retained { let point=context.points[index*2]; context.points[index]=point; }
        while context.points.len()>retained { context.points.pop(); }
        context.lasso_spacing*=2.0;
    }
    if context.points.try_push(point).is_err() { context.points_overflowed=true; }
}

/// ✊️ What a press grabbed once the session resolved it against the document: layers — with the handle and the
/// selection bounds when a transform handle was hit — or path points.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawingGrab {
    Layers { targets: PagedList<PagedUtf8<{usize::MAX}>,{usize::MAX}>, handle: Option<(usize, [f64; 4])> },
    Points { targets: Vec<DrawingPathPointTarget> },
}

/// 🎛️ The canvas tool's context — one flat record (XState convention: context is machine-global, never per-state):
/// the press, the cursor, the marquee or draft points, and what a drag grabbed. Tool state, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingToolContext {
    pub method: String,
    pub(super) lasso_spacing: f64,
    pub(super) merge: String,
    pub utility: String,
    pub start: [f64; 2],
    pub cursor: [f64; 2],
    pub points: UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>,
    pub points_overflowed: bool,
    pub(super) active: bool,
    pub grab: Option<DrawingGrab>,
    pub(super) constrained: bool,
    pub(super) centered: bool,
}

pub(super) fn tool_context_from_input(_input: ()) -> DrawingToolContext {
    DrawingToolContext::default()
}
//#endregion 🔖️ToolContext

//#region 🔖️DocumentHelpers
pub fn canvas_point_to_world(viewport: &store::Viewport2d, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = viewport.zoom.max(0.01);
    ((x - viewport_w * 0.5) / zoom + viewport.x, (y - viewport_h * 0.5) / zoom + viewport.y)
}

/// 🎯️ Maps shift/ctrl/meta modifiers to a framework `MergeMode` wire string (matches
/// `@semio-tech/ui-react`'s `marqueeModeFromModifiers`) — the actual set algebra runs inside the framework's
/// `next_selection` machine; this crate only ever computes WHICH ids were hit and asks the framework to apply them.
pub fn selection_merge_mode(shift: bool, ctrl: bool, meta: bool) -> &'static str {
    let ctrl_or_meta = ctrl || meta;
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "replace"
    }
}

/// 🕹️ Requests the shell to redispatch a framework-owned interaction verb (`interactionSelect`/`interactionHover`)
/// through its normal action funnel — selection and hover are framework-owned state, never a document mutation.
pub fn request_interaction_action(action_id: &str, args: semio_framework_value::DslValue) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(args) }
}

pub fn interaction_select_effect_from_targets(targets: String, merge: &str) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_SELECT_ACTION_ID,
        semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_DOMAIN.to_string())),
            ("targets".to_string(), semio_framework_value::DslValue::String(targets)),
            ("merge".to_string(), semio_framework_value::DslValue::String(merge.to_string())),
            ("method".to_string(), semio_framework_value::DslValue::String("pick".to_string())),
        ]),
    )
}

pub fn interaction_hover_effect_from_targets(targets: String) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_HOVER_ACTION_ID,
        semio_framework_value::DslValue::object([("domainId".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_DOMAIN.to_string())), ("channel".to_string(), semio_framework_value::DslValue::String("pointer".to_string())), ("targets".to_string(), semio_framework_value::DslValue::String(targets))]),
    )
}

pub fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    let items = ids.iter().map(|id| semio_framework_value::DslValue::object([("granularity".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_GRANULARITY.to_string())), ("id".to_string(), semio_framework_value::DslValue::String(id.clone()))])).collect::<Vec<_>>();
    let targets = semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(items));
    interaction_select_effect_from_targets(targets, merge)
}

pub fn point_selection_effect(ids:&[String])->Effect {
    let targets=ids.iter().map(|id|semio_framework_value::DslValue::object([("granularity".into(),semio_framework_value::DslValue::String(DRAWING_POINT_GRANULARITY.into())),("id".into(),semio_framework_value::DslValue::String(id.clone()))])).collect::<Vec<_>>();
    point_selection_effect_from_targets(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(targets)))
}

pub fn point_selection_effect_from_targets(targets:String)->Effect {
    request_interaction_action(semio_framework::INTERACTION_SELECT_ACTION_ID,semio_framework_value::DslValue::object([
        ("domainId".into(),semio_framework_value::DslValue::String(DRAWING_POINT_DOMAIN.into())),
        ("targets".into(),semio_framework_value::DslValue::String(targets)),
        ("merge".into(),semio_framework_value::DslValue::String("replace".into())),
        ("method".into(),semio_framework_value::DslValue::String("pick".into())),
    ]))
}

pub const DRAWING_MARQUEE_THRESHOLD_PX: f64 = 4.0;

pub const DRAWING_PICK_TOLERANCE_PX: f64 = 8.0;

pub fn shape_preview_segments(utility: &str, start: [f64; 2], end: [f64; 2]) -> UiFixedList<PathSegment, 7> {
    let mut segments = UiFixedList::default();
    if utility == "shapeLine" {
        let _ = segments.try_push(PathSegment::Move { to: start });
        let _ = segments.try_push(PathSegment::Line { to: end });
        return segments;
    }
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if utility == "shapeRect" {
        for segment in [PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close] {
            let _ = segments.try_push(segment);
        }
        return segments;
    }
    let cx = x + width / 2.0;
    let cy = y + height / 2.0;
    let rx = width / 2.0;
    let ry = height / 2.0;
    let k = 0.552_284_749_8;
    for segment in [
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + rx * k, cy - ry], ctrl2: [cx + rx, cy - ry * k], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + ry * k], ctrl2: [cx + rx * k, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - rx * k, cy + ry], ctrl2: [cx - rx, cy + ry * k], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - ry * k], ctrl2: [cx - rx * k, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ] {
        let _ = segments.try_push(segment);
    }
    segments
}

pub fn draft_preview_segments(utility: &str, points: &UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, cursor: [f64; 2]) -> UiFixedList<PathSegment, { DRAWING_GESTURE_PREVIEW_POINT_CAPACITY + 2 }> {

    let mut segments = UiFixedList::default();
    if points.is_empty() {
        return segments;
    }
    let _ = segments.try_push(PathSegment::Move { to: points[0] });
    for point in points.iter().skip(1) {
        let _ = segments.try_push(PathSegment::Line { to: *point });
    }
    let _ = segments.try_push(PathSegment::Line { to: cursor });
    if utility == "shapePolygon" && points.len() > 1 {
        let _ = segments.try_push(PathSegment::Close);
    }
    segments
}

/// 🔷️ The `create-layer` a shape drag yields, `None` when the drag is too small to commit.
pub(super) fn shape_drag_layer(doc: &DrawingSnapshot, utility: &str, start: [f64; 2], end: [f64; 2], identity:semio_s_artifact_draw_drawing_model::identity::DrawingIdentity) -> Option<DrawingMutation> {
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if width < 1.0 && height < 1.0 {
        return None;
    }
    let (name, shape_kind, _geometry) = match utility {
        "shapeLine" => ("Line", "line", [start[0], start[1], end[0], end[1]]),
        "shapeEllipse" => ("Ellipse", "ellipse", [x + width / 2.0, y + height / 2.0, width / 2.0, height / 2.0]),
        _ => ("Rectangle", "rect", [x, y, width, height]),
    };
    let base=semio_s_artifact_draw_drawing_model::default_layer_base(identity,name);
    let mut layer = DrawingLayerNode::Shape(semio_s_artifact_draw_drawing_model::DrawingShapeBody {
        base,
        shape_kind: shape_kind.into(),
        rect: if utility == "shapeRect" { Some(semio_s_artifact_draw_drawing_model::DrawingRect { x, y, width, height }) } else { None },
        ellipse: if utility == "shapeEllipse" { Some(semio_s_artifact_draw_drawing_model::DrawingEllipse { cx: x + width / 2.0, cy: y + height / 2.0, rx: width / 2.0, ry: height / 2.0 }) } else { None },
        circle: None,
        line: if utility == "shapeLine" { Some(semio_s_artifact_draw_drawing_model::DrawingLine { x1: start[0], y1: start[1], x2: end[0], y2: end[1] }) } else { None },
        polygon: None,
    });
    crate::initialize_appearance(&mut layer);
    Some(semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🖊️ The `create-layer` a committed pen or polygon draft yields, `None` below two points.
pub(super) fn draft_layer(doc: &DrawingSnapshot, utility: &str, points: &UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, identity:semio_s_artifact_draw_drawing_model::identity::DrawingIdentity) -> Option<DrawingMutation> {
    if points.len() < 2 {
        return None;
    }
    let mut layer = if utility == "pen" {
        create_drawing_path_layer(identity, "Path", points.iter().enumerate().map(|(index, point)| if index == 0 { PathSegment::Move { to: *point } } else { PathSegment::Line { to: *point } }).collect())
    } else {
        DrawingLayerNode::Shape(semio_s_artifact_draw_drawing_model::DrawingShapeBody {
            base: semio_s_artifact_draw_drawing_model::default_layer_base(identity, "Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(semio_s_artifact_draw_drawing_model::DrawingPolygon { points: points.iter().copied().collect() }),
        })
    };
    crate::initialize_appearance(&mut layer);
    Some(semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🖼️ The `create-layer` a finished trace yields, `None` without an image source.
pub(super) fn trace_layer(doc: &DrawingSnapshot, source_key: Option<&str>,identity:semio_s_artifact_draw_drawing_model::identity::DrawingIdentity) -> Option<DrawingMutation> {
    let source_key=source_key?;
    let mut layer = create_drawing_trace_layer(identity, "Trace", source_key);
    crate::initialize_appearance(&mut layer);
    Some(semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🧰️ The host effects a committed creation carries beside its one edit: the created layers become the selection and the
/// canvas returns to the default select utility (the active utility is host-owned, never a document operation).
pub(super) fn created_layer_effects(mutations: &[DrawingMutation]) -> Vec<Effect> {
    let created = mutations.iter().filter_map(|mutation| match mutation {
        DrawingMutation::CreateLayer(create) => Some(layer_id(&create.layer).to_string()),
        _ => None,
    }).collect::<Vec<_>>();
    if created.is_empty() {
        return Vec::new();
    }
    vec![interaction_select_effect(&created, "replace"), Effect::SetActiveUtility { window_id: crate::modes::edit::windows::canvas::DRAWING_PLAY_WINDOW_CANVAS.into(), utility_id: semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY.into() }]
}

/// 🎚️ The world-space motion a drag grabbed so far, as the parametric leaf it yields — `None` for the identity or a
/// motion no leaf admits (a non-finite offset, a collapsing zero scale).
pub fn drawing_grab_leaf(grab: &DrawingGrab, start: [f64; 2], cursor: [f64; 2], constrained: bool, centered: bool) -> Option<DrawingMutation> {
    match grab {
        DrawingGrab::Layers { targets, handle: None } => {
            let (dx, dy) = (cursor[0] - start[0], cursor[1] - start[1]);
            (dx.is_finite() && dy.is_finite() && (dx, dy) != (0.0, 0.0)).then(|| drag_layers(targets.clone(), dx, dy))
        }
        DrawingGrab::Layers { targets, handle: Some((handle, bounds)) } => match handle_motion(*handle, *bounds, start, cursor, constrained, centered)? {
            HandleMotion::Rotate { pivot, angle } => (angle != 0.0).then(|| rotate_layers(targets.clone(), pivot[0], pivot[1], angle)),
            HandleMotion::Scale { pivot, scale } => (scale != [1.0, 1.0] && scale[0] != 0.0 && scale[1] != 0.0).then(|| scale_layers(targets.clone(), pivot[0], pivot[1], scale[0], scale[1])),
        },
        DrawingGrab::Points { targets } => {
            let mut delta = [cursor[0] - start[0], cursor[1] - start[1]];
            if constrained {
                if delta[0].abs() >= delta[1].abs() { delta[1] = 0.0; } else { delta[0] = 0.0; }
            }
            (delta.iter().all(|value| value.is_finite()) && delta != [0.0, 0.0]).then(|| drag_path_points(targets.clone().into(), delta[0], delta[1]))
        }
    }
}
//#endregion 🔖️DocumentHelpers
