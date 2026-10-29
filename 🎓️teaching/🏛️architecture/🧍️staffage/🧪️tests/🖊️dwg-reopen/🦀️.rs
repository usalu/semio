//! 🖊️ DWG reopened with the independent reader `acadrust`: signature AC1024, metre insertion units, layers with ACI colour and lineweight,
//! LWPOLYLINE count, closed flags and coordinates to 1e-6, and the TEXT entity.
use acadrust::entities::EntityType;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/📐️model-scene/🔣️.json");

fn read(bytes: &[u8]) -> acadrust::CadDocument {
    acadrust::DwgReader::from_stream(std::io::Cursor::new(bytes.to_vec())).read().expect("acadrust reads the DWG")
}

#[test]
fn acadrust_reads_the_staffage_model_scene_back() {
    let bytes = staffage_dwg::encode_json(FIXTURE.as_bytes()).expect("the writer encodes the scene");
    assert_eq!(&bytes[..6], b"AC1024");
    let document = read(&bytes);
    assert_eq!(document.version, acadrust::types::DxfVersion::AC1024);
    assert_eq!(document.header.insertion_units, 6);
    let scene: Value = serde_json::from_str(FIXTURE).unwrap();
    for layer in scene["layers"].as_array().unwrap() {
        let name = layer["name"].as_str().unwrap();
        let found = document.layers.get(name).unwrap_or_else(|| panic!("layer {name} survives"));
        assert_eq!(u64::from(found.color.index().expect("ACI colour")), layer["aci"].as_u64().unwrap(), "{name} colour");
        assert_eq!(i64::from(found.line_weight.value()), layer["lineweight"].as_i64().unwrap(), "{name} lineweight");
    }
    let expected: Vec<(String, bool, Vec<[f64; 2]>)> = scene["layers"].as_array().unwrap().iter().flat_map(|layer| {
        let name = layer["name"].as_str().unwrap().to_string();
        layer["polylines"].as_array().unwrap().iter().map(move |line| (name.clone(), line["closed"].as_bool().unwrap(), line["points"].as_array().unwrap().iter().map(|p| [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap()]).collect()))
    }).collect();
    let mut actual: Vec<(String, bool, Vec<[f64; 2]>)> = document.entities().filter_map(|entity| match entity {
        EntityType::LwPolyline(line) => Some((line.common.layer.clone(), line.is_closed, line.vertices.iter().map(|v| [v.location.x, v.location.y]).collect())),
        _ => None,
    }).collect();
    assert_eq!(actual.len(), expected.len(), "LWPOLYLINE count");
    let key = |row: &(String, bool, Vec<[f64; 2]>)| format!("{}{}{:?}", row.0, row.1, row.2.iter().map(|p| p.map(|c| (c * 1e6).round() as i64)).collect::<Vec<_>>());
    let mut want = expected;
    want.sort_by_key(key);
    actual.sort_by_key(key);
    for (a, b) in actual.iter().zip(&want) {
        assert_eq!((&a.0, a.1, a.2.len()), (&b.0, b.1, b.2.len()));
        for (p, q) in a.2.iter().zip(&b.2) {
            assert!((p[0] - q[0]).abs() < 1e-6 && (p[1] - q[1]).abs() < 1e-6, "{p:?} vs {q:?}");
        }
    }
    let texts: Vec<_> = document.entities().filter_map(|entity| match entity { EntityType::Text(text) => Some((text.insertion_point.x, text.insertion_point.y, text.height, text.value.clone())), _ => None }).collect();
    assert_eq!(texts.len(), 1);
    assert!((texts[0].0 - 0.35).abs() < 1e-6 && (texts[0].1 - 0.1).abs() < 1e-6 && (texts[0].2 - 0.1).abs() < 1e-6);
    assert_eq!(texts[0].3, "1,80 m \u{b7} 2,5 m\u{b2}");
}

#[test]
fn an_empty_scene_still_encodes_a_genuine_ac1024() {
    let bytes = staffage_dwg::encode_json(br#"{"units":"m","asset":"probe","timeS":0,"layers":[]}"#).expect("empty scene");
    assert_eq!(&bytes[..6], b"AC1024");
    assert_eq!(read(&bytes).entities().count(), 0);
}

#[test]
fn non_metre_scenes_are_refused() {
    assert!(staffage_dwg::encode_json(br#"{"units":"mm","layers":[]}"#).is_err());
}
