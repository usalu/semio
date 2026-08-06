//! 🗄️ In-process [`crate::engine::DrawingKernel`] store with SVG/PDF/DWG export.

use crate::engine::{Affine2D, DrawingError, DrawingHandle, DrawingKernel, DrawingKind, DrawingNode, DrawingScene, FillStyle, GradientStop, PathSegment, SceneNode, StrokeStyle, Vec2};
use async_trait::async_trait;

// #region 🔖️Store
#[derive(Clone)]
struct StoredNode {
    kind: DrawingKind,
    node: DrawingNode,
    transform: Affine2D,
    fill: Option<FillStyle>,
    stroke: Option<StrokeStyle>,
    clip: Option<Vec<PathSegment>>,
    opacity: f64,
}

/// 🗄️ Scene-graph drawing store with `drawing-*` handles.
pub struct DrawingStore {
    seq: u32,
    registry: std::collections::HashMap<String, StoredNode>,
}

impl Default for DrawingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawingStore {
    pub fn new() -> Self {
        Self { seq: 0, registry: std::collections::HashMap::new() }
    }

    fn register(&mut self, kind: DrawingKind, node: DrawingNode) -> DrawingHandle {
        self.seq += 1;
        let handle = DrawingHandle::new(kind, self.seq);
        self.registry.insert(handle.as_str().to_string(), StoredNode { kind, node, transform: Affine2D::identity(), fill: None, stroke: None, clip: None, opacity: 1.0 });
        handle
    }

    fn fork(&mut self, source: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        let entry = self.entry(source)?.clone();
        self.seq += 1;
        let handle = DrawingHandle::new(entry.kind, self.seq);
        self.registry.insert(handle.as_str().to_string(), entry);
        Ok(handle)
    }

    fn entry(&self, handle: &DrawingHandle) -> Result<&StoredNode, DrawingError> {
        self.registry.get(handle.as_str()).ok_or_else(|| DrawingError::MissingHandle(handle.as_str().to_string()))
    }

    fn entry_mut(&mut self, handle: &DrawingHandle) -> Result<&mut StoredNode, DrawingError> {
        self.registry.get_mut(handle.as_str()).ok_or_else(|| DrawingError::MissingHandle(handle.as_str().to_string()))
    }

    pub fn registry_len(&self) -> usize {
        self.registry.len()
    }

    pub fn dispose_sync(&mut self, handle: &DrawingHandle) {
        self.registry.remove(handle.as_str());
    }

