
use super::MAX_VISIBLE_TILE_REQUESTS;
use super::canvas::camera::{Camera, Viewport};
use super::projection::{WORLD_HALF, default_world_camera, lonlat_to_world, tile_world_rect, world_to_lonlat};
use super::tiles::{self, visible_tiles};

fn test_png_1x1() -> Vec<u8> {
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    let mut buf = Vec::new();
    let enc = PngEncoder::new(&mut buf);
    enc.write_image(&[0, 0, 0, 255], 1, 1, ColorType::Rgba8.into()).expect("png");
    buf
}

/// 🧪️ A one-feature "landcover" vector tile — drawn unconditionally at world/continent LOD, so
/// it exercises the real `tile_local_to_screen` → `world_to_screen` path the cache must translate.
fn synthetic_land_tile() -> super::vector_tiles::VectorTile {
    use super::vector_tiles::{GeomType, VectorFeature, VectorLayer, VectorTile};
    VectorTile {
        layers: vec![VectorLayer {
            name: "landcover".to_string(),
            extent: super::vector_tiles::DEFAULT_MVT_EXTENT,
            features: vec![VectorFeature { id: None, geom_type: GeomType::Polygon, rings: vec![vec![(500.0, 500.0), (3500.0, 500.0), (3500.0, 3500.0), (500.0, 3500.0)]], lines: vec![], points: vec![], properties: std::collections::BTreeMap::new() }],
        }],
    }
}

#[test]
fn lonlat_world_round_trip() {
    let w = lonlat_to_world(8.5, 47.4);
    let (lon, lat) = world_to_lonlat(w.x, w.y);
    assert!((lon - 8.5).abs() < 0.01);
    assert!((lat - 47.4).abs() < 0.5);
}

#[test]
fn default_world_camera_fits_extent() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let cam = default_world_camera(&viewport);
    assert!(cam.zoom > 0.0);
    assert_eq!(cam.x, 0.0);
    assert_eq!(cam.y, 0.0);
}

#[test]
fn visible_tiles_at_world_view() {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 200.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let tiles = visible_tiles(&camera, &viewport, 2);
    assert!(!tiles.is_empty());
    assert!(tiles.len() < 256, "world view must not enumerate excessive tiles");
    let _ = WORLD_HALF;
}

#[test]
fn typed_visible_tile_cursor_matches_legacy_order_and_resumes_one_item_at_a_time() {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 200.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let legacy = visible_tiles(&camera, &viewport, 3);
    let mut cursor = tiles::visible_tile_cursor(&camera, &viewport, 3);
    assert_eq!(cursor.remaining(), legacy.len());
    let mut typed = Vec::new();
    while let Some(tile) = cursor.peek() {
        typed.push((tile.z, tile.x, tile.y));
        assert!(cursor.advance());
    }
    assert_eq!(typed, legacy);
    assert_eq!(cursor.remaining(), 0);
    assert!(!cursor.advance());
}

#[test]
fn map_set_size_reclamps_cover_zoom_when_viewport_grows() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let zoom = host.camera.zoom;
    host.set_size(1024, 768, 1.0);
    assert!(host.camera.zoom >= zoom - 1e-6);
    assert_viewport_corners_inside_world(&host);
}

#[test]
fn map_pan_and_zoom_stay_inside_world() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_camera(4.0, 4.0, super::MAP_CAMERA_ZOOM_MIN);
    assert_viewport_corners_inside_world(&host);
    host.wheel_screen(400.0, 300.0, -5000.0);
    assert_viewport_corners_inside_world(&host);
    host.pointer_down_screen(400.0, 300.0, 0);
    host.pointer_move_screen(40.0, 30.0);
    host.pointer_up_screen(40.0, 30.0);
    assert_viewport_corners_inside_world(&host);
}

fn assert_viewport_corners_inside_world(host: &super::MapHost) {
    let w = host.viewport.width as f64;
    let h = host.viewport.height as f64;
    for (sx, sy) in [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)] {
        let p = super::map_viewport::screen_to_world(&host.camera, &host.viewport, super::Point::new(sx, sy));
        assert!(p.x >= -WORLD_HALF - 1e-8 && p.x <= WORLD_HALF + 1e-8, "x out of world at ({sx},{sy}): {}", p.x);
        assert!(p.y >= -WORLD_HALF - 1e-8 && p.y <= WORLD_HALF + 1e-8, "y out of world at ({sx},{sy}): {}", p.y);
    }
}

#[test]
fn map_camera_zoom_not_clamped_to_puzzle_scale() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let fit_zoom = host.camera.zoom;
    assert!(fit_zoom > super::MAP_CAMERA_ZOOM_MIN);
    host.set_camera(0.0, 0.0, fit_zoom);
    assert!((host.camera.zoom - fit_zoom).abs() < 1e-6);
}

#[test]
fn map_camera_max_zoom_reaches_street_longitude_span() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let camera = Camera { x: 0.0, y: 0.0, zoom: super::MAP_CAMERA_ZOOM_MAX };
    let span = super::viewport_lon_span_degrees(&camera, &viewport);
    assert!(span < 0.01, "max camera zoom must reach sub-city scale (span={span:.6}°)");
    assert!(span < super::GIS_MAP_LOD_MAX_SPAN_DEG[6], "max zoom should enter street LOD band (span={span:.6}°)");
}

#[test]
fn map_lod_picks_bounded_tile_zoom_at_world_fit() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let camera = default_world_camera(&viewport);
    let z = tiles::pick_zoom(&camera, &viewport, None);
    assert_eq!(z, 0, "world-fit automatic LOD must use world-band raster tiles (got {z})");
    let tiles = visible_tiles(&camera, &viewport, z);
    assert!(tiles.len() < MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn visible_tiles_json_bounded_for_default_host() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    assert!(!raw.is_empty());
    assert!(raw.len() < 512);
}

#[test]
fn pick_vector_tile_zoom_clamped_to_openfreemap_max() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(host.pick_vector_tile_zoom() <= super::vector_tiles::MAP_VECTOR_TILE_MAX_Z);
}

#[test]
fn visible_vector_tiles_overzoom_at_max_camera_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(super::vector_tiles_available_at_camera_zoom(super::MAP_CAMERA_ZOOM_MAX));
    let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
    let z = host.pick_vector_tile_zoom();
    assert!(z <= super::vector_tiles::MAP_VECTOR_TILE_MAX_Z);
    assert!(z <= super::vector_tiles::max_tile_z_for_span(span));
    assert!(z <= host.pick_raster_tile_zoom());
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}

#[test]
fn map_lod_resolves_from_visible_longitude_span() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let camera = default_world_camera(&viewport);
    assert_eq!(super::resolve_map_lod_index_from_span(super::viewport_lon_span_degrees(&camera, &viewport)), 0, "default world fit should be world LOD");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
    let idx = super::resolve_map_lod_index_from_span(span);
    assert!(idx >= 2, "zoomed europe view should reach at least country LOD (span={span:.1}°, idx={idx})");
}

#[test]
fn ideal_tile_z_rises_when_viewport_span_shrinks() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let world = default_world_camera(&viewport);
    let zoomed = Camera { x: 0.0, y: 0.0, zoom: 1800.0 };
    assert!(super::ideal_tile_z_for_viewport(&zoomed, &viewport) > super::ideal_tile_z_for_viewport(&world, &viewport));
}

