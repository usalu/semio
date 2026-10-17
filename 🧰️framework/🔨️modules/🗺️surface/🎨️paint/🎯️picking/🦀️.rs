//! 🎯️ Picking of surface paint: the screen rectangle, the pick and query wire records and the raster host hit-testing and marquee operations. See [`super`].

use super::*;

// #region 🔖️Picking
/// 📐️ Axis-aligned screen/world rect, used for both hit-testing and bounds accumulation.
#[derive(Clone, Copy, Debug)]
pub(super) struct ScreenRect {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
}

impl ScreenRect {
    fn from_points(points: &[Point]) -> Self {
        let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        Self { x: min_x, y: min_y, width: max_x - min_x, height: max_y - min_y }
    }

    pub(super) fn union(acc: Option<Self>, next: Self) -> Self {
        match acc {
            None => next,
            Some(a) => {
                let min_x = a.x.min(next.x);
                let min_y = a.y.min(next.y);
                let max_x = (a.x + a.width).max(next.x + next.width);
                let max_y = (a.y + a.height).max(next.y + next.height);
                Self { x: min_x, y: min_y, width: max_x - min_x, height: max_y - min_y }
            }
        }
    }

    pub(super) fn contains(&self, inner: &ScreenRect) -> bool {
        inner.x >= self.x && inner.y >= self.y && inner.x + inner.width <= self.x + self.width && inner.y + inner.height <= self.y + self.height
    }

    pub(super) fn intersects(&self, other: &ScreenRect) -> bool {
        self.x <= other.x + other.width && self.x + self.width >= other.x && self.y <= other.y + other.height && self.y + self.height >= other.y
    }

