
use super::*;

#[test]
fn tile_key_roundtrip() {
    assert_eq!(tiles::tile_key(10, 3, 7), "10/3/7");
    assert_eq!(tiles::parse_tile_key("10/3/7"), Some((10, 3, 7)));
    assert_eq!(tiles::parse_tile_key("garbage"), None);
}

#[test]
fn lonlat_tile_xy_roundtrip_is_stable() {
    let (x, y) = projection::lonlat_to_tile_xy(9.7382, 52.3759, 12);
    let (lon, lat) = projection::tile_xy_to_lonlat(x.floor(), y.floor(), 12);
    // Round-tripping the tile's top-left corner should land close to (not equal to, since we
    // floor to the tile origin) the original point.
    assert!((lon - 9.7382).abs() < 1.0);
    assert!((lat - 52.3759).abs() < 1.0);
}

#[test]
fn local_meters_roundtrip() {
    let origin_lon = 9.7382;
    let origin_lat = 52.3759;
    let (x, y) = projection::lonlat_to_local_meters(9.75, 52.38, origin_lon, origin_lat);
    let (lon, lat) = projection::local_meters_to_lonlat(x, y, origin_lon, origin_lat);
    assert!((lon - 9.75).abs() < 1e-6);
    assert!((lat - 52.38).abs() < 1e-6);
}

#[test]
fn pick_zoom_clamps_to_bounds() {
    assert_eq!(tiles::pick_zoom(1.0), tiles::TERRAIN_TILE_MAX_ZOOM);
    assert_eq!(tiles::pick_zoom(1_000_000_000.0), tiles::TERRAIN_TILE_MIN_ZOOM);
}

#[test]
fn visible_tiles_returns_bounded_grid_around_center() {
    let rows = tiles::visible_tiles(9.7382, 52.3759, 12);
    assert!(!rows.is_empty());
    assert!(rows.len() <= 25);
}

fn solid_terrarium_png(elevation: f32) -> Vec<u8> {
    let value = (elevation + 32768.0).round() as i64;
    let r = ((value >> 8) & 0xff) as u8;
    let remainder = value - ((r as i64) << 8);
    let g = remainder.clamp(0, 255) as u8;
    let mut image = semio_framework_pixels::RasterImage::new(TERRARIUM_TILE_PX, TERRARIUM_TILE_PX);
    for pixel in image.pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[r, g, 0, 255]);
    }
    semio_framework_pixels::encode_png(&image).expect("encode png")
}

#[test]
fn upload_and_mesh_a_flat_tile_produces_grid_geometry() {
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(9.7382, 52.3759);
    let bytes = solid_terrarium_png(123.0);
    assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
    let mesh_json = session.terrain_tile_mesh_json(12, 2000, 1300);
    assert_ne!(mesh_json, "null");
    let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
    let n = TERRAIN_GRID_RESOLUTION as usize;
    assert_eq!(mesh.positions.len(), n * n * 3);
    assert_eq!(mesh.normals.len(), n * n * 3);
    assert_eq!(mesh.uvs.len(), n * n * 2);
    assert_eq!(mesh.indices.len(), (n - 1) * (n - 1) * 6);
    // A uniform-elevation tile should have (near-)zero elevation variance across the grid.
    let first_elevation = mesh.positions[2];
    for chunk in mesh.positions.chunks_exact(3) {
        assert!((chunk[2] - first_elevation).abs() < 5.0, "expected uniform elevation, got {} vs {}", chunk[2], first_elevation);
    }
}

#[test]
fn missing_tile_mesh_is_null() {
    let session = TerrainSessionCore::default();
    assert_eq!(session.terrain_tile_mesh_json(12, 0, 0), "null");
}

#[test]
fn pick_zoom_halves_reference_distance_per_level() {
    assert_eq!(tiles::pick_zoom(400.0), tiles::TERRAIN_TILE_MAX_ZOOM);
    assert_eq!(tiles::pick_zoom(800.0), tiles::TERRAIN_TILE_MAX_ZOOM - 1);
    assert_eq!(tiles::pick_zoom(1600.0), tiles::TERRAIN_TILE_MAX_ZOOM - 2);
}

#[test]
fn visible_tiles_clamps_at_world_edge() {
    let rows = tiles::visible_tiles(-179.9, 0.1, 0);
    assert_eq!(rows, vec![(0, 0, 0)]);
}

#[test]
fn decode_terrarium_png_invalid_bytes_returns_error() {
    let error = decode_terrarium_png(b"not a real png").expect_err("garbage bytes should not decode");
    assert!(!error.to_string().is_empty());
}

#[test]
fn sample_elevation_clamps_out_of_bounds_coordinates() {
    let mut image = semio_framework_pixels::RasterImage::new(2, 2);
    image.pixels[((1 * 2 + 1) * 4)..][..4].copy_from_slice(&[128, 0, 0, 255]);
    let inside = sample_elevation(&image, 1.0, 1.0);
    let clamped_high = sample_elevation(&image, 999.0, 999.0);
    let clamped_low = sample_elevation(&image, -50.0, -50.0);
    assert_eq!(inside, clamped_high);
    assert_ne!(inside, clamped_low);
}