#[test]
fn region_and_city_road_strokes_stay_thin() {
    let region_scale = super::vector_tiles::vector_line_scale(8.0);
    let city_scale = super::vector_tiles::vector_line_scale(2.5);
    let district_scale = super::vector_tiles::vector_line_scale(0.2);
    assert!(region_scale <= 1.38, "region band line_scale should not exceed cap (got {region_scale})");
    assert!(city_scale <= 1.38, "city band line_scale should not exceed cap (got {city_scale})");
    let region_lod = super::vector_tiles::transportation_stroke_lod_scale(8.0, None);
    let city_lod = super::vector_tiles::transportation_stroke_lod_scale(2.5, None);
    assert!((region_lod - 0.4).abs() < f64::EPSILON);
    assert!((city_lod - 0.3).abs() < f64::EPSILON);
    let primary_region = super::vector_tiles::transportation_stroke_width("primary", region_scale) * region_lod;
    let tertiary_city = super::vector_tiles::transportation_stroke_width("tertiary", city_scale) * city_lod;
    let residential_city = super::vector_tiles::transportation_stroke_width("residential", city_scale) * city_lod;
    let minor_district = super::vector_tiles::transportation_stroke_width("minor", district_scale);
    assert!(primary_region < 1.05, "primary roads at region zoom should stay under 1.05px (got {primary_region})");
    assert!(tertiary_city < 0.55, "tertiary roads at city zoom should stay under 0.55px (got {tertiary_city})");
    assert!(residential_city < 0.4, "residential roads at city zoom should stay under 0.4px (got {residential_city})");
    assert!(minor_district < 1.35, "minor roads at district zoom should stay under 1.35px (got {minor_district})");
}

#[test]
fn continental_span_hides_roads_and_caps_tile_z() {
    assert!(!super::vector_tiles::transportation_visible("motorway", 42.0, 4, None));
    assert!(super::vector_tiles::vector_detail_profile(42.0, 4, None).draw_landcover);
    assert_eq!(super::vector_tiles::max_tile_z_for_span(42.0), 5);
    let profile = super::vector_tiles::vector_detail_profile(42.0, 2, None);
    assert!(!profile.draw_boundary);
    assert!(profile.draw_coastline);
    assert!(!profile.draw_land_backdrop);
    assert!(profile.draw_landcover);
    assert!(!super::vector_tiles::boundary_visible(2, 42.0, 2, None));
    assert!(super::vector_tiles::boundary_visible(2, 20.0, 5, None));
    assert!(!super::vector_tiles::boundary_visible(4, 20.0, 5, None));
    assert!(profile.draw_water);
    assert!(!profile.draw_transportation);
}

#[test]
fn country_lod_hides_regional_boundaries() {
    let span = 20.0;
    assert_eq!(super::resolve_map_lod_index_from_span(span), 2);
    let profile = super::vector_tiles::vector_detail_profile(span, 7, None);
    assert!(profile.draw_boundary);
    assert!(profile.draw_land_backdrop);
    assert!(!profile.draw_landcover);
    assert_eq!(profile.max_admin_level, 2);
    assert!(super::vector_tiles::boundary_visible(2, span, 7, None));
    assert!(!super::vector_tiles::boundary_visible(4, span, 7, None));
    assert!(!super::vector_tiles::boundary_visible(6, span, 7, None));
    let forced = super::vector_tiles::vector_detail_profile(8.0, 10, Some("country"));
    assert_eq!(forced.max_admin_level, 2);
    assert!(super::vector_tiles::boundary_visible(2, 8.0, 10, Some("country")));
    assert!(!super::vector_tiles::boundary_visible(4, 8.0, 10, Some("country")));
    assert!(!super::vector_tiles::boundary_visible(6, 8.0, 10, Some("country")));
}

#[test]
fn region_lod_includes_country_and_state_boundaries() {
    let span = 8.0;
    assert_eq!(super::resolve_map_lod_index_from_span(span), 3);
    let profile = super::vector_tiles::vector_detail_profile(span, 10, None);
    assert!(profile.draw_landcover);
    assert!(profile.draw_transportation);
    assert_eq!(profile.max_admin_level, 6);
    assert!(super::vector_tiles::boundary_visible(2, span, 10, None));
    assert!(super::vector_tiles::boundary_visible(4, span, 10, None));
    assert!(super::vector_tiles::boundary_visible(6, span, 10, None));
    assert_eq!(super::vector_tiles::max_tile_z_for_span(span), 6);
    assert!(super::vector_tiles::transportation_visible("primary", span, 6, None));
    assert!(!super::vector_tiles::transportation_visible("secondary", span, 6, None));
    assert!(!super::vector_tiles::transportation_visible("tertiary", span, 6, None));
}

#[test]
fn city_lod_fills_tile_backdrop_to_hide_seams() {
    let span = 2.0;
    assert_eq!(super::resolve_map_lod_index_from_span(span), 4);
    let profile = super::vector_tiles::vector_detail_profile(span, 10, None);
    assert!(profile.draw_land_backdrop);
    assert!(profile.draw_landcover);
    assert!(profile.draw_transportation);
}

#[test]
fn region_lod_exceeds_country_detail() {
    let span = 8.0;
    let country = super::vector_tiles::vector_detail_profile(span, 10, Some("country"));
    let region = super::vector_tiles::vector_detail_profile(span, 10, Some("region"));
    assert!(!country.draw_landcover);
    assert!(region.draw_landcover);
    assert!(country.draw_land_backdrop);
    assert!(region.draw_land_backdrop);
    assert!(!country.draw_transportation);
    assert!(region.draw_transportation);
    assert_eq!(country.max_admin_level, 2);
    assert_eq!(region.max_admin_level, 6);
    assert!(super::vector_tiles::boundary_visible(2, span, 10, Some("region")));
    assert!(super::vector_tiles::boundary_visible(4, span, 10, Some("region")));
    assert!(!super::vector_tiles::boundary_visible(4, span, 10, Some("country")));
}

#[test]
fn country_span_vector_profile_matches_lod_band() {
    let span = 20.0;
    assert_eq!(super::resolve_map_lod_index_from_span(span), 2);
    let profile = super::vector_tiles::vector_detail_profile(span, 2, None);
    assert!(profile.draw_boundary);
    assert_eq!(super::vector_tiles::max_tile_z_for_span(span), 7);
    let coarse_tile = super::vector_tiles::vector_detail_profile(span, 1, None);
    assert_eq!(profile.draw_boundary, coarse_tile.draw_boundary);
}

