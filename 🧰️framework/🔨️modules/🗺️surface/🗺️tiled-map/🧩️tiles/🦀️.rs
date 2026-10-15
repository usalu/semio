//! 🧩️ Raster tile addressing of the tiled map: visible-tile cursor, tile keys and prefetch rings.
//!
//! Mounted as `pub mod tiles` of the tiled-map root.

// #region 🔖️Tiles
use super::canvas::camera::{Camera, Viewport};
use super::map_viewport;
use super::projection::WORLD_HALF;
use super::{pick_tile_z_target, Point, MAX_VISIBLE_TILE_REQUESTS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisibleTile {
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisibleTileCursor {
    z: u32,
    x: u32,
    x_min: u32,
    x_max: u32,
    y: u32,
    y_min: u32,
    y_max: u32,
    remaining: usize,
}

impl VisibleTileCursor {
    pub fn peek(&self) -> Option<VisibleTile> {
        (self.remaining != 0).then_some(VisibleTile { z: self.z, x: self.x, y: self.y })
    }

    pub fn advance(&mut self) -> bool {
        if self.remaining == 0 {
            return false;
        }
        self.remaining -= 1;
        if self.y < self.y_max {
            self.y += 1;
        } else {
            self.x += 1;
            self.y = self.y_min;
        }
        true
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }

    /// 🔭️ Zoom level this cursor enumerates tiles at — one of the two range fields the
    /// allocation-free dirty-revision signal hashes.
    pub fn z(&self) -> u32 {
        self.z
    }

    /// 📐️ Inclusive tile-index bounds `(x_min, x_max, y_min, y_max)` this cursor was built from —
    /// stable across `advance()`, unlike the mutable cursor position.
    pub fn bounds(&self) -> (u32, u32, u32, u32) {
        (self.x_min, self.x_max, self.y_min, self.y_max)
    }
}

pub fn pick_zoom(camera: &Camera, viewport: &Viewport, forced_lod_id: Option<&str>) -> u32 {
    let mut z = pick_tile_z_target(camera, viewport, forced_lod_id);
    while z > 0 && visible_tile_cursor(camera, viewport, z).remaining() > MAX_VISIBLE_TILE_REQUESTS {
        z -= 1;
    }
    z
}

pub fn visible_tile_cursor(camera: &Camera, viewport: &Viewport, z: u32) -> VisibleTileCursor {
    let corners = [
        map_viewport::screen_to_world(camera, viewport, Point::new(0.0, 0.0)),
        map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, 0.0)),
        map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, viewport.height as f64)),
        map_viewport::screen_to_world(camera, viewport, Point::new(0.0, viewport.height as f64)),
    ];
    let mut min_x = WORLD_HALF;
    let mut max_x = -WORLD_HALF;
    let mut min_y = WORLD_HALF;
    let mut max_y = -WORLD_HALF;
    for p in corners {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }
    let n = 2.0_f64.powi(z as i32);
    let step = (WORLD_HALF * 2.0) / n;
    let x0 = ((min_x + WORLD_HALF) / step).floor().max(0.0) as u32;
    let y0 = ((WORLD_HALF - max_y) / step).floor().max(0.0) as u32;
    // 🎯️ Last index is the tile *containing* the far edge, i.e. `ceil - 1`: a bare `ceil` named the
    // tile that starts where the viewport ends, so every window over-covered by a whole column and
    // a whole row (9×6 instead of 8×5 at 1920×1080/z14 — ~35% of each refresh's fetches and PNG/MVT
    // decodes spent on tiles `tile_rect_intersects_viewport` then discards at draw time). `ceil - 1`
    // also drops that zero-overlap tile when the edge lands exactly on a boundary, and the `max(x0)`
    // floor keeps a viewport smaller than one tile at a single tile rather than an empty range.
    let x1 = (((max_x + WORLD_HALF) / step).ceil() - 1.0).max(x0 as f64).min(n - 1.0) as u32;
    let y1 = (((WORLD_HALF - min_y) / step).ceil() - 1.0).max(y0 as f64).min(n - 1.0) as u32;
    let columns = usize::try_from(x1.saturating_sub(x0).saturating_add(1)).unwrap_or(usize::MAX);
    let rows = usize::try_from(y1.saturating_sub(y0).saturating_add(1)).unwrap_or(usize::MAX);
    VisibleTileCursor { z, x: x0, x_min: x0, x_max: x1, y: y0, y_min: y0, y_max: y1, remaining: columns.saturating_mul(rows) }
}

