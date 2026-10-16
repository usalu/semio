//! 📷️ MapHost policy wiring, camera, level-of-detail queries, tile cache, interaction commit, uploads, pointer input and feature picking.
//!
//! First half of `impl MapHost`; the second half lives in `host_scene`.

use super::*;

// #region 🔖️CameraAndPicking
// #region 🔖️MapContent
impl MapHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧩️ A host rendering by `extension`'s policy.
    pub fn with_extension(extension: Arc<dyn MapExtension>) -> Self {
        let mut host = Self::default();
        host.set_extension(extension);
        host
    }

    /// 🔁️ Replaces the domain extension (and with it the policy), invalidating the vector-scene cache.
    pub fn set_extension(&mut self, extension: Arc<dyn MapExtension>) {
        self.extension = extension;
        self.policy_revision = self.policy_revision.wrapping_add(1);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    pub fn extension(&self) -> &dyn MapExtension {
        self.extension.as_ref()
    }

    pub fn policy(&self) -> &dyn MapPolicy {
        self.extension.policy()
    }

    /// 📌️ The requested pinned band id, while the policy declares it.
    pub fn pinned_band_id(&self) -> Option<&str> {
        self.forced_lod_id.as_deref().filter(|id| self.policy().band_index(id).is_some())
    }

    /// 🌐️ Visible longitude span of the viewport, in degrees.
    pub fn viewport_span_degrees(&self) -> f64 {
        viewport_lon_span_degrees(&self.camera, &self.viewport)
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
        let policy = self.policy();
        let pinned = self.pinned_band_id();
        let span_deg = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let band = policy.band(policy.band_for_span(span_deg, pinned));
        serde_json::json!({
            "id": band.id,
            "name": band.name,
            "description": band.description,
            "tileZ": self.pick_raster_tile_zoom(),
            "spanDeg": span_deg,
            "mode": pinned.unwrap_or(MAP_LOD_MODE_AUTOMATIC),
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

    /// 🌈️ Requests the policy style preset `style`; an id the policy does not declare paints its first preset.
    pub fn set_vector_style(&mut self, style: &str) {
        self.vector_style = style.to_string();
    }

    /// 🎨️ The style preset the host paints with.
    pub fn style_preset(&self) -> &MapStylePreset {
        self.policy().style_preset(&self.vector_style)
    }

    /// 🔽️ [`MAP_LOD_MODE_AUTOMATIC`] or a band id; the id is kept even before the policy that declares it arrives, and pins only once it does.
    pub fn set_lod_mode(&mut self, mode: &str) {
        self.forced_lod_id = (mode != MAP_LOD_MODE_AUTOMATIC).then(|| mode.to_string());
    }

    /// 👁️ Replaces the per-toggle visibility with a `{ "<toggle key>": bool }` object; an absent key is visible.
    pub fn set_layer_visibility_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        self.layer_switches.visibility = serde_json::from_str(json)?;
        Ok(())
    }

    pub fn layer_visibility_json(&self) -> String {
        serde_json::to_string(&self.layer_switches.visibility).unwrap_or_else(|_| "{}".into())
    }

    /// 📏️ Replaces the per-toggle weights with a `{ "<toggle key>": number }` object, clamped to the slider range; an absent key weighs 1.0.
    pub fn set_layer_stroke_scale_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        self.layer_switches.weights = MapLayerSwitches::sanitized_weights(serde_json::from_str(json)?);
        Ok(())
    }

    pub fn layer_stroke_scale_json(&self) -> String {
        serde_json::to_string(&self.layer_switches.weights).unwrap_or_else(|_| "{}".into())
    }

    /// 👀️ Whether `target` paints under the policy's toggles and the current switches.
    pub fn layer_visible(&self, target: &MapLayerTarget) -> bool {
        self.layer_switches.visible(self.policy(), target)
    }

    /// ⚖️ The weight `target` paints with under the policy's toggles and the current switches.
    pub fn layer_weight(&self, target: &MapLayerTarget) -> f64 {
        self.layer_switches.weight(self.policy(), target)
    }

    pub fn pick_raster_tile_zoom(&self) -> u32 {
        tiles::pick_zoom(&self.camera, &self.viewport, self.policy(), self.pinned_band_id())
    }

    pub fn pick_vector_tile_zoom(&self) -> u32 {
        let policy = self.policy();
        let mut z = policy.vector_tile_z_target(map_view_facts(&self.camera, &self.viewport, policy), self.pick_raster_tile_zoom());
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

    pub fn vector_style_str(&self) -> &str {
        &self.style_preset().id
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

    /// 📨️ Mirrors the overlay features wholesale; a `policy` member that differs from the current policy replaces the extension with a [`DefaultMapExtension`] carrying it.
    pub fn sync_map_json(&mut self, json: &str) -> Result<(), FrameworkSurfaceTiledMapError> {
        let sync: MapSyncJson = serde_json::from_str(json)?;
        if let Some(descriptor) = sync.policy {
            if self.policy().descriptor() != &descriptor {
                self.set_extension(Arc::new(DefaultMapExtension::with_policy(DescribedMapPolicy::new(descriptor))));
            }
        }
        self.overlays.layers = sync.overlays.into_iter().map(|(layer, features)| (layer, features.into_iter().map(|feature| (feature.id.clone(), feature)).collect())).collect();
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

    /// 🖥️ Screen points of one overlay feature's coordinates through the live camera.
    fn overlay_screen_points(&self, feature: &MapOverlayFeature) -> Vec<Point> {
        feature.coordinates.iter().map(|[lon, lat]| map_viewport::world_to_screen(&self.camera, &self.viewport, projection::lonlat_to_world(*lon, *lat))).collect()
    }

    /// 📏️ Screen distance from `(sx, sy)` to one overlay feature: the marker centre for a point, the nearest segment for a line, zero inside or the nearest edge of an area; `None` when the feature has too few coordinates.
    fn overlay_screen_distance(&self, layer: &MapOverlayLayer, feature: &MapOverlayFeature, sx: f64, sy: f64) -> Option<f64> {
        let points = self.overlay_screen_points(feature);
        match layer.geometry {
            MapOverlayGeometry::Point => points.first().map(|s| ((s.x - sx).powi(2) + (s.y - sy).powi(2)).sqrt()),
            MapOverlayGeometry::Line | MapOverlayGeometry::Area => {
                if points.len() < 2 {
                    return None;
                }
                if layer.geometry == MapOverlayGeometry::Area && map_point_in_polygon(sx, sy, &points) {
                    return Some(0.0);
                }
                Some(points.windows(2).map(|pair| map_point_segment_distance(sx, sy, pair[0].x, pair[0].y, pair[1].x, pair[1].y)).fold(f64::INFINITY, f64::min))
            }
        }
    }

    /// 🎯️ The pickable overlay layers (those with a granularity), in declaration order.
    pub fn pickable_overlays(&self) -> impl Iterator<Item = &MapOverlayLayer> {
        self.policy().overlay_layers().iter().filter(|layer| layer.granularity.is_some())
    }

    /// 🎯️ The nearest pickable overlay feature within its layer's hit radius as `{ "kind": <granularity>, "id" }`, or `null`; the topmost (last declared) layer wins a tie.
    pub fn hit_test_feature_json(&self, sx: f64, sy: f64) -> String {
        let mut best: Option<(&str, &str, f64)> = None;
        let layers: Vec<&MapOverlayLayer> = self.pickable_overlays().collect();
        for layer in layers.into_iter().rev() {
            let Some(granularity) = layer.granularity.as_deref() else {
                continue;
            };
            for feature in self.overlays.features(&layer.id) {
                let Some(distance) = self.overlay_screen_distance(layer, feature, sx, sy) else {
                    continue;
                };
                if distance <= layer.hit_radius_px && best.is_none_or(|(_, _, best_distance)| distance < best_distance) {
                    best = Some((granularity, feature.id.as_str(), distance));
                }
            }
        }
        best.map_or_else(|| "null".into(), |(kind, id, _)| serde_json::json!({ "kind": kind, "id": id }).to_string())
    }

    /// 🧺️ `{ "<pickable layer id>": [ids] }` with one (possibly empty) row per pickable overlay layer.
    fn overlay_rows_json(&self, mut admits: impl FnMut(&MapOverlayLayer, &[Point]) -> bool) -> String {
        let mut rows = serde_json::Map::new();
        for layer in self.pickable_overlays() {
            let ids: Vec<serde_json::Value> = self
                .overlays
                .features(&layer.id)
                .filter(|feature| {
                    let points = self.overlay_screen_points(feature);
                    let enough = if layer.geometry == MapOverlayGeometry::Point { !points.is_empty() } else { points.len() >= 2 };
                    enough && admits(layer, &points)
                })
                .map(|feature| serde_json::Value::String(feature.id.clone()))
                .collect();
            rows.insert(layer.id.clone(), serde_json::Value::Array(ids));
        }
        serde_json::Value::Object(rows).to_string()
    }

    pub fn features_in_rect_json(&self, x0: f64, y0: f64, x1: f64, y1: f64, crossing: bool) -> String {
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let inside = |p: &Point| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y;
        self.overlay_rows_json(|layer, points| match layer.geometry {
            MapOverlayGeometry::Point => inside(&points[0]),
            MapOverlayGeometry::Line | MapOverlayGeometry::Area => {
                if crossing {
                    map_polyline_intersects_rect(points, min_x, min_y, max_x, max_y)
                } else {
                    points.iter().all(inside)
                }
            }
        })
    }

    pub fn features_in_polygon_json(&self, points_json: &str, crossing: bool) -> String {
        let polygon: Vec<Point> = serde_json::from_str::<Vec<[f64; 2]>>(points_json).map(|rows| rows.into_iter().map(|[x, y]| Point::new(x, y)).collect()).unwrap_or_default();
        if polygon.len() < 3 {
            return self.overlay_rows_json(|_, _| false);
        }
        self.overlay_rows_json(|layer, points| match layer.geometry {
            MapOverlayGeometry::Point => map_point_in_polygon(points[0].x, points[0].y, &polygon),
            MapOverlayGeometry::Line | MapOverlayGeometry::Area => {
                if crossing {
                    map_polyline_intersects_polygon(points, &polygon)
                } else {
                    points.iter().all(|p| map_point_in_polygon(p.x, p.y, &polygon))
                }
            }
        })
    }

    /// 📍️ Screen placement of one pickable feature (`kind` is its layer's granularity): `{ x, y }` for a point, the centroid plus `polyline` for a line or area, `null` when unknown.
    pub fn feature_screen_json(&self, kind: &str, id: &str) -> String {
        let Some(layer) = self.policy().overlay_for_granularity(kind) else {
            return "null".into();
        };
        let Some(feature) = self.overlays.feature(&layer.id, id) else {
            return "null".into();
        };
        let points = self.overlay_screen_points(feature);
        if points.is_empty() {
            return "null".into();
        }
        if layer.geometry == MapOverlayGeometry::Point {
            return serde_json::to_string(&serde_json::json!({ "x": points[0].x, "y": points[0].y })).unwrap_or_else(|_| "null".into());
        }
        let n = points.len() as f64;
        let cx = points.iter().map(|p| p.x).sum::<f64>() / n;
        let cy = points.iter().map(|p| p.y).sum::<f64>() / n;
        let polyline: Vec<[f64; 2]> = points.iter().map(|p| [p.x, p.y]).collect();
        serde_json::to_string(&serde_json::json!({ "x": cx, "y": cy, "polyline": polyline })).unwrap_or_else(|_| "null".into())
    }

    /// 🕹️ Replaces the deleted `set_selection_json`/`set_hover_json` push-setters. `granularity` is
    /// the domain's currently-active granularity (an overlay layer's granularity, from
    /// `InteractionState::active_granularity`) — the framework's `DomainSelection`/`DomainHover` carry
    /// one flat id list each, scoped to that granularity, so it clears every other granularity's ids
    /// and tags the hover accordingly. Called at render time with the framework's `InteractionState`,
    /// not pushed arbitrarily by app code.
    pub fn sync_interaction(&mut self, granularity: &str, selected_ids: &[String], hovered_id: Option<&str>) {
        self.selected.clear();
        if !selected_ids.is_empty() {
            self.selected.insert(granularity.to_string(), selected_ids.iter().cloned().collect());
        }
        self.hovered_kind = hovered_id.map(|_| granularity.to_string());
        self.hovered_id = hovered_id.map(str::to_string);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    /// 🕹️ Applies a scene's `selection_json` (`{ <overlay layer id>: [ids] }`) and `hover_json` (`{ "kind": <granularity>, "id" }` or `null`): the first declared pickable layer with selected ids wins, else the hover's granularity, else the first pickable one; the hover only lands when its kind is that granularity. The twin of React `resolveMapInteractionSync`.
    pub fn sync_interaction_json(&mut self, selection_json: &str, hover_json: &str) {
        let selection = serde_json::from_str::<serde_json::Value>(selection_json).unwrap_or_default();
        let hover = serde_json::from_str::<serde_json::Value>(hover_json).unwrap_or_default();
        let hover_kind = hover.get("kind").and_then(serde_json::Value::as_str).filter(|kind| !kind.is_empty());
        let ids = |layer: &MapOverlayLayer| -> Vec<String> { selection.get(&layer.id).and_then(serde_json::Value::as_array).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_owned)).collect()).unwrap_or_default() };
        let selected = self.pickable_overlays().map(|layer| (layer, ids(layer))).find(|(_, ids)| !ids.is_empty());
        let granularity = selected.as_ref().and_then(|(layer, _)| layer.granularity.clone()).or_else(|| hover_kind.map(str::to_owned)).or_else(|| self.pickable_overlays().find_map(|layer| layer.granularity.clone())).unwrap_or_default();
        let hovered = (hover_kind == Some(granularity.as_str())).then(|| hover.get("id").and_then(serde_json::Value::as_str)).flatten();
        let selected_ids = selected.map(|(_, ids)| ids).unwrap_or_default();
        self.sync_interaction(&granularity, &selected_ids, hovered);
    }

    /// 👻️ World coordinates to surface-local pixels through the map's live camera.
    pub fn world_to_screen_point(&self, wx: f64, wy: f64) -> (f64, f64) {
        let point = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy));
        (point.x, point.y)
    }

    /// 🔍️ Frames one pickable feature (`kind` is its layer's granularity): centres and zooms in on a point, fits a line or area of at least two coordinates.
    pub fn focus_feature(&mut self, kind: &str, id: &str) -> bool {
        let limits: serde_json::Value = serde_json::from_str(&map_camera_limits_json_for_viewport(&self.viewport)).unwrap_or_default();
        let min_zoom = limits.get("min").and_then(|value| value.as_f64()).unwrap_or(0.05);
        let max_zoom = limits.get("max").and_then(|value| value.as_f64()).unwrap_or(64.0);
        let Some((geometry, coordinates)) = self.policy().overlay_for_granularity(kind).and_then(|layer| self.overlays.feature(&layer.id, id).map(|feature| (layer.geometry, feature.coordinates.clone()))) else {
            return false;
        };
        if geometry == MapOverlayGeometry::Point {
            let Some([lon, lat]) = coordinates.first().copied() else {
                return false;
            };
            let world = projection::lonlat_to_world(lon, lat);
            self.camera.x = world.x;
            self.camera.y = world.y;
            self.camera.zoom = (self.camera.zoom * 1.75).clamp(min_zoom, max_zoom);
        } else {
            if coordinates.len() < 2 {
                return false;
            }
            let mut min_x = f64::INFINITY;
            let mut max_x = f64::NEG_INFINITY;
            let mut min_y = f64::INFINITY;
            let mut max_y = f64::NEG_INFINITY;
            for [lon, lat] in &coordinates {
                let world = projection::lonlat_to_world(*lon, *lat);
                min_x = min_x.min(world.x);
                max_x = max_x.max(world.x);
                min_y = min_y.min(world.y);
                max_y = max_y.max(world.y);
            }
            let span = (max_x - min_x).max(max_y - min_y).max(64.0);
            self.camera.x = (min_x + max_x) * 0.5;
            self.camera.y = (min_y + max_y) * 0.5;
            self.camera.zoom = ((self.viewport.width as f64) * 0.55 / span).clamp(min_zoom, max_zoom);
        }
        self.clamp_camera_to_world();
        self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
        true
    }

    pub fn has_tile(&self, key: &str) -> bool {
        self.tiles.tile_images.contains_key(key)
    }

    pub fn has_vector_tile(&self, key: &str) -> bool {
        self.tiles.vector_tiles.contains_key(key)
    }

    /// 🔎️ One overlay feature by layer id and feature id.
    pub fn overlay_feature(&self, layer: &str, id: &str) -> Option<&MapOverlayFeature> {
        self.overlays.feature(layer, id)
    }

    /// 🕹️ The selected feature ids of one granularity, in id order.
    pub fn selected_ids<'a>(&'a self, granularity: &str) -> impl Iterator<Item = &'a str> + 'a {
        self.selected.get(granularity).into_iter().flat_map(|ids| ids.iter()).map(String::as_str)
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
