//! 🗄️ Store and geometry of the drawing kernel: the drawing store with its node operations plus the primitive segment builders and the scene bounds. See [`super`].

use super::*;

// #region 🔖️Store
/// 🗄️ [`DrawingKernel`] using engine derive through the WIT `engine-derive`/`engine-read` guest↔host
/// boundary (process-local [`semio_framework_2d_compute::EngineCache`]) — flow's own ephemeral node-evaluation kernel,
/// not a persisted artifact store.
pub struct DrawingStore {
    cache: semio_framework_2d_compute::EngineCache,
    pub live: HashSet<String>,
}

impl Default for DrawingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawingStore {
    pub fn new() -> Self {
        let mut cache = semio_framework_2d_compute::EngineCache::new(ENGINE_CACHE_BUDGET_BYTES);
        cache.register(DrawingEngine);
        Self { cache, live: HashSet::new() }
    }

    /// 🧩 Attach to an existing host-owned cache (registers [`DrawingEngine`]).
    pub fn with_engine_cache(mut cache: semio_framework_2d_compute::EngineCache) -> Self {
        cache.register(DrawingEngine);
        Self { cache, live: HashSet::new() }
    }

    /// 🧠 Mutable engine cache backing the WIT `engine-derive`/`engine-read` guest↔host boundary.
    pub fn engine_cache_mut(&mut self) -> &mut semio_framework_2d_compute::EngineCache {
        &mut self.cache
    }

    fn derive_node(&mut self, node: &StoredNode) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let pack = StoredNode::encode_pack(node).map_err(semio_framework_2d::DrawingError::InvalidInput)?;
        let compute_handle = self.cache.derive(DrawingEngine::ENGINE_ID, &pack).map_err(map_engine_fault)?;
        let drawing = drawing_handle_from_key(compute_handle.key);
        self.live.insert(drawing.as_str().to_string());
        Ok(drawing)
    }

    pub(super) fn register(&mut self, kind: DrawingKind, node: DrawingNode) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.derive_node(&StoredNode { kind, node, transform: Affine2D::identity(), fill: None, stroke: None, clip: None, opacity: 1.0 })
    }

    pub(super) fn fork(&mut self, source: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let entry = self.entry(source)?;
        self.derive_node(&entry)
    }

    pub(super) fn with_mutated<F>(&mut self, source: &DrawingHandle, mutate: F) -> Result<DrawingHandle, semio_framework_2d::DrawingError>
    where
        F: FnOnce(&mut StoredNode),
    {
        let mut entry = self.entry(source)?;
        mutate(&mut entry);
        self.derive_node(&entry)
    }

    pub fn entry(&self, handle: &DrawingHandle) -> Result<StoredNode, semio_framework_2d::DrawingError> {
        if !self.live.contains(handle.as_str()) {
            return Err(semio_framework_2d::DrawingError::MissingHandle(handle.as_str().to_string()));
        }
        let compute_handle = compute_handle_for_drawing(handle)?;
        let pack = self.cache.read(&compute_handle).map_err(map_engine_fault)?;
        StoredNode::decode_pack(&pack).map_err(semio_framework_2d::DrawingError::InvalidInput)
    }

    pub fn registry_len(&self) -> usize {
        self.live.len()
    }

    pub fn dispose_sync(&mut self, handle: &DrawingHandle) {
        self.live.remove(handle.as_str());
    }

    pub fn retain_sync(&mut self, live: &HashSet<String>) {
        self.live.retain(|handle| live.contains(handle));
    }

    pub(super) fn node_to_segments(node: &DrawingNode) -> Vec<semio_framework_2d::PathSegment> {
        match node {
            DrawingNode::Rect { x, y, width, height } => rect_segments(*x, *y, *width, *height),
            DrawingNode::Ellipse { cx, cy, rx, ry } => ellipse_segments(*cx, *cy, *rx, *ry),
            DrawingNode::Circle { cx, cy, r } => ellipse_segments(*cx, *cy, *r, *r),
            DrawingNode::Line { x1, y1, x2, y2 } => vec![semio_framework_2d::PathSegment::Move { to: [*x1, *y1] }, semio_framework_2d::PathSegment::Line { to: [*x2, *y2] }],
            DrawingNode::Polygon { points } => polygon_segments(points),
            DrawingNode::Path { segments } => segments.clone(),
            DrawingNode::Text { .. } => Vec::new(),
            DrawingNode::Group { .. } => Vec::new(),
        }
    }

    fn flatten_handle(&self, handle: &DrawingHandle, parent: Affine2D) -> Result<Vec<SceneNode>, semio_framework_2d::DrawingError> {
        let entry = self.entry(handle)?;
        let transform = parent.multiply(entry.transform);
        match &entry.node {
            DrawingNode::Group { children } => children.iter().try_fold(Vec::new(), |mut acc, child| {
                let nested = self.flatten_handle(&DrawingHandle(child.clone()), transform)?;
                acc.extend(nested);
                Ok::<_, semio_framework_2d::DrawingError>(acc)
            }),
            _ => Ok(vec![SceneNode { transform, node: entry.node.clone(), fill: entry.fill.clone(), stroke: entry.stroke.clone(), opacity: entry.opacity, clip: entry.clip.clone() }]),
        }
    }

    pub fn flatten_scene_sync(&self, handle: &DrawingHandle) -> Result<DrawingScene, semio_framework_2d::DrawingError> {
        let nodes = self.flatten_handle(handle, Affine2D::identity())?;
        let (width, height) = scene_bounds(&nodes);
        Ok(DrawingScene { width, height, nodes })
    }

    pub fn export_svg_sync(&self, handle: &DrawingHandle) -> Result<String, semio_framework_2d::DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_svg(&scene))
    }

    pub fn export_pdf_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_pdf(&scene))
    }


}
// #endregion 🔖️Store

// #region 🔖️Geometry
pub(super) fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
    vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close]
}

fn polygon_segments(points: &[semio_framework_2d::Vec2]) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
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

fn ellipse_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
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

pub(super) fn polyline_segments(points: &[semio_framework_2d::Vec2]) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
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
