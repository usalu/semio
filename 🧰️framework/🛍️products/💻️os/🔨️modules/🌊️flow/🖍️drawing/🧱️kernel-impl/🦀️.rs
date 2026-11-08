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
        children.iter().try_for_each(|child| self.entry(child).map(drop))?;
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
        let merged = boolean_path_operands(&segments_list.iter().map(Vec::as_slice).collect::<Vec<_>>(), operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }

    fn boolean_segments(&self, a: &[semio_framework_2d::PathSegment], b: &[semio_framework_2d::PathSegment], operation: &str) -> Result<Vec<semio_framework_2d::PathSegment>, semio_framework_2d::DrawingError> {
        boolean_path_operands(&[a, b], operation)
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
        let merged = boolean_path_operands(&[&a_segments, &b_segments], operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }
}
// #endregion 🔖️KernelImpl

/// 🎯️ Drives one bounded planar boolean over closed linear operands to completion and drains its physical owners.
fn boolean_path_operands(inputs: &[&[semio_framework_2d::PathSegment]], operation: &str) -> Result<Vec<semio_framework_2d::PathSegment>, semio_framework_2d::DrawingError> {
    use semio_framework_2d::booleans::paths::{PathBooleanInput, PathBooleanJob, PathBooleanOperand};
    use semio_framework_2d::booleans::{BooleanFillRule, BooleanOperation};
    use semio_framework_2d::{DrawingError, PathSegment};
    use semio_framework_value::{work::RetainedWorkLedger, RetainedCloneGrant};
    const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 25, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
    let operation = BooleanOperation::parse(operation).map_err(|error| DrawingError::InvalidInput(error.to_string()))?;
    if inputs.is_empty() || inputs.len() > 1024 {
        return Err(DrawingError::InvalidInput("Boolean needs one to 1024 operands".into()));
    }
    let magnitude = inputs.iter().flat_map(|path| path.iter()).fold(0.0f64, |m, segment| match segment {
        PathSegment::Move { to } | PathSegment::Line { to } => m.max(to[0].abs()).max(to[1].abs()),
        _ => m,
    });
    let operands = inputs.iter().map(|path| PathBooleanOperand { segments: path.to_vec(), transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], tolerance: 0.01, fill_rule: BooleanFillRule::Nonzero }).collect();
    let mut job = PathBooleanJob::new(PathBooleanInput { operation, operands, epsilon: 1e-8f64.max(magnitude * f64::EPSILON * 32.0), max_edges: 65536, max_parameters: 262144, max_atomic_edges: 65536, max_segments: 65536, max_work: 100000000 });
    let outcome = (|| loop {
        let mut allocation = None;
        let progress = job.advance(4096, &mut RetainedWorkLedger::new(STOCK), &mut allocation).map_err(|error| DrawingError::Operation(error.to_string()))?;
        if progress.done {
            return job.result().map(<[PathSegment]>::to_vec).map_err(|error| DrawingError::Operation(error.to_string()));
        }
        if job.last_turn_work() == 0 {
            return Err(DrawingError::Operation("Boolean made no progress under its ledger".into()));
        }
    })();
    let (mut retired, _) = job.into_retirement();
    fn refused<E>(_: E) -> DrawingError {
        DrawingError::Operation("Boolean retirement refused its physical grant".into())
    }
    let mut wallet = semio_framework_value::AuthoringWallet::new(STOCK, 1 << 48);
    let drained = (|| -> Result<(), DrawingError> {
        while !retired.terminal_is_empty() {
            let mut body = 0;
            let mut quoted = None;
            for _ in 0..8 {
                let demand = retired.next_demand(body).map_err(refused)?;
                if demand.copy_bytes <= body {
                    quoted = Some(demand);
                    break;
                }
                body = demand.copy_bytes;
            }
            let Some(demand) = quoted else {
                return Err(DrawingError::Operation("Boolean retirement quote did not settle".into()));
            };
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: body, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let step = wallet.run_settled(grant, 1 << 24, |effect| retired.close_step(effect)).map_err(refused)?.ok_or_else(|| DrawingError::Operation("Boolean retirement work was not admitted".into()))?.0;
            let work = step.work;
            let step = step.result.map_err(refused)?;
            wallet.post(step.progress()).map_err(refused)?;
            if work == 0 {
                return Err(DrawingError::Operation("Boolean retirement made no progress".into()));
            }
        }
        Ok(())
    })();
    if !retired.terminal_is_empty() {
        std::mem::forget(retired);
    }
    let output = outcome?;
    drained?;
    Ok(output)
}