#[test]
fn layer_visibility_json_round_trip() {
    let mut host = super::MapHost::new();
    assert!(host.layer_visibility.positions);
    host.set_layer_visibility_from_json(r#"{"positions":false,"routes":true}"#).expect("parse");
    assert!(!host.layer_visibility.positions);
    assert!(host.layer_visibility.routes);
    let parsed: super::MapLayerVisibility = serde_json::from_str(&host.layer_visibility_json()).expect("serialize");
    assert!(!parsed.positions);
}

#[test]
fn weight_slider_keys_follow_lod_and_render_mode() {
    let world_combined = super::map_layer_weight_slider_keys_at_lod("world", "combined");
    assert!(world_combined.contains(&"raster"));
    assert!(world_combined.contains(&"water"));
    assert!(!world_combined.contains(&"roads"));
    assert!(!world_combined.contains(&"buildings"));
    let street_vector = super::map_layer_weight_slider_keys_at_lod("street", "vector");
    assert!(!street_vector.contains(&"raster"));
    assert!(street_vector.contains(&"roads"));
    assert!(!street_vector.contains(&"buildings"));
    let building_combined = super::map_layer_weight_slider_keys_at_lod("building", "combined");
    assert!(building_combined.contains(&"buildings"));
    assert!(building_combined.contains(&"raster"));
    let image_only = super::map_layer_weight_slider_keys_at_lod("city", "image");
    assert!(image_only.contains(&"raster"));
    assert!(!image_only.contains(&"roads"));
}

#[test]
fn layer_stroke_scale_json_clamps_weights() {
    let mut host = super::MapHost::new();
    host.set_layer_stroke_scale_from_json(r#"{"roads":9,"water":0.1}"#).expect("parse");
    assert!((host.layer_stroke_scale.roads - super::MAP_LAYER_WEIGHT_MAX).abs() < f64::EPSILON);
    assert!((host.layer_stroke_scale.water - super::MAP_LAYER_WEIGHT_MIN).abs() < f64::EPSILON);
}

#[test]
fn pick_vector_tile_zoom_tracks_viewport_not_camera_lod_steps() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let z_world = host.pick_vector_tile_zoom();
    host.set_camera(0.0, 0.0, 6000.0);
    let z_zoomed = host.pick_vector_tile_zoom();
    assert!(z_zoomed > z_world, "vector tile z must rise when zooming in (world={z_world}, zoomed={z_zoomed})");
}

#[test]
fn pick_vector_tile_zoom_uses_span_cap_at_world_lod_while_raster_stays_coarse() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    assert_eq!(host.pick_raster_tile_zoom(), 0, "world raster stays at z0");
    assert_eq!(host.pick_vector_tile_zoom(), super::vector_tiles::max_tile_z_for_span(super::viewport_lon_span_degrees(&host.camera, &host.viewport,)), "world vector tiles must be finer than raster so countries/coastlines paint");
}

#[test]
fn forced_building_lod_bounds_tile_requests_at_world_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_lod_mode("building");
    let z = host.pick_raster_tile_zoom();
    assert!(z < super::GIS_MAP_LOD_TILE_Z[7], "forced building at world view must clamp tile z (got {z})");
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    assert!(raw.len() <= MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn forced_country_lod_allows_finer_tiles_when_zoomed_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_lod_mode("country");
    host.set_camera(0.0, 0.0, 6000.0);
    let z = host.pick_raster_tile_zoom();
    assert!(z > super::GIS_MAP_LOD_TILE_Z[2], "pinned country LOD must not cap tile z when zoomed in (got {z})");
    let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("json");
    assert_eq!(lod["id"], "country");
    assert_eq!(lod["mode"], "country");
}

#[test]
fn visible_vector_tiles_at_world_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    assert!(super::vector_tiles_available_at_camera_zoom(host.camera.zoom));
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}

#[test]
fn map_wheel_screen_keeps_world_under_cursor_with_flipped_y() {
    use super::canvas::camera::{Camera, Viewport};
    let mut camera = Camera { x: 0.15, y: -0.25, zoom: 320.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let sx = 220.0;
    let sy = 140.0;
    let before = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
    super::map_wheel_screen(&mut camera, &viewport, sx, sy, -80.0);
    let after = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
    assert!(camera.zoom > 320.0);
    assert!((after.x - before.x).abs() < 1e-9, "world x should stay under cursor");
    assert!((after.y - before.y).abs() < 1e-9, "world y should stay under cursor");
}

#[test]
fn tile_raster_affine_maps_image_north_up() {
    let mut host = super::MapHost::new();
    host.set_size(256, 256, 1.0);
    host.fit_world_camera();
    let rect = tile_world_rect(1, 0, 0);
    let nw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y1()));
    let sw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y0()));
    assert!(nw.y < sw.y, "north edge should be above south edge on screen");
    let aff = host.tile_raster_affine(rect, 256, 256);
    let coeffs = aff.as_coeffs();
    assert!(coeffs[0] > 0.0, "east should map to increasing screen x");
    assert!(coeffs[3] > 0.0, "image rows should map downward on screen");
}

#[test]
fn map_render_mode_parse() {
    assert_eq!(super::MapTileMode::parse("image"), super::MapTileMode::Image);
    assert_eq!(super::MapTileMode::parse("vector"), super::MapTileMode::Vector);
    assert_eq!(super::MapTileMode::parse("combined"), super::MapTileMode::Combined);
}

#[test]
fn map_vector_style_parse() {
    assert_eq!(super::MapVectorStyle::parse("colored"), super::MapVectorStyle::Colored);
    assert_eq!(super::MapVectorStyle::parse("figureGround"), super::MapVectorStyle::FigureGround);
    assert_eq!(super::MapVectorStyle::parse("invertedFigure"), super::MapVectorStyle::InvertedFigure);
    assert_eq!(super::MapVectorStyle::parse("unknown"), super::MapVectorStyle::Colored);
}

#[test]
fn sync_map_json_keeps_position_labels() {
    let mut host = super::MapHost::new();
    let json = r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#;
    host.sync_map_json(json).expect("descriptor");
    let pos = host.features.positions.get("zurich").expect("position");
    assert_eq!(pos.label.as_deref(), Some("Zürich"));
}

#[test]
fn sync_map_json_parses_rich_position_metadata() {
    let mut host = super::MapHost::new();
    let json = r#"{"positions":[{"id":"donor-1","lon":8.54,"lat":47.37,"name":"Donor site","kind":"donor","icon":"package","sourceUrl":"https://example.test/donor"}],"routes":[],"regions":[]}"#;
    host.sync_map_json(json).expect("descriptor");
    let pos = host.features.positions.get("donor-1").expect("position");
    assert_eq!(pos.name.as_deref(), Some("Donor site"));
    assert_eq!(pos.kind.as_deref(), Some("donor"));
    assert_eq!(pos.icon.as_deref(), Some("package"));
    assert_eq!(pos.source_url.as_deref(), Some("https://example.test/donor"));
}

#[test]
fn position_screen_json_projects_known_position() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.position_screen_json("zurich")).expect("json");
    assert!(raw.get("x").and_then(|v| v.as_f64()).is_some());
    assert!(raw.get("y").and_then(|v| v.as_f64()).is_some());
    assert_eq!(host.position_screen_json("missing"), "null");
}

#[test]
fn hit_test_feature_prefers_closest_target() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[{"id":"route-a","points":[[8.54,47.37],[8.55,47.38]],"stroke_width":2}],"regions":[]}"#).expect("descriptor");
    let screen: serde_json::Value = serde_json::from_str(&host.position_screen_json("zurich")).expect("json");
    let sx = screen["x"].as_f64().expect("x");
    let sy = screen["y"].as_f64().expect("y");
    let hit: serde_json::Value = serde_json::from_str(&host.hit_test_feature_json(sx, sy)).expect("hit");
    assert!(hit.get("kind").is_some());
    assert!(hit.get("id").is_some());
}

#[test]
fn features_in_rect_crossing_includes_intersecting_route() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[{"id":"route-a","points":[[8.54,47.37],[8.55,47.38]],"stroke_width":2}],"regions":[]}"#).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.features_in_rect_json(0.0, 0.0, 800.0, 600.0, true)).expect("json");
    assert!(raw["positions"].as_array().is_some_and(|rows| !rows.is_empty()));
    assert!(raw["routes"].as_array().is_some_and(|rows| !rows.is_empty()));
}

