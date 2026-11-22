
use super::*;

#[test]
fn embedded_elevation_pick_uses_visible_triangles_and_southwest_sample_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️elevation-grid/🔣️.json")).unwrap();
    let bounds: [f64; 4] = serde_json::from_value(fixture["bounds"].clone()).unwrap();
    let [west, south, east, north] = bounds;
    let [lon, lat]: [f64; 2] = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let samples = &fixture["samples"];
    let size = samples["size"].as_i64().unwrap();
    let grid = &fixture["pick"]["raisedSample"];
    let rows_millimetres: Vec<Vec<i32>> = (0..size).map(|row| (0..size).map(|column| {
        let base = samples["southwest"].as_i64().unwrap()
            + row * samples["northwardStep"].as_i64().unwrap()
            + column * samples["eastwardStep"].as_i64().unwrap();
        let raised = grid["column"].as_i64() == Some(column) && grid["row"].as_i64() == Some(row);
        ((base + if raised { grid["meters"].as_i64().unwrap() } else { 0 }) * 1000) as i32
    }).collect()).collect();
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(lon, lat);
    session.set_exaggeration(fixture["exaggeration"].as_f64().unwrap());
    assert!(session.pick_elevation_grid([0.0, 0.0, 1000.0], [0.0, 0.0, -1.0]).is_none());
    assert!(session.set_elevation_grid(Some(TerrainElevationGrid { missing_samples: None, bounds, rows_millimetres })));
    let cells = (size - 1) as f64;
    for ray in fixture["pick"]["rays"].as_array().unwrap() {
        let [column, row, z]: [f64; 3] = serde_json::from_value(ray["originSample"].clone()).unwrap();
        let direction = serde_json::from_value(ray["direction"].clone()).unwrap();
        let (x, y) = projection::lonlat_to_local_meters(west + (east - west) * column / cells, south + (north - south) * row / cells, lon, lat);
        let hit = session.pick_elevation_grid([x, y, z], direction);
        if ray["miss"] == true {
            assert!(hit.is_none(), "{ray}");
            continue;
        }
        let hit = hit.expect("forward ray intersects the grid");
        if let Some(height) = ray["height"].as_f64() {
            assert!((hit.position[2] - height).abs() < 0.001, "{ray}");
        }
        if !ray["sample"].is_null() {
            assert_eq!(serde_json::json!(hit.sample), ray["sample"], "{ray}");
        }
        assert!((west..=east).contains(&hit.coordinates[0]));
        assert!((south..=north).contains(&hit.coordinates[1]));
    }
    assert_eq!(session.pick_elevation_grid_json(r#"{"origin":[0,0,1],"direction":[0,0,1],"extra":0}"#), "null");
}

#[test]
fn embedded_elevation_grid_publishes_one_mesh_and_samples_its_origin() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️elevation-grid/🔣️.json")).unwrap();
    let origin: [f64; 2] = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let samples = &fixture["samples"];
    let size = samples["size"].as_i64().unwrap();
    let rows_millimetres = (0..size).map(|row| (0..size).map(|column| {
        1000 * (samples["southwest"].as_i64().unwrap()
            + row * samples["northwardStep"].as_i64().unwrap()
            + column * samples["eastwardStep"].as_i64().unwrap())
    }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(origin[0], origin[1]);
    session.set_exaggeration(fixture["exaggeration"].as_f64().unwrap());
    assert!(session.set_elevation_grid_json(&serde_json::json!({"bounds": fixture["bounds"], "rowsMillimetres": rows_millimetres}).to_string()));
    let visible: serde_json::Value = serde_json::from_str(&session.visible_terrain_tiles_json("{}")).unwrap();
    assert_eq!(visible, serde_json::json!([{"z":0,"x":0,"y":0,"key":"0/0/0"}]));
    let mesh: TerrainTileMeshJson = serde_json::from_str(&session.terrain_tile_mesh_json(0, 0, 0)).unwrap();
    let expected = &fixture["expected"];
    assert_eq!(mesh.indices.len() / 3, expected["triangleCount"].as_u64().unwrap() as usize);
    assert_eq!(mesh.positions[2], expected["northwestHeight"].as_f64().unwrap() as f32);
    assert_eq!(mesh.positions.last().copied(), expected["southeastHeight"].as_f64().map(|height| height as f32));
    let height = expected["originHeight"].as_f64().unwrap() as f32;
    assert!((session.terrain_origin_elevation(0, 0, 0).unwrap() - height).abs() < 1e-3);
    let overlays: serde_json::Value = serde_json::from_str(&session.terrain_overlay_tile_json(0, 0, 0, &fixture["overlays"].to_string())).unwrap();
    assert_eq!(overlays["positions"][0]["id"], "center");
    assert!((overlays["positions"][0]["point"][2].as_f64().unwrap() as f32 - height).abs() < 1e-3);
    assert!(!overlays["routes"][0]["segments"].as_array().unwrap().is_empty());
    assert!(!overlays["regions"][0]["segments"].as_array().unwrap().is_empty());
}

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

#[test]
fn terrarium_tile_query_and_mesh_match_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🏔️terrarium-tile/🔣️.json")).unwrap();
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(fixture["origin"]["longitude"].as_f64().unwrap(), fixture["origin"]["latitude"].as_f64().unwrap());
    session.set_exaggeration(fixture["exaggeration"].as_f64().unwrap());
    let rows: Vec<serde_json::Value> = serde_json::from_str(&session.visible_terrain_tiles_json(&fixture["camera"].to_string())).unwrap();
    assert_eq!(rows.len(), fixture["rows"].as_u64().unwrap() as usize);
    assert_eq!(rows.first(), Some(&fixture["expectedCenter"]));
    assert!(rows.iter().all(|row| row["z"] == fixture["expectedCenter"]["z"]));
    let pixel: [u8; 4] = serde_json::from_value(fixture["pixel"].clone()).unwrap();
    let mut image = semio_framework_pixels::RasterImage::new(TERRARIUM_TILE_PX, TERRARIUM_TILE_PX);
    for target in image.pixels.chunks_exact_mut(4) {
        target.copy_from_slice(&pixel);
    }
    let bytes = semio_framework_pixels::encode_png(&image).unwrap();
    let (z, x, y) = tiles::parse_tile_key(rows[0]["key"].as_str().unwrap()).unwrap();
    assert!(session.upload_elevation_tile(z, x, y, &bytes));
    let mesh: TerrainTileMeshJson = serde_json::from_str(&session.terrain_tile_mesh_json(z, x, y)).unwrap();
    assert!(mesh.positions.chunks_exact(3).all(|vertex| f64::from(vertex[2]) == fixture["expectedMeshHeight"].as_f64().unwrap()));
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
    let camera = CameraRecord { position: Some([0.0, 0.0, 500.0]), target: Some([10.0, 10.0, 0.0]), ..CameraRecord::default() };
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


#[test]
fn rectangular_fractional_grid_agrees_across_mesh_pick_and_drape() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json")).unwrap();
    let shape: [u32; 2] = serde_json::from_value(fixture["shape"].clone()).unwrap();
    let [lon, lat]: [f64; 2] = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let grid: TerrainElevationGrid = from_json_str(&fixture["grid"].to_string(), JsonMemberPolicy::Reject).unwrap();
    assert_eq!(grid.dimensions(), Some(shape));
    assert!(grid.is_valid());
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(lon, lat);
    session.set_exaggeration(fixture["exaggeration"].as_f64().unwrap());
    assert!(session.set_elevation_grid_json(&fixture["grid"].to_string()));
    let mesh = session.terrain_tile_mesh(0, 0, 0).unwrap();
    let expected = &fixture["expected"];
    assert_eq!(serde_json::json!(mesh.indices), expected["indices"]);
    let near = |value: f64, key: &str| {
        assert!((value - expected[key].as_f64().unwrap()).abs() < 1e-5, "{key}: {value}");
    };
    near(f64::from(mesh.positions[2]), "northwestHeight");
    near(f64::from(*mesh.positions.last().unwrap()), "southeastHeight");
    near(f64::from(session.terrain_origin_elevation(0, 0, 0).unwrap()), "originHeight");
    let hit = session.pick_elevation_grid([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]).unwrap();
    assert_eq!(serde_json::json!(hit.sample), expected["originSample"]);
    near(hit.position[2], "originHeight");
    let overlays = session.overlay_tile(0, 0, 0, &from_json_str(&fixture["overlays"].to_string(), JsonMemberPolicy::Reject).unwrap()).unwrap();
    near(f64::from(overlays.positions[0].point[2]), "originHeight");
    for path in overlays.routes.iter().chain(&overlays.regions) {
        assert!(path.segments.len() > 3);
        for [a, b] in &path.segments {
            let [x, y, z] = std::array::from_fn(|axis| (f64::from(a[axis]) + f64::from(b[axis])) * 0.5);
            let hit = session.pick_elevation_grid([x, y, 20.0], [0.0, 0.0, -1.0]).unwrap();
            assert!((hit.position[2] - z).abs() < 1e-5);
        }
    }
}

#[test]
fn authored_grid_width_height_and_storage_are_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json")).unwrap();
    let [width, height]: [usize; 2] = serde_json::from_value(fixture["maximumShape"].clone()).unwrap();
    let bounds = serde_json::from_value(fixture["grid"]["bounds"].clone()).unwrap();
    let mut grid = TerrainElevationGrid { missing_samples: None, bounds, rows_millimetres: vec![vec![i32::MIN; width]; height] };
    grid.rows_millimetres[0][0] = i32::MAX;
    let mesh = grid.mesh(5.556, 50.586, 1.0).unwrap();
    assert_eq!(mesh.indices.len(), (width - 1) * (height - 1) * 6);
    let bytes = 4 * (mesh.positions.len() + mesh.normals.len() + mesh.indices.len() + mesh.uvs.len());
    assert_eq!(bytes as u64, fixture["maximumBytes"].as_u64().unwrap());
    for [width, height] in [[1, 2], [2, 1], [66, 2], [2, 66]] {
        grid.rows_millimetres = vec![vec![0; width]; height];
        assert!(!grid.is_valid());
        assert!(grid.mesh(5.556, 50.586, 1.0).is_none());
    }
    grid.rows_millimetres = vec![vec![0; 2], vec![0; 3]];
    assert!(!grid.is_valid());
    let mut session = TerrainSessionCore::default();
    for invalid in [
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,2147483648]]}"#,
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,-2147483649]]}"#,
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,0.1]]}"#,
        r#"{"bounds":[0,0,1,1],"rows":[[0,0],[0,0]]}"#,
    ] {
        assert!(!session.set_elevation_grid_json(invalid));
    }
}

