//! 🧱️ Kernel impl of the drawing kernel: the `DrawingKernel` implementation of the drawing store. See [`super`].

use super::*;

// #region 🔖️KernelImpl
impl DrawingKernel for DrawingStore {
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Rect, DrawingNode::Rect { x, y, width, height })
    }

    fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Ellipse, DrawingNode::Ellipse { cx, cy, rx, ry })
    }

    fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Circle, DrawingNode::Circle { cx, cy, r })
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Line, DrawingNode::Line { x1, y1, x2, y2 })
    }

    fn polygon(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if points.len() < 3 {
            return Err(semio_framework_2d::DrawingError::InvalidInput("polygon needs at least 3 points".into()));
        }
        self.register(DrawingKind::Polygon, DrawingNode::Polygon { points: points.to_vec() })
    }

    fn polyline_path(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if points.len() < 2 {
            return Err(semio_framework_2d::DrawingError::InvalidInput("polyline needs at least 2 points".into()));
        }
        self.register(DrawingKind::Path, DrawingNode::Path { segments: polyline_segments(points) })
    }

    fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Path, DrawingNode::Path { segments: rect_segments(x, y, width, height) })
    }

    fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.fill = Some(fill))
    }

    fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.stroke = Some(stroke))
    }

    fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.set_fill(handle, FillStyle::LinearGradient { x1, y1, x2, y2, stops: stops.to_vec() })
    }

    fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::translate(dx, dy)))
    }

    fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::rotate(angle)))
    }

    fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::scale(sx, sy)))
    }

    fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if children.is_empty() {
            return Err(semio_framework_2d::DrawingError::InvalidInput("group needs children".into()));
        }
        let handles: Vec<String> = children.iter().map(|h| h.as_str().to_string()).collect();
        self.register(DrawingKind::Group, DrawingNode::Group { children: handles })
    }

    fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "union")
    }

    fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "difference")
    }

    fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "intersection")
    }

    fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "xor")
    }

    fn bool_op_many(&mut self, operation: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if handles.is_empty() {
            return Err(semio_framework_2d::DrawingError::InvalidInput("boolean operation needs at least one handle".into()));
        }
        if handles.len() == 1 {
            return self.fork(&handles[0]);
        }
        let mut segments_list: Vec<Vec<semio_framework_2d::PathSegment>> = Vec::new();
        for handle in handles {
            segments_list.push(DrawingStore::node_to_segments(&self.entry(handle)?.node));
        }
        let merged = semio_framework_2d::booleans::boolean_paths_many(&segments_list, operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }

    fn boolean_segments(&self, a: &[semio_framework_2d::PathSegment], b: &[semio_framework_2d::PathSegment], operation: &str) -> Result<Vec<semio_framework_2d::PathSegment>, semio_framework_2d::DrawingError> {
        semio_framework_2d::booleans::boolean_paths(a, b, operation)
    }

    fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let segments = semio_framework_2d::trace::trace_bitmap_paths(width, height, mask_or_luma, threshold, simplify_epsilon)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments })
    }

    fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Text, DrawingNode::Text { x, y, content: content.to_string(), size })
    }

    fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let clip_segments = DrawingStore::node_to_segments(&self.entry(clip)?.node);
        self.with_mutated(target, |entry| entry.clip = Some(clip_segments))
    }

    fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, semio_framework_2d::DrawingError> {
        self.flatten_scene_sync(handle)
    }

    fn export_svg(&self, handle: &DrawingHandle) -> Result<String, semio_framework_2d::DrawingError> {
        self.export_svg_sync(handle)
    }

    fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        self.export_pdf_sync(handle)
    }


    fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, semio_framework_2d::DrawingError> {
        Ok(self.entry(handle)?.kind)
    }

    fn dispose(&mut self, handle: &DrawingHandle) {
        self.dispose_sync(handle);
    }

    fn retain_sync(&mut self, live: &HashSet<String>) {
        DrawingStore::retain_sync(self, live);
    }
}

impl DrawingStore {
    fn bool_operation(&mut self, a: &DrawingHandle, b: &DrawingHandle, operation: &str) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let a_segments = DrawingStore::node_to_segments(&self.entry(a)?.node);
        let b_segments = DrawingStore::node_to_segments(&self.entry(b)?.node);
        let merged = semio_framework_2d::booleans::boolean_paths(&a_segments, &b_segments, operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }
}
// #endregion 🔖️KernelImpl