pub fn visible_tiles(camera: &Camera, viewport: &Viewport, z: u32) -> Vec<(u32, u32, u32)> {
    let mut cursor = visible_tile_cursor(camera, viewport, z);
    let mut out = Vec::with_capacity(cursor.remaining().min(MAX_VISIBLE_TILE_REQUESTS));
    while let Some(tile) = cursor.peek() {
        out.push((tile.z, tile.x, tile.y));
        cursor.advance();
    }
    out
}

pub fn tile_key(z: u32, x: u32, y: u32) -> String {
    format!("{z}/{x}/{y}")
}

pub fn parse_tile_key(key: &str) -> Option<(u32, u32, u32)> {
    let mut parts = key.split('/');
    let z: u32 = parts.next()?.parse().ok()?;
    let x: u32 = parts.next()?.parse().ok()?;
    let y: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((z, x, y))
}

pub fn tile_key_ancestors(z: u32, x: u32, y: u32) -> Vec<String> {
    let mut out = Vec::new();
    let mut cz = z;
    let mut cx = x;
    let mut cy = y;
    loop {
        out.push(tile_key(cz, cx, cy));
        if cz == 0 {
            break;
        }
        cz -= 1;
        cx >>= 1;
        cy >>= 1;
    }
    out
}

/// 📌️ Tiles that must never be evicted by the LRU cap: the visible set plus every pyramid
/// ancestor of each visible tile (for LOD fallback while a finer tile is still loading).
pub fn pinned_tile_keys(visible: &[(u32, u32, u32)]) -> std::collections::BTreeSet<String> {
    let mut keys = std::collections::BTreeSet::new();
    for &(z, x, y) in visible {
        for k in tile_key_ancestors(z, x, y) {
            keys.insert(k);
        }
    }
    keys
}

/// 🛰️ One-tile ring immediately outside `bounds`, wrapped in x and clamped in y to `[0, n)`,
/// excluding anything already in `visible`, capped to `cap` rows.
pub fn prefetch_ring_tiles(bounds: (u32, u32, u32, u32), visible: &[(u32, u32, u32)], z: u32, n: u32, cap: usize) -> Vec<(u32, u32, u32)> {
    if n == 0 || cap == 0 {
        return Vec::new();
    }
    let (x_min, x_max, y_min, y_max) = bounds;
    let visible_set: std::collections::BTreeSet<(u32, u32, u32)> = visible.iter().copied().collect();
    let wrap_x = |gx: i64| -> u32 {
        let n = n as i64;
        (((gx % n) + n) % n) as u32
    };
    let x_lo = x_min as i64 - 1;
    let x_hi = x_max as i64 + 1;
    let y_lo = y_min as i64 - 1;
    let y_hi = y_max as i64 + 1;
    let mut ring: Vec<(u32, u32, u32)> = Vec::new();
    for gy in y_lo..=y_hi {
        if gy < 0 || gy as u32 > n - 1 {
            continue;
        }
        let cy = gy as u32;
        for gx in x_lo..=x_hi {
            let inside = gx >= x_min as i64 && gx <= x_max as i64 && cy >= y_min && cy <= y_max;
            if inside {
                continue;
            }
            let cx = wrap_x(gx);
            let t = (z, cx, cy);
            if visible_set.contains(&t) {
                continue;
            }
            ring.push(t);
        }
    }
    ring.sort();
    ring.dedup();
    ring.truncate(cap);
    ring
}
// #endregion 🔖️Tiles