    pub(super) fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[derive(serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct PickTargetJson {
    domain: String,
    pub(super) id: String,
    generality: u8,
}

#[derive(serde::Deserialize, semio_framework_value::FromValue)]
struct ScreenPointIn {
    pub(super) x: f64,
    pub(super) y: f64,
}

#[derive(serde::Deserialize, semio_framework_value::FromValue)]
struct MarqueeQueryIn {
    pub(super) points: Vec<ScreenPointIn>,
    crossing: bool,
}

#[derive(serde::Deserialize, semio_framework_value::FromValue)]
pub(super) struct CameraJsonIn {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) zoom: f64,
}

#[derive(serde::Deserialize, semio_framework_value::FromValue)]
struct ViewportJsonIn {
    pub(super) width: f64,
    pub(super) height: f64,
}

/// 🎯️ Flattened pick candidate — mirrors premigration `flattenRasterLayers` (document order, parent pushed before children, no visibility cascade).
enum PickEntry {
    Pixel { id: String, visible: bool, parent: Affine, transform: Affine, width: u32, height: u32, ancestors: Vec<(String, bool)> },
    Group { id: String, visible: bool, parent: Affine, children: Vec<LayerNode> },
}

impl RasterHost {
    fn pixel_screen_bounds(&self, parent: Affine, transform: &Affine, width: u32, height: u32) -> ScreenRect {
        let world = camera::camera_content_affine(&self.camera, &self.viewport) * parent * (*transform);
        let hw = width as f64 * 0.5;
        let hh = height as f64 * 0.5;
        let corners = [world * Point::new(-hw, -hh), world * Point::new(hw, -hh), world * Point::new(hw, hh), world * Point::new(-hw, hh)];
        ScreenRect::from_points(&corners)
    }

    /// 🎯️ Bounding box of a group's visible pixel descendants — port of premigration `rasterGroupScreenBounds`.
    fn group_screen_bounds(&self, parent: Affine, children: &[LayerNode]) -> Option<ScreenRect> {
        let mut acc: Option<ScreenRect> = None;
        for child in children {
            match child {
                LayerNode::Pixel { visible, transform, width, height, .. } => {
                    if !*visible {
                        continue;
                    }
                    acc = Some(ScreenRect::union(acc, self.pixel_screen_bounds(parent, transform, *width, *height)));
                }
                LayerNode::Group { transform, children, .. } => {
                    if let Some(bounds) = self.group_screen_bounds(parent * (*transform), children) {
                        acc = Some(ScreenRect::union(acc, bounds));
                    }
                }
                LayerNode::Adjustment { .. } => {}
            }
        }
        acc
    }

    fn flatten_pick_targets(&self) -> Vec<PickEntry> {
        fn walk(nodes: &[LayerNode], parent: Affine, ancestors: &[(String, bool)], out: &mut Vec<PickEntry>) {
            for node in nodes {
                match node {
                    LayerNode::Pixel { id, visible, transform, width, height, .. } => {
                        out.push(PickEntry::Pixel { id: id.clone(), visible: *visible, parent, transform: *transform, width: *width, height: *height, ancestors: ancestors.to_vec() });
                    }
                    LayerNode::Group { id, visible, transform, children, .. } => {
                        out.push(PickEntry::Group { id: id.clone(), visible: *visible, parent, children: children.clone() });
                        let mut next_ancestors = ancestors.to_vec();
                        next_ancestors.push((id.clone(), *visible));
                        walk(children, parent * (*transform), &next_ancestors, out);
                    }
                    LayerNode::Adjustment { .. } => {}
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.document.layers, Affine::IDENTITY, &[], &mut out);
        out
    }

    /// 🎯️ Stacked pick targets at a screen point, topmost first — port of premigration `resolveRasterPickTargetsAtScreenPoint`.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let entries = self.flatten_pick_targets();
        let mut hits: Vec<PickTargetJson> = Vec::new();
        for entry in entries.iter().rev() {
            match entry {
                PickEntry::Group { id, visible, parent, children } => {
                    if !*visible {
                        continue;
                    }
                    if let Some(bounds) = self.group_screen_bounds(*parent, children) {
                        if bounds.contains_point(sx, sy) && !hits.iter().any(|h| &h.id == id) {
                            hits.push(PickTargetJson { domain: "group".into(), id: id.clone(), generality: 0 });
                        }
                    }
                }
                PickEntry::Pixel { id, visible, parent, transform, width, height, ancestors } => {
                    if !*visible {
                        continue;
                    }
                    let bounds = self.pixel_screen_bounds(*parent, transform, *width, *height);
                    if !bounds.contains_point(sx, sy) {
                        continue;
                    }
                    if !hits.iter().any(|h| &h.id == id) {
                        hits.push(PickTargetJson { domain: "pixel".into(), id: id.clone(), generality: 2 });
                    }
                    for (group_id, group_visible) in ancestors {
                        if *group_visible && !hits.iter().any(|h| &h.id == group_id) {
                            hits.push(PickTargetJson { domain: "group".into(), id: group_id.clone(), generality: 0 });
                        }
                    }
                }
            }
        }
        serde_json::to_string(&hits).unwrap_or_else(|_| "[]".into())
    }

    /// 🖱️ Pixel layer ids hit by a screen-space marquee (rect or lasso bbox) — port of premigration `resolveRasterMarqueeLayerHits`.
    pub fn marquee_hits_json(&self, query_json: &str) -> Result<String, FrameworkSurfacePaintError> {
        let query: MarqueeQueryIn = serde_json::from_str(query_json)?;
        if query.points.len() < 2 {
            return Ok("[]".into());
        }
        let points: Vec<Point> = query.points.iter().map(|p| Point::new(p.x, p.y)).collect();
        let marquee = ScreenRect::from_points(&points);
        let mut hits = Vec::new();
        for entry in self.flatten_pick_targets() {
            if let PickEntry::Pixel { id, visible, parent, transform, width, height, .. } = entry {
                if !visible {
                    continue;
                }
                let bounds = self.pixel_screen_bounds(parent, &transform, width, height);
                let hit = if query.crossing { marquee.intersects(&bounds) } else { marquee.contains(&bounds) };
                if hit {
                    hits.push(id);
                }
            }
        }
        Ok(serde_json::to_string(&hits).unwrap_or_else(|_| "[]".into()))
    }

    /// 📐️ World-space bounds of visible pixel layers (own + ancestor transforms, no camera) — port of premigration `resolveRasterDocumentWorldBounds`.
    fn document_world_bounds(&self) -> Option<ScreenRect> {
        fn walk(nodes: &[LayerNode], parent: Affine, acc: &mut Option<ScreenRect>) {
            for node in nodes {
                match node {
                    LayerNode::Pixel { visible, transform, width, height, .. } => {
                        if !*visible {
                            continue;
                        }
                        let world = parent * (*transform);
                        let hw = *width as f64 * 0.5;
                        let hh = *height as f64 * 0.5;
                        let corners = [world * Point::new(-hw, -hh), world * Point::new(hw, -hh), world * Point::new(hw, hh), world * Point::new(-hw, hh)];
                        *acc = Some(ScreenRect::union(*acc, ScreenRect::from_points(&corners)));
                    }
                    LayerNode::Group { transform, children, .. } => walk(children, parent * (*transform), acc),
                    LayerNode::Adjustment { .. } => {}
                }
            }
        }
        let mut acc: Option<ScreenRect> = None;
        walk(&self.document.layers, Affine::IDENTITY, &mut acc);
        acc
    }

    /// 🧭️ Fits a camera to document content — port of premigration `rasterNavigatorFitCamera`. Falls back to the current camera when the document has no visible pixel content.
    pub fn navigator_fit_camera_json(&self, viewport_w: f64, viewport_h: f64) -> String {
        let padding = 24.0;
        let (x, y, zoom) = match self.document_world_bounds() {
            None => (self.camera.x, self.camera.y, self.camera.zoom),
            Some(bounds) => {
                let content_w = bounds.width.max(1.0);
                let content_h = bounds.height.max(1.0);
                let inner_w = (viewport_w.max(1.0) - padding * 2.0).max(1.0);
                let inner_h = (viewport_h.max(1.0) - padding * 2.0).max(1.0);
                let zoom = camera::clamp_zoom((inner_w / content_w).min(inner_h / content_h));
                (bounds.x + bounds.width * 0.5, bounds.y + bounds.height * 0.5, zoom)
            }
        };
        serde_json::json!({ "x": x, "y": y, "zoom": zoom }).to_string()
    }

    /// 🧭️ Maps the composite viewport into navigator screen space for the overview overlay rectangle — port of premigration `rasterNavigatorViewportOverlay`. `self.camera`/`self.viewport` act as the navigator's own camera/viewport.
    pub fn navigator_viewport_overlay_json(&self, content_camera_json: &str, content_viewport_json: &str) -> Result<String, FrameworkSurfacePaintError> {
        let content_camera: CameraJsonIn = serde_json::from_str(content_camera_json)?;
        let content_viewport: ViewportJsonIn = serde_json::from_str(content_viewport_json)?;
        let cc = Camera { x: content_camera.x, y: content_camera.y, zoom: camera::clamp_zoom(content_camera.zoom) };
        let cv = Viewport { width: (content_viewport.width.max(1.0)) as u32, height: (content_viewport.height.max(1.0)) as u32, dpr: 1.0 };
        let top_left_world = camera::screen_to_world(&cc, &cv, Point::new(0.0, 0.0));
        let bottom_right_world = camera::screen_to_world(&cc, &cv, Point::new(cv.width as f64, cv.height as f64));
        let top_left = camera::world_to_screen(&self.camera, &self.viewport, top_left_world);
        let bottom_right = camera::world_to_screen(&self.camera, &self.viewport, bottom_right_world);
        let rect = ScreenRect::from_points(&[top_left, bottom_right]);
        Ok(serde_json::json!({ "x": rect.x, "y": rect.y, "width": rect.width, "height": rect.height }).to_string())
    }
}
// #endregion 🔖️Picking
