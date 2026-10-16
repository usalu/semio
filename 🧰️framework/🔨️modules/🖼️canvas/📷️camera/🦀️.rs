//! 📷️ Canvas camera: the pan-and-zoom transform between world and screen space, the opening-camera fit of a content extent, and the level-of-detail bands zoom selects.

// #region 🔖️Camera
pub mod camera {
    use geometry::{Affine, Point};

    pub const CANVAS_CAMERA_ZOOM_MIN: f64 = ui_styling::metrics::camera::ZOOM_MIN;
    pub const CANVAS_CAMERA_ZOOM_MAX: f64 = ui_styling::metrics::camera::ZOOM_MAX;

    #[derive(Clone, Debug)]
    pub struct Camera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Viewport {
        pub width: u32,
        pub height: u32,
        pub dpr: f64,
    }

    impl Default for Viewport {
        fn default() -> Self {
            Self { width: 1, height: 1, dpr: 1.0 }
        }
    }

    impl Viewport {
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
        }

        pub fn physical_size(&self) -> (u32, u32) {
            let pw = ((self.width as f64 * self.dpr).round() as u32).max(1);
            let ph = ((self.height as f64 * self.dpr).round() as u32).max(1);
            (pw, ph)
        }
    }

    pub fn clamp_zoom(zoom: f64) -> f64 {
        zoom.clamp(CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX)
    }

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - camera.x) * camera.zoom + viewport.width as f64 / 2.0, (p.y - camera.y) * camera.zoom + viewport.height as f64 / 2.0)
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - viewport.width as f64 / 2.0) / camera.zoom + camera.x, (p.y - viewport.height as f64 / 2.0) / camera.zoom + camera.y)
    }

    pub fn camera_content_affine(camera: &Camera, viewport: &Viewport) -> Affine {
        let z = camera.zoom;
        Affine::new([z, 0.0, 0.0, z, viewport.width as f64 * 0.5 - camera.x * z, viewport.height as f64 * 0.5 - camera.y * z])
    }

    pub fn wheel_screen(camera: &mut Camera, viewport: &Viewport, sx: f64, sy: f64, delta_y: f64) {
        let zoom_factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let next_zoom = clamp_zoom(camera.zoom * zoom_factor);
        let screen = Point::new(sx, sy);
        let world_before = screen_to_world(camera, viewport, screen);
        camera.x = world_before.x - (sx - viewport.width as f64 / 2.0) / next_zoom;
        camera.y = world_before.y - (sy - viewport.height as f64 / 2.0) / next_zoom;
        camera.zoom = next_zoom;
    }

    // #region 🔖️ContentFraming
    /// 🖼️ Screen margin a fit leaves around the content it frames, in viewport pixels per side.
    pub const CONTENT_FIT_PADDING_PX: f64 = ui_styling::metrics::camera::CONTENT_FIT_PADDING_PX;

    /// 🖼️ How much of the content a STORED camera has to already show for that camera to be adopted
    /// on a first attach instead of being replaced by a fit.
    pub const CONTENT_FRAMED_MIN_COVERAGE: f64 = ui_styling::metrics::camera::CONTENT_FRAMED_MIN_COVERAGE;

    /// 🖼️ How little of the content has to be left on screen for a graph that CHANGED under a live
    /// camera (an example switch) to be re-fitted. Deliberately far below
    /// {@link CONTENT_FRAMED_MIN_COVERAGE}: a camera the viewer panned themselves is never yanked
    /// back just because a new node landed off screen.
    pub const CONTENT_REFIT_MAX_COVERAGE: f64 = ui_styling::metrics::camera::CONTENT_REFIT_MAX_COVERAGE;

    /// 🖼️ Axis-aligned world bounds of everything a board wants framed.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ContentBounds {
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
    }

    impl ContentBounds {
        pub fn width(&self) -> f64 {
            (self.max_x - self.min_x).max(0.0)
        }

        pub fn height(&self) -> f64 {
            (self.max_y - self.min_y).max(0.0)
        }

        pub fn center(&self) -> (f64, f64) {
            ((self.min_x + self.max_x) * 0.5, (self.min_y + self.max_y) * 0.5)
        }
    }

    fn view_bounds(camera: &Camera, viewport: &Viewport) -> ContentBounds {
        let zoom = camera.zoom.max(1e-9);
        let half_w = viewport.width.max(1) as f64 / (2.0 * zoom);
        let half_h = viewport.height.max(1) as f64 / (2.0 * zoom);
        ContentBounds { min_x: camera.x - half_w, min_y: camera.y - half_h, max_x: camera.x + half_w, max_y: camera.y + half_h }
    }

    /// 🖼️ Fraction (`0.0..=1.0`) of `content`'s own area a `camera` currently shows. Degenerate
    /// content (a single node, a single row) is measured on whichever axes have extent, so a
    /// zero-height graph is not reported as invisible.
    pub fn content_coverage(content: &ContentBounds, camera: &Camera, viewport: &Viewport) -> f64 {
        let view = view_bounds(camera, viewport);
        let overlap = |a0: f64, a1: f64, b0: f64, b1: f64| (a1.min(b1) - a0.max(b0)).max(0.0);
        let (cw, ch) = (content.width(), content.height());
        let ox = overlap(content.min_x, content.max_x, view.min_x, view.max_x);
        let oy = overlap(content.min_y, content.max_y, view.min_y, view.max_y);
        let inside_x = content.min_x >= view.min_x && content.max_x <= view.max_x;
        let inside_y = content.min_y >= view.min_y && content.max_y <= view.max_y;
        match (cw > 0.0, ch > 0.0) {
            (true, true) => (ox * oy) / (cw * ch),
            (true, false) => f64::from(u8::from(inside_y)) * (ox / cw),
            (false, true) => f64::from(u8::from(inside_x)) * (oy / ch),
            (false, false) => f64::from(u8::from(inside_x && inside_y)),
        }
    }

    /// 🖼️ The camera that frames the whole of `content` inside `viewport` with `padding_px` of
    /// screen margin per side. Zoom is clamped to the canvas camera range, so content far larger
    /// than the zoom-out limit is centred rather than silently over-zoomed.
    pub fn fit_camera(content: &ContentBounds, viewport: &Viewport, padding_px: f64) -> Camera {
        let (x, y) = content.center();
        let vw = (viewport.width.max(1) as f64 - padding_px * 2.0).max(1.0);
        let vh = (viewport.height.max(1) as f64 - padding_px * 2.0).max(1.0);
        let (cw, ch) = (content.width(), content.height());
        let zoom_x = if cw > 0.0 { vw / cw } else { f64::INFINITY };
        let zoom_y = if ch > 0.0 { vh / ch } else { f64::INFINITY };
        let zoom = zoom_x.min(zoom_y);
        Camera { x, y, zoom: clamp_zoom(if zoom.is_finite() { zoom } else { 1.0 }) }
    }

    /// 🖼️ The camera a surface opens on. A stored camera is honoured only when it already frames the
    /// content it is stored for (`>= min_coverage` of the content's area on screen); otherwise — and
    /// whenever there is no stored camera at all — the content is fitted. Returns `true` in the
    /// second component when the fit won, which is what the caller persists as a viewport gesture.
    ///
    /// 🔍️ A non-positive stored zoom is not a camera: it collapses every world point onto one screen
    /// pixel. With no content to fit against there is nothing to correct it, so it answers the
    /// identity camera instead — a plugin that publishes `{0, 0, 0}` for an unset viewport used to
    /// leave the live generation3d flow graph blank forever (every node and handle reported
    /// `visible: false` around the panel centre).
    pub fn startup_camera(stored: Option<&Camera>, content: Option<&ContentBounds>, viewport: &Viewport, padding_px: f64, min_coverage: f64) -> (Camera, bool) {
        let Some(content) = content else {
            return (stored.filter(|camera| camera.zoom > 0.0).cloned().unwrap_or_default(), false);
        };
        match stored {
            Some(camera) if camera.zoom > 0.0 && content_coverage(content, camera, viewport) >= min_coverage => (camera.clone(), false),
            _ => (fit_camera(content, viewport, padding_px), true),
        }
    }
    // #endregion 🔖️ContentFraming
}
// #endregion 🔖️Camera

