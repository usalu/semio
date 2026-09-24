use super::*;
use crate::standards::v_rfc8259::subsets::geojson::io::{JsonGeoJsonComposerComposition, JsonGeoJsonValidator};
use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeSource, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator};
use std::str::FromStr;

const VECTORS: &str = include_str!("../../../🧫️fixtures/🌍️read-write/🔣️.json");

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn vectors() -> (Vec<Value>, f64) {
    let document: Value = serde_json::from_str(VECTORS).expect("vectors are JSON");
    (document["cases"].as_array().expect("cases").clone(), document["tolerance"].as_f64().expect("tolerance"))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn agrees(actual: &Value, expected: &Value, tolerance: f64) -> bool {
    match (actual, expected) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().expect("finite") - b.as_f64().expect("finite")).abs() <= tolerance,
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| agrees(a, b, tolerance)),
        (Value::Object(a), Value::Object(b)) => a.len() == b.len() && a.iter().all(|(key, value)| b.get(key).is_some_and(|other| agrees(value, other, tolerance))),
        _ => actual == expected,
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn crs_name(crs: GeoJsonSourceCrs) -> &'static str {
    match crs {
        GeoJsonSourceCrs::Rfc7946 => "rfc7946",
        GeoJsonSourceCrs::DeclaredCrs84 => "declared-crs84",
        GeoJsonSourceCrs::WebMercator => "web-mercator",
    }
}

#[test]
fn language_neutral_vectors_read_and_write_as_declared() {
    let (cases, tolerance) = vectors();
    for case in &cases {
        let id = case["id"].as_str().expect("id");
        let read = read_geojson(&JsonValue::from(&case["input"]));
        if let Some(path) = case["errorPath"].as_str() {
            assert_eq!(read.expect_err(id).path, path, "{id}");
            continue;
        }
        let read = read.unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(crs_name(read.source_crs), case["sourceCrs"].as_str().expect("sourceCrs"), "{id}");
        assert_eq!(read.left_handed_rings as u64, case["leftHandedRings"].as_u64().expect("leftHandedRings"), "{id}");
        let written = write_geojson(&read.features).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert!(agrees(&written, &case["written"], tolerance), "{id}: wrote {written}");
        let reread = read_geojson(&JsonValue::from(&written)).unwrap_or_else(|error| panic!("{id} re-read: {error}"));
        assert_eq!((reread.source_crs, reread.left_handed_rings), (GeoJsonSourceCrs::Rfc7946, 0), "{id}: the writer's output is plain right-handed RFC 7946");
        assert_eq!(write_geojson(&reread.features).expect("rewrite"), written, "{id}: writing is a fixed point");
    }
}

/// 🔢️ Coordinates agree within two ulps: the oracle parses through `serde_json` without its
/// `float_roundtrip` feature (the workspace's setting), whose decimal reader is not correctly rounded
/// (`-119.99999999999999` reads as `-120.0`), while this subset reads each lexeme exactly.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn same_coordinate(a: f64, b: f64) -> bool {
    (a - b).abs() <= 2.0 * f64::EPSILON * a.abs().max(b.abs()).max(1.0)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn oracle_geometry_agrees(oracle: &geojson::Geometry, ours: &GeoJsonGeometry) -> bool {
    let same = |a: &geojson::Position, b: &GeoJsonPosition| a.as_slice().len() == b.len() && a.as_slice().iter().zip(b).all(|(a, b)| same_coordinate(*a, *b));
    let same_line = |a: &[geojson::Position], b: &[GeoJsonPosition]| a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same(a, b));
    let same_rings = |a: &[Vec<geojson::Position>], b: &[Vec<GeoJsonPosition>]| a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_line(a, b));
    match (&oracle.value, ours) {
        (geojson::GeometryValue::Point { coordinates }, GeoJsonGeometry::Point(position)) => same(coordinates, position),
        (geojson::GeometryValue::MultiPoint { coordinates }, GeoJsonGeometry::MultiPoint(points)) => same_line(coordinates, points),
        (geojson::GeometryValue::LineString { coordinates }, GeoJsonGeometry::LineString(line)) => same_line(coordinates, line),
        (geojson::GeometryValue::MultiLineString { coordinates }, GeoJsonGeometry::MultiLineString(lines)) => same_rings(coordinates, lines),
        (geojson::GeometryValue::Polygon { coordinates }, GeoJsonGeometry::Polygon(rings)) => same_rings(coordinates, rings),
        (geojson::GeometryValue::MultiPolygon { coordinates }, GeoJsonGeometry::MultiPolygon(polygons)) => coordinates.len() == polygons.len() && coordinates.iter().zip(polygons).all(|(a, b)| same_rings(a, b)),
        (geojson::GeometryValue::GeometryCollection { geometries }, GeoJsonGeometry::GeometryCollection(members)) => geometries.len() == members.len() && geometries.iter().zip(members).all(|(a, b)| oracle_geometry_agrees(a, b)),
        _ => false,
    }
}