    pub fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        self.registry.retain(|handle, _| live.contains(handle));
    }

    fn node_to_segments(node: &DrawingNode) -> Vec<PathSegment> {
        match node {
            DrawingNode::Rect { x, y, width, height } => rect_segments(*x, *y, *width, *height),
            DrawingNode::Ellipse { cx, cy, rx, ry } => ellipse_segments(*cx, *cy, *rx, *ry),
            DrawingNode::Circle { cx, cy, r } => ellipse_segments(*cx, *cy, *r, *r),
            DrawingNode::Line { x1, y1, x2, y2 } => vec![PathSegment::Move { to: [*x1, *y1] }, PathSegment::Line { to: [*x2, *y2] }],
            DrawingNode::Polygon { points } => polygon_segments(points),
            DrawingNode::Path { segments } => segments.clone(),
            DrawingNode::Text { .. } => Vec::new(),
            DrawingNode::Group { .. } => Vec::new(),
        }
    }

    fn flatten_handle(&self, handle: &DrawingHandle, parent: Affine2D) -> Result<Vec<SceneNode>, DrawingError> {
        let entry = self.entry(handle)?;
        let transform = parent.multiply(entry.transform);
        match &entry.node {
            DrawingNode::Group { children } => children.iter().try_fold(Vec::new(), |mut acc, child| {
                let nested = self.flatten_handle(&DrawingHandle(child.clone()), transform)?;
                acc.extend(nested);
                Ok::<_, DrawingError>(acc)
            }),
            _ => Ok(vec![SceneNode { transform, node: entry.node.clone(), fill: entry.fill.clone(), stroke: entry.stroke.clone(), opacity: entry.opacity, clip: entry.clip.clone() }]),
        }
    }

    pub fn flatten_scene_sync(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError> {
        let nodes = self.flatten_handle(handle, Affine2D::identity())?;
        let (width, height) = scene_bounds(&nodes);
        Ok(DrawingScene { width, height, nodes })
    }

    pub fn export_svg_sync(&self, handle: &DrawingHandle) -> Result<String, DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_svg(&scene))
    }

    pub fn export_pdf_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_pdf(&scene))
    }

    pub fn export_dwg_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        let mut drawing = semio_framework_core::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        for node in &scene.nodes {
            if node.opacity <= 0.0 {
                continue;
            }
            if let DrawingNode::Circle { cx, cy, r } = &node.node {
                let center = affine_apply_point(node.transform, [*cx, *cy]);
                drawing.entities.push(semio_framework_core::DwgEntity {
                    layer,
                    color: semio_framework_core::DwgColor::ByLayer,
                    geometry: semio_framework_core::DwgGeometry::Circle { center: [center[0], center[1], 0.0], radius: r * node.transform.0[0].abs(), normal: [0.0, 0.0, 1.0] },
                });
                continue;
            }
            if let DrawingNode::Text { x, y, content, size } = &node.node {
                let at = affine_apply_point(node.transform, [*x, *y]);
                drawing.entities.push(semio_framework_core::DwgEntity {
                    layer,
                    color: semio_framework_core::DwgColor::ByLayer,
                    geometry: semio_framework_core::DwgGeometry::Text { at: [at[0], at[1], 0.0], height: *size, rotation: 0.0, content: content.clone() },
                });
                continue;
            }
            if let Some(segments) = scene_node_world_segments(node) {
                let dwg_segments: Vec<semio_framework_core::DwgPathSegment> = segments.iter().map(engine_segment_to_dwg).collect();
                let mut sub = semio_framework_core::paths_to_dwg_drawing(&[dwg_segments]);
                drawing.entities.append(&mut sub.entities);
            }
        }
        semio_framework_core::dwg_to_bytes(&drawing).map_err(DrawingError::InvalidInput)
    }

    pub fn import_dwg_sync(&mut self, data: &[u8]) -> Result<DrawingHandle, DrawingError> {
        let drawing = semio_framework_core::dwg_from_bytes(data).map_err(DrawingError::InvalidInput)?;
        let mut children = Vec::new();
        for path in semio_framework_core::dwg_drawing_to_paths(&drawing) {
            let segments: Vec<PathSegment> = path.iter().map(dwg_segment_to_engine).collect();
            if segments.len() < 2 {
                continue;
            }
            children.push(self.register(DrawingKind::Path, DrawingNode::Path { segments }));
        }
        for entity in &drawing.entities {
            if let semio_framework_core::DwgGeometry::Text { at, height, content, .. } = &entity.geometry {
                children.push(self.register(DrawingKind::Text, DrawingNode::Text { x: at[0], y: at[1], content: content.clone(), size: *height }));
            }
        }
        if children.is_empty() {
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [0.0, 0.0] }] }));
        }
        if children.len() == 1 {
            return Ok(children.into_iter().next().expect("children.len() == 1 checked above"));
        }
        Ok(self.register(DrawingKind::Group, DrawingNode::Group { children: children.iter().map(|h| h.as_str().to_string()).collect() }))
    }
}
// #endregion 🔖️Store

fn affine_apply_point(m: Affine2D, p: Vec2) -> Vec2 {
    let a = m.0;
    [a[0] * p[0] + a[2] * p[1] + a[4], a[1] * p[0] + a[3] * p[1] + a[5]]
}

fn scene_node_world_segments(node: &SceneNode) -> Option<Vec<PathSegment>> {
    let segments = match &node.node {
        DrawingNode::Path { segments } => segments.clone(),
        DrawingNode::Line { x1, y1, x2, y2 } => vec![PathSegment::Move { to: [*x1, *y1] }, PathSegment::Line { to: [*x2, *y2] }],
        DrawingNode::Polygon { points } => {
            if points.is_empty() {
                return None;
            }
            let mut segments = vec![PathSegment::Move { to: points[0] }];
            for p in &points[1..] {
                segments.push(PathSegment::Line { to: *p });
            }
            segments.push(PathSegment::Close);
            segments
        }
        DrawingNode::Rect { x, y, width, height } => {
            vec![PathSegment::Move { to: [*x, *y] }, PathSegment::Line { to: [*x + *width, *y] }, PathSegment::Line { to: [*x + *width, *y + *height] }, PathSegment::Line { to: [*x, *y + *height] }, PathSegment::Close]
        }
        DrawingNode::Ellipse { cx, cy, rx, ry } => vec![
            PathSegment::Move { to: [*cx + *rx, *cy] },
            PathSegment::Arc { rx: *rx, ry: *ry, rotation: 0.0, large_arc: true, sweep: true, to: [*cx - *rx, *cy] },
            PathSegment::Arc { rx: *rx, ry: *ry, rotation: 0.0, large_arc: true, sweep: true, to: [*cx + *rx, *cy] },
            PathSegment::Close,
        ],
        DrawingNode::Circle { .. } | DrawingNode::Text { .. } | DrawingNode::Group { .. } => return None,
    };
    Some(
        segments
            .into_iter()
            .map(|segment| match segment {
                PathSegment::Move { to } => PathSegment::Move { to: affine_apply_point(node.transform, to) },
                PathSegment::Line { to } => PathSegment::Line { to: affine_apply_point(node.transform, to) },
                PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: affine_apply_point(node.transform, ctrl), to: affine_apply_point(node.transform, to) },
                PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: affine_apply_point(node.transform, ctrl1), ctrl2: affine_apply_point(node.transform, ctrl2), to: affine_apply_point(node.transform, to) },
                PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to: affine_apply_point(node.transform, to) },
                PathSegment::Close => PathSegment::Close,
            })
            .collect(),
    )
}