#[test]
fn normalize3_degenerate_vector_does_not_panic_or_nan() {
    let (x, y, z) = normalize3(0.0, 0.0, 0.0);
    assert!(x.is_finite() && y.is_finite() && z.is_finite());
}

#[test]
fn upload_elevation_tile_invalid_bytes_returns_false_and_no_mesh() {
    let mut session = TerrainSessionCore::default();
    assert!(!session.upload_elevation_tile(5, 1, 1, b"garbage"));
    assert_eq!(session.terrain_tile_mesh_json(5, 1, 1), "null");
}

#[test]
fn evict_terrain_tile_removes_previously_uploaded_tile() {
    let mut session = TerrainSessionCore::default();
    let bytes = solid_terrarium_png(50.0);
    assert!(session.upload_elevation_tile(8, 10, 10, &bytes));
    assert_ne!(session.terrain_tile_mesh_json(8, 10, 10), "null");
    session.evict_terrain_tile(8, 10, 10);
    assert_eq!(session.terrain_tile_mesh_json(8, 10, 10), "null");
}

#[test]
fn set_exaggeration_clamps_negative_to_zero() {
    let mut session = TerrainSessionCore::default();
    session.set_exaggeration(-3.0);
    let bytes = solid_terrarium_png(200.0);
    session.upload_elevation_tile(9, 5, 5, &bytes);
    let mesh_json = session.terrain_tile_mesh_json(9, 5, 5);
    let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
    for chunk in mesh.positions.chunks_exact(3) {
        assert_eq!(chunk[2], 0.0);
    }
}

#[test]
fn visible_terrain_tiles_json_falls_back_to_defaults_on_invalid_camera_json() {
    let session = TerrainSessionCore::default();
    let json = session.visible_terrain_tiles_json("not json");
    let rows: Vec<serde_json::Value> = serde_json::from_str(&json).expect("valid array");
    assert!(!rows.is_empty());
}

#[test]
fn visible_terrain_tiles_json_reflects_camera_distance_in_zoom() {
    let session = TerrainSessionCore::default();
    let close_json = session.visible_terrain_tiles_json(r#"{"position":[0,0,100],"target":[0,0,0]}"#);
    let far_json = session.visible_terrain_tiles_json(r#"{"position":[0,0,1000000],"target":[0,0,0]}"#);
    let close_rows: Vec<serde_json::Value> = serde_json::from_str(&close_json).expect("valid array");
    let far_rows: Vec<serde_json::Value> = serde_json::from_str(&far_json).expect("valid array");
    assert!(close_rows[0]["z"].as_u64().unwrap() > far_rows[0]["z"].as_u64().unwrap());
}

fn gradient_terrarium_png() -> Vec<u8> {
    let mut image = semio_framework_pixels::RasterImage::new(TERRARIUM_TILE_PX, TERRARIUM_TILE_PX);
    for (i, pixel) in image.pixels.chunks_exact_mut(4).enumerate() {
        let px = (i as u32) % TERRARIUM_TILE_PX;
        let value = (500.0 + px as f32 * 10.0 + 32768.0).round() as i64;
        let r = ((value >> 8) & 0xff) as u8;
        let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
        pixel.copy_from_slice(&[r, g, 0, 255]);
    }
    semio_framework_pixels::encode_png(&image).expect("encode png")
}

#[test]
fn sloped_tile_mesh_has_varying_elevation_and_nontrivial_normals() {
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(9.7382, 52.3759);
    let bytes = gradient_terrarium_png();
    assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
    let mesh_json = session.terrain_tile_mesh_json(12, 2000, 1300);
    let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
    let min_z = mesh.positions.iter().skip(2).step_by(3).cloned().fold(f32::INFINITY, f32::min);
    let max_z = mesh.positions.iter().skip(2).step_by(3).cloned().fold(f32::NEG_INFINITY, f32::max);
    assert!(max_z - min_z > 1.0, "expected sloped tile to have elevation spread, got {min_z}..{max_z}");
    let has_tilted_normal = mesh.normals.chunks_exact(3).any(|n| n[0].abs() > 1e-3);
    assert!(has_tilted_normal, "expected at least one non-vertical normal on a sloped tile");
}

#[test]
fn visible_tile_coords_is_deterministic_for_identical_input() {
    let camera = CameraRecord { position: Some([0.0, 0.0, 500.0]), target: Some([10.0, 10.0, 0.0]) };
    let first = visible_tile_coords(&camera, 9.7382, 52.3759);
    let second = visible_tile_coords(&camera, 9.7382, 52.3759);
    assert_eq!(first, second, "a tier-(e) pure query must return identical output for identical input");
}

#[test]
fn terrain_tile_mesh_json_is_deterministic_for_the_same_cached_tile() {
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(9.7382, 52.3759);
    let bytes = gradient_terrarium_png();
    assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
    let first = session.terrain_tile_mesh_json(12, 2000, 1300);
    let second = session.terrain_tile_mesh_json(12, 2000, 1300);
    assert_eq!(first, second, "mesh build over an unchanged cached tile must be byte-identical across calls");
}