#[test]
fn missing_terrain_keeps_existing_diagonals_for_mesh_picks_and_drape() {
    use super::overlay::{TerrainOverlayPosition, TerrainOverlayRoute, TerrainOverlays};
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🕳️nodata/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let source: TerrainElevationGrid = from_json_str(&case["source"].to_string(), JsonMemberPolicy::Reject).unwrap();
        let [west, south, east, north] = source.bounds;
        let [width, height] = source.dimensions().unwrap();
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(west, north);
        assert!(session.set_elevation_grid(Some(source.clone())));
        let mesh = session.terrain_tile_mesh(0, 0, 0).unwrap();
        let expected: Vec<[u32; 3]> = serde_json::from_value(case["facesNorthFirst"].clone()).unwrap();
        assert_eq!(mesh.indices, expected.iter().flatten().copied().collect::<Vec<_>>(), "{}", case["name"]);
        let compact = source.compact_mesh(west, north, 1.0).unwrap();
        assert_eq!(compact.indices.len(), mesh.indices.len());
        if mesh.indices.is_empty() {
            assert!(mesh.positions.is_empty() && mesh.normals.is_empty() && mesh.uvs.is_empty());
            assert!(session.terrain_origin_elevation(0, 0, 0).is_none());
            assert!(session.pick_elevation_grid([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]).is_none());
        }
        let geographic = |x, y| [west + (east - west) * x / f64::from(width - 1), north - (north - south) * y / f64::from(height - 1)];
        if let Some(queries) = case["surfaceQueriesNorthFirst"].as_array() {
            for query in queries {
                let [x, y]: [f64; 2] = serde_json::from_value(query["xy"].clone()).unwrap();
                let coordinate = geographic(x, y);
                let overlays = TerrainOverlays { positions: vec![TerrainOverlayPosition {
                    id: "point".into(), name: "Point".into(), coordinates: coordinate, elevation: None
                }], ..Default::default() };
                let drape = session.overlay_tile(0, 0, 0, &overlays).unwrap();
                let row = y.floor().min(f64::from(height - 2)) as usize;
                let column = x.floor().min(f64::from(width - 2)) as usize;
                let i = row * width as usize + column;
                let u = x - column as f64;
                let v = y - row as f64;
                let px = f64::from(mesh.positions[3 * i]) * (1.0 - u) + f64::from(mesh.positions[3 * (i + 1)]) * u;
                let py = f64::from(mesh.positions[3 * i + 1]) * (1.0 - v) + f64::from(mesh.positions[3 * (i + width as usize) + 1]) * v;
                let hit = session.pick_elevation_grid([px, py, 3_000_000.0], [0.0, 0.0, -1.0]);
                if let Some(mm) = query["millimetres"].as_f64() {
                    assert!((f64::from(drape.positions[0].point[2]) - mm / 1000.0).abs() < 0.01, "{query}");
                    let hit = hit.expect("surviving face hit");
                    assert!((hit.position[2] - mm / 1000.0).abs() < 0.01, "{query}");
                    assert!(!missing_samples::is_missing(source.missing_samples.as_deref(), (hit.sample[1] * width + hit.sample[0]) as usize));
                } else {
                    assert!(drape.positions.is_empty() && hit.is_none(), "{query}");
                }
            }
        }
        if let Some(routes) = case["horizontalDrapeNorthFirst"].as_array() {
            for route in routes {
                let y = route["y"].as_f64().unwrap();
                let expected: Vec<[f64; 2]> = serde_json::from_value(route["intervals"].clone()).unwrap();
                for reverse in [false, true] {
                    let mut coordinates = vec![geographic(0.0, y), geographic(f64::from(width - 1), y)];
                    if reverse { coordinates.reverse(); }
                    let input = TerrainOverlays { routes: vec![TerrainOverlayRoute {
                        id: "route".into(), name: "Route".into(), coordinates,
                    }], ..Default::default() };
                    let output = session.overlay_tile(0, 0, 0, &input).unwrap();
                    let segments = &output.routes[0].segments;
                    let position = |x: f32| f64::from(x) / f64::from(mesh.positions[3 * (width - 1) as usize]) * f64::from(width - 1);
                    let mut actual: Vec<[f64; 2]> = segments.iter().map(|[a, b]| {
                        let (a, b) = (position(a[0]), position(b[0]));
                        [a.min(b), a.max(b)]
                    }).collect();
                    actual.sort_by(|a, b| a[0].total_cmp(&b[0]));
                    let mut merged: Vec<[f64; 2]> = Vec::new();
                    for [start, end] in actual {
                        if let Some(last) = merged.last_mut().filter(|last| start <= last[1] + 1.0e-6) {
                            last[1] = last[1].max(end);
                        } else {
                            merged.push([start, end]);
                        }
                    }
                    assert_eq!(merged.len(), expected.len(), "{route}");
                    for (actual, expected) in merged.iter().flatten().zip(expected.iter().flatten()) {
                        assert!((actual - expected).abs() < 1.0e-6, "{route}");
                    }
                }
            }
        }
        if case["name"] == "isolated-valid-extreme" {
            assert!(mesh.positions.iter().skip(2).step_by(3).all(|z| *z <= 4.0));
            assert!(!compact.positions.contains(&(i32::MAX as f32 / 1000.0)));
        }
    }
}