#[test]
fn sync_interaction_updates_host_state_scoped_to_active_granularity() {
    let mut host = super::MapHost::new();
    host.sync_interaction("position", &["a".to_string()], Some("a"));
    assert!(host.selected_positions.contains("a"));
    assert!(host.selected_routes.is_empty());
    assert_eq!(host.hovered_kind.as_deref(), Some("position"));
    assert_eq!(host.hovered_id.as_deref(), Some("a"));

    host.sync_interaction("route", &["b".to_string()], None);
    assert!(host.selected_positions.is_empty(), "switching granularity clears the other kind's ids");
    assert!(host.selected_routes.contains("b"));
    assert!(host.hovered_kind.is_none());
    assert!(host.hovered_id.is_none());
}

#[test]
fn pointer_up_emits_camera_after_middle_button_pan() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.pointer_down_screen(100.0, 100.0, 1);
    host.pointer_move_screen(180.0, 140.0);
    host.pointer_up_screen(180.0, 140.0);
    let events: Vec<serde_json::Value> = serde_json::from_str(&host.drain_events_json()).expect("events");
    assert!(events.iter().any(|row| row["type"] == "camera"));
}

#[test]
fn position_labels_build_non_empty_scene() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#).expect("descriptor");
    let scene = host.build_vector_scene();
    assert!(!scene.is_empty());
}

#[test]
fn set_map_theme_from_json_updates_surface_clear() {
    let mut host = super::MapHost::new();
    let json = r#"{"surfaceClear":[1,2,3,255]}"#;
    host.set_map_theme_from_json(json).expect("theme json");
    assert_eq!(host.theme.surface_clear.to_rgba8(), super::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
}

#[test]
fn set_map_theme_from_json_zeros_land_stroke_alpha() {
    let mut host = super::MapHost::new();
    host.set_map_theme_from_json(r#"{"landStroke":[51,64,65,107]}"#).expect("theme json");
    assert_eq!(host.theme.land_stroke.to_rgba8().a, 0);
}

#[test]
fn build_render_scene_scales_for_device_pixel_ratio() {
    let mut host = super::MapHost::new();
    host.set_size(400, 300, 2.0);
    host.fit_world_camera();
    let logical = host.build_vector_scene();
    let scaled = host.build_render_scene();
    assert!(scaled.path_count() >= logical.path_count());
}

#[test]
fn build_vector_scene_respects_render_mode() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    let _ = host.build_vector_scene();
    host.set_render_mode("image");
    let _ = host.build_vector_scene();
}

#[test]
fn build_vector_scene_respects_vector_style() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    let _ = host.build_vector_scene();
    host.set_vector_style("invertedFigure");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_style_str(), "invertedFigure");
}

/// 🎣️ Seeds a vector host whose camera sits *off* a tile boundary. Straight from
/// `fit_world_camera` the viewport edge lands exactly on one, where `visible_tile_cursor`'s
/// `ceil - 1` correctly excludes the zero-overlap tile — so any nudge in either direction pulls a
/// whole extra row in (measured live: 48 → 56 tiles for a 0.2 px pan) and legitimately invalidates
/// the cache. A sub-tile pan only stays within one visible set away from that boundary.
fn vector_cache_host_off_tile_boundary() -> super::MapHost {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    host.camera.y -= 0.017;
    host
}

#[test]
fn vector_scene_cache_hit_reuses_translated_scene_on_pure_pan() {
    let mut host = vector_cache_host_off_tile_boundary();
    let first = host.build_vector_scene();
    assert!(!first.is_empty());
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    let anchor_x = host.camera.x;
    let anchor_y = host.camera.y;
    host.camera.x = anchor_x;
    host.camera.y = anchor_y - 0.002;
    let panned = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a sub-tile pan must reuse the cache, not rebuild");
    assert_eq!(panned.path_count(), first.path_count());

    let mut rebuilt = vector_cache_host_off_tile_boundary();
    rebuilt.camera.x = anchor_x;
    rebuilt.camera.y = anchor_y - 0.002;
    let forced = rebuilt.build_vector_scene();
    assert_eq!(rebuilt.vector_scene_rebuild_count.get(), 1);
    assert_eq!(panned.path_count(), forced.path_count(), "cached-translate pan must match a forced rebuild at the same camera");
}

#[test]
fn vector_scene_cache_rebuilds_when_a_pan_changes_the_visible_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let before = host.visible_vector_tiles_revision();
    host.camera.y += 0.017;
    assert_ne!(host.visible_vector_tiles_revision(), before, "this pan must cross a tile boundary for the test to mean anything");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a pan that changes the visible tile set must rebuild, never translate a stale set");
}

#[test]
fn vector_scene_cache_invalidates_on_zoom_change() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.camera.zoom *= 1.001;
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a zoom change must not take the translate path");
}

#[test]
fn vector_scene_cache_invalidates_on_viewport_resize() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_size(801, 600, 1.0);
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a viewport resize must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_new_tile_upload() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.upload_vector_tile(1, 0, 0, &[]).expect("upload");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a newly uploaded vector tile must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_vector_style_change() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_vector_style("figureGround");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a vector-style change must invalidate the cache");
}

/// 🚫️ The vector-tile layer must NOT key on interaction. Selection and hover are painted by the
/// regions/routes/positions layers, which are rebuilt every frame outside this cache, so keying on
/// `interaction_revision` bought nothing — and because `pointer_move_screen`/`set_camera` bump that
/// revision on every frame of a pan, it silently invalidated the cache once per frame and pinned a
/// drag at the uncached cost (measured live: 35 ms/frame with it, 2.7 ms without).
#[test]
fn vector_scene_cache_survives_interaction_change_because_selection_draws_outside_it() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.sync_interaction("position", &["a".to_string()], Some("a"));
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "selection/hover must not rebuild the vector-tile cache");
}

/// 🖐️ The whole point of the cache: a drag must reuse it on every frame that does not cross a tile
/// boundary, even though each `pointer_move_screen` bumps `interaction_revision`.
#[test]
fn vector_scene_cache_survives_a_drag_that_stays_within_one_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let revision = host.visible_vector_tiles_revision();

    host.pointer_down_screen(400.0, 300.0, 1);
    for step in 1..=8 {
        host.pointer_move_screen(400.0, 300.0 + f64::from(step));
        let _ = host.build_vector_scene();
    }
    host.pointer_up_screen(400.0, 308.0);

    assert_eq!(host.visible_vector_tiles_revision(), revision, "this drag must stay inside one tile set for the test to mean anything");
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a drag within one tile set must never rebuild the vector scene");
}

#[test]
fn parse_tile_key_round_trips_tile_key() {
    let key = tiles::tile_key(3, 5, 7);
    assert_eq!(tiles::parse_tile_key(&key), Some((3, 5, 7)));
    assert_eq!(tiles::parse_tile_key("bad"), None);
    assert_eq!(tiles::parse_tile_key("1/2"), None);
}

#[test]
fn tile_retention_keeps_raster_ancestor_after_zoom_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let png = test_png_1x1();
    host.upload_tile(0, 0, 0, &png).expect("upload");
    assert!(host.tiles.tile_images.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 6000.0);
    let z_fine = host.pick_raster_tile_zoom();
    assert!(z_fine >= 4, "zoomed camera should request finer raster tiles (got {z_fine})");
    host.prepare_visible_tiles();
    assert!(host.tiles.tile_images.contains_key("0/0/0"), "ancestor tile must survive zoom-level change for pyramid fallback");
}