/// 🔮️ The third-party `geojson` crate parses every document the writer emits into the same features,
/// ids, properties and coordinates our own reader recovers from it.
#[test]
fn geojson_crate_reads_every_written_document_identically() {
    let (cases, _) = vectors();
    let mut compared = 0;
    for case in cases.iter().filter(|case| case["errorPath"].is_null()) {
        let id = case["id"].as_str().expect("id");
        let written = serde_json::to_string(&write_geojson(&read_geojson(&JsonValue::from(&case["input"])).expect(id).features).expect(id)).expect("text");
        let ours = read_geojson_text(&written).expect(id).features;
        let geojson::GeoJson::FeatureCollection(collection) = geojson::GeoJson::from_str(&written).unwrap_or_else(|error| panic!("{id}: the geojson crate refuses our output: {error}")) else {
            panic!("{id}: not a FeatureCollection to the oracle")
        };
        assert_eq!(collection.features.len(), ours.len(), "{id}");
        for (oracle, ours) in collection.features.iter().zip(&ours) {
            let oracle_id = oracle.id.as_ref().map(|id| match id {
                geojson::feature::Id::String(text) => GeoJsonId::Text(text.clone()),
                geojson::feature::Id::Number(number) => GeoJsonId::Number(number.clone()),
            });
            assert_eq!(oracle_id, ours.id, "{id}");
            assert_eq!(oracle.properties, ours.properties, "{id}");
            match (&oracle.geometry, &ours.geometry) {
                (Some(oracle), Some(ours)) => assert!(oracle_geometry_agrees(oracle, ours), "{id}: {oracle:?} vs {ours:?}"),
                (None, None) => {}
                (oracle, ours) => panic!("{id}: {oracle:?} vs {ours:?}"),
            }
            compared += 1;
        }
    }
    assert_eq!(compared, 8);
}

/// 🔮️ A FeatureCollection written by the `geojson` crate reads back into exactly the geometry, ids
/// and properties it was built from.
#[test]
fn reader_accepts_what_the_geojson_crate_writes() {
    let properties = serde_json::json!({ "name": "Lighthouse", "height": 42 }).as_object().expect("object").clone();
    let features = vec![
        geojson::Feature { id: Some(geojson::feature::Id::String("lighthouse".into())), geometry: Some(geojson::Geometry::new_point([5.58, 50.60, 17.5])), properties: Some(properties.clone()), ..Default::default() },
        geojson::Feature { id: Some(geojson::feature::Id::Number(3.into())), geometry: Some(geojson::Geometry::new_line_string([[5.5, 50.5], [5.6, 50.6], [5.7, 50.5]])), properties: None, ..Default::default() },
        geojson::Feature {
            id: None,
            geometry: Some(geojson::Geometry::new_polygon([vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 0.0]], vec![[0.5, 0.5], [0.5, 1.0], [1.0, 1.0], [1.0, 0.5], [0.5, 0.5]]])),
            properties: Some(serde_json::Map::new()),
            ..Default::default()
        },
    ];
    let text = geojson::GeoJson::from(geojson::FeatureCollection { features, bbox: None, foreign_members: None }).to_string_pretty().expect("oracle writes");
    let read = read_geojson_text(&text).expect("RFC 7946 text written by the oracle is readable");
    assert_eq!(read.source_crs, GeoJsonSourceCrs::Rfc7946);
    assert_eq!(read.left_handed_rings, 0);
    assert_eq!(read.features[0], GeoJsonFeature { id: Some(GeoJsonId::Text("lighthouse".into())), geometry: Some(GeoJsonGeometry::Point(vec![5.58, 50.60, 17.5])), properties: Some(properties) });
    assert_eq!(read.features[1].id, Some(GeoJsonId::Number(3.into())));
    assert_eq!(read.features[1].geometry, Some(GeoJsonGeometry::LineString(vec![vec![5.5, 50.5], vec![5.6, 50.6], vec![5.7, 50.5]])));
    assert_eq!(read.features[2].geometry, Some(GeoJsonGeometry::Polygon(vec![vec![vec![0.0, 0.0], vec![2.0, 0.0], vec![2.0, 2.0], vec![0.0, 2.0], vec![0.0, 0.0]], vec![vec![0.5, 0.5], vec![0.5, 1.0], vec![1.0, 1.0], vec![1.0, 0.5], vec![0.5, 0.5]]])));
}

