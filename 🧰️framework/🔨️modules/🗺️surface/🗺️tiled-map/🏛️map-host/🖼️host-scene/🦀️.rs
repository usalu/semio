//! 🖼️ MapHost scene building: visible-tile preparation, tile geometry, raster and vector painting, overlays and the CanvasContent impl.
//!
//! Second half of `impl MapHost`; the first half lives in `camera_and_picking`. Implements `canvas_content::CanvasContent` for [`super::MapHost`].

use super::*;

// #region 🔖️HostScene
// #region 🔖️MapContent
#[derive(Clone, Copy)]
struct VectorTileCoordinates {
    z: u32,
    x: u32,
    y: u32,
    extent: u32,
}

#[derive(Clone, Copy)]
struct MapFigurePalette {
    ink: Color,
    paper: Color,
}

impl MapHost {
    pub fn prepare_visible_tiles(&mut self) {
        let z = self.pick_raster_tile_zoom();
        let visible = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let pinned = tiles::pinned_tile_keys(&visible);
        self.retain_tiles_for_keys(&pinned);
        if matches!(self.render_mode, MapTileMode::Vector | MapTileMode::Combined) {
            if vector_tiles_available_at_camera_zoom(self.camera.zoom) {
                let vz = self.pick_vector_tile_zoom();
                let vvisible = tiles::visible_tiles(&self.camera, &self.viewport, vz);
                let vpinned = tiles::pinned_tile_keys(&vvisible);
                self.retain_vector_tiles_for_keys(&vpinned);
            } else {
                self.tiles.vector_tiles.clear();
                self.tiles.vector_touch.borrow_mut().clear();
            }
        }
    }

