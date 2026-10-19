use super::*;

#[test]
fn retained_weight_dictionary_preserves_authored_wire_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let weights: Puzzle5dKindWeights = serde_json::from_value(row["weights"].clone()).unwrap();
        let ids: Vec<String> = serde_json::from_value(row["ids"].clone()).unwrap();
        let result = puzzle5d_normalize_kind_weight_group(&weights, &ids, row["changed"].as_str().unwrap(), row["value"].as_f64().unwrap());
        assert_eq!(result.len(), row["expected"].as_object().unwrap().len());
        for (id, expected) in row["expected"].as_object().unwrap() {
            assert!((result.get(id).copied().unwrap() - expected.as_f64().unwrap()).abs() < 1e-12);
        }
        let encoded = serde_json::to_value(&result).unwrap();
        for (id, value) in encoded.as_object().unwrap() {
            assert!((value.as_f64().unwrap() - row["expected"][id].as_f64().unwrap()).abs() < 1e-12);
        }
    }
    println!("[DEBUG] Puzzle5D native retained weight dictionary matches six shared wire vectors");
}
