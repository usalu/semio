//! 🧭️ Web Mercator projection of the tiled map: lon/lat to world space, tile rectangles and cover zoom.
//!
//! Mounted as `pub mod projection` of the tiled-map root; pure math with no host state.

// #region 🔖️Projection
use super::Point;
use super::Rect;

pub const MAX_LAT: f64 = 85.051_128_78;
pub const WORLD_HALF: f64 = 1.0;

pub fn lonlat_to_world(lon: f64, lat: f64) -> Point {
    let lon = lon.clamp(-180.0, 180.0);
    let lat = lat.clamp(-MAX_LAT, MAX_LAT);
    let x = lon / 180.0 * WORLD_HALF;
    let lat_rad = lat.to_radians();
    let y = (0.5 * (std::f64::consts::FRAC_PI_4 + lat_rad * 0.5).tan().ln() / std::f64::consts::PI) * WORLD_HALF * 2.0;
    Point::new(x, y)
}

pub fn world_to_lonlat(x: f64, y: f64) -> (f64, f64) {
    let lon = (x / WORLD_HALF) * 180.0;
    let lat_rad = 2.0 * (y / WORLD_HALF * std::f64::consts::PI).exp().atan() - std::f64::consts::FRAC_PI_2;
    let lat = lat_rad.to_degrees();
    (lon, lat)
}

/// 📐️ Mercator world span in map units (`[-WORLD_HALF, WORLD_HALF]` on each axis).
pub const WORLD_VISIBLE_SPAN: f64 = WORLD_HALF * 2.0;

/// 📐️ Minimum zoom so every viewport pixel maps inside the world (cover, no outscroll).
pub fn cover_zoom_for_viewport(viewport: &canvas::camera::Viewport) -> f64 {
    let vw = viewport.width.max(1) as f64;
    let vh = viewport.height.max(1) as f64;
    vw.max(vh) / WORLD_VISIBLE_SPAN
}

pub fn default_world_camera(viewport: &canvas::camera::Viewport) -> canvas::camera::Camera {
    let zoom = cover_zoom_for_viewport(viewport).max(super::MAP_CAMERA_ZOOM_MIN);
    canvas::camera::Camera { x: 0.0, y: 0.0, zoom }
}

pub fn tile_world_rect(z: u32, x: u32, y: u32) -> Rect {
    let n = 2.0_f64.powi(z as i32);
    let step = (WORLD_HALF * 2.0) / n;
    let min_x = -WORLD_HALF + x as f64 * step;
    let max_y = WORLD_HALF - y as f64 * step;
    Rect::new(min_x, max_y - step, min_x + step, max_y)
}
// #endregion 🔖️Projection
