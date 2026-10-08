//! 🖼️ Drawing play app — the canvas window's render() (constitutional: was `ui`'s `Render` region).

use crate::editor::drawing::commands::canvas_pointer_down::{draft_preview_segments, shape_preview_segments, DrawingGesturePreview, DrawingGesturePreviewPhase};
use crate::schema::resolve_drawing_artboard;
use crate::{DrawingArtboard, DrawingSnapshot, PathSegment};
use semio_framework_value::DslValue;
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, UiAssemblyResult};

pub const DRAWING_PLAY_WINDOW_CANVAS: &str = "drawing-composite";
pub const DRAWING_PLAY_SURFACE_ID: &str = "drawing.play.composite";
pub const DRAWING_PLAY_BODY_COMPOSITE: &str = "drawing.play.composite";

#[path = "🎚️config/🦀️.rs"]
pub mod config;
#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

const DRAWING_OVERLAY_SELECTION_STROKE: [f64; 4] = [0.98, 0.75, 0.14, 0.95];
const DRAWING_OVERLAY_SELECTION_FILL: [f64; 4] = [0.98, 0.75, 0.14, 0.16];
const DRAWING_OVERLAY_MARQUEE_STROKE: [f64; 4] = [0.36, 0.65, 0.98, 0.9];
const DRAWING_OVERLAY_MARQUEE_FILL: [f64; 4] = [0.36, 0.65, 0.98, 0.12];
const DRAWING_ARTBOARD_FILL: [f64; 4] = [0.969, 0.953, 0.890, 1.0];
const DRAWING_ARTBOARD_STROKE: [f64; 4] = [0.198, 0.223, 0.205, 0.55];
const DRAWING_ARTBOARD_LABEL: [f64; 4] = [0.198, 0.223, 0.205, 0.92];

fn overlay_record<T: semio_framework_value::ToValue + ?Sized>(id: &str, transform: [f64; 6], segments: &T, fill: Option<[f64; 4]>, stroke_color: [f64; 4], stroke_width: f64) -> DslValue {
    semio_framework_value::DslValue::object([
        ("id".to_string(), semio_framework_value::DslValue::String(id.to_string())),
        ("role".to_string(), semio_framework_value::DslValue::String("overlay".to_string())),
        ("transform".to_string(), semio_framework_value::ToValue::to_value(&transform.to_vec())),
        ("segments".to_string(), semio_framework_value::ToValue::to_value(segments)),
        ("fill".to_string(), fill.map_or(semio_framework_value::DslValue::Null, |color| semio_framework_value::DslValue::object([("kind".to_string(), semio_framework_value::DslValue::String("solid".to_string())), ("color".to_string(), semio_framework_value::ToValue::to_value(&color.to_vec()))]))),
        (
            "stroke".to_string(),
            semio_framework_value::DslValue::object([
                ("color".to_string(), semio_framework_value::ToValue::to_value(&stroke_color.to_vec())),
                ("width".to_string(), semio_framework_value::DslValue::float(stroke_width)),
                ("cap".to_string(), semio_framework_value::DslValue::String("round".to_string())),
                ("join".to_string(), semio_framework_value::DslValue::String("round".to_string())),
            ]),
        ),
        ("opacity".to_string(), semio_framework_value::DslValue::float(1.0)),
        ("blendMode".to_string(), semio_framework_value::DslValue::String("normal".to_string())),
        ("visible".to_string(), semio_framework_value::DslValue::Bool(true)),
        ("fillRule".to_string(), semio_framework_value::DslValue::String("evenodd".to_string())),
    ])
}

/// 📐️ Formats one artboard edge length for the dimension label (integers stay bare).
fn format_artboard_dimension(value: f64) -> String {
    if (value - value.round()).abs() < 1e-6 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.2}")
    }
}