#[test]
fn tile_rect_intersects_viewport_for_on_and_off_screen_tiles() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 3200.0);
    let z = host.pick_raster_tile_zoom();
    let visible = visible_tiles(&host.camera, &host.viewport, z);
    let (tz, tx, ty) = visible[0];
    let on_screen = tile_world_rect(tz, tx, ty);
    assert!(host.tile_rect_intersects_viewport(on_screen));
    let off_screen = super::Rect::new(4.0, 4.0, 5.0, 5.0);
    assert!(!host.tile_rect_intersects_viewport(off_screen));
}

#[test]
fn tile_retention_keeps_vector_ancestor_after_zoom_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, &[]).expect("upload");
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 2500.0);
    let vz = host.pick_vector_tile_zoom();
    assert!(vz >= 1, "zoomed camera should request finer vector tiles (got {vz})");
    host.prepare_visible_tiles();
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"), "ancestor vector tile must survive zoom-level change for pyramid fallback");
}

#[test]
fn decode_mvt_empty_bytes_yields_empty_tile() {
    let tile = super::vector_tiles::decode_mvt(&[]).expect("empty pbf");
    assert!(tile.layers.is_empty());
}

#[test]
#[ignore = "requires .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-2-2-1.pbf from demotiles"]
fn decode_demotile_fixture_has_named_layers() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-2-2-1.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
    assert!(!tile.layers.is_empty());
    assert!(tile.layers.iter().any(|l| !l.features.is_empty()));
    let names: Vec<&str> = tile.layers.iter().map(|l| l.name.as_str()).collect();
    assert!(names.contains(&"countries"));
    assert!(names.contains(&"centroids"));
}

#[test]
fn demotile_fixture_countries_use_multi_ring_polygons() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-2-2-1.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries layer");
    let multi = countries.features.iter().filter(|f| f.rings.len() > 1).count();
    assert!(multi > 0, "fixture should include multi-ring countries");
}

#[test]
fn fixture_linestrings_split_at_moveto() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES");
    let mut found = false;
    for entry in std::fs::read_dir(&dir).expect("fixture dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("pbf") {
            continue;
        }
        let bytes = std::fs::read(&path).expect("read pbf");
        let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
        for layer in &tile.layers {
            if layer.features.iter().any(|f| f.lines.len() > 1) {
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    assert!(found, "at least one fixture tile must decode multi-part line features");
}

#[test]
fn demotile_z5_has_countries_and_centroids() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
    let centroids = tile.layers.iter().find(|l| l.name == "centroids").expect("centroids");
    assert!(countries.features.len() >= 10);
    assert!(centroids.features.len() >= 5);
    assert!(centroids.features.iter().any(|f| super::vector_tiles::feature_label(&f.properties).is_some()));
}

#[test]
fn demotile_z0_has_many_country_features() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
    assert!(countries.features.len() >= 50, "world tile should include many countries");
}

#[test]
fn figure_world_tile_paints_many_countries() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    host.set_lod_mode("world");
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, &bytes).expect("vector tile");
    let scene = host.build_vector_scene();
    assert!(!scene.is_empty());
}

#[test]
fn colored_world_tile_paints_land_over_water_backdrop() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("colored");
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, &bytes).expect("vector tile");
    let scene = host.build_vector_scene();
    assert!(scene.path_count() > 4, "colored world LOD must paint country landmasses, not only the water backdrop");
}

#[test]
fn figure_country_lod_uses_land_mass_backdrop() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-3-4-2.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    host.set_lod_mode("country");
    host.fit_world_camera();
    host.upload_vector_tile(3, 4, 2, &bytes).expect("vector tile");
    let scene = host.build_vector_scene();
    assert!(!scene.is_empty());
}

fn zoom_host_to_representative_lod(host: &mut super::MapHost, lod_id: &str) {
    let idx = super::GIS_MAP_LOD_SCALE.index_of(lod_id).expect("lod id");
    let target = super::representative_viewport_span_for_lod(idx);
    host.fit_world_camera();
    let mut lo = host.camera.zoom;
    let mut hi = super::MAP_CAMERA_ZOOM_MAX;
    for _ in 0..48 {
        let mid = (lo + hi) * 0.5;
        host.set_camera(0.0, 0.0, mid);
        let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
        if span > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    host.set_camera(0.0, 0.0, hi);
}

fn zoom_host_over_tile(host: &mut super::MapHost, lod_id: &str, tz: u32, tx: u32, ty: u32) {
    zoom_host_to_representative_lod(host, lod_id);
    let rect = tile_world_rect(tz, tx, ty);
    let cx = (rect.x0() + rect.x1()) * 0.5;
    let cy = (rect.y0() + rect.y1()) * 0.5;
    host.set_camera(cx, -cy, host.camera.zoom);
}

#[test]
fn transportation_name_visible_is_stricter_than_road_geometry_at_city_span() {
    let v = super::vector_tiles::transportation_name_visible;
    assert!(v("primary", 2.0));
    assert!(!v("secondary", 2.0));
    assert!(!v("street", 2.0));
    assert!(v("secondary", 0.8));
}

#[test]
fn vector_label_px_scales_with_span_inside_city_band() {
    let wide = super::vector_tiles::vector_label_px(2.0, 1.0);
    let narrow = super::vector_tiles::vector_label_px(1.3, 1.0);
    assert!(narrow > wide, "labels should grow when zooming in (span shrinks): {wide} vs {narrow}");
    assert!((wide - 12.5 * 4.0 / 2.0).abs() < 1e-6);
}

#[test]
fn poi_labels_hidden_until_district_span() {
    let v = super::vector_tiles::poi_label_visible;
    assert!(!v(2.0));
    assert!(v(0.3));
}

#[test]
fn place_label_visible_covers_admin_document() {
    let v = super::vector_tiles::place_label_visible;
    assert!(v("", 20.0));
    assert!(!v("", 8.0));
    assert!(v("continent", 50.0));
    assert!(!v("continent", 20.0));
    assert!(v("country", 20.0));
    assert!(!v("country", 8.0));
    assert!(v("state", 8.0));
    assert!(v("province", 8.0));
    assert!(v("city", 2.0));
    assert!(v("town", 0.8));
    assert!(v("village", 0.2));
    assert!(v("suburb", 0.2));
    assert!(v("quarter", 0.05));
    assert!(v("neighbourhood", 0.05));
}

#[test]
fn label_camera_setup_intersects_fixture_tile() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_lod_mode("country");
    zoom_host_over_tile(&mut host, "country", 5, 17, 11);
    host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
    let rect = tile_world_rect(5, 17, 11);
    let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
    assert!(host.tile_rect_intersects_viewport(rect), "tile must intersect (span={span})");
    let scene = host.build_vector_scene();
    assert!(!scene.is_empty(), "fixture tile should paint geometry (span={span})");
}

