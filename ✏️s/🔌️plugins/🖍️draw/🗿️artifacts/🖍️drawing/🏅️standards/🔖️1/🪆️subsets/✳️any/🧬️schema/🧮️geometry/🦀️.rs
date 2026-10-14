//! 📐️ Affine composition and exact cubic extrema, independent of rendering.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::PathSegment;
use semio_s_artifact_draw_drawing_model::DrawingShapeBody;
use semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix;
use crate as geometry;
use semio_s_artifact_draw_drawing_model::layer_base;
use semio_s_artifact_draw_drawing_model::DrawingCanvasLayerRecord;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::flatten_drawing_layers;
use semio_s_artifact_draw_drawing_model::layer_kind_label;
use semio_s_artifact_draw_drawing_model::DrawingTransform;
use semio_s_artifact_draw_drawing_model::DrawingArtboard;
#[path = "📏️arrangement/🦀️.rs"]
pub mod arrangement;
#[path = "📷️framing/🦀️.rs"]
pub mod framing;
#[path = "🎛️handles/🦀️.rs"]
pub mod handles;
#[path = "🎯️picking/🦀️.rs"]
pub mod picking;
#[path = "✏️editing/🦀️.rs"]
pub mod editing;
#[path = "🎯️selection/🦀️.rs"]
pub mod selection;
#[path = "📷️raster/🦀️.rs"]
pub mod raster;
pub fn multiply(a: [f64; 6], b: [f64; 6]) -> [f64; 6] {
    [a[0]*b[0]+a[2]*b[1], a[1]*b[0]+a[3]*b[1], a[0]*b[2]+a[2]*b[3], a[1]*b[2]+a[3]*b[3], a[0]*b[4]+a[2]*b[5]+a[4], a[1]*b[4]+a[3]*b[5]+a[5]]
}

pub fn cubic_bounds(points: [[f64; 2]; 4]) -> [f64; 4] {
    let mut result = [0.0; 4];
    for axis in 0..2 {
        let [p0, p1, p2, p3] = points.map(|point| point[axis]);
        let a = -p0 + 3.0*p1 - 3.0*p2 + p3;
        let b = 2.0*(p0 - 2.0*p1 + p2);
        let c = p1 - p0;
        let mut min = p0.min(p3);
        let mut max = p0.max(p3);
        let mut roots = [f64::NAN; 2];
        if a.abs() < 1e-12 {
            if b.abs() >= 1e-12 { roots[0] = -c / b; }
        } else {
            let discriminant = b*b - 4.0*a*c;
            if discriminant >= 0.0 {
                roots = [(-b + discriminant.sqrt())/(2.0*a), (-b - discriminant.sqrt())/(2.0*a)];
            }
        }
        for t in roots {
            if t > 0.0 && t < 1.0 {
                let u = 1.0-t;
                let value = u*u*u*p0 + 3.0*u*u*t*p1 + 3.0*u*t*t*p2 + t*t*t*p3;
                min = min.min(value);
                max = max.max(value);
            }
        }
        result[axis] = min;
        result[axis + 2] = max - min;
    }
    result
}

pub fn inverse(matrix: [f64; 6]) -> Option<[f64; 6]> {
    if !matrix.iter().all(|value| value.is_finite()) { return None; }
    let [a,b,c,d,e,f] = matrix;
    let determinant = a*d-b*c;
    if determinant == 0.0 || !determinant.is_finite() { return None; }
    let output = [d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant];
    output.iter().all(|value| value.is_finite()).then_some(output)
}

pub fn split_cubic(points: [[f64; 2]; 4], t: f64) -> Option<[[[f64; 2]; 4]; 2]> {
    if !(0.0..=1.0).contains(&t) || !points.iter().flatten().all(|value| value.is_finite()) { return None; }
    let mix = |a: [f64; 2], b: [f64; 2]| [a[0]*(1.0-t)+b[0]*t,a[1]*(1.0-t)+b[1]*t];
    let [a,b,c,d] = points;
    let ab = mix(a,b);
    let bc = mix(b,c);
    let cd = mix(c,d);
    let abc = mix(ab,bc);
    let bcd = mix(bc,cd);
    let center = mix(abc,bcd);
    let output = [[a,ab,abc,center],[center,bcd,cd,d]];
    output.iter().flatten().flatten().all(|value| value.is_finite()).then_some(output)
}