/// 🖼️ Artboard paper + `W × H` dimension label — drawn under document content.
fn artboard_scene_records(document: &DrawingSnapshot) -> Vec<DslValue> {
    let artboard = resolve_drawing_artboard(document).unwrap_or(DrawingArtboard { width: 1024.0, height: 1024.0 });
    let width = artboard.width.max(1.0);
    let height = artboard.height.max(1.0);
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [width, 0.0] }, PathSegment::Line { to: [width, height] }, PathSegment::Line { to: [0.0, height] }, PathSegment::Close];
    let label = format!("{} × {}", format_artboard_dimension(width), format_artboard_dimension(height));
    let label_size = 12.0_f64;
    let label_x = (width * 0.5) - (label.len() as f64 * label_size * 0.28);
    vec![
        overlay_record("artboard:frame", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAWING_ARTBOARD_FILL), DRAWING_ARTBOARD_STROKE, 1.0),
        semio_framework_value::DslValue::object([
            ("id".to_string(), semio_framework_value::DslValue::String("artboard:dimensions".to_string())),
            ("role".to_string(), semio_framework_value::DslValue::String("overlay".to_string())),
            ("transform".to_string(), semio_framework_value::ToValue::to_value(&vec![1.0_f64, 0.0, 0.0, 1.0, label_x, height + label_size * 0.35])),
            ("segments".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
            ("fill".to_string(), semio_framework_value::DslValue::object([("kind".to_string(), semio_framework_value::DslValue::String("solid".to_string())), ("color".to_string(), semio_framework_value::ToValue::to_value(&DRAWING_ARTBOARD_LABEL.to_vec()))])),
            ("opacity".to_string(), semio_framework_value::DslValue::float(1.0)),
            ("blendMode".to_string(), semio_framework_value::DslValue::String("normal".to_string())),
            ("visible".to_string(), semio_framework_value::DslValue::Bool(true)),
            ("text".to_string(), semio_framework_value::DslValue::object([("content".to_string(), semio_framework_value::DslValue::String(label)), ("size".to_string(), semio_framework_value::DslValue::float(label_size))])),
        ]),
    ]
}

