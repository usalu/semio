//! 📷️ MapHost camera, level-of-detail queries, tile cache, interaction commit, uploads, pointer input and feature picking.
//!
//! First half of `impl MapHost`; the second half lives in `host_scene`.

use super::*;

// #region 🔖️CameraAndPicking
// #region 🔖️MapContent
impl MapHost {
    pub fn new() -> Self {
        Self::default()
    }

    fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.first()?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn apply_theme_field(next: &mut MapPalette, v: &serde_json::Value, key: &str, assign: impl FnOnce(&mut MapPalette, Color)) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = Self::color_from_json_rgba8(arr) {
                assign(next, c);
            }
        }
    }

    pub fn set_map_theme_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        let mut next = self.theme;
        Self::apply_theme_field(&mut next, &v, "surfaceClear", |t, c| t.surface_clear = c);
        Self::apply_theme_field(&mut next, &v, "landFill", |t, c| t.land_fill = c);
        Self::apply_theme_field(&mut next, &v, "landStroke", |t, c| {
            let rgba = c.to_rgba8();
            t.land_stroke = Color::from_rgba8(rgba.r, rgba.g, rgba.b, 0);
        });
        Self::apply_theme_field(&mut next, &v, "labelFill", |t, c| t.label_fill = c);
        Self::apply_theme_field(&mut next, &v, "labelHalo", |t, c| t.label_halo = c);
        Self::apply_theme_field(&mut next, &v, "regionFill", |t, c| t.region_fill = c);
        Self::apply_theme_field(&mut next, &v, "regionStroke", |t, c| t.region_stroke = c);
        Self::apply_theme_field(&mut next, &v, "routeStroke", |t, c| t.route_stroke = c);
        Self::apply_theme_field(&mut next, &v, "positionFill", |t, c| t.position_fill = c);
        Self::apply_theme_field(&mut next, &v, "positionStroke", |t, c| t.position_stroke = c);
        Self::apply_theme_field(&mut next, &v, "selectionStroke", |t, c| t.selection_stroke = c);
        Self::apply_theme_field(&mut next, &v, "hoverStroke", |t, c| t.hover_stroke = c);
        self.theme = next;
        Ok(())
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.set_size(width, height, dpr);
        self.clamp_camera_to_world();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn fit_world_camera(&mut self) {
        self.camera = projection::default_world_camera(&self.viewport);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn clamp_camera_to_world(&mut self) {
        clamp_camera_to_world_bounds(&mut self.camera, &self.viewport);
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({
            "x": self.camera.x,
            "y": self.camera.y,
            "zoom": self.camera.zoom,
        })
        .to_string()
    }

    pub fn current_lod_json(&self) -> String {
        let lod = active_map_lod(self.forced_lod_id.as_deref(), &self.camera, &self.viewport);
        let tile_z = self.pick_raster_tile_zoom();
        let span_deg = viewport_lon_span_degrees(&self.camera, &self.viewport);
        serde_json::json!({
            "id": lod.id,
            "name": lod.name,
            "description": lod.description,
            "tileZ": tile_z,
            "spanDeg": span_deg,
            "mode": self.forced_lod_id.as_deref().unwrap_or(GIS_MAP_LOD_MODE_AUTOMATIC),
        })
        .to_string()
    }

    pub fn visible_tiles_json(&self) -> String {
        let z = self.pick_raster_tile_zoom();
        let list = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let rows: Vec<serde_json::Value> = list.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn visible_vector_tiles_json(&self) -> String {
        if !vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            return "[]".into();
        }
        let z = self.pick_vector_tile_zoom();
        let list = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let rows: Vec<serde_json::Value> = list.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn visible_raster_tile_cursor(&self) -> tiles::VisibleTileCursor {
        tiles::visible_tile_cursor(&self.camera, &self.viewport, self.pick_raster_tile_zoom())
    }

    pub fn visible_vector_tile_cursor(&self) -> Option<tiles::VisibleTileCursor> {
        vector_tiles_available_at_camera_zoom(self.camera.zoom).then(|| tiles::visible_tile_cursor(&self.camera, &self.viewport, self.pick_vector_tile_zoom()))
    }

    /// 🚦️ Allocation-free dirty signal for the raster tile set — changes if and only if the visible
    /// tile range changes, so JS can poll it every frame instead of string-comparing `visible_tiles_json`.
    pub fn visible_tiles_revision(&self) -> u64 {
        tile_range_revision(&self.visible_raster_tile_cursor())
    }

    /// 🚦️ Same as [`Self::visible_tiles_revision`] for the vector tile set. Returns the sentinel `0`
    /// exactly when vector tiles are unavailable at the current camera zoom (mirrors
    /// `visible_vector_tiles_json`'s `"[]"` case).
    pub fn visible_vector_tiles_revision(&self) -> u64 {
        match self.visible_vector_tile_cursor() {
            Some(cursor) => tile_range_revision(&cursor),
            None => 0,
        }
    }

    /// 🛰️ One-tile prefetch ring outside the visible raster set, same row shape as
    /// [`Self::visible_tiles_json`], capped so `visible + prefetch <= MAX_VISIBLE_TILE_REQUESTS`.
    pub fn prefetch_tiles_json(&self) -> String {
        let z = self.pick_raster_tile_zoom();
        let visible = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let bounds = tiles::visible_tile_cursor(&self.camera, &self.viewport, z).bounds();
        let n = 2.0_f64.powi(z as i32) as u32;
        let cap = MAX_VISIBLE_TILE_REQUESTS.saturating_sub(visible.len());
        let ring = tiles::prefetch_ring_tiles(bounds, &visible, z, n, cap);
        let rows: Vec<serde_json::Value> = ring.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    /// 🛰️ Same as [`Self::prefetch_tiles_json`] for the vector tile set. Returns `"[]"` when vector
    /// tiles are unavailable at the current camera zoom.
    pub fn prefetch_vector_tiles_json(&self) -> String {
        if !vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            return "[]".into();
        }
        let z = self.pick_vector_tile_zoom();
        let visible = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let bounds = tiles::visible_tile_cursor(&self.camera, &self.viewport, z).bounds();
        let n = 2.0_f64.powi(z as i32) as u32;
        let cap = MAX_VISIBLE_TILE_REQUESTS.saturating_sub(visible.len());
        let ring = tiles::prefetch_ring_tiles(bounds, &visible, z, n, cap);
        let rows: Vec<serde_json::Value> = ring.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn interaction_revision(&self) -> u64 {
        self.interaction_revision
    }

    pub fn set_render_mode(&mut self, mode: &str) {
        let next = MapTileMode::parse(mode);
        if self.render_mode != next {
            self.render_mode = next;
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
        }
    }

    pub fn set_vector_style(&mut self, style: &str) {
        self.vector_style = MapVectorStyle::parse(style);
    }

    pub fn set_lod_mode(&mut self, mode: &str) {
        if mode == GIS_MAP_LOD_MODE_AUTOMATIC {
            self.forced_lod_id = None;
            return;
        }
        if GIS_MAP_LOD_SCALE.index_of(mode).is_some() {
            self.forced_lod_id = Some(mode.to_string());
        }
    }

    pub fn set_layer_visibility_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        self.layer_visibility = serde_json::from_str(json)?;
        Ok(())
    }

    pub fn layer_visibility_json(&self) -> String {
        serde_json::to_string(&self.layer_visibility).unwrap_or_else(|_| "{}".into())
    }

    pub fn set_layer_stroke_scale_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        let parsed: MapLayerStrokeScale = serde_json::from_str(json)?;
        self.layer_stroke_scale = parsed.sanitized();
        Ok(())
    }

    pub fn layer_stroke_scale_json(&self) -> String {
        serde_json::to_string(&self.layer_stroke_scale).unwrap_or_else(|_| "{}".into())
    }

    pub fn pick_raster_tile_zoom(&self) -> u32 {
        tiles::pick_zoom(&self.camera, &self.viewport, self.forced_lod_id.as_deref())
    }

    pub fn pick_vector_tile_zoom(&self) -> u32 {
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let span_cap = vector_tiles::max_tile_z_for_span(span);
        let lod_idx = resolve_map_lod_index_from_span(span);
        let mut z = if lod_idx <= 1 { span_cap } else { self.pick_raster_tile_zoom().min(span_cap) };
        z = z.min(vector_tiles::MAP_VECTOR_TILE_MAX_Z);
        while z > 0 && tiles::visible_tile_cursor(&self.camera, &self.viewport, z).remaining() > MAX_VISIBLE_TILE_REQUESTS {
            z -= 1;
        }
        z
    }

    pub fn render_mode_str(&self) -> &'static str {
        match self.render_mode {
            MapTileMode::Image => "image",
            MapTileMode::Vector => "vector",
            MapTileMode::Combined => "combined",
        }
    }

    pub fn vector_style_str(&self) -> &'static str {
        self.vector_style.as_str()
    }

    pub(super) fn retain_tiles_for_keys(&mut self, pinned: &std::collections::BTreeSet<String>) {
        if self.tiles.tile_images.len() <= MAX_MAP_TILE_CACHE_ENTRIES {
            return;
        }
        self.tiles.raster_touch.borrow_mut().retain(|k, _| self.tiles.tile_images.contains_key(k));
        while self.tiles.tile_images.len() > MAX_MAP_TILE_CACHE_ENTRIES {
            let touch = self.tiles.raster_touch.borrow();
            let victim = self.tiles.tile_images.keys().filter(|k| !pinned.contains(k.as_str())).min_by_key(|k| touch.get(k.as_str()).copied().unwrap_or(0)).cloned();
            drop(touch);
            match victim {
                Some(k) => {
                    self.tiles.tile_images.remove(&k);
                    self.tiles.raster_touch.borrow_mut().remove(&k);
                }
                None => break,
            }
        }
    }

    pub(super) fn retain_vector_tiles_for_keys(&mut self, pinned: &std::collections::BTreeSet<String>) {
        if self.tiles.vector_tiles.len() <= MAX_MAP_TILE_CACHE_ENTRIES {
            return;
        }
        self.tiles.vector_touch.borrow_mut().retain(|k, _| self.tiles.vector_tiles.contains_key(k));
        while self.tiles.vector_tiles.len() > MAX_MAP_TILE_CACHE_ENTRIES {
            let touch = self.tiles.vector_touch.borrow();
            let victim = self.tiles.vector_tiles.keys().filter(|k| !pinned.contains(k.as_str())).min_by_key(|k| touch.get(k.as_str()).copied().unwrap_or(0)).cloned();
            drop(touch);
            match victim {
                Some(k) => {
                    self.tiles.vector_tiles.remove(&k);
                    self.tiles.vector_touch.borrow_mut().remove(&k);
                }
                None => break,
            }
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, true);
    }

    /// 📷️ Returns the live map camera as `[x, y, zoom]`.
    pub fn camera(&self) -> [f64; 3] {
        [self.camera.x, self.camera.y, self.camera.zoom]
    }

    /// 🔇️ Updates the live camera without appending a camera event during a gesture.
    pub fn set_camera_silent(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, false);
    }

    fn set_camera_internal(&mut self, x: f64, y: f64, zoom: f64, emit_event: bool) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom;
        self.clamp_camera_to_world();
        if emit_event {
            self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
        }
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        map_wheel_screen(&mut self.camera, &self.viewport, sx, sy, delta_y);
        self.clamp_camera_to_world();
        self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn plan_interaction(&self, intent: MapInteractionIntent) -> MapInteractionPlan {
        let mut next_camera = self.camera.clone();
        let mut next_interaction = self.interaction.clone();
        let mut emit_camera = false;
        match intent {
            MapInteractionIntent::Wheel { sx, sy, delta_y } => {
                map_wheel_screen(&mut next_camera, &self.viewport, sx, sy, delta_y);
                clamp_camera_to_world_bounds(&mut next_camera, &self.viewport);
                emit_camera = true;
            }
            MapInteractionIntent::PointerDown { sx, sy, button } => {
                if button == 1 {
                    next_interaction = MapInteraction::Pan { origin: self.camera.clone(), start_screen: Point::new(sx, sy) };
                }
            }
            MapInteractionIntent::PointerMove { sx, sy } => {
                if let MapInteraction::Pan { origin, start_screen } = &self.interaction {
                    let dx = (sx - start_screen.x) / self.camera.zoom;
                    let dy = (sy - start_screen.y) / self.camera.zoom;
                    next_camera.x = origin.x - dx;
                    next_camera.y = origin.y - dy;
                    clamp_camera_to_world_bounds(&mut next_camera, &self.viewport);
                }
            }
            MapInteractionIntent::PointerUp { sx, sy } => {
                if let MapInteraction::Pan { start_screen, .. } = &self.interaction {
                    let dx = sx - start_screen.x;
                    let dy = sy - start_screen.y;
                    emit_camera = (dx * dx + dy * dy).sqrt() >= 6.0;
                }
                next_interaction = MapInteraction::None;
            }
        }
        MapInteractionPlan { revision: self.interaction_revision, expected_camera: self.camera.clone(), expected_interaction: self.interaction.clone(), next_camera, next_interaction, emit_camera }
    }

    pub fn commit_interaction(&mut self, plan: MapInteractionPlan) -> bool {
        if !self.can_commit_interaction(&plan) {
            return false;
        }
        self.camera = plan.next_camera;
        self.interaction = plan.next_interaction;
        if plan.emit_camera {
            self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
        }
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        true
    }

    pub fn can_commit_interaction(&self, plan: &MapInteractionPlan) -> bool {
        self.interaction_revision == plan.revision && map_camera_matches(&self.camera, &plan.expected_camera) && map_interaction_matches(&self.interaction, &plan.expected_interaction)
    }

    pub fn cancel_interaction(&mut self) -> bool {
        if matches!(self.interaction, MapInteraction::None) {
            return false;
        }
        self.interaction = MapInteraction::None;
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        true
    }

    pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, png_bytes: &[u8]) -> Result<(), FrameworkSurfaceTiledMapError> {
        let key = tiles::tile_key(z, x, y);
        if self.tiles.tile_images.contains_key(&key) {
            self.tiles.touch_raster(&key);
            return Ok(());
        }
        let decoded = semio_framework_pixels::decode_png(png_bytes)?;
        let image = RasterImage::rgba8(decoded.width, decoded.height, Arc::new(decoded.pixels));
        self.tiles.touch_raster(&key);
        self.tiles.tile_images.insert(key, Arc::new(image));
        Ok(())
    }

    pub fn upload_vector_tile(&mut self, z: u32, x: u32, y: u32, pbf_bytes: &[u8]) -> Result<(), FrameworkSurfaceTiledMapError> {
        let key = tiles::tile_key(z, x, y);
        if self.tiles.vector_tiles.contains_key(&key) {
            self.tiles.touch_vector(&key);
            return Ok(());
        }
        let tile = vector_tiles::decode_mvt(pbf_bytes)?;
        self.tiles.touch_vector(&key);
        self.tiles.vector_tiles.insert(key, tile);
        Ok(())
    }

    pub fn sync_map_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        let desc: MapDescriptorJson = serde_json::from_str(json)?;
        self.features.positions.clear();
        self.features.routes.clear();
        self.features.regions.clear();
        for p in desc.positions {
            self.features.positions.insert(p.id.clone(), p);
        }
        for r in desc.routes {
            self.features.routes.insert(r.id.clone(), r);
        }
        for reg in desc.regions {
            self.features.regions.insert(reg.id.clone(), reg);
        }
        Ok(())
    }

    pub fn drain_events_json(&mut self) -> String {
        let out = std::mem::take(&mut self.events);
        serde_json::to_string(&out).unwrap_or_else(|_| "[]".into())
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        if button != 1 {
            return;
        }
        self.interaction = MapInteraction::Pan { origin: self.camera.clone(), start_screen: Point::new(sx, sy) };
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        let MapInteraction::Pan { origin, start_screen } = self.interaction.clone() else {
            return;
        };
        let dx = (sx - start_screen.x) / self.camera.zoom;
        let dy = (sy - start_screen.y) / self.camera.zoom;
        self.camera.x = origin.x - dx;
        self.camera.y = origin.y - dy;
        self.clamp_camera_to_world();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        if matches!(self.interaction, MapInteraction::Pan { .. }) {
            let click = if let MapInteraction::Pan { start_screen, .. } = &self.interaction {
                let dx = sx - start_screen.x;
                let dy = sy - start_screen.y;
                (dx * dx + dy * dy).sqrt() < 6.0
            } else {
                false
            };
            if !click {
                self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
            }
        }
        self.interaction = MapInteraction::None;
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    fn hit_test_position(&self, sx: f64, sy: f64) -> Option<String> {
        let hit_r = 14.0;
        let hit_r2 = hit_r * hit_r;
        let mut best: Option<(String, f64)> = None;
        for pos in self.features.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let dx = s.x - sx;
            let dy = s.y - sy;
            let d2 = dx * dx + dy * dy;
            if d2 <= hit_r2 && best.as_ref().is_none_or(|(_, best_d2)| d2 < *best_d2) {
                best = Some((pos.id.clone(), d2));
            }
        }
        best.map(|(id, _)| id)
    }

    fn hit_test_route(&self, sx: f64, sy: f64) -> Option<String> {
        let hit_r = ui_styling::strokes::MAP_ROUTE_DEFAULT * 4.0 + 6.0;
        let mut best: Option<(String, f64)> = None;
        for route in self.features.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let mut min_d = f64::INFINITY;
            let mut prev: Option<Point> = None;
            for [lon, lat] in &route.points {
                let w = projection::lonlat_to_world(*lon, *lat);
                let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                if let Some(p0) = prev {
                    min_d = min_d.min(map_point_segment_distance(sx, sy, p0.x, p0.y, s.x, s.y));
                }
                prev = Some(s);
            }
            if min_d <= hit_r && best.as_ref().is_none_or(|(_, best_d)| min_d < *best_d) {
                best = Some((route.id.clone(), min_d));
            }
        }
        best.map(|(id, _)| id)
    }

    pub fn hit_test_feature_json(&self, sx: f64, sy: f64) -> String {
        let pos_hit = self.hit_test_position(sx, sy);
        let route_hit = self.hit_test_route(sx, sy);
        match (pos_hit, route_hit) {
            (Some(pos_id), Some(route_id)) => {
                let pos_d = self.position_screen_distance(sx, sy, &pos_id).unwrap_or(f64::INFINITY);
                let route_d = self.route_screen_distance(sx, sy, &route_id).unwrap_or(f64::INFINITY);
                if pos_d <= route_d {
                    serde_json::json!({ "kind": "position", "id": pos_id }).to_string()
                } else {
                    serde_json::json!({ "kind": "route", "id": route_id }).to_string()
                }
            }
            (Some(pos_id), None) => serde_json::json!({ "kind": "position", "id": pos_id }).to_string(),
            (None, Some(route_id)) => serde_json::json!({ "kind": "route", "id": route_id }).to_string(),
            (None, None) => "null".into(),
        }
    }

    fn position_screen_distance(&self, sx: f64, sy: f64, id: &str) -> Option<f64> {
        let pos = self.features.positions.get(id)?;
        let w = projection::lonlat_to_world(pos.lon, pos.lat);
        let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
        Some(((s.x - sx).powi(2) + (s.y - sy).powi(2)).sqrt())
    }

    fn route_screen_distance(&self, sx: f64, sy: f64, id: &str) -> Option<f64> {
        let route = self.features.routes.get(id)?;
        if route.points.len() < 2 {
            return None;
        }
        let mut min_d = f64::INFINITY;
        let mut prev: Option<Point> = None;
        for [lon, lat] in &route.points {
            let w = projection::lonlat_to_world(*lon, *lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            if let Some(p0) = prev {
                min_d = min_d.min(map_point_segment_distance(sx, sy, p0.x, p0.y, s.x, s.y));
            }
            prev = Some(s);
        }
        Some(min_d)
    }

    pub fn features_in_rect_json(&self, x0: f64, y0: f64, x1: f64, y1: f64, crossing: bool) -> String {
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let mut positions: Vec<String> = Vec::new();
        let mut routes: Vec<String> = Vec::new();
        for pos in self.features.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let hit = s.x >= min_x && s.x <= max_x && s.y >= min_y && s.y <= max_y;
            if hit {
                positions.push(pos.id.clone());
            }
        }
        for route in self.features.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let screen_pts: Vec<Point> = route
                .points
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    map_viewport::world_to_screen(&self.camera, &self.viewport, w)
                })
                .collect();
            let hit = if crossing { map_polyline_intersects_rect(&screen_pts, min_x, min_y, max_x, max_y) } else { screen_pts.iter().all(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y) };
            if hit {
                routes.push(route.id.clone());
            }
        }
        serde_json::json!({ "positions": positions, "routes": routes }).to_string()
    }

    pub fn features_in_polygon_json(&self, points_json: &str, crossing: bool) -> String {
        let points: Vec<Point> = match serde_json::from_str::<Vec<[f64; 2]>>(points_json) {
            Ok(rows) => rows.into_iter().map(|[x, y]| Point::new(x, y)).collect(),
            Err(_) => return serde_json::json!({ "positions": [], "routes": [] }).to_string(),
        };
        if points.len() < 3 {
            return serde_json::json!({ "positions": [], "routes": [] }).to_string();
        }
        let mut positions: Vec<String> = Vec::new();
        let mut routes: Vec<String> = Vec::new();
        for pos in self.features.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let inside = map_point_in_polygon(s.x, s.y, &points);
            if crossing {
                if inside {
                    positions.push(pos.id.clone());
                }
            } else if inside {
                positions.push(pos.id.clone());
            }
        }
        for route in self.features.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let screen_pts: Vec<Point> = route
                .points
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    map_viewport::world_to_screen(&self.camera, &self.viewport, w)
                })
                .collect();
            let hit = if crossing { map_polyline_intersects_polygon(&screen_pts, &points) } else { screen_pts.iter().all(|p| map_point_in_polygon(p.x, p.y, &points)) };
            if hit {
                routes.push(route.id.clone());
            }
        }
        serde_json::json!({ "positions": positions, "routes": routes }).to_string()
    }

    pub fn feature_screen_json(&self, kind: &str, id: &str) -> String {
        match kind {
            "position" => self.position_screen_json(id),
            "route" => {
                let Some(route) = self.features.routes.get(id) else {
                    return "null".into();
                };
                if route.points.is_empty() {
                    return "null".into();
                }
                let mut cx = 0.0;
                let mut cy = 0.0;
                let mut polyline = Vec::with_capacity(route.points.len());
                for [lon, lat] in &route.points {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                    cx += s.x;
                    cy += s.y;
                    polyline.push([s.x, s.y]);
                }
                let n = route.points.len() as f64;
                serde_json::to_string(&serde_json::json!({ "x": cx / n, "y": cy / n, "polyline": polyline })).unwrap_or_else(|_| "null".into())
            }
            _ => "null".into(),
        }
    }

    /// 🕹️ Replaces the deleted `set_selection_json`/`set_hover_json` push-setters. `granularity` is
    /// the domain's currently-active granularity (`"position"` or `"route"`, from
    /// `InteractionState::active_granularity`) — the framework's `DomainSelection`/`DomainHover` carry
    /// one flat id list each, scoped to that granularity, so it clears the other kind's ids and tags
    /// the hover accordingly. Called at render time with the framework's `InteractionState`, not
    /// pushed arbitrarily by app code.
    pub fn sync_interaction(&mut self, granularity: &str, selected_ids: &[String], hovered_id: Option<&str>) {
        self.selected_positions.clear();
        self.selected_routes.clear();
        match granularity {
            "route" => self.selected_routes = selected_ids.iter().cloned().collect(),
            _ => self.selected_positions = selected_ids.iter().cloned().collect(),
        }
        self.hovered_kind = hovered_id.map(|_| granularity.to_string());
        self.hovered_id = hovered_id.map(str::to_string);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn position_screen_json(&self, id: &str) -> String {
        let Some(pos) = self.features.positions.get(id) else {
            return "null".into();
        };
        let w = projection::lonlat_to_world(pos.lon, pos.lat);
        let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
        serde_json::to_string(&serde_json::json!({ "x": s.x, "y": s.y })).unwrap_or_else(|_| "null".into())
    }

    /// 👻️ World coordinates to surface-local pixels through the map's live camera.
    pub fn world_to_screen_point(&self, wx: f64, wy: f64) -> (f64, f64) {
        let point = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy));
        (point.x, point.y)
    }

    pub fn focus_feature(&mut self, kind: &str, id: &str) -> bool {
        let limits: serde_json::Value = serde_json::from_str(&gis_map_camera_limits_json_for_viewport(&self.viewport)).unwrap_or_default();
        let min_zoom = limits.get("min").and_then(|value| value.as_f64()).unwrap_or(0.05);
        let max_zoom = limits.get("max").and_then(|value| value.as_f64()).unwrap_or(64.0);
        match kind {
            "position" => {
                let Some(pos) = self.features.positions.get(id) else {
                    return false;
                };
                let world = projection::lonlat_to_world(pos.lon, pos.lat);
                self.camera.x = world.x;
                self.camera.y = world.y;
                self.camera.zoom = (self.camera.zoom * 1.75).clamp(min_zoom, max_zoom);
                self.clamp_camera_to_world();
                self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
                true
            }
            "route" => {
                let Some(route) = self.features.routes.get(id) else {
                    return false;
                };
                if route.points.len() < 2 {
                    return false;
                }
                let mut min_x = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for [lon, lat] in &route.points {
                    let world = projection::lonlat_to_world(*lon, *lat);
                    min_x = min_x.min(world.x);
                    max_x = max_x.max(world.x);
                    min_y = min_y.min(world.y);
                    max_y = max_y.max(world.y);
                }
                let span = (max_x - min_x).max(max_y - min_y).max(64.0);
                let fit_zoom = ((self.viewport.width as f64) * 0.55 / span).clamp(min_zoom, max_zoom);
                self.camera.x = (min_x + max_x) * 0.5;
                self.camera.y = (min_y + max_y) * 0.5;
                self.camera.zoom = fit_zoom;
                self.clamp_camera_to_world();
                self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
                true
            }
            _ => false,
        }
    }

    pub fn has_tile(&self, key: &str) -> bool {
        self.tiles.tile_images.contains_key(key)
    }

    pub fn has_vector_tile(&self, key: &str) -> bool {
        self.tiles.vector_tiles.contains_key(key)
    }

    pub fn selected_positions_json(&self) -> Vec<String> {
        self.selected_positions.iter().cloned().collect()
    }

    pub fn selected_position_ids(&self) -> impl ExactSizeIterator<Item = &str> {
        self.selected_positions.iter().map(String::as_str)
    }

    pub fn selected_routes_json(&self) -> Vec<String> {
        self.selected_routes.iter().cloned().collect()
    }

    pub fn selected_route_ids(&self) -> impl ExactSizeIterator<Item = &str> {
        self.selected_routes.iter().map(String::as_str)
    }

    pub fn hovered_kind(&self) -> Option<&str> {
        self.hovered_kind.as_deref()
    }

    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered_id.as_deref()
    }

    fn push_event(&mut self, kind: &str, payload: serde_json::Value) {
        self.events.push(serde_json::Value::Object(serde_json::Map::from_iter([("type".into(), kind.into()), ("payload".into(), payload)])));
    }
}
// #endregion 🔖️MapContent
// #endregion 🔖️CameraAndPicking