    fn tile_local_to_screen(&self, z: u32, x: u32, y: u32, extent: u32, tx: f64, ty: f64) -> Point {
        let rect = projection::tile_world_rect(z, x, y);
        let step = rect.width() / extent as f64;
        let wx = rect.x0() + tx * step;
        let wy = rect.y1() - ty * step;
        map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy))
    }

    pub(crate) fn tile_rect_intersects_viewport(&self, rect: Rect) -> bool {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        let corners = [
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0())),
        ];
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in corners {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        min_x < w && max_x > 0.0 && min_y < h && max_y > 0.0
    }

    pub(crate) fn tile_raster_affine(&self, rect: Rect, img_w: u32, img_h: u32) -> Affine {
        let nw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1()));
        let ne = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1()));
        let sw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0()));
        let w = img_w.max(1) as f64;
        let h = img_h.max(1) as f64;
        Affine::new([(ne.x - nw.x) / w, (ne.y - nw.y) / w, (sw.x - nw.x) / h, (sw.y - nw.y) / h, nw.x, nw.y])
    }

    fn screen_segment_jump_limit(&self) -> f64 {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        (w * w + h * h).sqrt() * 0.45
    }

    fn tile_screen_segment_jump_limit(&self, z: u32, x: u32, y: u32) -> f64 {
        let rect = projection::tile_world_rect(z, x, y);
        let nw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1()));
        let se = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0()));
        nw.distance(se) * 1.2
    }

    fn bleed_screen_quad_corners(corners: [Point; 4], bleed: f64) -> [Point; 4] {
        let cx = corners.iter().map(|p| p.x).sum::<f64>() / 4.0;
        let cy = corners.iter().map(|p| p.y).sum::<f64>() / 4.0;
        std::array::from_fn(|i| {
            let p = corners[i];
            let dx = p.x - cx;
            let dy = p.y - cy;
            let len = (dx * dx + dy * dy).sqrt();
            if len < 1e-9 {
                return p;
            }
            let scale = (len + bleed) / len;
            Point::new(cx + dx * scale, cy + dy * scale)
        })
    }

    fn append_viewport_fill(&self, scene: &mut Scene, fill: Color) {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        let bleed = 1.0;
        let corners = Self::bleed_screen_quad_corners([Point::new(0.0, 0.0), Point::new(w, 0.0), Point::new(w, h), Point::new(0.0, h)], bleed);
        let mut path = BezPath::new();
        path.move_to(corners[0]);
        for p in &corners[1..] {
            path.line_to(*p);
        }
        path.close_path();
        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
    }

    fn append_vector_tile_quad_backdrop(&self, scene: &mut Scene, z: u32, x: u32, y: u32, fill: Color) {
        let rect = projection::tile_world_rect(z, x, y);
        let corners = Self::bleed_screen_quad_corners(
            [
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0())),
            ],
            2.0,
        );
        let mut path = BezPath::new();
        path.move_to(corners[0]);
        for p in &corners[1..] {
            path.line_to(*p);
        }
        path.close_path();
        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
    }

    fn append_vector_tile_land_backdrop(&self, scene: &mut Scene, z: u32, x: u32, y: u32, fill: Color) {
        self.append_vector_tile_quad_backdrop(scene, z, x, y, fill);
    }

    fn append_screen_polyline(path: &mut BezPath, pts: &[(f64, f64)], to_screen: impl Fn(f64, f64) -> Point, jump_limit: f64) {
        if pts.len() < 2 {
            return;
        }
        let mut prev: Option<Point> = None;
        for (i, &(lx, ly)) in pts.iter().enumerate() {
            let s = to_screen(lx, ly);
            let jump = prev.map_or(0.0, |p| p.distance(s));
            if i == 0 || jump > jump_limit {
                path.move_to(s);
            } else {
                path.line_to(s);
            }
            prev = Some(s);
        }
    }

    fn append_screen_ring(path: &mut BezPath, ring: &[(f64, f64)], to_screen: impl Fn(f64, f64) -> Point, jump_limit: f64) {
        if ring.len() < 3 {
            return;
        }
        Self::append_screen_polyline(path, ring, to_screen, jump_limit);
        path.close_path();
    }

    fn append_vector_tile_polygon(&self, scene: &mut Scene, tile: VectorTileCoordinates, rings: &[Vec<(f64, f64)>], fill: Color, stroke: Color, stroke_width: f64) {
        let VectorTileCoordinates { z: tz, x: tx, y: ty, extent } = tile;
        if rings.is_empty() {
            return;
        }
        let jump = self.tile_screen_segment_jump_limit(tz, tx, ty);
        let mut path = BezPath::new();
        let mut has_path = false;
        for ring in rings {
            if ring.len() < 3 {
                continue;
            }
            has_path = true;
            Self::append_screen_ring(&mut path, ring, |lx, ly| self.tile_local_to_screen(tz, tx, ty, extent, lx, ly), jump);
        }
        if !has_path {
            return;
        }
        scene.fill(FillRule::EvenOdd, Affine::IDENTITY, fill, None, &path);
        if stroke.to_rgba8().a > 5 {
            scene.stroke(&Stroke::new(stroke_width), Affine::IDENTITY, stroke, None, &path);
        }
    }

    fn append_vector_tile_polygon_rings_nonzero(&self, scene: &mut Scene, tile: VectorTileCoordinates, rings: &[Vec<(f64, f64)>], fill: Color) {
        self.append_vector_tile_polygon(scene, tile, rings, fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
    }

    fn append_vector_tile_polygon_filled_rings(&self, scene: &mut Scene, tile: VectorTileCoordinates, rings: &[Vec<(f64, f64)>], fill: Color) {
        for ring in rings {
            self.append_vector_tile_polygon(scene, tile, std::slice::from_ref(ring), fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
        }
    }

    fn append_vector_tile_lines(&self, scene: &mut Scene, tile: VectorTileCoordinates, lines: &[Vec<(f64, f64)>], stroke: Color, width: f64) {
        let VectorTileCoordinates { z: tz, x: tx, y: ty, extent } = tile;
        if lines.is_empty() || stroke.to_rgba8().a <= 5 || width <= 0.0 {
            return;
        }
        let to_screen = |lx: f64, ly: f64| self.tile_local_to_screen(tz, tx, ty, extent, lx, ly);
        for line in lines {
            if line.len() < 2 {
                continue;
            }
            if vector_tiles::mvt_polyline_is_tile_bbox_artifact(extent, line) {
                continue;
            }
            let jump = self.tile_screen_segment_jump_limit(tz, tx, ty);
            let mut path = BezPath::new();
            let mut prev: Option<Point> = None;
            for window in line.windows(2) {
                let a = (window[0].0, window[0].1);
                let b = (window[1].0, window[1].1);
                if vector_tiles::mvt_segment_is_tile_seam(extent, a, b) {
                    prev = None;
                    continue;
                }
                let sa = to_screen(a.0, a.1);
                let sb = to_screen(b.0, b.1);
                match prev {
                    None => path.move_to(sa),
                    Some(p) if p.distance(sa) > jump => path.move_to(sa),
                    _ => {}
                }
                path.line_to(sb);
                prev = Some(sb);
            }
            if prev.is_some() {
                scene.stroke(&Stroke::new(width), Affine::IDENTITY, stroke, None, &path);
            }
        }
    }

    /// 🔢️ Mixes the per-toggle visibility and weights (key bytes plus exact value bits) into the cache key.
    fn layer_switches_hash(switches: &MapLayerSwitches) -> u64 {
        let visibility = switches.visibility.iter().fold(REVISION_FNV_OFFSET, |h, (key, visible)| fnv1a_mix(fnv1a_bytes(h, key.as_bytes()), *visible as u64));
        switches.weights.iter().fold(visibility, |h, (key, weight)| fnv1a_mix(fnv1a_bytes(h, key.as_bytes()), weight.to_bits()))
    }

    /// 👀️ Whether vector paint role `role` is switched on.
    fn role_on(&self, role: MapLayerRole) -> bool {
        self.layer_visible(&MapLayerTarget::Role(role))
    }

    /// ⚖️ The weight vector paint role `role` paints with.
    fn role_weight(&self, role: MapLayerRole) -> f64 {
        self.layer_weight(&MapLayerTarget::Role(role))
    }

    /// 🔢️ Mixes the 12-color theme palette into the cache key via its 8-bit-per-channel form.
    fn theme_hash(t: &MapPalette) -> u64 {
        [t.surface_clear, t.land_fill, t.land_stroke, t.label_fill, t.label_halo, t.region_fill, t.region_stroke, t.route_stroke, t.position_fill, t.position_stroke, t.selection_stroke, t.hover_stroke]
            .into_iter()
            .fold(REVISION_FNV_OFFSET, |h, c| {
                let rgba = c.to_rgba8();
                fnv1a_mix(h, u32::from_be_bytes([rgba.r, rgba.g, rgba.b, rgba.a]) as u64)
            })
    }

    /// 🔑️ Hashes every vector-scene input except `camera.x`/`camera.y` — a hit means only the camera
    /// translated (or nothing changed), so [`Self::append_vector_tiles_cached`] may reuse the cached
    /// `Scene` in place of a full [`Self::append_vector_tiles`] rebuild.
    fn vector_scene_cache_key(&self) -> u64 {
        let mut h = REVISION_FNV_OFFSET;
        h = fnv1a_mix(h, self.camera.zoom.to_bits());
        h = fnv1a_mix(h, self.viewport.width as u64);
        h = fnv1a_mix(h, self.viewport.height as u64);
        h = fnv1a_mix(h, self.viewport.dpr.to_bits());
        h = fnv1a_mix(h, self.visible_vector_tiles_revision());
        h = fnv1a_mix(h, self.tiles.vector_tiles.len() as u64);
        h = fnv1a_mix(h, self.render_mode as u64);
        h = fnv1a_bytes(h, self.style_preset().id.as_bytes());
        h = fnv1a_mix(h, self.policy_revision);
        h = fnv1a_bytes(h, self.pinned_band_id().unwrap_or("").as_bytes());
        h = fnv1a_mix(h, Self::layer_switches_hash(&self.layer_switches));
        h = fnv1a_mix(h, Self::theme_hash(&self.theme));
        // 🚫️ `interaction_revision` deliberately absent: `append_vector_tiles` never reads selection or
        // hover (those belong to the overlay layers, which are drawn fresh outside this
        // cache), while `pointer_move_screen`/`set_camera` bump the revision on *every* frame of a pan —
        // keying on it invalidated the cache once per frame and left a drag at the uncached 35 ms.
        h
    }

    /// 🕰️ Refreshes the vector-tile LRU touch ledger for the currently visible set without emitting
    /// any geometry — used on a cached-scene hit, where [`Self::append_vector_tiles`] itself (the
    /// call that would otherwise touch these keys) is skipped.
    fn touch_visible_vector_tiles(&self) {
        for key in self.tiles.vector_tiles.keys() {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            if self.tile_rect_intersects_viewport(projection::tile_world_rect(tz, tx, ty)) {
                self.tiles.touch_vector(key);
            }
        }
    }

    /// 🔭️ Max z among held vector tiles whose bbox intersects the viewport — `None` when none do,
    /// mirroring the gate `append_vector_tiles` itself applies before drawing any tile content.
    fn vector_tile_render_z(&self) -> Option<u32> {
        let mut z: Option<u32> = None;
        for key in self.tiles.vector_tiles.keys() {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            if self.tile_rect_intersects_viewport(projection::tile_world_rect(tz, tx, ty)) {
                z = Some(z.map_or(tz, |cur| cur.max(tz)));
            }
        }
        z
    }

    /// 🖌️ The viewport-covering wash `append_vector_tiles_colored`/`append_vector_tiles_figure` used
    /// to paint themselves. It approximates "everything visible is backdrop-colored" rather than
    /// tracking any world position, so it MUST stay screen-pinned — [`Self::append_vector_tiles_cached`]
    /// draws it fresh every frame outside the translate-cached tile content, never inside it.
    fn vector_tile_backdrop_fill(&self, span: f64, render_z: u32, pinned: Option<&str>) -> Option<Color> {
        let policy = self.policy();
        let preset = self.style_preset();
        let land_on = self.role_on(MapLayerRole::LandCover);
        let band = policy.band_for_span(span, pinned);
        let gates = policy.layer_gates(band, span, render_z);
        match preset.technique {
            MapPaintTechnique::Colored => {
                let fine_land_canvas = gates.land_backdrop && land_on;
                (land_on && (fine_land_canvas || policy.band(band).land_wash)).then(|| weighted_opaque_fill(self.theme.land_fill, self.role_weight(MapLayerRole::LandCover)))
            }
            MapPaintTechnique::FigureGround => {
                let (ink, paper) = (self.theme.slot(preset.ink), self.theme.slot(preset.paper));
                let draw_buildings = gates.buildings && self.role_on(MapLayerRole::Building);
                let draw_land_backdrop = gates.land_backdrop && land_on;
                Some(if draw_buildings {
                    paper
                } else if draw_land_backdrop {
                    ink
                } else {
                    paper
                })
            }
        }
    }

    /// 🧊️ Vector-tile-layer entry point behind the frame cache: a pure pan (only `camera.x`/`camera.y`
    /// moved) reuses the last built `Scene` translated in screen space instead of re-walking decoded
    /// MVT geometry; any other change rebuilds via [`Self::append_vector_tiles`]. See
    /// `map_viewport::world_to_screen` — screen x and y both scale as `-camera.zoom * delta`, so the
    /// translate needs no extra axis flip beyond that shared sign. The screen-pinned backdrop wash is
    /// drawn fresh below, outside the cached/translated content, for the reason on
    /// [`Self::vector_tile_backdrop_fill`].
    fn append_vector_tiles_cached(&self, out: &mut Scene) {
        if vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            if let Some(render_z) = self.vector_tile_render_z() {
                let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
                if let Some(fill) = self.vector_tile_backdrop_fill(span, render_z, self.pinned_band_id()) {
                    self.append_viewport_fill(out, fill);
                }
            }
        }
        let key = self.vector_scene_cache_key();
        {
            let cache = self.vector_scene_cache.borrow();
            if let Some(cached) = cache.as_ref() {
                if cached.key == key {
                    self.touch_visible_vector_tiles();
                    let dx = -self.camera.zoom * (self.camera.x - cached.anchor_x);
                    let dy = -self.camera.zoom * (self.camera.y - cached.anchor_y);
                    let transform = if dx == 0.0 && dy == 0.0 { None } else { Some(Affine::IDENTITY.translate((dx, dy))) };
                    out.append(&cached.scene, transform);
                    return;
                }
            }
        }
        let mut fresh = Scene::new();
        self.append_vector_tiles(&mut fresh);
        self.vector_scene_rebuild_count.set(self.vector_scene_rebuild_count.get().wrapping_add(1));
        out.append(&fresh, None);
        *self.vector_scene_cache.borrow_mut() = Some(CachedVectorScene { scene: fresh, key, anchor_x: self.camera.x, anchor_y: self.camera.y });
    }

    fn append_vector_tiles(&self, scene: &mut Scene) {
        if !vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            return;
        }
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let forced_lod = self.pinned_band_id();

        let mut draw: Vec<(u32, u32, u32, &vector_tiles::VectorTile)> = Vec::new();
        for (key, tile) in &self.tiles.vector_tiles {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            let rect = projection::tile_world_rect(tz, tx, ty);
            if !self.tile_rect_intersects_viewport(rect) {
                continue;
            }
            self.tiles.touch_vector(key);
            draw.push((tz, tx, ty, tile));
        }
        if draw.is_empty() {
            return;
        }
        let render_z = draw.iter().map(|(tz, _, _, _)| *tz).max().unwrap_or(0);
        draw.retain(|(tz, _, _, _)| *tz == render_z);
        draw.sort_by_key(|(tz, tx, ty, _)| (*tz, *tx, *ty));

        let preset = self.style_preset();
        match preset.technique {
            MapPaintTechnique::Colored => self.append_vector_tiles_colored(scene, &draw, render_z, span, forced_lod),
            MapPaintTechnique::FigureGround => self.append_vector_tiles_figure(scene, &draw, render_z, span, forced_lod, MapFigurePalette { ink: self.theme.slot(preset.ink), paper: self.theme.slot(preset.paper) }),
        }
        if [MapLayerRole::RoadLabel, MapLayerRole::PointLabel, MapLayerRole::PlaceLabel].into_iter().any(|role| self.role_on(role)) {
            self.append_vector_tile_labels(scene, &draw, span, self.theme.slot(preset.label_fill), self.theme.slot(preset.label_halo));
        }
    }

    fn append_vector_tile_labels(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], span: f64, label_fill: Color, label_halo: Color) {
        struct LabelCandidate {
            label: String,
            screen: Point,
            rank: u16,
            px: f64,
        }

        let policy = self.policy();
        let band = policy.band_for_span(span, None);
        let role_px = |role: MapLayerRole| policy.label_px(band, span, self.role_weight(role));
        let px = [MapLayerRole::RoadLabel, MapLayerRole::PointLabel, MapLayerRole::PlaceLabel].into_iter().filter(|role| self.role_on(*role)).map(role_px).fold(0.0, f64::max);
        let mut candidates: Vec<LabelCandidate> = Vec::new();
        for (tz, tx, ty, tile) in draw {
            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| policy.draw_rank(l.name.as_str()));
            for layer in layers {
                let extent = layer.extent.max(1);
                let lname = layer.name.as_str();
                let Some(role @ (MapLayerRole::RoadLabel | MapLayerRole::PointLabel | MapLayerRole::PlaceLabel)) = policy.layer_role(lname) else {
                    continue;
                };
                if !self.role_on(role) {
                    continue;
                }
                let layer_px = role_px(role);
                for feat in &layer.features {
                    let Some(rank) = policy.label_gate_and_rank(lname, policy.feature_facts(&feat.properties).class, span) else {
                        continue;
                    };
                    let Some(label) = policy.feature_label(&feat.properties) else {
                        continue;
                    };
                    let anchor = vector_tiles::feature_label_anchor(feat, extent);
                    let s = self.tile_local_to_screen(*tz, *tx, *ty, extent, anchor.0, anchor.1);
                    candidates.push(LabelCandidate { label, screen: s, rank, px: layer_px });
                }
            }
        }
        if candidates.is_empty() {
            return;
        }
        candidates.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.label.cmp(&b.label)));
        let cell = (px * ui_styling::metrics::label::DECLUTTER_CELL_RATIO).clamp(ui_styling::metrics::label::DECLUTTER_CELL_MIN, ui_styling::metrics::label::DECLUTTER_CELL_MAX);
        let viewport_area = self.viewport.width.max(1) as f64 * self.viewport.height.max(1) as f64;
        let max_labels = (viewport_area / (cell * cell * 2.6)).round() as usize;
        let max_labels = max_labels.clamp(48, 140);
        let mut declutter = LabelDeclutter::for_viewport(&self.viewport, cell, max_labels);
        for candidate in candidates {
            if declutter.try_place(&candidate.label, candidate.screen, candidate.px) {
                text::append_label(scene, &candidate.label, candidate.screen, candidate.px, label_fill, label_halo);
            }
        }
    }

    fn append_vector_tiles_colored(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], render_z: u32, span: f64, pinned: Option<&str>) {
        let policy = self.policy();
        let land_weight = self.role_weight(MapLayerRole::LandCover);
        let building_weight = self.role_weight(MapLayerRole::Building);
        let land_fill = weighted_opaque_fill(self.theme.land_fill, land_weight);
        let border_stroke = self.theme.land_stroke;
        let road_stroke = self.theme.route_stroke;
        let region_stroke = self.theme.region_stroke;
        let water_fill = weighted_opaque_fill(self.theme.region_fill, self.role_weight(MapLayerRole::WaterArea));
        let waterway_fill = weighted_opaque_fill(self.theme.region_fill, self.role_weight(MapLayerRole::Waterway));
        let building_fill = color_with_alpha(land_fill, (220.0 * building_weight).clamp(32.0, 255.0) as u8);
        let line_scale = policy.line_scale(span);
        let band = policy.band_for_span(span, pinned);
        let band_row = policy.band(band);
        let road_lod_scale = band_row.road_stroke_scale;

        let land_on = self.role_on(MapLayerRole::LandCover);
        let green_weight = self.role_weight(MapLayerRole::GreenArea);
        let gates = policy.layer_gates(band, span, render_z);
        let fine_land_canvas = gates.land_backdrop && land_on;
        let draw_coastline = gates.coastline && self.role_on(MapLayerRole::Coastline);
        let park_fill = if fine_land_canvas { weighted_opaque_fill(self.theme.land_fill, (green_weight * 0.94).clamp(0.25, 1.0)) } else { weighted_opaque_fill(self.theme.region_fill, green_weight) };
        let draw_water = gates.water && self.role_on(MapLayerRole::WaterArea);
        let draw_waterways = gates.water && self.role_on(MapLayerRole::Waterway) && policy.waterway_visible(band);
        let draw_land = gates.land_cover && land_on;
        let draw_green = gates.land_cover && self.role_on(MapLayerRole::GreenArea);
        let draw_admin = gates.land_cover && self.role_on(MapLayerRole::AdminArea);
        let draw_buildings = gates.buildings && self.role_on(MapLayerRole::Building);
        let draw_roads = gates.roads && self.role_on(MapLayerRole::Road);
        let draw_borders = gates.boundaries && self.role_on(MapLayerRole::Boundary) && !fine_land_canvas;
        let (road_weight, border_weight, coastline_weight, waterway_weight) = (self.role_weight(MapLayerRole::Road), self.role_weight(MapLayerRole::Boundary), self.role_weight(MapLayerRole::Coastline), self.role_weight(MapLayerRole::Waterway));
        for (tz, tx, ty, tile) in draw {
            let tile_has_admin_areas = tile.layers.iter().any(|l| policy.layer_role(l.name.as_str()) == Some(MapLayerRole::AdminArea) && l.features.iter().any(|f| !f.rings.is_empty()));
            let draw_tile_admin_areas = draw_admin && !fine_land_canvas && tile_has_admin_areas;
            let draw_land_cover = draw_land && !fine_land_canvas && (!draw_tile_admin_areas || band_row.land_cover_over_admin_areas);
            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| policy.draw_rank(l.name.as_str()));

            for layer in layers {
                let extent = layer.extent.max(1);
                let role = policy.layer_role(layer.name.as_str());

                for feat in &layer.features {
                    match role {
                        Some(MapLayerRole::WaterArea) if draw_water => {
                            if !feat.rings.is_empty() && policy.water_area_visible(band, policy.feature_facts(&feat.properties).class) {
                                self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, water_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        Some(MapLayerRole::LandCover) if draw_land_cover => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, land_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        Some(MapLayerRole::GreenArea) if draw_green => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, park_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        Some(MapLayerRole::Building) if draw_buildings => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, building_fill, border_stroke, 0.5 * building_weight);
                            }
                        }
                        Some(MapLayerRole::Road) if draw_roads => {
                            let facts = policy.feature_facts(&feat.properties);
                            if policy.road_visible(facts.class, span, band) && !feat.lines.is_empty() {
                                let w = policy.stroke_width(MapLayerRole::Road, &facts, line_scale) * road_lod_scale * road_weight;
                                self.append_vector_tile_lines(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.lines, road_stroke, w);
                            }
                        }
                        Some(role @ (MapLayerRole::Boundary | MapLayerRole::Coastline)) if draw_borders || draw_coastline => {
                            let facts = policy.feature_facts(&feat.properties);
                            if facts.maritime || role == MapLayerRole::Coastline {
                                if draw_coastline && !feat.lines.is_empty() {
                                    let w = policy.stroke_width(MapLayerRole::Coastline, &facts, line_scale) * coastline_weight;
                                    self.append_vector_tile_lines(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.lines, border_stroke, w);
                                }
                                continue;
                            }
                            let Some(admin) = facts.admin_level else {
                                continue;
                            };
                            if policy.boundary_visible(admin, &gates) && !feat.lines.is_empty() {
                                let w = policy.stroke_width(MapLayerRole::Boundary, &facts, line_scale) * border_weight;
                                self.append_vector_tile_lines(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.lines, region_stroke, w);
                            }
                        }
                        Some(MapLayerRole::Waterway) if draw_waterways => {
                            if !feat.lines.is_empty() {
                                self.append_vector_tile_lines(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.lines, waterway_fill, (1.0 * line_scale * waterway_weight).clamp(0.5, 6.0));
                            }
                        }
                        Some(MapLayerRole::AdminArea) if draw_tile_admin_areas && !feat.rings.is_empty() => {
                            if policy.admin_area_holes(band) {
                                self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, land_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            } else {
                                self.append_vector_tile_polygon_filled_rings(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, land_fill);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn append_vector_tiles_figure(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], render_z: u32, span: f64, pinned: Option<&str>, palette: MapFigurePalette) {
        let MapFigurePalette { ink, paper } = palette;
        let policy = self.policy();
        let transparent_stroke = Color::from_rgba8(0, 0, 0, 0);
        let band = policy.band_for_span(span, pinned);
        let gates = policy.layer_gates(band, span, render_z);
        let land_on = self.role_on(MapLayerRole::LandCover);
        let draw_land_backdrop = gates.land_backdrop && land_on;
        let draw_coastline = gates.coastline && self.role_on(MapLayerRole::Coastline);
        let draw_buildings = gates.buildings && self.role_on(MapLayerRole::Building);
        let draw_water = gates.water && self.role_on(MapLayerRole::WaterArea);
        let draw_land = gates.land_cover && land_on;
        let draw_green = gates.land_cover && self.role_on(MapLayerRole::GreenArea);
        let draw_admin_areas = gates.land_cover && self.role_on(MapLayerRole::AdminArea) && !draw_land_backdrop;

        for (tz, tx, ty, tile) in draw {
            let has_admin_areas = tile.layers.iter().any(|l| policy.layer_role(l.name.as_str()) == Some(MapLayerRole::AdminArea) && l.features.iter().any(|f| !f.rings.is_empty()));
            let use_land_mass_silhouette = draw_land_backdrop || (draw_coastline && !has_admin_areas && draw_water && !draw_buildings);

            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| policy.draw_rank(l.name.as_str()));

            if draw_buildings {
                for layer in &layers {
                    if policy.layer_role(layer.name.as_str()) != Some(MapLayerRole::Building) {
                        continue;
                    }
                    let extent = layer.extent.max(1);
                    for feat in &layer.features {
                        if !feat.rings.is_empty() {
                            self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, ink, transparent_stroke, 0.0);
                        }
                    }
                }
                continue;
            }

            if use_land_mass_silhouette {
                self.append_vector_tile_land_backdrop(scene, *tz, *tx, *ty, ink);
            }

            for layer in &layers {
                let extent = layer.extent.max(1);
                let role = policy.layer_role(layer.name.as_str());
                for feat in &layer.features {
                    match role {
                        Some(role @ (MapLayerRole::LandCover | MapLayerRole::GreenArea)) if !use_land_mass_silhouette && (if role == MapLayerRole::LandCover { draw_land } else { draw_green }) => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon_rings_nonzero(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, ink);
                            }
                        }
                        Some(MapLayerRole::AdminArea) if draw_admin_areas => {
                            if !feat.rings.is_empty() {
                                if policy.admin_area_holes(band) {
                                    self.append_vector_tile_polygon_rings_nonzero(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, ink);
                                } else {
                                    self.append_vector_tile_polygon_filled_rings(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, ink);
                                }
                            }
                        }
                        Some(MapLayerRole::WaterArea) if draw_water && !feat.rings.is_empty() && policy.water_area_visible(band, policy.feature_facts(&feat.properties).class) => {
                            self.append_vector_tile_polygon(scene, VectorTileCoordinates { z: *tz, x: *tx, y: *ty, extent }, &feat.rings, paper, transparent_stroke, 0.0);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn append_tiles(&self, scene: &mut Scene) {
        if !self.layer_visible(&MapLayerTarget::Raster) {
            return;
        }
        let mut draw: Vec<(u32, u32, u32, Arc<RasterImage>)> = Vec::new();
        for (key, img) in &self.tiles.tile_images {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            let rect = projection::tile_world_rect(tz, tx, ty);
            if !self.tile_rect_intersects_viewport(rect) {
                continue;
            }
            self.tiles.touch_raster(key);
            draw.push((tz, tx, ty, img.clone()));
        }
        draw.sort_by_key(|(z, x, y, _)| (*z, *x, *y));
        for (tz, tx, ty, img) in draw {
            let rect = projection::tile_world_rect(tz, tx, ty);
            let aff = self.tile_raster_affine(rect, img.width(), img.height());
            raster::draw_image_arc(scene, &img, aff);
        }
    }

    /// 🕹️ Whether one overlay feature is selected or hovered under its layer's granularity.
    fn overlay_marks(&self, layer: &MapOverlayLayer, id: &str) -> (bool, bool) {
        let Some(granularity) = layer.granularity.as_deref() else {
            return (false, false);
        };
        let selected = self.selected.get(granularity).is_some_and(|ids| ids.contains(id));
        let hovered = self.hovered_kind.as_deref() == Some(granularity) && self.hovered_id.as_deref() == Some(id);
        (selected, hovered)
    }

    /// 🗺️ Every overlay layer the policy declares, in declaration order, each behind its layer toggle.
    fn append_overlays(&self, scene: &mut Scene) {
        for layer in self.policy().overlay_layers() {
            if !self.layer_visible(&MapLayerTarget::Overlay(layer.id.clone())) {
                continue;
            }
            match layer.geometry {
                MapOverlayGeometry::Area => self.append_overlay_areas(scene, layer),
                MapOverlayGeometry::Line => self.append_overlay_lines(scene, layer),
                MapOverlayGeometry::Point => self.append_overlay_points(scene, layer),
            }
        }
    }

    fn append_overlay_areas(&self, scene: &mut Scene, layer: &MapOverlayLayer) {
        let fill = self.theme.slot(layer.fill);
        let stroke = self.theme.slot(layer.stroke);
        let width = layer.line_width * self.layer_weight(&MapLayerTarget::Overlay(layer.id.clone()));
        let jump = self.screen_segment_jump_limit();
        for feature in self.overlays.features(&layer.id) {
            if feature.coordinates.len() < 3 {
                continue;
            }
            let mut path = BezPath::new();
            let ring: Vec<(f64, f64)> = feature
                .coordinates
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    (w.x, w.y)
                })
                .collect();
            Self::append_screen_ring(&mut path, &ring, |wx, wy| map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy)), jump);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
            scene.stroke(&Stroke::new(width), Affine::IDENTITY, stroke, None, &path);
        }
    }

    fn append_overlay_lines(&self, scene: &mut Scene, layer: &MapOverlayLayer) {
        let stroke_color = self.theme.slot(layer.stroke);
        let selection_color = self.theme.selection_stroke;
        let hover_color = self.theme.hover_stroke;
        let weight = self.layer_weight(&MapLayerTarget::Overlay(layer.id.clone()));
        for feature in self.overlays.features(&layer.id) {
            if feature.coordinates.len() < 2 {
                continue;
            }
            let (selected, hovered) = self.overlay_marks(layer, &feature.id);
            let mut path = BezPath::new();
            for (i, [lon, lat]) in feature.coordinates.iter().enumerate() {
                let w = projection::lonlat_to_world(*lon, *lat);
                let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                if i == 0 {
                    path.move_to(s);
                } else {
                    path.line_to(s);
                }
            }
            let width = feature.width.unwrap_or(layer.line_width) * weight;
            if selected || hovered {
                let halo = if selected { selection_color } else { hover_color };
                scene.stroke(&Stroke::new(width * 2.4), Affine::IDENTITY, halo, None, &path);
            }
            let color = if selected {
                selection_color
            } else if hovered {
                hover_color
            } else {
                stroke_color
            };
            scene.stroke(&Stroke::new(width), Affine::IDENTITY, color, None, &path);
        }
    }

    fn append_overlay_points(&self, scene: &mut Scene, layer: &MapOverlayLayer) {
        let label_fill = self.theme.label_fill;
        let label_halo = self.theme.label_halo;
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let policy = self.policy();
        let weight = self.layer_weight(&MapLayerTarget::Overlay(layer.id.clone()));
        let labels = MapLayerTarget::OverlayLabels(layer.id.clone());
        let labels_on = self.layer_visible(&labels);
        let label_px = policy.label_px(policy.band_for_span(span, None), span, self.layer_weight(&labels));
        for feature in self.overlays.features(&layer.id) {
            let Some([lon, lat]) = feature.coordinates.first().copied() else {
                continue;
            };
            let (selected, hovered) = self.overlay_marks(layer, &feature.id);
            let fill_slot = feature.style.as_deref().and_then(|style| layer.style_fills.iter().find(|row| row.style == style)).map_or(layer.fill, |row| row.fill);
            let fill = self.theme.slot(fill_slot);
            let stroke = if selected {
                self.theme.selection_stroke
            } else if hovered {
                self.theme.hover_stroke
            } else {
                self.theme.slot(layer.stroke)
            };
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, projection::lonlat_to_world(lon, lat));
            let r = layer.point_radius * weight;
            let circle = Circle::new(s, r);
            if selected || hovered {
                let halo = Circle::new(s, r * 1.75);
                let halo_color = if selected { self.theme.selection_stroke } else { self.theme.hover_stroke };
                scene.fill(FillRule::NonZero, Affine::IDENTITY, halo_color, None, &halo);
            }
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            let stroke_width = if selected || hovered { layer.point_stroke * weight * 1.5 } else { layer.point_stroke * weight };
            scene.stroke(&Stroke::new(stroke_width), Affine::IDENTITY, stroke, None, &circle);
            if labels_on {
                if let Some(label) = feature.label.as_deref().map(str::trim).filter(|t| !t.is_empty()) {
                    let anchor = Point::new(s.x, s.y - r - layer.label_offset_px);
                    text::append_label(scene, label, anchor, label_px, label_fill, label_halo);
                }
            }
        }
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut scene = Scene::new();
        match self.render_mode {
            MapTileMode::Image | MapTileMode::Combined => self.append_tiles(&mut scene),
            MapTileMode::Vector => {}
        }
        match self.render_mode {
            MapTileMode::Vector | MapTileMode::Combined => self.append_vector_tiles_cached(&mut scene),
            MapTileMode::Image => {}
        }
        self.append_overlays(&mut scene);
        scene
    }

    /// 📐️ Scales the logical viewport scene to the physical GPU surface (matches puzzle2d dpr handling).
    pub fn build_render_scene(&self) -> Scene {
        let inner = self.build_vector_scene();
        let scale = self.viewport.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            return inner;
        }
        let mut scene = Scene::new();
        scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
        scene
    }
}

impl canvas_content::CanvasContent for MapHost {
    fn build_scene(&self) -> Scene {
        self.build_render_scene()
    }

    fn clear_color(&self) -> Color {
        self.theme.surface_clear
    }
}
// #endregion 🔖️MapContent
// #endregion 🔖️HostScene