fn engine_segment_to_dwg(segment: &PathSegment) -> semio_framework_core::DwgPathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        PathSegment::Move { to } => DwgPathSegment::Move { to: *to },
        PathSegment::Line { to } => DwgPathSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => DwgPathSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => DwgPathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => DwgPathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => DwgPathSegment::Close,
    }
}

fn dwg_segment_to_engine(segment: &semio_framework_core::DwgPathSegment) -> PathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        DwgPathSegment::Move { to } => PathSegment::Move { to: *to },
        DwgPathSegment::Line { to } => PathSegment::Line { to: *to },
        DwgPathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        DwgPathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        DwgPathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        DwgPathSegment::Close => PathSegment::Close,
    }
}

// #region 🔖️Geometry
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<PathSegment> {
    vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close]
}

fn polygon_segments(points: &[Vec2]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        if points.len() > 2 {
            segments.push(PathSegment::Close);
        }
    }
    segments
}

fn ellipse_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<PathSegment> {
    const K: f64 = 0.5522847498;
    let ox = rx * K;
    let oy = ry * K;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + ox, cy - ry], ctrl2: [cx + rx, cy - oy], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + oy], ctrl2: [cx + ox, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - ox, cy + ry], ctrl2: [cx - rx, cy + oy], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - oy], ctrl2: [cx - ox, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn polyline_segments(points: &[Vec2]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
    }
    segments
}

fn scene_bounds(nodes: &[SceneNode]) -> (f64, f64) {
    let mut max_x = 512.0_f64;
    let mut max_y = 512.0_f64;
    for node in nodes {
        match &node.node {
            DrawingNode::Rect { x, y, width, height } => {
                max_x = max_x.max(x + width);
                max_y = max_y.max(y + height);
            }
            DrawingNode::Circle { cx, cy, r } => {
                max_x = max_x.max(cx + r);
                max_y = max_y.max(cy + r);
            }
            DrawingNode::Ellipse { cx, cy, rx, ry } => {
                max_x = max_x.max(cx + rx);
                max_y = max_y.max(cy + ry);
            }
            DrawingNode::Text { x, y, size, .. } => {
                max_x = max_x.max(x + size * 4.0);
                max_y = max_y.max(y + size);
            }
            _ => {}
        }
    }
    (max_x.max(1.0), max_y.max(1.0))
}
// #endregion 🔖️Geometry