/// 🥚 Center-parameterized SVG ellipse, with radii corrected to reach both endpoints.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcGeometry {
    pub center: [f64; 2],
    pub radii: [f64; 2],
    pub rotation: f64,
    pub start: f64,
    pub sweep: f64,
}

impl ArcGeometry {
    pub fn point(&self, t: f64) -> [f64; 2] {
        let angle = self.start + self.sweep * t;
        let (sin, cos) = angle.sin_cos();
        let (sr, cr) = self.rotation.sin_cos();
        [self.center[0] + self.radii[0]*cos*cr - self.radii[1]*sin*sr, self.center[1] + self.radii[0]*cos*sr + self.radii[1]*sin*cr]
    }
}

pub fn arc_geometry(from: [f64; 2], radii: [f64; 2], rotation_degrees: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Option<ArcGeometry> {
    if !from.iter().chain(to.iter()).chain(radii.iter()).all(|value| value.is_finite()) || !rotation_degrees.is_finite() || radii.iter().any(|value| *value <= 0.0) || from == to { return None; }
    let rotation = rotation_degrees.to_radians();
    let (sin, cos) = rotation.sin_cos();
    let dx = (from[0] - to[0])/2.0;
    let dy = (from[1] - to[1])/2.0;
    let x = cos*dx + sin*dy;
    let y = -sin*dx + cos*dy;
    let factor = (x/radii[0]).hypot(y/radii[1]).max(1.0);
    let [rx, ry] = radii.map(|radius| radius*factor);
    let nx = x/rx; let ny = y/ry;
    let length = nx*nx + ny*ny;
    if length <= 0.0 || !length.is_finite() { return None; }
    let coefficient = ((1.0-length).max(0.0)/length).sqrt() * if large_arc == sweep { -1.0 } else { 1.0 };
    let cx = coefficient*rx*ny;
    let cy = -coefficient*ry*nx;
    let center = [cos*cx - sin*cy + (from[0]+to[0])/2.0, sin*cx + cos*cy + (from[1]+to[1])/2.0];
    let a = [(x-cx)/rx, (y-cy)/ry];
    let b = [(-x-cx)/rx, (-y-cy)/ry];
    let start = a[1].atan2(a[0]);
    let mut delta = (a[0]*b[1]-a[1]*b[0]).atan2(a[0]*b[0]+a[1]*b[1]);
    if sweep && delta < 0.0 { delta += std::f64::consts::TAU; }
    if !sweep && delta > 0.0 { delta -= std::f64::consts::TAU; }
    [center[0],center[1],rx,ry,rotation,start,delta].iter().all(|value| value.is_finite()).then_some(ArcGeometry { center, radii: [rx,ry], rotation, start, sweep: delta })
}

/// 🎯 Exact affine bounds for one path segment, usable by incremental selection queries.
pub fn segment_bounds(segment: &semio_s_artifact_draw_drawing_model::PathSegment, from: [f64;2], contour_start: [f64;2], matrix: [f64;6]) -> [f64;4] {
    use semio_s_artifact_draw_drawing_model::PathSegment;
    let map = |point: [f64;2]| [matrix[0]*point[0]+matrix[2]*point[1]+matrix[4],matrix[1]*point[0]+matrix[3]*point[1]+matrix[5]];
    let line = |a: [f64;2], b: [f64;2]| { let a=map(a); let b=map(b); [a[0].min(b[0]),a[1].min(b[1]),(a[0]-b[0]).abs(),(a[1]-b[1]).abs()] };
    match *segment {
        PathSegment::Move { to } => { let to=map(to); [to[0],to[1],0.0,0.0] }
        PathSegment::Line { to } => line(from,to),
        PathSegment::Close => line(from,contour_start),
        PathSegment::Cubic { ctrl1,ctrl2,to } => cubic_bounds([map(from),map(ctrl1),map(ctrl2),map(to)]),
        PathSegment::Quad { ctrl,to } => {
            let a=[from[0]+(ctrl[0]-from[0])*2.0/3.0,from[1]+(ctrl[1]-from[1])*2.0/3.0];
            let b=[to[0]+(ctrl[0]-to[0])*2.0/3.0,to[1]+(ctrl[1]-to[1])*2.0/3.0];
            cubic_bounds([map(from),map(a),map(b),map(to)])
        }
        PathSegment::Arc { rx,ry,rotation,large_arc,sweep,to } => {
            let Some(arc)=arc_geometry(from,[rx,ry],rotation,large_arc,sweep,to) else { return line(from,to) };
            let (sin,cos)=arc.rotation.sin_cos();
            let [rx,ry]=arc.radii;
            let u=[matrix[0]*rx*cos+matrix[2]*rx*sin,matrix[1]*rx*cos+matrix[3]*rx*sin];
            let v=[-matrix[0]*ry*sin+matrix[2]*ry*cos,-matrix[1]*ry*sin+matrix[3]*ry*cos];
            let mut result=line(from,to);
            for axis in 0..2 {
                let angle=v[axis].atan2(u[axis]);
                for candidate in [angle,angle+std::f64::consts::PI] {
                    let distance=if arc.sweep>=0.0 { (candidate-arc.start).rem_euclid(std::f64::consts::TAU) } else { (arc.start-candidate).rem_euclid(std::f64::consts::TAU) };
                    if distance<=arc.sweep.abs() {
                        let point=map(arc.point(distance/arc.sweep.abs()));
                        let max=(result[axis]+result[axis+2]).max(point[axis]);
                        result[axis]=result[axis].min(point[axis]);
                        result[axis+2]=max-result[axis];
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/📐️bounds/🦀️.rs"]
mod tests;

#[path = "↔️translation/🦀️.rs"]
pub mod translation;

pub fn layer_to_path_segments(layer: &DrawingLayerNode) -> Vec<PathSegment> {
    match layer {
        DrawingLayerNode::Path(path) => path.segments.iter().cloned().collect(),
        DrawingLayerNode::Shape(shape) => shape_to_path_segments(shape),
        _ => Vec::new(),
    }
}

fn ellipse_path_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> [PathSegment;6] {
    let arc=|to|PathSegment::Arc {rx:rx.abs(),ry:ry.abs(),rotation:0.0,large_arc:false,sweep:(rx>=0.0)==(ry>=0.0),to};
    [PathSegment::Move {to:[cx,cy-ry]},arc([cx+rx,cy]),arc([cx,cy+ry]),arc([cx-rx,cy]),arc([cx,cy-ry]),PathSegment::Close]
}

/// 🔷️ Reads one primitive contour segment without cloning a polygon.
pub fn shape_path_segment(shape:&DrawingShapeBody,index:usize)->Option<PathSegment> {
    match () {
        () if shape.shape_kind.eq_str("rect")=>shape.rect.as_ref().and_then(|r|[
            PathSegment::Move {to:[r.x,r.y]},PathSegment::Line {to:[r.x+r.width,r.y]},
            PathSegment::Line {to:[r.x+r.width,r.y+r.height]},PathSegment::Line {to:[r.x,r.y+r.height]},PathSegment::Close,
        ].get(index).cloned()),
        () if shape.shape_kind.eq_str("line")=>shape.line.as_ref().and_then(|l|[PathSegment::Move {to:[l.x1,l.y1]},PathSegment::Line {to:[l.x2,l.y2]}].get(index).cloned()),
        () if shape.shape_kind.eq_str("polygon")=>shape.polygon.as_ref().and_then(|p|p.points.get(index).map(|to|if index==0 {PathSegment::Move {to:*to}}else{PathSegment::Line {to:*to}}).or_else(||(!p.points.is_empty()&&index==p.points.len()).then_some(PathSegment::Close))),
        () if shape.shape_kind.eq_str("ellipse")=>shape.ellipse.as_ref().and_then(|e|ellipse_path_segments(e.cx,e.cy,e.rx,e.ry).get(index).cloned()),
        () if shape.shape_kind.eq_str("circle")=>shape.circle.as_ref().and_then(|c|ellipse_path_segments(c.cx,c.cy,c.r,c.r).get(index).cloned()),
        _=>None,
    }
}

fn shape_to_path_segments(shape:&DrawingShapeBody)->Vec<PathSegment> {
    let mut index=0;
    std::iter::from_fn(||{let segment=shape_path_segment(shape,index);index+=1;segment}).collect()
}

pub fn drawing_layer_world_bounds(layer: &DrawingLayerNode) -> Option<(f64, f64, f64, f64)> {
    drawing_layer_bounds_with_parent(layer,[1.0,0.0,0.0,1.0,0.0,0.0])
}

/// 🌍️ Measures complete geometry in the coordinate system of its ancestor matrix.
pub fn drawing_layer_bounds_with_parent(layer: &DrawingLayerNode, parent: [f64;6]) -> Option<(f64,f64,f64,f64)> {
    let matrix = crate::multiply(parent, drawing_transform_to_matrix(&layer_base(layer).transform));
    if let DrawingLayerNode::Group(group) = layer {
        return group.children.iter().filter_map(|child| drawing_layer_bounds_with_parent(child, matrix)).reduce(|a, b| {
            let x = a.0.min(b.0);
            let y = a.1.min(b.1);
            (x, y, (a.0+a.2).max(b.0+b.2)-x, (a.1+a.3).max(b.1+b.3)-y)
        });
    }
    let rectangle = match layer {
        DrawingLayerNode::Text(text) => {
            let [width, height] = semio_framework_2d::text::drawing_text_fallback_extent(&text.content, text.size);
            Some((text.x, text.y, width.max(8.0), height.max(8.0)))
        },
        DrawingLayerNode::Image(image) => Some((0.0, 0.0, image.width, image.height)),
        _ => None,
    };
    let segments = if let Some((x, y, w, h)) = rectangle {
        vec![PathSegment::Move { to: [x,y] }, PathSegment::Line { to: [x+w,y] }, PathSegment::Line { to: [x+w,y+h] }, PathSegment::Line { to: [x,y+h] }, PathSegment::Close]
    } else { layer_to_path_segments(layer) };
    path_segments_bounds_with_matrix(&segments, matrix)
}

fn segment_to_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

pub fn canvas_layer_records(doc: &DrawingSnapshot) -> Vec<DrawingCanvasLayerRecord> {
    flatten_drawing_layers(&doc.layers)
        .into_iter()
        .filter(|layer| !matches!(layer, DrawingLayerNode::Group(_)))
        .map(|layer| {
            let base = layer_base(layer);
            let bounds = drawing_layer_world_bounds(layer);
            DrawingCanvasLayerRecord { id: base.id.to_string_owner(), kind: layer_kind_label(layer), name: base.name.to_string_owner(), x: bounds.map(|b| b.0), y: bounds.map(|b| b.1), width: bounds.map(|b| b.2), height: bounds.map(|b| b.3) }
        })
        .collect()
}

fn drawing_map_point_by_matrix(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

pub fn transform_path_segments(segments: &[PathSegment], transform: &DrawingTransform) -> Vec<PathSegment> {
    transform_path_by_matrix(segments, drawing_transform_to_matrix(transform))
}

pub fn transform_path_by_matrix(segments: &[PathSegment], matrix: [f64; 6]) -> Vec<PathSegment> {
    flatten_curve_segments(segments)
        .iter()
        .map(|segment| match segment {
            PathSegment::Move { to } => PathSegment::Move { to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Line { to } => PathSegment::Line { to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: drawing_map_point_by_matrix(matrix, *ctrl), to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: drawing_map_point_by_matrix(matrix, *ctrl1), ctrl2: drawing_map_point_by_matrix(matrix, *ctrl2), to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Close => PathSegment::Close,
        })
        .collect()
}

pub fn scale_path_segments(segments: &[PathSegment], scale_x: f64, scale_y: f64) -> Vec<PathSegment> {
    if scale_x == 1.0 && scale_y == 1.0 {
        return segments.to_vec();
    }
    transform_path_segments(segments, &DrawingTransform { x: 0.0, y: 0.0, scale_x, scale_y, rotation: 0.0, shear: 0.0 })
}

pub fn split_path_segments_by_contour(segments: &[PathSegment]) -> Vec<Vec<PathSegment>> {
    let mut contours = Vec::new();
    let mut current: Vec<PathSegment> = Vec::new();
    for segment in segments {
        if matches!(segment, PathSegment::Move { .. }) && !current.is_empty() {
            contours.push(std::mem::take(&mut current));
        }
        current.push(segment.clone());
    }
    if !current.is_empty() {
        contours.push(current);
    }
    if contours.is_empty() {
        contours.push(Vec::new());
    }
    contours
}

pub fn path_segments_bounds(segments: &[PathSegment]) -> Option<(f64, f64, f64, f64)> {
    path_segments_bounds_with_matrix(segments,[1.0,0.0,0.0,1.0,0.0,0.0])
}

pub fn path_segments_bounds_with_matrix(segments: &[PathSegment], matrix: [f64;6]) -> Option<(f64, f64, f64, f64)> {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut current = [0.0; 2];
    let mut start = current;
    for segment in segments {
        if matches!(segment, PathSegment::Close) && !min[0].is_finite() { continue; }
        let bounds = crate::segment_bounds(segment,current,start,matrix);
        match segment {
            PathSegment::Move { to } => { current=*to; start=*to; }
            PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => current=*to,
            PathSegment::Close => current=start,
        }
        for axis in 0..2 { min[axis] = min[axis].min(bounds[axis]); max[axis] = max[axis].max(bounds[axis]+bounds[axis+2]); }
    }
    min.iter().all(|n| n.is_finite()).then_some((min[0], min[1], max[0]-min[0], max[1]-min[1]))
}

pub fn filter_path_segments_by_contour_area(segments: &[PathSegment], min_area: f64) -> Vec<PathSegment> {
    if min_area <= 0.0 {
        return segments.to_vec();
    }
    let mut kept = Vec::new();
    for contour in split_path_segments_by_contour(segments) {
        let Some((_, _, width, height)) = path_segments_bounds(&contour) else { continue };
        if width * height < min_area {
            continue;
        }
        kept.extend(contour);
    }
    kept
}

fn arc_ellipse_point(unit: [f64; 2], rx: f64, ry: f64, cos_phi: f64, sin_phi: f64, cx: f64, cy: f64) -> [f64; 2] {
    let x = unit[0] * rx;
    let y = unit[1] * ry;
    [cos_phi * x - sin_phi * y + cx, sin_phi * x + cos_phi * y + cy]
}

fn arc_approx_unit_arc(ang1: f64, ang2: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
    let a = (4.0 / 3.0) * ((ang2 - ang1) / 4.0).tan();
    let (sin1, cos1) = ang1.sin_cos();
    let (sin2, cos2) = ang2.sin_cos();
    ([cos1 - sin1 * a, sin1 + cos1 * a], [cos2 + sin2 * a, sin2 - cos2 * a], [cos2, sin2])
}

/// 🌙️ Converts one SVG endpoint-parameterized arc into cubic Bézier control triples (SVG spec F.6.5).
fn arc_segment_to_cubics(from: [f64; 2], rx: f64, ry: f64, rotation_deg: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<([f64; 2], [f64; 2], [f64; 2])> {
    let Some(arc) = crate::arc_geometry(from, [rx.abs(),ry.abs()], rotation_deg, large_arc, sweep, to) else { return Vec::new() };
    let [rx, ry] = arc.radii;
    let [cx, cy] = arc.center;
    let (sin_phi, cos_phi) = arc.rotation.sin_cos();
    let ang1 = arc.start;
    let ang2 = arc.sweep;
    let mut ratio = ang2.abs() / std::f64::consts::FRAC_PI_2;
    if (1.0 - ratio).abs() < 1e-7 {
        ratio = 1.0;
    }
    let segment_count = ratio.ceil().max(1.0) as usize;
    let delta = ang2 / segment_count as f64;
    let mut cubics = Vec::with_capacity(segment_count);
    let mut angle = ang1;
    for _ in 0..segment_count {
        let (unit_ctrl1, unit_ctrl2, unit_to) = arc_approx_unit_arc(angle, angle + delta);
        cubics.push((arc_ellipse_point(unit_ctrl1, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_ctrl2, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_to, rx, ry, cos_phi, sin_phi, cx, cy)));
        angle += delta;
    }
    cubics
}

/// 🌙️ Flattens `Arc` segments into `Cubic` runs so downstream consumers (booleans, canvas hosts) never see SVG endpoint arcs.
pub fn flatten_curve_segments(segments: &[PathSegment]) -> Vec<PathSegment> {
    let mut out = Vec::with_capacity(segments.len());
    let mut cursor = [0.0, 0.0];
    for segment in segments {
        match segment {
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                let cubics = arc_segment_to_cubics(cursor, *rx, *ry, *rotation, *large_arc, *sweep, *to);
                if cubics.is_empty() {
                    out.push(PathSegment::Line { to: *to });
                } else {
                    for (ctrl1, ctrl2, point) in cubics {
                        out.push(PathSegment::Cubic { ctrl1, ctrl2, to: point });
                    }
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}

const CURVE_LINE_SAMPLE_STEPS: usize = 16;

fn sample_quad_points(from: [f64; 2], ctrl: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [mt * mt * from[0] + 2.0 * mt * t * ctrl[0] + t * t * to[0], mt * mt * from[1] + 2.0 * mt * t * ctrl[1] + t * t * to[1]]
        })
        .collect()
}

fn sample_cubic_points(from: [f64; 2], ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [mt * mt * mt * from[0] + 3.0 * mt * mt * t * ctrl1[0] + 3.0 * mt * t * t * ctrl2[0] + t * t * t * to[0], mt * mt * mt * from[1] + 3.0 * mt * mt * t * ctrl1[1] + 3.0 * mt * t * t * ctrl2[1] + t * t * t * to[1]]
        })
        .collect()
}

/// 📏️ Flattens `Quad`/`Cubic`/`Arc` segments into `Line` segments — the planar boolean kernel only understands polygons.
pub fn flatten_segments_to_lines(segments: &[PathSegment]) -> Vec<PathSegment> {
    let curved = flatten_curve_segments(segments);
    let mut out = Vec::with_capacity(curved.len());
    let mut cursor = [0.0, 0.0];
    for segment in &curved {
        match segment {
            PathSegment::Quad { ctrl, to } => {
                for point in sample_quad_points(cursor, *ctrl, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                for point in sample_cubic_points(cursor, *ctrl1, *ctrl2, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}

pub fn to_kernel_segment(segment: &PathSegment) -> semio_framework_2d::PathSegment {
    use semio_framework_2d::PathSegment as KernelSegment;
    match segment {
        PathSegment::Move { to } => KernelSegment::Move { to: *to },
        PathSegment::Line { to } => KernelSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => KernelSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => KernelSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => KernelSegment::Close,
    }
}

pub fn from_kernel_segment(segment: &semio_framework_2d::PathSegment) -> PathSegment {
    use semio_framework_2d::PathSegment as KernelSegment;
    match segment {
        KernelSegment::Move { to } => PathSegment::Move { to: *to },
        KernelSegment::Line { to } => PathSegment::Line { to: *to },
        KernelSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        KernelSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        KernelSegment::Close => PathSegment::Close,
    }
}

pub fn to_kernel_segments(segments: &[PathSegment]) -> Vec<semio_framework_2d::PathSegment> {
    segments.iter().map(to_kernel_segment).collect()
}

pub fn from_kernel_segments(segments: &[semio_framework_2d::PathSegment]) -> Vec<PathSegment> {
    segments.iter().map(from_kernel_segment).collect()
}

/// 📐️ Premigration artboard resolution: explicit artboard wins, else layer bounds excluding group/boolean/trace kinds.
pub fn resolve_drawing_artboard(doc: &DrawingSnapshot) -> Option<DrawingArtboard> {
    if let Some(artboard) = &doc.artboard {
        if artboard.width > 0.0 && artboard.height > 0.0 {
            return Some(artboard.clone());
        }
    }
    let mut max_x = 0.0_f64;
    let mut max_y = 0.0_f64;
    for layer in flatten_drawing_layers(&doc.layers) {
        if matches!(layer, DrawingLayerNode::Trace(_) | DrawingLayerNode::Boolean(_) | DrawingLayerNode::Group(_)) {
            continue;
        }
        if let Some((x, y, width, height)) = drawing_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    if max_x <= 0.0 || max_y <= 0.0 {
        return None;
    }
    Some(DrawingArtboard { width: max_x, height: max_y })
}

/// 🧮️ The identity affine matrix `[a, b, c, d, e, f]`.
pub const DRAWING_IDENTITY_MATRIX: [f64; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// 🌍️ One addressed layer of `base` in document order: its parent chain's world matrix, whether it and every ancestor is
/// visible and unlocked, and the nearest addressed ancestor, if any.
pub struct DrawingPlacedLayer<'a> {
    pub layer: &'a DrawingLayerNode,
    pub parent: [f64; 6],
    pub editable: bool,
    pub addressed_ancestor: Option<&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
}

/// 🗂️ Every layer of `base` whose id is in `ids`, in document (pre-)order, placed in the world.
pub fn drawing_placed_layers<'a, T: semio_framework_value::paged::Utf8Text>(base: &'a DrawingSnapshot, ids: &[T]) -> Vec<DrawingPlacedLayer<'a>> {
    let mut placed = Vec::new();
    let mut stack: Vec<(semio_framework_value::list::PagedIter<'a, DrawingLayerNode, {usize::MAX}>, [f64; 6], bool, Option<&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>>)> = vec![(base.layers.iter(), DRAWING_IDENTITY_MATRIX, true, None)];
    while let Some((layers, parent, editable, ancestor)) = stack.last_mut() {
        let Some(layer) = layers.next() else {
            stack.pop();
            continue;
        };
        let (parent, ancestor) = (*parent, *ancestor);
        let layer_base = layer_base(layer);
        let editable = *editable && layer_base.visible && !layer_base.locked;
        let addressed = ids.iter().any(|id| layer_base.id.eq_text(id));
        if addressed {
            placed.push(DrawingPlacedLayer { layer, parent, editable, addressed_ancestor: ancestor });
        }
        if let DrawingLayerNode::Group(group) = layer {
            let matrix = crate::multiply(parent, semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix(&layer_base.transform));
            stack.push((group.children.iter(), matrix, editable, if addressed { Some(&layer_base.id) } else { ancestor }));
        }
    }
    placed
}

/// 🚨️ The schema-stated target invariant every selection transform shares: at least one id, none repeated.
pub fn drawing_targets_invariant(targets: &semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>) -> Result<(), &'static str> {
    if targets.is_empty() {
        return Err("a selection transform addresses at least one layer");
    }
    let unique: std::collections::BTreeSet<_> = targets.iter().collect();
    if unique.len() != targets.len() {
        return Err("a selection transform addresses every layer once");
    }
    Ok(())
}

/// 🚨️ The `mutation.partial` warnings of a layer selection transform: addressed ids missing from the drawing, locked or hidden,
/// or placed through a singular transform — each skipped, never fatal.
pub fn drawing_selection_partial(targets: &semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>, placed: &[DrawingPlacedLayer<'_>], locked: Vec<String>, singular: Vec<String>) -> Vec<protocol::MutationMessage> {
    let missing: Vec<String> = targets.iter().filter(|id| !placed.iter().any(|entry| &layer_base(entry.layer).id == *id)).map(|id| id.to_string_owner()).collect();
    [(missing, "not in this drawing"), (locked, "locked or hidden"), (singular, "placed through a singular transform")]
        .into_iter()
        .filter(|(skipped, _)| !skipped.is_empty())
        .map(|(skipped, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", skipped.len(), targets.len(), skipped.join(", "))).at(skipped))
        .collect()
}

/// ↔️ A layer's transform after its world-space drag by `delta` through its parent's world matrix — only the origin moves.
pub fn drawing_dragged_transform(source: &semio_s_artifact_draw_drawing_model::DrawingTransform, parent: [f64; 6], delta: [f64; 2]) -> Option<semio_s_artifact_draw_drawing_model::DrawingTransform> {
    let [x, y, scale_x, scale_y, rotation] = crate::translation::translate([source.x, source.y, source.scale_x, source.scale_y, source.rotation], parent, delta)?;
    Some(semio_s_artifact_draw_drawing_model::DrawingTransform { x, y, scale_x, scale_y, rotation, shear: source.shear })
}

/// 🌐️ A layer's transform after the world-space affine `motion` through its parent's world matrix:
/// `parent⁻¹ · motion · parent · local`.
pub fn drawing_moved_transform(source: &semio_s_artifact_draw_drawing_model::DrawingTransform, parent: [f64; 6], motion: [f64; 6]) -> Option<semio_s_artifact_draw_drawing_model::DrawingTransform> {
    use crate::{inverse, multiply};
    let local = multiply(inverse(parent)?, multiply(motion, multiply(parent, semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix(source))));
    local.iter().all(|value| value.is_finite()).then(|| semio_s_artifact_draw_drawing_model::affine::drawing_matrix_to_transform(local))
}

/// 🔄️ The world-space rotation by `angle` radians about `(pivot_x, pivot_y)`.
pub fn drawing_rotation_matrix(pivot_x: f64, pivot_y: f64, angle: f64) -> [f64; 6] {
    let (s, c) = angle.sin_cos();
    [c, s, -s, c, pivot_x - c * pivot_x + s * pivot_y, pivot_y - s * pivot_x - c * pivot_y]
}

/// 📐️ The world-space scaling by `(scale_x, scale_y)` about `(pivot_x, pivot_y)`.
pub fn drawing_scaling_matrix(pivot_x: f64, pivot_y: f64, scale_x: f64, scale_y: f64) -> [f64; 6] {
    [scale_x, 0.0, 0.0, scale_y, pivot_x * (1.0 - scale_x), pivot_y * (1.0 - scale_y)]
}
