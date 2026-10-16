//! 🧪️ Unit proofs of the MVT decoder against the six owned protobuf tiles, read at run time from the tiled-map fixture folder so no tile bytes are compiled into the crate.

use super::{decode_mvt, feature_label, feature_label_anchor, GeomType, VectorFeature};
use std::collections::BTreeMap;
use std::path::PathBuf;

const VECTOR_TILE_KEYS: [&str; 6] = ["0-0-0", "1-1-0", "2-2-1", "3-4-2", "4-8-5", "5-17-11"];

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../🧪️tests/🧫️fixtures/🗺️vector-tiles")
}

fn vector_tile_fixture(key: &str) -> Vec<u8> {
    std::fs::read(fixture_root().join(key).join("🛰️.pbf")).expect("owned vector tile fixture")
}

#[test]
fn owned_vector_tile_inventory_agrees_with_independent_protobuf_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(fixture_root().join("🔣️.json")).expect("vector tile corpus file")).expect("vector tile corpus");
    let rows = corpus["tiles"].as_array().expect("tile roster");
    assert_eq!(rows.len(), VECTOR_TILE_KEYS.len());
    for row in rows {
        let key = row["key"].as_str().expect("tile key");
        let bytes = vector_tile_fixture(key);
        assert_eq!(bytes.len() as u64, row["bytes"].as_u64().expect("byte count"));
        let tile = decode_mvt(&bytes).expect("owned vector tile");
        let layers: Vec<_> = tile
            .layers
            .iter()
            .map(|layer| {
                let mut geometry_types = [0usize; 4];
                for feature in &layer.features {
                    let kind = match feature.geom_type {
                        GeomType::Unknown => 0,
                        GeomType::Point => 1,
                        GeomType::LineString => 2,
                        GeomType::Polygon => 3,
                    };
                    geometry_types[kind] += 1;
                }
                serde_json::json!({"name": layer.name, "extent": layer.extent, "features": layer.features.len(), "geometryTypes": geometry_types})
            })
            .collect();
        assert_eq!(serde_json::Value::Array(layers), row["layers"], "tile {key}");
    }
    println!("[DEBUG] Owned vector tile layer/feature/geometry inventory matches the independent Protobuf.js corpus for all six original tiles");
}

#[test]
fn decode_mvt_empty_bytes_yields_empty_tile() {
    let tile = decode_mvt(&[]).expect("empty pbf");
    assert!(tile.layers.is_empty());
}

#[test]
fn decode_demotile_fixture_has_named_layers() {
    let bytes = vector_tile_fixture("2-2-1");
    let tile = decode_mvt(&bytes).expect("decode");
    assert!(!tile.layers.is_empty());
    assert!(tile.layers.iter().any(|l| !l.features.is_empty()));
    let names: Vec<&str> = tile.layers.iter().map(|l| l.name.as_str()).collect();
    assert!(names.contains(&"countries"));
    assert!(names.contains(&"centroids"));
}

#[test]
fn demotile_fixture_countries_use_multi_ring_polygons() {
    let bytes = vector_tile_fixture("2-2-1");
    let tile = decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries layer");
    let multi = countries.features.iter().filter(|f| f.rings.len() > 1).count();
    assert!(multi > 0, "fixture should include multi-ring countries");
}

#[test]
fn fixture_linestrings_split_at_moveto() {
    let mut found = false;
    for key in VECTOR_TILE_KEYS {
        let tile = decode_mvt(&vector_tile_fixture(key)).expect("decode");
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
    let bytes = vector_tile_fixture("5-17-11");
    let tile = decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
    let centroids = tile.layers.iter().find(|l| l.name == "centroids").expect("centroids");
    assert!(countries.features.len() >= 10);
    assert!(centroids.features.len() >= 5);
    let label_keys: Vec<String> = ["name:en", "name_en", "name_int", "name", "name:latin", "NAME", "Name", "title", "TITLE", "ref"].iter().map(|key| (*key).to_string()).collect();
    assert!(centroids.features.iter().any(|f| feature_label(&f.properties, &label_keys).is_some()));
    assert!(centroids.features.iter().all(|f| feature_label(&f.properties, &[]).is_none()), "no label key, no label");
}

#[test]
fn demotile_z0_has_many_country_features() {
    let bytes = vector_tile_fixture("0-0-0");
    let tile = decode_mvt(&bytes).expect("decode");
    let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
    assert!(countries.features.len() >= 50, "world tile should include many countries");
}

#[test]
fn vector_label_anchor_uses_line_midpoint_for_transportation_names() {
    let feature = VectorFeature { id: None, geom_type: GeomType::LineString, rings: Vec::new(), lines: vec![vec![(100.0, 100.0), (200.0, 200.0), (300.0, 300.0)]], points: Vec::new(), properties: BTreeMap::from([("ref".into(), "A81".into())]) };
    let anchor = feature_label_anchor(&feature, 4096);
    assert_eq!(anchor, (200.0, 200.0), "line labels must not fall back to the tile center");
    let center = feature_label_anchor(&VectorFeature { lines: Vec::new(), points: Vec::new(), rings: Vec::new(), geom_type: GeomType::Unknown, id: None, properties: BTreeMap::new() }, 4096);
    assert_eq!(center, (2048.0, 2048.0));
}