// #region 🔖️Export
fn color_css(color: [f64; 4]) -> String {
    let r = (color[0] * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color[1] * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color[2] * 255.0).round().clamp(0.0, 255.0) as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn segments_to_svg_d(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1])),
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1]));
            }
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn serialize_svg(scene: &DrawingScene) -> String {
    let mut body = String::new();
    for node in &scene.nodes {
        match &node.node {
            DrawingNode::Text { x, y, content, size } => {
                let [tx, ty] = node.transform.transform_point([*x, *y]);
                let fill = node.fill.as_ref().map_or_else(
                    || "black".into(),
                    |f| match f {
                        FillStyle::Solid { color } => color_css(*color),
                        _ => "black".into(),
                    },
                );
                body.push_str(&format!(r#"<text x="{tx}" y="{ty}" font-size="{size}" fill="{fill}">{content}</text>"#));
            }
            _ => {
                let segments = DrawingStore::node_to_segments(&node.node);
                if segments.is_empty() {
                    continue;
                }
                let d = segments_to_svg_d(&segments);
                let mut attrs = format!(r#"d="{d}""#);
                if let Some(fill) = &node.fill {
                    match fill {
                        FillStyle::Solid { color } => attrs.push_str(&format!(r#" fill="{}""#, color_css(*color))),
                        FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                            let id = format!("lg{}", body.len());
                            body.push_str(&format!(r#"<defs><linearGradient id="{id}" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</linearGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                        FillStyle::RadialGradient { cx, cy, r, stops } => {
                            let id = format!("rg{}", body.len());
                            body.push_str(&format!(r#"<defs><radialGradient id="{id}" cx="{cx}" cy="{cy}" r="{r}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</radialGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                    }
                } else {
                    attrs.push_str(r#" fill="none""#);
                }
                if let Some(stroke) = &node.stroke {
                    attrs.push_str(&format!(r#" stroke="{}" stroke-width="{}""#, color_css(stroke.color), stroke.width));
                }
                body.push_str(&format!("<path {attrs}/>"));
            }
        }
    }
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{body}</svg>"#, scene.width, scene.height, scene.width, scene.height)
}

fn segments_to_pdf_operations(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("{} {} m\n", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("{} {} l\n", to[0], to[1])),
            PathSegment::Close => out.push_str("h\n"),
            _ => {}
        }
    }
    out
}

fn serialize_pdf(scene: &DrawingScene) -> Vec<u8> {
    let mut stream = String::new();
    stream.push_str("0.1 0.1 0.1 rg\n");
    for node in &scene.nodes {
        let segments = DrawingStore::node_to_segments(&node.node);
        if segments.is_empty() {
            continue;
        }
        stream.push_str(&segments_to_pdf_operations(&segments));
        if node.fill.is_some() {
            stream.push_str("f\n");
        } else if node.stroke.is_some() {
            stream.push_str("S\n");
        }
    }
    let content = stream.as_bytes();
    let objects = [
        "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        &format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources << >> >>\nendobj\n", scene.width, scene.height),
        &format!("4 0 obj\n<< /Length {} >>\nstream\n", content.len()),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for object in &objects[..3] {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    offsets.push(pdf.len());
    pdf.push_str(objects[3]);
    pdf.push_str(std::str::from_utf8(content).unwrap_or(""));
    pdf.push_str("\nendstream\nendobj\n");
    let xref = pdf.len();
    pdf.push_str("xref\n0 5\n0000000000 65535 f \n");
    for offset in offsets {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"));
    pdf.into_bytes()
}
// #endregion 🔖️Export

// #region 🔖️Kernel
#[async_trait(?Send)]
impl DrawingKernel for DrawingStore {
    async fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Rect, DrawingNode::Rect { x, y, width, height }))
    }

    async fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Ellipse, DrawingNode::Ellipse { cx, cy, rx, ry }))
    }

    async fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Circle, DrawingNode::Circle { cx, cy, r }))
    }

    async fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Line, DrawingNode::Line { x1, y1, x2, y2 }))
    }

    async fn polygon(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError> {
        if points.len() < 3 {
            return Err(DrawingError::InvalidInput("polygon needs at least 3 points".into()));
        }
        Ok(self.register(DrawingKind::Polygon, DrawingNode::Polygon { points: points.to_vec() }))
    }

    async fn polyline_path(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError> {
        if points.len() < 2 {
            return Err(DrawingError::InvalidInput("polyline needs at least 2 points".into()));
        }
        Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: polyline_segments(points) }))
    }

    async fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: rect_segments(x, y, width, height) }))
    }

    async fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        self.entry_mut(&next)?.fill = Some(fill);
        Ok(next)
    }

    async fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        self.entry_mut(&next)?.stroke = Some(stroke);
        Ok(next)
    }

    async fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, DrawingError> {
        self.set_fill(handle, FillStyle::LinearGradient { x1, y1, x2, y2, stops: stops.to_vec() }).await
    }

    async fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::translate(dx, dy));
        Ok(next)
    }

    async fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::rotate(angle));
        Ok(next)
    }

    async fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, DrawingError> {
        let next = self.fork(handle)?;
        let transform = self.entry(&next)?.transform;
        self.entry_mut(&next)?.transform = transform.multiply(Affine2D::scale(sx, sy));
        Ok(next)
    }

    async fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError> {
        if children.is_empty() {
            return Err(DrawingError::InvalidInput("group needs children".into()));
        }
        let handles: Vec<String> = children.iter().map(|h| h.as_str().to_string()).collect();
        Ok(self.register(DrawingKind::Group, DrawingNode::Group { children: handles }))
    }

    async fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_operation(a, b, "union").await
    }

    async fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_operation(a, b, "difference").await
    }

    async fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_operation(a, b, "intersection").await
    }

    async fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        self.bool_operation(a, b, "xor").await
    }

    async fn bool_op_many(&mut self, operation: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError> {
        if handles.is_empty() {
            return Err(DrawingError::InvalidInput("boolean operation needs at least one handle".into()));
        }
        if handles.len() == 1 {
            return self.fork(&handles[0]);
        }
        let mut segments_list: Vec<Vec<PathSegment>> = Vec::new();
        for handle in handles {
            segments_list.push(DrawingStore::node_to_segments(&self.entry(handle)?.node));
        }
        #[cfg(feature = "booleans")]
        {
            let merged = crate::booleans::boolean_paths_many(&segments_list, operation)?;
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: merged }));
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (segments_list, operation);
            Err(DrawingError::Operation("boolean operations require booleans feature".into()))
        }
    }

    async fn boolean_segments(&self, a: &[PathSegment], b: &[PathSegment], operation: &str) -> Result<Vec<PathSegment>, DrawingError> {
        #[cfg(feature = "booleans")]
        {
            return crate::booleans::boolean_paths(a, b, operation);
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (a, b, operation);
            Err(DrawingError::Operation("boolean operations require booleans feature".into()))
        }
    }

    async fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, DrawingError> {
        #[cfg(feature = "trace")]
        {
            let segments = crate::trace::trace_bitmap_paths(width, height, mask_or_luma, threshold, simplify_epsilon)?;
            return Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments }));
        }
        #[cfg(not(feature = "trace"))]
        {
            let _ = (width, height, mask_or_luma, threshold, simplify_epsilon);
            Err(DrawingError::Operation("trace requires trace feature".into()))
        }
    }

    async fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, DrawingError> {
        Ok(self.register(DrawingKind::Text, DrawingNode::Text { x, y, content: content.to_string(), size }))
    }

    async fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, DrawingError> {
        let clip_segments = DrawingStore::node_to_segments(&self.entry(clip)?.node);
        let next = self.fork(target)?;
        self.entry_mut(&next)?.clip = Some(clip_segments);
        Ok(next)
    }

    async fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError> {
        self.flatten_scene_sync(handle)
    }

    async fn export_svg(&self, handle: &DrawingHandle) -> Result<String, DrawingError> {
        self.export_svg_sync(handle)
    }

    async fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        self.export_pdf_sync(handle)
    }

    async fn export_dwg(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError> {
        self.export_dwg_sync(handle)
    }

    async fn import_dwg(&mut self, data: &[u8]) -> Result<DrawingHandle, DrawingError> {
        self.import_dwg_sync(data)
    }

    async fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, DrawingError> {
        Ok(self.entry(handle)?.kind)
    }

    async fn dispose(&mut self, handle: &DrawingHandle) {
        self.dispose_sync(handle);
    }

    fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        DrawingStore::retain_sync(self, live);
    }
}