#[test]
fn right_handed_ring_closes_and_orients() {
    let clockwise = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0], vec![1.0, 0.0]];
    let exterior = right_handed_ring(&clockwise, true);
    assert_eq!(exterior.first(), exterior.last());
    assert!(ring_signed_area2(&exterior) > 0.0);
    assert!(ring_signed_area2(&right_handed_ring(&clockwise, false)) < 0.0);
    assert!(write_geojson(&[GeoJsonFeature { geometry: Some(GeoJsonGeometry::Polygon(vec![vec![vec![0.0, 0.0], vec![1.0, 1.0]]])), ..Default::default() }]).is_err(), "a ring of two distinct positions cannot close into four");
    assert!(write_geojson(&[GeoJsonFeature { geometry: Some(GeoJsonGeometry::Point(vec![621000.0, 5600000.0])), ..Default::default() }]).is_err(), "the writer never emits a non-WGS 84 position");
}

const ANY: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

#[semio_framework_async_macros::async_test]
async fn composer_stamps_only_conforming_documents_and_carries_soft_findings() {
    let mercator = r#"{"type":"Feature","crs":{"type":"name","properties":{"name":"EPSG:3857"}},"geometry":{"type":"Point","coordinates":[0,0]},"properties":null}"#;
    let composed = JsonGeoJsonComposerComposition::compose(&[ComposeSource { dialect: ANY, payload: AnalyzeSource::Text(mercator) }]).expect("a GJ2008 Web Mercator document stamps geojson");
    assert!(composed.diagnostics.iter().any(|d| d.code.0 == CODE_LEGACY_CRS && d.severity == dsl::Severity::Warning), "{:?}", composed.diagnostics);
    let projected = r#"{"type":"Point","coordinates":[621000,5600000]}"#;
    let refused = JsonGeoJsonComposerComposition::compose(&[ComposeSource { dialect: ANY, payload: AnalyzeSource::Text(projected) }]).expect_err("projected metres are not RFC 7946");
    assert!(refused.diagnostics.iter().any(|d| d.code.0 == CODE_NOT_GEOJSON && d.severity == dsl::Severity::Error), "{:?}", refused.diagnostics);
    let snapshot = JsonSnapshot::from_value(serde_json::json!({ "type": "FeatureCollection", "features": [] }));
    assert!(JsonGeoJsonValidator::validate(&IoPayload::Binary(<JsonSnapshot as store::ArtifactPack>::encode_pack(&snapshot))).await.is_empty());
    let not_geojson = JsonSnapshot::from_value(serde_json::json!({ "type": "Topology" }));
    assert!(JsonGeoJsonValidator::validate(&IoPayload::Binary(<JsonSnapshot as store::ArtifactPack>::encode_pack(&not_geojson))).await.iter().any(|d| d.code.0 == CODE_NOT_GEOJSON));
}
