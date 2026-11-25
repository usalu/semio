use super::*;
use serde_json::Value;

#[test]
fn card_height_snap_language_neutral_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("bands.cases.json")).unwrap();
    for snap in fixture["snaps"].as_array().unwrap() {
        let got = snap_card_height(snap["height"].as_f64().unwrap(), snap["pixel"].as_f64().unwrap());
        assert!((got - snap["expected"].as_f64().unwrap()).abs() < 1e-9, "{snap}");
    }
}