#[test]
fn figure_ground_labels_increase_scene_when_enabled() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    host.set_lod_mode("country");
    zoom_host_over_tile(&mut host, "country", 5, 17, 11);
    host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
    host.set_layer_visibility_from_json(r#"{"labels":false}"#).expect("labels off");
    let without = host.build_vector_scene().path_count();
    host.set_layer_visibility_from_json(r#"{"labels":true}"#).expect("labels on");
    let with_labels = host.build_vector_scene().path_count();
    assert!(with_labels > without, "figure-ground labels should add glyph paths (with={with_labels}, without={without})");
}

#[test]
fn colored_vector_labels_increase_scene_when_enabled() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
    let bytes = std::fs::read(path).expect("fixture pbf");
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("colored");
    host.set_lod_mode("country");
    zoom_host_over_tile(&mut host, "country", 5, 17, 11);
    host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
    host.set_layer_visibility_from_json(r#"{"labels":false}"#).expect("labels off");
    let without = host.build_vector_scene().path_count();
    host.set_layer_visibility_from_json(r#"{"labels":true}"#).expect("labels on");
    let with_labels = host.build_vector_scene().path_count();
    assert!(with_labels > without, "colored vector labels should add glyph paths (with={with_labels}, without={without})");
}

#[test]
fn map_point_segment_distance_zero_length_segment_uses_point_distance() {
    let d = super::map_point_segment_distance(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
    assert!((d - 5.0).abs() < 1e-9);
}

#[test]
fn interaction_plan_matches_direct_wheel_and_pan_semantics() {
    let mut direct = super::MapHost::new();
    let mut planned = super::MapHost::new();
    direct.set_size(800, 600, 1.0);
    planned.set_size(800, 600, 1.0);
    direct.wheel_screen(320.0, 240.0, -12.0);
    let wheel = planned.plan_interaction(super::MapInteractionIntent::Wheel { sx: 320.0, sy: 240.0, delta_y: -12.0 });
    assert!(planned.commit_interaction(wheel));
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));

    for intent in [super::MapInteractionIntent::PointerDown { sx: 100.0, sy: 120.0, button: 1 }, super::MapInteractionIntent::PointerMove { sx: 130.0, sy: 150.0 }, super::MapInteractionIntent::PointerUp { sx: 130.0, sy: 150.0 }] {
        let plan = planned.plan_interaction(intent);
        assert!(planned.commit_interaction(plan));
    }
    direct.pointer_down_screen(100.0, 120.0, 1);
    direct.pointer_move_screen(130.0, 150.0);
    direct.pointer_up_screen(130.0, 150.0);
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));
    assert_eq!(direct.drain_events_json(), planned.drain_events_json());
}

#[test]
fn stale_interaction_plan_never_mutates_replaced_camera() {
    let mut host = super::MapHost::new();
    let plan = host.plan_interaction(super::MapInteractionIntent::Wheel { sx: 10.0, sy: 20.0, delta_y: -1.0 });
    host.set_camera(42.0, -7.0, 2.0);
    let replacement = [host.camera.x, host.camera.y, host.camera.zoom];
    assert!(!host.commit_interaction(plan));
    assert_eq!([host.camera.x, host.camera.y, host.camera.zoom], replacement);
}

#[test]
fn map_point_in_polygon_rejects_degenerate_polygon() {
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[]));
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[super::Point::new(0.0, 0.0), super::Point::new(1.0, 1.0)]));
}

#[test]
fn map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside() {
    let line = [super::Point::new(-10.0, 5.0), super::Point::new(10.0, 5.0)];
    assert!(super::map_polyline_intersects_rect(&line, 0.0, 0.0, 4.0, 10.0));
    let miss = [super::Point::new(-10.0, 50.0), super::Point::new(10.0, 50.0)];
    assert!(!super::map_polyline_intersects_rect(&miss, 0.0, 0.0, 4.0, 10.0));
}

#[test]
fn map_polyline_intersects_polygon_detects_crossing_edge() {
    let square = [super::Point::new(0.0, 0.0), super::Point::new(10.0, 0.0), super::Point::new(10.0, 10.0), super::Point::new(0.0, 10.0)];
    let crossing = [super::Point::new(-5.0, 5.0), super::Point::new(15.0, 5.0)];
    assert!(super::map_polyline_intersects_polygon(&crossing, &square));
    let outside = [super::Point::new(-5.0, 50.0), super::Point::new(-1.0, 50.0)];
    assert!(!super::map_polyline_intersects_polygon(&outside, &square));
}

#[test]
fn features_in_polygon_json_invalid_json_returns_empty() {
    let host = super::MapHost::new();
    let out = host.features_in_polygon_json("not json", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["positions"].as_array().unwrap().is_empty());
    assert!(v["routes"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_requires_at_least_a_triangle() {
    let host = super::MapHost::new();
    let out = host.features_in_polygon_json("[[0.0,0.0],[1.0,1.0]]", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["positions"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_finds_enclosed_position() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.sync_map_json(r#"{"positions":[{"id":"p1","lon":0.0,"lat":0.0}]}"#).expect("sync");
    let s = host.position_screen_json("p1");
    let sv: serde_json::Value = serde_json::from_str(&s).expect("json");
    let (sx, sy) = (sv["x"].as_f64().unwrap(), sv["y"].as_f64().unwrap());
    let polygon = format!("[[{},{}],[{},{}],[{},{}]]", sx - 20.0, sy - 20.0, sx + 20.0, sy - 20.0, sx, sy + 20.0);
    let out = host.features_in_polygon_json(&polygon, true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert_eq!(v["positions"].as_array().unwrap().first().and_then(|x| x.as_str()), Some("p1"));
}

#[test]
fn feature_screen_json_covers_route_missing_and_unknown_kind() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.sync_map_json(r#"{"routes":[{"id":"r1","points":[[0.0,0.0],[1.0,1.0]]}]}"#).expect("sync");
    let route_json = host.feature_screen_json("route", "r1");
    assert_ne!(route_json, "null");
    assert_eq!(host.feature_screen_json("route", "missing"), "null");
    assert_eq!(host.feature_screen_json("bogus", "r1"), "null");
    host.sync_map_json(r#"{"routes":[{"id":"empty","points":[]}]}"#).expect("sync");
    assert_eq!(host.feature_screen_json("route", "empty"), "null");
}

#[test]
fn sync_interaction_replaces_previous_selection_within_active_granularity() {
    let mut host = super::MapHost::new();
    host.sync_interaction("position", &["p1".to_string(), "p2".to_string()], None);
    assert_eq!(host.selected_positions_json().len(), 2);
    host.sync_interaction("route", &["r1".to_string()], None);
    assert!(host.selected_positions_json().is_empty(), "switching to route clears positions");
    assert_eq!(host.selected_routes_json(), vec!["r1".to_string()]);
    host.sync_interaction("route", &[], None);
    assert!(host.selected_routes_json().is_empty());
}

#[test]
fn sync_interaction_none_clears_hover_and_some_sets_kind_and_id() {
    let mut host = super::MapHost::new();
    host.sync_interaction("position", &[], Some("p1"));
    assert_eq!(host.hovered_kind(), Some("position"));
    assert_eq!(host.hovered_id(), Some("p1"));
    host.sync_interaction("position", &[], None);
    assert_eq!(host.hovered_kind(), None);
    assert_eq!(host.hovered_id(), None);
}

#[test]
fn focus_feature_position_moves_camera_and_missing_id_returns_false() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.sync_map_json(r#"{"positions":[{"id":"p1","lon":10.0,"lat":20.0}]}"#).expect("sync");
    assert!(!host.focus_feature("position", "missing"));
    assert!(host.focus_feature("position", "p1"));
    let expected = lonlat_to_world(10.0, 20.0);
    assert!((host.camera.x - expected.x).abs() < 1e-9);
    assert!((host.camera.y - expected.y).abs() < 1e-9);
}

#[test]
fn focus_feature_route_requires_at_least_two_points_and_fits_bounds() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.sync_map_json(r#"{"routes":[{"id":"short","points":[[0.0,0.0]]},{"id":"r1","points":[[0.0,0.0],[5.0,5.0]]}]}"#).expect("sync");
    assert!(!host.focus_feature("route", "short"));
    assert!(host.focus_feature("route", "r1"));
    assert!(!host.focus_feature("route", "missing"));
}

#[test]
fn focus_feature_unknown_kind_returns_false() {
    let mut host = super::MapHost::new();
    assert!(!host.focus_feature("bogus", "anything"));
}

#[test]
fn has_tile_and_has_vector_tile_reflect_uploads() {
    let mut host = super::MapHost::new();
    assert!(!host.has_tile("0/0/0"));
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("upload png");
    assert!(host.has_tile("0/0/0"));
    assert!(!host.has_tile("1/0/0"));
    assert!(!host.has_vector_tile("0/0/0"));
}

#[test]
fn upload_tile_invalid_png_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_tile(0, 0, 0, b"not a png").is_err());
}

#[test]
fn upload_vector_tile_invalid_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_vector_tile(0, 0, 0, &[0x80]).is_err());
}

#[test]
fn upload_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("first upload decodes");
    let before = host.tiles.tile_images.get("0/0/0").cloned().expect("cached after first upload");
    let result = host.upload_tile(0, 0, 0, b"not a png");
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    let after = host.tiles.tile_images.get("0/0/0").cloned().expect("still cached after second upload");
    assert!(std::sync::Arc::ptr_eq(&before, &after), "cached image must be untouched by the skipped decode");
}

#[test]
fn upload_vector_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_vector_tile(0, 0, 0, &[]).expect("first upload decodes");
    let result = host.upload_vector_tile(0, 0, 0, &[0x80]);
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    assert!(host.has_vector_tile("0/0/0"), "still cached after second upload");
}

#[test]
fn visible_tiles_revision_is_stable_then_changes_on_pan_and_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let r1 = host.visible_tiles_revision();
    let r2 = host.visible_tiles_revision();
    assert_eq!(r1, r2, "revision must be stable across calls with an unchanged camera");
    let z = host.pick_raster_tile_zoom();
    let tile_span = tile_world_rect(z, 0, 0).width();
    host.set_camera(tile_span * 4.0, 0.0, host.camera.zoom);
    let r3 = host.visible_tiles_revision();
    assert_ne!(r1, r3, "revision must change after a pan that moves the visible tile range");
    host.set_camera(0.0, 0.0, host.camera.zoom * 6.0);
    let r4 = host.visible_tiles_revision();
    assert_ne!(r1, r4, "revision must change after a zoom that changes the tile z");
}

#[test]
fn visible_vector_tiles_revision_matches_json_unavailable_sentinel() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.visible_vector_tiles_json(), "[]");
    assert_eq!(host.visible_vector_tiles_revision(), 0, "unavailable sentinel must match the json [] case");
    host.camera.zoom = 6000.0;
    assert_ne!(host.visible_vector_tiles_json(), "[]");
}

