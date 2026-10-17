/// 🌍️ Mercator oracle and tile budget: the committed Web Mercator fixture, the fixture band policy, the band selection, the host band report and the visible tile request bound.

// #region 🔖️MercatorOracleFixture
/// 🌐️ `lodBands` are specification vectors of the sample band table in `lodConstants` (`maxSpanDeg`, `tileZ`), fed to a
/// [`super::DescribedMapPolicy`] — no third party defines a band scheme. The `projection`/`tileNumbering`/`tileBounds`
/// groups of the same fixture ARE cross-checked against the `mercantile` library by
/// `../../🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs` and by `🧪️tests/🕸️web-mercator-tile-oracle/🐍️.py`; all three read this
/// one fixture so oracle and subject compare identical inputs.
///
/// @see 🧪️tests/🕸️web-mercator-tile-oracle/🥒️.feature
const MERCATOR_ORACLE_FIXTURE: &str = include_str!("../../🧫️fixtures/🕸️web-mercator-tile-oracle/🔣️.json");

/// 📶️ A policy whose band table is the fixture's `lodConstants`, band ids `band-<index>`.
fn fixture_band_policy() -> super::DescribedMapPolicy {
    let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
    let constants = &fixture["lodConstants"];
    let template = super::MapPolicyDescriptor::neutral().bands.remove(0);
    let bands = constants["maxSpanDeg"]
        .as_array()
        .expect("maxSpanDeg")
        .iter()
        .zip(constants["tileZ"].as_array().expect("tileZ"))
        .enumerate()
        .map(|(index, (span, tile_z))| super::MapLodBand { id: format!("band-{index}"), max_span_deg: span.as_f64().expect("span"), raster_tile_z: tile_z.as_u64().expect("tile z") as u32, ..template.clone() })
        .collect();
    super::DescribedMapPolicy::new(super::MapPolicyDescriptor { bands, ..super::MapPolicyDescriptor::neutral() })
}

#[test]
fn band_selection_of_a_described_policy_matches_frozen_specification_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
    let policy = fixture_band_policy();
    for entry in fixture["lodBands"].as_array().expect("lodBands array") {
        let span = entry["spanDeg"].as_f64().expect("spanDeg");
        let expected_idx = entry["lodIndex"].as_u64().expect("lodIndex") as usize;
        let expected_tile_z = entry["tileZ"].as_u64().expect("tileZ") as u32;
        let idx = policy.band_for_span(span, None);
        assert_eq!(idx, expected_idx, "band index for span={span}");
        assert_eq!(policy.band(idx).raster_tile_z, expected_tile_z, "tile z for span={span}");
    }
}

#[test]
fn a_host_reports_the_band_its_installed_policy_resolves() {
    let mut host = super::MapHost::with_extension(super::Arc::new(super::DefaultMapExtension::with_policy(fixture_band_policy())));
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let fit = host.camera.zoom;
    for &span_probe in &[180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05] {
        host.set_camera(0.0, 0.0, fit * (180.0 / f64::max(span_probe, 1e-6)));
        let expected = format!("band-{}", host.policy().band_for_span(host.viewport_span_degrees(), None));
        let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("current lod json");
        assert_eq!(lod["id"], expected.as_str(), "current band for span probe {span_probe}");
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