// #region 🔖️Lod
pub mod lod {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Lod {
        pub id: &'static str,
        pub name: &'static str,
        pub description: &'static str,
        pub max_zoom: f64,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LodScale {
        pub lods: &'static [Lod],
    }

    impl LodScale {
        pub fn resolve_index(self, zoom: f64) -> usize {
            for (i, lod) in self.lods.iter().enumerate() {
                if zoom < lod.max_zoom {
                    return i;
                }
            }
            self.lods.len().saturating_sub(1)
        }

        pub fn resolve(self, zoom: f64) -> &'static Lod {
            &self.lods[self.resolve_index(zoom)]
        }

        pub fn index_of(self, id: &str) -> Option<usize> {
            self.lods.iter().position(|lod| lod.id == id)
        }
    }

    /// 🔤️ Fixed screen label px for a LOD band; stays constant while zooming inside the band.
    pub fn band_label_screen_px(band_px: &[f64], band_index: usize, fallback: f64) -> f64 {
        band_px.get(band_index).copied().unwrap_or(fallback)
    }

    /// 🔤️ Lower camera-zoom bound for a LOD band (previous band `max_zoom`, or `zoom_min`).
    pub fn band_floor_zoom(band_floor_zoom: &[f64], band_index: usize, zoom_min: f64) -> f64 {
        band_floor_zoom.get(band_index).copied().unwrap_or(zoom_min).max(zoom_min)
    }

    /// 🔤️ Label screen px scaled with camera zoom inside one LOD band so text keeps the same proportion to world geometry.
    pub fn lod_band_label_screen_px(base_screen_px: f64, zoom: f64, band_floor_zoom: f64) -> f64 {
        let z = zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        let floor = band_floor_zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        base_screen_px * z / floor
    }
}
// #endregion 🔖️Lod

// #region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/📷️camera-fit/🦀️.rs"]
mod camera_fit_laws;

#[cfg(test)]
#[path = "../🧪️tests/🧭️camera-unit/🦀️.rs"]
mod unit_tests;
// #endregion 🧪️Tests