#[test]
fn json_published_mesh_points_pick_the_surviving_face_exactly() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🕳️nodata/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let Some(queries) = case["surfaceQueriesNorthFirst"].as_array() else { continue };
        let source = case["source"].to_string();
        let grid: TerrainElevationGrid = from_json_str(&source, JsonMemberPolicy::Reject).unwrap();
        let [west, _, _, north] = grid.bounds;
        let [width, _] = grid.dimensions().unwrap();
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(west, north);
        assert!(session.set_elevation_grid_json(&source));
        let mesh: serde_json::Value = serde_json::from_str(&session.terrain_tile_mesh_json(0, 0, 0)).unwrap();
        let positions: Vec<f64> = serde_json::from_value(mesh["positions"].clone()).unwrap();
        for query in queries {
            let [x, y]: [f64; 2] = serde_json::from_value(query["xy"].clone()).unwrap();
            let column = x.floor().min(f64::from(width - 2)) as usize;
            let row = y.floor().min(f64::from(grid.dimensions().unwrap()[1] - 2)) as usize;
            let i = 3 * (row * width as usize + column);
            let (fx, fy) = (x - column as f64, y - row as f64);
            let corner = |offset: usize, axis: usize| positions[i + 3 * offset + axis];
            let bilinear = |axis: usize| (1.0 - fy) * ((1.0 - fx) * corner(0, axis) + fx * corner(1, axis))
                + fy * ((1.0 - fx) * corner(width as usize, axis) + fx * corner(width as usize + 1, axis));
            let (px, py) = (f64::from(bilinear(0) as f32), f64::from(bilinear(1) as f32));
            let hit = session.pick_elevation_grid_json(&format!(
                r#"{{"origin":[{px},{py},3000000],"direction":[0,0,-1]}}"#
            ));
            let hit: serde_json::Value = serde_json::from_str(&hit).unwrap();
            match query["millimetres"].as_f64() {
                None => assert!(hit.is_null(), "{query}"),
                Some(mm) => {
                    let z = hit["position"][2].as_f64().unwrap_or_else(|| panic!("{query}: {hit}"));
                    assert!((z - mm / 1000.0).abs() < 0.01, "{query}: {hit}");
                }
            }
        }
    }
}