impl DrawingStore {
    async fn bool_operation(&mut self, a: &DrawingHandle, b: &DrawingHandle, operation: &str) -> Result<DrawingHandle, DrawingError> {
        let a_segments = DrawingStore::node_to_segments(&self.entry(a)?.node);
        let b_segments = DrawingStore::node_to_segments(&self.entry(b)?.node);
        #[cfg(feature = "booleans")]
        {
            let merged = crate::booleans::boolean_paths(&a_segments, &b_segments, operation)?;
            Ok(self.register(DrawingKind::Path, DrawingNode::Path { segments: merged }))
        }
        #[cfg(not(feature = "booleans"))]
        {
            let _ = (a_segments, b_segments, operation);
            Err(DrawingError::Operation("boolean operations require booleans feature".into()))
        }
    }
}
// #endregion 🔖️Kernel

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{block_on, LineCap, LineJoin};

    #[test]
    fn rect_exports_svg() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 10.0, 20.0)).expect("rect");
        let svg = store.export_svg_sync(&rect).expect("svg");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("10"));
    }

    #[test]
    fn rect_exports_pdf() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 10.0, 20.0)).expect("rect");
        let pdf = store.export_pdf_sync(&rect).expect("pdf");
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    fn group_flattens_children() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let b = block_on(store.circle(10.0, 10.0, 3.0)).unwrap();
        let group = block_on(store.group(&[a, b])).unwrap();
        let scene = store.flatten_scene_sync(&group).unwrap();
        assert_eq!(scene.nodes.len(), 2);
    }

    #[test]
    fn dwg_export_import_round_trips_a_group() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect_path(0.0, 0.0, 5.0, 5.0)).unwrap();
        let circle = block_on(store.circle(10.0, 10.0, 3.0)).unwrap();
        let group = block_on(store.group(&[rect, circle])).unwrap();

        let bytes = store.export_dwg_sync(&group).expect("export dwg");
        assert!(!bytes.is_empty());

        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        let scene = store.flatten_scene_sync(&imported).expect("flatten imported scene");
        assert!(!scene.nodes.is_empty());
    }

    // #region Geometry primitives export
    #[test]
    fn ellipse_exports_svg_with_cubic_curves() {
        let mut store = DrawingStore::new();
        let ellipse = block_on(store.ellipse(5.0, 5.0, 4.0, 2.0)).unwrap();
        let svg = store.export_svg_sync(&ellipse).expect("svg");
        assert!(svg.contains("C "));
        assert!(svg.contains("Z"));
    }

    #[test]
    fn line_exports_svg_move_and_line() {
        let mut store = DrawingStore::new();
        let line = block_on(store.line(0.0, 0.0, 10.0, 10.0)).unwrap();
        let svg = store.export_svg_sync(&line).expect("svg");
        assert!(svg.contains("M 0 0"));
        assert!(svg.contains("L 10 10"));
    }

    #[test]
    fn polygon_exports_closed_svg_path() {
        let mut store = DrawingStore::new();
        let polygon = block_on(store.polygon(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]])).unwrap();
        let svg = store.export_svg_sync(&polygon).expect("svg");
        assert!(svg.contains("Z"));
    }

    #[test]
    fn polyline_path_exports_open_path_without_close() {
        let mut store = DrawingStore::new();
        let polyline = block_on(store.polyline_path(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]])).unwrap();
        let svg = store.export_svg_sync(&polyline).expect("svg");
        assert!(!svg.contains("Z"));
    }

    #[test]
    fn polygon_errors_on_too_few_points() {
        let mut store = DrawingStore::new();
        let err = block_on(store.polygon(&[[0.0, 0.0], [1.0, 1.0]])).unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }

    #[test]
    fn polyline_path_errors_on_too_few_points() {
        let mut store = DrawingStore::new();
        let err = block_on(store.polyline_path(&[[0.0, 0.0]])).unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }
    // #endregion Geometry primitives export

    // #region Style
    #[test]
    fn set_fill_solid_renders_opaque_hex_color() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let filled = block_on(store.set_fill(&rect, FillStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] })).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains(r##"fill="#ff0000""##));
    }

    #[test]
    fn set_fill_with_alpha_renders_rgba() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let filled = block_on(store.set_fill(&rect, FillStyle::Solid { color: [0.0, 1.0, 0.0, 0.5] })).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("rgba(0,255,0,0.500)"));
    }

    #[test]
    fn linear_gradient_fill_renders_gradient_defs() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 1.0] }];
        let filled = block_on(store.linear_gradient_fill(&rect, 0.0, 0.0, 5.0, 5.0, &stops)).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("fill=\"url(#lg"));
    }

    #[test]
    fn set_fill_radial_gradient_renders_defs() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }];
        let fill = FillStyle::RadialGradient { cx: 2.5, cy: 2.5, r: 2.0, stops };
        let filled = block_on(store.set_fill(&rect, fill)).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("<radialGradient"));
        assert!(svg.contains("fill=\"url(#rg"));
    }

    #[test]
    fn set_stroke_renders_stroke_attributes() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let stroke = StrokeStyle { color: [0.0, 0.0, 1.0, 1.0], width: 2.0, cap: LineCap::Round, join: LineJoin::Round, dash: vec![] };
        let stroked = block_on(store.set_stroke(&rect, stroke)).unwrap();
        let svg = store.export_svg_sync(&stroked).expect("svg");
        assert!(svg.contains(r##"stroke="#0000ff""##));
        assert!(svg.contains(r#"stroke-width="2""#));
    }
    // #endregion Style

    // #region Transforms
    #[test]
    fn translate_moves_exported_geometry() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let moved = block_on(store.translate(&rect, 10.0, 20.0)).unwrap();
        let scene = store.flatten_scene_sync(&moved).unwrap();
        assert_eq!(scene.nodes[0].transform.transform_point([0.0, 0.0]), [10.0, 20.0]);
    }

    #[test]
    fn rotate_and_scale_compose_into_transform() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let rotated = block_on(store.rotate(&rect, std::f64::consts::FRAC_PI_2)).unwrap();
        let scaled = block_on(store.scale(&rotated, 2.0, 2.0)).unwrap();
        let scene = store.flatten_scene_sync(&scaled).unwrap();
        let [x, y] = scene.nodes[0].transform.transform_point([1.0, 0.0]);
        assert!((x - 0.0).abs() < 1e-9);
        assert!((y - 2.0).abs() < 1e-9);
    }
    // #endregion Transforms

    // #region Group and clip
    #[test]
    fn group_errors_on_empty_children() {
        let mut store = DrawingStore::new();
        let err = block_on(store.group(&[])).unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }

    #[test]
    fn apply_clip_stores_clip_segments_on_flatten() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let circle = block_on(store.circle(2.0, 2.0, 1.0)).unwrap();
        let clipped = block_on(store.apply_clip(&rect, &circle)).unwrap();
        let scene = store.flatten_scene_sync(&clipped).unwrap();
        assert!(scene.nodes[0].clip.as_ref().is_some_and(|segments| !segments.is_empty()));
    }
    // #endregion Group and clip

    // #region Text
    #[test]
    fn text_with_fill_renders_colored_text_element() {
        let mut store = DrawingStore::new();
        let text = block_on(store.text(1.0, 2.0, "hi", 12.0)).unwrap();
        let colored = block_on(store.set_fill(&text, FillStyle::Solid { color: [0.0, 0.0, 1.0, 1.0] })).unwrap();
        let svg = store.export_svg_sync(&colored).expect("svg");
        assert!(svg.contains(r##"<text x="1" y="2" font-size="12" fill="#0000ff">hi</text>"##));
    }

    #[test]
    fn text_without_fill_defaults_to_black() {
        let mut store = DrawingStore::new();
        let text = block_on(store.text(0.0, 0.0, "plain", 10.0)).unwrap();
        let svg = store.export_svg_sync(&text).expect("svg");
        assert!(svg.contains(r#"fill="black">plain"#));
    }

    #[test]
    fn text_with_gradient_fill_falls_back_to_black_in_svg() {
        let mut store = DrawingStore::new();
        let text = block_on(store.text(0.0, 0.0, "grad", 10.0)).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }];
        let gradient = block_on(store.linear_gradient_fill(&text, 0.0, 0.0, 1.0, 1.0, &stops)).unwrap();
        let svg = store.export_svg_sync(&gradient).expect("svg");
        assert!(svg.contains(r#"fill="black">grad"#));
    }
    // #endregion Text

    // #region Boolean operations via kernel trait
    #[test]
    fn bool_union_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect_path(0.0, 0.0, 10.0, 10.0)).unwrap();
        let b = block_on(store.rect_path(5.0, 5.0, 10.0, 10.0)).unwrap();
        let merged = block_on(store.bool_union(&a, &b)).unwrap();
        assert_eq!(block_on(store.kind(&merged)).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn bool_difference_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect_path(0.0, 0.0, 10.0, 10.0)).unwrap();
        let b = block_on(store.rect_path(5.0, 5.0, 10.0, 10.0)).unwrap();
        let diff = block_on(store.bool_difference(&a, &b)).unwrap();
        let scene = store.flatten_scene_sync(&diff).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_intersection_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect_path(0.0, 0.0, 10.0, 10.0)).unwrap();
        let b = block_on(store.rect_path(5.0, 5.0, 10.0, 10.0)).unwrap();
        let intersection = block_on(store.bool_intersection(&a, &b)).unwrap();
        let scene = store.flatten_scene_sync(&intersection).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_xor_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect_path(0.0, 0.0, 10.0, 10.0)).unwrap();
        let b = block_on(store.rect_path(5.0, 5.0, 10.0, 10.0)).unwrap();
        let xor = block_on(store.bool_xor(&a, &b)).unwrap();
        let scene = store.flatten_scene_sync(&xor).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_op_many_forks_single_handle() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let forked = block_on(store.bool_op_many("union", std::slice::from_ref(&rect))).unwrap();
        assert_ne!(forked.as_str(), rect.as_str());
        assert_eq!(block_on(store.kind(&forked)).unwrap(), DrawingKind::Rect);
    }

    #[test]
    fn bool_op_many_merges_multiple_handles() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect_path(0.0, 0.0, 10.0, 10.0)).unwrap();
        let b = block_on(store.rect_path(5.0, 0.0, 10.0, 10.0)).unwrap();
        let c = block_on(store.rect_path(0.0, 5.0, 10.0, 10.0)).unwrap();
        let merged = block_on(store.bool_op_many("union", &[a, b, c])).unwrap();
        assert_eq!(block_on(store.kind(&merged)).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn bool_op_many_errors_on_empty_handles() {
        let mut store = DrawingStore::new();
        let err = block_on(store.bool_op_many("union", &[])).unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }

    #[test]
    fn boolean_segments_trait_delegates_to_booleans_module() {
        let store = DrawingStore::new();
        let a = rect_segments(0.0, 0.0, 10.0, 10.0);
        let b = rect_segments(5.0, 5.0, 10.0, 10.0);
        let merged = block_on(store.boolean_segments(&a, &b, "union")).expect("union");
        assert!(!merged.is_empty());
    }
    // #endregion Boolean operations via kernel trait

    // #region Trace via kernel trait
    #[test]
    fn trace_bitmap_trait_delegates_to_trace_module() {
        let mut store = DrawingStore::new();
        let width = 6_u32;
        let height = 6_u32;
        let mut mask = vec![0_u8; (width * height) as usize];
        for y in 1..5 {
            for x in 1..5 {
                mask[(y * width + x) as usize] = 255;
            }
        }
        let traced = block_on(store.trace_bitmap(width, height, &mask, 0.5, 0.5)).unwrap();
        assert_eq!(block_on(store.kind(&traced)).unwrap(), DrawingKind::Path);
    }
    // #endregion Trace via kernel trait

    // #region Registry lifecycle
    #[test]
    fn registry_len_tracks_inserted_handles() {
        let mut store = DrawingStore::new();
        assert_eq!(store.registry_len(), 0);
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        assert_eq!(store.registry_len(), 1);
        block_on(store.set_fill(&rect, FillStyle::Solid { color: [1.0, 1.0, 1.0, 1.0] })).unwrap();
        assert_eq!(store.registry_len(), 2);
    }

    #[test]
    fn dispose_sync_removes_handle_from_registry() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        store.dispose_sync(&rect);
        assert_eq!(store.registry_len(), 0);
        let err = block_on(store.kind(&rect)).unwrap_err();
        assert!(matches!(err, DrawingError::MissingHandle(_)));
    }

    #[test]
    fn retain_sync_keeps_only_live_handles() {
        let mut store = DrawingStore::new();
        let a = block_on(store.rect(0.0, 0.0, 5.0, 5.0)).unwrap();
        let b = block_on(store.circle(1.0, 1.0, 1.0)).unwrap();
        let live: std::collections::HashSet<String> = [a.as_str().to_string()].into_iter().collect();
        store.retain_sync(&live);
        assert!(block_on(store.kind(&a)).is_ok());
        assert!(block_on(store.kind(&b)).is_err());
    }

    #[test]
    fn missing_handle_errors_on_set_fill_and_translate() {
        let mut store = DrawingStore::new();
        let bogus = DrawingHandle("drawing-rect-999".to_string());
        let fill_err = block_on(store.set_fill(&bogus, FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] })).unwrap_err();
        assert!(matches!(fill_err, DrawingError::MissingHandle(_)));
        let translate_err = block_on(store.translate(&bogus, 1.0, 1.0)).unwrap_err();
        assert!(matches!(translate_err, DrawingError::MissingHandle(_)));
    }

    #[test]
    fn flatten_scene_errors_on_missing_handle() {
        let store = DrawingStore::new();
        let bogus = DrawingHandle("drawing-rect-999".to_string());
        let err = block_on(store.flatten_scene(&bogus)).unwrap_err();
        assert!(matches!(err, DrawingError::MissingHandle(_)));
    }
    // #endregion Registry lifecycle

    // #region DWG export/import branches
    #[test]
    fn export_dwg_includes_circle_and_text_entities() {
        let mut store = DrawingStore::new();
        let circle = block_on(store.circle(5.0, 5.0, 3.0)).unwrap();
        let text = block_on(store.text(0.0, 0.0, "hi", 5.0)).unwrap();
        let group = block_on(store.group(&[circle, text])).unwrap();
        let bytes = store.export_dwg_sync(&group).expect("export dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        let scene = store.flatten_scene_sync(&imported).expect("flatten imported scene");
        assert_eq!(scene.nodes.len(), 2);
    }

    #[test]
    fn import_dwg_of_single_path_skips_group_wrapper() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect_path(0.0, 0.0, 5.0, 5.0)).unwrap();
        let bytes = store.export_dwg_sync(&rect).expect("export dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        assert_eq!(block_on(store.kind(&imported)).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn import_dwg_of_empty_drawing_returns_degenerate_path() {
        let mut store = DrawingStore::new();
        let empty = semio_framework_core::DwgDrawing::default();
        let bytes = semio_framework_core::dwg_to_bytes(&empty).expect("encode empty dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import empty dwg");
        assert_eq!(block_on(store.kind(&imported)).unwrap(), DrawingKind::Path);
    }
    // #endregion DWG export/import branches

    // #region Scene bounds
    #[test]
    fn scene_bounds_grows_to_fit_text_and_shapes() {
        let mut store = DrawingStore::new();
        let rect = block_on(store.rect(600.0, 0.0, 10.0, 10.0)).unwrap();
        let text = block_on(store.text(0.0, 700.0, "wide label", 20.0)).unwrap();
        let group = block_on(store.group(&[rect, text])).unwrap();
        let scene = store.flatten_scene_sync(&group).unwrap();
        assert!(scene.width >= 610.0);
        assert!(scene.height >= 720.0);
    }
    // #endregion Scene bounds
}
// #endregion 🔖️Tests