#[test]
fn prefetch_tiles_are_disjoint_from_visible_share_z_and_respect_cap() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let visible: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    let prefetch: Vec<serde_json::Value> = serde_json::from_str(&host.prefetch_tiles_json()).expect("json");
    assert!(!prefetch.is_empty(), "expect a prefetch ring around a finite visible window");
    let visible_z = visible[0]["z"].as_u64().unwrap();
    let visible_keys: std::collections::BTreeSet<String> = visible.iter().map(|v| v["key"].as_str().unwrap().to_string()).collect();
    for row in &prefetch {
        let key = row["key"].as_str().unwrap();
        assert!(!visible_keys.contains(key), "prefetch row {key} must not duplicate a visible tile");
        assert_eq!(row["z"].as_u64().unwrap(), visible_z, "prefetch must share the visible z");
    }
    assert!(visible.len() + prefetch.len() <= MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn prefetch_vector_tiles_json_is_empty_when_vector_tiles_unavailable() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.prefetch_vector_tiles_json(), "[]");
}

#[test]
fn lru_eviction_never_evicts_a_pinned_visible_tile_and_takes_the_stalest_one_first() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let vz = host.pick_raster_tile_zoom();
    assert!(vz >= 4, "expected a finer raster zoom for this camera (got {vz})");
    let visible_now = visible_tiles(&host.camera, &host.viewport, vz);
    let (_, vx, vy) = visible_now[0];
    let visible_key = tiles::tile_key(vz, vx, vy);

    let n = 1u32 << vz;
    let stale_x = (vx + n / 2) % n;
    let stale_key = tiles::tile_key(vz, stale_x, vy);

    host.upload_tile(vz, stale_x, vy, &test_png_1x1()).expect("seed stale tile");
    host.upload_tile(vz, vx, vy, &test_png_1x1()).expect("seed visible tile");
    host.prepare_visible_tiles();
    assert!(host.has_tile(&stale_key), "an off-screen tile must survive below the cache cap");
    assert!(host.has_tile(&visible_key));

    let filler_z = 999;
    for i in 0..(super::MAX_MAP_TILE_CACHE_ENTRIES as u32 + 8) {
        host.upload_tile(filler_z, i, 0, &test_png_1x1()).expect("filler upload");
    }
    host.prepare_visible_tiles();

    assert!(!host.has_tile(&stale_key), "least-recently-touched tile must be evicted first under cap pressure");
    assert!(host.has_tile(&visible_key), "a currently-visible (pinned) tile must never be evicted");
}

#[test]
fn set_map_theme_from_json_invalid_json_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.set_map_theme_from_json("not json").is_err());
}

#[test]
fn set_layer_visibility_from_json_invalid_json_returns_err_and_partial_json_overrides_only_given_fields() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_visibility_from_json("not json").is_err());
    host.set_layer_visibility_from_json(r#"{"roads":false}"#).expect("partial");
    let v: serde_json::Value = serde_json::from_str(&host.layer_visibility_json()).expect("json");
    assert_eq!(v["roads"], false);
    assert_eq!(v["water"], true);
}

