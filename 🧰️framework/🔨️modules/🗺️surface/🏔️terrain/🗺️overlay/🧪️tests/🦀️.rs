use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    visible_positions: Vec<String>,
    raised_world_height: f32,
    visible_routes: Vec<String>,
    visible_regions: Vec<String>,
    closed_ring_starts: Vec<[f64; 2]>,
}

#[derive(Deserialize)]
struct Fixture {
    origin: [f64; 2],
    tile: [u32; 3],
    exaggeration: f64,
    overlays: TerrainOverlays,
    expected: Expected,
}

#[test]
fn visible_features_drape_on_the_loaded_dem_and_close_each_ring() {
    let fixture: Fixture = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let mut session = TerrainSessionCore::default();
    let [z, x, y] = fixture.tile;
    session.set_project_origin(fixture.origin[0], fixture.origin[1]);
    session.set_exaggeration(fixture.exaggeration);
    assert!(session.overlay_tile(z, x, y, &fixture.overlays).is_none());
    let mut pixels = Vec::with_capacity(256 * 256 * 4);
    for row in 0..256 {
        for col in 0..256 {
            pixels.extend([128, if (row / 32 + col / 32) % 2 == 0 { 40 } else { 125 }, 0, 255]);
        }
    }
    let terrain = semio_framework_pixels::RasterImage { width: 256, height: 256, pixels };
    let png = semio_framework_pixels::encode_png(&terrain).unwrap();
    assert!(session.upload_elevation_tile(z, x, y, &png));
    assert_eq!(session.terrain_origin_elevation(z, x, y), Some(80.0));
    assert_eq!(session.terrain_origin_elevation(z, x - 1, y), None);
    let overlay = session.overlay_tile(z, x, y, &fixture.overlays).unwrap();
    let expected = fixture.expected;
    assert_eq!(overlay.positions.iter().map(|p| &p.id).collect::<Vec<_>>(), expected.visible_positions.iter().collect::<Vec<_>>());
    assert_eq!(overlay.routes.iter().map(|p| &p.id).collect::<Vec<_>>(), expected.visible_routes.iter().collect::<Vec<_>>());
    assert_eq!(overlay.regions.iter().map(|p| &p.id).collect::<Vec<_>>(), expected.visible_regions.iter().collect::<Vec<_>>());
    assert!((overlay.positions[1].point[2] - expected.raised_world_height).abs() < 1e-3);
    assert!(overlay.positions[0].point[2] >= 80.0);
    let route = &overlay.routes[0].segments;
    assert!(route.len() > 32);
    assert!(route.first().unwrap()[0][0].abs() < 1e-3);
    assert!(route.last().unwrap()[1][0] > 2400.0);
    assert!(route.windows(2).all(|pair| pair[0][1] == pair[1][0]));
    for [lon, lat] in expected.closed_ring_starts {
        let [x, y] = [(lon * 111_320.0) as f32, (lat * 111_320.0) as f32];
        let at_start = |point: [f32; 3]| (point[0] - x).abs() < 1e-3 && (point[1] - y).abs() < 1e-3;
        assert!(overlay.regions[0].segments.iter().any(|segment| at_start(segment[0])));
        assert!(overlay.regions[0].segments.iter().any(|segment| at_start(segment[1])));
    }
}

#[test]
fn authored_polar_overlays_use_exact_grid_bounds_and_local_mesh_triangles() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🧊️polar.json")).unwrap();
    let bounds = serde_json::from_value(fixture["bounds"].clone()).unwrap();
    let [lon, lat]: [f64; 2] = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let exaggeration = fixture["exaggeration"].as_f64().unwrap();
    let height = |field: &str| fixture[field].as_i64().unwrap() as i32 * 1000;
    let rows_millimetres = (0..GRID as i32).map(|row| (0..GRID as i32).map(|column| {
        height("heightAtSouthWest") + height("heightStepEast") * column + height("heightStepNorth") * row
    }).collect()).collect();
    let source = super::super::TerrainElevationGrid { missing_samples: None, bounds, rows_millimetres };
    let overlays = serde_json::from_value(fixture["overlays"].clone()).unwrap();
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(lon, lat);
    session.set_exaggeration(exaggeration);
    assert!(session.set_elevation_grid_json(&to_json_string(&source)));
    let (z, x, y) = ELEVATION_GRID_TILE;
    let tile = session.overlay_tile(z, x, y, &overlays).unwrap();
    let expected = &fixture["expected"];
    let near = |actual: [f32; 3], wanted: &serde_json::Value| {
        let wanted = serde_json::from_value::<[f32; 3]>(wanted.clone()).unwrap();
        assert!(actual.into_iter().zip(wanted).all(|(a, b)| (a - b).abs() < 1e-3), "{actual:?} != {wanted:?}");
    };
    let markers = expected["positions"].as_array().unwrap();
    assert_eq!(tile.positions.len(), markers.len());
    for (marker, desired) in tile.positions.iter().zip(markers) {
        assert_eq!(marker.id, desired["id"].as_str().unwrap());
        near(marker.point, &desired["point"]);
    }
    let [route] = tile.routes.as_slice() else { panic!("expected one polar route") };
    near(route.segments.first().unwrap()[0], &expected["route"][0]);
    near(route.segments.last().unwrap()[1], &expected["route"][1]);
    let [region] = tile.regions.as_slice() else { panic!("expected one polar region") };
    near(region.segments.first().unwrap()[0], &expected["regionStart"]);
    near(region.segments.last().unwrap()[1], &expected["regionStart"]);
}