/// 🎯️ Projects the request-owned selection and the retained gesture preview into shared canvas paths.
pub fn render(prepared: Option<&crate::schema::scene_paint::scene::PreparedScene>, revision: u32, document: &DrawingSnapshot, config: &config::DrawingCanvasWindowConfig, preview: &DrawingGesturePreview, active_utility: &str, selection: &[String], point_selection: &[String]) -> UiAssemblyResult<BuiltNode> {
    let plan=prepared.map(|scene|&scene.plan);
    let mut scene_nodes=match plan{Some(plan)=>crate::schema::scene_view::nodes(plan,preview.transformation.as_ref()).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.geometry.canvas",error.to_string()))?,None=>Vec::new()};
    let selected_bounds=if let Some(scene)=prepared{crate::schema::scene_paint::scene::query::prepared_selection_bounds(scene,selection).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.geometry.selection",error.to_string()))?.map(|[x,y,r,b]|[x,y,r-x,b-y])}else{None};
    if let Some(movement)=&preview.node_translation {
        for node in &mut scene_nodes {
            let points=movement.targets.iter().filter(|target|target.layer_id==node.id).map(|target|crate::schema::geometry::editing::PathPointRef {index:target.index,point:target.point}).collect::<Vec<_>>();
            if points.is_empty() {continue;}
            if let Ok(segments)=crate::schema::geometry::editing::translate_world_path_points(node.segments.iter(),&points,node.transform,movement.delta) {node.segments=std::borrow::Cow::Owned(segments);}
        }
    }
    let artboard_records = artboard_scene_records(document);
    let mut records: Vec<DslValue> = Vec::with_capacity(scene_nodes.len() + artboard_records.len() + 4);
    records.push(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String("meta:utility".to_string())), ("role".to_string(), semio_framework_value::DslValue::String("meta".to_string())), ("utility".to_string(), semio_framework_value::DslValue::String(active_utility.to_string()))]));
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(semio_framework_value::ToValue::to_value(node));
    }
    if active_utility=="editNodes" {
        let zoom=config.viewport.zoom.max(1e-6);
        for node in scene_nodes.iter().filter(|node|selection.iter().any(|id|id==node.id)) {
            if !matches!(crate::schema::find_drawing_layer(document,&node.id),Some(crate::DrawingLayerNode::Path(_))) || crate::schema::drawing_layer_is_locked(document,&node.id) {continue;}
            let [a,b,c,d,e,f]=node.transform;
            let world=|point:[f64;2]|[a*point[0]+c*point[1]+e,b*point[0]+d*point[1]+f];
            let selected=point_selection.iter().filter_map(|id|crate::editor::drawing::interaction::points::parse_point_id(id)).filter(|point|point.layer_id==node.id).collect::<Vec<_>>();
            let geometry=if selected.is_empty() {None} else {match crate::schema::find_drawing_layer(document,&node.id) {Some(crate::DrawingLayerNode::Path(path))=>crate::editor::drawing::interaction::points::geometry_id(&path.segments),_=>None}};
            let selected_point=|index,point|selected.iter().any(|reference|reference.index==index && reference.point==point && Some(reference.geometry)==geometry.as_deref());
            let mut anchors=Vec::new();let mut controls=Vec::new();let mut stems=Vec::new();
            let mut selected_anchors=Vec::new();let mut selected_controls=Vec::new();
            let mut previous=[0.0,0.0];let mut start=previous;
            for (index,segment) in node.segments.iter().enumerate() {
                let to=match segment {PathSegment::Move {to}|PathSegment::Line {to}|PathSegment::Quad {to,..}|PathSegment::Cubic {to,..}|PathSegment::Arc {to,..}=>*to,PathSegment::Close=>{previous=start;continue;}};
                let mut control=|point:[f64;2],anchor:[f64;2],kind| {
                    let [x,y]=world(point);let radius=4.0/zoom;
                    let target=if selected_point(index,kind) {&mut selected_controls} else {&mut controls};
                    target.extend([PathSegment::Move {to:[x,y-radius]},PathSegment::Line {to:[x+radius,y]},PathSegment::Line {to:[x,y+radius]},PathSegment::Line {to:[x-radius,y]},PathSegment::Close]);
                    stems.extend([PathSegment::Move {to:world(anchor)},PathSegment::Line {to:[x,y]}]);
                };
                match segment {
                    PathSegment::Move {..}=>start=to,
                    PathSegment::Quad {ctrl,..}=>{control(*ctrl,previous,crate::schema::geometry::editing::PathPoint::Control1);stems.extend([PathSegment::Move {to:world(*ctrl)},PathSegment::Line {to:world(to)}]);},
                    PathSegment::Cubic {ctrl1,ctrl2,..}=>{control(*ctrl1,previous,crate::schema::geometry::editing::PathPoint::Control1);control(*ctrl2,to,crate::schema::geometry::editing::PathPoint::Control2);},
                    _=>{},
                }
                let [x,y]=world(to);let radius=4.0/zoom;
                let target=if selected_point(index,crate::schema::geometry::editing::PathPoint::Anchor) {&mut selected_anchors} else {&mut anchors};
                target.extend([PathSegment::Move {to:[x-radius,y-radius]},PathSegment::Line {to:[x+radius,y-radius]},PathSegment::Line {to:[x+radius,y+radius]},PathSegment::Line {to:[x-radius,y+radius]},PathSegment::Close]);
                previous=to;
            }
            let identity=[1.0,0.0,0.0,1.0,0.0,0.0];
            records.push(overlay_record(&format!("overlay:node:{}:stems",node.id),identity,&stems,None,DRAWING_OVERLAY_MARQUEE_STROKE,1.0/zoom));
            records.push(overlay_record(&format!("overlay:node:{}:controls",node.id),identity,&controls,Some([1.0;4]),DRAWING_OVERLAY_MARQUEE_STROKE,1.0/zoom));
            records.push(overlay_record(&format!("overlay:node:{}:anchors",node.id),identity,&anchors,Some([1.0;4]),DRAWING_OVERLAY_SELECTION_STROKE,1.0/zoom));
            records.push(overlay_record(&format!("overlay:node:{}:selected-controls",node.id),identity,&selected_controls,Some(DRAWING_OVERLAY_MARQUEE_STROKE),DRAWING_OVERLAY_MARQUEE_STROKE,1.0/zoom));
            records.push(overlay_record(&format!("overlay:node:{}:selected-anchors",node.id),identity,&selected_anchors,Some(DRAWING_OVERLAY_SELECTION_STROKE),DRAWING_OVERLAY_SELECTION_STROKE,1.0/zoom));
        }
    }
    if active_utility=="selectDirect" {
        if let Some(bounds)=selected_bounds {
            let [x,y,w,h]=bounds;
            let transform=preview.transformation.as_ref().map_or([1.0,0.0,0.0,1.0,0.0,0.0],|(_,matrix)|*matrix);
            let zoom=config.viewport.zoom.max(1e-6);
            let outline=vec![PathSegment::Move {to:[x,y]},PathSegment::Line {to:[x+w,y]},PathSegment::Line {to:[x+w,y+h]},PathSegment::Line {to:[x,y+h]},PathSegment::Close];
            records.push(overlay_record("overlay:selection-bounds",transform,&outline,None,DRAWING_OVERLAY_SELECTION_STROKE,1.0/zoom));
            let points=crate::schema::geometry::handles::handle_points(bounds,zoom);
            let connector=vec![PathSegment::Move {to:points[1]},PathSegment::Line {to:points[8]}];
            records.push(overlay_record("overlay:rotation-stem",transform,&connector,None,DRAWING_OVERLAY_SELECTION_STROKE,1.0/zoom));
            for (index,point) in points.into_iter().enumerate() {
                let [a,b,c,d,e,f]=transform;
                let [cx,cy]=[a*point[0]+c*point[1]+e,b*point[0]+d*point[1]+f];
                let radius=4.0/zoom;
                let square=vec![PathSegment::Move {to:[cx-radius,cy-radius]},PathSegment::Line {to:[cx+radius,cy-radius]},PathSegment::Line {to:[cx+radius,cy+radius]},PathSegment::Line {to:[cx-radius,cy+radius]},PathSegment::Close];
                records.push(overlay_record(&format!("overlay:transform-handle:{index}"),[1.0,0.0,0.0,1.0,0.0,0.0],&square,Some([1.0,1.0,1.0,1.0]),DRAWING_OVERLAY_SELECTION_STROKE,1.0/zoom));
            }
        }
    }
    if preview.phase == DrawingGesturePreviewPhase::Marquee {
        let ctx = &preview.context;
        let x = ctx.start[0].min(ctx.cursor[0]);
        let y = ctx.start[1].min(ctx.cursor[1]);
        let width = (ctx.cursor[0] - ctx.start[0]).abs();
        let height = (ctx.cursor[1] - ctx.start[1]).abs();
        let segments = if ctx.method == "lasso" {
            let mut segments=ctx.points.iter().enumerate().map(|(index,point)| if index==0 { PathSegment::Move { to: *point } } else { PathSegment::Line { to: *point } }).collect::<Vec<_>>();
            segments.push(PathSegment::Line { to: ctx.cursor });
            segments.push(PathSegment::Close);
            segments
        } else { vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close] };
        records.push(overlay_record("overlay:marquee", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAWING_OVERLAY_MARQUEE_FILL), DRAWING_OVERLAY_MARQUEE_STROKE, 1.0));
    } else if preview.phase == DrawingGesturePreviewPhase::Shape {
        let ctx = &preview.context;
        let segments = shape_preview_segments(&ctx.utility, ctx.start, ctx.cursor).iter().cloned().collect::<Vec<_>>();
        records.push(overlay_record("overlay:preview", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAWING_OVERLAY_SELECTION_FILL), DRAWING_OVERLAY_SELECTION_STROKE, 1.5));
    } else if preview.phase == DrawingGesturePreviewPhase::Draft {
        let ctx = &preview.context;
        let segments = draft_preview_segments(&ctx.utility, &ctx.points, ctx.cursor).iter().cloned().collect::<Vec<_>>();
        records.push(overlay_record("overlay:preview", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAWING_OVERLAY_SELECTION_FILL), DRAWING_OVERLAY_SELECTION_STROKE, 1.5));
    }
    scene_surface(
        DRAWING_PLAY_SURFACE_ID,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { framing: (!config.framed && plan.is_some()).then(|| semio_framework_plugin::Canvas2dFraming { revision,bounds: crate::schema::scene_view::bounds(document.artboard.as_ref(),&scene_nodes),padding: 48.0 }), camera_x: config.viewport.x, camera_y: config.viewport.y, zoom: config.viewport.zoom, layers_json: semio_framework_pack_json::to_json_string(&records), snapshot: None, tool_run_trace: None, lanes: Vec::new() },
    )
}