#[test]
fn set_layer_stroke_scale_from_json_sanitizes_out_of_range_weights() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_stroke_scale_from_json("not json").is_err());
    host.set_layer_stroke_scale_from_json(r#"{"roads":999.0,"water":-5.0}"#).expect("scale");
    let v: serde_json::Value = serde_json::from_str(&host.layer_stroke_scale_json()).expect("json");
    assert_eq!(v["roads"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(v["water"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MIN);
}

#[test]
fn clamp_map_layer_weight_handles_bounds_and_non_finite_values() {
    assert_eq!(super::clamp_map_layer_weight(super::MAP_LAYER_WEIGHT_MIN - 1.0), super::MAP_LAYER_WEIGHT_MIN);
    assert_eq!(super::clamp_map_layer_weight(super::MAP_LAYER_WEIGHT_MAX + 1.0), super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(super::clamp_map_layer_weight(f64::NAN), 1.0);
    assert_eq!(super::clamp_map_layer_weight(f64::INFINITY), 1.0);
}

#[test]
fn map_layer_stroke_scale_sanitized_clamps_every_field() {
    let scale = super::MapLayerStrokeScale { raster: 100.0, water: -1.0, land: 100.0, roads: 100.0, buildings: 100.0, borders: 100.0, labels: 100.0, positions: 100.0, position_labels: 100.0, routes: 100.0, regions: -1.0 };
    let s = scale.sanitized();
    assert_eq!(s.raster, super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(s.water, super::MAP_LAYER_WEIGHT_MIN);
    assert_eq!(s.regions, super::MAP_LAYER_WEIGHT_MIN);
}

#[test]
fn gis_map_lod_scale_json_lists_all_lod_bands() {
    let v: Vec<serde_json::Value> = serde_json::from_str(&super::gis_map_lod_scale_json()).expect("json");
    assert_eq!(v.len(), 8);
    assert_eq!(v[0]["id"], "world");
    assert!(v[0]["maxZoom"].as_f64().unwrap() > 0.0);
}

#[test]
fn gis_map_camera_limits_json_min_never_exceeds_max() {
    let v: serde_json::Value = serde_json::from_str(&super::gis_map_camera_limits_json()).expect("json");
    assert!(v["min"].as_f64().unwrap() <= v["max"].as_f64().unwrap());
}

#[test]
fn map_layer_weight_slider_keys_respect_render_mode() {
    let image_keys = super::map_layer_weight_slider_keys_at_lod("city", "image");
    assert!(image_keys.contains(&"raster"));
    assert!(!image_keys.contains(&"water"));
    let vector_keys = super::map_layer_weight_slider_keys_at_lod("city", "vector");
    assert!(!vector_keys.contains(&"raster"));
    let combined_keys = super::map_layer_weight_slider_keys_at_lod("city", "bogus-mode");
    assert!(combined_keys.contains(&"raster"));
    assert!(combined_keys.contains(&"positions"));
}

#[test]
fn resolve_detail_lod_index_falls_back_when_forced_lod_unknown() {
    let span = 50.0;
    assert_eq!(super::resolve_detail_lod_index(span, Some("bogus-lod")), super::resolve_map_lod_index_from_span(span));
    assert_eq!(super::resolve_detail_lod_index(span, Some("city")), super::GIS_MAP_LOD_SCALE.index_of("city").unwrap());
}

#[test]
fn parse_tile_key_rejects_malformed_keys() {
    assert_eq!(tiles::parse_tile_key("1/2/3"), Some((1, 2, 3)));
    assert_eq!(tiles::parse_tile_key("1/2"), None);
    assert_eq!(tiles::parse_tile_key("1/2/3/4"), None);
    assert_eq!(tiles::parse_tile_key("a/b/c"), None);
}

#[test]
fn map_vector_style_as_str_round_trips_all_variants() {
    for style in [super::MapVectorStyle::Colored, super::MapVectorStyle::FigureGround, super::MapVectorStyle::InvertedFigure] {
        assert_eq!(super::MapVectorStyle::parse(style.as_str()), style);
    }
    assert_eq!(super::MapVectorStyle::parse("bogus"), super::MapVectorStyle::Colored);
}

#[test]
fn label_declutter_rejects_when_over_capacity_or_offscreen() {
    let viewport = Viewport { width: 200, height: 200, dpr: 1.0 };
    let mut declutter = super::LabelDeclutter::for_viewport(&viewport, 20.0, 1);
    assert!(declutter.try_place("first", super::Point::new(50.0, 50.0), 12.0));
    assert!(!declutter.try_place("second", super::Point::new(150.0, 150.0), 12.0), "max_count of 1 must reject a second label");
    let mut roomy = super::LabelDeclutter::for_viewport(&viewport, 20.0, 10);
    assert!(!roomy.try_place("offscreen", super::Point::new(10_000.0, 10_000.0), 12.0));
}

#[test]
fn clamp_map_zoom_and_for_viewport_stay_within_bounds() {
    assert_eq!(super::clamp_map_zoom(super::MAP_CAMERA_ZOOM_MIN - 5.0), super::MAP_CAMERA_ZOOM_MIN);
    assert_eq!(super::clamp_map_zoom(super::MAP_CAMERA_ZOOM_MAX + 5.0), super::MAP_CAMERA_ZOOM_MAX);
    let viewport = Viewport { width: 4000, height: 4000, dpr: 1.0 };
    let cover = super::projection::cover_zoom_for_viewport(&viewport);
    assert!(super::clamp_map_zoom_for_viewport(0.0, &viewport) >= cover);
}

// #region 🔖️MercatorOracleFixture
/// 🌐️ `lodBands` are specification vectors — no third party defines this repository's own
/// `GIS_MAP_LOD_MAX_SPAN_DEG`/`GIS_MAP_LOD_TILE_Z` banding. The `projection`/`tileNumbering`/
/// `tileBounds` groups of the same fixture ARE cross-checked against the `mercantile` library by
/// `../📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs` and by
/// `🧪️tests/🕸️web-mercator-tile-oracle/🐍️.py`; all three read this one fixture so oracle and subject
/// compare identical inputs.
///
/// @see 🧪️tests/🕸️web-mercator-tile-oracle/🥒️.feature
const MERCATOR_ORACLE_FIXTURE: &str = include_str!("../🕸️web-mercator-tile-oracle/🧫️fixtures/🔣️.json");

#[test]
fn lod_band_selection_matches_frozen_specification_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
    for entry in fixture["lodBands"].as_array().expect("lodBands array") {
        let span = entry["spanDeg"].as_f64().expect("spanDeg");
        let expected_idx = entry["lodIndex"].as_u64().expect("lodIndex") as usize;
        let expected_tile_z = entry["tileZ"].as_u64().expect("tileZ") as u32;
        let idx = super::resolve_map_lod_index_from_span(span);
        assert_eq!(idx, expected_idx, "lod index for span={span}");
        assert_eq!(super::GIS_MAP_LOD_TILE_Z[idx], expected_tile_z, "tile z for span={span}");
    }
}

#[test]
fn active_map_lod_tracks_the_same_bands_as_the_span_resolver() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    for &span_probe in &[180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05] {
        let cam = default_world_camera(&viewport);
        let scaled = Camera { x: cam.x, y: cam.y, zoom: cam.zoom * (180.0 / f64::max(span_probe, 1e-6)) };
        let span = super::viewport_lon_span_degrees(&scaled, &viewport);
        let expected_idx = super::resolve_map_lod_index_from_span(span);
        let lod = super::active_map_lod(None, &scaled, &viewport);
        assert_eq!(lod.id, super::GIS_MAP_LODS[expected_idx].id, "active_map_lod band for span={span}");
    }
}

#[test]
fn visible_tile_count_never_exceeds_max_visible_tile_requests() {
    for zoom in [super::MAP_CAMERA_ZOOM_MIN, 5_000.0, 500_000.0, super::MAP_CAMERA_ZOOM_MAX] {
        let mut host = super::MapHost::new();
        host.set_size(1920, 1080, 2.0);
        host.set_camera(0.0, 0.0, zoom);
        assert!(visible_tiles(&host.camera, &host.viewport, host.pick_raster_tile_zoom()).len() <= MAX_VISIBLE_TILE_REQUESTS);
        assert!(visible_tiles(&host.camera, &host.viewport, host.pick_vector_tile_zoom()).len() <= MAX_VISIBLE_TILE_REQUESTS);
    }
}
// #endregion 🔖️MercatorOracleFixture
